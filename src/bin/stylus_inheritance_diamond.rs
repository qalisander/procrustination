// trait Erc20Virtual {
//     type Base: Erc20Virtual;
//
//     fn update(&self) {
//         Self::Base::update(self);
//     }
// }
//
// struct Erc20Override;
//
// impl Erc20Virtual for Erc20Override {
//     type Base = Erc20Override;
//
//     fn update(&self) {
//         println!("base update")
//     }
// }
//
// ////////////////
//
// trait PausableVirtual {
//     type Base: PausableVirtual;
//
//     fn pause(&self) {
//         Self::Base::pause(self)
//     }
// }
//
// struct PausableOverride;
//
// impl PausableVirtual for PausableOverride {
//     type Base = PausableOverride;
//     fn pause(&self) {
//         println!("base pause")
//     }
// }
//
// /////////////////////////
//
// trait UserTokenVirtual: Erc20Virtual + PausableVirtual {
//     // type Base: Erc20Virtual;
//     // type Base: PausableVirtual;
// }
//
// struct UserTokenOverride;
//
// impl Erc20Virtual for UserTokenOverride {
//     type Base = Erc20Override;
// }
//
// impl PausableVirtual for UserTokenOverride {
//     type Base = PausableOverride;
// }
//
// impl UserTokenVirtual for UserTokenOverride {
//     // type Base = PausableOverride;
// }

#[tokio::main]
async fn main() {
    println!("Hello world");
}
