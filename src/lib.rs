mod multimixer_128;

#[cfg(feature = "internal")]
pub use multimixer_128::{f_128, multimixer_128, BLOCK_SIZE};
#[cfg(not(feature = "internal"))]
pub use multimixer_128::{multimixer_128, BLOCK_SIZE};
