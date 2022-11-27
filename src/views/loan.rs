//! [`iced::Element`]s to be used in the loan info view

use chrono::{Datelike, Month};
use iced::{
    widget::{button, column, row, scrollable, text, text_input},
    Alignment, Element, Length,
};
use iced_aw::{Card, DatePicker, Modal};
use num_traits::FromPrimitive;
use sqlx::postgres::types::PgMoney;

use crate::{
    constants::{
        COLUMN_PADDING, SIZE_TEXT, SIZE_TEXT_INPUT, SIZE_TEXT_LABEL, SPACE_COLUMNS, SPACE_ROWS,
        TO_DECIMAL_DIGITS,
    },
    controllers::loan::LoanData,
    helpers::{get_btn_check_icon, get_btn_plus_icon, get_btn_trash_icon},
    kinds::{AppEvents, LoanDatePicker, LoanInputs, LoanModal},
    models::{loan::LoanPayment, sale::ProductSale},
    style::btns::get_style_btn_listed_items,
};

/// Groups the different views, loan info has
#[derive(Default)]
pub struct LoanView {}

impl LoanView {
    /// View to search specific loans and list them
    pub fn search_results<'a>(
        loan_data: &'a LoanData,
        modal: &'a LoanModal,
        produts_sale: &'a [ProductSale],
    ) -> Element<'a, AppEvents> {
        let search_btn = get_btn_check_icon().on_press(AppEvents::LoanSearchRequested);

        let mut list_loans = column!()
            .align_items(Alignment::Start)
            .padding(COLUMN_PADDING)
            .spacing(20);

        for loan in loan_data.loans.iter() {
            list_loans = list_loans.push(
                row!(
                    button(
                        text(format!(
                            "{name_debtor} ({date}): ${money_amount}",
                            name_debtor = loan.name_debtor,
                            date = loan.sold_at.date(),
                            money_amount = loan.loan_balance.to_bigdecimal(TO_DECIMAL_DIGITS),
                        ))
                        .size(SIZE_TEXT),
                    )
                    .on_press(AppEvents::LoanShowLoanSale(loan.id))
                    .style(get_style_btn_listed_items())
                    .width(Length::Fill),
                    get_btn_plus_icon().on_press(AppEvents::LoanShowPaymentsDetails(loan.id))
                )
                .spacing(SPACE_ROWS),
            );
        }

        let content = column!(
            row!(
                DatePicker::new(
                    loan_data.widgets_states.show_start_date,
                    loan_data.search_info.start_date,
                    button(text(loan_data.search_info.start_date).size(SIZE_TEXT_LABEL)).on_press(
                        AppEvents::LoanShowDatePicker(true, LoanDatePicker::StartDatePicker,)
                    ),
                    AppEvents::LoanShowDatePicker(false, LoanDatePicker::StartDatePicker),
                    |date| AppEvents::LoanSubmitDatePicker(date, LoanDatePicker::StartDatePicker),
                ),
                DatePicker::new(
                    loan_data.widgets_states.show_end_date,
                    loan_data.search_info.end_date,
                    button(text(loan_data.search_info.end_date).size(SIZE_TEXT_LABEL)).on_press(
                        AppEvents::LoanShowDatePicker(true, LoanDatePicker::EndDatePicker,)
                    ),
                    AppEvents::LoanShowDatePicker(false, LoanDatePicker::EndDatePicker),
                    |date| AppEvents::LoanSubmitDatePicker(date, LoanDatePicker::EndDatePicker),
                )
            )
            .spacing(SPACE_ROWS),
            row!(
                text_input("", &loan_data.search_info.client, |input_value| {
                    AppEvents::LoanInputChanged(input_value, LoanInputs::DebtorNameLike)
                })
                .on_submit(AppEvents::LoanSearchRequested)
                .size(SIZE_TEXT_INPUT),
                search_btn,
                get_btn_trash_icon().on_press(AppEvents::LoanClearLoanViewData),
            )
            .spacing(SPACE_ROWS),
            scrollable(list_loans).height(Length::Fill)
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .padding(COLUMN_PADDING)
        .spacing(SPACE_COLUMNS)
        .align_items(Alignment::Center);

        let card = move || {
            Card::new(
                "detalles",
                match modal {
                    LoanModal::LoanPayments => LoanView::get_modal_body_payments_loan(
                        &loan_data.payments_loan,
                        &loan_data.loan_payment,
                    ),
                    LoanModal::LoanSale => LoanView::get_modal_loan_sale(produts_sale),
                },
            )
            .max_width(500)
            .max_height(300)
            .into()
        };

        Modal::new(loan_data.widgets_states.show_modal, content, card)
            .backdrop(AppEvents::LoanCloseModalPaymentsLoan)
            .on_esc(AppEvents::LoanCloseModalPaymentsLoan)
            .into()
    }

    /// Payments made to a loan and make a new one
    fn get_modal_body_payments_loan<'a>(
        payments: &[LoanPayment],
        loan_payment: &str,
    ) -> Element<'a, AppEvents> {
        let mut list_payments = column!().spacing(SPACE_COLUMNS);

        for payment in payments {
            let date = payment.payed_at.date();
            let format_date = format!(
                "{day}-{month}-{year}",
                day = date.day(),
                month = Month::from_u32(date.month())
                    .unwrap_or(Month::December)
                    .name(),
                year = date.year()
            );

            list_payments = list_payments.push(
                text(format!(
                    "${money_amount}: {date}",
                    money_amount = payment.money_amount.to_bigdecimal(TO_DECIMAL_DIGITS),
                    date = format_date,
                ))
                .size(SIZE_TEXT),
            );
        }

        column!(
            row!(
                text_input("", loan_payment, |input_value| {
                    AppEvents::LoanInputChanged(input_value, LoanInputs::PaymentLoanAmount)
                })
                .on_submit(AppEvents::LoanAddNewPaymentToLoan)
                .size(SIZE_TEXT),
                get_btn_plus_icon().on_press(AppEvents::LoanAddNewPaymentToLoan)
            )
            .spacing(SPACE_ROWS),
            scrollable(list_payments.width(Length::Fill)),
        )
        .spacing(SPACE_COLUMNS)
        .into()
    }

    /// Loan's sale details
    fn get_modal_loan_sale(produts: &[ProductSale]) -> Element<AppEvents> {
        let mut total = PgMoney(0);

        let mut products_container = column!().padding(COLUMN_PADDING);

        for product in produts {
            products_container =
                products_container.push(text(&product.product_name).size(SIZE_TEXT));

            products_container = products_container.push(
                text(format!(
                    " {amount}=${charge}",
                    amount = product.amount_description,
                    charge = product.charge.to_bigdecimal(TO_DECIMAL_DIGITS),
                ))
                .size(SIZE_TEXT_LABEL),
            );

            total += product.charge;
        }

        column!(
            scrollable(products_container),
            text(format!(
                "Total: ${}",
                total.to_bigdecimal(TO_DECIMAL_DIGITS)
            ))
            .size(SIZE_TEXT),
        )
        .spacing(SPACE_COLUMNS)
        .width(Length::Fill)
        .into()
    }
}
