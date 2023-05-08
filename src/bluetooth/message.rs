use std::error::Error;
use std::string;
use std::time::Duration;
use std::borrow::Borrow;
use tokio::time;
use crate::bluetooth::utils::*;

const HEADER: &str = "77616E670000";

pub enum Speed {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

pub enum Animation {
    Left,
    Right,
    Up,
    Down,
    FixedMiddle,
    FixedLeft,
    Picture,
    Curtain,
    Laser,
}


pub struct Message {
    pub(crate) texts: Vec<String>, //up to 8 Messages possible which will be done one by one
    pub(crate) inverted: Vec<bool>,
    pub(crate) flash: bool,
    pub(crate) marquee: bool,
    pub(crate) speed: Vec<Speed>,
    pub(crate) mode: Vec<Animation> //Vec<Animation>
}

impl Message {
    pub fn build_bluetooth_message(&self) -> Vec<Vec<u8>> {

        let mut bluetooth_message_string = String::from("") + &HEADER + &self.get_hex_flash() + &self.get_hex_marquee() + &self.get_hex_mode() + &self.get_hex_sizes() + &self.get_hex_timestamp() + "00000000000000000000000000000000"; // + &self.get_hex_string();
        for msg in self.get_hex_string().into_iter() {
            bluetooth_message_string = bluetooth_message_string + &msg;
        }

        bluetooth_message_string = Message::fill_with_zeroes(bluetooth_message_string, 32);
        println!("{}", bluetooth_message_string);

        let mut bluetooth_message = hex_to_byte_array(bluetooth_message_string);

        return bluetooth_message;
    }

    fn get_hex_string(&self) -> Vec<String> {
        let mut hex_strings : Vec<String> = Vec::new();
        for i in 0..self.texts.len() {
            let mut result = letters_to_hex(&self.texts[i]);
            if self.inverted[i] {   //invert
                let temp: Vec<u8> = decode_hex(&result).unwrap().iter().map(|b| {!b} ).collect();
                result = bytes_to_hex_string(&temp);
            }
            println!("HexString is: {}", result);
            hex_strings.push(result);
        }
        hex_strings
    }

    pub(crate) fn get_hex_sizes(&self) -> String {      // Couldn't get it working with return type &str
        let mut hex_sizes: String = "".to_string();
        for i in 0..self.texts.len() {
            let elem = &self.texts[i];
            let mut current_hex_size: String = elem.len().to_string();
            while(current_hex_size.len() < 4) {
                current_hex_size = "0".to_owned() + &current_hex_size;
            }
            hex_sizes = format!("{}{}", hex_sizes, current_hex_size);
        }
        while(hex_sizes.len() < 32) {
            hex_sizes = hex_sizes + "0";
        }
        hex_sizes
    }

    fn fill_with_zeroes(mut bluetooth_message_string: String, total_amount: i32) -> String {
        let mut amount_zeros:i32 = 0;
        if (bluetooth_message_string.len() as i32 % total_amount != 0) {
            amount_zeros = total_amount - (bluetooth_message_string.len() as i32 % total_amount);
        }
        //fill the rest of the last row with zeros
        for _i in 0..amount_zeros {
            bluetooth_message_string = bluetooth_message_string + "0";
        }
        bluetooth_message_string
    }

    fn get_hex_flash (&self) -> &str {
        if self.flash {
            return "01";
        }
        "00"
    }

    fn get_hex_marquee(&self) -> &str {
        if self.marquee {
            return "01";
        }
        "00"
    }

    fn get_hex_speed(&self) -> &str {
        "00" //todo find the position in hexString where it changes the speed
    }

    fn get_hex_mode(&self) -> String {
        let mut result = String::from("");
        for animation in &self.mode {
            let mut a = match animation {
                Animation::Left => "00",
                Animation::Right => "01",
                Animation::Up => "02",
                Animation::Down => "03",
                Animation::FixedMiddle => "04",
                Animation::FixedLeft => "05",
                Animation::Picture => "06",
                Animation::Curtain => "07",
                Animation::Laser => "08",
                _ => ""
            };
            result = result + a;
        }
        result = Message::fill_with_zeroes(result, 16); //8*"00"
        result
    }

    fn get_hex_timestamp(&self) -> &str {
        "000000000000E10C0700203100000000" //todo
    }
}