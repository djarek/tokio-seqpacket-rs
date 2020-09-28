pub mod mio;
mod ucred;
mod unix_seqpacket;
mod unix_seqpacket_listener;

pub use unix_seqpacket::UnixSeqpacket;
pub use unix_seqpacket_listener::UnixSeqpacketListener;

/// Get the socket type for a close-on-exec non-blocking seqpacket socket.
fn socket_type() -> socket2::Type {
	socket2::Type::seqpacket().cloexec().non_blocking()
}

/// Convert a [`socket2::SockAddr`] to a [`std::os::unix::net::SocketAddr`].
fn sockaddr_as_unix(addr: &socket2::SockAddr) -> Option<std::os::unix::net::SocketAddr> {
	if addr.family() != libc::AF_UNIX as libc::sa_family_t {
		return None;
	}

	#[allow(dead_code)]
	struct SocketAddrInternal {
		addr: libc::sockaddr_un,
		len: libc::socklen_t,
	}

	unsafe {
		let internal = SocketAddrInternal {
			addr: std::ptr::read(addr.as_ptr() as *const libc::sockaddr_un),
			len: addr.len(),
		};
		Some(std::mem::transmute(internal))
	}
}