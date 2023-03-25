use crate::platform::types::*;

/// Open Read-only
pub const O_RDONLY: c_int = 0o0;
/// Open Write-only
pub const O_WRONLY: c_int = 0o1;
/// Open read/write
pub const O_RDWR: c_int = 0o2;
/// Mask for file access modes
pub const O_ACCMODE: c_int = 0o00000003;

/* Bits OR'd into the second argument to open.  */
/// Create file if it does not exist
pub const O_CREAT: c_int = 0o00000100;
/// Fail if file already exists
pub const O_EXCL: c_int = 0o00000200;
/// Do not assign controlling terminal
pub const O_NOCTTY: c_int = 0o00000400;
/// 文件存在且是普通文件，并以O_RDWR或O_WRONLY打开，则它会被清空
pub const O_TRUNC: c_int = 0o00001000;
/// 文件指针会被移动到文件末尾
pub const O_APPEND: c_int = 0o00002000;
/// 非阻塞式IO模式
pub const O_NONBLOCK: c_int = 0o00004000;
/// used to be O_SYNC, see below
pub const O_DSYNC: c_int = 0o00010000;
/// fcntl, for BSD compatibility
pub const FASYNC: c_int = 0o00020000;
/* direct disk access hint */
pub const O_DIRECT: c_int = 0o00040000;
pub const O_LARGEFILE: c_int = 0o00100000;
/// 打开的必须是一个目录
pub const O_DIRECTORY: c_int = 0o00200000;
/// Do not follow symbolic links
pub const O_NOFOLLOW: c_int = 0o00400000;
pub const O_NOATIME: c_int = 0o01000000;
/// set close_on_exec
pub const O_CLOEXEC: c_int = 0o02000000;
