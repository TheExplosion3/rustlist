pub mod fileman {
    use std::{fs::File, io::Write, path::Path};

    use crate::structs::ListItem;

    pub fn opener(path_destination: &str) -> File {
        let path = Path::new(path_destination);
        let display = path.display();

        let f = match File::open(&path) {
            Err(e) => panic!("couldn't open {}: {}", display, e),
            Ok(file) => file,
        };

        return f;


    }

    pub fn saver(path: &str, list: Vec<ListItem>) {
        let p = Path::new(path);
        let display = p.display();

        let mut f = match File::open(&p) {
            Err(e) => panic!("couldn't open {}: {}", display, e),
            Ok(file) => file,
        };

        let mut writer: String = String::new();

        for i in list.clone().into_iter() {

            writer = writer + &i.item_id.to_string() + ". " + &i.contents;

            if i.item_id - 1 == u128::try_from(list.len()).unwrap() {
                f.write(writer.as_bytes()).expect("write failed");
            }
            else {
                writer = writer + "\n";
                f.write(writer.as_bytes()).expect("write failed");
            }
        }
    }

    pub fn file_saver(mut file_to_save: &File, list: Vec<ListItem>) {
        let mut writer: String = String::new();

        for i in list.clone().into_iter() {

            file_to_save.set_len(0).expect("File cannot be negatively sized.");
            writer = writer + &i.item_id.to_string() + ". " + &i.contents;

            if i.item_id - 1 == u128::try_from(list.len()).unwrap() {
                file_to_save.write(writer.as_bytes()).expect("write failed");
            }
            else {
                writer = writer + "\n";
                file_to_save.write(writer.as_bytes()).expect("write failed");
            }
        }
    }
}