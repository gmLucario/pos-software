//! [`iced::Element`]s to be used in the app menu

use iced::{
    theme,
    widget::{button, column, row, text, Button},
    Element,
};

use crate::{
    constants::{
        CATALOG_BTN_MSG, LOAN_BTN_MSG, SALES_INFO_BTN_MSG, SALE_BTN_MSG, SIZE_BTNS_TEXT,
        SPACE_ROWS, TO_BUY_BTN_MSG,
    },
    events::AppEvent,
    kinds::{AppModule, View},
    views::{
        icon::{catalog_icon, loan_icon, sale_icon, sale_info_icon, tobuy_icon},
        style::btns,
    },
};

fn get_button_styled<'a>(
    label: Element<'a, AppEvent>,
    appmodule: &AppModule,
    current_appmodule: &AppModule,
    default_view: &View,
) -> Button<'a, AppEvent> {
    let button = button(label).style(if appmodule == current_appmodule {
        theme::Button::Primary
    } else {
        btns::get_style_btn_main_menu()
    });

    button.on_press(AppEvent::ChangeAppModule(*appmodule, default_view.clone()))
}

pub fn get_menu_btns<'a>(selected_appmodule: &AppModule) -> Element<'a, AppEvent> {
    let btn_container = |module: AppModule| {
        let (label, icon) = match module {
            AppModule::Catalog => (CATALOG_BTN_MSG, catalog_icon()),
            AppModule::Sale => (SALE_BTN_MSG, sale_icon()),
            AppModule::Loans => (LOAN_BTN_MSG, loan_icon()),
            AppModule::ToBuyList => (TO_BUY_BTN_MSG, tobuy_icon()),
            AppModule::Stats => (SALES_INFO_BTN_MSG, sale_info_icon()),
        };
        column![text(label).size(SIZE_BTNS_TEXT), icon.size(SIZE_BTNS_TEXT)]
            .align_items(iced::Alignment::Center)
    };

    row!(
        get_button_styled(
            btn_container(AppModule::Catalog).into(),
            &AppModule::Catalog,
            selected_appmodule,
            &View::CatalogProducts
        ),
        get_button_styled(
            btn_container(AppModule::Sale).into(),
            &AppModule::Sale,
            selected_appmodule,
            &View::SaleListProducts
        ),
        get_button_styled(
            btn_container(AppModule::Loans).into(),
            &AppModule::Loans,
            selected_appmodule,
            &View::LoansByDeptor
        ),
        get_button_styled(
            btn_container(AppModule::ToBuyList).into(),
            &AppModule::ToBuyList,
            selected_appmodule,
            &View::ToBuy
        ),
        get_button_styled(
            btn_container(AppModule::Stats).into(),
            &AppModule::Stats,
            selected_appmodule,
            &View::SaleInfo
        ),
    )
    .spacing(SPACE_ROWS)
    .into()
}
