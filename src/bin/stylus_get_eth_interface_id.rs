use alloy_primitives::keccak256;
use stylus_sdk::{
    abi::Bytes,
    alloy_primitives::{Address, FixedBytes, U256},
    function_selector,
};
// 0x01ffc9a7u32

fn main() {
    let interface = "onERC721Received(address,address,uint256,bytes)";
    let interface_id = get_interface_function_id(interface);
    println!("{:#x}", interface_id);
    let interface_id = get_interface_id([interface]);
    println!("{:#x}", interface_id);
    println!(
        "{:#x}",
        u32::from_be_bytes(function_selector!(
            "onERC721Received",
            Address,
            Address,
            U256,
            Bytes
        ))
    );

    println!("{:#x}", get_interface_id(["supportsInterface(bytes4)"]));
    println!(
        "{:#x}",
        u32::from_be_bytes(function_selector!("supportsInterface", FixedBytes<4>))
    );

    println!(
        "{:#x}",
        get_interface_id([
            "balanceOf(address)",
            "ownerOf(uint256)",
            "safeTransferFrom(address,address,uint256,bytes)",
            "safeTransferFrom(address,address,uint256)",
            "transferFrom(address,address,uint256)",
            "approve(address,uint256)",
            "setApprovalForAll(address,bool)",
            "getApproved(uint256)",
            "isApprovedForAll(address,address)",
        ])
    )
}

fn get_interface_function_id(fn_signature: &str) -> u32 {
    let interface_id = keccak256(fn_signature);
    let interface_id: [u8; 4] = interface_id.0[..4]
        .try_into()
        .unwrap_or_else(|_| panic!("function id is {}", fn_signature));
    u32::from_be_bytes(interface_id)
}

pub const fn get_interface_id<const L: usize>(input: [&'static str; L]) -> u32 {
    const fn hash(line: &str) -> u32 {
        let digest = stylus_sdk::keccak_const::Keccak256::new()
            .update(line.as_bytes())
            .finalize();
        let truncated_digest = [digest[0], digest[1], digest[2], digest[3]];
        u32::from_be_bytes(truncated_digest)
    }

    // NOTE: const fn context limits us to imperative implementation
    let mut xor = 0;
    let mut i = 0;
    while i < input.len() {
        xor ^= hash(input[i]);
        i += 1;
    }
    xor
}
