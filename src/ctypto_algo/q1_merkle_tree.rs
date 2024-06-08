use sha2::{Digest, Sha256};

fn merkle_root(data: &[Vec<u8>]) -> Vec<u8> {
    if data.is_empty() {
        return Sha256::new().finalize().to_vec();
    }

    let mut hashes = data.iter().map(|d| hash(d)).collect::<Vec<_>>();

    while hashes.len() > 1 {
        hashes = hashes
            .chunks(2)
            .map(|c| {
                let mut hasher = Sha256::new();
                hasher.update(&c[0]);
                hasher.update(&c.get(1).unwrap_or(&c[0]));
                hasher.finalize().to_vec()
            })
            .collect::<Vec<_>>();
    }

    hashes[0].clone()
}

fn hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let ret = hasher.finalize().to_vec();
    eprintln!("hash ret = {:?}", ret);
    ret
}
#[test]
fn merkle_root_work() {
    let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    let root_hash = merkle_root(&data);

    println!("Merkle root hash: {:?}", root_hash);
}
