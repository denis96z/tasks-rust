use crate::students::{load_students, save_students};
use argparse::{ArgumentParser, Store};

mod students;

fn main() {
    let mut input_filename = String::new();
    let mut output_filename = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut input_filename).required().add_argument(
            "input_filename",
            Store,
            "name of the YAML file to load data from",
        );
        ap.refer(&mut output_filename).required().add_argument(
            "output_filename",
            Store,
            "name of the YAML file to save data to",
        );
        ap.parse_args_or_exit();
    }

    let mut db = match load_students(&input_filename) {
        Ok(v) => v,
        Err(err) => {
            eprintln!("failed to parse students database: {}", err);
            std::process::exit(1);
        }
    };

    for student in &mut db {
        student.grades.insert(String::from("IT"), 5);
    }

    if let Err(err) = save_students(&output_filename, &db) {
        eprintln!("failed to save students database: {}", err);
        std::process::exit(1);
    };
}
