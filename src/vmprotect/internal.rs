use std::os::raw::c_char;

extern "C" {
	pub fn VMProtectBegin(name: *const c_char);
	pub fn VMProtectBeginVirtualization(name: *const c_char);
	pub fn VMProtectBeginMutation(name: *const c_char);
	pub fn VMProtectBeginUltra(name: *const c_char);
	pub fn VMProtectBeginVirtualizationLockByKey(name: *const c_char);
	pub fn VMProtectBeginUltraLockByKey(name: *const c_char);
	pub fn VMProtectEnd();
	pub fn VMProtectIsProtected() -> u8;
	pub fn VMProtectIsDebuggerPresent(kernel: u8) -> u8;
	pub fn VMProtectIsVirtualMachinePresent() -> u8;
	pub fn VMProtectIsValidImageCRC() -> u8;
	pub fn VMProtectDecryptStringA(value: *const c_char) -> *const c_char;
	pub fn VMProtectDecryptStringW(value: *const u16) -> *const u16;
	pub fn VMProtectFreeString(value: *const c_char) -> u8;
}
