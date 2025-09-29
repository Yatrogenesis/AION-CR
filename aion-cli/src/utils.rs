use colored::*;

pub fn print_banner() {
    println!("{}", r#"
   ___    ____  ____  _   __      ______ ____
  / _ |  /  _/ / __ \/ | / /     / ____// __ \
 / __ | _/ /  / / / /  |/ /_____/ /    / /_/ /
/_/ |_|/___/ /_/ /_/_/|_/_____/_/     \____/

Artificial Intelligence Operations Network
Compliance & Regulations Framework
"#.cyan().bold());
    println!("{}", "Advanced Normative Conflict Resolution System".green());
    println!("{}", "Version 1.0.0\n".yellow());
}

pub fn print_success(message: &str) {
    println!("{} {}", "✅".green(), message);
}

pub fn print_error(message: &str) {
    println!("{} {}", "❌".red(), message);
}

pub fn print_warning(message: &str) {
    println!("{} {}", "⚠️".yellow(), message);
}

pub fn print_info(message: &str) {
    println!("{} {}", "ℹ️".blue(), message);
}