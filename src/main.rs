use rslwe::lwe::lwe_cypher::LweCypher;
use rslwe::lwe::lwe_params::LweParams;
use rslwe::lwe::lwe_private_key::PrivateKey;
use rslwe::lwe::lwe_public_key::PublicKey;

struct User {
    _name: String,
    pub_key: PublicKey,
    sec_key: PrivateKey,
    engine: LweCypher,
}

impl User {
    fn new(name: &str, params: LweParams, seed: u64) -> Self {
        let mut engine = LweCypher::new(params, seed);
        let (pub_key, sec_key) = engine.keygen();
        Self {
            _name: name.to_string(),
            pub_key,
            sec_key,
            engine,
        }
    }
}

fn main() {
    let params = LweParams {
        n: 512,
        m: 1024,
        q: 3329,
    };

    let alice = User::new("Alice", params, 12345);
    let mut bob = User::new("Bob", params, 67890);

    println!("--- LWE ---");

    let secret_msg = "Rust LWE";
    println!("Bob tries to send : \"{}\"", secret_msg);

    let bits = string_to_bits(secret_msg);
    let mut encrypted_bundle = Vec::new();

    for &bit in &bits {
        encrypted_bundle.push(bob.engine.encrypt(&alice.pub_key, bit));
    }

    let mut decrypted_bits = Vec::new();
    for ct in encrypted_bundle {
        decrypted_bits.push(alice.engine.decrypt(&alice.sec_key, ct));
    }

    let final_msg = bits_to_string(&decrypted_bits);
    println!("Alice received : \"{}\"", final_msg);

    if secret_msg == final_msg {
        println!("--- All Good :D ---");
    } else {
        println!("--- Well... Something is not working. ---");
    }
}

fn string_to_bits(s: &str) -> Vec<bool> {
    s.as_bytes()
        .iter()
        .flat_map(|&b| (0..8).rev().map(move |i| (b >> i) & 1 == 1))
        .collect()
}

fn bits_to_string(bits: &[bool]) -> String {
    let bytes: Vec<u8> = bits
        .chunks(8)
        .map(|chunk| chunk.iter().fold(0, |acc, &b| (acc << 1) | (b as u8)))
        .collect();
    String::from_utf8_lossy(&bytes).into_owned()
}
