mod vmprotect;

// = "Correct definition\0"
const MARKER_NAME: [i8; 19] = [
    67i8, 111i8, 114i8, 114i8, 101i8, 99i8, 116i8, 32i8, 100i8, 101i8, 102i8, 105i8, 110i8, 105i8,
    116i8, 105i8, 111i8, 110i8, 0i8,
];

fn main() {
    unsafe {
        vmprotect::internal::VMProtectBegin(
            std::ffi::CString::new("Incorrect definition")
                .unwrap()
                .as_ptr(),
        )
    };
    println!("Hello, world! #1");
    unsafe { crate::vmprotect::internal::VMProtectEnd() };
    unsafe { vmprotect::internal::VMProtectBegin(&MARKER_NAME as *const i8) };
    println!("Hello, world! #2");
    unsafe { crate::vmprotect::internal::VMProtectEnd() };
}
