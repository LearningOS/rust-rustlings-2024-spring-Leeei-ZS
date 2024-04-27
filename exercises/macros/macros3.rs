// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

// // I AM NOT DONE


mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
    // 将一个模块中的项公开导出到模块外部
    pub use my_macro;
    
}
fn main() {
    macros::my_macro!();
}
