// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

// // I AM NOT DONE

fn main() {
    let mut res = 42;
    let option = Some(12);
    // for x in option {
    //     res += x;
    // }
    // 使用 while let 把 option 里面的值提取到 x 中(模式与表达式匹配成功的前提下)
    while let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
