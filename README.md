use web3::signing::{keccak256, recover};

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

/*
{
  "address": "0x2353CB004f075faa3293b8219f2A38b9311F90ad",
  "msg": "login",
  "sig": "0xd69e56c834c353c6f21d36e6ea3c882f2dc1961f60b8fa2edc1193e7e6960aed4ba135477e987d4ebe62a072aec4c882b69e3249b3aaaa67948796e6498f780e1c",
  "version": "2"
}
*/
fn eth_message(message: String) -> [u8; 32] {
    keccak256(
        format!(
            "{}{}{}",
            "\x19Ethereum Signed Message:\n",
            message.len(),
            message
        )
        .as_bytes(),
    )
}




fn main() {
    println!("Hello, world!");
    let account = "0x2353CB004f075faa3293b8219f2A38b9311F90ad".to_string();
        let message = "0xhuhun".to_string();
        let message = eth_message(message);
        let signature = hex::decode("934816558a24c9ec9510ee7c2c77cdb0e503a6efbbc4e7af4fd3a2cae79554d814d858261aa28b2bc2a0395a95ebf642afd25cda35d69085c3c2369f421563681b").unwrap();
        println!("{} {:?} {:?}", account, message, signature);
        let pubkey = recover(&message, &signature[..64], 0);
        assert!(pubkey.is_ok());
        let pubkey = pubkey.unwrap();
        let pubkey = format!("{:02X?}", pubkey);
        
        assert_eq!(account.to_uppercase(), pubkey.to_uppercase())
}