use std::{
    io::{self, Result},
    net::TcpStream,
    os::fd::AsRawFd,
};

use crate::ffi;

type Events = Vec<ffi::Event>;

pub struct Poll {
    registry: Registry,
}

pub struct Registry {
    raw_fd: i32,
}

impl Poll {
    pub fn new() -> Result<Self> {
        unimplemented!()
    }

    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    pub fn poll(&mut self, events: &mut Events, timeout: Option<i32>) -> Result<()> {
        unimplemented!()
    }
}

impl Registry {
    pub fn register(&self, source: &TcpStream, token: usize, interests: i32) -> Result<()> {
        unimplemented!()
    }
}

impl Drop for Registry {
    fn drop(&mut self) {
        unimplemented!()
    }
}
