mod stylus_lib {
    pub trait TopLevelStorage {
        type Extension;

        fn get_extension_mut(&mut self) -> &mut Self::Extension;
    }

    // pub trait Extension {}
}

mod oz_lib {
    use crate::stylus_lib::TopLevelStorage;
    use std::marker::PhantomData;

    pub trait Erc721Virtual {}

    #[derive(Debug, Default)]
    pub struct Erc721Override;

    impl Erc721Virtual for Erc721Override {}

    #[derive(Debug, Default)]
    pub struct Erc721<T: Erc721Virtual> {
        erc721: Erc721Base<T>,
        pausable: Erc721Pausable<T>,
        phantom_data: PhantomData<T>,
    }

    #[derive(Debug, Default)]
    pub struct Erc721Pausable<T: Erc721Virtual> {
        phantom_data: PhantomData<T>,
    }

    impl<T: Erc721Virtual> Erc721Pausable<T> {
        pub fn get_erc721base(
            storage: &mut impl TopLevelStorage<Extension = Erc721<T>>,
        ) -> &mut Erc721Base<T> {
            &mut storage.get_extension_mut().erc721
        }
    }

    #[derive(Debug, Default)]
    pub struct Erc721Base<T: Erc721Virtual> {
        phantom_data: PhantomData<T>,
    }

    impl<T: Erc721Virtual> Erc721Base<T> {
        pub fn get_pausable(
            storage: &mut impl TopLevelStorage<Extension = Erc721<T>>,
        ) -> &mut Erc721Pausable<T> {
            &mut storage.get_extension_mut().pausable
        }
    }
}

use crate::oz_lib::{Erc721, Erc721Override, Erc721Pausable};
use crate::stylus_lib::TopLevelStorage;

#[derive(Default)]
struct Token {
    erc721: Erc721<Erc721Override>,
}

impl TopLevelStorage for Token {
    type Extension = Erc721<Erc721Override>;

    fn get_extension_mut(&mut self) -> &mut Self::Extension {
        &mut self.erc721
    }
}

fn main() {
    let mut token = Token::default();
    let base = Erc721Pausable::<Erc721Override>::get_erc721base(&mut token);
    dbg!(&base);
}
