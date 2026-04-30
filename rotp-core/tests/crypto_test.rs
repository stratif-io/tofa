use rotp_core::crypto::{decrypt, encrypt};

#[test]
fn roundtrip_encrypt_decrypt() {
    let passphrase = "correct horse battery staple";
    let plaintext = b"hello world secret";

    let ciphertext = encrypt(passphrase, plaintext).unwrap();
    let decrypted = decrypt(passphrase, &ciphertext).unwrap();

    assert_eq!(decrypted, plaintext);
}

#[test]
fn wrong_passphrase_fails() {
    let ciphertext = encrypt("right", b"data").unwrap();
    let result = decrypt("wrong", &ciphertext);
    assert!(result.is_err());
}

#[test]
fn ciphertext_is_not_plaintext() {
    let plaintext = b"supersecret";
    let ciphertext = encrypt("pass", plaintext).unwrap();
    assert!(!ciphertext.windows(plaintext.len()).any(|w| w == plaintext));
}

#[test]
fn two_encryptions_produce_different_ciphertext() {
    let ciphertext1 = encrypt("pass", b"data").unwrap();
    let ciphertext2 = encrypt("pass", b"data").unwrap();
    assert_ne!(ciphertext1, ciphertext2); // different random salt+nonce
}
