//! Handle all the app logic between the MVC modules and the iced framework

use crate::{
    constants::{
        ASK_FILL_CATALOG_FORM, COLUMN_PADDING, GENERAL_RETRY_MSG, NO_PRODUCT, SPACE_COLUMNS,
        STOCK_IS_EMPTY_MSG, TO_DECIMAL_DIGITS, WINDOW_TITTLE,
    },
    controllers,
    db::AppDb,
    events::AppEvent,
    helpers,
    kinds::{AppDatePicker, AppModule, ModalView, OnScroll, PickList, TextInput, View},
    models::sale::{Sale, SaleLoan},
    repo::{loan_repo, product_repo, sale_repo},
    schemas::{catalog::CatalogProductForm, sale::SaleInfo},
    views::{self, style::container::get_black_border_style},
};
use custom_crates::widgets::{
    modal::Modal,
    toast::{self, Toast},
};
use num_traits::Zero;
use std::str::FromStr;

use iced::{
    executor, keyboard, subscription,
    widget::{self, column, container, scrollable::RelativeOffset},
    Alignment, Application, Command, Element, Event, Length, Subscription,
};
use sqlx::types::BigDecimal;
use validator::Validate;

#[derive(Default)]
pub struct ModalInfo {
    pub show_modal: bool,
    pub modal_view: ModalView,
}

#[derive(Default)]
pub struct AppProcessor {
    /// Current app module user is interacting with
    pub current_appmodule: AppModule,
    /// Current app view user is interacting with
    pub current_view: View,
    /// Modal info state
    pub modal: ModalInfo,
    /// Toasts messages
    pub toasts: Vec<Toast>,

    /// Controller handles Catalog module logic
    pub catalog_controller: controllers::catalog::Catalog,
    /// Controller handles Sale module logic
    pub sale_controller: controllers::sale::Sale,
    /// Controller handles Loan module logic
    pub loan_controller: controllers::loan::Loan,
    /// Controller handles ToBuy module logic
    pub tobuy_controller: controllers::to_buy::ToBuy,
    /// Controller handles Sale Info module logic
    pub saleinfo_controller: controllers::sale_info::SaleInfo,
}

