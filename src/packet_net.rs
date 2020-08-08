// These functions were placed in a separate file due to signature conflicts between ByteOrders
// read_ functions and tokio's async equivalents. But sadly we're required to use ByteOrder as well
// because Tokio only supports Big Endian.

use std::pin::Pin;

use byteorder::{ByteOrder, LittleEndian};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::{error::RconError, packet::Packet};

impl Packet {
	pub(crate) async fn send_internal(&self, mut stream: Pin<&mut impl AsyncWrite>) -> Result<(), RconError> {
		stream.write_all(&self.create_packet_buffer()?).await?;
		Ok(stream.flush().await?)
	}

	pub(crate) async fn read(mut stream: Pin<&mut impl AsyncRead>) -> Result<Self, RconError> {
		let mut len_buf = vec![0; 4];
		stream.read_exact(&mut len_buf).await?;
		let len = LittleEndian::read_i32(&len_buf) as usize;

		let mut buf = Vec::with_capacity(len);
		stream.take(len as u64).read_to_end(&mut buf).await?;
		Ok(Packet::decode_packet_buffer(len, &buf)?)
	}
}
