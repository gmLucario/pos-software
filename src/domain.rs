use std::collections::HashMap;

use iced::{
    button, executor, Alignment, Application, Button, Column, Command, Element, Row, Subscription,
    Text,
};

use crate::{
    constants::{
        CATALOG_BTN_MSG, SALES_INFO_BTN_MSG, SALE_BTN_MSG, SIZE_BTNS_TEXT, SPACE_COLUMNS,
        SPACE_ROWS, TO_BUY_BTN_MSG, WINDOW_TITTLE,
    },
    controllers,
    data::{loan_repo::LoanRepo, product_repo::ProductRepo, sale_repo::SaleRepo},
    db::Db,
    kinds::{AppEvents, CatalogInputs, SaleInputs, UnitsMeasurement, Views},
    models::{
        self,
        sale::{Sale, SaleLoan},
    },
    schemas::{catalog::LoadProduct, sale::ProductToAdd},
    views::sales_info,
};

pub struct App {
    pub catalog_btn: button::State,
    pub sale_btn: button::State,
    pub sales_info_btn: button::State,
    pub to_buy_btn: button::State,

    pub current_view: Views,
    pub catalog_controller: controllers::catalog::Catalog,
    pub sale_controller: controllers::sale::Sale,
    pub sales_info_view: sales_info::SalesInfo,
    pub to_buy_controller: controllers::to_buy::ToBuy,

    pub listen_barcode_device: bool,
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
                catalog_controller: controllers::catalog::Catalog::new(),
                sale_controller: controllers::sale::Sale::default(),
                sales_info_view: sales_info::SalesInfo::new(),
                to_buy_controller: controllers::to_buy::ToBuy::new(),

