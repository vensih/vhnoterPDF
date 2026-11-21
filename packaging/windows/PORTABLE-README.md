# vhnoterPDF

vhnoterPDF is a PDF viewer and editor. It reads, comments on, fills in, signs, organizes,
combines, splits, protects and exports PDFs. Documents are processed on this computer.

## Portable copy

Run `pdfcraft.exe`. Settings, recent files and recovery data are kept in the `PdfCraftData`
folder beside it when that folder exists (portable mode); otherwise they are kept in your user
profile.

## Interface language

The interface starts in English. To use Albanian, choose **Menu > Edit > Preferences… >
Interface language > Shqip**. The choice is kept between launches.

## Albanian text recognition (OCR)

Albanian is recognized with Tesseract, installed separately:

1. Install Tesseract 5.
2. Add the Albanian language data, `sqi.traineddata`, to Tesseract's `tessdata` folder.
3. Put `tesseract.exe` on the `PATH`, or set the `PDFCRAFT_TESSERACT` environment variable to
   its full path.
4. In **Scan & OCR**, choose **Albanian** (or **Albanian + English**) as the document language.
   The document language is independent of the interface language.

Recognition runs locally; no document data leaves the computer.

## Source code and licences

Source code: https://github.com/vensih/vhnoterPDF

vhnoterPDF is adapted from PdfCraft (https://github.com/storytold/pdfcraft). It is licensed
under the MIT License or the Apache License, Version 2.0, at your option (`LICENSE-MIT`,
`LICENSE-APACHE`). Third-party notices are in `NOTICE`.
