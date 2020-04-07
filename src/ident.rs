use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident(String);

impl Ident {
    pub fn new(name: &str) -> Ident {
        Ident(name.to_string())
    }
    pub fn fresh() -> Ident {
        Ident(format!("<fresh-{}>", COUNTER.fetch_add(1, SeqCst)))
    }
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);
