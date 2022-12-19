pub fn inspect(arg: &str) {
    if arg.ends_with('s') {
        println!("The contents of the string is plural");
    } else {
        println!("The contents of the string is singular");
    }
}

pub fn change(arg: &mut String) {
    if !arg.ends_with('s') {
        arg.push('s');
    }
}

#[must_use]
pub fn eat(arg: String) -> bool {
    arg.starts_with('b') && arg.contains('a')
}

pub fn bedazzle(arg: &mut String) {
    *arg = String::from("sparkly");
}
