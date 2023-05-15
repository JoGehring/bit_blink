use std::{env, fs};
use std::fs::File;
use std::path::Path;

struct Storage {
    badge_storage_dir: String,
    clip_storage_dir: String,
    badge_ext: String,
    clip_ext: String
}

impl Storage {
    fn build_storage() -> Storage {
        let main_dir : String = Storage::create_and_get_storage_dir();
        Storage {
            clip_storage_dir: format!("{}{}", main_dir, String::from("/ClipArts/")),
            badge_storage_dir: main_dir,
            badge_ext: String::from(".txt"),
            clip_ext: String::from("png")
        }
    }
    fn create_and_get_storage_dir() -> String {
        let mut working_dir: String = format!("{}{}", env::current_dir().unwrap().into_os_string().into_string().unwrap(), String::from("/badgeMagicData/"));
        fs::create_dir_all(&working_dir).unwrap();
        working_dir
    }
    fn save_badge(&self, f_name: &String, json: &String) {
        let target: String = self.get_full_badge_filename(&f_name);
        File::create(&target).unwrap();
        fs::write(Path::new(&target), json).expect("Unable to write file")
    }
    fn delete_badge(&self, f_name: &String) {
        fs::remove_file(self.get_full_badge_filename(&f_name)).expect("File couldn't be deleted");
        println!("File deleted successfully!");
    }
    fn import_badge(&self, path_to_file: &String) {   // will the file path given as a String or as a Path element? + does the path look like (...)/<fileName>/ or like (...)/<fileName>
        // current implementation assumes the latter since it is the standard when copying the path of a file in windows OS
        let mut parts: Vec<&str> = path_to_file.split("/").collect();
        let f_name: &str = parts[&parts.len()-1];
        fs::copy(path_to_file, self.get_full_badge_filename(&f_name.to_owned())).expect("Badge Import failed");
    }
    fn save_clipart(&self, f_name: &String, png_bytes: &mut [u8]) {
        image::save_buffer(self.get_full_clipart_filename(&f_name), &png_bytes, 800, 600, image::ColorType::Rgb8).unwrap();  // to be tested
    }
    fn delete_clipart(&self, f_name: &String) {
        fs::remove_file(self.get_full_clipart_filename(&f_name)).expect("File couldn't be deleted");
    }
    fn get_full_badge_filename(&self, f_name: &String) -> String {
        format!("{}{}", format!("{}{}", &self.badge_storage_dir, f_name), &self.badge_ext)
    }
    fn get_full_clipart_filename(&self, f_name: &String) -> String {
        format!("{}{}", format!("{}{}", &self.badge_storage_dir, f_name), &self.badge_ext)
    }
}
