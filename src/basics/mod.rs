use chrono::Local;
use tera::Context;

pub fn context() -> Context {
    let mut context = Context::new();

    context.insert("date", &date());
    context.insert("time", &time());

    context
}

fn date() -> String {
    format!("{}", Local::today().format("%F"))
}
fn time() -> String {
    format!("{}", Local::now().format("%X"))
}
