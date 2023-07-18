# soccer
Associated constants for Rust enums

**NOTE: Tests and documentation are very much still a work in progress for the time being**

## A Basic Example

`soccer` can generate code to make treating a fieldless enum as a discrete set of constants easier by deriving `TryFrom`, `Into`, and `Display`:

```rust
#[derive(Clone, Copy, PartialEq, Eq, TryFrom, Into, Display)]
#[const_ty(char)]
enum Punctuation {
    Plus,
    Minus,
    Star,
    Equals,
}

fn main() {
    assert_eq!(Punctuation::try_from('+'), Ok(Punctuation::Plus));

    let val: char = Punctuation::Star.into();
    assert_eq!(val, '*');

    println!("{}", Punctuation::Minus); // prints "-"
}
```

It can also use the discriminant to do this:

```rust
#[derive(Clone, Copy, PartialEq, Eq, TryFrom, Into, Display)]
#[repr(u8)]
enum Opcode {
    Add,
    Sub,
    Mul,
    Load,
    Store,
}

fn main() {
    assert_eq!(Opcode::try_from(0), Ok(Opcode::Add));

    let val: u8 = Opcode::Load.into();
    assert_eq!(val, 3);

    println!("{}", Opcode::Store); // prints "4"
}
```

Those are both actual use cases I've encountered in my own Rust projects that motivated the creation of this crate.

(I used to maintain the [https://github.com/a-lafrance/discrim](`discrim`) crate for this discriminant-based conversion codegen, but since it got absorbed into `soccer` I've archived it and don't work on it anymore)
