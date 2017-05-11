#![feature(exclusive_range_pattern)]

mod dom;
mod parser;
mod css;
// use model;


pub fn test() {
    let html = "<html><body>Hello, world!</body></html>";

    let dommer = parser::Parser::parse(html.to_string());
    println!(", World!");
    println!("{:?}", dommer);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
