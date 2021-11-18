use libc::ENOENT;
use std::path::Path;

use time::Timespec;

use fuse_mt::{
    FileAttr, FileType, FilesystemMT, RequestInfo, ResultEmpty, ResultEntry, ResultOpen,
    ResultReaddir, ResultStatfs, Statfs,
};

#[macro_use]
extern crate log;

pub struct SeafileFS {}

const TTL: Timespec = Timespec { sec: 1, nsec: 0 };

impl SeafileFS {
    pub fn new(_server: &str, _username: &str, _password: &str) -> SeafileFS {
        SeafileFS {}
    }

    fn fileattr(req: RequestInfo, kind: FileType, perm: u16, size: u64, mtime: u64) -> FileAttr {
        FileAttr {
            size,
            blocks: 100u64,
            atime: Timespec {
                sec: mtime as i64,
                nsec: 0i32,
            },
            mtime: Timespec {
                sec: mtime as i64,
                nsec: 0i32,
            },
            ctime: Timespec {
                sec: mtime as i64,
                nsec: 0i32,
            },
            crtime: Timespec {
                sec: mtime as i64,
                nsec: 0,
            },
            kind,
            perm,
            nlink: 0u32,
            uid: req.uid,
            gid: req.gid,
            rdev: 0u32,
            flags: 0,
        }
    }
}

impl FilesystemMT for SeafileFS {
    fn init(&self, _req: RequestInfo) -> ResultEmpty {
        debug!("init");
        Ok(())
    }

    fn destroy(&self, _req: RequestInfo) {
        debug!("destroy");
    }

    fn statfs(&self, _req: RequestInfo, path: &Path) -> ResultStatfs {
        debug!("statfs: {:?}", path);

        Ok(Statfs {
            blocks: 100u64,
            bfree: 100u64,
            bavail: 0u64,
            files: 100u64,
            ffree: 100u64,
            bsize: 100u32,
            namelen: 255u32,
            frsize: 100u32,
        })
    }

    fn opendir(&self, _req: RequestInfo, path: &Path, _flags: u32) -> ResultOpen {
        debug!("opendir: {:?} (flags = {:#o})", path, _flags);
        match path.parent() {
            None => Ok((0, 0)),
            Some(_) => Ok((0, 0)),
        }
    }

    fn readdir(&self, _req: RequestInfo, path: &Path, _fh: u64) -> ResultReaddir {
        debug!("readdir: {:?}", path);

        Ok(Vec::new())
    }

    fn getattr(&self, req: RequestInfo, path: &Path, _fh: Option<u64>) -> ResultEntry {
        debug!("getattr: {:?}", path);
        let components = path.components().collect::<Vec<_>>();
        debug!("getattr: {:?}", components);

        match components.len() {
            1 => Ok((
                TTL,
                SeafileFS::fileattr(req, FileType::Directory, 0o755, 0, 0),
            )),
            _ => Err(ENOENT),
        }
    }
}
