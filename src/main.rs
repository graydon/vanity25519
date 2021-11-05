extern crate rand;
extern crate ed25519_dalek;

use std::{process::exit, thread};

use rand::{prelude::StdRng,rngs::OsRng,SeedableRng};
use ed25519_dalek::{Keypair, PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH};

fn search(target: String) {
    let mut csprng = StdRng::from_rng(OsRng{}).unwrap();
    let needle = target.as_bytes();
    loop {        
        let keypair: Keypair = Keypair::generate(&mut csprng);
        let pub_bytes: [u8; PUBLIC_KEY_LENGTH] = keypair.public.to_bytes();
        let mut pub_enc = [0;256];
        let npub = base64::encode_config_slice(&pub_bytes, base64::STANDARD, &mut pub_enc);
        if pub_enc.windows(needle.len()).any(|window| window == needle) {
            let sec_bytes: [u8; SECRET_KEY_LENGTH] = keypair.secret.to_bytes();
            let mut sec_enc = [0;256];
            let npriv = base64::encode_config_slice(&sec_bytes, base64::STANDARD, &mut sec_enc);
            println!("secret: {}", String::from_utf8_lossy(&sec_enc[0..npriv]));
            println!("public: {}", String::from_utf8_lossy(&pub_enc[0..npub]));
            exit(0);
        }
    }
}

fn main()
{
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("usage: {} <searchstring>", args[0]);
        exit(1);
    }
    let target = &args[1];
    let num = num_cpus::get();
    println!("searching for key with public part containing '{}' on {} threads", target, num);
    let handles = (0..num)
        .into_iter()
        .map(|_| {
            let t = target.clone();
            thread::spawn(move || search(t))})
        .collect::<Vec<_>>();
    handles.into_iter().for_each(|h| h.join().unwrap());
}