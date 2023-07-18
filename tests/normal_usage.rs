use soccer::{Display, Into, TryFrom};

/* --- Normal usage with char (non-repr scalar primitive) --- */

#[derive(Clone, Copy, Debug, Display, Eq, Into, PartialEq, TryFrom)]
#[const_ty(char)]
enum Punctuation {
    #[const_val('+')]
    Plus,
    #[const_val('-')]
    Minus,
    #[const_val('*')]
    Star,
    #[const_val('=')]
    Equals,
}

#[test]
fn char_try_from() {
    assert_eq!(Punctuation::try_from('+'), Ok(Punctuation::Plus));
    assert_eq!(Punctuation::try_from('-'), Ok(Punctuation::Minus));
    assert_eq!(Punctuation::try_from('*'), Ok(Punctuation::Star));
    assert_eq!(Punctuation::try_from('='), Ok(Punctuation::Equals));
    assert_eq!(Punctuation::try_from('!'), Err(()));
}

#[test]
fn char_into() {
    assert_eq!(<Punctuation as Into<char>>::into(Punctuation::Plus), '+');
    assert_eq!(<Punctuation as Into<char>>::into(Punctuation::Minus), '-');
    assert_eq!(<Punctuation as Into<char>>::into(Punctuation::Star), '*');
    assert_eq!(<Punctuation as Into<char>>::into(Punctuation::Equals), '=');
}

#[test]
fn char_display() {
    assert_eq!(&format!("{}", Punctuation::Plus), "+");
    assert_eq!(&format!("{}", Punctuation::Minus), "-");
    assert_eq!(&format!("{}", Punctuation::Star), "*");
    assert_eq!(&format!("{}", Punctuation::Equals), "=");
}

/* --- Normal usage with u32 via const_ty (repr primitive without repr) --- */

#[derive(Clone, Copy, Debug, Display, Eq, Into, PartialEq, TryFrom)]
#[const_ty(u32)]
enum StatusCode {
    #[const_val(200)]
    Success,
    #[const_val(400)]
    BadRequest,
    #[const_val(404)]
    NotFound,
    #[const_val(500)]
    ServerError,
}

#[test]
fn u32_const_ty_try_from() {
    assert_eq!(StatusCode::try_from(200), Ok(StatusCode::Success));
    assert_eq!(StatusCode::try_from(400), Ok(StatusCode::BadRequest));
    assert_eq!(StatusCode::try_from(404), Ok(StatusCode::NotFound));
    assert_eq!(StatusCode::try_from(500), Ok(StatusCode::ServerError));
    assert_eq!(StatusCode::try_from(999), Err(()));
}

#[test]
fn u32_const_ty_into() {
    assert_eq!(<StatusCode as Into<u32>>::into(StatusCode::Success), 200);
    assert_eq!(<StatusCode as Into<u32>>::into(StatusCode::BadRequest), 400);
    assert_eq!(<StatusCode as Into<u32>>::into(StatusCode::NotFound), 404);
    assert_eq!(<StatusCode as Into<u32>>::into(StatusCode::ServerError), 500);
}

#[test]
fn u32_const_ty_display() {
    assert_eq!(&format!("{}", StatusCode::Success), "200");
    assert_eq!(&format!("{}", StatusCode::BadRequest), "400");
    assert_eq!(&format!("{}", StatusCode::NotFound), "404");
    assert_eq!(&format!("{}", StatusCode::ServerError), "500");
}

/* --- Normal usage with strings --- */

#[derive(Clone, Copy, Debug, Display, Eq, Into, PartialEq, TryFrom)]
#[const_ty(&'static str)]
enum Keyword {
    #[const_val("if")]
    If,
    #[const_val("else")]
    Else,
    #[const_val("for")]
    For,
    #[const_val("while")]
    While,
}

#[test]
fn str_try_from() {
    assert_eq!(Keyword::try_from("if"), Ok(Keyword::If));
    assert_eq!(Keyword::try_from("else"), Ok(Keyword::Else));
    assert_eq!(Keyword::try_from("for"), Ok(Keyword::For));
    assert_eq!(Keyword::try_from("while"), Ok(Keyword::While));
    assert_eq!(Keyword::try_from("bad"), Err(()));
}

