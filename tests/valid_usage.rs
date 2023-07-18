use consty::{Display, Into, TryFrom};

#[derive(Clone, Copy, Debug, Display, Eq, Into, PartialEq, TryFrom)]
#[const_ty(char)]
enum Punctuation {
    #[const_val('+')] Plus,
    #[const_val('-')] Minus,
    #[const_val('*')] Star,
    #[const_val('=')] Equals,
}

#[test]
fn try_from_sanity_check() {
    assert_eq!(Punctuation::try_from('+'), Ok(Punctuation::Plus));
    assert_eq!(Punctuation::try_from('-'), Ok(Punctuation::Minus));
    assert_eq!(Punctuation::try_from('*'), Ok(Punctuation::Star));
    assert_eq!(Punctuation::try_from('='), Ok(Punctuation::Equals));
    assert_eq!(Punctuation::try_from('!'), Err('!'));
}

#[test]
fn into_sanity_check() {
    assert_eq!(<Punctuation as Into<char>>::into(Punctuation::Plus), '+');
    assert_eq!(<Punctuation as Into<char>>::into(Punctuation::Minus), '-');
    assert_eq!(<Punctuation as Into<char>>::into(Punctuation::Star), '*');
    assert_eq!(<Punctuation as Into<char>>::into(Punctuation::Equals), '=');
}

#[test]
fn display_sanity_check() {
    assert_eq!(&format!("{}", Punctuation::Plus), "+");
    assert_eq!(&format!("{}", Punctuation::Minus), "-");
    assert_eq!(&format!("{}", Punctuation::Star), "*");
    assert_eq!(&format!("{}", Punctuation::Equals), "=");
}
