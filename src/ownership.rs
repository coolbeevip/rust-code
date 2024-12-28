use std::rc::Rc;

fn foo(v: Vec<i32>) {
    println!("{:?}", v)
}

fn foo_mut(v: &mut Vec<i32>) {
    // 可变引用传递的是引用，所以不会发生所有权转移
    v.push(5);
    println!("{:?}", v)
}

// 导致引用悬垂
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // 返回对局部变量的引用
// } // s 在这里被释放，返回的引用变成悬垂引用

struct Label {
    number: u32,
}

#[derive(Copy, Clone)]
struct LabelCopy {
    number: u32,
}

// #[derive(Copy, Clone)]
// struct LabelMistake{number: String} 因为 String 不是 Copy 类型，所以这个声明会失败

#[test]
fn test_ownership() {
    // 所有权转移
    let s = vec!["a", "b"];
    let t = s.clone(); // 此处使用了复制，所有权并没有发生转移
    let u = s; // s的所有权转移到u
               // let t = s; // 错误: 因为 s 的所有权已经转移到了 u
    assert_eq!(u, vec!["a", "b"]);
    assert_eq!(t, vec!["a", "b"]);

    // 因为 "hello" &'static str 是不可变引用，所以不存在所有权转移问题
    let a = "hello";
    let b = a;
    let c = a;
    assert_eq!(a, "hello");
    assert_eq!(b, "hello");
    assert_eq!(c, "hello");

    // 函数接管所有权
    let x = vec![1, 2, 3];
    foo(x);
    // assert_eq!(x, vec![1, 2, 3]); // 错误: x 的所有权已经转移到了 foo 函数

    // 可变引用不会导致所有权转移
    let mut y = vec![1, 2, 3];
    foo_mut(&mut y);
    assert_eq!(y, vec![1, 2, 3, 5]); // 正确: y 的所有权没有转移

    // 并不是所有对象的拥有都能被移动
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }
    // let third = v[2]; // 这个赋值会失败，不能移动到 Vec 索引结构之外
    let third = v[2].clone(); // 正确: 使用 clone 方法复制字符串
    assert_eq!(third, "103");
    let third = v.get(2).unwrap(); // 正确: 使用 get 方法获取引用
    assert_eq!(third, "103");
    let third = &v[2]; // 正确: 使用引用
    assert_eq!(third, "103");

    // 循环 vec 会导致所有权转移，循环后 v 将被设置成未初始化状态
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }
    for s in &v {
        // 使用引用循环不会发生所有权转移
        println!("{}", s);
    }
    assert_eq!(v, vec!["101", "102", "103", "104", "105"]);
    for s in v {
        // 所有权转移
        println!("{}", s);
    }
    // assert_eq!(v, vec!["101", "102", "103", "104", "105"]); // 错误 v 未初始化

    // Copy 类型不会发生所有权移动
    let a = 5;
    let _b = a; // 所有的整数，浮点数，布尔值，字符，元组，固定大小的数组都是 Copy 类型，赋值时会复制值
    assert_eq!(a, 5);

    // 自定义类型默认不是 Copy 类型，会发生所有权转移
    let x = Label { number: 3 };
    assert_eq!(x.number, 3);
    let _y = x; // x 的所有权转移到 y
                // assert_eq!(x.number, 3); 错误：发生了所有权转移

    // 自定义类型显示的声明为 Copy 类型
    let x = LabelCopy { number: 3 };
    let _y = x; // 因为 LabelCopy 被声明成了 Copy 类型，所以会复制，而不是所有权转移
    assert_eq!(x.number, 3);

    // 引用计数器 Rc, Arc(线程安全)
    // 引用计数器可以让一个数值拥有多个所有者，但是这限制了不可修改
    let s: Rc<String> = Rc::new("hello".to_string());
    let t: Rc<String> = s.clone(); // clone 方法并不会复制底层的数据，而是增加引用计数器的计数。
    let u: Rc<String> = s.clone(); // 这意味着多个 Rc<T> 实例可以共享同一个底层数据，而不会发生所有权转移
    assert!(s.contains("he"));
    assert_eq!(t.find("ll"), Some(2));
    assert_eq!(u.len(), 5);
    // s.push_str(" world"); // 错误: 计数器引用对象不允许修改
}
