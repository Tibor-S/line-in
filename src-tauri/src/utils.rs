use chrono::Utc;
use std::env;

pub fn log(file: &str, fnname: &str, msg: &str) -> () {
    println!("[{}] ({}:{}): {}", Utc::now(), file, fnname, msg)
}

pub fn tmp_sample_file() -> String {
    match env::var("TMP_SAMPLE_FILE") {
        Ok(v) => v,
        Err(e) => {
            panic!("Error: {}", e)
        }
    }
}
