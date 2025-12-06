//! Loan Receipt PDF generation with Typst templates

use crate::models::{Loan, LoanPayment, Operation, Sale};
use crate::utils::formatting::format_currency;
use crate::utils::pdf::{compile_typst_to_pdf, escape_typst};
use chrono_tz::America::Mexico_City;
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "loan_receipt.typ.stpl")]
struct LoanReceiptTemplate {
    receipt_id: String,
    date: String,
    debtor_name: String,
    debtor_phone: Option<String>,
    items_count: usize,
    items: Vec<ReceiptItem>,
    total: String,
    payments: Vec<PaymentItem>,
    total_paid: String,
    remaining_amount: String,
    is_fully_paid: bool,
}

struct ReceiptItem {
    product_name: String,
    quantity: String,
    price: String,
    subtotal: String,
}

struct PaymentItem {
    date: String,
    amount: String,
}

pub fn generate_loan_receipt_pdf(
    loan: &Loan,
    sale: &Sale,
    operations: &[Operation],
    payments: &[LoanPayment],
    file_path: std::path::PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    // Format sale date
    let formatted_date = sale
        .sold_at
        .with_timezone(&Mexico_City)
        .format("%d-%b-%Y %H:%M")
        .to_string();

    // Prepare items data
    let items: Vec<ReceiptItem> = operations
        .iter()
        .map(|op| ReceiptItem {
            product_name: escape_typst(&op.product_name),
            quantity: format!("{:.3}", op.quantity),
            price: escape_typst(&format_currency(op.unit_price)),
            subtotal: escape_typst(&format_currency(op.subtotal)),
        })
        .collect();

    // Prepare payment history (without notes)
    let mut payment_items: Vec<PaymentItem> = Vec::new();

    // Add initial payment if any was made at the time of sale
    if sale.paid_amount > rust_decimal::Decimal::ZERO {
        let initial_payment_date = sale
            .sold_at
            .with_timezone(&Mexico_City)
            .format("%d-%b-%Y %H:%M")
            .to_string();

        payment_items.push(PaymentItem {
            date: escape_typst(&initial_payment_date),
            amount: escape_typst(&format_currency(sale.paid_amount)),
        });
    }

    // Add subsequent payments from loan_payment table
    for payment in payments {
        let payment_date = payment
            .payment_date
            .with_timezone(&Mexico_City)
            .format("%d-%b-%Y %H:%M")
            .to_string();

        payment_items.push(PaymentItem {
            date: escape_typst(&payment_date),
            amount: escape_typst(&format_currency(payment.amount)),
        });
    }

    let template = LoanReceiptTemplate {
        receipt_id: escape_typst(&sale.id),
        date: escape_typst(&formatted_date),
        debtor_name: escape_typst(&loan.debtor_name),
        debtor_phone: loan.debtor_phone.as_ref().map(|p| escape_typst(p)),
        items_count: operations.len(),
        items,
        total: escape_typst(&format_currency(sale.total_amount)),
        payments: payment_items,
        total_paid: escape_typst(&format_currency(loan.paid_amount)),
        remaining_amount: escape_typst(&format_currency(loan.remaining_amount)),
        is_fully_paid: loan.is_paid_off(),
    };

    let typst_content = template.render_once()?;

    // Compile Typst to PDF and save
    compile_typst_to_pdf(typst_content, file_path)?;

    Ok(())
}
