use crate::sbi::shutdown;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // lang_items不能直接用println!，直接调用底层print接口，绕过宏
    let msg = info.message().as_str().unwrap_or("(no message)");
    if let Some(loc) = info.location() {
        crate::print!("Panicked at {}:{} {}\n", loc.file(), loc.line(), msg);
    } else {
        crate::print!("Panicked: {}\n", msg);
    }
    shutdown()
}
