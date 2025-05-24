use crossterm::style::Stylize;

#[inline]
pub fn log_error(input: &str) {
    eprintln!("{}", format!("🟥 {input}").red().bold());
}

#[inline]
pub fn log_warn(input: &str) {
    println!("{}", format!("🟨 {input}").yellow().bold());
}

#[inline]
pub fn log_success(input: &str) {
    println!("{}", format!("🟩 {input}").green().bold());
}

#[inline]
pub fn log_info(input: &str) {
    println!("{}", format!("🟦 {input}").blue().bold());
}
