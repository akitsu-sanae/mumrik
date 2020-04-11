use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident(String);

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Ident {
    pub fn new(name: &str) -> Ident {
        Ident(name.to_string())
    }
    pub fn fresh() -> Ident {
        Ident(format!("<fresh-{}>", COUNTER.fetch_add(1, SeqCst)))
    }
    pub fn to_nf_ident(self) -> nf::ident::Ident {
        nf::ident::Ident(self.0)
    }
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);
