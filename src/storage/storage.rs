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
        let mut target: String = format!("{}{}", format!("{}{}", &self.badge_storage_dir, f_name), &self.badge_ext);
        let mut file = File::create(&target).unwrap();
        fs::write(Path::new(&target), json).expect("Unable to write file")
    }
    fn save_clipart(&self, f_name: &String, png_bytes: &mut [u8]) {
        let mut target: String = format!("{}{}", format!("{}{}", &self.clip_storage_dir, f_name), &self.badge_ext);
        image::save_buffer(&target, png_bytes, 800, 600, image::ColorType::Rgb8).unwrap()   /// to be tested
    }
}
