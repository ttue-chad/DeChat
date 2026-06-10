use wasm_bindgen::prelude::*;

use hkdf::Hkdf;

use sha2::Sha256;

use rand::rngs::OsRng;

use x25519_dalek::{
    PublicKey,
    StaticSecret
};

use chacha20poly1305::{
    ChaCha20Poly1305,
    KeyInit,
    Nonce,
    aead::Aead
};

#[wasm_bindgen]
pub fn version() -> String {
    "0.1.0".to_string()
}

#[wasm_bindgen]
pub struct Identity {
    private_key: StaticSecret,
    public_key: PublicKey,
}

#[wasm_bindgen]
impl Identity {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {

        let private_key =
            StaticSecret::random_from_rng(
                OsRng
            );

        let public_key =
            PublicKey::from(
                &private_key
            );

        Self {
            private_key,
            public_key
        }
    }

    pub fn public_key_hex(
        &self
    ) -> String {

        hex::encode(
            self.public_key.as_bytes()
        )

    }

}

#[wasm_bindgen]
impl Identity {

    pub fn create_shared_secret(
        &self,
        peer_public_hex: String
    ) -> String {

        let peer_bytes =
            hex::decode(
                peer_public_hex
            )
            .unwrap();

        let peer_pub =
            PublicKey::from(
                <[u8;32]>::try_from(
                    peer_bytes
                ).unwrap()
            );

        let shared =
            self.private_key
                .diffie_hellman(
                    &peer_pub
                );

        hex::encode(
            shared.as_bytes()
        )
    }

}

#[wasm_bindgen]
pub fn derive_key(
    secret_hex:String
) -> String {

    let secret =
        hex::decode(
            secret_hex
        ).unwrap();

    let hk =
        Hkdf::<Sha256>::new(
            None,
            &secret
        );

    let mut okm =
        [0u8;32];

    hk.expand(
        b"chat-key",
        &mut okm
    )
    .unwrap();

    hex::encode(okm)
}

#[wasm_bindgen]
pub fn encrypt(
    key_hex:String,
    plaintext:String
) -> String {

    let key_bytes =
        hex::decode(
            key_hex
        ).unwrap();

    let cipher =
        ChaCha20Poly1305::new_from_slice(
            &key_bytes
        )
        .unwrap();

    let nonce =
        Nonce::from_slice(
            b"unique nonce"
        );

    let encrypted =
        cipher.encrypt(
            nonce,
            plaintext.as_bytes()
        )
        .unwrap();

    hex::encode(
        encrypted
    )
}

#[wasm_bindgen]
pub fn decrypt(
    key_hex:String,
    cipher_hex:String
) -> String {

    let key =
        hex::decode(
            key_hex
        ).unwrap();

    let cipher =
        ChaCha20Poly1305::new_from_slice(
            &key
        )
        .unwrap();

    let nonce =
        Nonce::from_slice(
            b"unique nonce"
        );

    let ciphertext =
        hex::decode(
            cipher_hex
        ).unwrap();

    let plain =
        cipher.decrypt(
            nonce,
            ciphertext.as_ref()
        )
        .unwrap();

    String::from_utf8(
        plain
    )
    .unwrap()
}

