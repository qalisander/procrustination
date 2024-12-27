use crate::oz_lib::{Erc721, IErc721, IPausable, Pausable, VErc721};
use crate::stylus_lib::Router;
use std::borrow::{Borrow, BorrowMut};

// aka stylus_sdk
mod stylus_lib {
    pub trait StorageType {}

    pub trait Router {
        fn route(&mut self, selector: u32, input: Vec<u8>) -> Option<()>;
    }
}

// aka rust_contracts_stylus
mod oz_lib {
    use std::borrow::BorrowMut;
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::hash::Hash;

    // iterface
    pub trait IErc721: VErc721 {
        const SELECTORS: [u32; 4] = [0, 1, 2, 3];

        fn transfer(&mut self) {
            println!("call base transfer");
            self.update()
        }

        fn mint(&mut self) {
            println!("call base mint");
            self.update()
        }

        fn __route(&mut self, selector: u32, input: &Vec<u8>) -> Option<()> {
            match selector {
                0 => {
                    self.transfer();
                    Some(())
                }
                1 => {
                    self.mint();
                    Some(())
                }
                _ => None,
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct Erc721 {
        pub balances: HashMap<u32, u32>,
    }

    impl VErc721 for Erc721 {
        type Base = Self;

        fn update(&mut self) {
            println!("call base update");
        }

        fn owner_of(&mut self) {
            println!("call base owner_of");
        }
    }

    impl IErc721 for Erc721 {}

    pub trait VErc721: BorrowMut<Self::Base> {
        type Base: VErc721;

        fn update(&mut self) {
            self.borrow_mut().update()
        }

        fn owner_of(&mut self) {
            self.borrow_mut().owner_of()
        }
    }

    // inteface
    pub trait IPausable {
        fn pause(&mut self);
        fn unpause(&mut self);

        fn __route(&mut self, selector: u32, input: &Vec<u8>) -> Option<()> {
            match selector {
                2 => {
                    self.pause();
                    Some(())
                }
                3 => {
                    self.unpause();
                    Some(())
                }
                _ => None,
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct Pausable {
        paused: bool,
    }

    impl Pausable {
        pub fn when_paused(&mut self) {
            println!("call when_paused - paused: {}", self.paused);
        }
    }

    impl IPausable for Pausable {
        fn pause(&mut self) {
            println!("call pause");
        }

        fn unpause(&mut self) {
            println!("call unpause");
        }
    }
}

// Client code
#[derive(Debug, Default)]
struct Erc721Example {
    base: Erc721,
    pausable: Pausable,
}

impl Borrow<Erc721> for Erc721Example {
    fn borrow(&self) -> &Erc721 {
        &self.base
    }
}

impl BorrowMut<Erc721> for Erc721Example {
    fn borrow_mut(&mut self) -> &mut Erc721 {
        &mut self.base
    }
}

impl VErc721 for Erc721Example {
    type Base = Erc721;

    fn update(&mut self) {
        self.pausable.when_paused();
        self.base.update();
    }
}

impl IErc721 for Erc721Example {}

impl Router for Erc721Example {
    fn route(&mut self, selector: u32, input: Vec<u8>) -> Option<()> {
        if let Some(()) = self.base.__route(selector, &input) {
            return Some(());
        }

        if let Some(()) = self.pausable.__route(selector, &input) {
            return Some(());
        }

        None
    }
}

macro_rules! const_assert {
    ($x:expr $(,)?) => {
        const_assert!($x, "const assertion failed",);
    };
    ($x:expr, $es:expr $(,)?) => {
        const _: () = {
            const fn assert_internal() {
                if !$x {
                    core::panic!($es);
                }
            }
            const _: () = assert_internal();
        };
    };
}

fn main() {
    const_assert!(selector_exists(10), "selector 10 not found");

    let mut erc721 = Erc721Example::default();
    erc721.transfer()
}

const fn selector_exists(selector: u32) -> bool {
    let mut index = 0;
    while index < <Erc721Example as IErc721>::SELECTORS.len() {
        if selector == <Erc721Example as IErc721>::SELECTORS[index] {
            return true;
        }
        index += 1;
    }
    return false;
}
