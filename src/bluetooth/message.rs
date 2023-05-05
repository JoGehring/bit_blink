use std::error::Error;
use std::string;
use std::time::Duration;
use std::borrow::Borrow;
use tokio::time;
use crate::bluetooth::utils::utils::*;
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
        let bluetooth_message_string = String::from("") + &HEADER + &self.get_hex_flash() + &self.get_hex_marquee() + &self.get_hex_mode() + &self.get_hex_sizes() + &self.get_hex_timestamp() + "00000000000000000000000000000000" + &self.get_hex_string();
        println!("{}", bluetooth_message_string);


        let mut bluetooth_message = hex_to_byte_array(bluetooth_message_string);

        return bluetooth_message;
    }

    fn get_hex_string(&self) -> String {
        //todo implement for more than one message
        let mut result = letters_to_hex(texts[0]);
        //todo invert
        let mut amount_zeros = 0;
        if result.len() != 32 {
            amount_zeros = 32 - (result.len() % 32);
        }
        //fill the rest of the last row with zeros
        for _i in 0..amount_zeros {
            result = result + "0";
        }


        println!("HexString is: {}", result);
        String::from(result)
    }

    fn get_hex_sizes(&self) -> &str {
        let mut result = "";
        //todo implement for more than one message and for messages longer than 9
        result = "000" + texts[0].len;

        let mut amount_zeros = 0;
        if result.len() != 32 {
            amount_zeros = 32 - (result.len() % 32);
        }
        //fill the rest of the last row with zeros
        for _i in 0..amount_zeros {
            result = result + "0";
        }



        "00010000000000000000000000000000" //8 * 0000 todo

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