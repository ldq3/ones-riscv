use ones::system_call::{ Lib as L, Hal };

pub struct Lib;

impl Hal for Lib {
    fn write(_fd: usize, _buf: *const u8, _len: usize) -> isize {
        0
        // let token = Self::current_user_token();
        // let task = current_task().unwrap();
        // let inner = task.inner_exclusive_access();
        // if fd >= inner.fd_table.len() {
        //     return -1;
        // }
        // if let Some(file) = &inner.fd_table[fd] {
        //     let file = file.clone();
        //     // release current task TCB manually to avoid multi-borrow
        //     drop(inner);
        //     file.write(
        //         UserBuffer::new(translated_byte_buffer(token, buf, len))
        //     ) as isize
        // } else {
        //     -1
        // }
    }
}

impl L for Lib { }