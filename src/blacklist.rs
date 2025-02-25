use crate::errors::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::Arc;

const MAX_ITERATIONS: usize = 5;

#[derive(Debug)]
struct BlackListInner {
    map: HashMap<Vec<u8>, ()>,
}

#[derive(Clone, Debug)]
pub struct BlackList {
    inner: Arc<BlackListInner>,
    max_iterations: usize,
}

impl BlackList {
    pub fn new(map: HashMap<Vec<u8>, ()>, max_iterations: usize) -> Self {
        let inner = Arc::new(BlackListInner { map });
        BlackList {
            inner,
            max_iterations,
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut map = HashMap::new();
        let fp = BufReader::new(File::open(path)?);
        for (line_nb, line) in fp.lines().enumerate() {
            let line = line?;
            let mut line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            while line.starts_with("*.") {
                line = &line[2..];
            }
            while line.ends_with('.') {
                line = &line[..line.len() - 1];
            }
            let qname = line.as_bytes().to_vec().to_ascii_lowercase();
            if qname.is_empty() {
                bail!(format_err!("Unexpected blacklist rule at line {}", line_nb))
            }
            map.insert(qname, ());
        }
        Ok(BlackList::new(map, MAX_ITERATIONS))
    }

    pub fn find(&self, qname: &[u8]) -> bool {
        let qname = qname.to_vec().to_ascii_lowercase();
        let mut qname = qname.as_slice();
        let map = &self.inner.map;
        let mut iterations = self.max_iterations;
        while qname.len() >= 4 && iterations > 0 {
            if map.contains_key(qname) {
                return true;
            }
            let mut it = qname.splitn(2, |x| *x == b'.');
            if it.next().is_none() {
                break;
            }
            qname = match it.next() {
                None => break,
                Some(qname) => qname,
            };
            iterations -= 1;
        }
        false
    }
}
