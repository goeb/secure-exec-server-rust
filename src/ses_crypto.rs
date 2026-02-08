pub struct PublicKey {
    pub key: Vec<u8>,
    pub filename: String
}

impl Clone for PublicKey {
    fn clone(&self) -> PublicKey {
        PublicKey {
            key: self.key.to_vec(),
            filename: self.filename.clone()
        }
    }
}

pub fn load_public_key(certpath: &String) -> std::io::Result<PublicKey> {
    Ok(PublicKey {
        key: Vec::new(), // TODO
        filename: "TODO".to_string(),
    })
}
