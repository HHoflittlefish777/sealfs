// Copyright 2022 labring. All rights reserved.
//
// SPDX-License-Identifier: Apache-2.0

use libc::ioctl;
use nix::fcntl::{open, OFlag};

use crate::server::EngineError;

const _BLOCKGETSIZE: u64 = 0x1260;

pub trait BlockStorage {
    /**
     * Default aio.
     */
    fn write_file(&self, file_name: String, data: &[u8], offset: i64);

    /**
     * Default aio.  
     */
    fn read_file(&self, file_name: String, data: &[u8], offset: i64);

    fn create_file(&self, file_name: String);

    fn delete_file(&self, file_name: String);
}
struct _BlockDevice {
    block_num: u64,
}

impl _BlockDevice {
    fn _new(path: String) -> Result<_BlockDevice, EngineError> {
        let block_num = Self::_get_block_info(path)?;
        Ok(_BlockDevice { block_num })
    }

    fn _get_block_info(path: String) -> Result<u64, EngineError> {
        let fd = open(
            path.as_str(),
            OFlag::O_DIRECT,
            nix::sys::stat::Mode::S_IRWXU,
        );
        if fd? < 0 {
            return Err(EngineError::Exist);
        }
        let block_num = 0;
        unsafe {
            let result = ioctl(fd?, _BLOCKGETSIZE, &block_num);
            if result < 0 {
                return Err(EngineError::BlockInfo);
            }
        }
        Ok(block_num)
    }
}

#[cfg(test)]
mod tests {
    // use super::BlockDevice;

    // #[test]
    // fn block_info_test() {
    //     let block_num = BlockDevice::get_block_info("/dev/sda1".to_string());
    //     println!("{:?}",block_num.unwrap());
    // }
}
