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

    use base::Erc721Base;
    use pausable::Erc721Pausable;

    use crate::stylus_lib::TopLevelStorage;

    // Trait used for overriding behaviour of update function.
    // Other functions can be added to this update function.
    // Or associated type for every "virtual" method can be used,
    // that will be restricted with trait named like (Erc721UpdateVirtual).
    pub trait Erc721Virtual: 'static + std::fmt::Debug {
        type Base: Erc721Virtual;
        fn update<V: Erc721Virtual>(storage: &mut impl TopLevelStorage) {
            Self::Base::update::<V>(storage);
        }
    }

    // Library contract that will be reused by our consumers
    #[derive(Debug, Default)]
    pub struct Erc721<T: Erc721Virtual> {
        pub base: Erc721Base<T>,
        pub pausable: Erc721Pausable<T>,
    }

    pub mod pausable {
        use std::marker::PhantomData;

        use crate::stylus_lib::TopLevelStorage;

        use super::Erc721Virtual;

        #[derive(Debug, Default)]
        pub struct Erc721Pausable<V: Erc721Virtual> {
            // Other fields for pausable omitted for simplicity
            phantom_data: PhantomData<V>,
        }

        // Overriding update function for pausalbe extension.
        // Here we can access Erc721<_> parent storage.
        // Basically Erc721Pausable and Erc721Base can be mutated.
        #[derive(Debug, Default)]
        pub struct Erc721PausableOverride<B: Erc721Virtual>(B);
        impl<Base: Erc721Virtual> Erc721Virtual for Erc721PausableOverride<Base> {
            type Base = Base;
            fn update<V>(storage: &mut impl TopLevelStorage)
            where
                V: Erc721Virtual,
            {
                println!("call pausable update");
                Base::update::<V>(storage);
            }
        }
    }

    pub mod base {
        use std::marker::PhantomData;

        use crate::stylus_lib::TopLevelStorage;

        use super::Erc721Virtual;

        #[derive(Debug, Default)]
        pub struct Erc721Base<V: Erc721Virtual> {
            // Other fields for erc721 missed for simplicity
            phantom_data: PhantomData<V>,
        }

        // Simplicity sake, we omit #[external] attribute and stylus sdk dependency.
        impl<V: Erc721Virtual> Erc721Base<V> {
            // Public transfer function of Erc721Base contract.
            pub fn transfer(storage: &mut impl TopLevelStorage) {
                println!("call base transfer");
                V::update::<V>(storage);
            }
        }

        // Base implementation of update function.
        #[derive(Debug, Default)]
        pub struct Erc721BaseOverride;
        impl Erc721Virtual for Erc721BaseOverride {
            type Base = Self;
            fn update<V>(storage: &mut impl TopLevelStorage)
            where
                V: Erc721Virtual,
            {
                println!("call base update")
            }
        }
    }
}

// Client code
use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};

use crate::oz_lib::base::Erc721Base;
use crate::oz_lib::{Erc721, Erc721Virtual};
use crate::stylus_lib::{StorageLevel, TopLevelStorage};

type Override =
    Erc721UserOverride<oz_lib::pausable::Erc721PausableOverride<oz_lib::base::Erc721BaseOverride>>;
// User can override and access storage of his own contract (UserToken)
// because of constraint of Erc721Virtual trait.
#[derive(Debug, Default)]
pub struct Erc721UserOverride<V: Erc721Virtual>(V);
impl<Base: Erc721Virtual> Erc721Virtual for Erc721UserOverride<Base> {
    type Base = Base;
    fn update<V>(storage: &mut impl TopLevelStorage)
    where
        V: Erc721Virtual,
    {
        println!("call user update");
        Base::update::<V>(storage);
    }
}

#[derive(Default, Debug)]
struct UserToken {
    // Smth else can be at UserToken storage
    erc721: Erc721<Override>,
}

impl UserToken {
    fn user_custom_transfer(&mut self) {
        Erc721Base::<Override>::transfer(self);
    }
}

unsafe impl StorageLevel for UserToken {
    unsafe fn try_get_storage<S: 'static>(&mut self) -> Option<&mut S> {
        if TypeId::of::<S>() == self.erc721.pausable.type_id() {
            Some(unsafe { std::mem::transmute::<_, _>(&mut self.erc721.pausable) })
        } else if TypeId::of::<S>() == self.erc721.base.type_id() {
            Some(unsafe { std::mem::transmute::<_, _>(&mut self.erc721.base) })
        } else if TypeId::of::<S>() == TypeId::of::<Self>() {
            Some(unsafe { &mut *(self as *mut Self as *mut S) })
        } else {
            None
        }
    }
}

// UserToken is terminal struct of contract. Then it should be TopLevelStorage.
// Should be auto implemented recursively for every inner contract
impl TopLevelStorage for UserToken {}

fn main() {
    let mut token = UserToken::default();
    token.user_custom_transfer();
}
