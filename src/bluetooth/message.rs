use std::error::Error;
use std::string;
use std::time::Duration;
use std::borrow::Borrow;
use tokio::time;
use crate::bluetooth::utils::*;
//use crate::bluetooth::utils::utils::{hex_to_byte_array, letters_to_hex};

const HEADER: &str = "77616E670000";

pub enum Speed {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
}

pub enum Animation {
    Left = 00,
    Right = 01,
    Up = 02,
    Down = 03,
    FixedMiddle = 04,
    FixedLeft = 05,
    Picture = 06,
    Curtain = 07,
    Laser = 08,
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

        println!("{}", bluetooth_message_string);

        let mut bluetooth_message = hex_to_byte_array(bluetooth_message_string);

        return bluetooth_message;
    }

    fn get_hex_string(&self) -> Vec<String> {
        //todo implement for more than one message
        let mut hex_strings : Vec<String> = Vec::new();
        for i in 0..self.texts.len() {
            let mut result = letters_to_hex(&self.texts[i]);
            if self.inverted[i] {   //invert
                let temp: Vec<u8> = decode_hex(&result).unwrap().iter().map(|b| {!b} ).collect();
                result = bytes_to_hex_string(&temp);
            }
            let mut amount_zeros:i32 = 0;
            if (result.len() as i32 % 32 != 0) {
                amount_zeros = 32 - (result.len() as i32 % 32);
            }
            //fill the rest of the last row with zeros
            for _i in 0..amount_zeros {
                result = result + "0";
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

    fn get_hex_mode(&self) -> &str {
        "0000000000000000" //todo 8*"00"
    }

    fn get_hex_timestamp(&self) -> &str {
        "000000000000E10C0700203100000000" //todo
    }
}