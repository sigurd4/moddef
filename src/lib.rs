#![no_std]

#[macro_export]
macro_rules! flat_use {
    ($($mod:ident $(for $($attribute:meta)+)?),* $(,)?) => {};
    (($vis_flat:vis) $($mod:ident $(for $($attribute:meta)+)?),* $(,)?) => {
        $(
            $(
                $(
                    #[$attribute]
                )*
            )?
            #[allow(unused)]
            $vis_flat use $mod::*;
        )*
    };
    (() $($mod:ident $(for $($attribute:meta)+)?),* $(,)?) => {
        $(
            #[allow(unused)]
            $(
                $(
                    #[$attribute]
                )*
            )?
            use $mod::*;
        )*
    };
}

#[macro_export]
macro_rules! moddef {
    ($vis:vis $(flat $(($vis_flat:vis))?)? mod $mod:ident $(for $($attribute:meta)+)? $(,$($more:tt)+)? $(,)?) => {
        flat_use!{$(($($vis_flat)?))? $mod $(for $($attribute)*)?}
        $(
            $(
                #[$attribute]
            )*
        )?
        $vis mod $mod;
        $(moddef!($($more)*);)?
    };
    ($vis:vis $(flat $(($vis_flat:vis))?)? mod {$($mods:ident $(for $($attribute:meta)+)?),*} $(,$($more:tt)+)? $(,)?) => {
        flat_use!{$(($($vis_flat)?))? $($mods $(for $($attribute)*)?),*}
        moddef!(
            $(
                $vis mod $mods $(for $($attribute)*)?,
            )*
            $($($more)*)?
        );
    };
    ($(flat $(($vis_flat:vis))?)? mod {$($vis:vis $mods:ident $(for $($attribute:meta)+)?),*} $(,$($more:tt)+)? $(,)?) => {
        moddef!(
            $(
                $vis mod $mods $(for $($attribute)*)?,
            )*
            $($($more)*)?
        );
        flat_use!($(($($vis_flat)?))? $($mods $(for $($attribute)*)?),*);
    };

    ($vis:vis mod {$(flat $(($vis_flat:vis))? $mods:ident $(for $($attribute:meta)+)?),*} $(,$($more:tt)+)? $(,)?) => {
        moddef!(
            $(
                $vis flat $(($vis_flat))? mod $mods $(for $($attribute)*)?,
            )*
            $($($more)*)?
        );
    };
    (mod {$($vis:vis flat $(($vis_flat:vis))? $mods:ident $(for $($attribute:meta)+)?),*} $(,$($more:tt)+)? $(,)?) => {
        moddef!(
            $(
                $vis flat $(($vis_flat))? mod $mods $(for $($attribute)*)?,
            )*
            $($($more)*)?
        );
    };
}

moddef!(
    pub mod {
        flat test for cfg(test)
    },
    flat mod {
        pub(crate) test2 for cfg(test)
    }
);

#[allow(unused)]
#[cfg(test)]
const T: u8 = TEST;