use std::process::{Command, Stdio, ChildStderr, Child};
use std::ptr;
use std::io::{Read, Error};
use std::fs::File;

#[cfg(target_os = "linux")]
pub fn get_tty(mut command: Command) -> Handle {
	use libc::{self, winsize, c_int, TIOCGWINSZ};
	use std::os::unix::io::FromRawFd;
	use std::fs::File;

    let mut master: c_int = 0;
    let mut slave: c_int = 0;

    let mut win = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    let res = unsafe {
		libc::ioctl(0, TIOCGWINSZ, &mut win);
        libc::openpty(&mut master, &mut slave, ptr::null_mut(), ptr::null(), &win)
    };  

    if res < 0 { 
        panic!("Failed to open pty: {}", res);
    }

	let child = command.stderr(unsafe { Stdio::from_raw_fd(slave) } )
		.spawn()
		.unwrap();

    unsafe {
		Handle::Pty {child: child, fd: master as usize, file: File::from_raw_fd(master) }
	}
}

pub enum Handle {
	Pty {child: Child, fd: usize, file: File },
	Stderr(ChildStderr)
}

impl Read for Handle {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
		match *self {
			Handle::Pty{ref mut file, ..} => file.read(buf),
			Handle::Stderr(ref mut stderr) => stderr.read(buf) 
		}
	}
}


pub fn handle_err(handle: &mut Handle, err: &Error) -> bool {
	if let Handle::Pty {ref mut child, ..} = *handle {
		if child.try_wait().unwrap().is_some() {
			return true;
		}
	}
	panic!("Error: {:?}", err)
}

#[cfg(target_os = "linux")]
pub fn get_handle(command: Command, tty: bool) -> Handle {
    if tty {
        return get_tty(command)
    }
    get_handle_base(command)
}

#[cfg(not(target_os = "linux"))]
pub fn get_handle(mut command: Command, tty: bool) -> Handle {
    get_handle_base(command)
}

fn get_handle_base(mut command: Command) -> Handle {
	let child = command.stderr(Stdio::piped())
		.spawn()
		.unwrap();

	Handle::Stderr(child.stderr.unwrap())
}
