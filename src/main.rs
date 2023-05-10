use getopts::Options;
use std::env;
use tera::{Context, Tera};

mod basics;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} TEMPLATE_FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optmulti("v", "var", "provide name=value variable pair", "NAME=VAL");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => { panic!("Error: {}", f) }
    };

    // print help when requested
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    // Filename is the first free option
    let filename = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        eprintln!("No template file given");
        print_usage(&program, opts);
        ::std::process::exit(1);
    };
    let template = std::fs::read_to_string(filename).expect("No such file or directory");

    // Extract name value pairs.
    // TODO: avoid mut. Assign from an iter() maybe?
    // TODO: investigate why the borrowchecker accepts a String but not a str
    // and how and if we want to fix that?
    let mut user_pairs: Vec<Vec<String>> = vec![];
    for pair in &matches.opt_strs("v") {
        user_pairs.push(pair.splitn(2, '=').map(|part| part.to_string()).collect());
    }

    let tera = match Tera::one_off(&template, &context(user_pairs), true) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error: {}", e);
            ::std::process::exit(1);
        }
    };

    println!("{}", tera);
}

fn context(user_pairs: Vec<Vec<String>>) -> Context {
    let mut context = basics::context();

    for pair in user_pairs {
        context.insert(pair[0].as_str(), pair[1].as_str());
    }

    context
}
