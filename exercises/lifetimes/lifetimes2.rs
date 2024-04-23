// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.

// // I AM NOT DONE

// 既然引用会导致生命周期冲突，那直接不使用引用
fn longest<'a>(x: &'a str, y: &'a str) -> String {
    if x.len() > y.len() {
        x.to_string()
    } else {
        y.to_string()
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        // result 活的跟 longest 函数中最短的一样
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is '{}'", result);
}
