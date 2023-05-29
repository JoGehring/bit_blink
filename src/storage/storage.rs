use std::{env, fs};
use std::fs::File;
use std::path::Path;

use crate::bluetooth::Message;
use crate::bluetooth::utils::{hex_string_to_letter, split_string};

pub struct Storage {
    badge_storage_dir: String,
    clip_storage_dir: String,
    badge_ext: String,
    clip_ext: String,
}

impl Storage {
    fn create_and_get_storage_dir() -> String {
        let mut working_dir: String = format!("{}{}", env::current_dir().unwrap().into_os_string().into_string().unwrap(), String::from("/badgeMagicData/"));
        fs::create_dir_all(&working_dir).unwrap();
        working_dir
    }
    pub fn save_message(&self, message: &Message) {
        let json = hex_string_to_json(message);
        let timestamp = chrono::Utc::now().timestamp().to_string();
        let target: String = self.get_full_badge_filename(&timestamp);
        File::create(&target).unwrap();
        fs::write(Path::new(&target), json).expect("Unable to write file")
    }
    fn load_badge(&self, f_name: &String) -> Message {
        let target: String = self.get_full_badge_filename(&f_name);
        let message: Message = json_to_message(&fs::read_to_string(target).expect("Unable to read file"));
        message
    }
    fn delete_badge(&self, f_name: &String) {
        fs::remove_file(self.get_full_badge_filename(&f_name)).expect("File couldn't be deleted");
        println!("File deleted successfully!");
    }
    fn import_badge_to_app_dir(&self, path_to_file: &String) {   // will the file path given as a String or as a Path element? + does the path look like (...)/<fileName>/ or like (...)/<fileName>
        // current implementation assumes the latter since it is the standard when copying the path of a file in windows OS
        let mut parts: Vec<&str> = path_to_file.split("/").collect();
        let f_name: &str = parts[&parts.len() - 1];
        fs::copy(path_to_file, self.get_full_badge_filename(&f_name.to_owned())).expect("Badge Import failed");
    }
    fn save_clipart(&self, f_name: &String, png_bytes: &mut [u8]) { // TODO: Define JSON fields
        image::save_buffer(self.get_full_clipart_filename(&f_name), &png_bytes, 800, 600, image::ColorType::Rgb8).unwrap();  // to be tested
    }
    fn delete_clipart(&self, f_name: &String) {
        fs::remove_file(self.get_full_clipart_filename(&f_name)).expect("File couldn't be deleted");
    }
    fn get_full_badge_filename(&self, f_name: &String) -> String {
        let filename = (self.badge_storage_dir.clone() + f_name) + &self.badge_ext;
        filename
    }
    fn get_full_clipart_filename(&self, f_name: &String) -> String {
        let filename = (self.clip_storage_dir.clone() + f_name) + &self.clip_ext;
        filename
    }
}

fn json_to_message(mut json: &String) -> Message {
    let mut json_copy = json.clone();
    if (json.contains("hex_strings")) {
        json_copy = json.replace("hex_strings", "texts");
    }
    let mut message: Message = serde_json::from_str(&*json_copy).unwrap();
    for i in 0..message.texts.len() {
        let subs: Vec<&str> = split_string(&message.texts[i], 22);
        let mut hex_string: String = "".to_owned();
        for sub in subs.into_iter() {
            hex_string = hex_string + hex_string_to_letter(sub);
        }
        message.texts[i] = hex_string;
    }
    message
}

fn hex_string_to_json(message: &Message) -> String {
    let json: String = serde_json::to_string(&message).unwrap();
    json
}

pub fn build_storage() -> Storage {     // needs to be executed before the Storage struct can be used
    // todo: call methode from constructor only
    let main_dir: String = Storage::create_and_get_storage_dir();
    Storage {
        clip_storage_dir: main_dir.clone() + &String::from("/ClipArts/"),
        badge_storage_dir: main_dir,
        badge_ext: String::from(".json"),
        clip_ext: String::from("png"),
    }
}