#![cfg(test)]

use crate as moddef;

moddef::moddef!(
    flat mod {
        pub core {
            pub(super) const TEST3: u8 = crate::TEST;
        }
    }
);

#[test]
fn it_works()
{
    use crate::*;

    use self::core as conflicting_name;

    assert_eq!(self::core::TEST3, crate::TEST)
}