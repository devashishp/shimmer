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
        println!(
            "[read] fd={fd}, path={}, size={nbytes}",
            file_name.display()
        );
    }

    unsafe fn write(
        &mut self,
        fd: libc::c_int,
        buf: *mut libc::c_void,
        nbytes: libc::size_t,
    ) -> libc::c_int {
        let path_fd = PathBuf::from(format!("/proc/self/fd/{}", fd));
        let file_name = read_link(path_fd).unwrap();
        println!(
            "[write] fd={fd}, path={}, size={nbytes}",
            file_name.display()
        );
    }
}