                listen_barcode_device: false,
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
                    Err(err) => eprintln!("{err}"),
                    Ok(to_buy) => self.to_buy_controller.products = to_buy,
                }
                Command::none()
            }
            AppEvents::ShowSalesInfo => {
                self.current_view = Views::SalesInfo;
                Command::none()
            }
            AppEvents::ShowSale => {
                self.listen_barcode_device = true;
                self.current_view = Views::Sale;
                Command::none()
            }
            AppEvents::SaleProductInfoRequested(result) => {
                match result {
                    Err(err) => eprintln!("{err}"),
                    Ok(record) => match record {
                        Some(data) => {
                            let unit = UnitsMeasurement::from(data.unit_measurement_id);
                            self.sale_controller.product_to_add = ProductToAdd::from(data);

                            match unit {
                                UnitsMeasurement::Pieces => {
                                    self.sale_controller.add_new_product_to_sale();

                                    self.current_view = Views::Sale;
                                    self.listen_barcode_device = true;
                                    self.sale_controller.product_to_add.reset_values();
                                }
                                _ => {
                                    self.listen_barcode_device = false;
                                    self.current_view = Views::SaleAddProductForm;
                                }
                            }
                        }
                        None => self.sale_controller.product_to_add.barcode.clear(),
                    },
                }
                Command::none()
            }
            AppEvents::SaleInputChanged(input_value, input_type) => {
                match input_type {
                    SaleInputs::AmountProduct if input_value.parse::<f64>().is_ok() => {
                        self.sale_controller.product_to_add.amount = input_value;
                    }
                    SaleInputs::UserPay if input_value.parse::<f64>().is_ok() => {
                        self.sale_controller.sale_info.client_pay = input_value;
                        self.sale_controller.calculate_payback_money();
                    }
                    SaleInputs::ClientName => {
                        self.sale_controller.sale_info.client_name = input_value;
                    }
                    _ => (),
                }

                Command::none()
            }
            AppEvents::SaleNewProductCancel => {
                self.current_view = Views::Sale;

                self.listen_barcode_device = true;
                self.sale_controller.reset_sale_form_values();
                Command::none()
            }
            AppEvents::SaleNewProductOk => {
                self.sale_controller.add_new_product_to_sale();

                self.current_view = Views::Sale;
                self.listen_barcode_device = true;
                self.sale_controller.product_to_add.reset_values();

                Command::none()
            }
            AppEvents::SaleProductsToBuyCancel => {
                self.current_view = Views::Sale;
                self.sale_controller.sale_info.products.clear();
                Command::none()
            }
            AppEvents::SaleProductsToBuyOk => {
                self.current_view = Views::SaleChargeForm;
                Command::none()
            }
            AppEvents::SaleRemoveProductToBuyList(id) => {
                self.sale_controller.sale_info.products.remove(&id);
                Command::none()
            }
            AppEvents::SaleCreateNewSale => Command::perform(
                SaleRepo::process_new_sale_flow(
                    db_connection,
                    Sale::from(&self.sale_controller.sale_info),
                ),
                AppEvents::SaleCreateNewSaleRequested,
            ),
            AppEvents::SaleCreateNewSaleRequested(result) => {
                self.current_view = Views::Sale;

                let next_event = match result {
                    Ok(sale_id) => {
                        let mut loan = SaleLoan::from(&self.sale_controller.sale_info);
                        loan.sale_id = sale_id;
                        Command::perform(
                            LoanRepo::save_new_loan(db_connection, loan),
                            AppEvents::SaleCreateNewSaleLoan,
                        )
                    }
                    Err(err) => {
                        eprintln!("{err}");
                        Command::none()
                    }
                };

                self.sale_controller.reset_sale_form_values();
                self.sale_controller.sale_info.products.clear();

                next_event
            }
            AppEvents::ShowCatalog => {
                self.listen_barcode_device = true;
                self.catalog_controller.reset_values();
                self.current_view = Views::Catalog;
                Command::none()
            }
            AppEvents::EventOccurred(event) if self.listen_barcode_device => {
                match self.current_view {
                    Views::Sale => self
                        .sale_controller
                        .process_barcode_input(event, db_connection),
                    Views::Catalog => self
                        .catalog_controller
                        .process_barcode_input(event, db_connection),
                    _ => Command::none(),
                }
            }
            AppEvents::CatalogProductInfoRequested(result) => {
                self.current_view = Views::CatalogAddRecord;
                self.listen_barcode_device = false;
                match result {
                    Err(err) => eprintln!("{err}"),
                    Ok(record) => match record {
                        Some(data) => {
                            self.catalog_controller.load_product = LoadProduct::from(data)
                        }
                        None => {
                            self.catalog_controller.load_product = LoadProduct {
                                barcode: self.catalog_controller.load_product.barcode.to_string(),
                                ..LoadProduct::default()
                            };
                        }
                    },
                }
                Command::none()
            }
            AppEvents::CatalogInputChanged(input_value, input_type) => {
                match input_type {
                    CatalogInputs::ProductName => {
                        self.catalog_controller.load_product.product_name = input_value;
                    }
                    CatalogInputs::AmountProduct => {
                        match self.catalog_controller.load_product.unit_measurement {
                            UnitsMeasurement::Kilograms | UnitsMeasurement::Liters
                                if input_value.parse::<f64>().is_ok() =>
                            {
                                self.catalog_controller.load_product.amount = input_value;
                            }
                            UnitsMeasurement::Pieces if input_value.parse::<u64>().is_ok() => {
                                self.catalog_controller.load_product.amount = input_value;
                            }
                            _ => (),
                        }
                    }
                    CatalogInputs::ClientPrice if input_value.parse::<f64>().is_ok() => {
                        self.catalog_controller.load_product.user_price = input_value
                    }
                    CatalogInputs::MinAmountProduct if input_value.parse::<f64>().is_ok() => {
                        self.catalog_controller.load_product.min_amount = input_value
                    }
                    CatalogInputs::CostProduct if input_value.parse::<f64>().is_ok() => {
                        self.catalog_controller.load_product.cost = input_value
                    }
                    _ => (),
                }
                Command::none()
            }
            AppEvents::CatalogNewRecordCancel => {
                self.current_view = Views::Catalog;
                self.listen_barcode_device = true;
                self.catalog_controller.reset_values();
                Command::none()
            }
            AppEvents::CatalogNewRecordOk => {
                self.current_view = Views::Catalog;
                self.catalog_controller.products_to_add.insert(
                    self.catalog_controller.load_product.get_id(),
                    self.catalog_controller.load_product.clone(),
                );
                self.listen_barcode_device = true;
                self.catalog_controller.reset_values();
                Command::none()
            }
            AppEvents::CatalogSaveAllRecords => Command::perform(
                ProductRepo::save_products_catalog(
                    db_connection,
                    self.catalog_controller
                        .products_to_add
                        .values()
                        .map(models::catalog::LoadProduct::from)
                        .collect::<Vec<models::catalog::LoadProduct>>(),
                ),
                AppEvents::CatalogNewRecordPerformed,
            ),
            AppEvents::CatalogNewRecordPerformed(result) => {
                match result {
                    Ok(_) => (),
                    Err(err) => eprintln!("{:#?}", err),
                };
                self.catalog_controller.products_to_add = HashMap::new();
                self.current_view = Views::Catalog;
                Command::none()
            }
            AppEvents::CatalogPickListSelected(unit) => {
                self.catalog_controller.load_product.unit_measurement = unit;
                self.catalog_controller.load_product.amount = "1".to_string();
                Command::none()
            }
            AppEvents::CatalogRemoveRecordList(id) => {
                self.catalog_controller.products_to_add.remove(&id);
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
            .padding(10)
            .align_items(Alignment::Center)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .push(box_buttons_next_views);

        let content = match self.current_view {
            Views::Sale => {
                self.listen_barcode_device = true;
                self.sale_controller.scan_barcodes_view()
            }
            Views::SaleAddProductForm => self.sale_controller.product_to_add_view(),
            Views::SaleChargeForm => self.sale_controller.charge_sale_view(),
            Views::SalesInfo => self.sales_info_view.view(),
            Views::ToBuy => self.to_buy_controller.view(),
            Views::Catalog => self.catalog_controller.catalog_list_view(),
            Views::CatalogAddRecord => self.catalog_controller.populate_record_view(),
        };

        let col = col.push(content);

        col.into()
    }
}
