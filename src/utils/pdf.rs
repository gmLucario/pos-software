//! PDF generation utilities using Typst

use std::fs;
use std::path::PathBuf;
use typst::diag::FileResult;
use typst::foundations::{Bytes, Smart};
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::World;

/// Minimal Typst World implementation for PDF compilation
struct MinimalWorld {
    library: LazyHash<typst::Library>,
    source_id: FileId,
    source: Source,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
}

impl World for MinimalWorld {
    fn library(&self) -> &LazyHash<typst::Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn main(&self) -> FileId {
        self.source_id
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.source_id {
            Ok(self.source.clone())
        } else {
            Err(typst::diag::FileError::NotFound(PathBuf::new()))
        }
    }

    fn file(&self, _id: FileId) -> FileResult<Bytes> {
        Err(typst::diag::FileError::NotFound(PathBuf::new()))
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn today(&self, _offset: Option<i64>) -> Option<typst::foundations::Datetime> {
        typst::foundations::Datetime::from_ymd(2024, 1, 1)
    }
}

/// Compile Typst content to PDF and save to file
///
/// # Arguments
/// * `typst_content` - The Typst markup string to compile
/// * `file_path` - Path where the PDF should be saved
///
/// # Returns
/// * `Ok(())` on success
/// * `Err` with error details if compilation or file writing fails
pub fn compile_typst_to_pdf(
    typst_content: String,
    file_path: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    // Debug: Log the generated Typst content
    tracing::debug!("Compiling Typst content to PDF:\n{}", typst_content);

    // Create source
    let source_id = FileId::new(None, VirtualPath::new("document.typ"));
    let source = Source::new(source_id, typst_content);

    // Load fonts - use embedded system fonts
    let font_data = include_bytes!("/System/Library/Fonts/Helvetica.ttc");
    let buffer = Bytes::from_static(font_data);
    let fonts: Vec<Font> = Font::iter(buffer).collect();
    let book = LazyHash::new(FontBook::from_fonts(&fonts));

    let world = MinimalWorld {
        library: LazyHash::new(typst::Library::default()),
        source_id,
        source,
        book,
        fonts,
    };

    // Compile the document
    let result = typst::compile(&world);
    let document = match result.output {
        Ok(doc) => doc,
        Err(errors) => {
            tracing::error!("Typst compilation failed with {} errors", errors.len());
            for (i, error) in errors.iter().enumerate() {
                tracing::error!("Error {}: {:?}", i + 1, error);
                tracing::error!("  Message: {}", error.message);
                tracing::error!("  Span: {:?}", error.span);
            }
            return Err(format!("Failed to compile Typst document: {}", errors[0].message).into());
        }
    };

    // Generate PDF
    let pdf_options = typst_pdf::PdfOptions {
        ident: Smart::Auto,
        timestamp: None,
        ..Default::default()
    };
    let pdf_result = typst_pdf::pdf(&document, &pdf_options);
    let pdf_data = match pdf_result {
        Ok(data) => data,
        Err(errors) => {
            for error in errors {
                tracing::error!("PDF generation error: {:?}", error);
            }
            return Err("Failed to generate PDF".into());
        }
    };

    // Write to file
    fs::write(&file_path, pdf_data)?;

    tracing::info!("PDF saved to: {:?}", file_path);

    Ok(())
}

/// Escape special Typst characters in text content
///
/// This escapes characters that have special meaning in Typst markup:
/// - `\` (backslash)
/// - `$` (math mode)
/// - `[` `]` (content blocks)
/// - `#` (code expressions)
/// - `` ` `` (raw text)
/// - `"` (strings)
pub fn escape_typst(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('$', "\\$")
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('#', "\\#")
        .replace('`', "\\`")
        .replace('"', "\\\"")
}
