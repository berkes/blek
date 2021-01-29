use tera::{Context, Tera};

mod basics;

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
    context.extend(basics::context());
    context
}
