use crc32fast::Hasher;

const TOTAL_BUCKET_NUM: u32 = 2 /* Replace with the total number of buckets */;

pub fn crc32_num(entity_id: &str, salt: &str) -> u32 {
    // crc32 is good in terms of uniform distribution
    // http://michiel.buddingh.eu/distribution-of-hash-values
    let mut hasher = Hasher::new();
    hasher.update(salt.as_bytes());
    hasher.update(entity_id.as_bytes());
    let checksum = hasher.finalize();
    checksum % TOTAL_BUCKET_NUM
}
