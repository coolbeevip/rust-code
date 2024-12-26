fn build_vector() -> Vec<i16> {
    let mut v = Vec::new(); // v 没有声明类型，但是 Rust 可以根据返回类型推断出来
    v.push(10);
    v.push(20);
    v.push(30);
    v
}

fn split_at(input: &str, mid: usize) -> (&str, &str) {
    let len = input.len();
    let left = &input[..mid];
    let right = &input[mid..len];
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_vector() {
        let v = build_vector();
        assert_eq!(v, vec![10, 20, 30]);
    }

    #[test]
    fn test_split_at() {
        let (left, right) = split_at("hello, world!", 7);
        assert_eq!(left, "hello, ");
        assert_eq!(right, "world!");
    }

    #[test]
    fn test_array() {
        // 数组的长度会在编译期固定下来，不可变
        let a = [1, 3, 2]; // 定义时初始化
        assert_eq!(a, [1, 3, 2]);
        assert_eq!(a.len(), 3);

        let b = [0i32; 3]; // 定义自填充数组
        assert_eq!(b, [0, 0, 0]);

        // 数组的操作必须时 mut
        let mut c = [1, 3, 2];
        c.sort(); // 会自动转换为切片方法
        assert_eq!(c, [1, 2, 3]);
        let _ = &c.reverse(); // 切片方法
        assert_eq!(c, [3, 2, 1]);
    }

    #[test]
    fn test_vector() {
        // 创建一个可变向量，增加或删除元素
        let mut v = Vec::new();
        v.push(1);
        v.push(2);
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], 1);
        assert_eq!(v.pop(), Some(2));
        assert_eq!(v.len(), 1);

        // 使用宏创建向量
        let mut vm = vec![1, 2, 3];

        // 如果想让一个可变变量变成不可变，可以使用 &mut
        let _v = &mut vm;

        // 使用宏创建不可变向量
        let _vr = vec![1, 2, 3];

        // 通过迭代器创建向量
        let _vri: Vec<i32> = (0..5).collect();

        let mut palindrome = vec!["a man", "a plan", "panama"];
        palindrome.reverse();
        assert_eq!(palindrome, vec!["panama", "a plan", "a man"]);
    }

    #[test]
    fn test_string() {
        // 字符串字面量
        let mut speech = "Hello World!";
        println!("{}", speech);

        // 多行字符串字面量
        speech = "Hello,
        Rust!";
        println!("{}", speech);

        // 使用 \ 换行丢弃前导空格和换行
        speech = "Hello, \
        Rust!";
        println!("{}", speech);

        // 原始字符串避免转义
        speech = r#"Hello, "Rust"!"#;
        println!("{}", speech);

        // 字节串
        let b = b"hello";
        assert_eq!(b, &[b'h', b'e', b'l', b'l', b'o']);

        // 字符串是 UTF-8 编码的
        let mut s = String::new();
        s.push('a');
        s.push_str("b");
        assert_eq!(s, "ab");

        // 使用宏创建字符串
        let s = String::from("hello, ");
        let s = s + "world!";
        assert_eq!(s, "hello, world!");

        // 使用 format! 宏创建字符串
        let s = format!("{}-{}-{}", "tic", "tac", "toe");
        assert_eq!(s, "tic-tac-toe");

        // 字符串切片
        let s = "hello, world!";
        let hello = &s[0..5];
        let world = &s[7..12];
        assert_eq!(hello, "hello");
        assert_eq!(world, "world");

        // 对于 Unicode 文本使用 String 或者 &str
        // 当使用文件名时使用 std::path::PathBuf 或者 &std::path::Path
        // 当处理二进制时使用 Vec<u8> 或者 &[u8]
        // 当处理操作系统原生字符串时使用 std::ffi::OsString 或者 &std::ffi::OsStr
        // 当与 C 代码交互时使用 std::ffi::CString 或者 &std::ffi::CStr
    }
}
