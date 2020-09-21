//! Rust macro to automatically implement the **dependency injection** pattern for arbitrary structs.
//! A simple `#[derive(Container)]` will generate new getters and setters for every field of your struct.
//! Also, the Container will implement the `Default` trait, where will inject every field with `Injectable` trait.
//! This makes the possible to mock any `dyn Trait` field in your container.
//!
//! # Quick start
//! Just add the `derive_di` to you Cargo.toml and import the deps.
//!
//! ## Simple example
//! ```rust
//! use derive_di::*;
//! #[injectable]
//! #[derive(Default)]
//! struct InjectableStruct;
//!
//! #[derive(Container)]
//! struct MyContainer {
//!     i_struct: InjectableStruct,
//! }
//! ```
//!
//! That code, which will be generated for you
//!
//! ```rust
//! use derive_di::*;
//!
//! #[derive(Default)]
//! struct InjectableStruct;
//! impl Injectable for InjectableStruct {
//!     fn get_service() -> Self {
//!         Default::default()
//!     }
//! }
//!
//! struct MyContainer {
//!     i_struct: InjectableStruct,
//! }
//!
//! impl MyContainer {
//!     pub fn get_i_struct(&self) -> &InjectableStruct {
//!         &self.i_struct
//!     }
//!     pub fn get_mut_i_struct(&mut self) -> &mut InjectableStruct {
//!         &mut self.i_struct
//!     }
//!     pub fn set_i_struct(&mut self, i_struct: InjectableStruct) {
//!         self.i_struct = i_struct
//!     }
//! }
//!
//! impl Default for MyContainer {
//!     fn default() -> Self {
//!         Self {
//!             i_struct: Injectable::get_service()
//!         }
//!     }
//! }
//! ```
//!
//! ## Additional features
//!
//! ### Factory
//! You can pass any factory to the `injectable` macro for building you struct.
//!
//! #### Factory struct
//! You can build you struct inside `injectable` macro.
//! ```rust
//! use derive_di::*;
//! #[injectable(factory => InjectableStruct {inner: "test".to_owned()})]
//! struct InjectableStruct {
//!     inner: String,
//! }
//! ```
//! The `Injectable` will be look like this
//! ```rust
//! use derive_di::*;
//! struct InjectableStruct {
//!     inner: String,
//! }
//!
//! impl Injectable for InjectableStruct {
//!     fn get_service() -> Self {
//!         InjectableStruct {inner: "test".to_owned()}
//!     }
//! }
//! ```
//!
//! #### Factory fn
//! You can build you struct inside `injectable` macro with factory method.
//! ```rust
//! use derive_di::*;
//! fn factory_struct() -> InjectableStruct {
//!     InjectableStruct {
//!         inner: "test".to_owned(),
//!     }
//! }
//! #[injectable(factory => factory_struct())]
//! struct InjectableStruct {
//!     inner: String,
//! }
//! ```
//! The `Injectable` will be look like this
//! ```rust
//! use derive_di::*;
//!
//! fn factory_struct() -> InjectableStruct {
//!     InjectableStruct {
//!         inner: "test".to_owned(),
//!     }
//! }
//! struct InjectableStruct {
//!     inner: String,
//! }
//! impl Injectable for InjectableStruct {
//!     fn get_service() -> Self {
//!         factory_struct()
//!     }
//! }
//! ```
//! #### Factory closure
//! You can build you struct inside `injectable` macro with factory closure.
//! ```rust
//! use derive_di::*;
//! #[injectable(factory => || InjectableStruct {inner: "test".to_owned()})]
//! struct InjectableStruct {
//!     inner: String,
//! }
//! ```
//! The `Injectable` will be look like this
//! ```rust
//! use derive_di::*;
//! struct InjectableStruct {
//!     inner: String,
//! }
//! impl Injectable for InjectableStruct {
//!     fn get_service() -> Self {
//!         (|| InjectableStruct {inner: "test".to_owned()})()
//!     }
//! }
//! ```
//!
//! ### Auto injecting a structs to the `dyn Trait` container fields
//! With the `inject` macro, you can easy to solve `dyn Trait` fields in tou container.
//! ```rust
//! use derive_di::*;
//!
//! #[injectable(factory => InjectableStruct)]
//! struct InjectableStruct;
//!
//! trait Getter {
//!     fn get(&self) -> String;
//! }
//!
//! impl Getter for InjectableStruct {
//!     fn get(&self) -> String {
//!         "test".to_owned()
//!     }
//! }
//!
//! #[derive(Container)]
//! struct MyContainer {
//!     #[inject(InjectableStruct)]
//!     i_struct: Box<dyn Getter>,
//! }
//! ```
//! The `Default` impl of the`MyContainer` will be looks like
//!
//! ```rust
//! use derive_di::*;
//!
//! #[injectable(factory => InjectableStruct)]
//! struct InjectableStruct;
//!
//! trait Getter {
//!     fn get(&self) -> String;
//! }
//!
//! impl Getter for InjectableStruct {
//!     fn get(&self) -> String {
//!         "test".to_owned()
//!     }
//! }
//!
//! struct MyContainer {
//!     i_struct: Box<dyn Getter>,
//! }
//! impl Default for MyContainer {
//!     fn default() -> Self {
//!         Self {
//!             i_struct: Box::from(InjectableStruct::get_service())
//!         }
//!     }
//! }
//! ```
//!
//! ### Mocks
//! You can combine the `dyn Trait` fields and setters in your container
//! and mock any logic for simple testing.
//!
//! ```rust
//! use derive_di::*;
//!
//! #[injectable(factory => || InjectableStruct)]
//! struct InjectableStruct;
//!
//! trait Getter {
//!     fn get(&self) -> String;
//! }
//!
//! impl Getter for InjectableStruct {
//!     fn get(&self) -> String {
//!         "test".to_owned()
//!     }
//! }
//!
//! struct GetterMock;
//! impl Getter for GetterMock {
//!     fn get(&self) -> String {
//!         "mocked".to_owned()
//!     }
//! }
//!
//! #[derive(Container)]
//! struct MyContainer {
//!     #[inject(InjectableStruct)]
//!     i_struct: Box<dyn Getter>,
//! }
//!
//! fn main() {      
//!     let mut container = MyContainer::default();
//!     assert_eq!("test", container.get_i_struct().get());
//!     container.set_i_struct(Box::from(GetterMock));
//!     assert_eq!("mocked", container.get_i_struct().get())
//! }
//! ```

