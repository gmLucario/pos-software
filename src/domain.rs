use iced::{
    button, executor, widget::Container, Alignment, Application, Button, Column, Command, Element,
    Row, Text,
};

use crate::constants::{
    CATALOG_BTN_MSG, SALES_INFO_BTN_MSG, SALE_BTN_MSG, SIZE_BTNS_TEXT, SPACE_COLUMNS, SPACE_ROWS,
    TO_BUY_BTN_MSG, WINDOW_TITTLE,
};
use crate::data::catalog::CatalogRepo;
use crate::db::Db;
use crate::kinds::{AppEvents, Views};
use crate::schemas::catalog::ProductsToBy;
use crate::views::{catalog, sale, sales_info, to_buy};

pub struct App {
    pub catalog_btn: button::State,
    pub sale_btn: button::State,
    pub sales_info_btn: button::State,
    pub to_buy_btn: button::State,

    pub current_view: Views,
    pub catalog_view: catalog::Catalog,
    pub sale_view: sale::Sale,
    pub sales_info_view: sales_info::SalesInfo,
    pub to_buy_view: to_buy::ToBuy,

    pub products_to_buy: Vec<ProductsToBy>,
}

impl Application for App {
    type Message = AppEvents;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                catalog_btn: button::State::new(),
                sale_btn: button::State::new(),
                sales_info_btn: button::State::new(),
                to_buy_btn: button::State::new(),

                current_view: Views::Sale,
                catalog_view: catalog::Catalog::new(),
                sale_view: sale::Sale::new(),
                sales_info_view: sales_info::SalesInfo::new(),
                to_buy_view: to_buy::ToBuy::new(),

                products_to_buy: vec![],
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        WINDOW_TITTLE.to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        let db_connection = &Db::global().connection;

        match message {
            AppEvents::ShowToBuy => Command::perform(
                CatalogRepo::get_products_to_buy(db_connection),
                AppEvents::ToBuyData,
            ),
            AppEvents::ToBuyData(result) => {
                self.current_view = Views::ToBuy;
                match result {
                    Err(_) => (),
                    Ok(to_buy) => self.products_to_buy = to_buy,
                }
                Command::none()
            }
            AppEvents::ShowSale => {
                self.current_view = Views::Sale;
                Command::none()
            }
            AppEvents::ShowSalesInfo => {
                self.current_view = Views::SalesInfo;
                Command::none()
            }
            AppEvents::ShowCatalog => {
                self.current_view = Views::Catalog;
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let catalog_btn = Button::new(
            &mut self.catalog_btn,
            Text::new(CATALOG_BTN_MSG).size(SIZE_BTNS_TEXT),
        )
        .on_press(AppEvents::ShowCatalog);
        let sale_btn = Button::new(
            &mut self.sale_btn,
            Text::new(SALE_BTN_MSG).size(SIZE_BTNS_TEXT),
        )
        .on_press(AppEvents::ShowSale);
        let sales_info_btn = Button::new(
            &mut self.sales_info_btn,
            Text::new(SALES_INFO_BTN_MSG).size(SIZE_BTNS_TEXT),
        )
        .on_press(AppEvents::ShowSalesInfo);
        let to_buy_btn = Button::new(
            &mut self.to_buy_btn,
            Text::new(TO_BUY_BTN_MSG).size(SIZE_BTNS_TEXT),
        )
        .on_press(AppEvents::ShowToBuy);
        let box_buttons_next_views = Row::new()
            .spacing(SPACE_ROWS)
            .push(catalog_btn)
            .push(sale_btn)
            .push(sales_info_btn)
            .push(to_buy_btn);

        let col = Column::new()
            .spacing(SPACE_COLUMNS)
            .align_items(Alignment::Center)
            .push(box_buttons_next_views);

        let content = match self.current_view {
            Views::Sale => self.sale_view.view(),
            Views::SalesInfo => self.sales_info_view.view(),
            Views::ToBuy => self.to_buy_view.view(&self.products_to_buy),
            Views::Catalog => self.catalog_view.view(),
        };

        let col = col.push(content);

        Container::new(col)
            .center_x()
            .center_y()
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}
