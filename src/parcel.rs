use pyxis_parcel::InodeKind;
use crate::FileType;
use crate::FileAttr;

impl From<pyxis_parcel::FileAttr> for FileAttr {
    fn from(i: pyxis_parcel::FileAttr) -> Self {
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

impl From<InodeKind> for FileType {
    fn from(i: InodeKind) -> Self {
        match i {
            InodeKind::RegularFile => Self::RegularFile,
            InodeKind::Directory => Self::Directory,
            InodeKind::Symlink => Self::Symlink,
            InodeKind::CharDevice => Self::CharDevice,
            InodeKind::Whiteout => Self::Whiteout,
        }
    }
}