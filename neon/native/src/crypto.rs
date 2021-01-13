use std::vec::Vec;
use sodiumoxide::crypto::{hash, pwhash, secretbox};
use libsodium_sys;

pub use sodiumoxide::crypto::secretbox::Key;

/// Derives a secret key from the password and the pseudo
pub fn derive_key(pseudo: &str, password: &str) -> Key {
    let mut key = Key([0; secretbox::KEYBYTES]);
    let hash = hash::sha256::hash(&Vec::from(password));
    let salt = pwhash::Salt::from_slice(hash.as_ref()).unwrap();
    {
        let secretbox::Key(ref mut kb) = key;
        pwhash::derive_key(kb, pseudo.as_ref(), &salt,
                           pwhash::OPSLIMIT_INTERACTIVE,
                           pwhash::MEMLIMIT_INTERACTIVE).unwrap();
    }
    key
}

pub fn encrypt(plain: &[u8], &Key(ref key): &Key) -> Vec<u8> {
    let nonce = secretbox::gen_nonce();
    let secretbox::Nonce(nonceb) = nonce;

    let clen = plain.len() + secretbox::MACBYTES;
    let mut cipher = Vec::with_capacity(clen + secretbox::NONCEBYTES);
    unsafe {
        cipher.set_len(clen);
        libsodium_sys::crypto_secretbox_easy(cipher.as_mut_ptr(),
                                             plain.as_ptr(),
                                             plain.len() as u64,
                                             nonceb.as_ptr(),
                                             key.as_ptr());
    }

    cipher.extend_from_slice(&nonceb);
    cipher
}

pub fn decrypt(cipher: &[u8], key: &Key) -> Result<Vec<u8>, String> {
    if cipher.len() < secretbox::NONCEBYTES {
        return Err("Input too small, data length shorter than nonce".to_string());
    }
    let nonce_index = cipher.len() - secretbox::NONCEBYTES;
    let mut nonce = [0; secretbox::NONCEBYTES];
    for (dst, src) in nonce.iter_mut().zip(cipher[nonce_index..].iter()) {
        *dst = *src;
    }

    let maybe_plain = secretbox::open(&cipher[0..nonce_index], &secretbox::Nonce(nonce), key);
    if maybe_plain.is_ok() {
        Ok(maybe_plain.unwrap())
    } else {
        Err("Unable to open secretbox, key is probably wrong".to_string())
    }
}
