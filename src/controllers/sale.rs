//! Handle logic to link [`crate::views::sale`]
//! module with the [`crate::data::sale_repo::SaleRepo`]

use std::str::FromStr;

use iced::{button, keyboard::Event, keyboard::KeyCode, scrollable, text_input, Command};
use iced_native::Event::Keyboard;
use sqlx::{
    pool::Pool,
    postgres::{types::PgMoney, Postgres},
    types::BigDecimal,
};

use crate::{
    constants::{CHARS_SAVED_AS_BARCODE, PGMONEY_DECIMALS, TO_DECIMAL_DIGITS},
    data::product_repo::ProductRepo,
    kinds::AppEvents,
    schemas::sale::{ProductList, ProductToAdd, SaleInfo},
};

/// Controller links [`crate::views::sale`] module with the [`crate::data::sale_repo::SaleRepo`]
pub struct Sale {
    // States
    /// Iced text input state for product amount/quantity to sale
    pub amount_input_state: text_input::State,
    /// Iced text input state for the amount of money that the client pays
    pub client_pay_input_state: text_input::State,
    /// Iced text input state for the client name. If the sale is a loan
    pub client_name_input_state: text_input::State,
    /// Iced button state to cancel adding a new product to sale list
    pub cancel_new_record_btn_state: button::State,
    /// Iced button state to agree adding a new product to sale list
    pub ok_new_record_btn_state: button::State,
    /// Iced scroll state for track sale products list
    pub scroll_list_state: scrollable::State,
    /// Iced button state to agree list product to sale
    pub ok_list_to_pay_state: button::State,
    /// Iced button cancel list product to sale
    pub cancel_list_to_pay_state: button::State,

    // Data
    /// Product info which will be added to the list to sale
    pub product_to_add: ProductToAdd,
    /// Info of the general sale
    pub sale_info: SaleInfo,
}

impl Default for Sale {
    fn default() -> Self {
        Self {
            amount_input_state: text_input::State::focused(),
            client_pay_input_state: text_input::State::focused(),
            client_name_input_state: text_input::State::new(),
            cancel_new_record_btn_state: Default::default(),
            ok_new_record_btn_state: Default::default(),
            scroll_list_state: Default::default(),
            ok_list_to_pay_state: Default::default(),
            cancel_list_to_pay_state: Default::default(),
            product_to_add: ProductToAdd::default(),
            sale_info: SaleInfo::default(),
        }
    }
}

impl Sale {
    /// Reset values store user inputs
    pub fn reset_sale_form_values(&mut self) {
        self.product_to_add.reset_values();
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
        let user_pay = PgMoney::from_bigdecimal(
            BigDecimal::from_str(&self.sale_info.client_pay).unwrap(),
            PGMONEY_DECIMALS,
        )
        .unwrap();

        self.sale_info.payback_money = user_pay - self.sale_info.total_pay;
    }

    /// Add new product to the sale list
    pub fn add_new_product_to_sale(&mut self) {
        let barcode: String = self.product_to_add.barcode.to_string();

        let price = BigDecimal::from_str(&self.product_to_add.price).unwrap();
        let mut amount = BigDecimal::from_str(&self.product_to_add.amount).unwrap();
        amount = match self.sale_info.products.get(&barcode) {
            Some(product) => amount + &product.amount,
            None => amount,
        };

        let price = PgMoney::from_bigdecimal(price * &amount, PGMONEY_DECIMALS).unwrap();

        if amount <= self.product_to_add.total_amount {
            self.sale_info
                .products
                .entry(barcode)
                .and_modify(|element| {
                    element.price = price;
                    element.amount = amount;
                })
                .or_insert_with(|| ProductList::from(&self.product_to_add));
        };
    }

    /// Insert `c` into the product `barcode` if `c` is `is_alphanumeric`
    fn process_char_event(&mut self, c: &char) {
        if c.is_alphanumeric() {
            self.product_to_add.barcode.push(*c);
        }

        if self.product_to_add.barcode.len() > CHARS_SAVED_AS_BARCODE {
            self.product_to_add.barcode.clear();
        }
    }

    /// Process `Keyboard` events to save the `barcode`
    pub fn process_barcode_input(
        &mut self,
        event: iced_native::Event,
        db_connection: &'static Pool<Postgres>,
    ) -> Command<AppEvents> {
        match event {
            Keyboard(Event::CharacterReceived(c)) => self.process_char_event(&c),
            Keyboard(Event::KeyPressed {
                key_code: KeyCode::Enter,
                ..
            }) => {
                if !self.product_to_add.barcode.is_empty() {
                    return Command::perform(
                        ProductRepo::get_product_info_sale(
                            db_connection,
                            self.product_to_add.barcode.to_string(),
                        ),
                        AppEvents::SaleProductInfoRequested,
                    );
                }
            }
            _ => (),
        }
        Command::none()
    }
}
