//! [`iced::Element`]s to be used in the loan view
//!
use iced::{
    widget::{button, column, row, scrollable, text, text_input, Column},
    Element, Length,
};
use sqlx::postgres::types::PgMoney;
use std::collections::BTreeMap;

use crate::{
    constants::{
        COLUMN_PADDING, SIZE_TEXT, SIZE_TEXT_INPUT, SIZE_TEXT_LABEL, SPACE_COLUMNS, SPACE_ROWS,
        TO_DECIMAL_DIGITS,
    },
    events::AppEvent,
    helpers::get_btn_plus_icon,
    kinds::TextInput,
    models::{
        loan::{LoanInfo, LoanItem, LoanPayment},
        sale::ProductSale,
    },
    views::style::btns::get_style_btn_listed_items,
};

/// View to search specific loans and list them
pub fn search_results<'a>(
    loans_by_debtor: &'a BTreeMap<String, LoanInfo>,
    debtor_name: &'a str,
) -> Element<'a, AppEvent> {
    let loans: Vec<Element<AppEvent>> = loans_by_debtor
        .iter()
        .map(|(name_debtor, loan_info)| {
            column!(
                text(format!(
                    "{} ${}",
                    name_debtor,
                    loan_info.total.to_bigdecimal(TO_DECIMAL_DIGITS)
                ))
                .size(SIZE_TEXT),
                Column::with_children(
                    loan_info
                        .loans
                        .iter()
                        .map(|loan: &LoanItem| {
                            row!(
                                button(
                                    text(format!(
                                        "{name_debtor} ({date}): ${money_amount}",
                                        name_debtor = loan.name_debtor,
                                        date = loan.sold_at.date(),
                                        money_amount =
                                            loan.loan_balance.to_bigdecimal(TO_DECIMAL_DIGITS),
                                    ))
                                    .size(SIZE_TEXT),
                                )
                                .on_press(AppEvent::LoanShowLoanSale(loan.id))
                                .style(get_style_btn_listed_items())
                                .width(Length::Fill),
                                get_btn_plus_icon()
                                    .on_press(AppEvent::LoanShowPaymentsDetails(loan.id))
                            )
                            .spacing(SPACE_ROWS)
                            .padding(SPACE_COLUMNS)
                            .into()
                        })
                        .collect::<Vec<Element<AppEvent>>>()
                )
            )
            .into()
        })
        .collect();

    let list_loans = Column::with_children(loans)
        .padding(COLUMN_PADDING)
        .spacing(SPACE_COLUMNS);

    column!(
        row!(text_input("", debtor_name)
            .on_input(|input_value| {
                AppEvent::TextInputChanged(input_value, TextInput::LoanDebtorName)
            })
            .on_submit(AppEvent::LoanSearchRequested)
            .size(SIZE_TEXT_INPUT),)
        .spacing(SPACE_ROWS),
        scrollable(list_loans).height(Length::Fill)
    )
    .padding(COLUMN_PADDING)
    .into()
}

/// Show the loan's sale details
pub fn loan_sale_details(products: &[ProductSale]) -> Element<AppEvent> {
    let mut total = PgMoney(0);

    let products: Vec<Element<AppEvent>> = products
        .iter()
        .map(|product| {
            total += product.charge;

            column!(
                text(&product.product_name).size(SIZE_TEXT),
                text(format!(
                    " {amount}=${charge}",
                    amount = product.amount_description,
                    charge = product.charge.to_bigdecimal(TO_DECIMAL_DIGITS),
                ))
                .size(SIZE_TEXT_LABEL)
            )
            .into()
        })
        .collect();

    column!(
        scrollable(Column::with_children(products)),
        text(format!(
            "Total: ${}",
            total.to_bigdecimal(TO_DECIMAL_DIGITS)
        ))
        .size(SIZE_TEXT),
    )
    .padding(COLUMN_PADDING)
    .spacing(SPACE_COLUMNS)
    .into()
}

/// Show all the loan's payments
pub fn payments_loan<'a>(payments: &[LoanPayment], loan_payment: &str) -> Element<'a, AppEvent> {
    let payments: Vec<Element<AppEvent>> = payments
        .iter()
        .map(|payment| {
            text(format!(
                "${money_amount}: {date}",
                money_amount = payment.money_amount.to_bigdecimal(TO_DECIMAL_DIGITS),
                date = payment.payed_at.date(),
            ))
            .size(SIZE_TEXT)
            .into()
        })
        .collect();

    let payments = scrollable(Column::with_children(payments).spacing(SPACE_COLUMNS));

    column!(
        row!(
            text_input("", loan_payment)
                .on_input(|input_value| {
                    AppEvent::TextInputChanged(input_value, TextInput::LoanPaymentAmountLoan)
                })
                .on_submit(AppEvent::LoanAddNewPaymentToLoan)
                .width(Length::Fixed(300.0))
                .size(SIZE_TEXT),
            get_btn_plus_icon().on_press(AppEvent::LoanAddNewPaymentToLoan)
        )
        .spacing(SPACE_ROWS),
        payments,
    )
    .padding(COLUMN_PADDING)
    .spacing(SPACE_COLUMNS)
    .into()
}
