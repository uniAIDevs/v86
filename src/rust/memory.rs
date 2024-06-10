use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;

pub struct OptimizedMemory {
    ptr: NonNull<u8>,
    layout: Layout,
}

impl OptimizedMemory {
    pub fn new(size: usize) -> Self {
        let layout = Layout::from_size_align(size, std::mem::align_of::<u8>()).unwrap();
        let ptr = unsafe { alloc(layout) };
        let ptr = NonNull::new(ptr).expect("Memory allocation failed");
        Self { ptr, layout }
    }

    pub fn optimized_memory_allocation(size: usize) -> Self {
        Self::new(size)
    }

    pub fn optimized_memory_read_write(&self, offset: usize, data: &[u8]) {
        assert!(offset + data.len() <= self.layout.size(), "Out of bounds memory access");
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), self.ptr.as_ptr().add(offset), data.len());
        }
    }

    pub fn read(&self, offset: usize, buffer: &mut [u8]) {
        assert!(offset + buffer.len() <= self.layout.size(), "Out of bounds memory access");
        unsafe {
            std::ptr::copy_nonoverlapping(self.ptr.as_ptr().add(offset), buffer.as_mut_ptr(), buffer.len());
        }
    }
}

impl Drop for OptimizedMemory {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.ptr.as_ptr(), self.layout);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimized_memory_allocation() {
        let mem = OptimizedMemory::optimized_memory_allocation(1024);
        assert_eq!(mem.layout.size(), 1024);
    }

    #[test]
    fn test_optimized_memory_read_write() {
        let mem = OptimizedMemory::optimized_memory_allocation(1024);
        let data = vec![1, 2, 3, 4];
        mem.optimized_memory_read_write(0, &data);

        let mut buffer = vec![0; 4];
        mem.read(0, &mut buffer);
        assert_eq!(buffer, data);
    }
}