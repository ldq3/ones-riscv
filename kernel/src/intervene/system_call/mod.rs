use ones::intervene::system_call::{ Lib, Dependence };

pub struct Handler;

impl Dependence for Handler {
    fn current_user_token() -> usize {
        0
    }
}

impl Lib for Handler {}

// pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
//     match syscall_id {
//         config::WRITE => write(args[0], args[1] as *const u8, args[2]),
//         config::EXIT => exit(args[0] as i32),
//         _ => panic!("Unsupported syscall_id: {}", syscall_id),
//     }
// }

// fn write(fd: usize, buf: *const u8, len: usize) -> isize {
//     match fd {
//         config::FD_STDOUT => {
//             let slice = unsafe {
//                 core::slice::from_raw_parts(buf, len)
//             };
//             let str = core::str::from_utf8(slice).unwrap();
//             print!("{}", str);
//             len as isize
//         },
//         _ => {
//             panic!("Unsupported fd in sys_write!");
//         }
//     }
// }

// fn exit(xstate: i32) -> ! {
//     println!("[kernel] Application exited with code {}", xstate);
//     panic!("process exit");
// }

// mod config {
//     pub const WRITE: usize = 64;
//     pub const EXIT: usize = 93;

//     // write
//     pub const FD_STDOUT: usize = 1;
// }