use shimmer::{shimmer, shimmer_hook};
use std::fs::read_link;
use std::path::PathBuf;

#[shimmer]
#[derive(Default)]
struct State {}

trait BasicIO {
    unsafe fn read(
        &mut self,
        fd: libc::c_int,
        buf: *mut libc::c_void,
        nbytes: libc::size_t,
    ) -> libc::c_int;

    unsafe fn write(
        &mut self,
        fd: libc::c_int,
        buf: *mut libc::c_void,
        nbytes: libc::size_t,
    ) -> libc::c_int;
}

#[shimmer_hook]
impl BasicIO for State {
    unsafe fn read(
        &mut self,
        fd: libc::c_int,
        buf: *mut libc::c_void,
        nbytes: libc::size_t,
    ) -> libc::c_int {
        let path_fd = PathBuf::from(format!("/proc/self/fd/{}", fd));
        let file_name = read_link(path_fd).unwrap();
        let msg = format!(
            "[read], fd={fd}, path={} size={nbytes}\n",
            file_name.display()
        );
        let _x = unsafe {
            libc::syscall(
                libc::SYS_write,
                0,
                msg.as_bytes().as_ptr() as usize,
                msg.len(),
            )
        };
        //        println!("[read] fd={fd}, size={nbytes}");
    }

    unsafe fn write(
        &mut self,
        fd: libc::c_int,
        buf: *mut libc::c_void,
        nbytes: libc::size_t,
    ) -> libc::c_int {
        // Works!!
        let path_fd = PathBuf::from(format!("/proc/self/fd/{}", fd));
        let file_name = read_link(path_fd).unwrap();
        let msg = format!(
            "[write], fd={fd}, path={} size={nbytes}\n",
            file_name.display()
        );
        let _x = unsafe {
            libc::syscall(
                libc::SYS_write,
                0,
                msg.as_bytes().as_ptr() as usize,
                msg.len(),
            )
        };
        // Doesn't !!
        //      println!("[write] fd={fd}, size={nbytes}");
    }
}
