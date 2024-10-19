use web3::types::H160;
use hex::FromHex;

fn string_to_H160(hex_str: &str) -> H160 {
    let cleaned_hex = hex_str.trim_start_matches("0x");

    // Convert hex string to bytes
    let bytes = <[u8; 20]>::from_hex(cleaned_hex).unwrap();

    // Convert bytes to H160
    let address = H160::from(bytes);
    address
}

pub fn make_contract_address_list(hex_address_list: Vec<&str>) -> Vec<H160> {
    let mut address_list = Vec::<H160>::new();
    for address in hex_address_list {
        address_list.push(string_to_H160(address));
    }
    address_list
}

pub const BASE_HEX_ADDRESS_LIST: &[&str] = &[
    "0xfde4C96c8593536E31F229EA8f37b2ADa2699bb2",
    "0x4200000000000000000000000000000000000006",
    "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
    "0xcbB7C0000aB88B473b1f5aFd9ef808440eed33Bf",
    "0xcf3D55c10DB69f28fD1A75Bd73f3D8A2d9c595ad",
    "0x88Fb150BDc53A65fe94Dea0c9BA0a6dAf8C6e196",
    "0xB6f7B61342A873C82fbd366aaa5a4567Fb7c4D11",
];