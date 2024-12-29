// 引用是非拥有型指针，不同于 Box，Vec String 这样的拥有型指针，引用不拥有所指向的内存，因此不会在引用离开作用域时释放内存。
// 引用是 Rust 语言中的一种非拥有型指针，它允许你借用一个值，而不拥有它。引用是通过 & 符号来创建的，
// 例如 &T，其中 T 是被引用的值的类型。引用的生命周期是在编译时确定的，编译器会检查引用的生命周期是否符合 Rust 的借用规则。

use std::collections::HashMap;
use std::time::Instant;

struct Label {
    number: u32,
}
type Table = HashMap<String, Vec<String>>;

fn show(table: Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for works in table.values_mut() {
        works.sort();
    }
}

fn show_ref(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

#[test]
fn test_show() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of Saint Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    show(table);
    // assert_eq!(table.len(), 3); // 错误，因为在 show 函数中所有权已经转移
}

#[test]
fn test_show_ref() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of Saint Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    show_ref(&table);
    assert_eq!(table.len(), 3); // 因为在 show_ref 函数中传递的引用，所以没有所有权转移

    sort_works(&mut table);
    show_ref(&table);
}

#[test]
fn test_deref() {
    // 显示解引用
    let x = 10;
    let r = &x;
    assert_eq!(*r, 10); // 显示解引用

    // . 操作符号可以隐式解引用
    let x = Label { number: 3 };
    let r = &x;
    assert_eq!((*r).number, 3); // 显示解引用
    assert_eq!(r.number, 3) // . 操作符隐式解引用
}

#[test]
fn test_modify_ref() {
    // 给应用重新赋值，会改变引用的指向，而不会修改引用原来的值，这和 C++ 皆然相反
    let x = 10;
    let y = 20;
    let r = &x;
    assert_eq!(*r, 10);
    let r = &y;
    assert_eq!(*r, 20);
    assert_eq!(x, 10); // 在 C++ 中引用被重新赋值会修改引用指向的值，而在 Rust 中 x 的值没有改变
}

#[test]
fn test_ref_ref() {
    // 引用的引用
    // 引用的引用在Rust中并不常见，但在某些情况下可能会有用。
    // 一个经典的使用场景是当你需要传递一个引用的引用给一个函数时，这个函数需要对引用本身进行操作，而不是对引用指向的值进行操作
    let x = 5;
    let r = &x; // r 是 x 的引用
    let rr = &r; // rr 是 r 的引用，即引用的引用

    assert_eq!(x, 5);
    assert_eq!(*r, 5);
    assert_eq!(**rr, 5);
    assert_eq!(***&rr, 5);
}

#[test]
fn test_ref_compare() {
    // 引用的比较
    let x = 10;
    let y = 10;
    let rx = &x;
    let rx1 = &x;
    let ry = &y;

    assert_eq!(rx, ry); // 引用的比较是比较引用指向的值
    assert!(!std::ptr::eq(rx, ry)); // eq 比较的是引用的地址，所以不等
    assert!(std::ptr::eq(rx, rx1)); // rx 和 rx1 指向的都是相同的地址 x 所以相等
}

#[test]
fn test_ref_null() {
    // 引用的空指针
    // Rust 中没有空指针，所以引用也不可能为空
    // let r: &i32 = null; // 错误，引用不可能为空

    struct Container {
        value: i32,
    }

    impl Container {
        fn new(value: i32) -> Self {
            Container { value }
        }

        fn maybe_get_value(&self, condition: bool) -> Option<&i32> {
            if condition {
                Some(&self.value) // 返回对结构体字段的引用
            } else {
                None // 返回一个空引用
            }
        }
    }

    let container = Container::new(5);

    if let Some(value) = container.maybe_get_value(true) {
        println!("{}", value); // 输出: 5
    } else {
        println!("没有值");
    }
}

#[test]
fn test_expression_ref() {
    fn factorial(n: usize) -> usize {
        (1..n + 1).product()
    }

    let iterations = 10_000_000;

    // Measure time for using reference
    let start = Instant::now();
    for _ in 0..iterations {
        let r = &factorial(6); // avoid data copying
        let _ = r + &1000;
    }
    let duration_ref = start.elapsed();

    // Measure time for not using reference
    let start = Instant::now();
    for _ in 0..iterations {
        let result = factorial(6); // data copying
        let _ = result + 1000;
    }
    let duration_no_ref = start.elapsed();

    println!("Time with reference: {:?}", duration_ref);
    println!("Time without reference: {:?}", duration_no_ref);
    assert!(duration_ref > duration_no_ref); // 这是一个错误的断言
}

#[test]
fn test_expression_ref2() {
    fn factorial(n: usize) -> usize {
        (1..n + 1).product()
    }

    let iterations = 10_000_000;
    let mut total_duration_ref = std::time::Duration::new(0, 0);
    let mut total_duration_no_ref = std::time::Duration::new(0, 0);
    let test_runs = 10;

    for _ in 0..test_runs {
        // Measure time for using reference
        let start = Instant::now();
        for _ in 0..iterations {
            let r = &factorial(6); // avoid data copying
            let _ = r + &1000;
        }
        total_duration_ref += start.elapsed();

        // Measure time for not using reference
        let start = Instant::now();
        for _ in 0..iterations {
            let result = factorial(6); // copying
            let _ = result + 1000;
        }
        total_duration_no_ref += start.elapsed();
    }

    let avg_duration_ref = total_duration_ref / test_runs;
    let avg_duration_no_ref = total_duration_no_ref / test_runs;

    println!("Average time with reference: {:?}", avg_duration_ref);
    println!("Average time without reference: {:?}", avg_duration_no_ref);
    assert!(avg_duration_ref > avg_duration_no_ref); // 这是一个错误的断言
}
