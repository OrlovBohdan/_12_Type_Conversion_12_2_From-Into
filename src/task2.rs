#[test]

/*
// From is now included in `std::prelude`, so there is no need to introduce it into the current scope
// use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    // IMPLEMENT `from` method
}

// FILL in the blanks
fn main() {
    let num = __(30);
    assert_eq!(num.value, 30);

    let num: Number = __;
    assert_eq!(num.value, 30);

    println!("Success!");
}
*/

// Заповнюємо пропуски
fn main() {
    let num = Number::from(30); // Створюємо екземпляр Number через метод from
    assert_eq!(num.value, 30);

    let num: Number = 30.into(); // Використовуємо метод into для створення Number
    assert_eq!(num.value, 30);

    println!("Success!");
}
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    // Реалізуємо метод from
    fn from(value: i32) -> Self {
        Number { value }
    }
}




/*
Реалізація методу from: Метод from приймає значення типу i32 і повертає новий екземпляр Number з цим значенням.
Number::from(30): Створює екземпляр Number, використовуючи реалізований метод from.
30.into(): Використовує метод into, щоб конвертувати значення i32 у Number.
*/