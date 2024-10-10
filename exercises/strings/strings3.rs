// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.


fn trim_me(input: &str) -> String {
    // 注意：这里不需要将 &str 转换成 String，可以直接使用 trim 方法
    input.trim().to_string()
}

// 向字符串末尾添加 " world!"
fn compose_me(input: &str) -> String {
    format!("{} world!", input)
}

// 替换字符串中的 "cars" 为 "balloons"
fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
