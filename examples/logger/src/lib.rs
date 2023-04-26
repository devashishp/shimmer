use log::info;
use log4rs;
use serde_yaml;
use shimmer::prelude::*;

#[shimmer]
struct State {
    logger: Result<bool, bool>,
}
impl Default for State {
    fn default() -> Self {
        Self { logger: Err(true) }
    }
}
impl State {
    fn init_logger(&mut self) {
        if !self.logger.is_ok() {
            let _logger =
                log4rs::init_raw_config(serde_yaml::from_str(include_str!("log4rs.yml")).unwrap());
        }
    }
}

trait BasicIO {
    unsafe fn write(
        &mut self,
        fd: libc::c_int,
        buf: *mut libc::c_void,
        nbytes: libc::size_t,
    ) -> libc::c_int;
    unsafe fn read(
        &mut self,
        fd: libc::c_int,
        buf: *mut libc::c_void,
        nbytes: libc::size_t,
    ) -> libc::c_int;
}

#[shimmer_hook]
impl BasicIO for State {
    unsafe fn write(
        &mut self,
        fd: libc::c_int,
        buf: *mut libc::c_void,
        nbytes: libc::size_t,
    ) -> libc::c_int {
        self.init_logger();
        info!("[write] fd={fd}, size={nbytes}");
    }
    unsafe fn read(
        &mut self,
        fd: libc::c_int,
        buf: *mut libc::c_void,
        nbytes: libc::size_t,
    ) -> libc::c_int {
        self.init_logger();
        info!("[read] fd={fd}, size={nbytes}");
    }
}
