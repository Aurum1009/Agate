use crate::main_vm::MainVM;
use crossbeam::queue::SegQueue;
use parking_lot::RwLock;
use std::sync::Arc;

/// Similar to `Value`, but no branch is ref-counted or thread-safe.
pub enum RawValue {}

pub struct MemPool {
    ident_code: usize,
    values: RwLock<SegQueue<Arc<RawValue>>>,
}
impl MemPool {
    pub fn new(code: Option<usize>, vm: &MainVM) -> Self {
        let code = code.unwrap_or_else(|| vm.mem_pool_count());
        Self {
            ident_code: code,
            values: RwLock::new(SegQueue::new()),
        }
    }
    pub fn write_raw(&mut self, value: RawValue) {
        let mut lock = self.values.write();
        let lock: &mut SegQueue<Arc<RawValue>> = &mut *lock;

        lock.push(Arc::new(value));
    }
    pub fn dealloc(mut self) {
        std::mem::drop(self.values);
    }
}
