#[test]

/*
// TryFrom and TryInto are included in `std::prelude`, so there is no need to introduce it into the current scope
// use std::convert::TryInto;

fn main() {
    let n: i16 = 256;

    // Into trait has a method `into`,
    // hence TryInto has a method ?
    let n: u8 = match n.__() {
        Ok(n) => n,
        Err(e) => {
            println!("there is an error when converting: {:?}, but we catch it", e.to_string());
            0
        }
    };

    assert_eq!(n, __);

    println!("Success!");
}
*/

// TryFrom and TryInto are included in `std::prelude`, so there is no need to introduce it into the current scope
// use std::convert::TryInto;

fn main() {
    let n: i16 = 256;

    // Into trait has a method `into`,
    // hence TryInto has a method `try_into`
    let n: u8 = match n.try_into() {
        Ok(n) => n,
        Err(e) => {
            println!("there is an error when converting: {:?}, but we catch it", e.to_string());
            0
        }
    };

    assert_eq!(n, 0); // Оскільки 256 не може бути переведено в u8, результатом буде 0

    println!("Success!");
}

/*
Метод try_into(): Замість __() використовується try_into(), який намагається безпечним чином конвертувати значення з одного типу в інший.
assert_eq!(n, 0): Заповнення пропуску для перевірки, що результат конвертації буде 0 у випадку,
якщо виникає помилка (оскільки 256 не може бути представлене в u8).
*/