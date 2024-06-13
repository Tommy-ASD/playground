use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::sync::mpsc;
use tokio::time::Duration;

pub struct RtpPacket {
    pub ssrc: u32,
    pub sequence: u16,
    pub timestamp: u32,
    pub payload: Vec<u8>,
}

pub struct RtpStream {
    receiver: mpsc::Receiver<RtpPacket>,
    sender: mpsc::Sender<RtpPacket>,
    timeout_duration: Duration,
}

impl RtpStream {
    pub fn new(buffer_size: usize, timeout_duration: Duration) -> Self {
        let (sender, receiver) = mpsc::channel(buffer_size);
        let stream = RtpStream {
            receiver,
            sender: sender,
            timeout_duration,
        };
        stream
    }

    pub fn sender(&self) -> mpsc::Sender<RtpPacket> {
        self.sender.clone()
    }
}

impl AsyncRead for RtpStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        match Pin::new(&mut self.receiver).poll_recv(cx) {
            Poll::Ready(Some(packet)) => {
                let payload = packet.payload;
                buf.put_slice(&payload);
                Poll::Ready(Ok(()))
            }
            Poll::Ready(None) => Poll::Ready(Ok(())), // Stream ended
            Poll::Pending => Poll::Pending,
        }
    }
}

impl AsyncWrite for RtpStream {
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        let packet = RtpPacket {
            ssrc: 0, // Replace with actual SSRC value if needed
            sequence: 0, // Replace with actual sequence number if needed
            timestamp: 0, // Replace with actual timestamp if needed
            payload: buf.to_vec(),
        };
        
        match self.sender.try_send(packet) {
            Ok(_) => Poll::Ready(Ok(buf.len())),
            Err(_) => Poll::Ready(Err(std::io::ErrorKind::BrokenPipe.into())), // Sender closed
        }
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

unsafe impl Send for RtpStream {}
unsafe impl Sync for RtpStream {}
impl Unpin for RtpStream {}
