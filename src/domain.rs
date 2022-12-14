//! Handle main app logic and components

use std::collections::HashMap;

use iced::{
    executor, keyboard, theme,
    widget::{self, button, column, row, text},
    Alignment, Application, Command, Element, Event, Subscription,
};

use crate::{
    constants::{
        CATALOG_BTN_MSG, LOAN_BTN_MSG, SALES_INFO_BTN_MSG, SALE_BTN_MSG, SIZE_BTNS_TEXT,
        SPACE_COLUMNS, SPACE_ROWS, TO_BUY_BTN_MSG, TO_DECIMAL_DIGITS, WINDOW_TITTLE,
    },
    controllers::{self},
    data::{loan_repo::LoanRepo, product_repo::ProductRepo, sale_repo::SaleRepo},
    db::Db,
    kinds::{AppEvents, CatalogInputs, LoanInputs, LoanModal, SaleInputs, UnitsMeasurement, Views},
    models::{
        catalog::LoadProduct as ModelLoadProduct,
        sale::{Sale, SaleLoan},
    },
    schemas::{catalog::LoadProduct, sale::ProductToAdd},
    views::{catalog, loan, sale::SaleView, sales_info, to_buy},
};

#[derive(Default)]
/// Represents app modules and components
pub struct App {
    /// Current view user is interacting with
    pub current_view: Views,
    /// Controller handles Catalog logic
    pub catalog_controller: controllers::catalog::Catalog,
    /// Controller handles Sale logic
    pub sale_controller: controllers::sale::Sale,
    /// Controller handles Products to buy logic
    pub to_buy_controller: controllers::to_buy::ToBuy,
    /// Controller handles Loan details/info logic
    pub loan_info_controller: controllers::loan::Loan,
    /// Controller handles Sale info logic
    pub sale_info_controller: controllers::sale_info::SaleInfo,
}

/// Implements the traits for an interactive cross-platform application.
impl Application for App {
    type Message = AppEvents;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = theme::Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        WINDOW_TITTLE.to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        let db_connection = &Db::global().unwrap().connection;

