// 在 Rust 中，生命周期的定义是为了确保引用的安全性

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    // 当函数接受多个引用，并且返回一个引用时，编译器需要知道这些引用之间的关系，以确保返回的引用不会超出任何输入引用的生命周期。
    // 这种情况下就需要定义生命周期
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
} // 函数签名中的生命周期注解表明 longest 函数返回的引用将生存于参数 s1 和 s2 中较短的那个生命周期中

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        let s1 = String::from("long string is long");
        let s2 = String::from("xyz");
        let result;
        {
            result = longest(s1.as_str(), s2.as_str());
        }
        assert_eq!(result, "long string is long");
    }

    #[test]
    fn test_ref_parameter() {
        // 把引用传递给函数，会检查引用的生命周期是否符合函数的生命周期定义
        static P: i32 = 10;
        fn f(p: &'static i32) -> &i32 {
            p
        }

        let x = 10;
        _ = f(&P);
        // _ = f(&x); 编译失败，因为 x 的生命周期比 'static 短
    }

    #[test]
    fn test_ref_struct() {
        // 包含引用的结构体 S 的生命周期不能超过引用的生命周期
        struct S<'a> {
            r: &'a i32
        }

        let s;
        let x = 10;
        {
            // let x = 10; // 如果 x 在这里定义则编译失败，因为 x 的生命周期比 s 的生命周期短
            s = S { r: &x };
        }
        assert_eq!(s.r, &10);
    }
}
