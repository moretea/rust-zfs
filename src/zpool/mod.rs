pub struct PoolIterator {
}

#[derive(Debug,PartialEq)]
pub struct Pool {
}

pub fn iter() -> PoolIterator {
    PoolIterator { }
}

impl Iterator for PoolIterator {
    type Item = Pool;

    fn next(&mut self) -> Option<Pool> {
        None
    }
}
