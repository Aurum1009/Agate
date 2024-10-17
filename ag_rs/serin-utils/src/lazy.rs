use parking_lot::Mutex;
use std::{
    ptr::null_mut,
    sync::atomic::{AtomicPtr, Ordering},
};

// test use case
static EXAMPLE: LazyInit<bool> = LazyInit::new(|| true);

/// A lazy initialization type allowing non-const structures to be used in `const` and `static` variables.
/// Say you want to make a static variable with a type that can't use a `const fn` for its `new()`: alone,
/// you cannot use that type in a `static` or `const` variable because its constructor cannot be evaluated
/// at compile time. Therefore, `LazyInit` allows you to use these types in these types of variables when
/// you need them, and will not be initialized if they are never used.
pub struct LazyInit<T>
where
    T: Sized,
{
    value: AtomicPtr<T>,
    is_init: Mutex<bool>,
    func: fn() -> T,
}
impl<T> LazyInit<T> {
    pub const fn new(func: fn() -> T) -> Self {
        LazyInit::<T> {
            value: AtomicPtr::new(null_mut()),
            is_init: Mutex::new(false),
            func,
        }
    }
    #[inline(always)]
    pub fn init(&self) {
        let mut init_lock = self.is_init.lock();
        if !*init_lock {
            let value = (self.func)();
            let r#box = Box::new(value);
            let ptr = Box::into_raw(r#box);

            self.value.store(ptr, Ordering::SeqCst);
            *init_lock = true;
        }
    }
    pub fn read(&self) -> &'static T {
        {
            let init_lock = self.is_init.lock();
            if !*init_lock {
                drop(init_lock);
                self.init();
            }
        }
        unsafe { &*self.value.load(Ordering::SeqCst) }
    }
}

impl<T> Drop for LazyInit<T> {
    fn drop(&mut self) {
        let ptr = self.value.load(Ordering::SeqCst);

        if !ptr.is_null() {
            unsafe {
                let _ = Box::from_raw(ptr);
            }
        }
    }
}
