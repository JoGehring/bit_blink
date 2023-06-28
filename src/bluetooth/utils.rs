use std::num::ParseIntError;
use crate::bluetooth::Message;

pub fn hex_to_bytes_array(bluetooth_message_string: String) -> Vec<Vec<u8>> {
    let mut messages_as_bytes: Vec<Vec<u8>> = Vec::new();
    let subs: Vec<&str> = split_string(&bluetooth_message_string, 32);
    for sub in subs.iter() {
        messages_as_bytes.push(encode_hex(sub).unwrap());
    }
    messages_as_bytes
}

pub fn bytes_to_hex_string(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("")
}

pub fn encode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn encode_and_invert(hex_string : &String) -> String {
    let temp: Vec<u8> = encode_hex(&hex_string).unwrap().iter().map(|b| { !b }).collect();
    let inverted_text = bytes_to_hex_string(&temp);
    inverted_text
}

pub fn split_string(string: &String, length: usize) -> Vec<&str> {
    let subs: Vec<&str> = string.as_bytes()
        .chunks(length)
        .map(|buf| unsafe { std::str::from_utf8_unchecked(buf) })
        .collect::<Vec<&str>>();
    subs
}

//this functionality is currently not implemented in the frontend but can easily be used for further
pub fn build_single_message_from_first_text_vec_of_given_messages(given_messages : &Vec<Message>) -> Message {   //given_messages must not longer than 8 elements long
    let mut result_message : Message = Message {
        file_name: "".to_owned(),
        texts: vec![],
        inverted: vec![],
        flash: vec![],
        marquee: vec![],
        speed: vec![],
        mode: vec![],
    };
    for message in given_messages {
        result_message.texts.push(message.texts[0].clone());
        result_message.inverted.push(message.inverted[0].clone());
        result_message.flash.push(message.flash[0].clone());
        result_message.marquee.push(message.marquee[0].clone());
        result_message.speed.push(message.speed[0].clone());
        result_message.mode.push(message.mode[0].clone());
    }
    result_message
}

pub fn letters_to_hex(message_text: &str) -> String {
    let mut result = String::new();

    for c in message_text.chars() {
        if keyword_to_hex(c) != "" {
            result = result + keyword_to_hex(c);
        }
        else {
            result = result + letter_to_hex(c);
        }
    }
    result
}

pub fn keyword_to_hex(keyword: char) -> &'static str{   // all keyword hex_string should be multiples of 22
    let b: &str = match keyword {
        '⚽' => "00003c7efffffffffe3c00",                                 //ball
        '😁' => "00003c42a581a599423c00",                                //happ1
        '😄' => "0008140801000061301c0700205020008080860c38e0",          //happy2
        '❤' => "00006c9282824428100000",                                //heart
        '💕' => "00006cfefefe7c38100000",                                //HEART
        '💟' => "000c1221202010080402010060900808081020408000",         //heart2
        '💗' => "000c1e3f3f3f1f0f0703010060f0f8f8f8f0e0c08000",         //HEART2
        '⚙' => "070e1b03212c2e26141c06806030808838e8c81030c0",           //fablab
        '🚲' => "01020001070912121008070087815f2294495f4980000080008070c824e4048870",    //bike
        '🔁' => "000000000709121310080700f040fd229449fd49800040a0804070c82424048870",    //bike_r
        '☁' => "00010203060c1a1311190f78cc87fc428181818143bd0000008080e030102828d0",     //owncloud

        _ => ""
    };
    b
}

