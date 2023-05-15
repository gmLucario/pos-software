//! Handle logic to link [`crate::views::sale`]
//! module with the [`crate::data::sale_repo::SaleRepo`]

use std::str::FromStr;

use custom_crates::widgets::toast;
use iced::{
    keyboard::{self, KeyCode},
    Command,
};
use num_traits::Zero;
use sqlx::{
    postgres::{types::PgMoney, PgPool},
    types::BigDecimal,
};

use crate::{
    constants::{CHARS_SAVED_AS_BARCODE, PGMONEY_DECIMALS, TO_DECIMAL_DIGITS},
    errors::AppError,
    events::AppEvent,
    kinds::{ModalView, UnitsMeasurement},
    models::sale::SaleProductInfo,
    repo::product_repo,
    result::AppResult,
    schemas::sale::{SaleInfo, SaleUserInput},
};

/// Controller links [`crate::views::sale`] module with the [`crate::data::sale_repo::SaleRepo`]
#[derive(Default)]
pub struct Sale {
    /// Product info which will be added to the list to sale
    pub user_input: SaleUserInput,
    /// Info of the general sale
    pub sale_info: SaleInfo,
}

fn get_validation_error<T>(raw_msg: &str, msg: &str, function_name: &str) -> AppResult<T> {
    AppError::validation_error(
        &format!("src/controllers/sale.rs::{function_name}"),
        msg,
        raw_msg,
    )
}

impl Sale {
    /// Reset values store user inputs
    pub fn reset_sale_form_values(&mut self) {
        self.user_input.reset_values();
        self.sale_info.payback_money = PgMoney(0);
        self.sale_info.client_pay.clear();
        self.sale_info.client_name.clear();
    }

    /// Checks if everithing is set correctly to make the sale
    pub fn is_ok_charge(&self) -> bool {
        !self.sale_info.client_pay.is_empty() && !self.is_pay_later()
            || (!self.sale_info.client_pay.is_empty()
                && self.is_pay_later()
                && !self.sale_info.client_name.is_empty())
    }

    /// Checks if the sale is a loan and client will pay later
    pub fn is_pay_later(&self) -> bool {
        self.sale_info
            .payback_money
            .to_bigdecimal(TO_DECIMAL_DIGITS)
            < BigDecimal::default()
    }

    /// Calculate money to be return to the client
    pub fn calculate_payback_money(&mut self) {
        let client_pay_is_empty = self.sale_info.client_pay.is_empty();

        self.sale_info.payback_money = if client_pay_is_empty {
            PgMoney(0)
        } else {
            let user_pay = PgMoney::from_bigdecimal(
                BigDecimal::from_str(&self.sale_info.client_pay).unwrap_or_default(),
                PGMONEY_DECIMALS,
            )
            .unwrap_or(PgMoney(0));
            user_pay - self.sale_info.total_pay
        };
    }

    // Add new product to the sale list
    pub fn add_new_product_to_sale(
        &mut self,
        product_info: &SaleProductInfo,
        is_edit: bool,
    ) -> AppResult<()> {
        let amount = match self.sale_info.products.get(&product_info.barcode) {
            Some(product) => {
                if !is_edit {
                    &product_info.amount + &product.amount
                } else {
                    product_info.amount.clone()
                }
            }
            None => product_info.amount.clone(),
        };

        if amount.is_zero() {
            return get_validation_error(
                "product_info.total_amount.is_zero",
                "Cantidad definida tiene que ser mayor a 0",
                "Sale::add_new_product_to_sale",
            );
        }

        if product_info.total_amount < amount {
            return get_validation_error(
                "product_info.total_amount < amount",
                "Cantidad definida sobrepasa pruductos en almacen",
                "Sale::add_new_product_to_sale",
            );
        }

        let barcode = product_info.barcode.to_string();
        self.sale_info
            .products
            .entry(barcode.to_string())
            .and_modify(|element| {
                element.amount = amount.clone();
            })
            .or_insert_with(|| SaleProductInfo {
                barcode,
                amount,
                ..product_info.clone()
            });

        Ok(())
    }

    /// Insert `c` into the product `barcode` if `c` is `is_alphanumeric`
    fn process_char_event(&mut self, c: &char) {
        if c.is_alphanumeric() {
            self.user_input.barcode.push(*c);
        }

        if self.user_input.barcode.len() > CHARS_SAVED_AS_BARCODE {
            self.user_input.barcode.clear();
        }
    }

    /// Process `Keyboard` events to save the `barcode`
    pub fn process_barcode_input(
        &mut self,
        event: keyboard::Event,
        db_connection: &'static PgPool,
    ) -> Command<AppEvent> {
        match event {
            keyboard::Event::CharacterReceived(c) => self.process_char_event(&c),
            keyboard::Event::KeyPressed {
                key_code: KeyCode::Enter,
                ..
            } => {
                if !self.user_input.barcode.is_empty() {
                    return Command::perform(
                        product_repo::get_product_info_sale(
                            db_connection,
                            self.user_input.barcode.to_string(),
                        ),
                        AppEvent::SaleProductInfoRequested,
                    );
                }
            }
            _ => (),
        }
        Command::none()
    }

    pub fn update_total_pay(&mut self) {
        self.sale_info.total_pay = PgMoney(0);
        for (_, product) in self.sale_info.products.iter() {
            let total_price = PgMoney::from_bigdecimal(
                product.price.to_bigdecimal(TO_DECIMAL_DIGITS) * &product.amount,
                PGMONEY_DECIMALS,
            );
            self.sale_info.total_pay += total_price.unwrap_or(PgMoney(0))
        }
    }

    /// Set total to pay to zero
    pub fn reset_total_pay(&mut self) {
        self.sale_info.total_pay = PgMoney(0);
    }

    /// Process product info to be shown in sale module
    pub fn process_product_info(&mut self, product_info: SaleProductInfo) -> Command<AppEvent> {
        match UnitsMeasurement::from(product_info.unit_measurement_id) {
            UnitsMeasurement::Kilograms | UnitsMeasurement::Liters => Command::batch(vec![
                Command::perform(async {}, |_| {
                    AppEvent::ChangeModalView(ModalView::SaleProductAddEditForm(
                        product_info,
                        false,
                    ))
                }),
                Command::perform(async {}, |_| AppEvent::ShowModal),
            ]),
            UnitsMeasurement::Pieces => {
                if let Err(err) = self.add_new_product_to_sale(&product_info, false) {
                    Command::perform(async {}, |_| {
                        AppEvent::AddToast(toast::Status::Danger, err.msg)
                    })
                } else {
                    self.update_total_pay();
                    Command::none()
                }
            }
        }
    }
}
