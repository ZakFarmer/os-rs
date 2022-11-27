#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Fail = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        // Write the exit code to isa-debug-exit device port to signal to QEMU to terminate
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
