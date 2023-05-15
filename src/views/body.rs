use iced::Element;

use crate::{domain::AppProcessor, events::AppEvent, kinds::View, views};

/// Decides the gui to be shown based on the current app view
pub fn get_body_based_current_view(app: &AppProcessor) -> Element<AppEvent> {
    let body = match &app.current_view {
        View::CatalogProducts => views::catalog::show_list_products(
            &app.catalog_controller.stock_products,
            &app.catalog_controller.product_name,
        ),
        View::CatalogAddProductForm(is_edit) => {
            views::catalog::product_form(&app.catalog_controller.form, *is_edit)
        }
        View::CatalogProductsToBeAdded => {
            views::catalog::products_be_added_catalog(&app.catalog_controller.products_to_add)
        }
        View::SaleListProducts => views::sale::products_to_be_sold(
            &app.sale_controller.sale_info,
            app.sale_controller.is_pay_later(),
            app.sale_controller.is_ok_charge(),
        ),
        View::LoansByDeptor => views::loan::search_results(&app.loan_controller.data),
        View::ToBuy => views::to_buy::show_list_products(
            &app.tobuy_controller.products,
            &app.tobuy_controller.product_name,
        ),
        View::SaleInfo => views::sales_info::view(
            &app.saleinfo_controller.search_info,
            &app.saleinfo_controller.widgets_states,
            &app.saleinfo_controller.data_stats,
        ),
    };

    body
}
