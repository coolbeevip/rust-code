// 引用是非拥有型指针，不同于 Box，Vec String 这样的拥有型指针，引用不拥有所指向的内存，因此不会在引用离开作用域时释放内存。
// 引用是 Rust 语言中的一种非拥有型指针，它允许你借用一个值，而不拥有它。引用是通过 & 符号来创建的，
// 例如 &T，其中 T 是被引用的值的类型。引用的生命周期是在编译时确定的，编译器会检查引用的生命周期是否符合 Rust 的借用规则。

use std::collections::HashMap;

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
    table.insert("Gesualdo".to_string(), vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(), vec!["The Musicians".to_string(), "The Calling of Saint Matthew".to_string()]);
    table.insert("Cellini".to_string(), vec!["Perseus with the head of Medusa".to_string(), "a salt cellar".to_string()]);

    show(table);
    // assert_eq!(table.len(), 3); // 错误，因为在 show 函数中所有权已经转移
}

#[test]
fn test_show_ref() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(), vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(), vec!["The Musicians".to_string(), "The Calling of Saint Matthew".to_string()]);
    table.insert("Cellini".to_string(), vec!["Perseus with the head of Medusa".to_string(), "a salt cellar".to_string()]);

    show_ref(&table);
    assert_eq!(table.len(), 3); // 因为在 show_ref 函数中传递的引用，所以没有所有权转移

    sort_works(&mut table);
    show_ref(&table);
}