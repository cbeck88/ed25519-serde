use ed25519_dalek::{Keypair, KEYPAIR_LENGTH};

static KEYPAIR_BYTES: [u8; KEYPAIR_LENGTH] = [
    239, 085, 017, 235, 167, 103, 034, 062,
    007, 010, 032, 146, 113, 039, 096, 174,
    003, 219, 232, 166, 240, 121, 167, 013,
    098, 238, 122, 116, 193, 114, 215, 213,
    175, 181, 075, 166, 224, 164, 140, 146,
    053, 120, 010, 037, 104, 094, 136, 225,
    249, 102, 171, 160, 097, 132, 015, 071,
    035, 056, 000, 074, 130, 168, 225, 071, ];

fn main() {
    let keypair = Keypair::from_bytes(&KEYPAIR_BYTES).unwrap();
    let cbor_bytes = serde_cbor::to_vec(&keypair).unwrap();
    println!("Encoded length: {}", cbor_bytes.len());
}
