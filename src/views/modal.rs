//! [`iced::Element`]s to be used in the modal widget

use iced::{widget::container, Element};

use crate::{domain::AppProcessor, events::AppEvent, kinds::ModalView, views};

pub fn get_modal_based_modalview(app: &AppProcessor) -> Element<AppEvent> {
    let body = match &app.modal.modal_view {
        ModalView::SaleProductAddEditForm(product_info, is_edit) => {
            views::sale::product_to_add_form(
                product_info,
                app.sale_controller.user_input.amount.to_string(),
                *is_edit,
            )
        }
        ModalView::LoanSaleDetails(products) => views::loan::loan_sale_details(products),
        ModalView::LoanPayments(payments) => {
            views::loan::payments_loan(payments, &app.loan_controller.data.loan_payment)
        }
    };

    container(body).into()
}
