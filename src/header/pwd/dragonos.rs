use super::{parsed, passwd};
use crate::platform::types::*;

pub fn split(line: &mut [u8]) -> Option<passwd> {
    return None;
    // 由于DragonOS尚不支持多用户的功能，因此返回None

    // === 以下是linux的实现
    // let mut parts = line.split_mut(|&c| c == b'\0');
    // Some(passwd {
    //     pw_name: parts.next()?.as_mut_ptr() as *mut c_char,
    //     pw_passwd: parts.next()?.as_mut_ptr() as *mut c_char,
    //     pw_uid: parsed(parts.next())?,
    //     pw_gid: parsed(parts.next())?,
    //     pw_gecos: parts.next()?.as_mut_ptr() as *mut c_char,
    //     pw_dir: parts.next()?.as_mut_ptr() as *mut c_char,
    //     pw_shell: parts.next()?.as_mut_ptr() as *mut c_char,
    // })
}
