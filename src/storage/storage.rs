use std::{env, fs};
use std::fs::File;
use std::path::Path;

use crate::bluetooth::Message;
use crate::bluetooth::utils::{hex_string_to_letter, hex_to_keyword, split_string};

#[derive(Debug, Clone)]
pub struct Storage {
    badge_storage_dir: String,
    clip_storage_dir: String,
    badge_ext: String,
}

impl Storage {
    fn create_and_get_storage_dir() -> String {
        let working_dir: String = format!("{}{}", env::current_dir().unwrap().into_os_string().into_string().unwrap(), String::from("/bitBlinkData/"));
        fs::create_dir_all(&working_dir).unwrap();
        working_dir
    }
    pub fn save_message(&self, message: &mut Message) {
        let timestamp = chrono::Utc::now().format("%d-%m-%Y-%M-%S-%f").to_string();
        let target: String = self.get_full_badge_filename(&timestamp) + &*self.badge_ext;
        message.file_name = timestamp + &*self.badge_ext;
        let json = hex_string_to_json(message);
        File::create(&target).unwrap();
        fs::write(Path::new(&target), json).expect("Unable to write file")
    }
    pub fn build_single_message_from_first_text_vec_of_given_messages(&self, given_messages : &Vec<Message>) -> Message {   //given_messages must not longer than 8 elements long
        let mut result_message : Message = Message {
            file_name: "".to_owned(),
            texts: vec![], //,"test2".to_owned(), "test3".to_owned()],
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

    pub fn get_all_messages(&self) -> Vec<Message> {
        let mut messages : Vec<Message> = vec![];
        let paths = fs::read_dir("./bitBlinkData").unwrap();
        for path in paths {
            let file_name : String = path.unwrap().file_name().into_string().unwrap();
            if file_name.contains(&self.badge_ext) {
                messages.push(self.load_badge(&file_name));
            }
        }
        messages
    }
    fn load_badge(&self, f_name: &String) -> Message {
        let target: String = self.get_full_badge_filename(&f_name);
        let message: Message = json_to_message(&fs::read_to_string(target).expect("Unable to read file"));
        message
    }
    pub fn delete_badge(&self, f_name: &String) {
        fs::remove_file(self.get_full_badge_filename(&f_name)).expect("File couldn't be deleted");
        println!("File deleted successfully!");
    }
    fn import_badge_to_app_dir(&self, path_to_file: &String) {   // will the file path given as a String or as a Path element? + does the path look like (...)/<fileName>/ or like (...)/<fileName>
        // current implementation assumes the latter since it is the standard when copying the path of a file in windows OS
        let parts: Vec<&str> = path_to_file.split("/").collect();
        let f_name: &str = parts[&parts.len() - 1];
        fs::copy(path_to_file, self.get_full_badge_filename(&f_name.to_owned())).expect("Badge Import failed");
    }
    fn get_full_badge_filename(&self, f_name: &String) -> String {
        let filename = self.badge_storage_dir.clone() + f_name;
        filename
    }
}

fn json_to_message(json: &String) -> Message {
    let mut json_copy = json.clone();
    if json.contains("hex_strings") {
        json_copy = json.replace("hex_strings", "texts");
    }
    let mut message: Message = serde_json::from_str(&*json_copy).unwrap();
    for i in 0..message.texts.len() {
        let message_text = message.texts[i].clone();
        let subs: Vec<&str> = split_string(&message_text, 22);
        let mut hex_string: String = "".to_owned();
        for j in 0..subs.len() {
            let mut letter = hex_string_to_letter(subs[j]);
            if letter == "" {
                letter = hex_to_keyword(subs[j]);
                if letter == "" && j < subs.len()-1 {
                    letter = hex_to_keyword((subs[j].to_owned() + subs[j+1]).as_str());
                }
                if letter == "" && j < subs.len()-2 {
                    letter = hex_to_keyword((subs[j].to_owned() + subs[j+1] + subs[j+2]).as_str());
                }
            }
            hex_string = hex_string + letter;
        }
        message.texts[i] = hex_string;
    }
    message
}

fn hex_string_to_json(message: &Message) -> String {
    let json: String = serde_json::to_string(&message).unwrap();
    println!("{}", json);
    json
}

pub fn build_storage() -> Storage {     // needs to be executed before the Storage struct can be used
    // todo: call methode from constructor only
    let main_dir: String = Storage::create_and_get_storage_dir();
    Storage {
        clip_storage_dir: main_dir.clone() + &String::from("/ClipArts/"),
        badge_storage_dir: main_dir,
        badge_ext: ".json".to_owned()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::bluetooth::message;
    use crate::bluetooth::message::{Animation, Message, Speed};

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

    fn initialize_empty_storage_for_test() -> Storage {
        let storage = build_storage();
        delete_all_message(&storage);
        storage
    }

    fn initialize_storage_for_test() -> Storage {
        let storage = build_storage();
        delete_all_message(&storage);
        let mut message = give_example_message();
        storage.save_message(&mut message);
        storage
    }

    fn delete_all_message(storage: &Storage) {
        for m in storage.get_all_messages()  {
            println!("{}", &m.file_name);
            storage.delete_badge(&m.file_name);
        }
    }

    #[test]
    fn save_message_test() {
        let storage = initialize_empty_storage_for_test();
        let mut message = give_example_message();
        storage.save_message(&mut message);
        let result = storage.get_all_messages();

        let message_result = storage.load_badge(&result[0].file_name);

        assert_eq!("test", message_result.texts[0]);
    }

    #[test]
    fn delete_badge_test() {
        let storage = initialize_storage_for_test();
        let v1 = storage.get_all_messages();

        storage.delete_badge(&v1[0].file_name);
        let v2 = storage.get_all_messages();

        assert_eq!(v1.len() - 1, v2.len());
    }

    #[test]
    fn get_all_messages_test() {
        let storage = initialize_storage_for_test();
        let result = storage.get_all_messages();

        assert_eq!(1, result.len());
    }

    /*
    #[test]
    fn build_single_message_from_first_text_vec_of_given_messages_test () {
        let file_name = String::from("Test");
        let texts = vec!(String::from("test"));
        let inverted = vec!(false);
        let flash = vec!(false);
        let marquee = vec!(false);
        let speed = vec!(Speed::One);
        let mode = vec!(Animation::Left);

        let message_result = Message{file_name, texts, inverted, flash, marquee, speed, mode};

        let message = give_example_message();
        let storage = initialize_storage_for_test();
        storage.build_single_message_from_first_text_vec_of_given_messages(&message);
        assert!(true);
    }
     */

}
