//! A library that can deconjugate japanese verbs/adjectives into possible root words that can be
//! looked up in a dictionary

mod conjugate;
mod deconjugate;
mod root;
#[cfg(test)]
mod tests;

pub use {
    deconjugate::deconjugate,
    root::{Root, RootKind, Step},
};
