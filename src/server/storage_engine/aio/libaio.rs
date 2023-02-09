// Copyright 2022 labring. All rights reserved.
//
// SPDX-License-Identifier: Apache-2.0

use crate::server::storage_engine::block_device::block_storage::BlockStorage;
use libc::{
    c_int, c_uint, close, epoll_create1, epoll_ctl, epoll_event, epoll_wait, eventfd, EFD_NONBLOCK,
    EPOLLET, EPOLLIN, EPOLL_CTL_ADD, FD_CLOEXEC, O_CLOEXEC,
};
use linux::kty::{__u32, aio_context_t, io_event, iocb, k_long, k_uint, timespec, IOCB_FLAG_RESFD};
use linux::syscall::raw::io_getevents;
use linux::syscall::raw::io_setup;
use linux::syscall::raw::io_submit;
use serde_yaml::Value::Null;
use std::collections::VecDeque;
use std::os::unix::io::RawFd;
use std::sync::Arc;
use std::{io, ptr, vec};

struct Aio {
    epoll: Epoll,
}

// impl BlockStorage for Aio {
//     fn write_file(&self, file_name: String, data: &[u8], offset: i64) {
//         //todo index
//         AioContext::new(self.epoll);
//     }
//
    // fn read_file(&self, file_name: String, data: &[u8], offset: i64) {
    //     todo!()
    // }
//
//     fn create_file(&self, file_name: String) {
//         todo!()
//     }
//
//     fn delete_file(&self, file_name: String) {
//         todo!()
//     }
// }

impl Aio {
    fn new() -> Aio {
        Aio {
            epoll: Epoll::new(),
        }
    }
}

pub struct AioContext {

}

impl AioContext {
    pub fn new<E>(epoll: Epoll) {
        //register event to epoll.
        let flags = O_CLOEXEC | EFD_NONBLOCK as i32;
        let eventfd = unsafe { eventfd(0, flags) };
        if eventfd < 0 {
            Err(io::Error::last_os_error())
        }
        Event::create_callback_event(eventfd, epoll);

        let mut context: aio_context_t = 0;
        unsafe {
            if io_setup(32, &mut context) != 0 {
                Err(io::Error::last_os_error())
            }
        };
        let mut request: [*mut iocb; 1] = [&mut state.request as *mut iocb; 1];
        request[0] = iocb {
            aio_data: 0,
            aio_key: 0,
            aio_reserved1: 0,
            aio_lio_opcode: 0,
            aio_reqprio: 0,
            aio_fildes: 0,
            aio_buf: 0,
            aio_nbytes: 0,
            aio_offset: 0,
            aio_reserved2: 0,
            aio_flags: IOCB_FLAG_RESFD as __u32,
            aio_resfd: eventfd.clone() as __u32,
        };
        unsafe {
            io_submit(context, 10, request as *mut *mut iocb);
        }
        let result: [epoll_event; 1] = [epoll_event { events: 0, u64: 0 }];
        let io_events: [io_event; 1] = [io_event {
            data: 0,
            obj: 0,
            res: 0,
            res2: 0,
        }];
        unsafe {
            let num = epoll_wait(epoll.efd.clone(), result as *mut epoll_event, 32, -1);
            io_getevents(
                context.clone(),
                num as k_long,
                num.clone() as k_long,
                io_events as *mut io_event,
                ptr::null_mut(),
            );
        }
    }
}

//linux eventfd.
pub struct EventFd {
    pub fd: RawFd,
}

impl Drop for EventFd {
    fn drop(&self) {
        if self.fd >= 0 {
            unsafe { close(self.fd.clone()) };
            unsafe {}
        }
    }
}

struct Epoll {
    efd: c_int,
}

impl Epoll {
    fn new() -> Epoll {
        let efd = unsafe { epoll_create1(FD_CLOEXEC) };
        if efd == -1 {
            Err(io::Error::last_os_error())
        }
        Epoll { efd }
    }

    fn register_event(&self, fd: c_int) {
        let mut epoll_event = epoll_event {
            events: EPOLLIN | EPOLLET as u32,
            u64: 0,
        };
        let result = unsafe {
            epoll_ctl(self.efd.clone(), EPOLL_CTL_ADD, fd, &mut epoll_event);
        };
        if result == -1 {
            Err(io::Error::last_os_error())
        }
    }
}

pub struct Event {}

impl Event {
    fn create_callback_event(eventfd: c_int, epoll: Epoll) {
        epoll.register_event(eventfd)
    }
}

// //todo suit for multi-thread
// struct Buffer {
//     write_buffer: VecDeque,
// }