extern crate derive_di_macro;

pub use derive_di_core::injectable::Injectable;
pub use derive_di_macro::{injectable, Container};

#[cfg(test)]
mod tests {
    use crate::{injectable, Container, Injectable};

    #[test]
    fn injectable_default_test() {
        #[injectable]
        #[derive(Default)]
        struct InjectableStruct;

        impl InjectableStruct {
            fn get(&self) -> String {
                "test".to_owned()
            }
        }

        #[derive(Container)]
        struct MyContainer {
            i_struct: InjectableStruct,
        }

        assert_eq!("test", MyContainer::default().get_i_struct().get())
    }

    #[test]
    fn injectable_test() {
        #[injectable(factory => InjectableStruct {inner: "test".to_owned()})]
        struct InjectableStruct {
            inner: String,
        }

        impl InjectableStruct {
            fn get(&self) -> String {
                self.inner.clone()
            }
        }

        #[derive(Container)]
        struct MyContainer {
            i_struct: InjectableStruct,
        }

        assert_eq!("test", MyContainer::default().get_i_struct().get())
    }

    #[test]
    fn injectable_test_fn_factory() {
        fn factory_struct() -> InjectableStruct {
            InjectableStruct {
                inner: "test".to_owned(),
            }
        }
        #[injectable(factory => factory_struct())]
        struct InjectableStruct {
            inner: String,
        }

        impl InjectableStruct {
            fn get(&self) -> String {
                self.inner.clone()
            }
        }

        #[derive(Container)]
        struct MyContainer {
            i_struct: InjectableStruct,
        }

        assert_eq!("test", MyContainer::default().get_i_struct().get())
    }

    #[test]
    fn injectable_test_closure() {
        #[injectable(factory => || InjectableStruct)]
        struct InjectableStruct;

        impl InjectableStruct {
            fn get(&self) -> String {
                "test".to_owned()
            }
        }

        #[derive(Container)]
        struct MyContainer {
            i_struct: InjectableStruct,
        }

        assert_eq!("test", MyContainer::default().get_i_struct().get())
    }

    #[test]
    fn injectable_test_box() {
        #[injectable(factory => || InjectableStruct)]
        struct InjectableStruct;

        trait Getter {
            fn get(&self) -> String;
        }

        impl Getter for InjectableStruct {
            fn get(&self) -> String {
                "test".to_owned()
            }
        }

        #[derive(Container)]
        struct MyContainer {
            #[inject(InjectableStruct)]
            i_struct: Box<dyn Getter>,
        }

        assert_eq!("test", MyContainer::default().get_i_struct().get())
    }

    #[test]
    fn injectable_test_mock() {
        #[injectable(factory => || InjectableStruct)]
        struct InjectableStruct;

        trait Getter {
            fn get(&self) -> String;
        }

        impl Getter for InjectableStruct {
            fn get(&self) -> String {
                "test".to_owned()
            }
        }

        struct GetterMock;
        impl Getter for GetterMock {
            fn get(&self) -> String {
                "mocked".to_owned()
            }
        }

        #[derive(Container)]
        struct MyContainer {
            #[inject(InjectableStruct)]
            i_struct: Box<dyn Getter>,
        }

        let mut container = MyContainer::default();

        container.set_i_struct(Box::from(GetterMock));

        assert_eq!("mocked", container.get_i_struct().get())
    }

    #[test]
    fn injectable_test_inject_self() {
        #[injectable(factory => InjectableStruct)]
        struct InjectableStruct;

        trait Getter {
            fn get(&self) -> String;
        }

        impl Getter for InjectableStruct {
            fn get(&self) -> String {
                "test".to_owned()
            }
        }

        #[derive(Container)]
        struct MyContainer {
            #[inject(InjectableStruct)]
            i_struct: InjectableStruct,
        }

        let container = MyContainer::default();

        assert_eq!("test", container.get_i_struct().get())
    }
}
