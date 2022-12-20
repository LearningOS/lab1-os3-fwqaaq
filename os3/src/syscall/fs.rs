const FD_STDOUT: usize = 1;
use core::{slice, str};
// utf8
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let slice = unsafe { slice::from_raw_parts(buf, len) };
            let str = str::from_utf8(slice).unwrap();
            println!("{}", str);
            len as isize
        }
        _ => {
            panic!("Unsupported fd in sys_write");
        }
    }
}