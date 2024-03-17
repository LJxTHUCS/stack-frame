use x86::bits64::registers::rbp;

const STACKFRAME_DEPTH: usize = 7;

fn print_stackframe() {
    let mut fp = rbp();
    let mut depth = 0;
    while fp != 0 && depth < STACKFRAME_DEPTH {
        println!("In stack frame: fp = {:#x}", fp);
        let frame_base = fp as *const u64;
        // old fp saved in `frame_base`
        let old_fp = unsafe { *frame_base };
        // ra saved in `frame_base + size_of::<u64>()`
        let ra = unsafe { *(frame_base.add(1)) };
        println!("    ra = {:#x}", ra);
        backtrace::resolve(ra as _, |s| {
            println!("    From caller: {:?}", s.name());
        });
        // backtrace to last frame
        fp = old_fp;
        depth += 1;
    }
}

fn main() {
    func1_inlined();
}

#[inline(always)]
fn func1_inlined() {
    func2();
}

fn func2() {
    func3();
}

fn func3() {
    print_stackframe();
}
