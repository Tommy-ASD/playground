#![cfg(not(loom))]

//! TCP/UDP/Unix bindings for `tokio`.
//!
//! This module contains the TCP/UDP/Unix networking types, similar to the standard
//! library, which can be used to implement networking protocols.
//!
//! # Organization
//!
//! * [`TcpListener`] and [`TcpStream`] provide functionality for communication over TCP
//! * [`UdpSocket`] provides functionality for communication over UDP
//! * [`UnixListener`] and [`UnixStream`] provide functionality for communication over a
//! Unix Domain Stream Socket **(available on Unix only)**
//! * [`UnixDatagram`] provides functionality for communication
//! over Unix Domain Datagram Socket **(available on Unix only)**

//!
//! [`TcpListener`]: TcpListener
//! [`TcpStream`]: TcpStream
//! [`UdpSocket`]: UdpSocket
//! [`UnixListener`]: UnixListener
//! [`UnixStream`]: UnixStream
//! [`UnixDatagram`]: UnixDatagram

mod addr;
cfg_not_wasi! {
    #[cfg(feature = "net")]
    pub(crate) use addr::to_socket_addrs;
}
pub(crate)use addr::ToSocketAddrs;

cfg_net! {
    mod lookup_host;
    pub(crate)use lookup_host::lookup_host;

    pub(crate)mod tcp;
    pub(crate)use tcp::listener::TcpListener;
    pub(crate)use tcp::stream::TcpStream;
    cfg_not_wasi! {
        pub(crate)use tcp::socket::TcpSocket;

        mod udp;
        pub(crate)use udp::UdpSocket;
    }
}

cfg_net_unix! {
    pub(crate)mod unix;
    pub(crate)use unix::datagram::socket::UnixDatagram;
    pub(crate)use unix::listener::UnixListener;
    pub(crate)use unix::stream::UnixStream;
}

cfg_net_windows! {
    pub(crate)mod windows;
}
