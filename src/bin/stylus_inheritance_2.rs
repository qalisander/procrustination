// aka stylus_sdk
mod stylus_lib {
    // Stylus sdk demands this trait to be implemented for
    // the terminal struct of the smart contract.
    pub trait TopLevelStorage: StorageLevel {
        fn get_storage<S: StorageLevel + 'static>(&mut self) -> &mut S {
            unsafe {
                self.try_get_storage().unwrap_or_else(|| {
                    panic!(
                        "storage for type doesn't exist - type name is {}",
                        core::any::type_name::<S>()
                    )
                })
            }
        }
    }

    pub unsafe trait StorageLevel {
        unsafe fn try_get_storage<S: StorageLevel + 'static>(&mut self) -> Option<&mut S> {
            None
        }
    }
}

// aka rust_contracts_stylus
mod oz_lib {
    use std::borrow::BorrowMut;

    use base::Erc721;
    use pausable::Erc721Pausable;

    use crate::stylus_lib::TopLevelStorage;

    pub trait Erc721External: std::fmt::Debug {
        fn transfer(&mut self);
    }

    pub trait Erc721Virtual: std::fmt::Debug {
        type Base: Erc721Virtual;

        fn update(&mut self) {
            println!("updated routed to base");
            Self::Base::update(self);
        }
    }

    pub mod pausable {
        use crate::oz_lib::base::Erc721;
        use crate::stylus_lib::TopLevelStorage;

        use super::Erc721Virtual;

        #[derive(Debug, Default)]
        pub struct Erc721Pausable {
            erc721: Erc721,
        }

        impl Erc721Virtual for Erc721Pausable {
            type Base = Erc721;
            // fn update(&mut self) {
            //     println!("call pausable update");
            //     Self::Base::update(&mut self.erc721);
            // }
        }
    }

    pub mod base {
        use std::marker::PhantomData;

        use crate::stylus_lib::TopLevelStorage;

        use super::{Erc721External, Erc721Virtual};

        #[derive(Debug, Default)]
        pub struct Erc721 {}

        impl Erc721External for Erc721 {
            // Public transfer function of Erc721Base contract.
            fn transfer(&mut self) {
                println!("call base transfer");
                self.update();
            }
        }

        impl Erc721Virtual for Erc721 {
            type Base = Self;
            fn update(&mut self) {
                println!("call base update")
            }
        }
    }
}

// Client code
use std::any::Any;
use std::borrow::{Borrow, BorrowMut};

use crate::oz_lib::pausable::Erc721Pausable;
use crate::oz_lib::Erc721Virtual;
use crate::stylus_lib::{StorageLevel, TopLevelStorage};

impl Erc721Virtual for UserToken {
    type Base = Erc721Pausable;
    // fn update(&mut self) {
    //     println!("call user update");
    //     todo!()
    // }
}

#[derive(Default, Debug)]
struct UserToken {}

impl UserToken {
    fn user_custom_transfer(&mut self) {
        self.update()
    }
}

fn main() {
    let mut token = UserToken::default();
    token.user_custom_transfer();
}
