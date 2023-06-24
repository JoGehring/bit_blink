use std::fmt;
use std::fmt::Formatter;


use serde::{Deserialize, Serialize, Serializer};

use serde::ser::SerializeStruct;

use crate::bluetooth::utils::*;

const HEADER: &str = "77616E670000";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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

impl Speed {
    pub fn get(val: f64) -> Speed {
        match val {
            val if val == 1.0 => Speed::One,
            val if val == 2.0 => Speed::Two,
            val if val == 3.0 => Speed::Three,
            val if val == 4.0 => Speed::Four,
            val if val == 5.0 => Speed::Five,
            val if val == 6.0 => Speed::Six,
            val if val == 7.0 => Speed::Seven,
            val if val == 8.0 => Speed::Eight,
            _ => Speed::Five
        }
    }

    pub fn get_value(speed: Speed) -> f64 {
        match speed {
            val if val == Speed::One => 1.0,
            val if val == Speed::Two => 2.0,
            val if val == Speed::Three => 3.0,
            val if val == Speed::Four => 4.0,
            val if val == Speed::Five => 5.0,
            val if val == Speed::Six => 6.0,
            val if val == Speed::Seven => 7.0,
            val if val == Speed::Eight => 8.0,
            _ => 5.0
        }
    }
}

impl fmt::Display for Speed {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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

impl Animation {
    pub fn get(val: u32) -> Animation {
        match val {
            val if val == 0 => Animation::Left,
            val if val == 1 => Animation::Right,
            val if val == 2 => Animation::Up,
            val if val == 3 => Animation::Down,
            val if val == 4 => Animation::FixedMiddle,
            val if val == 5 => Animation::FixedLeft,
            val if val == 6 => Animation::Picture,
            val if val == 7 => Animation::Curtain,
            val if val == 8 => Animation::Laser,
            _ => Animation::Left
        }
    }

    pub fn get_value(mode: Animation) -> u32 {
        match mode {
            val if val == Animation::Left => 0,
            val if val == Animation::Right => 1,
            val if val == Animation::Up => 2,
            val if val == Animation::Down => 3,
            val if val == Animation::FixedMiddle => 4,
            val if val == Animation::FixedLeft => 5,
            val if val == Animation::Picture => 6,
            val if val == Animation::Curtain => 7,
            val if val == Animation::Laser => 8,
            _ => 5
        }
    }
}

impl fmt::Display for Animation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Message {
    pub (crate) file_name: String,
    pub(crate) texts: Vec<String>,
    //up to 8 Messages possible which will be done one by one
    pub(crate) inverted: Vec<bool>,
    pub(crate) flash: Vec<bool>,
    pub(crate) marquee: Vec<bool>,
    pub(crate) speed: Vec<Speed>,
    pub(crate) mode: Vec<Animation>,
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("Message", 7)?;
        state.serialize_field("file_name", &self.file_name)?;
        state.serialize_field("hex_strings", &self.convert_text_to_hex_for_json())?;
        state.serialize_field("inverted", &self.inverted)?;
        state.serialize_field("flash", &self.flash)?;
        state.serialize_field("marquee", &self.marquee)?;
        state.serialize_field("speed", &self.speed)?;
        state.serialize_field("mode", &self.mode)?;

        state.end()
    }
}

impl Message {
    pub fn build_bluetooth_message(&self) -> Vec<Vec<u8>> {

        if &self.texts.len() != &self.inverted.len()
            || &self.texts.len() != &self.flash.len()
            || &self.texts.len() != &self.marquee.len()
            || &self.texts.len() != &self.speed.len()
            || &self.texts.len() != &self.mode.len() {
            panic!("Amounts of data in messages weren't equal for all parameters");
        }

        let mut bluetooth_messages: Vec<String> = vec![];
        for msg in self.get_hex_string().into_iter() {
            bluetooth_messages.push(msg);
        }

        let mut bluetooth_message_string = String::from("") + &HEADER + &self.get_hex_flash() + &self.get_hex_marquee() + &self.get_hex_speed_and_mode() + &*Message::get_hex_sizes(&bluetooth_messages) + &self.get_hex_timestamp() + "00000000000000000000000000000000";

        for msg in bluetooth_messages.into_iter() {
            bluetooth_message_string = bluetooth_message_string + &msg;
        }

        bluetooth_message_string = Message::fill_with_zeroes(bluetooth_message_string, 32, false);
        println!("{}", bluetooth_message_string);

        let bluetooth_message = hex_to_byte_array(bluetooth_message_string);

        return bluetooth_message;
    }

    fn get_hex_string(&self) -> Vec<String> {
        let mut hex_strings: Vec<String> = Vec::new();
        for i in 0..self.texts.len() {
            let mut result = letters_to_hex(&self.texts[i]);
            if self.inverted[i] {   //invert
                result = encode_and_invert(&result);
            }
            hex_strings.push(result);
        }
        hex_strings
    }

