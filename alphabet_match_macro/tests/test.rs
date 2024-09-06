use alphabet_match_macro::*;

#[derive(Debug, PartialEq)]
enum SomeEnum {
    A,
    B,
}

#[test]
fn test() {
    let ch = 'a';

    let res: &[SomeEnum] =
        alphabet_match!([(SomeEnum::A, ['a', 'b', 'c']), (SomeEnum::B, ['a', 'c']),]);

    assert_eq!(res, &[SomeEnum::A, SomeEnum::B]);

    let ch = 'd';

    let res: &[SomeEnum] =
        alphabet_match!([(SomeEnum::A, ['a', 'b', 'c']), (SomeEnum::B, ['a', 'c']),]);

    assert_eq!(res, &[SomeEnum::A, SomeEnum::B]);
}
