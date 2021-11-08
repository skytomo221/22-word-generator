use serde::{Deserialize, Serialize};


#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Phoneme {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    SCHWA,
}
