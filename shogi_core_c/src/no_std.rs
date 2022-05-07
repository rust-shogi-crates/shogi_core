// unoptimized version of memory-related functions

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    for i in 0..n {
        let value = core::ptr::read(src.add(i));
        core::ptr::write(dest.add(i), value);
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memset(b: *mut u8, c: i32, n: usize) -> *mut u8 {
    for i in 0..n {
        core::ptr::write(b.add(i), c as u8);
    }
    b
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    for i in 0..n {
        let value1 = core::ptr::read(s1.add(i));
        let value2 = core::ptr::read(s2.add(i));
        if value1 != value2 {
            return value1 as i32 - value2 as i32;
        }
    }
    0
}

// Found in https://refspecs.linuxfoundation.org/LSB_1.1.0/gLSB/baselib---bzero-1.html.
// Some targets (cdylib, rustc 1.60, x86_64-apple-darwin, LLVM version 3.9.0svn) might need this symbol.
#[no_mangle]
pub unsafe extern "C" fn __bzero(s: *mut u8, n: usize) {
    // Really want to suppress all optimizations.
    // memset(s, 0, n) seems to be optimized to ___bzero(s, n), which causes an infinite loop.
    for i in 0..n {
        core::ptr::write(s.add(i), 0);
    }
}

// panic-related functions

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

// alloc-related functions

#[cfg(feature = "alloc")]
#[alloc_error_handler]
fn error_handler(_: core::alloc::Layout) -> ! {
    loop {}
}

// SimpleAllocator found in https://doc.rust-lang.org/core/alloc/trait.GlobalAlloc.html.
// Modified so that ARENA_BSS is on .bss section and SimpleAllocator doesn't take much space.
#[cfg(feature = "alloc")]
mod simple_alloc {
    use core::alloc::{GlobalAlloc, Layout};
    use core::cell::UnsafeCell;
    use core::ptr::null_mut;
    use core::sync::atomic::{AtomicUsize, Ordering::SeqCst};
    const ARENA_SIZE: usize = 128 * 1024;
    const MAX_SUPPORTED_ALIGN: usize = 4096;
    struct SimpleAllocator {
        remaining: AtomicUsize, // we allocate from the top, counting down
    }

    static mut ARENA_BSS: UnsafeCell<[u8; ARENA_SIZE]> = UnsafeCell::new([0x00; ARENA_SIZE]);

    #[global_allocator]
    static ALLOCATOR: SimpleAllocator = SimpleAllocator {
        remaining: AtomicUsize::new(ARENA_SIZE),
    };

    unsafe impl Sync for SimpleAllocator {}

    unsafe impl GlobalAlloc for SimpleAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let size = layout.size();
            let align = layout.align();

            // `Layout` contract forbids making a `Layout` with align=0, or align not power of 2.
            // So we can safely use a mask to ensure alignment without worrying about UB.
            let align_mask_to_round_down = !(align - 1);

            if align > MAX_SUPPORTED_ALIGN {
                return null_mut();
            }

            let mut allocated = 0;
            if self
                .remaining
                .fetch_update(SeqCst, SeqCst, |mut remaining| {
                    if size > remaining {
                        return None;
                    }
                    remaining -= size;
                    remaining &= align_mask_to_round_down;
                    allocated = remaining;
                    Some(remaining)
                })
                .is_err()
            {
                return null_mut();
            };
            (ARENA_BSS.get() as *mut u8).add(allocated)
        }
        unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
    }
}
