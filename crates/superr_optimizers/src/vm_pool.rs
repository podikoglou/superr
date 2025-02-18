use std::{
    collections::VecDeque,
    mem,
    sync::{Arc, Mutex},
};

use superr_vm::vm::VM;

// A thread-safe pool of VMs
pub struct VMPool {
    vms: Arc<Mutex<VecDeque<VM>>>,
}

impl VMPool {
    pub fn new(capacity: usize) -> Self {
        let mut vms = VecDeque::with_capacity(capacity);

        for _ in 0..capacity {
            vms.push_back(VM::default());
            dbg!(vms.len());
        }

        Self {
            vms: Arc::new(Mutex::new(vms)),
        }
    }

    // Get a VM from the pool, or create a new one if none are available
    pub fn get(&self) -> VMHandle {
        let mut pool = self.vms.lock().unwrap();
        let vm = pool.pop_front().unwrap_or_else(|| VM::default());

        VMHandle {
            vm,
            pool: Arc::clone(&self.vms),
        }
    }
}

// RAII handle for a VM that returns it to the pool when dropped
pub struct VMHandle {
    vm: VM,
    pool: Arc<Mutex<VecDeque<VM>>>,
}

impl VMHandle {
    pub fn get_mut(&mut self) -> &mut VM {
        &mut self.vm
    }
}

impl Drop for VMHandle {
    fn drop(&mut self) {
        self.vm.reset(); // Reset the VM state before returning to pool
        let mut pool = self.pool.lock().unwrap();
        pool.push_back(mem::replace(&mut self.vm, VM::default()));
    }
}
