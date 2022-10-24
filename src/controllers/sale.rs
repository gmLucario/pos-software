use std::{collections::HashMap, str::FromStr};

use iced::{button, keyboard::Event, keyboard::KeyCode, scrollable, text_input, Command};
use iced_native::Event::Keyboard;
use sqlx::{
    pool::Pool,
    postgres::{types::PgMoney, Postgres},
    types::BigDecimal,
};

use crate::{
    constants::{CHARS_SAVED_AS_BARCODE, PGMONEY_DECIMALS},
    data::product_repo::ProductRepo,
    kinds::AppEvents,
    schemas::sale::{ProductList, ProductToAdd},
};

#[derive(Default)]
pub struct Sale {
    // States
    pub amount_input_state: text_input::State,
    pub cancel_new_record_btn_state: button::State,
    pub ok_new_record_btn_state: button::State,
    pub scroll_list_state: scrollable::State,
    pub ok_list_to_pay_state: button::State,
    pub cancel_list_to_pay_state: button::State,

    // Data
    pub product_to_add: ProductToAdd,
    pub products: HashMap<String, ProductList>,
}

impl Sale {
    pub fn add_new_product_to_sale(&mut self) {
        let barcode: String = format!("{}", self.product_to_add.barcode);

        let price = BigDecimal::from_str(&self.product_to_add.price).unwrap();
        let mut amount = BigDecimal::from_str(&self.product_to_add.amount).unwrap();
        amount = match self.products.get(&barcode) {
            Some(product) => amount + &product.amount,
            None => amount,
        };

        let price = PgMoney::from_bigdecimal(price * &amount, PGMONEY_DECIMALS).unwrap();

        if amount <= self.product_to_add.total_amount {
            self.products
                .entry(barcode)
                .and_modify(|element| {
                    element.price = price;
                    element.amount = amount;
                })
                .or_insert(ProductList::from(&self.product_to_add));
        };
    }

    fn process_char_event(&mut self, c: &char) {
        if c.is_alphanumeric() {
            self.product_to_add.barcode.push(*c);
        }

        if self.product_to_add.barcode.len() > CHARS_SAVED_AS_BARCODE {
            self.product_to_add.barcode.clear();
        }
    }

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
