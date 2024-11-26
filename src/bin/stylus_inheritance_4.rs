use crate::oz_lib::{Erc721, Erc721Public, Erc721Storage, Erc721Virtual};

// aka stylus_sdk
mod stylus_lib {
    pub trait StorageType {}
}

// aka rust_contracts_stylus
mod oz_lib {
    use crate::stylus_lib::StorageType;
    use std::borrow::BorrowMut;
    use std::collections::HashMap;
    use std::fmt::{Debug, Formatter};
    use std::hash::Hash;

    pub trait Erc721Public: Erc721Virtual {
        fn transfer(&mut self) {
            println!("call public transfer");
            self.update();
        }
    }

    pub trait Erc721Virtual: Erc721Storage {
        fn mint(&mut self) {
            println!("call virtual mint");
            self.update();
        }

        fn update(&mut self) {
            println!("call virtual update");
            self.owner_of();
            self.get_mut().balances.insert(1, 1);
        }

        fn owner_of(&self) {
            println!("call virtual owner_of");
        }
    }

    pub trait Erc721Storage {
        fn get(&self) -> &Erc721;
        fn get_mut(&mut self) -> &mut Erc721;
    }

    #[derive(Debug, Default)]
    pub struct Erc721 {
        pub balances: HashMap<u32, u32>,
    }

    impl Erc721Public for Erc721 {}

    impl Erc721Virtual for Erc721 {}

    impl Erc721Storage for Erc721 {
        fn get(&self) -> &Erc721 {
            self
        }

        fn get_mut(&mut self) -> &mut Erc721 {
            self
        }
    }
}

// Client code
#[derive(Debug, Default)]
struct Erc721Example {
    base: Erc721,
    paused: bool,
}

impl Erc721Storage for Erc721Example {
    fn get(&self) -> &oz_lib::Erc721 {
        &self.base
    }

    fn get_mut(&mut self) -> &mut oz_lib::Erc721 {
        &mut self.base
    }
}

impl Erc721Virtual for Erc721Example {
    fn mint(&mut self) {
        println!("call example mint");
        self.update();
    }

    fn update(&mut self) {
        println!("call example update");
        self.base.update();
    }

    fn owner_of(&self) {
        println!("call example owner_of");
        self.base.owner_of();
    }
}

impl Erc721Public for Erc721Example {}

fn main() {
    let mut erc721 = Erc721Example::default();
    erc721.mint();
}
