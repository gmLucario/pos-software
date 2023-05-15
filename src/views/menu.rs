//! [`iced::Element`]s to be used in the app menu

use iced::{
    theme,
    widget::{button, row, text, Button},
    Element,
};

use crate::{
    constants::{
        CATALOG_BTN_MSG, LOAN_BTN_MSG, SALES_INFO_BTN_MSG, SALE_BTN_MSG, SIZE_BTNS_TEXT,
        SPACE_ROWS, TO_BUY_BTN_MSG,
    },
    events::AppEvent,
    kinds::{AppModule, View},
    views::style::btns,
};

fn get_button_styled<'a>(
    label: &str,
    appmodule: &AppModule,
    current_appmodule: &AppModule,
    default_view: &View,
) -> Button<'a, AppEvent> {
    let button =
        button(text(label).size(SIZE_BTNS_TEXT)).style(if appmodule == current_appmodule {
            theme::Button::Primary
        } else {
            btns::get_style_btn_main_menu()
        });

    button.on_press(AppEvent::ChangeAppModule(*appmodule, default_view.clone()))
}

pub fn get_menu_btns<'a>(selected_appmodule: &AppModule) -> Element<'a, AppEvent> {
    row!(
        get_button_styled(
            CATALOG_BTN_MSG,
            &AppModule::Catalog,
            selected_appmodule,
            &View::CatalogProducts
        ),
        get_button_styled(
            SALE_BTN_MSG,
            &AppModule::Sale,
            selected_appmodule,
            &View::SaleListProducts
        ),
        get_button_styled(
            LOAN_BTN_MSG,
            &AppModule::Loans,
            selected_appmodule,
            &View::LoansByDeptor
        ),
        get_button_styled(
            TO_BUY_BTN_MSG,
            &AppModule::ToBuyList,
            selected_appmodule,
            &View::ToBuy
        ),
        get_button_styled(
            SALES_INFO_BTN_MSG,
            &AppModule::Stats,
            selected_appmodule,
            &View::SaleInfo
        ),
    )
    .spacing(SPACE_ROWS)
    .into()
}