    fn convert_text_to_hex_for_json(&self) -> Vec<String> {
        let mut hex_strings: Vec<String> = Vec::new();
        for i in 0..self.texts.len() {
            let result = letters_to_hex(&self.texts[i]);
            hex_strings.push(result);
        }
        hex_strings
    }

    fn get_hex_sizes(texts: &Vec<String>) -> String {
        let mut hex_sizes: String = "".to_string();

        for text in texts {
            let size = text.len() as i32 / 22;
            let mut size_as_hex: String = format!("{size:X}");

            while size_as_hex.len() < 4 {
                size_as_hex = "0".to_owned() + &size_as_hex;
            }
            println!("current_size_as_hex: {}", size_as_hex);
            hex_sizes = hex_sizes + &*size_as_hex;
        }
        while hex_sizes.len() < 32 {
            hex_sizes = hex_sizes + "0";
        }
        hex_sizes
    }

    fn fill_with_zeroes(mut bluetooth_message_string: String, total_amount: i32, front: bool) -> String {
        let mut amount_zeros: i32 = 0;
        if bluetooth_message_string.len() as i32 % total_amount != 0 {
            amount_zeros = total_amount - (bluetooth_message_string.len() as i32 % total_amount);
        }
        if front {
            //fill the rest of the first row with zeros
            for _i in 0..amount_zeros {
                bluetooth_message_string = "0".to_owned() + &*bluetooth_message_string;
            }
        } else {
            //fill the rest of the last row with zeros
            for _i in 0..amount_zeros {
                bluetooth_message_string = bluetooth_message_string + "0";
            }
        }

        bluetooth_message_string
    }

    fn get_hex_flash(&self) -> String {
        //every message can have a flash. To tell the badge that the first one is on the string is "01", second = "02" and first and second = "03" and so on
        let mut res = 0;
        let mut i = 1;
        for f in &self.flash {
            if *f {
                res = res + i;
            }
            i = i * 2;
        }
        let result = format!("{:x}", res);
        Message::fill_with_zeroes(result, 2, true)
    }


    fn get_hex_marquee(&self) -> String {
        //every message can have a marquee. To tell the badge that the first one is on the string is "01", second = "02" and first and second = "03" and so on
        let mut res = 0;
        let mut i = 1;
        for f in &self.marquee {
            if *f {
                res = res + i;
            }
            i = i * 2;
        }
        let result = format!("{:x}", res);
        Message::fill_with_zeroes(result, 2, true)
    }

    fn get_hex_speed_and_mode(&self) -> String {
        let mut result = String::from("");
        for i in 0..8 {
            if &self.speed.len() > &i {
                let a = match &self.speed[i] {
                    Speed::One => "0",
                    Speed::Two => "1",
                    Speed::Three => "2",
                    Speed::Four => "3",
                    Speed::Five => "4",
                    Speed::Six => "5",
                    Speed::Seven => "6",
                    Speed::Eight => "7",
                };
                result = result + a;
            } else {
                result = result + "0";
            }

            if &self.speed.len() > &i {
                let b = match &self.mode[i] {
                    Animation::Left => "0",
                    Animation::Right => "1",
                    Animation::Up => "2",
                    Animation::Down => "3",
                    Animation::FixedMiddle => "4",
                    Animation::FixedLeft => "5",
                    Animation::Picture => "6",
                    Animation::Curtain => "7",
                    Animation::Laser => "8",
                };
                result = result + b;
            } else {
                result = result + "0";
            }
        }

        result = Message::fill_with_zeroes(result, 16, false); //8*"00"
        result
    }

