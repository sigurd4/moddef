#![no_std]

#[macro_export]
macro_rules! mods {
    ($($mods:tt)*) => {
        $(
            mod $mods;
        )*
    }
}
#[macro_export]
macro_rules! pub_mods {
    ($($mods:tt)*) => {
        $(
            pub mod $mods;
        )*
    }
}
#[macro_export]
macro_rules! flat_mods {
    ($($mods:tt)*) => {
        $(
            mod $mods;
            use $mods::*;
        )*
    }
}
#[macro_export]
macro_rules! flat_pub_mods {
    ($($mods:tt)*) => {
        $(
            pub mod $mods;
            use $mods::*;
        )*
    }
}
#[macro_export]
macro_rules! pub_flat_mods {
    ($($mods:tt)*) => {
        $(
            mod $mods;
            pub use $mods::*;
        )*
    }
}