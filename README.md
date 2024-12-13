# moddef
Organize your module declaration with this "simple" macro.

## Why?

I just hated writing the same stuff over and over again.

## Example

With `moddef`, you can write your module declarations like this:

```rust
moddef::moddef!(
    flat(pub) mod {
        maybe_rtf_or_system,
        maybe_system,
        rtf_or_system,
        system,
        validate_filter_bands
    },
    pub mod {
        analysis,
        decompositions,
        gen,
        identification,
        operations,
        quantities,
        systems,
        transforms,
        util,
        windows
    },
    mod {
        plot for cfg(test)
    }
);
```

Instead of this:

```rust
mod maybe_rtf_or_system; pub use maybe_rtf_or_system::*;
mod maybe_system; pub use maybe_system::*;
mod rtf_or_system; pub use rtf_or_system::*;
mod system; pub use system::*;
mod validate_filter_bands; pub use validate_filter_bands::*;

pub mod analysis;
pub mod decompositions;
pub mod gen;
pub mod identification;
pub mod operations;
pub mod quantities;
pub mod systems;
pub mod transforms;
pub mod util;
pub mod windows

#[cfg(test)]
mod plot;
```

The two are equivalent, but, since i prefer the first one, i wrote a macro to do it easily, which i use in every project of mine. It's really just personal preference.

I find it makes it a lot easier when i have a lot of modules with similar properties, and especially when i want to rename a module, since its name only has to be written once when i re-export (which i do often).

## Structure

The schema used for the macro is like this:

```txt
$MODULE_VISIBLITY mod {
    $MODULE_NAME,
    ...
    // or:
    $MODULE_NAME for $MODULE_ATTRIBUTES, // the trailing comma is optional
    ...
},
...
// Alternatively, for just a single module:
$MODULE_VISIBLITY mod $MODULE_NAME,
...
// or
$MODULE_VISIBILITY mod $MODULE_NAME for $MODULE_ATTRIBUTES, // The trailing comma here is also optional
```

Trailing commas are optional, but comma-seperators are not.

## Module visibility

Before the `mod` token, a descriptor can be chosen to set the visibility of the module, and wether or not the module should be "flat".

Flatness of a module is something i just made up, but it means that the inner members of the module will be re-exported, and from the outside, seem as if they belong to the parent module. I just do this a lot, so i gave it a name and a quick shortcut within the macro.

Here are some examples of valid module visibility descriptors:

- Private
    - Equivalent to `mod mymodule;`
- `pub` Public
    - Equivalent to `pub mod mymodule;`
- `pub(crate)` Crate-wide
    - Equivalent to `pub(crate) mod mymodule;`
- `pub(self)` Local
    - Equivalent to `pub(self) mod mymodule;`
- `pub(super)` One-layer up in module tree
    - Equivalent to `pub(super) mod mymodule;`
- `flat` Private with members privately re-exported
    - Equivalent to `mod mymodule; use mymodule::*;`
- `flat(pub)` Private with members publicly re-exported
    - Equivalent to `mod mymodule; pub use mymodule::*;`
- `flat(pub(crate))` Private with members re-exported crate-wide
    - Equivalent to `mod mymodule; pub(crate) use mymodule::*;`
- `pub flat(pub(crate))` Public with members re-exported crate-wide
    - Equivalent to `pub mod mymodule; pub(crate) use mymodule::*;`

In short: if you don't use the `flat` descriptor, any valid visiblity descriptor in the Rust language should work fine. If you do use `flat`, a visibility to the re-export can be given in the parenthesis after.

## Module attributes

This is just a whitespace-separated list of attributes that will be appended within a `#[...]` before the module when the macro expands. I often use `cfg(test)` to tell the compiler to ignore the module when not compiling tests.

If attributes are to be given, a single `for` token must be applied after the module name.

### Example

```rust
moddef::moddef!(
    flat mod {
        impl_macos for cfg(target_os = "macos"),
        impl_linux for cfg(target_os = "linux") cfg(feature = "std")),
        impl_windows for cfg(target_os = "windows"),
        impl_linux_nostd for cfg(target_os = "linux") cfg(not(feature = "std")))
    }
);
```

## Note

If the macro fails to do anything given in the documentation it is a bug. It can maybe do other things, like assign visibility to each module in a module-group seperately (at least it can right now, as of writing this), but those are really just implementation details that i needed to make the macro work. They're not specified features.

I hope this helps. I've used this macro for a long time, but i think other people should be able to try it. There's not much point in publishing code that only you yourself know how to use. Enjoy.
