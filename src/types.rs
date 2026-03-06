/// {0, 1}^λ
/// A 512-bit element used as input to the PSI Protocol
/// Elementes are assumed to be uniformly random 64-byte values
pub type Bytes<const N: usize> = [u8; N];

pub type Element = Bytes<64>;