        match message {
            AppEvents::ToBuyDataRequested => Command::perform(
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

                Command::perform(async {}, |_| AppEvents::SaleInfoSearchStats)
            }
            AppEvents::ShowSale => {
                self.current_view = Views::Sale;
                self.sale_controller.update_total_pay();
                Command::none()
            }
            AppEvents::SaleProductInfoRequested(result) => match result {
                Err(err) => {
                    eprintln!("{err}");
                    Command::none()
                }
                Ok(record) => match record {
                    Some(data) => {
                        let unit = UnitsMeasurement::from(data.unit_measurement_id);
                        self.sale_controller.product_to_add = ProductToAdd::from(data);

                        match unit {
                            UnitsMeasurement::Pieces => {
                                self.sale_controller.add_new_product_to_sale();
                                self.sale_controller.product_to_add.reset_values();

                                self.current_view = Views::Sale;
                                Command::perform(async {}, |_| AppEvents::ShowSale)
                            }
                            _ => {
                                self.current_view = Views::SaleAddProductForm;
                                Command::none()
                            }
                        }
                    }
                    None => {
                        self.sale_controller.product_to_add.reset_values();
                        Command::perform(async {}, |_| AppEvents::ShowSale)
                    }
                },
            },
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

                self.sale_controller.reset_sale_form_values();
                Command::perform(async {}, |_| AppEvents::ShowSale)
            }
            AppEvents::SaleNewProductOk => {
                self.sale_controller.add_new_product_to_sale();

                self.current_view = Views::Sale;
                self.sale_controller.product_to_add.reset_values();

                Command::perform(async {}, |_| AppEvents::ShowSale)
            }
            AppEvents::SaleProductsToBuyCancel => {
                self.current_view = Views::Sale;
                self.sale_controller.sale_info.products.clear();
                Command::perform(async {}, |_| AppEvents::ShowSale)
            }
            AppEvents::SaleProductsToBuyOk => {
                self.current_view = Views::SaleChargeForm;
                Command::none()
            }
            AppEvents::SaleRemoveProductToBuyList(id) => {
                self.sale_controller.sale_info.products.remove(&id);
                Command::perform(async {}, |_| AppEvents::ShowSale)
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
                        Command::batch(vec![
                            Command::perform(
                                LoanRepo::save_new_loan(db_connection, loan),
                                AppEvents::SaleCreateNewSaleLoan,
                            ),
                            Command::perform(async {}, |_| AppEvents::ShowSale),
                        ])
                    }
                    Err(err) => {
                        eprintln!("{err}");
                        Command::perform(async {}, |_| AppEvents::ShowSale)
                    }
                };

                self.sale_controller.reset_sale_form_values();
                self.sale_controller.sale_info.products.clear();

                next_event
            }
            AppEvents::ExternalDeviceEventOccurred(event) => match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code: keyboard::KeyCode::Tab,
                    modifiers,
                }) => {
                    if modifiers.shift() {
                        widget::focus_previous()
                    } else {
                        widget::focus_next()
                    }
                }
                _ => match self.current_view {
                    Views::Sale => self
                        .sale_controller
                        .process_barcode_input(event, db_connection),
                    Views::Catalog => self
                        .catalog_controller
                        .process_barcode_input(event, db_connection),
                    _ => Command::none(),
                },
            },
            AppEvents::ShowCatalog => {
                self.catalog_controller.reset_values();
                self.current_view = Views::Catalog;
                Command::none()
            }
            AppEvents::CatalogProductInfoRequested(result) => {
                self.current_view = Views::CatalogAddRecord;
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
                        match self.catalog_controller.load_product.unit_measurement {
                            UnitsMeasurement::Kilograms | UnitsMeasurement::Liters
                                if input_value.parse::<f64>().is_ok() =>
                            {
                                self.catalog_controller.load_product.min_amount = input_value
                            }
                            UnitsMeasurement::Pieces if input_value.parse::<u64>().is_ok() => {
                                self.catalog_controller.load_product.min_amount = input_value
                            }
                            _ => (),
                        }
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
                self.catalog_controller.reset_values();
                Command::perform(async {}, |_| AppEvents::ShowCatalog)
            }
            AppEvents::CatalogNewRecordOk => {
                self.current_view = Views::Catalog;
                self.catalog_controller.products_to_add.insert(
                    self.catalog_controller.load_product.get_id(),
                    self.catalog_controller.load_product.clone(),
                );
                self.catalog_controller.reset_values();
                Command::perform(async {}, |_| AppEvents::ShowCatalog)
            }
            AppEvents::CatalogSaveAllRecords => Command::perform(
                ProductRepo::save_products_catalog(
                    db_connection,
                    self.catalog_controller
                        .products_to_add
                        .values()
                        .map(ModelLoadProduct::from)
                        .collect::<Vec<ModelLoadProduct>>(),
                ),
                AppEvents::CatalogNewRecordPerformed,
            ),
            AppEvents::CatalogNewRecordPerformed(result) => {
                match result {
                    Ok(_) => (),
                    Err(err) => eprintln!("{:#?}", err),
                };
                self.catalog_controller.products_to_add = HashMap::new();
                Command::perform(async {}, |_| AppEvents::ShowCatalog)
            }
            AppEvents::CatalogPickListSelected(unit) => {
                self.catalog_controller.load_product.unit_measurement = unit;
                self.catalog_controller.reset_product_amounts();
                Command::none()
            }
            AppEvents::CatalogRemoveRecordList(id) => {
                self.catalog_controller.products_to_add.remove(&id);
                Command::none()
            }
            AppEvents::ShowLoanInfo => {
                self.current_view = Views::LoanInfo;
                Command::none()
            }
            AppEvents::LoanShowDatePicker(state, date_picker) => {
                self.loan_info_controller
                    .set_state_datepicker(date_picker, state);
                Command::none()
            }
            AppEvents::LoanSubmitDatePicker(date, date_picker) => {
                self.loan_info_controller
                    .set_datepicker_value(date_picker, date);
                Command::none()
            }
            AppEvents::LoanInputChanged(input_value, input_type) => {
                match input_type {
                    LoanInputs::DebtorNameLike => {
                        self.loan_info_controller.data.search_info.client = input_value;
                    }
                    LoanInputs::PaymentLoanAmount => {
                        if input_value.parse::<f64>().is_ok() {
                            self.loan_info_controller.data.loan_payment = input_value;
                        }
                    }
                }
                Command::none()
            }
            AppEvents::LoanSearchRequested => {
                let data = self.loan_info_controller.get_loan_search();

                Command::perform(
                    LoanRepo::get_loans_user_by_date_range(db_connection, data),
                    AppEvents::LoanSearchData,
                )
            }
            AppEvents::LoanClearLoanViewData => {
                self.loan_info_controller.reset_loan_data();
                Command::none()
            }
            AppEvents::LoanSearchData(data) => {
                match data {
                    Ok(loans) => {
                        self.loan_info_controller.data.loans = loans;
                    }
                    Err(err) => eprintln!("{err}"),
                }
                Command::perform(async {}, |_| AppEvents::ShowLoanInfo)
            }
            AppEvents::LoanShowPaymentsDetails(loan_id) => {
                self.loan_info_controller.data.loan_id = loan_id;
                self.loan_info_controller.data.loan_payment.clear();

                Command::perform(
                    LoanRepo::get_payments_loan(db_connection, loan_id),
                    AppEvents::LoanPaymentDetailsData,
                )
            }
            AppEvents::LoanPaymentDetailsData(result) => {
                match result {
                    Ok(payments) => {
                        self.loan_info_controller
                            .set_modal_show(LoanModal::LoanPayments);
                        self.loan_info_controller.data.payments_loan = payments;
                    }
                    Err(err) => eprintln!("{err}"),
                };
                Command::perform(async {}, |_| AppEvents::ShowLoanInfo)
            }
            AppEvents::LoanCloseModalPaymentsLoan => {
                self.loan_info_controller.hide_modal();
                self.loan_info_controller.data.payments_loan = vec![];

                Command::perform(async {}, |_| AppEvents::LoanSearchRequested)
            }
            AppEvents::LoanAddNewPaymentToLoan => {
                let payment_amount = self.loan_info_controller.get_payment_loan();

                Command::perform(
                    LoanRepo::add_new_payment_loan(
                        db_connection,
                        self.loan_info_controller.data.loan_id,
                        payment_amount,
                    ),
                    AppEvents::LoanAddNewPaymentToLoanRequested,
                )
            }
            AppEvents::LoanAddNewPaymentToLoanRequested(result) => {
                if result.is_err() {
                    eprintln!("{:#?}", result.err());
                }

                let loan_id = self.loan_info_controller.data.loan_id;
                let lambda_fn = async move { loan_id };

                Command::perform(lambda_fn, AppEvents::LoanShowPaymentsDetails)
            }
            AppEvents::LoanShowLoanSale(sale_id) => Command::perform(
                SaleRepo::get_products_sale(db_connection, sale_id),
                AppEvents::LoanSaleProductsData,
            ),
            AppEvents::LoanSaleProductsData(result) => {
                match result {
                    Ok(products) => {
                        self.loan_info_controller
                            .set_modal_show(LoanModal::LoanSale);
                        self.loan_info_controller.sale_products = products;
                    }
                    Err(err) => {
                        eprintln!("{err}")
                    }
                }

                Command::none()
            }
            AppEvents::SaleInfoShowDatePicker(state, date_picker) => {
                self.sale_info_controller
                    .set_state_datepicker(date_picker, state);
                Command::none()
            }
            AppEvents::SaleInfoSubmitDatePicker(date, date_picker) => {
                self.sale_info_controller
                    .set_datepicker_value(date_picker, date);
                Command::perform(async {}, |_| AppEvents::SaleInfoSearchStats)
            }
            AppEvents::SaleInfoSearchStats => {
                let start_date = self
                    .sale_info_controller
                    .data
                    .search_info
                    .start_date
                    .to_string();
                let end_date = self
                    .sale_info_controller
                    .data
                    .search_info
                    .end_date
                    .to_string();

                let events = vec![
                    Command::perform(
                        SaleRepo::get_total_earnings(
                            db_connection,
                            start_date.to_string(),
                            end_date.to_string(),
                        ),
                        AppEvents::SaleInfoEarningsData,
                    ),
                    Command::perform(
                        SaleRepo::get_total_sales(
                            db_connection,
                            start_date.to_string(),
                            end_date.to_string(),
                        ),
                        AppEvents::SaleInfoTotalSales,
                    ),
                    Command::perform(
                        LoanRepo::get_total_loans(db_connection, start_date, end_date),
                        AppEvents::SaleInfoTotalLoans,
                    ),
                ];

                Command::batch(events)
            }
            AppEvents::SaleInfoEarningsData(result) => {
                match result {
                    Ok(total_earnings) => {
                        self.sale_info_controller.data.data_stats.earnings =
                            total_earnings.to_bigdecimal(TO_DECIMAL_DIGITS)
                    }
                    Err(err) => eprintln!("{err}"),
                };

                Command::none()
            }
            AppEvents::SaleInfoTotalSales(result) => {
                match result {
                    Ok(totals) => {
                        self.sale_info_controller.data.data_stats.sales = totals.sales;
                        self.sale_info_controller.data.data_stats.total_sales =
                            totals.total_sales.to_bigdecimal(TO_DECIMAL_DIGITS);
                    }
                    Err(err) => eprintln!("{err}"),
                }

                Command::none()
            }
            AppEvents::SaleInfoTotalLoans(result) => {
                match result {
                    Ok(totals) => {
                        self.sale_info_controller.data.data_stats.loans = totals.loans;
                        self.sale_info_controller.data.data_stats.total_loans =
                            totals.money_loans.to_bigdecimal(TO_DECIMAL_DIGITS);
                    }
                    Err(err) => eprintln!("{err}"),
                };
                Command::none()
            }
            _ => Command::none(),
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced_native::subscription::events().map(AppEvents::ExternalDeviceEventOccurred)
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let content = match self.current_view {
            Views::Sale => SaleView::scan_barcodes_view(&self.sale_controller.sale_info),
            Views::SaleAddProductForm => {
                SaleView::product_to_add_view(&self.sale_controller.product_to_add)
            }
            Views::SaleChargeForm => SaleView::charge_sale_view(
                &self.sale_controller.sale_info,
                self.sale_controller.is_pay_later(),
                self.sale_controller.is_ok_charge(),
            ),
            Views::ToBuy => to_buy::show_list_products(&self.to_buy_controller.products),
            Views::Catalog => catalog::catalog_list_view(&self.catalog_controller.products_to_add),
            Views::CatalogAddRecord => {
                catalog::load_product_view(&self.catalog_controller.load_product)
            }
            Views::SalesInfo => sales_info::view(&self.sale_info_controller.data),
            Views::LoanInfo => loan::LoanView::search_results(
                &self.loan_info_controller.data,
                &self.loan_info_controller.modal_show,
                &self.loan_info_controller.sale_products,
            ),
        };

        column!(
            row!(
                button(text(CATALOG_BTN_MSG).size(SIZE_BTNS_TEXT))
                    .on_press(AppEvents::ShowCatalog)
                    .style(crate::style::btns::get_style_btn_main_menu()),
                button(text(SALE_BTN_MSG).size(SIZE_BTNS_TEXT))
                    .on_press(AppEvents::ShowSale)
                    .style(crate::style::btns::get_style_btn_main_menu()),
                button(text(LOAN_BTN_MSG).size(SIZE_BTNS_TEXT))
                    .on_press(AppEvents::ShowLoanInfo)
                    .style(crate::style::btns::get_style_btn_main_menu()),
                button(text(TO_BUY_BTN_MSG).size(SIZE_BTNS_TEXT))
                    .on_press(AppEvents::ToBuyDataRequested)
                    .style(crate::style::btns::get_style_btn_main_menu()),
                button(text(SALES_INFO_BTN_MSG).size(SIZE_BTNS_TEXT))
                    .on_press(AppEvents::ShowSalesInfo)
                    .style(crate::style::btns::get_style_btn_main_menu()),
            )
            .spacing(SPACE_ROWS),
            content
        )
        .spacing(SPACE_COLUMNS)
        .padding(10)
        .align_items(Alignment::Center)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
    }
}
