use chrono::Local;
use tera::{Context, Tera};

fn main() {
    let filename = std::env::args().nth(1).expect("No template given");
    let template = std::fs::read_to_string(filename).expect("No such file or directory");

    let tera = match Tera::one_off(&template, &context(), true) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error: {}", e);
            ::std::process::exit(1);
        }
    };

    println!("{}", tera);
}

fn context() -> Context {
    let mut context = Context::new();
    let time = format!("{}", Local::now().format("%X"));
    let date = format!("{}", Local::today().format("%F"));

    context.insert("date", &date);
    context.insert("time", &time);

    context
}
