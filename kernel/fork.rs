// fork.rs — Rotinas auxiliares corrigidas para 'fork'

use core::ptr;

// Constantes do sistema
const NR_TASKS: usize = 64;
const NR_OPEN: usize = 20;
const PAGE_SIZE: usize = 4096;
const EAGAIN: i32 = 11;
const ENOMEM: i32 = 12;

// --- Tipos e Estruturas ---

#[repr(C)]
pub struct Tss {
    pub back_link: u32,
    pub esp0: u32,
    pub ss0: u16,
    _pad0: u16,
    pub eip: u32,
    pub eflags: u32,
    pub eax: u32,
    pub ecx: u32,
    pub edx: u32,
    pub ebx: u32,
    pub esp: u32,
    pub ebp: u32,
    pub esi: u32,
    pub edi: u32,
    pub es: u16,
    _pad1: u16,
    pub cs: u16,
    _pad2: u16,
    pub ss: u16,
    _pad3: u16,
    pub ds: u16,
    _pad4: u16,
    pub fs: u16,
    _pad5: u16,
    pub gs: u16,
    _pad6: u16,
    pub ldt: u32,
    pub trace_bitmap: u32,
    pub i387: [u8; 108],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct LdtEntry {
    pub limit: u16,
    pub base_low: u16,
    pub base_mid: u8,
    pub access: u8,
    pub granularity: u8,
    pub base_high: u8,
}

#[repr(C)]
pub struct File { pub f_count: u32 }

#[repr(C)]
pub struct Inode { pub i_count: u32 }

#[repr(C)]
pub struct TaskStruct {
    pub state: i32,
    pub pid: i32,
    pub father: i32,
    pub counter: i32,
    pub priority: i32,
    pub signal: u32,
    pub alarm: u32,
    pub leader: i32,
    pub utime: u64,
    pub stime: u64,
    pub cutime: u64,
    pub cstime: u64,
    pub start_time: u64,
    pub tss: Tss,
    pub ldt: [LdtEntry; 3],
    pub filp: [*mut File; NR_OPEN],
    pub pwd: *mut Inode,
    pub root: *mut Inode,
}

// --- Estado Global ---

static mut LAST_PID: i32 = 0;
static mut TASK: [*mut TaskStruct; NR_TASKS] = [ptr::null_mut(); NR_TASKS];
static mut JIFFIES: u64 = 0;

extern "C" {
    fn write_verify(address: u32);
    fn get_free_page() -> *mut TaskStruct;
    fn free_page(addr: usize);
    fn copy_page_tables(old_base: u32, new_base: u32, limit: u32) -> i32;
    fn free_page_tables(base: u32, limit: u32) -> i32;
    fn get_base(ldt: &LdtEntry) -> u32;
    fn get_limit(selector: u16) -> u32;
    fn set_base(ldt: &mut LdtEntry, base: u32);
    fn set_tss_desc(gdt_entry: *mut u8, tss: &Tss);
    fn set_ldt_desc(gdt_entry: *mut u8, ldt: &[LdtEntry; 3]);
    fn panic(msg: *const u8) -> !;
    static mut last_task_used_math: *mut TaskStruct;
    static mut current: *mut TaskStruct;
    static mut gdt: *mut u8;
}

const FIRST_TSS_ENTRY: usize = 4;
const FIRST_LDT_ENTRY: usize = 5;

// --- Implementações Corrigidas ---

pub unsafe fn copy_mem(nr: i32, p: *mut TaskStruct) -> Result<(), i32> {
    let code_limit = get_limit(0x0f);
    let data_limit = get_limit(0x17);
    let old_code_base = get_base(&(*current).ldt[1]);
    let old_data_base = get_base(&(*current).ldt[2]);

    if old_data_base != old_code_base {
        panic(b"Segmentos separados nao suportados\0".as_ptr());
    }

    let new_base = (nr as u32) * 0x0400_0000;
    set_base(&mut (*p).ldt[1], new_base);
    set_base(&mut (*p).ldt[2], new_base);

    if copy_page_tables(old_data_base, new_base, data_limit) != 0 {
        free_page_tables(new_base, data_limit);
        return Err(ENOMEM);
    }
    Ok(())
}

pub unsafe fn copy_process(
    nr: i32, ebp: u32, edi: u32, esi: u32, gs: u32,
    _none: u32, ebx: u32, ecx: u32, edx: u32,
    fs: u32, es: u32, ds: u32,
    eip: u32, cs: u32, eflags: u32, esp: u32, ss: u32,
) -> i32 {
    let p = get_free_page();
    if p.is_null() { return -EAGAIN; }

    // Copia a estrutura original
    ptr::copy_nonoverlapping(current, p, 1);

    let task = &mut *p;
    task.state = 0; // TASK_RUNNING
    task.pid = LAST_PID;
    task.father = (*current).pid;
    task.counter = task.priority;
    task.signal = 0;
    task.alarm = 0;
    task.leader = 0;
    task.utime = 0;
    task.stime = 0;
    task.start_time = JIFFIES;

    // Configuração do TSS
    task.tss.esp0 = (p as usize + PAGE_SIZE) as u32;
    task.tss.ss0 = 0x10;
    task.tss.eip = eip;
    task.tss.eflags = eflags;
    task.tss.eax = 0; // Retorno do filho é 0
    task.tss.ecx = ecx;
    task.tss.edx = edx;
    task.tss.ebx = ebx;
    task.tss.esp = esp;
    task.tss.ebp = ebp;
    task.tss.esi = esi;
    task.tss.edi = edi;
    task.tss.es = (es & 0xffff) as u16;
    task.tss.cs = (cs & 0xffff) as u16;
    task.tss.ss = (ss & 0xffff) as u16;
    task.tss.ds = (ds & 0xffff) as u16;
    task.tss.fs = (fs & 0xffff) as u16;
    task.tss.gs = (gs & 0xffff) as u16;
    task.tss.ldt = ldt_selector(nr);

    if last_task_used_math == current {
        core::arch::asm!("fnsave [{}]", in(reg) &mut task.tss.i387);
    }

    if let Err(_) = copy_mem(nr, p) {
        TASK[nr as usize] = ptr::null_mut();
        free_page(p as usize);
        return -ENOMEM;
    }

    // Incrementar referências
    for f in task.filp.iter() {
        if !f.is_null() { (**f).f_count += 1; }
    }
    if !task.pwd.is_null() { (*task.pwd).i_count += 1; }
    if !task.root.is_null() { (*task.root).i_count += 1; }

    set_tss_desc(gdt.add((nr as usize * 2 + FIRST_TSS_ENTRY) * 8), &task.tss);
    set_ldt_desc(gdt.add((nr as usize * 2 + FIRST_LDT_ENTRY) * 8), &task.ldt);
    
    TASK[nr as usize] = p;
    LAST_PID
}

pub unsafe fn find_empty_process() -> i32 {
    loop {
        LAST_PID += 1;
        if LAST_PID < 0 { LAST_PID = 1; }
        let mut in_use = false;
        for i in 0..NR_TASKS {
            if !TASK[i].is_null() && (*TASK[i]).pid == LAST_PID {
                in_use = true;
                break;
            }
        }
        if !in_use { break; }
    }
    for i in 1..NR_TASKS {
        if TASK[i].is_null() { return i as i32; }
    }
    -EAGAIN
}

#[inline]
fn ldt_selector(nr: i32) -> u32 {
