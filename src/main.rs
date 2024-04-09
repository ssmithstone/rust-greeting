use std::env;

fn main() {
    print_greeting();

}

fn print_greeting() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        let name = "Luke Skywalker".to_string();
        println!("Hello, {}!", name);
    } else {
        let name = args.get(1);
        match name {
            Some(n) => println!("Hello, {}!", n),
            None => panic!(),
        }
    }
}
