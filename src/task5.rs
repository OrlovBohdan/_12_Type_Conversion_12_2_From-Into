#[test]

/*
#[derive(Debug, PartialEq)]
struct EvenNum(i32);

impl TryFrom<i32> for EvenNum {
    type Error = ();

    // IMPLEMENT `try_from`
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNum(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    assert_eq!(EvenNum::try_from(5), Err(()));

    // FILL in the blanks
    let result: Result<EvenNum, ()> = 8i32.try_into();
    assert_eq!(result, __);
    let result: Result<EvenNum, ()> = 5i32.try_into();
    assert_eq!(result, __);

    println!("Success!");
}
*/
fn main() {
    assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    assert_eq!(EvenNum::try_from(5), Err(()));

    // FILL in the blanks
    let result: Result<EvenNum, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNum(8))); // Заповнено

    let result: Result<EvenNum, ()> = 5i32.try_into();
    assert_eq!(result, Err(())); // Заповнено

    println!("Success!");
}


#[derive(Debug, PartialEq)]
struct EvenNum(i32);

impl TryFrom<i32> for EvenNum {
    type Error = ();

    // IMPLEMENT `try_from`
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNum(value))
        } else {
            Err(())
        }
    }
}


/*
Перше заповнення: assert_eq!(result, Ok(EvenNum(8))); — оскільки 8 парне, try_into() успішно поверне Ok(EvenNum(8)).

Друге заповнення: assert_eq!(result, Err(())); — оскільки 5 непарне, try_into() поверне Err(()).
*/