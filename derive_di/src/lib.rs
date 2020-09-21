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
        #[injectable(factory => InjectableStruct)]
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
