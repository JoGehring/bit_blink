use crate::bluetooth::{Animation, Message, Speed};

//dummy implementation for testing message list
pub fn get_all_messages() -> Vec<Message> {
    let mut result = vec![];
    for i in 0..50 {
        let texts = vec![String::from("Hello world!")];
        let speed = vec![Speed::Six];
        let mode = vec![Animation::Left];
        let flash = vec![true];
        let marquee = vec![false];
        let inverted = vec![false];
        let bt_message1 = Message {file_name: "".to_owned(), texts, inverted, flash, marquee, speed, mode };
        result.push(bt_message1);
    }
    result
}