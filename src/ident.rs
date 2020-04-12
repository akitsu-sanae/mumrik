use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};

#[cfg(not(test))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ident(String);

#[cfg(test)]
#[derive(Debug, Clone, PartialOrd, Ord, Hash)] // redefine `eq` for test
pub struct Ident(String);

#[cfg(test)]
mod test_impl {
    impl PartialEq for super::Ident {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
                || (self.0.starts_with("<fresh-") && other.0.as_str() == "<fresh-expected>")
        }
    }

    impl Eq for super::Ident {}
}

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
