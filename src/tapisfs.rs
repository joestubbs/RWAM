use fuse_mt::{FilesystemMT, FileAttr};
use libc::{ENOENT, ENOSYS};

// helper functions
// fn path_to_fuse_fileattr(_path: &std::path::Path) -> FileAttr {
//     FileAttr { size: (), blocks: (), atime: (), mtime: (), ctime: (), crtime: (), kind: (), perm: (), nlink: (), uid: (), gid: (), rdev: (), flags: () }
// }


/// FUSE File System based on Tapis Files API
pub struct TapisFileSystem {
    pub base_url: String,
    pub system_id: String, 
    pub jwt: String, 
}

impl TapisFileSystem {
    pub fn new(base_url: String, system_id: String, jwt: String) -> TapisFileSystem {
        TapisFileSystem {
            base_url,
            system_id,
            jwt
        }
    }
    
}


impl FilesystemMT for TapisFileSystem {

    fn init(&self, _req: fuse_mt::RequestInfo) -> fuse_mt::ResultEmpty {
        println!("top of init: {:?}", _req);
        Ok(())
    }

    fn getattr(&self, _req: fuse_mt::RequestInfo, _path: &std::path::Path, _fh: Option<u64>) -> fuse_mt::ResultEntry {
        println!("top of getattr; req: {:?}; path: {:?}; fh: {:?}", _req, _path, _fh);
        Err(ENOSYS)
    }

    fn readdir(&self, _req: fuse_mt::RequestInfo, _path: &std::path::Path, _fh: u64) -> fuse_mt::ResultReaddir {
        println!("readdir: {:?}", _path);
        Err(ENOSYS)
    }
}