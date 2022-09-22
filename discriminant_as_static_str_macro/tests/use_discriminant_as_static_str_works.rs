extern crate discriminant_as_static_str_macro;
use discriminant_as_static_str_macro::use_discriminant_as_static_str;
use strum_macros::IntoStaticStr;

#[use_discriminant_as_static_str]
#[derive(PartialEq, IntoStaticStr)]
#[repr(u8)]
enum Digit {
    One = b'1',
    Two = b'2',
    #[strum(serialize = "THREE")]
    Three,
    Four = b'4',
    Five,
}

#[test]
fn the_macro_works() {
    let one: &'static str = Digit::One.into();
    let two: &'static str = Digit::Two.into();
    let three: &'static str = Digit::Three.into();
    let four: &'static str = Digit::Four.into();
    let five: &'static str = Digit::Five.into();

    assert_eq!(one, "1");
    assert_eq!(two, "2");
    assert_eq!(three, "THREE");
    assert_eq!(four, "4");
    assert_eq!(five, "5");
}
