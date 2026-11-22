# vhnoterPDF

vhnoterPDF is a PDF viewer and editor. It reads, comments on, fills in, signs, organizes,
combines, splits, protects and exports PDFs. Documents are processed on this computer.

## Portable copy

Run `vhnoterpdf.exe`. Settings, recent files and recovery data are kept in the `PdfCraftData`
folder beside it when that folder exists (portable mode); otherwise they are kept in your user
profile.

## Interface language

The interface starts in English. To use Albanian, choose **Menu > Edit > Preferences… >
Interface language > Shqip**. The choice is kept between launches.

## Albanian text recognition (OCR)

Albanian is recognized with Tesseract. The Albanian language data comes with vhnoterPDF (in
`models\`); only Tesseract itself is installed separately:

1. In **Scan & OCR**, choose **Albanian** (or **Albanian + English**) as the document language.
   The document language is independent of the interface language.
2. If Tesseract isn't installed, click **Install Tesseract…** in that dialog (it runs
   `winget install UB-Mannheim.TesseractOCR`) and approve the Windows prompt. vhnoterPDF finds it
   in `C:\Program Files\Tesseract-OCR`, on the `PATH`, or where the `PDFCRAFT_TESSERACT`
   environment variable points.

Recognition runs locally; no document data leaves the computer.

## Source code and licences

Source code: https://github.com/vensih/vhnoterPDF

vhnoterPDF is adapted from PdfCraft (https://github.com/storytold/pdfcraft). It is licensed
under the MIT License or the Apache License, Version 2.0, at your option (`LICENSE-MIT`,
`LICENSE-APACHE`). Third-party notices are in `NOTICE`.
