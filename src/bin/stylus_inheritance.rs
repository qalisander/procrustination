// aka stylus_sdk
mod stylus_lib {
    // Stylus sdk demands this trait to be implemented for
    // the terminal struct of the smart contract.
    pub trait TopLevelStorage {
        fn get_storage<T: 'static>(&mut self) -> &mut T {
            panic!("arbitrary storage access is not implemented",)
        }
    }
}

// aka rust_contracts_stylus
mod oz_lib {
    use crate::stylus_lib::TopLevelStorage;
    use base::{Erc721Base, Erc721BaseOverride};
    use pausable::{Erc721Pausable, Erc721PausableOverride};
    use std::borrow::BorrowMut;

    pub type Override = Erc721PausableOverride<Erc721BaseOverride>;

    // Trait used for overriding behaviour of update function.
    // Other functions can be added to this update function.
    // Or associated type for every "virtual" method can be used,
    // that will be restricted with trait named like (Erc721UpdateVirtual).
    pub trait Erc721Virtual: 'static + std::fmt::Debug {
        fn update<S, T>(storage: &mut S)
        where
            T: Erc721Virtual,
            S: TopLevelStorage;
    }

    // Library contract that will be reused by our consumers
    #[derive(Debug, Default)]
    pub struct Erc721<T: Erc721Virtual> {
        pub base: Erc721Base<T>,
        pub pausable: Erc721Pausable<T>,
    }

    pub mod pausable {
        use super::{Erc721, Erc721Virtual};
        use crate::oz_lib::base::Erc721Base;
        use crate::stylus_lib::TopLevelStorage;
        use std::borrow::BorrowMut;
        use std::marker::PhantomData;

        #[derive(Debug, Default)]
        pub struct Erc721Pausable<T: Erc721Virtual> {
            // Other fields for pausable omitted for simplicity
            phantom_data: PhantomData<T>,
        }

        // Overriding update function for pausalbe extension.
        // Here we can access Erc721<_> parent storage.
        // Basically Erc721Pausable and Erc721Base can be mutated.
        #[derive(Debug, Default)]
        pub struct Erc721PausableOverride<T: Erc721Virtual>(PhantomData<T>);
        impl<Base: Erc721Virtual> Erc721Virtual for Erc721PausableOverride<Base> {
            fn update<S, This>(storage: &mut S)
            where
                This: Erc721Virtual,
                S: TopLevelStorage,
            {
                let p: &mut Erc721Pausable<This> = storage.get_storage();
                dbg!(&p);
                println!("call pausable update");
                Base::update::<_, This>(storage);
            }
        }
    }

    pub mod base {
        use super::{Erc721, Erc721Virtual};
        use crate::stylus_lib::TopLevelStorage;
        use std::borrow::BorrowMut;
        use std::marker::PhantomData;

        #[derive(Debug, Default)]
        pub struct Erc721Base<T: Erc721Virtual> {
            // Other fields for erc721 missed for simplicity
            phantom_data: PhantomData<T>,
        }

        // Simplicity sake, we omit #[external] attribute and stylus sdk dependency.
        impl<T: Erc721Virtual> Erc721Base<T> {
            // Public transfer function of Erc721Base contract.
            pub fn transfer<S>(storage: &mut S)
            where
                S: TopLevelStorage,
            {
                println!("call base transfer");
                T::update::<_, T>(storage);
            }
        }

        // Base implementation of update function.
        #[derive(Debug, Default)]
        pub struct Erc721BaseOverride;
        impl Erc721Virtual for Erc721BaseOverride {
            fn update<S, T>(storage: &mut S)
            where
                T: Erc721Virtual,
                S: TopLevelStorage,
            {
                println!("call base update")
            }
        }
    }
}

use std::any::{Any, TypeId};
// Client code
use crate::oz_lib::base::Erc721Base;
use crate::oz_lib::{Erc721, Erc721Virtual};
use crate::stylus_lib::TopLevelStorage;
use std::borrow::{Borrow, BorrowMut};
use std::marker::PhantomData;

type Override = Erc721UserOverride<oz_lib::Override>;

// User can override but won't access storage of his own contract (UserToken)
// because of constraint of Erc721Virtual trait.
#[derive(Debug, Default)]
pub struct Erc721UserOverride<T: Erc721Virtual>(PhantomData<T>);
impl<Base: Erc721Virtual> Erc721Virtual for Erc721UserOverride<Base> {
    fn update<S, This>(storage: &mut S)
    where
        This: Erc721Virtual,
        S: TopLevelStorage,
    {
        println!("call user update");
        Base::update::<_, This>(storage);
    }
}

#[derive(Default)]
struct UserToken {
    // Smth else can be at UserToken storage
    erc721: Erc721<Override>,
}

impl UserToken {
    fn user_custom_transfer(&mut self) {
        Erc721Base::<Override>::transfer(self);
    }
}

// UserToken is terminal struct of contract. Then it should be TopLevelStorage.
// may be for auto implementation introduce Storage trait
impl TopLevelStorage for UserToken {
    fn get_storage<T: 'static>(&mut self) -> &mut T {
        if TypeId::of::<T>() == self.erc721.pausable.type_id() {
            unsafe { std::mem::transmute::<_, _>(&mut self.erc721.pausable) }
        } else if TypeId::of::<T>() == self.erc721.base.type_id() {
            unsafe { std::mem::transmute::<_, _>(&mut self.erc721.base) }
        } else {
            panic!(
                "storage for type doesn't exist - type name is {}",
                std::any::type_name::<T>()
            )
        }
    }
}

// Auto implemented with #[borrow] proc macro from stylus lib.
impl Borrow<Erc721<Override>> for UserToken {
    fn borrow(&self) -> &Erc721<Override> {
        &self.erc721
    }
}

// Auto implemented with #[borrow] proc macro from stylus lib.
impl BorrowMut<Erc721<Override>> for UserToken {
    fn borrow_mut(&mut self) -> &mut Erc721<Override> {
        &mut self.erc721
    }
}

fn main() {
    let mut token = UserToken::default();
    token.user_custom_transfer();
}
