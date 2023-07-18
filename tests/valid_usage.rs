use consty::{Display, Into, TryFrom};

// lots more test surface area to cover for this than discrim:
    // normal sanity check for all 3 macros: done
    // normal sanity check strings
    // normal sanity check numeric primitive
    // normal sanity check struct
    // normal sanity check array
    // normal sanity check tuple
    // repr sanity check (steal discrim tests)
    // const_ty + repr both
    // fieldless but with tuple/struct variants
    // all kinds of error cases:
        // not fieldless
        // neither const_ty nor repr present
        // bounds not met
            // TryFrom and Into:
                // no PartialEq
                // no Eq
                // neither
            // Display:
                // const type no Display
                // no Copy
                // no Into
        // type isn't fully concrete
            // type generics
            // lifetime generics
            // const generics
        // generics in enum
        // not an enum
        // empty enum
        // un-parseable const_ty or repr
        // non-constant const_val
        // un-parseable const_val
        // missing const_val
    // manually implemented Into + derived Display
// housekeeping:
    // documentation
        // must document all requirements for proper use of library
    // rather than keeping everything in one file maybe move them around to separate files
    // publish on crates.io + make repo public
    // archive discrim repo, push discrim update mentioning that it's inactive and merged into this project
// then, use in Aqua

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
