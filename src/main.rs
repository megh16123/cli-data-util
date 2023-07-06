use std::collections::HashMap;
use std::{env, fs};
fn main() {
    let mut args = env::args().skip(1);
    let command = match args.next() {
        Some(c) => c,
        None => String::from("default"),
    };
    let mut filename = match args.next() {
        Some(f) => f,
        None => String::from("default"),
    };
    filename = filename + ".db";
    // let content = format!("{key}\t{value}\n");
    // let write_result = std::fs::write("mt.db", content);
    // match write_result {
    //     Ok(()) => {
    //         println!("Added to Database");
    //     }
    //     Err(e) => {
    //         println!("Failed to write to Database {e}");
    //     }
    // }
    let mut database = Database::new(&filename).expect("DB failed");
    if command == "set" {
        let key = match args.next() {
            Some(key) => key,
            None => String::from(""),
        };
        let value = match args.next() {
            Some(val) => val,
            None => String::from(""),
        };
        database.insert(key, value);
    } else if command == "get" {
        match args.next() {
            Some(key) => {
                database.getter(&key);
            }
            None => database.get_all(),
        };
    } else if command == "clear" {
        database.do_clear();
    } else if command == "drop" {
        database.do_drop();
    } else if command == "list" {
        let _ = list_db();
    } else if command == "delete" {
        match args.next() {
            Some(key) => {
                database.deleter(&key);
            }
            None => database.get_all(),
        };
    } else {
        println!("Unknown command");
    }
    // database.insert(key.to_uppercase(), value.clone());
    // database.flush().unwrap();
    // Database::insert(database,key,value);
}

struct Database {
    map: HashMap<String, String>,
    filename: String,
    is_dropped: bool,
}

impl Database {
    fn new(filename: &String) -> Result<Database, std::io::Error> {
        // let content = std::fs::read_to_string("mt.db")?;
        let content = match fs::read_to_string(&filename) {
            Ok(contents) => contents,
            Err(_e) => {
                std::fs::File::create(&filename).unwrap();
                String::from("")
            }
        };
        let mut map = HashMap::new();
        // Parse the content string
        for line in content.lines() {
            // let (key,value) = line.split_once('\t').expect("Corrupted Database");
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key");
            let value = chunks.next().expect("No Value");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database {
            map,
            filename: filename.to_owned(),
            is_dropped: false,
        })
    }
    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }
    fn getter(&mut self, key: &String) {
        match self.map.get(key) {
            Some(val) => println!("{}", val),
            None => println!("No such key"),
        }
        self.is_dropped = true;
    }
    fn get_all(&mut self) {
        println!("Key\t:\tValue\n");
        for (key, value) in &self.map {
            println!("{}\t:\t{}", key, value);
        }
        self.is_dropped = true;
    }
    fn deleter(&mut self, key: &str) {
        self.map.remove(key);
    }
    fn do_clear(&mut self) {
        self.map.clear();
    }
    fn do_drop(&mut self) {
        match fs::remove_file(&self.filename) {
            Ok(()) => println!("File Removed !!"),
            Err(err) => println!("Error Occured {}", err),
        }
        self.is_dropped = true;
    }
    // fn flush(self) -> std::io::Result<()> {
    //     let mut contents = String::new();
    //     for (key, value) in &self.map {
    //         // let mtpair = format!("{}\t{}\n",key,value);
    //         // contents.push_str(&mtpair);
    //         contents.push_str(key);
    //         contents.push('\t');
    //         contents.push_str(value);
    //         contents.push('\n');
    //     }
    //     std::fs::write("mt.db", contents)
    // }
}

fn list_db() -> std::io::Result<()> {
    let directory_path = "./";
    let target_extension = "db"; // Specify the target extension
    println!("!!DATABASES!!\n");
    let entries = fs::read_dir(directory_path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        // Check if the entry is a file with the target extension
        if path.is_file() && path.extension() == Some(&std::ffi::OsStr::new(target_extension)) {
            let file_name = path.file_stem().unwrap();
            println!("   {}", file_name.to_string_lossy());
        }
    }

    Ok(())
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.is_dropped {
            let mut contents = String::new();
            for (key, value) in &self.map {
                // let mtpair = format!("{}\t{}\n",key,value);
                // contents.push_str(&mtpair);
                contents.push_str(key);
                contents.push('\t');
                contents.push_str(value);
                contents.push('\n');
            }
            let _ = fs::write(&self.filename, contents);
        }
    }
}
