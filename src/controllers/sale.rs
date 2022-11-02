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

pub struct Sale {
    // States
    pub amount_input_state: text_input::State,
    pub client_pay_input_state: text_input::State,
    pub client_name_input_state: text_input::State,
    pub cancel_new_record_btn_state: button::State,
    pub ok_new_record_btn_state: button::State,
    pub scroll_list_state: scrollable::State,
    pub ok_list_to_pay_state: button::State,
    pub cancel_list_to_pay_state: button::State,

    // Data
    pub product_to_add: ProductToAdd,
    // pub products: HashMap<String, ProductList>,
    // pub total_pay: PgMoney,
    // pub client_pay: String,
    // pub client_name: String,
    // pub payback_money: PgMoney,
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
    pub fn reset_sale_form_values(&mut self) {
        self.product_to_add.reset_values();
        self.sale_info.client_pay.clear();
        self.sale_info.client_name.clear();
    }

    pub fn is_ok_charge(&self) -> bool {
        !self.sale_info.client_pay.is_empty() && !self.is_pay_later()
            || (!self.sale_info.client_pay.is_empty()
                && self.is_pay_later()
                && !self.sale_info.client_name.is_empty())
    }

    pub fn is_pay_later(&self) -> bool {
        self.sale_info
            .payback_money
            .to_bigdecimal(TO_DECIMAL_DIGITS)
            < BigDecimal::default()
    }

    pub fn calculate_payback_money(&mut self) {
        let user_pay = PgMoney::from_bigdecimal(
            BigDecimal::from_str(&self.sale_info.client_pay).unwrap(),
            PGMONEY_DECIMALS,
        )
        .unwrap();

        self.sale_info.payback_money = user_pay - self.sale_info.total_pay;
    }

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
