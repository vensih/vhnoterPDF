# Maintaining the vhnoterPDF fork

vhnoterPDF is a fork of [storytold/pdfcraft](https://github.com/storytold/pdfcraft). This page lists
what the fork changes, what it keeps on purpose, and what to check each time upstream is merged.
The full per-file list is at the end.

## What the fork changes

| Change | Where | Merge risk |
|---|---|---|
| **Display name** PdfCraft → vhnoterPDF in every user-visible string, message, test string and most code comments | about 230 files across `crates/` and `apps/` | High: upstream edits near any of these lines conflict. Resolve by keeping upstream's change and re-applying the name. |
| **Executable** `pdfcraft(.exe)` → `vhnoterpdf(.exe)` (`[[bin]] name` in `apps/pdfcraft/Cargo.toml`; the package and folder keep `pdfcraft`) | `apps/pdfcraft/{Cargo.toml,build.rs,src/main.rs,src/logging.rs,tests/help.rs}`, `packaging/**` (MSI, install policy, `.desktop`, metainfo, nfpm, AppImage, flatpak, FreeBSD, macOS) | Medium: packaging files change upstream often. |
| **Updater** asks only `vensih/vhnoterPDF`; `id = "vhnoterpdf"`, `former_ids = ["pdfcraft", "printcraft"]`, bundle id kept | `crates/ui-egui/src/lib.rs` (`UPDATE_APP`), `crates/ui-egui/src/platform/updates.rs` (`RELEASES_PAGE`, test `updates_come_only_from_the_fork`) | Low, but never take upstream's `UPDATE_APP` or `RELEASES_PAGE` values. |
| **Links**: Discord, ArtCraft and getartcraft.com removed; the vhnoter.al link (`help.website`) and the source code link stay; the installer's support and about links go to vhnoter.al | `crates/engine/src/links.rs` (+ `links/tests.rs`), `crates/engine/src/commands.rs` (help.* commands), `crates/ui-egui/src/{chrome,home,menus,widgets,dialogs}.rs`, `crates/ui-egui/tests/it/{links,ui}.rs`, `apps/pdfcraft-cli/src/main.rs`, `crates/automation/src/mcp.rs`, packaging metadata | Medium: upstream may add new community links or a new home card; remove them again. |
| **Interface language**: English on a fresh launch (upstream: Auto) | `crates/ui-egui/src/app.rs` (`language: "en"`), `crates/ui-egui/src/i18n/tests.rs` | Low. |
| **Albanian OCR**: `sq` → `sqi`, `sq+en` → `sqi+eng`; Albanian never read by ocrs; missing pack named | `crates/ocr/src/{lib.rs,tesseract.rs}`, `crates/engine/src/ocr.rs`, `crates/ui-egui/src/workflows/{ocr_ui,ocr_verify}.rs` | Medium: upstream OCR work lands in these files. |
| **Tesseract**: Albanian and English data shipped in `models/` (`[[fetched]]` `sqi.traineddata`, `eng.traineddata` in `ATTRIBUTION.toml`, fetched by `cargo xtask models`) and used when the installed Tesseract lacks the pack; TSV through `-c tessedit_create_tsv=1`; Windows install folders searched after the PATH; **Install Tesseract…** (`ocr.install_tesseract`) runs winget; Linux packages recommend Tesseract | `crates/ocr/src/tesseract.rs`, `crates/engine/src/{ocr.rs,commands.rs}`, `crates/ui-egui/src/{commands.rs,lib.rs,app.rs,workflows/ocr_ui.rs}`, `packaging/linux/nfpm.yaml`, `packaging/windows/PORTABLE-README.md`, `docs/localization.md` | Medium: upstream says Tesseract is "never bundled"; keep the fallback, the `-c` switch and the command when merging OCR work. |
| **OCR defaults and word lists**: the desktop app starts Recognize text on Albanian + English, engine Automatic (the built-in engine refuses Albanian), with deskew and denoise; Albanian always reads the shipped `tessdata_best` model; Albanian + English checks words against the Albanian dictionary; two read-only lists are merged with the user's: the public `models/ocr-dictionary-sq.txt` (built by `cargo xtask ocr-dictionary --frequency` from FrequencyWords `sq_50k.txt`, CC-BY-SA-4.0, fetched by `cargo xtask models`) and the office's `ocr-dictionary-sq-office.txt` in the user's Dictionary folder (built from the private templates by `deploy.ps1`, placed by `deploy.cmd`, never in the MSI); suggestions are computed once per word; Install Tesseract starts winget only in the desktop app (`may_start_installers`) | `apps/pdfcraft/src/main.rs`, `crates/ocr/src/tesseract.rs` (`choose_tessdata`), `crates/ui-egui/src/workflows/ocr_verify.rs`, `xtask/src/ocr_dictionary.rs`, `packaging/windows/{package.ps1,deploy.ps1,deploy.cmd}`, `ATTRIBUTION.toml`, `NOTICE` | Low to medium: keep them when merging OCR work. |
| **Office deploy folder and installer logo**: `deploy.ps1` makes `dist/deploy/` (the MSI, the official Tesseract installer at a pinned SHA-256, the office word list, `deploy.cmd`); Tesseract is never inside the MSI (its Windows build has LGPL and GPL libraries). The installer shows the office logo when `package.ps1` finds the private `app.vhnoter.al/assets/images/logo.png` (`LogoBmp`); it is a trademark and never committed. Both outputs are private: office PCs only | `packaging/windows/{deploy.ps1,deploy.cmd,package.ps1,installer-ui.wxs}` | Low: upstream rarely edits the installer UI; keep the `LogoBmp` blocks. |
| **Em dashes** removed from app strings (keys changed too) | `crates/ui-egui/**`, `crates/engine/src/{edit/labels.rs,session/editing.rs}`, `crates/ocr/src/merge.rs`, `NOTICE`, a snapshot | Low per file; new upstream strings may bring em dashes back. |
| **Catalogs** pinned to the fork `vensih/craft-i18n` (branch `vhnoterpdf`) | `craft-i18n.rev`, both flatpak manifests (`url:`), `apps/pdfcraft/src/tests.rs`, `ATTRIBUTION.toml` | High, see below. |
| **Icons**: gold V app icon and V document icon | `assets/app-icon/**`, `packaging/icons.sh`, `ATTRIBUTION.*`, `LICENSES.toml`, `NOTICE` | Low: upstream rarely touches icons; keep ours. |
| **Packaging text**: installer names, shortcuts, context menus, Info.plist display names, Credits, portable README | `packaging/windows/{pdfcraft.wxs,install-policy.json,test-msi.ps1,package.ps1,PORTABLE-README.md}`, `packaging/linux/*`, `packaging/macos/{Info.plist.in,Credits.rtf}` | Medium. |
| **Docs**: `docs/localization.md` (English default, Albanian OCR), this page | `docs/` | Low. |

## Windows only

vhnoterPDF ships for Windows only. `ci.yml` runs only the `windows-latest` gate (with `cargo fmt --check`, clippy and the tests), `release.yml` turns off the web, RISC-V, FreeBSD and Arch builds, and the FreeBSD, Nix and macOS-nightly workflows are disabled in the repository's Actions settings. The Linux, macOS and FreeBSD packaging files stay in the tree, unused, so upstream merges don't conflict on them. When merging upstream, keep these workflow settings.

## Defaults and a smaller Windows package

- **Tools pane on the right** by default (`SidePanels::default` in `crates/ui-egui/src/panels/side_panels.rs`); a saved choice wins, and old layouts migrate to the side they had.
- **Interface languages offered:** English, Italiano, Deutsch, Français, Shqip (`OFFERED` in `crates/ui-egui/src/i18n.rs`). Preferences lists only these, and Auto picks the system language only when it is one of them (English otherwise). The other catalogs stay available to automation and the tests.
- **Release packages** (`packaging/windows/package.ps1`) set `CRAFT_I18N_LOCALES=it,de,fr,sq` (only those catalogs embedded, `crates/ui-egui/build.rs`) and `CRAFT_FONTS_NO_CHINESE=1` (no Simplified Chinese UI face, `crates/fonts/build.rs`), and ship `vhnoterpdf.exe` without `pdfcraft-cli.exe` (MSI component and install-policy payload removed). The CLI code stays in the tree. `windows-arm64.yml`, which expects `pdfcraft.exe` and a CLI, is disabled.

## Identity: vhnoter suite

- **Publisher** `vhnoter` (vhnoterPDF is part of the vhnoter suite of internal tools, vhnoter.al): MSI `Manufacturer`, the exe's `CompanyName`, the update prompt's developer, nfpm/AppStream vendor.
  The shared updater (craft-update) recognises MSI installs only from upstream's publisher, so on an MSI install the update check opens the release page instead of reinstalling by itself.
- **Version** started at 1.0.0 (now 1.0.2) (`[workspace.package]` and the internal crates' requirements in `Cargo.toml`). When merging upstream, keep ours.
- **Install folder** `Programs\vhnoter\vhnoterPDF` (per user) or `Program Files\vhnoter\vhnoterPDF` (`pdfcraft.wxs`, `install-policy.wxi`, `check-install-policy.ps1`).
- **Data folders** `%APPDATA%\vhnoterPDF` and `%LOCALAPPDATA%\vhnoterPDF` (`DATA_FOLDER` in `crates/ui-egui/src/lib.rs`): settings, logs, recovery, the OCR dictionary, digital IDs, the clipboard cache. The PdfCraft ones are copied once at the first start (`migrate_legacy_folders` in `apps/pdfcraft/src/main.rs`), not moved. The portable `PdfCraftData` folder keeps its name.
- **Installer identity**, so vhnoterPDF installs beside PdfCraft instead of upgrading over it: its own MSI `UpgradeCode` and component GUIDs (`pdfcraft.wxs`, `install-policy.json`), and its own registry keys `Software\vhnoterPDF`, `vhnoterPDF.Document`, `vhnoterPDF.Edit`, `vhnoterPDF.Combine`, `vhnoterPDF.CreatePdf` and the `RegisteredApplications` name `vhnoterPDF`. When merging upstream installer changes, keep ours.
- **Release files** `vhnoterpdf-<version>-windows-<arch>.msi` / `-portable.zip`, built by `.github/workflows/release.yml` (ours, Windows x64 and arm64) on a `v*` tag or by hand, as a draft release. Builds are unsigned unless signing secrets are set.
- **Credits:** `vensih` (shown as vhnoter) in `contributors/contributors.json` and `contributors/people.toml`.
- **Home screen tip** about Albanian OCR and Install Tesseract (`crates/ui-egui/src/home.rs`).

## Kept on purpose (do not rename)

These still say PdfCraft or pdfcraft so that installs, settings and files carry over, and so that
upstream merges stay small:

- Crate and folder names (`pdfcraft-*`, `apps/pdfcraft`), the CLI `pdfcraft-cli`, environment variables `PDFCRAFT_*`.
- Data folders on macOS and Linux (`~/Library/Application Support/PdfCraft`, `pdfcraft` under XDG), the portable `PdfCraftData` folder and `PdfCraft.portable` marker. (Windows uses `vhnoterPDF`, see above.)
- The `%PdfCraft` content-stream markers in `crates/edit/src/place.rs`, which recognise PdfCraft's own wrappers in existing files.
- Bundle and app id `ai.storyteller.pdfcraft`, the macOS `PdfCraft.app` and `CFBundleExecutable`.
- Marker strings that tests match against `vendor/` (for example `"PdfCraft patch: …"` in `apps/pdfcraft/src/tests.rs`).
- The `pdfcraft` stem in `former_ids`, so release files named `pdfcraft-<version>-…` (none so far) would still match.

## Merging upstream

`.github/workflows/upstream-sync.yml` does step 2 (and starts step 6 in CI) every Monday (or from Actions ▸ Upstream sync ▸ Run workflow): a clean merge becomes a `sync/upstream-<sha>` pull request with CI started on it; a merge with conflicts opens an issue naming the files, to resolve by hand as below.

1. `git config rerere.enabled true` once, so git reapplies conflict resolutions it has seen.
2. `git fetch upstream && git merge upstream/main` on a branch. Merge often (weekly) so each merge stays small.
3. Resolve conflicts: take upstream's change, then re-apply the fork's choice from the table above.
4. Re-run the rename passes on new upstream code (they skip the kept identifiers):
   - Display name: replace `\bPdfCraft\b` with `vhnoterPDF` in Rust string literals and comments, skipping lines with `storage_dir(`, `%PdfCraft`, `PdfCraft/`, `/PdfCraft`, `join("PdfCraft")`, `.app`, `.portable`, `Adapted from PdfCraft` and vendor markers.
   - Executable: `pdfcraft.exe`, `bin/pdfcraft`, `release/pdfcraft`, `Exec=pdfcraft`, `<binary>pdfcraft</binary>` and `CARGO_BIN_EXE_pdfcraft` (not `-cli`) become `vhnoterpdf`.
   - Promotion: search for `discord`, `getartcraft`, `ArtCraft` in `crates/`, `apps/` and `packaging/`.
   - Em dashes: search shipped strings for `—`, `\u{2014}`, `&mdash;`.
5. Rebuild with `CRAFT_I18N_DIR` pointing at the catalog fork and run `cargo xtask i18n-report`: new upstream strings show as missing for `sq`.
6. Run the checks in AGENTS.md (`cargo fmt --check`, clippy, tests, `cargo xtask ci`). Tests that catch a lost fork change: `updates_come_only_from_the_fork`, `links::no_community_or_promotion_links_on_the_home_screen`, `i18n::tests::language_persists_and_invalid_input_keeps_current_language`, `albanian_needs_tesseract_and_never_falls_back_to_ocrs`, `a_missing_pack_is_named_not_replaced_by_english`.

## The catalog fork (vensih/craft-i18n)

Translations live in craft-i18n, not here. The fork's `vhnoterpdf` branch carries:

- the Albanian (`sq`) catalog, completed for this app;
- `PdfCraft` → `vhnoterPDF` in every locale's rows that name the app (these are lookup keys, so they must match the code);
- the em-dash key changes and punctuation in every locale.

To take upstream translation updates: in the craft-i18n checkout, `git fetch origin && git merge origin/main` on `vhnoterpdf`, re-run the brand and em-dash pass on new rows, translate new `sq` keys, push, then move `craft-i18n.rev` and both flatpak manifests' `commit:` to the new SHA. CI reads the SHA through `storytold/craft-i18n`, which GitHub serves for any commit in the fork network.

## Not done yet

- macOS: the DMG background still shows the lion, and the `.app` folder and DMG volume are still named PdfCraft (needs a Mac to regenerate and test).
- The Albanian strings added for this app have not been reviewed by a native speaker.
- The updater with the new `id` can only be tested end to end after the first vhnoterPDF release.

## Files changed from upstream

Branches `vhnoterpdf-albanian-rebrand` and `vhnoterpdf-exe-rename` (272 files; M modified, A added, D deleted):

<details><summary>Albanian, rebrand, links, em dashes, catalog pin, executable</summary>

```text
M	ATTRIBUTION.md
M	ATTRIBUTION.toml
M	NOTICE
M	README.md
M	apps/pdfcraft-cli/build.rs
M	apps/pdfcraft-cli/src/main.rs
M	apps/pdfcraft-cli/tests/it/render.rs
M	apps/pdfcraft-cli/tests/it/stdout.rs
M	apps/pdfcraft-cli/tests/it/version_resource.rs
M	apps/pdfcraft-ocr-worker/src/main.rs
M	apps/pdfcraft-web/src/api.rs
M	apps/pdfcraft-web/src/main.rs
M	apps/pdfcraft/Cargo.toml
M	apps/pdfcraft/build.rs
M	apps/pdfcraft/src/apple_events.rs
M	apps/pdfcraft/src/decorations.rs
M	apps/pdfcraft/src/logging.rs
M	apps/pdfcraft/src/main.rs
M	apps/pdfcraft/src/reopen.rs
M	apps/pdfcraft/src/single_instance.rs
M	apps/pdfcraft/src/startup.rs
M	apps/pdfcraft/src/tests.rs
M	apps/pdfcraft/tests/help.rs
M	apps/pdfcraft/tests/it/vendor_graph.rs
M	apps/pdfcraft/tests/it/winit_macos_emoji_picker.rs
M	apps/pdfcraft/tests/it/winit_macos_korean_first_key.rs
M	apps/pdfcraft/tests/it/winit_macos_korean_space.rs
M	apps/pdfcraft/tests/it/winit_wayland_text_input.rs
M	apps/pdfcraft/tests/it/winit_windows_hidden_maximized.rs
M	apps/pdfcraft/tests/it/winit_x11_dpi.rs
M	apps/pdfcraft/tests/it/winit_x11_ime_focus.rs
M	craft-i18n.rev
M	crates/annot/src/appearance.rs
M	crates/annot/src/appearance/symbols.rs
M	crates/annot/src/create.rs
M	crates/annot/src/draw.rs
M	crates/annot/src/links.rs
M	crates/annot/src/lookup.rs
M	crates/annot/src/modify.rs
M	crates/annot/src/tests.rs
M	crates/annot/src/text_style.rs
M	crates/annot/src/types.rs
M	crates/annot/tests/malformed.rs
M	crates/annot/tests/round_trip.rs
M	crates/annot/tests/support/fixture.rs
M	crates/automation/src/comments.rs
M	crates/automation/src/json.rs
M	crates/automation/src/lib.rs
M	crates/automation/src/mcp.rs
M	crates/automation/src/tools.rs
M	crates/automation/tests/it/automation.rs
M	crates/automation/tests/it/hostile_args.rs
M	crates/create/src/extract.rs
M	crates/create/src/lib.rs
M	crates/create/src/office.rs
M	crates/create/src/tests.rs
M	crates/crypt/tests/it/proptests.rs
M	crates/edit/src/added/layout.rs
M	crates/edit/src/added/types.rs
M	crates/edit/src/bidi.rs
M	crates/edit/src/flatten.rs
M	crates/edit/src/images.rs
M	crates/edit/src/place.rs
M	crates/edit/src/tests.rs
M	crates/edit/src/text/fallback.rs
M	crates/engine/src/actions.rs
M	crates/engine/src/catalog.rs
M	crates/engine/src/commands.rs
M	crates/engine/src/document.rs
M	crates/engine/src/edit.rs
M	crates/engine/src/edit/labels.rs
M	crates/engine/src/edit/run.rs
M	crates/engine/src/export.rs
M	crates/engine/src/js.rs
M	crates/engine/src/links.rs
M	crates/engine/src/links/tests.rs
M	crates/engine/src/ocr.rs
M	crates/engine/src/ocr/layer.rs
M	crates/engine/src/open_parameters.rs
M	crates/engine/src/scan_clean.rs
M	crates/engine/src/session/editing.rs
M	crates/engine/src/session/files.rs
M	crates/engine/src/session/save.rs
M	crates/engine/src/tests.rs
M	crates/engine/src/xfa.rs
M	crates/engine/src/xfa_form.rs
M	crates/export/src/as_docx.rs
M	crates/export/src/as_rtf.rs
M	crates/export/src/as_word/pictures.rs
M	crates/export/src/tests.rs
M	crates/fonts/src/craft.rs
M	crates/fonts/src/fallback.rs
M	crates/fonts/src/lib.rs
M	crates/fonts/src/shaped_text.rs
M	crates/forms/src/af.rs
M	crates/forms/src/appearance.rs
M	crates/forms/src/appearances.rs
M	crates/forms/src/author/look.rs
M	crates/forms/src/author/tree.rs
M	crates/forms/src/calc_order.rs
M	crates/forms/src/read.rs
M	crates/forms/src/tests.rs
M	crates/forms/src/values.rs
M	crates/geom/src/lib.rs
M	crates/js/src/model.rs
M	crates/js/src/tests.rs
M	crates/js/src/xfa/host.rs
M	crates/js/src/xfa/props.rs
M	crates/js/src/xfa/props_tests.rs
M	crates/ocr/src/lib.rs
M	crates/ocr/src/merge.rs
M	crates/ocr/src/tesseract.rs
M	crates/optimize/src/audit.rs
M	crates/optimize/src/fragments.rs
M	crates/optimize/src/images.rs
M	crates/optimize/src/samples.rs
M	crates/optimize/tests/jpeg_color_transform.rs
M	crates/optimize/tests/optimize.rs
M	crates/oracle/src/lib.rs
M	crates/organize/src/import/fields.rs
M	crates/organize/src/tests.rs
M	crates/platform/src/crypto_env.rs
M	crates/preflight/src/icc_legacy_test.rs
M	crates/print/src/font_names.rs
M	crates/print/src/spool.rs
M	crates/print/src/tests.rs
M	crates/redact/src/image.rs
M	crates/redact/src/lib.rs
M	crates/redact/src/sanitize.rs
M	crates/redact/src/tests.rs
M	crates/render/src/inspect/model.rs
M	crates/render/src/inspect/tests.rs
M	crates/render/src/raster.rs
M	crates/render/src/raster/tests.rs
M	crates/render/src/raster/tests/isolated_cmyk_groups.rs
M	crates/render/tests/content_differential.rs
M	crates/sign/src/der.rs
M	crates/sign/src/keychain.rs
M	crates/sign/src/keys.rs
M	crates/sign/src/pdf/listing.rs
M	crates/sign/src/pdf/revision_classification_tests.rs
M	crates/sign/src/pdf/signing.rs
M	crates/sign/src/pdf/types.rs
M	crates/sign/src/pdf/validate.rs
M	crates/sign/src/status_layers.rs
M	crates/sign/src/windows.rs
M	crates/sign/src/x509.rs
M	crates/sign/tests/it/chain.rs
M	crates/sign/tests/it/pdf.rs
M	crates/ui-egui/build.rs
M	crates/ui-egui/examples/properties_scenarios.rs
M	crates/ui-egui/examples/shot.rs
M	crates/ui-egui/src/app.rs
M	crates/ui-egui/src/app/links.rs
M	crates/ui-egui/src/canvas.rs
M	crates/ui-egui/src/canvas/area.rs
M	crates/ui-egui/src/canvas/bars.rs
M	crates/ui-egui/src/canvas/image_pick.rs
M	crates/ui-egui/src/canvas/tests.rs
M	crates/ui-egui/src/chrome.rs
M	crates/ui-egui/src/dialogs.rs
M	crates/ui-egui/src/dialogs/appearance.rs
M	crates/ui-egui/src/dialogs/comment_props.rs
M	crates/ui-egui/src/dialogs/credits.rs
M	crates/ui-egui/src/dialogs/preferences.rs
M	crates/ui-egui/src/dialogs/standards_ui.rs
M	crates/ui-egui/src/editing/close.rs
M	crates/ui-egui/src/home.rs
M	crates/ui-egui/src/hooks.rs
M	crates/ui-egui/src/hotbar.rs
M	crates/ui-egui/src/i18n.rs
M	crates/ui-egui/src/i18n/tests.rs
M	crates/ui-egui/src/lib.rs
M	crates/ui-egui/src/menus.rs
M	crates/ui-egui/src/panels/comments_panel.rs
M	crates/ui-egui/src/panels/format_text_ui.rs
M	crates/ui-egui/src/panels/organize_panel.rs
M	crates/ui-egui/src/panels/right.rs
M	crates/ui-egui/src/panels/tools.rs
M	crates/ui-egui/src/persist/last_session.rs
M	crates/ui-egui/src/persist/recovery.rs
M	crates/ui-egui/src/persist/window_state.rs
M	crates/ui-egui/src/platform/file_watch.rs
M	crates/ui-egui/src/platform/open_worker/tests.rs
M	crates/ui-egui/src/platform/portable.rs
M	crates/ui-egui/src/platform/ui_scale.rs
M	crates/ui-egui/src/platform/updates.rs
M	crates/ui-egui/src/script_dialog.rs
M	crates/ui-egui/src/theme.rs
M	crates/ui-egui/src/tools/comments/geometry.rs
M	crates/ui-egui/src/tools/content_ui.rs
M	crates/ui-egui/src/tools/field_table_ui.rs
M	crates/ui-egui/src/tools/fill_sign.rs
M	crates/ui-egui/src/tools/fill_sign/scan.rs
M	crates/ui-egui/src/tools/link_ui.rs
M	crates/ui-egui/src/tools/sign_ui.rs
M	crates/ui-egui/src/tools/sign_ui/dialog.rs
M	crates/ui-egui/src/widgets.rs
M	crates/ui-egui/src/widgets/icons.rs
M	crates/ui-egui/src/workflows/actions_ui.rs
M	crates/ui-egui/src/workflows/combine_ui/session.rs
M	crates/ui-egui/src/workflows/combine_ui/table.rs
M	crates/ui-egui/src/workflows/files.rs
M	crates/ui-egui/src/workflows/js_ui.rs
M	crates/ui-egui/src/workflows/ocr_ui.rs
M	crates/ui-egui/src/workflows/ocr_verify.rs
M	crates/ui-egui/src/workflows/print_ui/body.rs
M	crates/ui-egui/src/workflows/web_print.rs
M	crates/ui-egui/tests/it/accessibility.rs
M	crates/ui-egui/tests/it/attachment_row.rs
M	crates/ui-egui/tests/it/buttons.rs
M	crates/ui-egui/tests/it/commands.rs
M	crates/ui-egui/tests/it/comments.rs
M	crates/ui-egui/tests/it/content.rs
M	crates/ui-egui/tests/it/control.rs
M	crates/ui-egui/tests/it/editing.rs
M	crates/ui-egui/tests/it/fill_sign.rs
M	crates/ui-egui/tests/it/fonts.rs
M	crates/ui-egui/tests/it/javascript.rs
M	crates/ui-egui/tests/it/links.rs
M	crates/ui-egui/tests/it/open_action.rs
M	crates/ui-egui/tests/it/pickers.rs
M	crates/ui-egui/tests/it/recent_files.rs
M	crates/ui-egui/tests/it/signing.rs
M	crates/ui-egui/tests/it/ui.rs
M	crates/ui-egui/tests/it/updates.rs
M	crates/ui-egui/tests/it/view.rs
M	crates/ui-egui/tests/it/viewer.rs
M	crates/ui-egui/tests/snapshots/panel_comments.txt
M	crates/xfa/src/data/values.rs
M	crates/xfa/src/fixtures.rs
M	crates/xfa/src/lib.rs
M	crates/xfa/src/pdf.rs
M	crates/xfa/src/pdf/strip.rs
M	crates/xfa/src/script.rs
M	crates/xfa/src/tests.rs
M	crates/xfdf/src/fdf.rs
M	crates/xfdf/src/tests.rs
M	crates/xfdf/src/tests_format.rs
M	crates/xfdf/src/xfdf.rs
A	docs/fork-maintenance.md
M	docs/localization.md
M	packaging/freebsd/package.sh
M	packaging/icons.sh
M	packaging/linux/ai.storyteller.pdfcraft.desktop
M	packaging/linux/ai.storyteller.pdfcraft.metainfo.xml.in
M	packaging/linux/flatpak-bundle.sh
M	packaging/linux/flatpak/ai.storyteller.pdfcraft.bundle.yml
M	packaging/linux/flatpak/ai.storyteller.pdfcraft.yml
M	packaging/linux/nfpm.yaml
M	packaging/linux/package.sh
M	packaging/macos/Credits.rtf
M	packaging/macos/Info.plist.in
M	packaging/macos/package.sh
A	packaging/windows/PORTABLE-README.md
M	packaging/windows/install-policy.json
M	packaging/windows/install-policy.wxi
M	packaging/windows/package.ps1
M	packaging/windows/pdfcraft.wxs
M	packaging/windows/portable.txt
M	packaging/windows/security-audit.ps1
M	packaging/windows/sign.ps1
M	packaging/windows/test-msi.ps1
M	xtask/src/assets.rs
M	xtask/src/demo_pdf.rs
M	xtask/src/demo_pdf/annots.rs
M	xtask/src/demo_pdf/form.rs
M	xtask/src/demo_pdf/postprocess.rs
M	xtask/src/macos_integration.rs
M	xtask/src/main.rs
M	xtask/src/screenshots.rs
M	xtask/src/web.rs
```

</details>

Branch `icons-v-mark` (26 files):

<details><summary>Icons</summary>

```text
M	ATTRIBUTION.md
M	ATTRIBUTION.toml
M	LICENSES.toml
M	NOTICE
M	README.md
M	assets/app-icon/LICENSE-document.txt
M	assets/app-icon/LICENSE.txt
M	assets/app-icon/README.md
M	assets/app-icon/hicolor/128x128/apps/ai.storyteller.pdfcraft.png
M	assets/app-icon/hicolor/16x16/apps/ai.storyteller.pdfcraft.png
M	assets/app-icon/hicolor/24x24/apps/ai.storyteller.pdfcraft.png
M	assets/app-icon/hicolor/256x256/apps/ai.storyteller.pdfcraft.png
M	assets/app-icon/hicolor/32x32/apps/ai.storyteller.pdfcraft.png
M	assets/app-icon/hicolor/48x48/apps/ai.storyteller.pdfcraft.png
M	assets/app-icon/hicolor/512x512/apps/ai.storyteller.pdfcraft.png
M	assets/app-icon/hicolor/64x64/apps/ai.storyteller.pdfcraft.png
M	assets/app-icon/hicolor/scalable/apps/ai.storyteller.pdfcraft.svg
M	assets/app-icon/pdfcraft-1024.png
M	assets/app-icon/pdfcraft-document.ico
D	assets/app-icon/pdfcraft-document.svg
D	assets/app-icon/pdfcraft-small.svg
M	assets/app-icon/pdfcraft.icns
M	assets/app-icon/pdfcraft.ico
A	assets/app-icon/v-file.svg
A	assets/app-icon/v-mark.svg
M	packaging/icons.sh
```

</details>