#[test]
fn str_non_static_try_from() {
    let if_kw = "if".to_string();
    assert_eq!(Keyword::try_from(&if_kw[..]), Ok(Keyword::If));
}

#[test]
fn str_into() {
    assert_eq!(<Keyword as Into<&'static str>>::into(Keyword::If), "if");
    assert_eq!(<Keyword as Into<&'static str>>::into(Keyword::Else), "else");
    assert_eq!(<Keyword as Into<&'static str>>::into(Keyword::For), "for");
    assert_eq!(<Keyword as Into<&'static str>>::into(Keyword::While), "while");
}

#[test]
fn str_display() {
    assert_eq!(&format!("{}", Keyword::If), "if");
    assert_eq!(&format!("{}", Keyword::Else), "else");
    assert_eq!(&format!("{}", Keyword::For), "for");
    assert_eq!(&format!("{}", Keyword::While), "while");
}

/* --- Normal usage with arrays --- */

#[derive(Clone, Copy, Debug, Eq, Into, PartialEq, TryFrom)]
#[const_ty([u8; 3])]
enum Color {
    #[const_val([0; 3])]
    Black,

    #[const_val([150, 0, 0])]
    Red,

    #[const_val([0, 150, 0])]
    Green,

    #[const_val([0, 0, 150])]
    Blue,

    #[const_val([255; 3])]
    White,
}

#[test]
fn array_try_from() {
    assert_eq!(Color::try_from([0, 0, 0]), Ok(Color::Black));
    assert_eq!(Color::try_from([150, 0, 0]), Ok(Color::Red));
    assert_eq!(Color::try_from([0, 150, 0]), Ok(Color::Green));
    assert_eq!(Color::try_from([0, 0, 150]), Ok(Color::Blue));
    assert_eq!(Color::try_from([255; 3]), Ok(Color::White));
    assert_eq!(Color::try_from([200, 140, 120]), Err(()));
}

#[test]
fn array_into() {
    assert_eq!(<Color as Into<[u8; 3]>>::into(Color::Black), [0; 3]);
    assert_eq!(<Color as Into<[u8; 3]>>::into(Color::Red), [150, 0, 0]);
    assert_eq!(<Color as Into<[u8; 3]>>::into(Color::Green), [0, 150, 0]);
    assert_eq!(<Color as Into<[u8; 3]>>::into(Color::Blue), [0, 0, 150]);
    assert_eq!(<Color as Into<[u8; 3]>>::into(Color::White), [255; 3]);
}

/* --- Normal usage with tuples --- */

#[derive(Clone, Copy, Debug, Eq, Into, PartialEq, TryFrom)]
#[const_ty((i32, i32))]
enum Point {
    #[const_val((0, 0))]
    Origin,

    #[const_val((1, 1))]
    Quad1,

    #[const_val((-1, 1))]
    Quad2,

    #[const_val((-1, -1))]
    Quad3,

    #[const_val((1, -1))]
    Quad4,
}

#[test]
fn tuple_try_from() {
    assert_eq!(Point::try_from((0, 0)), Ok(Point::Origin));
    assert_eq!(Point::try_from((1, 1)), Ok(Point::Quad1));
    assert_eq!(Point::try_from((-1, 1)), Ok(Point::Quad2));
    assert_eq!(Point::try_from((-1, -1)), Ok(Point::Quad3));
    assert_eq!(Point::try_from((1, -1)), Ok(Point::Quad4));
    assert_eq!(Point::try_from((7, 4)), Err(()));
}

#[test]
fn tuple_into() {
    assert_eq!(<Point as Into<(i32, i32)>>::into(Point::Origin), (0, 0));
    assert_eq!(<Point as Into<(i32, i32)>>::into(Point::Quad1), (1, 1));
    assert_eq!(<Point as Into<(i32, i32)>>::into(Point::Quad2), (-1, 1)); 
    assert_eq!(<Point as Into<(i32, i32)>>::into(Point::Quad3), (-1, -1));
    assert_eq!(<Point as Into<(i32, i32)>>::into(Point::Quad4), (1, -1));
}

/* --- Normal usage with slices --- */

/* --- Normal usage with copyable custom struct --- */

/* --- Normal usage with non-copy custom struct --- */

/* --- Normal usage with repr --- */

// steal discrim tests for this
