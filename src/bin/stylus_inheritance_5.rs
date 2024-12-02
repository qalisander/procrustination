use crate::oz_lib::{Erc721, IErc721, IPausable, Pausable};
use crate::stylus_lib::Router;

// aka stylus_sdk
mod stylus_lib {
    pub trait StorageType {}

    pub trait Router {
        fn route(&mut self, selector: u32, input: Vec<u8>) -> Option<()>;
    }
}

// aka rust_contracts_stylus
mod oz_lib {
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::hash::Hash;

    pub trait IErc721 {
        fn transfer(&mut self);

        fn __route(&mut self, selector: u32, input: Vec<u8>) -> Option<()> {
            match selector {
                0 => {
                    self.transfer();
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

    impl IErc721 for Erc721 {
        fn transfer(&mut self) {
            println!("call public transfer");
        }
    }

    pub trait IPausable {
        fn pause(&mut self);
        fn unpause(&mut self);

        fn __route(&mut self, selector: u32, input: Vec<u8>) -> Option<()> {
            match selector {
                0 => {
                    self.pause();
                    Some(())
                }
                1 => {
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

    impl IPausable for Pausable {
        fn pause(&mut self) {
            println!("call public pause");
        }

        fn unpause(&mut self) {
            println!("call public unpause");
        }
    }

    // pub trait VErc721: SErc721 {
    //     fn mint(&mut self) {
    //         println!("call virtual mint");
    //         self.update();
    //     }
    //
    //     fn update(&mut self) {
    //         println!("call virtual update");
    //         self.owner_of();
    //         self.get_mut().balances.insert(1, 1);
    //     }
    //
    //     fn owner_of(&self) {
    //         println!("call virtual owner_of");
    //     }
    // }
    //
    // pub trait SErc721 {
    //     fn get(&self) -> &Erc721;
    //     fn get_mut(&mut self) -> &mut Erc721;
    // }
}

// Client code
#[derive(Debug, Default)]
struct Erc721Example {
    base: Erc721,
    pausable: Pausable,
}

impl Router for Erc721Example {
    fn route(&mut self, selector: u32, input: Vec<u8>) -> Option<()> {
        if let Some(()) = self.base.__route(selector, input) {
            return Some(());
        }

        if let Some(()) = self.pausable.__route(selector, input) {
            return Some(());
        }

        None
    }
}

fn main() {
    let mut erc721 = Erc721Example::default();
    erc721.route(0, vec![]).unwrap()
}
