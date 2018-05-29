mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my::function()`");
}

pub fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_access() {
    print!("called `my::indirect_access()`, that\n> ");

    private_function();
}