pub fn hex_to_keyword(hex_string: &str) -> &'static str {
    let b: &str = match hex_string {
        "00003c7efffffffffe3c00" => "⚽",                                 //ball
        "00003c42a581a599423c00" => "😁",                                //happ1
        "0008140801000061301c0700205020008080860c38e0" => "😄",          //happy2
        "00006c9282824428100000" => "❤",                                //heart
        "00006cfefefe7c38100000" => "💕",                                //HEART
        "000c1221202010080402010060900808081020408000" => "💟",         //heart2
        "000c1e3f3f3f1f0f0703010060f0f8f8f8f0e0c08000" => "💗",         //HEART2
        "070e1b03212c2e26141c06806030808838e8c81030c0" => "⚙",           //fablab
        "01020001070912121008070087815f2294495f4980000080008070c824e4048870" => "🚲",    //bike
        "000000000709121310080700f040fd229449fd49800040a0804070c82424048870" => "🔁",    //bike_r
        "00010203060c1a1311190f78cc87fc428181818143bd0000008080e030102828d0" => "☁",     //owncloud

        _ => ""
    };

    b
}

pub fn hex_string_to_letter(s: &str) -> &str  {
    let b: &str = match s {
        "007CC6CEDEF6E6C6C67C00" => "0",
        "0018387818181818187E00" => "1",
        "007CC6060C183060C6FE00" => "2",
        "007CC606063C0606C67C00" => "3",
        "000C1C3C6CCCFE0C0C1E00" => "4",
        "00FEC0C0FC060606C67C00" => "5",
        "007CC6C0C0FCC6C6C67C00" => "6",
        "00FEC6060C183030303000" => "7",
        "007CC6C6C67CC6C6C67C00" => "8",
        "007CC6C6C67E0606C67C00" => "9",
        "00000000780C7CCCCC7600" => "a",
        "00E060607C666666667C00" => "b",
        "000000007CC6C0C0C67C00" => "c",
        "001C0C0C7CCCCCCCCC7600" => "d",
        "000000007CC6FEC0C67C00" => "e",
        "001C363078303030307800" => "f",
        "00000076CCCCCC7C0CCC78" => "g",
        "00E060606C76666666E600" => "h",
        "0018180038181818183C00" => "i",
        "0C0C001C0C0C0C0CCCCC78" => "j",
        "00E06060666C78786CE600" => "k",
        "0038181818181818183C00" => "l",
        "00000000ECFED6D6D6C600" => "m",
        "00000000DC666666666600" => "n",
        "000000007CC6C6C6C67C00" => "o",
        "000000DC6666667C6060F0" => "p",
        "0000007CCCCCCC7C0C0C1E" => "q",
        "00000000DE76606060F000" => "r",
        "000000007CC6701CC67C00" => "s",
        "00103030FC303030341800" => "t",
        "00000000CCCCCCCCCC7600" => "u",
        "00000000C6C6C66C381000" => "v",
        "00000000C6D6D6D6FE6C00" => "w",
        "00000000C66C38386CC600" => "x",
        "000000C6C6C6C67E060CF8" => "y",
        "00000000FE8C183062FE00" => "z",
        "00386CC6C6FEC6C6C6C600" => "A",
        "00FC6666667C666666FC00" => "B",
        "007CC6C6C0C0C0C6C67C00" => "C",
        "00FC66666666666666FC00" => "D",
        "00FE66626878686266FE00" => "E",
        "00FE66626878686060F000" => "F",
        "007CC6C6C0C0CEC6C67E00" => "G",
        "00C6C6C6C6FEC6C6C6C600" => "H",
        "003C181818181818183C00" => "I",
        "001E0C0C0C0C0CCCCC7800" => "J",
        "00E6666C6C786C6C66E600" => "K",
        "00F060606060606266FE00" => "L",
        "0082C6EEFED6C6C6C6C600" => "M",
        "0086C6E6F6DECEC6C6C600" => "N",
        "007CC6C6C6C6C6C6C67C00" => "O",
        "00FC6666667C606060F000" => "P",
        "007CC6C6C6C6C6D6DE7C06" => "Q",
        "00FC6666667C6C6666E600" => "R",
        "007CC6C660380CC6C67C00" => "S",
        "007E7E5A18181818183C00" => "T",
        "00C6C6C6C6C6C6C6C67C00" => "U",
        "00C6C6C6C6C6C66C381000" => "V",
        "00C6C6C6C6D6FEEEC68200" => "W",
        "00C6C66C7C387C6CC6C600" => "X",
        "00666666663C1818183C00" => "Y",
        "00FEC6860C183062C6FE00" => "Z",

        "00cccc00780c7ccccc7600" => "ä",
        "00c6c6007cc6c6c6c67c00" => "ö",
        "00cccc00cccccccccc7600" => "ü",
        "c6c6386cc6fec6c6c6c600" => "Ä",
        "c6c6007cc6c6c6c6c67c00" => "Ö",
        "c6c600c6c6c6c6c6c67c00" => "Ü",
        "003c6666667c6666666c60" => "ß",

        "006C6CFE6C6CFE6C6C0000" => "#",
        "00386C6C3876DCCCCC7600" => "&",
        "00000000000000000000FF" => "_",
        "0000000000FE0000000000" => "-",
        "007CC6C60C181800181800" => "?",
        "00003C429DA5ADB6403C00" => "@",
        "000C183030303030180C00" => "(",
        "0030180C0C0C0C0C183000" => ")",
        "0000007E00007E00000000" => "=",
        "00000018187E1818000000" => "+",
        "00183C3C3C181800181800" => "!",
        "1818081000000000000000" => "'",
        "0000001818000018180000" => ":",
        "006092966C106CD2920C00" => "%",
        "000002060C183060C08000" => "/",
        "6666222200000000000000" => "\"",
        "0000000000000000000000" => " ",
        "000000663CFF3C66000000" => "*",
        "0000000000000030301020" => ",",
        "0000000000000000303000" => ".",
        "107CD6D6701CD6D67C1010" => "$",
        "0076DC0000000000000000" => "~",
        "003C303030303030303C00" => "[",
        "003C0C0C0C0C0C0C0C3C00" => "]",
        "000E181818701818180E00" => "{",
        "00701818180E1818187000" => "}",
        "00060C18306030180C0600" => "<",
        "006030180C060C18306000" => ">",
        "386CC60000000000000000" => "^",
        "1818100800000000000000" => "`",
        "0000001818000018180810" => ";",
        "0080C06030180C06020000" => "\\",
        "0018181818001818181800" => "|",

        "003E7A7A7A3A1A0A0A0A00" => "¶",
        "001C222220782020207E00" => "£",
        "001010282844444482FE00" => "∆",
        "0038283800000000000000" => "°",
        "000E10207E207E20100E00" => "€",
        "00081C20404040201C0800" => "¢",
        "0082444428103810381000" => "¥",
        "000000007E242424640000" => "π",
        "007C087C08702010080400" => "₹",
        "0000000000001818000000" => "•",
        "0000006C7C387C6C000000" => "×",
        "00000010007C0010000000" => "÷",
        "0004040C08482828181000" => "√",
        "003CFF22FF3C2020202000" => "₱",

        _ => ""
    };
    b
}

