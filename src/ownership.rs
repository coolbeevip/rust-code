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
    for i in 101 .. 106 {
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
    for i in 101 .. 106 {
        v.push(i.to_string());
    }
    for s in &v { // 使用引用循环不会发生所有权转移
        println!("{}", s);
    }
    assert_eq!(v, vec!["101", "102", "103", "104", "105"]);
    for s in v { // 所有权转移
        println!("{}", s);
    }
    // assert_eq!(v, vec!["101", "102", "103", "104", "105"]); // 错误 v 未初始化
}
