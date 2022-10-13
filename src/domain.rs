use iced::keyboard::KeyCode;
use iced::{
    button, executor, widget::Container, Alignment, Application, Button, Column, Command, Element,
    Row, Subscription, Text,
};

use crate::constants::{
    CATALOG_BTN_MSG, CHARS_SAVED_AS_BARCODE, SALES_INFO_BTN_MSG, SALE_BTN_MSG, SIZE_BTNS_TEXT,
    SPACE_COLUMNS, SPACE_ROWS, TO_BUY_BTN_MSG, WINDOW_TITTLE,
};
use crate::controllers;
use crate::data::product_repo::ProductRepo;
use crate::db::Db;
use crate::kinds::{AppEvents, CatalogInputs, UnitsMeasurement, Views};
use crate::schemas::catalog::LoadProduct;
use crate::views::{sale, sales_info};

pub struct App {
    pub catalog_btn: button::State,
    pub sale_btn: button::State,
    pub sales_info_btn: button::State,
    pub to_buy_btn: button::State,

    pub current_view: Views,
    pub catalog_view: controllers::catalog::Catalog,
    pub sale_view: sale::Sale,
    pub sales_info_view: sales_info::SalesInfo,
    pub to_buy_view: controllers::to_buy::ToBuy,
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
                catalog_view: controllers::catalog::Catalog::new(),
                sale_view: sale::Sale::new(),
                sales_info_view: sales_info::SalesInfo::new(),
                to_buy_view: controllers::to_buy::ToBuy::new(),
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
                ProductRepo::get_products_to_buy(db_connection),
                AppEvents::ToBuyData,
            ),
            AppEvents::ToBuyData(result) => {
                self.current_view = Views::ToBuy;
                match result {
                    Err(_) => (),
                    Ok(to_buy) => self.to_buy_view.products = to_buy,
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
                self.catalog_view.reset_values();
                self.current_view = Views::Catalog;
                Command::none()
            }
            AppEvents::EventOccurred(event)
                if self.current_view == Views::Catalog
                    && self.catalog_view.listen_barcode_device =>
            {
                match event {
                    iced_native::Event::Keyboard(iced::keyboard::Event::CharacterReceived(c)) => {
                        if c.is_alphanumeric() {
                            self.catalog_view.load_product.barcode.push(c);
                        }

                        if self.catalog_view.load_product.barcode.len() > CHARS_SAVED_AS_BARCODE {
                            self.catalog_view.load_product.barcode.clear();
                        }
                    }
                    iced_native::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                        key_code: KeyCode::Enter,
                        ..
                    }) => {
                        if !self.catalog_view.load_product.barcode.is_empty() {
                            self.catalog_view.listen_barcode_device = false;
                            return Command::perform(
                                ProductRepo::get_product_info_catalog(
                                    db_connection,
                                    self.catalog_view.load_product.barcode.to_string(),
                                ),
                                AppEvents::CatalogAddRecordData,
                            );
                        }
                        self.catalog_view.load_product.barcode.clear();
                    }
                    _ => (),
                }
                Command::none()
            }
            AppEvents::CatalogAddRecordData(result) => {
                self.current_view = Views::CatalogAddRecord;
                match result {
                    Err(_) => (),
                    Ok(record) => match record {
                        Some(data) => self.catalog_view.load_product = LoadProduct::from(data),
                        None => {
                            self.catalog_view.load_product = LoadProduct {
                                barcode: self.catalog_view.load_product.barcode.to_string(),
                                ..LoadProduct::default()
                            };
                        }
                    },
                }
                Command::none()
            }
            AppEvents::InputChangedCatalog(input_value, input_type) => {
                match input_type {
                    CatalogInputs::ProductName => {
                        self.catalog_view.load_product.product_name = input_value;
                    }
                    CatalogInputs::AmountProduct => {
                        match self.catalog_view.load_product.unit_measurement {
                            UnitsMeasurement::Kilograms | UnitsMeasurement::Liters
                                if input_value.parse::<f64>().is_ok() =>
                            {
                                self.catalog_view.load_product.amount = input_value;
                            }
                            UnitsMeasurement::Pieces if input_value.parse::<u64>().is_ok() => {
                                self.catalog_view.load_product.amount = input_value;
                            }
                            _ => (),
                        }
                    }
                    CatalogInputs::ClientPrice => {
                        if input_value.parse::<f64>().is_ok() {
                            self.catalog_view.load_product.user_price = input_value
                        }
                    }
                    CatalogInputs::MinAmountProduct => {
                        if input_value.parse::<f64>().is_ok() {
                            self.catalog_view.load_product.min_amount = input_value
                        }
                    }
                }
                Command::none()
            }
            AppEvents::CatalogNewRecordCancel => {
                self.current_view = Views::Catalog;
                self.catalog_view.reset_values();
                Command::none()
            }
            AppEvents::CatalogPickListSelected(unit) => {
                self.catalog_view.load_product.unit_measurement = unit;
                self.catalog_view.load_product.amount.clear();
                Command::none()
            }
            _ => Command::none(),
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced_native::subscription::events().map(AppEvents::EventOccurred)
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
            Views::ToBuy => self.to_buy_view.view(),
            Views::Catalog => self.catalog_view.catalog_list_view(),
            Views::CatalogAddRecord => self.catalog_view.populate_record_view(),
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
