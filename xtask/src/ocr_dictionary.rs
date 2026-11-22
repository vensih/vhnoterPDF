//! `cargo xtask ocr-dictionary <out.txt> [--frequency <list>] [--reference <list>] [<source>...]`:
//! the Albanian OCR word lists.
//!
//! - The public list shipped beside the models (`models/ocr-dictionary-sq.txt`) comes from a
//!   word-frequency list (`--frequency`, FrequencyWords `sq_50k.txt`, `word count` per line,
//!   fetched by `cargo xtask models`): its most frequent words.
//! - The office list (`ocr-dictionary-sq-office.txt`, placed by `deploy.cmd`, never in the
//!   installer) comes from the office's own templates (`<source>...`), with the public list as
//!   `--reference` so that words typed without `ë` are recognised as such.
//!
//! The templates are private, so the office list is built only on the packaging machine and
//! never enters this repository. Only the text inside string literals is read
//! (template wording, not code), markup and `{placeholders}` are dropped, and a word is kept only
//! when it also appears written in lower case: names of people and places, which are always
//! capitalised, stay out. Files under a `people` folder are never read. The templates have their
//! own typos and words typed without `ë`, so rare words and `ë`-less twins are left out too.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

/// The app reads at most this many words from a dictionary file (`ocr::dictionary`).
const MAX_WORDS: usize = 65_536;
/// Shortest and longest word kept, in letters.
const MIN_LETTERS: usize = 2;
const MAX_LETTERS: usize = 40;
/// A word seen fewer times is left out: the templates are not free of typos, and a typo in the
/// list would let the same misreading pass as correct.
const MIN_SEEN: usize = 2;
/// The fewest times a word must occur in the frequency list to be kept (subtitle text has typos).
const MIN_FREQUENCY: usize = 30;
/// From this length on, a spelling without `ë`/`ç` is dropped when the same word is also written
/// with them (`sipermarresit` beside `sipërmarrësit`): OCR that loses the `ë` must stay flagged.
/// Shorter words keep both, since many pairs are different words (`te`/`të`, `me`/`më`).
const MIN_LETTERS_FOR_DIACRITIC_CHECK: usize = 5;

const USAGE: &str = "usage: cargo xtask ocr-dictionary <out.txt> [--frequency <list>] [--reference <list>] [<source dir or file>...]";

pub fn run(args: &[String]) -> Result<()> {
    let [out, rest @ ..] = args else { bail!(USAGE) };
    let mut counts = Counts::default();
    let mut sources = Vec::new();
    let mut rest = rest.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            flag @ ("--frequency" | "--reference") => {
                let Some(list) = rest.next() else { bail!(USAGE) };
                let text = std::fs::read_to_string(list).with_context(|| format!("reading {list}"))?;
                counts.add_frequency_list(&text, flag == "--frequency");
            }
            source => sources.push(source),
        }
    }
    if sources.is_empty() && counts.seen.is_empty() {
        bail!(USAGE);
    }
    let mut files = Vec::new();
    for s in sources {
        collect(Path::new(s), &mut files).with_context(|| format!("reading {s}"))?;
    }
    for f in &files {
        let text = std::fs::read_to_string(f).with_context(|| format!("reading {}", f.display()))?;
        counts.add_source(&text);
    }
    let words = counts.words();
    std::fs::write(out, words.iter().map(|w| format!("{w}\n")).collect::<String>()).with_context(|| format!("writing {out}"))?;
    println!("ocr-dictionary: {} words from {} files into {out}", words.len(), files.len());
    Ok(())
}

/// Every `.js`, `.json`, `.html`, `.txt` or `.md` file under `path`, skipping `people` folders.
fn collect(path: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    if path.is_file() {
        out.push(path.to_path_buf());
        return Ok(());
    }
    for entry in std::fs::read_dir(path)? {
        let p = entry?.path();
        let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if p.is_dir() {
            if name != "people" && !name.starts_with('.') {
                collect(&p, out)?;
            }
        } else if matches!(p.extension().and_then(|e| e.to_str()), Some("js" | "json" | "html" | "txt" | "md")) {
            out.push(p);
        }
    }
    out.sort();
    Ok(())
}

#[derive(Default)]
struct Counts {
    /// Lower-cased word: (times seen, seen written in lower case).
    seen: BTreeMap<String, (usize, bool)>,
    /// Words of a `--reference` list: they only reveal `ë`-less twins, and are not output.
    reference: BTreeSet<String>,
}

impl Counts {
    /// A `word count` list. Kept words (`--frequency`) need [`MIN_FREQUENCY`] occurrences.
    fn add_frequency_list(&mut self, text: &str, keep: bool) {
        for line in text.lines() {
            let mut parts = line.split_whitespace();
            let (Some(word), Some(n)) = (parts.next(), parts.next().and_then(|n| n.parse::<usize>().ok())) else { continue };
            let letters = word.chars().count();
            if !(MIN_LETTERS..=MAX_LETTERS).contains(&letters) || !word.chars().all(is_albanian_letter) {
                continue;
            }
            let lower = word.to_lowercase();
            if !keep {
                self.reference.insert(lower);
            } else if n >= MIN_FREQUENCY {
                let entry = self.seen.entry(lower).or_default();
                entry.0 = entry.0.max(n);
                entry.1 = true;
            }
        }
    }

    fn add_source(&mut self, source: &str) {
        for literal in string_literals(source) {
            self.add_text(&strip_markup(&literal));
        }
    }

