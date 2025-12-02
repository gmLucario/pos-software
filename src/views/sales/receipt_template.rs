//! Receipt PDF generation with Typst templates

use crate::models::{Operation, Sale};
use crate::utils::formatting::format_currency;
use crate::utils::pdf::{compile_typst_to_pdf, escape_typst};
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "receipt.typ.stpl")]
struct ReceiptTemplate {
    receipt_id: String,
    date: String,
    items_count: usize,
    items: Vec<ReceiptItem>,
    total: String,
    paid: String,
    change_amount: Option<String>,
    amount_owed: Option<String>,
}

struct ReceiptItem {
    product_name: String,
    quantity: String,
    price: String,
    subtotal: String,
}

pub fn generate_receipt_pdf(
    sale: &Sale,
    operations: &[Operation],
    formatted_date: &str,
    file_path: std::path::PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    // Prepare template data
    let items: Vec<ReceiptItem> = operations
        .iter()
        .map(|op| ReceiptItem {
            product_name: escape_typst(&op.product_name),
            quantity: format!("{:.3}", op.quantity),
            price: escape_typst(&format_currency(op.unit_price)),
            subtotal: escape_typst(&format_currency(op.subtotal)),
        })
        .collect();

    let change_amount = if sale.change_amount > rust_decimal::Decimal::ZERO {
        Some(escape_typst(&format_currency(sale.change_amount)))
    } else {
        None
    };

    let amount_owed = if sale.is_loan {
        Some(escape_typst(&format_currency(
            sale.total_amount - sale.paid_amount,
        )))
    } else {
        None
    };

    let template = ReceiptTemplate {
        receipt_id: escape_typst(&sale.id),
        date: escape_typst(formatted_date),
        items_count: operations.len(),
        items,
        total: escape_typst(&format_currency(sale.total_amount)),
        paid: escape_typst(&format_currency(sale.paid_amount)),
        change_amount,
        amount_owed,
    };

    let typst_content = template.render_once()?;

    // Compile Typst to PDF and save
    compile_typst_to_pdf(typst_content, file_path)?;

    Ok(())
}
