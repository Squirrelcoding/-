use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

pub fn set_secret(secret: &str) {
    let home_dir = dirs::home_dir().unwrap().into_os_string().into_string().unwrap();
    let dir_path = format!("{}/.moyaibot", home_dir);
    if !Path::new(&dir_path).exists() {
        fs::create_dir(&dir_path).unwrap();
    }
    let mut file = File::create(format!("{}/secret.txt", dir_path)).unwrap();
    file.write_all(secret.as_bytes()).unwrap();
}

pub fn get_secret() -> String {
    let home_dir = dirs::home_dir().unwrap().into_os_string().into_string().unwrap();
    let dir_path = format!("{}/.moyaibot", home_dir);
    let mut secret = String::new();
    File::open(format!("{}/secret.txt", dir_path))
        .unwrap()
        .read_to_string(&mut secret)
        .unwrap();
    secret
}
