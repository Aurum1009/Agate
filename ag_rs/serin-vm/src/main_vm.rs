use parking_lot::Mutex;

pub struct MainVM {
    pool_count: Mutex<usize>,
}
impl MainVM {
    pub fn mem_pool_count(&self) -> usize {
        let mut count = self.pool_count.lock();
        *count += 1;
        *count - 1
    }
}