    fn add_text(&mut self, text: &str) {
        for word in text.split(|c: char| !(c.is_alphabetic() || c == '-')).flat_map(|w| w.split('-')) {
            let letters = word.chars().count();
            if !(MIN_LETTERS..=MAX_LETTERS).contains(&letters) || !word.chars().all(is_albanian_letter) {
                continue;
            }
            let lower = word.to_lowercase();
            let entry = self.seen.entry(lower.clone()).or_default();
            entry.0 += 1;
            entry.1 |= word == lower;
        }
    }

    /// The words kept: written in lower case somewhere, seen [`MIN_SEEN`] times, not an `ë`-less
    /// twin of a longer word, most frequent first up to the cap, then sorted.
    fn words(&self) -> BTreeSet<String> {
        let with_diacritics: BTreeSet<String> =
            self.seen.keys().chain(&self.reference).filter(|w| w.contains(['ë', 'ç'])).map(|w| plain(w)).collect();
        let mut kept: Vec<(&String, usize)> = self
            .seen
            .iter()
            .filter(|(_, (n, lower))| *lower && *n >= MIN_SEEN)
            .filter(|(w, _)| w.contains(['ë', 'ç']) || w.chars().count() < MIN_LETTERS_FOR_DIACRITIC_CHECK || !with_diacritics.contains(w.as_str()))
            .map(|(w, (n, _))| (w, *n))
            .collect();
        kept.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
        kept.into_iter().take(MAX_WORDS).map(|(w, _)| w.clone()).collect()
    }
}

/// `word` with `ë` and `ç` written as `e` and `c`.
fn plain(word: &str) -> String {
    word.chars()
        .map(|c| match c {
            'ë' => 'e',
            'ç' => 'c',
            c => c,
        })
        .collect()
}

/// The Latin letters Albanian is written with (`ë`, `ç` and the plain alphabet).
fn is_albanian_letter(c: char) -> bool {
    c.is_ascii_alphabetic() || matches!(c, 'ë' | 'Ë' | 'ç' | 'Ç')
}

/// The contents of every `'…'`, `"…"` and `` `…` `` literal in a JS-like source, comments skipped.
fn string_literals(source: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut chars = source.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '/' if chars.peek() == Some(&'/') => {
                for c in chars.by_ref() {
                    if c == '\n' {
                        break;
                    }
                }
            }
            '/' if chars.peek() == Some(&'*') => {
                chars.next();
                let mut last = ' ';
                for c in chars.by_ref() {
                    if last == '*' && c == '/' {
                        break;
                    }
                    last = c;
                }
            }
            '\'' | '"' | '`' => {
                let quote = c;
                let mut literal = String::new();
                while let Some(c) = chars.next() {
                    match c {
                        '\\' => {
                            if let Some(escaped) = chars.next() {
                                literal.push(if escaped == 'n' { ' ' } else { escaped });
                            }
                        }
                        c if c == quote => break,
                        c => literal.push(c),
                    }
                }
                out.push(literal);
            }
            _ => {}
        }
    }
    out
}

/// `text` without `<tags>`, `${…}` and `{placeholders}`, which are markup, not wording.
fn strip_markup(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut depth_tag = false;
    let mut depth_brace = 0usize;
    for c in text.chars() {
        match c {
            '<' if depth_brace == 0 => depth_tag = true,
            '>' if depth_tag => {
                depth_tag = false;
                out.push(' ');
            }
            '{' => depth_brace += 1,
            '}' if depth_brace > 0 => {
                depth_brace -= 1;
                out.push(' ');
            }
            _ if depth_tag || depth_brace > 0 => {}
            c => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rare_words_and_words_typed_without_e_diaeresis_stay_out() {
        let mut counts = Counts::default();
        counts.add_text("sipërmarrësit sipërmarrësit sipermarresit sipermarresit te te të të shrkim");
        let words = counts.words();
        assert!(words.contains("sipërmarrësit"));
        assert!(!words.contains("sipermarresit"), "the ë-less twin of a long word");
        assert!(words.contains("te") && words.contains("të"), "short pairs are different words");
        assert!(!words.contains("shrkim"), "seen once: maybe a typo");
    }

    #[test]
    fn a_frequency_list_keeps_frequent_words_and_a_reference_only_flags_twins() {
        let mut counts = Counts::default();
        counts.add_frequency_list(
            "të 1000
ngatërruar 40
ngaterruar 35
rrallë 3
hello! 90
",
            true,
        );
        let words = counts.words();
        assert!(words.contains("të") && words.contains("ngatërruar"));
        assert!(!words.contains("ngaterruar"), "ë-less twin");
        assert!(!words.contains("rrallë"), "too rare");
        let mut office = Counts::default();
        office.add_frequency_list(
            "sigurojmë 500
",
            false,
        );
        office.add_text("sigurojme sigurojme noterit noterit");
        let words = office.words();
        assert!(!words.contains("sigurojme"), "the reference knows the ë spelling");
        assert!(words.contains("noterit") && !words.contains("sigurojmë"), "reference words are not output");
    }

    #[test]
    fn names_markup_and_code_stay_out_of_the_word_list() {
        let mut counts = Counts::default();
        for _ in 0..2 {
            counts.add_source(
            "// komenti nuk\nexport const T = { title: 'Prokurë e Përgjithshme', body: `<b>Unë</b>, {emri}, i lindur në Shkodër, vërtetoj prokurën.` };\nconst x = \"përgjithshme vërtetim\";",
            );
        }
        let words = counts.words();
        for kept in ["vërtetoj", "prokurën", "lindur", "përgjithshme", "vërtetim"] {
            assert!(words.contains(kept), "{kept} in {words:?}");
        }
        for left_out in ["shkodër", "prokurë", "export", "const", "title", "emri", "komenti", "b"] {
            assert!(!words.contains(left_out), "{left_out} in {words:?}");
        }
    }
}
