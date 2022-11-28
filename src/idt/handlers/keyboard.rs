use lazy_static::lazy_static;
use pc_keyboard::KeyCode;
use x86_64::structures::idt::InterruptStackFrame;

use crate::{print, idt::{InterruptIndex, PICS}, qemu};

pub extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: InterruptStackFrame,
) {
    use pc_keyboard::{Keyboard, layouts, ScancodeSet1, HandleControl, DecodedKey};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Uk105Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(layouts::Uk105Key, ScancodeSet1, HandleControl::Ignore));
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::RawKey(KeyCode::Escape) => qemu::exit_qemu(qemu::QemuExitCode::Success),
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}