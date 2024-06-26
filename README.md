
# nscfg

Nifty Simple CFG provides a revamped syntax and macros to easily manage all #[cfg] parameters in one package. See [features](https://github.com/NickelAngeStudio/nscfg/wiki/Features) to get the full list of features like aliases, attributes, automatic dependency tag documentation and more.

## Example
**Transform this :**
```
#[cfg(any(doc, any(target_os = "linux", target_os = "macos", target_os = "windows")))]
#[cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
pub mod desktop_mod;

#[cfg(any(doc, any(target_os = "linux", target_os = "macos", target_os = "windows")))]
#[cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))))]
pub use desktop_mod::Struct as Struct;

#[cfg(any(doc, any(target_os = "ios", target_os = "android")))]
#[cfg_attr(docsrs, doc(cfg(any(target_os = "ios", target_os = "android"))))]
pub mod mobile_mod;

#[cfg(any(doc, any(target_os = "ios", target_os = "android")))]
#[cfg_attr(docsrs, doc(cfg(any(target_os = "ios", target_os = "android"))))]
pub use mobile_mod::Struct1 as Struct1;

#[cfg(any(doc, any(target_os = "ios", target_os = "android")))]
#[cfg_attr(docsrs, doc(cfg(any(target_os = "ios", target_os = "android"))))]
pub use mobile_mod::Struct2 as Struct2;

#[cfg(any(doc, any(target_os = "ios", target_os = "android")))]
#[cfg_attr(docsrs, doc(cfg(any(target_os = "ios", target_os = "android"))))]
pub fn mobile_only_fn() {}
```

**Into this :**
```
target_cfg!{
    desktop => {
        pub mod desktop_mod;
        pub use desktop_mod::Struct as Struct;
    },
    mobile => {
        pub mod mobile_mod;
        pub use mobile_mod::Struct1 as Struct1;
        pub use mobile_mod::Struct2 as Struct2;
        pub fn mobile_only_fn() {}
    }
}
```

See [examples](https://github.com/NickelAngeStudio/nscfg/wiki/Examples) for more use cases.


## Installation
Execute this command in your Rust project folder.
```
cargo add nscfg
```

## Dependencies
nscfg has no dependencies and only use stable rust library.

## Question?
See [nscfg wiki](https://github.com/NickelAngeStudio/nscfg/wiki), it contains a **LOT** of information.

