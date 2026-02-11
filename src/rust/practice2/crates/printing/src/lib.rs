pub mod common_prints {
    use console::style;

    pub fn say_hello_blue() {
        println!("{}", style("Hello, world!").blue().italic());
    }
}