    fn get_hex_timestamp(&self) -> String {
        let now = chrono::Local::now();
        println!("Message sent at: {}", now.format("%y %m %d %H:%M:%S").to_string());

        let year = now.format("%y").to_string();
        let month = now.format("%m").to_string();
        let day = now.format("%d").to_string();
        let hour = now.format("%H").to_string();
        let minute = now.format("%M").to_string();
        let second = now.format("%S").to_string();

        println!("{}", String::from("000000000000") + &*year + &*month + &*day + &*hour + &*minute + &*second + "00000000");

        String::from("000000000000") + &*year + &*month + &*day + &*hour + &*minute + &*second + "00000000"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn give_example_message () -> Message {
        let file_name = String::from("Test");
        let texts = vec!(String::from("test"),String::from("abc"),String::from("123"));
        let inverted = vec!(false, true, true);
        let flash = vec!(false, true, true);
        let marquee = vec!(false, true, true);
        let speed = vec!(Speed::One, Speed::Eight, Speed::Four);
        let mode = vec!(Animation::Left, Animation::Laser, Animation::Curtain);

        Message{file_name, texts, inverted, flash, marquee, speed, mode}
    }

    fn give_wrong_example_message () -> Message {
        let file_name = String::from("Test");
        let texts = vec!(String::from("test"),String::from("abc"),String::from("123"));
        let inverted = vec!(false, true, true);
        let flash = vec!(false, true, true);
        let marquee = vec!(false, true, true);
        let speed = vec!(Speed::One, Speed::Eight); //Missing Speed
        let mode = vec!(Animation::Left, Animation::Laser, Animation::Curtain);

        Message{file_name, texts, inverted, flash, marquee, speed, mode}
    }

    #[test]
    #[should_panic]
    pub fn build_bluetooth_message_test() {
        let message = give_wrong_example_message();
        let result: Vec<Vec<u8>> = vec!(vec!(119,97,110,103,0,0,6,6,0,120,55,0,0,0,0,0),vec!(0,4,0,3,0,3,0,0,0,0,0,0,0,0,0,0),vec!(0, 0, 0, 0, 0, 0, 35, 6, 36, 20, 24, 4, 0, 0, 0, 0),vec!(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0),vec!(0,16,48,48,252,48,48,48,52,24,0,0,0,0,0,124),vec!(198,254,192,198,124,0,0,0,0,0,124,198,112,28,198,124),vec!(0,0,16,48,48,252,48,48,48,52,24,0,255,255,255,255),vec!(135,243,131,51,51,137,255,255,31,159,159,131,153,153,153,153),vec!(131,255,255,255,255,255,131,57,63,63,57,131,255,255,231,199),vec!(135,231,231,231,231,231,129,255,255,131,57,249,243,231,207,159),vec!(57,1,255,255,131,57,249,249,195,249,249,57,131,255,0,0));
        let build_message = message.build_bluetooth_message();
        for i in 0..result.len() {
            if i != 2 { //timestamp always differs between the two. Therefore the timestamp part is skipped at the comparison
                assert_eq!(result[i], build_message[i]);
            }
        }
    }

    #[test]
    pub fn build_bluetooth_message_test_wrong() {
        let message = give_example_message();
        let result: Vec<Vec<u8>> = vec!(vec!(119,97,110,103,0,0,6,6,0,120,55,0,0,0,0,0),vec!(0,4,0,3,0,3,0,0,0,0,0,0,0,0,0,0),vec!(0, 0, 0, 0, 0, 0, 35, 6, 36, 20, 24, 4, 0, 0, 0, 0),vec!(0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0),vec!(0,16,48,48,252,48,48,48,52,24,0,0,0,0,0,124),vec!(198,254,192,198,124,0,0,0,0,0,124,198,112,28,198,124),vec!(0,0,16,48,48,252,48,48,48,52,24,0,255,255,255,255),vec!(135,243,131,51,51,137,255,255,31,159,159,131,153,153,153,153),vec!(131,255,255,255,255,255,131,57,63,63,57,131,255,255,231,199),vec!(135,231,231,231,231,231,129,255,255,131,57,249,243,231,207,159),vec!(57,1,255,255,131,57,249,249,195,249,249,57,131,255,0,0));
        let build_message = message.build_bluetooth_message();
        for i in 0..result.len() {
            if i != 2 { //timestamp always differs between the two. Therefore the timestamp part is skipped at the comparison
                assert_eq!(result[i], build_message[i]);
            }
        }
    }

    #[test]
    fn get_hex_string_test() {
        let message = give_example_message();
        let str = vec!("00103030FC303030341800000000007CC6FEC0C67C00000000007CC6701CC67C0000103030FC303030341800",
                       "ffffffff87f383333389ffff1f9f9f839999999983ffffffffff83393f3f3983ff",
                       "ffe7c787e7e7e7e7e781ffff8339f9f3e7cf9f3901ffff8339f9f9c3f9f93983ff");
        assert_eq!(str, message.get_hex_string());
    }

    #[test]
    fn convert_text_to_hex_for_json_test() {
        let message = give_example_message();
        let result: Vec<String> = vec!(String::from("00103030FC303030341800000000007CC6FEC0C67C00000000007CC6701CC67C0000103030FC303030341800"),String::from("00000000780C7CCCCC760000E060607C666666667C00000000007CC6C0C0C67C00"),String::from("0018387818181818187E00007CC6060C183060C6FE00007CC606063C0606C67C00"));

        assert_eq!(result, message.convert_text_to_hex_for_json());
    }

    #[test]
    fn get_hex_sizes_test() {
        let message = give_example_message();
        let texts = message.get_hex_string();
        assert_eq!("00040003000300000000000000000000", &*Message::get_hex_sizes(&texts));
    }

    #[test]
    fn fill_with_zeros_test() {
        //fn fill_with_zeroes_test(mut bluetooth_message_string: String, total_amount: i32, front: bool) -> String {
        let test = Message::fill_with_zeroes(String::from("test"), 32, false);
        assert_eq!(32, test.len());
    }


    #[test]
    fn get_hex_flash_test() {
        let message = give_example_message();
        assert_eq!("06", message.get_hex_flash());
    }

    #[test]
    fn get_hex_marquee_test() {
        let message = give_example_message();
        assert_eq!("06", message.get_hex_marquee());
    }

    #[test]
    fn get_hex_speed_and_mode_test() {
        let message = give_example_message();
        assert_eq!("0078370000000000", message.get_hex_speed_and_mode());
    }

    #[test]
    fn get_hex_timestamp_test() {
        let message = give_example_message();
        assert_eq!(32, message.get_hex_timestamp().len());
    }


}