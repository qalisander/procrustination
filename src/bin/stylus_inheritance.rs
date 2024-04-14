use crate::oz_lib::base::{Erc721Base, Erc721BaseOverride};
use crate::oz_lib::pausable::Erc721PausableOverride;
use crate::oz_lib::Erc721;
use crate::stylus_lib::TopLevelStorage;
use std::borrow::{Borrow, BorrowMut};

// NOTE: In this example other extensions won't access their own storage after update override

mod stylus_lib {
    pub trait TopLevelStorage {}
}

mod oz_lib {
    use crate::stylus_lib::TopLevelStorage;
    use base::Erc721Base;
    use pausable::Erc721Pausable;
    use std::borrow::BorrowMut;
    use std::marker::PhantomData;

    pub trait Erc721Virtual {
        fn update<S, T>(storage: &mut S)
        where
            T: Erc721Virtual,
            S: TopLevelStorage + BorrowMut<Erc721<T>>;
    }

    #[derive(Debug, Default)]
    pub struct Erc721<T: Erc721Virtual> {
        base: Erc721Base<T>,
        pausable: Erc721Pausable<T>,
        phantom_data: PhantomData<T>,
    }

    pub mod pausable {
        use super::{Erc721, Erc721Virtual};
        use crate::stylus_lib::TopLevelStorage;
        use std::borrow::BorrowMut;
        use std::marker::PhantomData;

        #[derive(Debug, Default)]
        pub struct Erc721Pausable<T: Erc721Virtual> {
            phantom_data: PhantomData<T>,
        }

        #[derive(Debug, Default)]
        pub struct Erc721PausableOverride<T: Erc721Virtual>(PhantomData<T>);

        impl<Base: Erc721Virtual> Erc721Virtual for Erc721PausableOverride<Base> {
            fn update<S, This>(storage: &mut S)
            where
                This: Erc721Virtual,
                S: TopLevelStorage + BorrowMut<Erc721<This>>,
            {
                println!("call pausable update");
                Base::update(storage);
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
            phantom_data: PhantomData<T>,
        }

        impl<T: Erc721Virtual> Erc721Base<T> {
            pub fn transfer<S>(storage: &mut S)
            where
                S: TopLevelStorage + BorrowMut<Erc721<T>>,
            {
                println!("call base transfer");
                T::update(storage);
            }
        }

        #[derive(Debug, Default)]
        pub struct Erc721BaseOverride;

        impl Erc721Virtual for Erc721BaseOverride {
            fn update<S, T>(storage: &mut S)
            where
                T: Erc721Virtual,
                S: TopLevelStorage + BorrowMut<Erc721<T>>,
            {
                println!("call base update")
            }
        }
    }
}

// NOTE: client code

type Override = Erc721PausableOverride<Erc721BaseOverride>;

#[derive(Default)]
struct Token {
    erc721: Erc721<Override>,
}

impl Borrow<Erc721<Override>> for Token {
    fn borrow(&self) -> &Erc721<Override> {
        &self.erc721
    }
}

impl BorrowMut<Erc721<Override>> for Token {
    fn borrow_mut(&mut self) -> &mut Erc721<Override> {
        &mut self.erc721
    }
}

impl TopLevelStorage for Token {}

fn main() {
    let mut token = Token::default();
    Erc721Base::<Override>::transfer(&mut token);
}