pub fn letter_to_hex(c: char) -> &'static str {
    let b = match c {
        '0' => "007CC6CEDEF6E6C6C67C00",
        '1' => "0018387818181818187E00",
        '2' => "007CC6060C183060C6FE00",
        '3' => "007CC606063C0606C67C00",
        '4' => "000C1C3C6CCCFE0C0C1E00",
        '5' => "00FEC0C0FC060606C67C00",
        '6' => "007CC6C0C0FCC6C6C67C00",
        '7' => "00FEC6060C183030303000",
        '8' => "007CC6C6C67CC6C6C67C00",
        '9' => "007CC6C6C67E0606C67C00",

        'a' => "00000000780C7CCCCC7600",
        'b' => "00E060607C666666667C00",
        'c' => "000000007CC6C0C0C67C00",
        'd' => "001C0C0C7CCCCCCCCC7600",
        'e' => "000000007CC6FEC0C67C00",
        'f' => "001C363078303030307800",
        'g' => "00000076CCCCCC7C0CCC78",
        'h' => "00E060606C76666666E600",
        'i' => "0018180038181818183C00",
        'j' => "0C0C001C0C0C0C0CCCCC78",
        'k' => "00E06060666C78786CE600",
        'l' => "0038181818181818183C00",
        'm' => "00000000ECFED6D6D6C600",
        'n' => "00000000DC666666666600",
        'o' => "000000007CC6C6C6C67C00",
        'p' => "000000DC6666667C6060F0",
        'q' => "0000007CCCCCCC7C0C0C1E",
        'r' => "00000000DE76606060F000",
        's' => "000000007CC6701CC67C00",
        't' => "00103030FC303030341800",
        'u' => "00000000CCCCCCCCCC7600",
        'v' => "00000000C6C6C66C381000",
        'w' => "00000000C6D6D6D6FE6C00",
        'x' => "00000000C66C38386CC600",
        'y' => "000000C6C6C6C67E060CF8",
        'z' => "00000000FE8C183062FE00",

        'A' => "00386CC6C6FEC6C6C6C600",
        'B' => "00FC6666667C666666FC00",
        'C' => "007CC6C6C0C0C0C6C67C00",
        'D' => "00FC66666666666666FC00",
        'E' => "00FE66626878686266FE00",
        'F' => "00FE66626878686060F000",
        'G' => "007CC6C6C0C0CEC6C67E00",
        'H' => "00C6C6C6C6FEC6C6C6C600",
        'I' => "003C181818181818183C00",
        'J' => "001E0C0C0C0C0CCCCC7800",
        'K' => "00E6666C6C786C6C66E600",
        'L' => "00F060606060606266FE00",
        'M' => "0082C6EEFED6C6C6C6C600",
        'N' => "0086C6E6F6DECEC6C6C600",
        'O' => "007CC6C6C6C6C6C6C67C00",
        'P' => "00FC6666667C606060F000",
        'Q' => "007CC6C6C6C6C6D6DE7C06",
        'R' => "00FC6666667C6C6666E600",
        'S' => "007CC6C660380CC6C67C00",
        'T' => "007E7E5A18181818183C00",
        'U' => "00C6C6C6C6C6C6C6C67C00",
        'V' => "00C6C6C6C6C6C66C381000",
        'W' => "00C6C6C6C6D6FEEEC68200",
        'X' => "00C6C66C7C387C6CC6C600",
        'Y' => "00666666663C1818183C00",
        'Z' => "00FEC6860C183062C6FE00",

        'ä' => "00cccc00780c7ccccc7600",
        'ö' => "00c6c6007cc6c6c6c67c00",
        'ü' => "00cccc00cccccccccc7600",
        'Ä' => "c6c6386cc6fec6c6c6c600",
        'Ö' => "c6c6007cc6c6c6c6c67c00",
        'Ü' => "c6c600c6c6c6c6c6c67c00",
        'ß' => "003c6666667c6666666c60",

        '#' => "006C6CFE6C6CFE6C6C0000",
        '&' => "00386C6C3876DCCCCC7600",
        '_' => "00000000000000000000FF",
        '-' => "0000000000FE0000000000",
        '?' => "007CC6C60C181800181800",
        '@' => "00003C429DA5ADB6403C00",
        '(' => "000C183030303030180C00",
        ')' => "0030180C0C0C0C0C183000",
        '=' => "0000007E00007E00000000",
        '+' => "00000018187E1818000000",
        '!' => "00183C3C3C181800181800",
        '\'' => "1818081000000000000000",
        ':' => "0000001818000018180000",
        '%' => "006092966C106CD2920C00",
        '/' => "000002060C183060C08000",
        '"' => "6666222200000000000000",
        ' ' => "0000000000000000000000",
        '*' => "000000663CFF3C66000000",
        ',' => "0000000000000030301020",
        '.' => "0000000000000000303000",
        '$' => "107CD6D6701CD6D67C1010",
        '~' => "0076DC0000000000000000",
        '[' => "003C303030303030303C00",
        ']' => "003C0C0C0C0C0C0C0C3C00",
        '{' => "000E181818701818180E00",
        '}' => "00701818180E1818187000",
        '<' => "00060C18306030180C0600",
        '>' => "006030180C060C18306000",
        '^' => "386CC60000000000000000",
        '`' => "1818100800000000000000",
        ';' => "0000001818000018180810",
        '\\' => "0080C06030180C06020000",
        '|' => "0018181818001818181800",

        '¶' => "003E7A7A7A3A1A0A0A0A00",
        '£' => "001C222220782020207E00",
        '∆' => "001010282844444482FE00",
        '°' => "0038283800000000000000",
        '€' => "000E10207E207E20100E00",
        '¢' => "00081C20404040201C0800",
        '¥' => "0082444428103810381000",
        'π' => "000000007E242424640000",
        '₹' => "007C087C08702010080400",
        '•' => "0000000000001818000000",
        '×' => "0000006C7C387C6C000000",
        '÷' => "00000010007C0010000000",
        '√' => "0004040C08482828181000",
        '₱' => "003CFF22FF3C2020202000",

        _ => ""
    };
    b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_to_bytes_array_test() {
        let result: Vec<Vec<u8>> = vec!(vec!(0,126,126,90,24,24,24,24,24,60,0,0,0,0,0,124),vec!(198,254,192,198,124,0,0,0,0,0,124,198,112,28,198,124),vec!(0,0,16,48,48,252,48,48,48,52,24,0,0,24,60,60),vec!(60,24,24,0,24,24,0));
        assert_eq!(result, hex_to_bytes_array(String::from("007E7E5A18181818183C00000000007CC6FEC0C67C00000000007CC6701CC67C0000103030FC30303034180000183C3C3C181800181800")));
    }

    #[test]
    fn bytes_to_hex_string_test() {
        let bytes: [u8; 2] = [0, 126];
        assert_eq!(String::from("007e"), bytes_to_hex_string(&bytes));
    }

    #[test]
    fn encode_hex_test() {
        let result: Vec<u8> = vec![255,255];
        assert_eq!(result, encode_hex("ffff").unwrap());
    }

    #[test]
    fn encode_and_invert_test() {
        assert_eq!("00", encode_and_invert(&String::from("ff")));
    }

    #[test]
    fn encode_and_invert_test2() {
        assert_eq!("ff", encode_and_invert(&String::from("00")));
    }

    #[test]
    fn split_string_test() {
        let text: Vec<&str> = vec!["01", "23", "45", "67", "89"];
        assert_eq!(text, split_string(&String::from("0123456789"), 2));
    }

    #[test]
    fn letters_to_hex_test() {
        assert_eq!(String::from("007E7E5A18181818183C00000000007CC6FEC0C67C00000000007CC6701CC67C0000103030FC30303034180000183C3C3C181800181800"), letters_to_hex("Test!"));
    }

    #[test]
    fn keyword_to_hex_test() {
        assert_eq!("070e1b03212c2e26141c06806030808838e8c81030c0", keyword_to_hex('⚙'));
    }

    #[test]
    fn hex_to_keyword_test()  {
        assert_eq!("😄", hex_to_keyword("0008140801000061301c0700205020008080860c38e0"));
    }

    #[test]
    fn hex_string_to_letter_test()  {
        assert_eq!("4", hex_string_to_letter("000C1C3C6CCCFE0C0C1E00"));
    }

    #[test]
    fn letter_to_hex_test() {
        assert_eq!("000C1C3C6CCCFE0C0C1E00", letter_to_hex('4'));
    }

}