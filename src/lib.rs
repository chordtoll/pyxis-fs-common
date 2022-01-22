use std::time::SystemTime;

use serde::{Deserialize, Serialize};

const SIZE_LIMIT: usize = 1024*1024;    // Individual packets should not be larger than 1MB

pub async fn transact(conn: &mut quinn::Connection, req: &Request) -> Response {
    let (mut sender, recver) = conn.open_bi().await.unwrap();
    send_req(&mut sender, req).await;
    recv_rsp(recver).await
}

pub async fn send_req(sock: &mut quinn::SendStream, req: &Request) {
    let buf = bincode::serialize(req).unwrap();
    sock.write_all(&buf).await.unwrap();
    sock.finish().await.unwrap();
}

pub async fn recv_req(sock: quinn::RecvStream) -> Request {
    let buf = sock.read_to_end(SIZE_LIMIT).await.unwrap();
    bincode::deserialize(&buf).unwrap()
}
pub async fn send_rsp(sock: &mut quinn::SendStream, rsp: &Response) {
    let buf = bincode::serialize(rsp).unwrap();
    sock.write_all(&buf).await.unwrap();
    sock.finish().await.unwrap();
}

pub async fn recv_rsp(sock: quinn::RecvStream) -> Response {
    let buf = sock.read_to_end(SIZE_LIMIT).await.unwrap();
    bincode::deserialize(&buf).unwrap()
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Request {
    Mount(String),
    Read(u64, i64, u32),
    ReadDir(u64, i64),
    Lookup(u64, String),
    Getattr(u64),
    Readlink(u64),
    Mknod(u64, String, u32, u32, u32),
    Mkdir(u64, String, u32, u32),
    Write(u64, i64, Vec<u8>),
    Unlink(u64, String),
    Rename(u64, String, u64, String, u32),
    Setattr(
        u64,
        Option<u32>,
        Option<u32>,
        Option<u32>,
        Option<u64>,
        Option<std::time::SystemTime>,
        Option<std::time::SystemTime>,
        Option<std::time::SystemTime>,
        Option<u64>,
        Option<std::time::SystemTime>,
        Option<std::time::SystemTime>,
        Option<std::time::SystemTime>,
        Option<u32>,
    ),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Response {
    Mount(u64),
    Read(Vec<u8>),
    ReadDir(Vec<(u64, FileType, String)>),
    Lookup(FileAttr),
    Getattr(FileAttr),
    Readlink(Vec<u8>),
    Mknod(FileAttr),
    Mkdir(FileAttr),
    Write(u32),
    Unlink,
    Rename,
    Setattr(FileAttr),
    Error(libc::c_int),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum FileType {
    NamedPipe,
    CharDevice,
    BlockDevice,
    Directory,
    RegularFile,
    Symlink,
    Socket,
    Whiteout,
}

impl From<FileType> for fuser::FileType {
    fn from(i: FileType) -> Self {
        match i {
            FileType::RegularFile => Self::RegularFile,
            FileType::Directory => Self::Directory,
            FileType::Symlink => Self::Symlink,
            FileType::BlockDevice => Self::BlockDevice,
            FileType::CharDevice => Self::CharDevice,
            FileType::Socket => Self::Socket,
            FileType::NamedPipe => Self::NamedPipe,
            FileType::Whiteout => panic!("Whiteout device should never be transmitted"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct FileAttr {
    pub ino:     u64,
    pub size:    u64,
    pub blocks:  u64,
    pub atime:   SystemTime,
    pub mtime:   SystemTime,
    pub ctime:   SystemTime,
    pub crtime:  SystemTime,
    pub kind:    FileType,
    pub perm:    u16,
    pub nlink:   u32,
    pub uid:     u32,
    pub gid:     u32,
    pub rdev:    u32,
    pub blksize: u32,
    pub flags:   u32,
}

impl From<FileAttr> for fuser::FileAttr {
    fn from(i: FileAttr) -> Self {
        Self {
            ino:     i.ino,
            size:    i.size,
            blocks:  i.blocks,
            atime:   i.atime,
            mtime:   i.mtime,
            ctime:   i.ctime,
            crtime:  i.crtime,
            kind:    i.kind.into(),
            perm:    i.perm,
            nlink:   i.nlink,
            uid:     i.uid,
            gid:     i.gid,
            rdev:    i.rdev,
            blksize: i.blksize,
            flags:   i.flags,
        }
    }
}

#[cfg(feature = "parcel")]
pub mod parcel;
