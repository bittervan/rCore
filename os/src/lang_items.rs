use crate::sbi::shutdown;
use core::panic::PanicInfo;
use crate::println;

#[panic_handler]
fn my_panic(info: &PanicInfo) -> ! {
	if let Some(location) = info.location() {
		println!(
			"Panic at {}:{} {}",
			location.file(),
			location.line(),
			info.message().unwrap()
		);
	} else {
		println!("Panic: {}", info.message().unwrap());
	}
	shutdown()
}
