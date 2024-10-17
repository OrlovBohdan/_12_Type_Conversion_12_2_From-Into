#[test]

/*
fn main() {
    // impl From<bool> for i32
    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    // FIX the error in two ways
    /* 1. use a similar type which `impl From<char>`, maybe you
    should check the docs mentioned above to find the answer */
    // 2. a keyword from the last chapter
    let i3: i32 = 'a'.into();

    // FIX the error in two ways
    let s: String = 'a' as String;

    println!("Success!");
}
*/


fn main() {
    // impl From<bool> for i32
    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    // FIX the error in two ways
    /* 1. use a similar type which `impl From<char>`, maybe you
    should check the docs mentioned above to find the answer */
    let _i3: i32 = 'a' as u32 as i32; // Перший спосіб: перетворюємо 'a' на u32, а потім на i32

    // FIX the error in two ways
    let _s: String = 'a'.to_string(); // Другий спосіб: використовуємо метод to_string() для конвертації char в String
    let _s2: String = String::from('a'); // Третій спосіб: використовуємо метод from() для конвертації char в String

    println!("Success!");
}




/*
Для перетворення символу на i32 можна використати u32 як проміжний тип, оскільки char можна перетворити на u32, а потім на i32.

Щоб конвертувати char в String, можна використати методи, які дозволяють створити String з символу.

let i3: i32 = 'a' as u32 as i32;: Це перетворення спочатку конвертує символ 'a' у його числовий код (число 97), а потім у i32.
let s: String = 'a'.to_string();: Цей метод конвертує символ 'a' у String.
let s2: String = String::from('a');: Цей метод також конвертує символ 'a' у String.
*/