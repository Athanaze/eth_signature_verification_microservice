#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/<addr>/<signature>")]
fn login(addr: String, signature: String) -> String {
   if verify(addr, signature){
    "Valid signature".to_string()
   }else{
    "Invalid signature".to_string()
   }
}

fn get_vec_from_signature(signature: String) -> Result<Vec<u8>, FromHexError> {
    let mut s = signature;
    s.remove(0);
    s.remove(0);
    hex::decode(s)
}

fn verify(a: String, s: String) -> bool {
    let account: String = a;
    let message: String = "0xhuhun".to_string();
    let message = eth_message(message);
    let mut b = false;
    match get_vec_from_signature(s) {
        Ok(signature) => {

            match recover(&message, &signature[..64], 0) {
                Ok(pubkey) => {
                    let pubkey = format!("{:02X?}", pubkey);
                    println!("{}", account.to_uppercase());
                    println!("{}", pubkey.to_uppercase());
                    b = (account.to_uppercase() == pubkey.to_uppercase());
                },
                _ => (),
            }
        },
        _ => (),
    };
    b
}

fn main() {
    rocket::ignite().mount("/login", routes![login]).launch();
}

use hex::FromHexError;
use web3::signing::{keccak256, recover};

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