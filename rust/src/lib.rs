/// cbindgen:ignore
#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
mod bindings;
use bindings::*;
mod rask_action;

pub struct RArmSimKernel {
    host: *const ask_host_services,
    flags: ask_config,
    stats: ask_stats,
    regs: [u32; 16],
    cpsr: u32,
    stop: bool,
    // lastpc: u32,
}

type OwningHandle = Option<Box<RArmSimKernel>>;
type BorrowHandle = *const RArmSimKernel;
type BorrowMutHandle = *mut RArmSimKernel;

impl RArmSimKernel {
    fn new(hostp: *const ask_host_services) -> RArmSimKernel {
        let rask = RArmSimKernel {
            host: hostp,
            flags: ask_config_ac_nothing,
            stats: ask_stats {
                instructions: 0,
                loads: 0,
                stores: 0,
                load_misses: 0,
                store_misses: 0,
            },
            regs: [0; 16],
            cpsr: 0,
            stop: false,
            // lastpc: 0,
        };
        rask.host_log("CPU initialized");
        rask
    }

    fn host_load(&self, address: u32) -> u32 {
        unsafe { (*self.host).mem_load.unwrap()(address) }
    }

    fn host_store(&self, address: u32, value: u32) {
        unsafe {
            (*self.host).mem_store.unwrap()(address, value);
        }
    }

    fn host_log(&self, msg: &str) {
        let cmsg = std::ffi::CString::new(msg).expect("CString::new failed");
        unsafe {
            (*self.host).log_msg.unwrap()(cmsg.as_ptr());
        }
    }

    fn host_trace(
        &self,
        step: std::ffi::c_uint,
        pc: word,
        cpsr: word,
        r0: word,
        r1: word,
        r2: word,
        r3: word,
        r4: word,
        r5: word,
        r6: word,
        r7: word,
        r8: word,
        r9: word,
        r10: word,
        r11: word,
        r12: word,
        sp: word,
        lr: word,
    ) {
        unsafe {
            (*self.host).log_trace.unwrap()(
                step, pc, cpsr, r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, sp, lr,
            );
        }
    }

    fn host_panic(&self, msg: &str) {
        let cmsg = std::ffi::CString::new(msg).expect("CString::new failed");
        unsafe {
            (*self.host).panic.unwrap()(cmsg.as_ptr());
        }
    }

    fn cpu_run(&mut self, cycles: i32) -> i32 {
        // println!("1,2,4 and 6 working");
        let mut i: i32 = 0;
        let mut flag: i32;
        // self.regs[15] = self.regs[15] + 8; // necessary for pc relative addressing
        if cycles != 0 {
            while i < cycles {
                self.stats.instructions += 1 as u32;
                let instruction = self.host_load(self.regs[15]);
                let lastpc = self.regs[15];
                flag = rask_action::decode(instruction, self);

                i = i + 1;
                self.host_trace(
                    i as u32,
                    lastpc,
                    self.cpsr,
                    self.regs[0],
                    self.regs[1],
                    self.regs[2],
                    self.regs[3],
                    self.regs[4],
                    self.regs[5],
                    self.regs[6],
                    self.regs[7],
                    self.regs[8],
                    self.regs[9],
                    self.regs[10],
                    self.regs[11],
                    self.regs[12],
                    self.regs[13],
                    self.regs[14],
                );
                if self.stop {
                    break;
                }
                if flag == 2 {
                    break;
                }
                self.regs[15] = self.regs[15] + 4;
            }
        } else {
            loop {
                self.stats.instructions += 1 as u32;
                let instruction = self.host_load(self.regs[15]);

                let lastpc = self.regs[15];

                flag = rask_action::decode(instruction, self);
                i = i + 1;
                self.host_trace(
                    i as u32,
                    lastpc,
                    self.cpsr,
                    self.regs[0],
                    self.regs[1],
                    self.regs[2],
                    self.regs[3],
                    self.regs[4],
                    self.regs[5],
                    self.regs[6],
                    self.regs[7],
                    self.regs[8],
                    self.regs[9],
                    self.regs[10],
                    self.regs[11],
                    self.regs[12],
                    self.regs[13],
                    self.regs[14],
                );

                if self.stop {
                    break;
                }
                if flag == 2 {
                    break;
                }
                self.regs[15] = self.regs[15] + 4;
            }
        }

        self.stop = false;
        return i;
    }
}

#[no_mangle]
pub extern "C" fn rask_init(hostp: *const ask_host_services_t) -> OwningHandle {
    Some(Box::new(RArmSimKernel::new(hostp)))
}

#[no_mangle]
pub extern "C" fn rask_fini(_h: OwningHandle) {
    // silently consume/drop the owning handle (avoid leaks on re-ask_init)
}

#[no_mangle]
pub extern "C" fn rask_config_get(h: BorrowHandle) -> ask_config_t {
    unsafe {
        let k = &*h;
        k.flags
    }
}

#[no_mangle]
pub extern "C" fn rask_config_set(h: BorrowMutHandle, flags: ask_config_t) {
    unsafe {
        let k = &mut *h;
        k.flags = flags;
    }
}

#[no_mangle]
pub extern "C" fn rask_stats_report(h: BorrowHandle, output: *mut ask_stat_t) {
    unsafe {
        let k = &*h;
        *output = k.stats;
    }
}

#[no_mangle]
pub extern "C" fn rask_reg_get(h: BorrowHandle, _bank: ask_mode_t, index: i32) -> u32 {
    unsafe {
        let k = &*h;
        k.regs[index as usize]
    }
}

#[no_mangle]
pub extern "C" fn rask_reg_set(h: BorrowMutHandle, _bank: ask_mode_t, index: i32, value: u32) {
    unsafe {
        let k = &mut *h;
        k.regs[index as usize] = value;
    }
}

#[no_mangle]
pub extern "C" fn rask_cpsr_get(h: BorrowHandle) -> u32 {
    unsafe {
        let k = &*h;
        k.cpsr
    }
}

#[no_mangle]
pub extern "C" fn rask_cpsr_set(h: BorrowMutHandle, value: u32) {
    unsafe {
        let k = &mut *h;
        k.cpsr = value;
    }
}

#[no_mangle]
pub extern "C" fn rask_cpu_running(_h: BorrowHandle) -> i32 {
    // TODO
    0
}

#[no_mangle]
pub extern "C" fn rask_cpu_signal(_h: BorrowMutHandle, _signal: ask_signal_t) {
    // TODO
}

#[no_mangle]
pub extern "C" fn rask_cpu_run(h: BorrowMutHandle, cycles: i32) -> i32 {
    unsafe {
        let k = &mut *h;
        k.cpu_run(cycles)
    }
}
