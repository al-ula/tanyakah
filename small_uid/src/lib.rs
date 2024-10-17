mod error;

pub use error::SmallUidError;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::time::{SystemTime, UNIX_EPOCH};

type Error = SmallUidError;

const BASE64URL_CHARS: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '-', '_',
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SmallUid(pub u64);

impl SmallUid {
    pub fn new() -> Result<SmallUid, Error> {
        gen()
    }
    pub fn from_parts(timestamp: u64, random: u64) -> Result<SmallUid, Error> {
        compose(timestamp, random)
    }
    pub fn from_timestamp(timestamp: u64) -> Result<SmallUid, Error> {
        let timestamp = timestamp_check(timestamp)?;
        let random = random_gen()?;
        compose(timestamp, random)
    }
    pub fn from_random(random: u64) -> Result<SmallUid, Error> {
        let random = rng_size_check(random)?;
        let timestamp = timestamp_gen()?;
        compose(timestamp, random)
    }
}

impl From<u64> for SmallUid {
    fn from(value: u64) -> Self {
        SmallUid(value)
    }
}

impl TryFrom<String> for SmallUid {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let smalluidstr = base64url_check(&value)?;
        let mut smalluidvec = Vec::new();
        base64_url::decode_to_vec(smalluidstr, &mut smalluidvec)?;
        if smalluidvec.len() != 8 {
            return Err(SmallUidError::StringLen);
        };
        let smalluidu64 = u64::from_be_bytes(smalluidvec[0..8].try_into().unwrap());
        Ok(SmallUid(smalluidu64))
    }
}

impl From<SmallUid> for String {
    fn from(value: SmallUid) -> Self {
        value.to_string()
    }
}

impl From<SmallUid> for u64 {
    fn from(value: SmallUid) -> Self {
        value.0
    }
}

impl Display for SmallUid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let smalluid = base64_url::encode(&self.0.to_be_bytes());
        write!(f, "{}", smalluid)
    }
}

fn timestamp_gen() -> Result<u64, Error> {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)?;
    let timestamp = since_the_epoch.as_millis() as u64;
    let timestamp_str = format!("{:0b}", timestamp);
    timestamp_check(timestamp)
}

fn timestamp_check(timestamp: u64) -> Result<u64, Error> {
    let timestamp_str = format!("{:0b}", timestamp);
    if timestamp_str.len() <= 44 {
        Ok(timestamp)
    } else {
        Err(SmallUidError::TimestampLimit)
    }
}

fn random_gen() -> Result<u64, Error> {
    let random_20bit: u64 = rand::thread_rng().gen_range(0..(1 << 20));
    rng_size_check(random_20bit)
}

fn rng_size_check(rn: u64) -> Result<u64, Error> {
    let random_20bit_str = format!("{:0b}", rn);
    if random_20bit_str.len() <= 20 {
        Ok(rn)
    } else {
        Err(SmallUidError::RandomSizeLimit)
    }
}

fn gen() -> Result<SmallUid, Error> {
    let timestamp = timestamp_gen()?;
    let random = random_gen()?;
    Ok(SmallUid((timestamp << 20) | random))
}

fn compose(timestamp: u64, random: u64) -> Result<SmallUid, Error> {
    let timestamp = timestamp_check(timestamp)?;
    let random = rng_size_check(random)?;
    Ok(SmallUid((timestamp << 20) | random))
}

fn base64url_check(input: &str) -> Result<&str, Error> {
    if input.chars().all(|c| BASE64URL_CHARS.contains(&c)) {
        Ok(input)
    } else {
        Err(SmallUidError::NotABase64Url)
    }
}
