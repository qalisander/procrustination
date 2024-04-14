mod stylus_lib {
    use std::borrow::BorrowMut;

    pub trait TopLevelStorage {}
}

mod oz_lib {
    use crate::stylus_lib::TopLevelStorage;
    use std::borrow::BorrowMut;
    use std::marker::PhantomData;

    pub trait Erc721Virtual {}

    #[derive(Debug, Default)]
    pub struct Erc721Override;

    impl Erc721Virtual for Erc721Override {}

    #[derive(Debug, Default)]
    pub struct Erc721<T: Erc721Virtual> {
        base: Erc721Base<T>,
        pausable: Erc721Pausable<T>,
        phantom_data: PhantomData<T>,
    }

    #[derive(Debug, Default)]
    pub struct Erc721Pausable<T: Erc721Virtual> {
        phantom_data: PhantomData<T>,
    }

    impl<T: Erc721Virtual> Erc721Pausable<T> {
        pub fn get_erc721base<S>(storage: &mut S) -> &mut Erc721Base<T>
        where
            S: TopLevelStorage + BorrowMut<Erc721<T>>,
        {
            &mut storage.borrow_mut().base
        }
    }

    #[derive(Debug, Default)]
    pub struct Erc721Base<T: Erc721Virtual> {
        phantom_data: PhantomData<T>,
    }

    impl<T: Erc721Virtual> Erc721Base<T> {
        pub fn get_pausable<S>(storage: &mut S) -> &mut Erc721Pausable<T>
        where
            S: TopLevelStorage + BorrowMut<Erc721<T>>,
        {
            &mut storage.borrow_mut().pausable
        }
    }
}

use crate::oz_lib::{Erc721, Erc721Override, Erc721Pausable};
use crate::stylus_lib::TopLevelStorage;
use std::borrow::{Borrow, BorrowMut};

#[derive(Default)]
struct Token {
    erc721: Erc721<Erc721Override>,
}

impl Borrow<Erc721<Erc721Override>> for Token {
    fn borrow(&self) -> &Erc721<Erc721Override> {
        &self.erc721
    }
}

impl BorrowMut<Erc721<Erc721Override>> for Token {
    fn borrow_mut(&mut self) -> &mut Erc721<Erc721Override> {
        &mut self.erc721
    }
}

impl TopLevelStorage for Token {}

fn main() {
    let mut token = Token::default();
    let base = Erc721Pausable::<Erc721Override>::get_erc721base(&mut token);
    dbg!(&base);
}