impl Application for AppProcessor {
    type Executor = executor::Default;
    type Message = AppEvent;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<AppEvent>) {
        (AppProcessor::default(), Command::none())
    }

    fn title(&self) -> String {
        WINDOW_TITTLE.into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        subscription::events().map(AppEvent::ExternalDeviceEventOccurred)
    }

    fn update(&mut self, message: AppEvent) -> Command<AppEvent> {
        let db_connection = &AppDb::get().connection;

        match message {
            // General App events
            AppEvent::AddToast(status, body) => {
                self.toasts.push(Toast {
                    title: toast::Status::get_title(status),
                    body,
                    status,
                });
                Command::none()
            }
            AppEvent::CloseToast(index) => {
                if index < self.toasts.len() {
                    self.toasts.remove(index);
                }
                Command::none()
            }
            AppEvent::ShowModal => {
                self.modal.show_modal = true;
                Command::none()
            }
            AppEvent::HideModal => {
                self.modal.show_modal = false;
                Command::none()
            }
            AppEvent::ExternalDeviceEventOccurred(event) => match event {
                Event::Keyboard(keyboard_event) => match keyboard_event {
                    keyboard::Event::KeyPressed {
                        key_code: keyboard::KeyCode::Tab,
                        modifiers,
                    } => {
                        if modifiers.shift() {
                            widget::focus_previous()
                        } else {
                            widget::focus_next()
                        }
                    }
                    keyboard::Event::KeyPressed {
                        key_code: keyboard::KeyCode::Space,
                        modifiers,
                    } if modifiers.shift() => {
                        if self.current_appmodule != AppModule::Catalog {
                            return Command::none();
                        }

                        Command::perform(async {}, |_| {
                            AppEvent::ChangeView(View::CatalogAddProductForm(false))
                        })
                    }
                    keyboard::Event::KeyPressed {
                        key_code: keyboard::KeyCode::Escape,
                        ..
                    } if self.modal.show_modal => {
                        Command::perform(async {}, |_| AppEvent::HideModal)
                    }
                    _ => match self.current_view {
                        View::SaleListProducts => self
                            .sale_controller
                            .process_barcode_input(keyboard_event, db_connection),
                        _ => Command::none(),
                    },
                },
                _ => Command::none(),
            },
            AppEvent::ChangeAppModule(appmodule, defaultview) => {
                self.current_appmodule = appmodule;
                Command::perform(async move { defaultview }, AppEvent::ChangeView)
            }
            AppEvent::ChangeModalView(modal_view) => {
                self.modal.modal_view = modal_view;
                Command::none()
            }
            AppEvent::ChangeView(app_view) => {
                self.current_view = app_view;
                Command::perform(async {}, |_| AppEvent::SetDefaultDataView)
            }
            AppEvent::SetDefaultDataView => match self.current_view {
                View::CatalogProducts => {
                    if self.catalog_controller.products_to_add.is_empty() {
                        self.catalog_controller.stock_products.clear();
                        self.catalog_controller.products_catalog_page = 0;

                        Command::perform(
                            product_repo::get_products_catalog_like(
                                db_connection,
                                self.catalog_controller.product_name_to_lowercase(),
                                self.catalog_controller.products_catalog_page,
                            ),
                            AppEvent::CatalogProductsData,
                        )
                    } else {
                        Command::perform(async {}, |_| {
                            AppEvent::ChangeView(View::CatalogProductsToBeAdded)
                        })
                    }
                }
                View::LoansByDeptor => {
                    self.loan_controller = controllers::loan::Loan::default();
                    Command::none()
                }
                View::ToBuy => Command::perform(
                    product_repo::get_products_tobuy_like(
                        db_connection,
                        self.tobuy_controller.product_name.to_string(),
                    ),
                    AppEvent::ToBuySearchData,
                ),
                View::SaleInfo => Command::perform(async {}, |_| AppEvent::SaleInfoSearchStats),
                _ => Command::none(),
            },
            AppEvent::TextInputChanged(input_value, text_input) => {
                match text_input {
                    TextInput::CatalogFilterStockList => {
                        self.catalog_controller.product_name = input_value
                    }
                    TextInput::CatalogFormBarcode => {
                        self.catalog_controller.form.barcode = input_value;
                    }
                    TextInput::CatalogFormProductName => {
                        self.catalog_controller.form.product_name = input_value
                    }
                    TextInput::CatalogFormAmountProduct
                        if helpers::is_valid_input_text_value_for_amount_data(
                            &input_value,
                            &self.catalog_controller.form.unit_measurement,
                        ) =>
                    {
                        self.catalog_controller.form.amount = input_value
                    }
                    TextInput::CatalogFormClientPrice if input_value.parse::<f64>().is_ok() => {
                        self.catalog_controller.form.user_price = input_value
                    }
                    TextInput::CatalogFormCostProduct if input_value.parse::<f64>().is_ok() => {
                        self.catalog_controller.form.cost = input_value
                    }
                    TextInput::CatalogFormMinAmountProduct
                        if helpers::is_valid_input_text_value_for_amount_data(
                            &input_value,
                            &self.catalog_controller.form.unit_measurement,
                        ) =>
                    {
                        self.catalog_controller.form.min_amount = input_value
                    }
                    TextInput::SaleFormProductAmount(unit)
                        if helpers::is_valid_input_text_value_for_amount_data(
                            &input_value,
                            &unit,
                        ) =>
                    {
                        self.sale_controller.user_input.amount = input_value
                    }
                    TextInput::SaleUserPayment if input_value.parse::<f64>().is_ok() => {
                        self.sale_controller.sale_info.client_pay = input_value;
                        self.sale_controller.calculate_payback_money();
                    }
                    TextInput::SaleClientNameLoan => {
                        self.sale_controller.sale_info.client_name = input_value
                    }
                    TextInput::LoanDebtorName => {
                        self.loan_controller.data.debtor_name = input_value
                    }
                    TextInput::LoanPaymentAmountLoan if input_value.parse::<f64>().is_ok() => {
                        self.loan_controller.data.loan_payment = input_value
                    }
                    TextInput::ToBuyProductLike => self.tobuy_controller.product_name = input_value,
                    _ => (),
                }
                Command::none()
            }
            AppEvent::PickListSelected(pick_list) => {
                match pick_list {
                    PickList::CatalogFormPickListUnitMeasurement(unit) => {
                        self.catalog_controller.form.unit_measurement = unit;
                        self.catalog_controller.reset_product_amounts();
                    }
                };
                Command::none()
            }
            AppEvent::ShowDatePicker(to_show, date_picker) => {
                match date_picker {
                    AppDatePicker::SaleStartDatePicker => {
                        self.saleinfo_controller.show_start_date(to_show)
                    }
                    AppDatePicker::SaleEndDatePicker => {
                        self.saleinfo_controller.show_end_date(to_show)
                    }
                }
                Command::none()
            }
            AppEvent::SubmitDatePicker(value, date_picker) => {
                match date_picker {
                    AppDatePicker::SaleStartDatePicker => {
                        self.saleinfo_controller.set_start_date_value(value)
                    }
                    AppDatePicker::SaleEndDatePicker => {
                        self.saleinfo_controller.set_end_date_value(value)
                    }
                }
                Command::batch(vec![
                    Command::perform(async {}, |_| AppEvent::ShowDatePicker(false, date_picker)),
                    Command::perform(async {}, |_| AppEvent::SaleInfoSearchStats),
                ])
            }
            AppEvent::ScrollScrolled(on_scroll, RelativeOffset { x: _, y }) => match on_scroll {
                OnScroll::CatalogListProducts if y == 1.0 => {
                    self.catalog_controller.products_catalog_page += 1;

                    Command::perform(
                        product_repo::get_products_catalog_like(
                            db_connection,
                            self.catalog_controller.product_name_to_lowercase(),
                            self.catalog_controller.products_catalog_page,
                        ),
                        AppEvent::CatalogProductsData,
                    )
                }
                _ => Command::none(),
            },

            //Catalog module events
            AppEvent::CatalogProductsData(result) => match result {
                Ok(products) => {
                    let no_products = products.is_empty();

                    if no_products {
                        helpers::send_toast_ok(STOCK_IS_EMPTY_MSG.into())
                    } else {
                        self.catalog_controller.stock_products.extend(products);
                        Command::none()
                    }
                }
                Err(_) => helpers::send_toast_err(GENERAL_RETRY_MSG.into()),
            },
            AppEvent::CatalogRequestProductInfoForm => Command::perform(
                product_repo::get_product_info_catalog(
                    db_connection,
                    self.catalog_controller.form.barcode.to_string(),
                ),
                AppEvent::CatalogProductInfoFormRequested,
            ),
            AppEvent::CatalogProductInfoFormRequested(result) => self
                .catalog_controller
                .process_product_info_form_data(result),
            AppEvent::CatalogNewRecordListTobeSavedCancel => {
                self.catalog_controller.reset_values_form();

                if self.catalog_controller.products_to_add.is_empty() {
                    Command::perform(async {}, |_| AppEvent::ChangeView(View::CatalogProducts))
                } else {
                    Command::perform(async {}, |_| {
                        AppEvent::ChangeView(View::CatalogProductsToBeAdded)
                    })
                }
            }
            AppEvent::CatalogNewRecordListTobeSavedOk(is_edit) => {
                match self.catalog_controller.form.validate() {
                    Ok(_) => self
                        .catalog_controller
                        .process_new_product_into_list_to_be_added(is_edit),
                    Err(_) => helpers::send_toast_err(ASK_FILL_CATALOG_FORM.into()),
                }
            }
            AppEvent::CatalogEditRecordListTobeSaved(barcode) => {
                let product_info = self.catalog_controller.products_to_add.get(&barcode);
                self.catalog_controller.form = CatalogProductForm::from(product_info);

                Command::perform(async {}, |_| {
                    AppEvent::ChangeView(View::CatalogAddProductForm(true))
                })
            }
            AppEvent::CatalogRemoveRecordListTobeSaved(barcode) => {
                self.catalog_controller.products_to_add.remove(&barcode);

                if self.catalog_controller.products_to_add.is_empty() {
                    Command::perform(async {}, |_| AppEvent::ChangeView(View::CatalogProducts))
                } else {
                    Command::none()
                }
            }
            AppEvent::CatalogSaveAllRecords => Command::perform(
                product_repo::save_products_catalog(
                    db_connection,
                    self.catalog_controller
                        .products_to_add
                        .values()
                        .cloned()
                        .collect(),
                ),
                AppEvent::CatalogSaveAllRecordsPerformed,
            ),
            AppEvent::CatalogSaveAllRecordsPerformed(result) => {
                if result.is_err() {
                    return helpers::send_toast_err(GENERAL_RETRY_MSG.into());
                }

                self.catalog_controller.products_to_add.clear();

                Command::batch(vec![
                    Command::perform(async {}, |_| AppEvent::ChangeView(View::CatalogProducts)),
                    Command::perform(async {}, |_| {
                        AppEvent::AddToast(
                            toast::Status::Success,
                            "Se han insertado los productos".into(),
                        )
                    }),
                ])
            }

            // Sale module events
            AppEvent::SaleProductInfoRequested(result) => {
                self.sale_controller.user_input.reset_values();
                match result {
                    Err(_) => helpers::send_toast_err(GENERAL_RETRY_MSG.into()),
                    Ok(record) => match record {
                        Some(product_info) => {
                            self.sale_controller.process_product_info(product_info)
                        }
                        None => helpers::send_toast_err(NO_PRODUCT.into()),
                    },
                }
            }
            AppEvent::SaleResetProductsToBeSold => {
                self.sale_controller.sale_info = SaleInfo::default();
                Command::none()
            }
            AppEvent::SaleEditProductToBeSold(product_info) => {
                self.sale_controller.user_input.amount = product_info.amount.to_string();

                Command::batch(vec![
                    Command::perform(async {}, |_| {
                        AppEvent::ChangeModalView(ModalView::SaleProductAddEditForm(
                            product_info,
                            true,
                        ))
                    }),
                    Command::perform(async {}, |_| AppEvent::ShowModal),
                ])
            }
            AppEvent::SaleRemoveProductToBeSold(barcode) => {
                let product = self.sale_controller.sale_info.products.remove(&barcode);
                if product.is_some() {
                    self.sale_controller.update_total_pay();
                    self.sale_controller.calculate_payback_money();
                }

                if self.sale_controller.sale_info.products.is_empty() {
                    self.sale_controller.sale_info = SaleInfo::default();
                }
                Command::none()
            }
            AppEvent::SaleProductAddEditFormOk(product_info, is_edit) => {
                let mut product_info = product_info;

                product_info.amount = BigDecimal::from_str(&self.sale_controller.user_input.amount)
                    .unwrap_or(BigDecimal::default());
                self.sale_controller.user_input.reset_values();

                let next_event = if let Err(err) = self
                    .sale_controller
                    .add_new_product_to_sale(&product_info, is_edit)
                {
                    helpers::send_toast_err(err.msg)
                } else {
                    self.sale_controller.update_total_pay();
                    self.sale_controller.calculate_payback_money();
                    Command::perform(async {}, |_| AppEvent::ChangeView(View::SaleListProducts))
                };

                Command::batch(vec![
                    Command::perform(async {}, |_| AppEvent::HideModal),
                    next_event,
                ])
            }
            AppEvent::SaleCreateNewSale => {
                if self.sale_controller.sale_info.products.is_empty() {
                    return Command::none();
                }
                Command::perform(
                    sale_repo::process_new_sale_flow(
                        db_connection,
                        Sale::from(&self.sale_controller.sale_info),
                    ),
                    AppEvent::SaleCreateNewSaleRequested,
                )
            }
            AppEvent::SaleCreateNewSaleRequested(result) => {
                self.sale_controller.user_input.reset_values();
                let commands = vec![
                    Command::perform(async {}, |_| AppEvent::SaleResetProductsToBeSold),
                    // Command::perform(async {}, |_| AppEvent::ChangeView(View::SaleListProducts)),
                    match result {
                        Ok(sale_id) => {
                            let mut loan = SaleLoan::from(&self.sale_controller.sale_info);
                            loan.sale_id = sale_id;
                            Command::batch(vec![
                                if loan.is_valid {
                                    // if its not a loan do not save it
                                    Command::perform(
                                        loan_repo::save_new_loan(db_connection, loan),
                                        AppEvent::SaleCreateNewSaleLoan,
                                    )
                                } else {
                                    Command::none()
                                },
                                Command::perform(async {}, |_| {
                                    AppEvent::AddToast(
                                        toast::Status::Success,
                                        "venta exitosa".into(),
                                    )
                                }),
                            ])
                        }
                        Err(err) => helpers::send_toast_err(err.msg),
                    },
                ];

                Command::batch(commands)
            }
            AppEvent::SaleCreateNewSaleLoan(result) => match result {
                Ok(_) => Command::perform(async {}, move |_| {
                    AppEvent::AddToast(
                        toast::Status::Success,
                        "prestamo registrado exitosamente".into(),
                    )
                }),
                Err(err) => helpers::send_toast_err(err.msg),
            },

            //Loan module events
            AppEvent::LoanSearchRequested => Command::perform(
                loan_repo::get_loans_by_debtor_name(
                    db_connection,
                    self.loan_controller.debtor_name_to_lowercase(),
                ),
                AppEvent::LoanSearchData,
            ),
            AppEvent::LoanSearchData(result) => match result {
                Ok(loans) => {
                    self.loan_controller.data.loans_by_debtor = loans;
                    Command::none()
                }
                Err(err) => helpers::send_toast_err(err.msg),
            },
            AppEvent::LoanShowLoanSale(sale_id) => Command::perform(
                sale_repo::get_products_sale(db_connection, sale_id),
                AppEvent::LoanSaleProductsData,
            ),
            AppEvent::LoanSaleProductsData(result) => match result {
                Ok(products) => Command::batch(vec![
                    Command::perform(async {}, |_| {
                        AppEvent::ChangeModalView(ModalView::LoanSaleDetails(products))
                    }),
                    Command::perform(async {}, |_| AppEvent::ShowModal),
                ]),
                Err(err) => helpers::send_toast_err(err.msg),
            },
            AppEvent::LoanShowPaymentsDetails(loan_id) => {
                self.loan_controller.data.loan_id = loan_id;
                self.loan_controller.data.loan_payment.clear();

                Command::perform(
                    loan_repo::get_payments_loan(db_connection, loan_id),
                    AppEvent::LoanPaymentDetailsData,
                )
            }
            AppEvent::LoanPaymentDetailsData(result) => match result {
                Ok(payments) => Command::batch(vec![
                    Command::perform(async {}, |_| {
                        AppEvent::ChangeModalView(ModalView::LoanPayments(payments))
                    }),
                    Command::perform(async {}, |_| AppEvent::ShowModal),
                ]),
                Err(err) => helpers::send_toast_err(err.msg),
            },
            AppEvent::LoanAddNewPaymentToLoan => {
                let payment_amount = self.loan_controller.get_payment_loan();

                if payment_amount.to_bigdecimal(TO_DECIMAL_DIGITS) > BigDecimal::zero() {
                    Command::perform(
                        loan_repo::add_new_payment_loan(
                            db_connection,
                            self.loan_controller.data.loan_id,
                            payment_amount,
                        ),
                        AppEvent::LoanAddNewPaymentToLoanRequested,
                    )
                } else {
                    helpers::send_toast_err("monto debe ser mayor a cero".into())
                }
            }
            AppEvent::LoanAddNewPaymentToLoanRequested(result) => match result {
                Err(err) => helpers::send_toast_err(err.to_string()),
                Ok(_) => {
                    let loan_id = self.loan_controller.data.loan_id;
                    Command::batch(vec![
                        helpers::send_toast_ok("Pago registrado exitosamente".to_string()),
                        Command::perform(async {}, move |_| {
                            AppEvent::LoanShowPaymentsDetails(loan_id)
                        }),
                        Command::perform(async {}, |_| AppEvent::LoanSearchRequested),
                    ])
                }
            },

            //ToBuy module events
            AppEvent::ToBuySearchData(result) => match result {
                Ok(products) => {
                    let is_empty = products.is_empty();
                    self.tobuy_controller.products = products;
                    if is_empty {
                        return helpers::send_toast_ok("no se encontraron productos".into());
                    }
                    Command::none()
                }
                Err(err) => helpers::send_toast_err(err.msg),
            },

            // Stats module events
            AppEvent::SaleInfoSearchStats => {
                let start_date = self.saleinfo_controller.search_info.start_date.to_string();
                let end_date = self.saleinfo_controller.search_info.end_date.to_string();

                Command::batch(vec![
                    Command::perform(
                        sale_repo::get_total_earnings(
                            db_connection,
                            start_date.to_string(),
                            end_date.to_string(),
                        ),
                        AppEvent::SaleInfoEarningsData,
                    ),
                    Command::perform(
                        sale_repo::get_total_sales(
                            db_connection,
                            start_date.to_string(),
                            end_date.to_string(),
                        ),
                        AppEvent::SaleInfoTotalSales,
                    ),
                    Command::perform(
                        loan_repo::get_total_loans(db_connection, start_date, end_date),
                        AppEvent::SaleInfoTotalLoans,
                    ),
                ])
            }
            AppEvent::SaleInfoEarningsData(result) => match result {
                Ok(total_earnings) => {
                    self.saleinfo_controller.data_stats.earnings =
                        total_earnings.to_bigdecimal(TO_DECIMAL_DIGITS);
                    Command::none()
                }
                Err(err) => helpers::send_toast_err(err.msg),
            },
            AppEvent::SaleInfoTotalSales(result) => match result {
                Ok(totals) => {
                    self.saleinfo_controller.data_stats.sales = totals.sales;
                    self.saleinfo_controller.data_stats.total_sales =
                        totals.total_sales.to_bigdecimal(TO_DECIMAL_DIGITS);
                    Command::none()
                }
                Err(err) => helpers::send_toast_err(err.msg),
            },
            AppEvent::SaleInfoTotalLoans(result) => match result {
                Ok(totals) => {
                    self.saleinfo_controller.data_stats.loans = totals.loans;
                    self.saleinfo_controller.data_stats.total_loans =
                        totals.money_loans.to_bigdecimal(TO_DECIMAL_DIGITS);
                    Command::none()
                }
                Err(err) => helpers::send_toast_err(err.msg),
            },
        }
    }

    fn view<'a>(&'a self) -> Element<'a, AppEvent> {
        let menu = views::menu::get_menu_btns(&self.current_appmodule);

        let content = column![
            menu, //TODO: center menu
            container(views::body::get_body_based_current_view(self))  // .width(Length::Fill)
                  // .height(Length::Fill)
        ]
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(SPACE_COLUMNS)
        .padding(COLUMN_PADDING);

        let content: Element<'a, AppEvent> = if self.modal.show_modal {
            Modal::new(
                content,
                container(views::modal::get_modal_based_modalview(self))
                    .style(get_black_border_style()),
            )
            .on_blur(AppEvent::HideModal)
            .into()
        } else {
            content.into()
        };

        toast::Manager::new(content, &self.toasts, AppEvent::CloseToast).into()
    }
}
