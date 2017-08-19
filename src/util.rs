//! Utilities
use std::env;
use std::ptr;
use std::ffi::CStr;
use libc;

/// The informations in /etc/passwd corresponding to the current user.
struct Passwd {
    pub shell: String
}

/// Return the informations in /etc/passwd corresponding to the current user.
fn get_passwd() -> Result<Passwd, ()> {
    unsafe {
        let passwd = libc::getpwuid(libc::getuid()).to_result()?;

        let shell = CStr::from_ptr(passwd.pw_shell)
            .to_str()
            .map_err(|_| ())?
            .to_string();

        Ok(Passwd { shell })
    }
}

/// Returns the path to the shell executable.
///
/// Tries in order:
///
/// * to read the SHELL env var
/// * to read the user's passwd
/// * defaults to /bin/sh
///
/// # Example
///
/// ```rust
/// # use term_mux::util::get_shell;
/// let shell = get_shell();
/// assert!(shell.contains("/bin/"));
/// assert!(shell.contains("sh"));
/// ```
pub fn get_shell() -> String {
    env::var("SHELL")
        .or_else(|_| get_passwd().map(|passwd| passwd.shell))
        .unwrap_or_else(|_| "/bin/sh".to_string())
}

/// Converts a value returned by a libc function to a rust result.
pub trait FromLibcResult: Sized {
    type Target;

    /// The intented use is for the user to call map_err() after this function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use term_mux::util::FromLibcResult;
    /// let result = -1;
    /// assert_eq!(Err(()), result.to_result());
    ///
    /// let result = 42;
    /// assert_eq!(Ok(42), result.to_result());
    /// ```
    fn to_result(self) -> Result<Self::Target, ()>;
}

impl FromLibcResult for libc::c_int {
    type Target = libc::c_int;

    fn to_result(self) -> Result<libc::c_int, ()> {
        match self {
            -1  => Err(()),
            res => Ok(res),
        }
    }
}

impl FromLibcResult for *mut libc::passwd {
    type Target = libc::passwd;

    fn to_result(self) -> Result<libc::passwd, ()> {
        if self == ptr::null_mut() { return Err(()) }
        else                       { unsafe { Ok(*self) } }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use libc;

    #[test]
    fn to_result_on_c_int() {
        let result = -1;
        assert_eq!(Err(()), result.to_result());

        let result = 42;
        assert_eq!(Ok(42), result.to_result());
    }

    #[test]
    fn to_result_on_passwd() {
        let passwd: *mut libc::passwd = ptr::null_mut();
        assert!(passwd.to_result().is_err());

        let passwd: *mut libc::passwd = unsafe { &mut mem::uninitialized() };
        assert!(passwd.to_result().is_ok());
    }
}
