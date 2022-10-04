use async_trait::async_trait;
use my_tcp_sockets::{
    socket_reader::{ReadBuffer, ReadingTcpContractFail, SocketReader},
    TcpSocketSerializer,
};

use super::orderbook_tcp_contract::OrderbookTcpContract;

static CLCR: &[u8] = &[13u8, 10u8];
const MAX_PACKET_CAPACITY: usize = 512;

pub struct SourceFeedSerializer {
    read_buffer: ReadBuffer,
}

impl SourceFeedSerializer {
    pub fn new() -> Self {
        Self {
            read_buffer: ReadBuffer::new(1024 * 24),
        }
    }
}

#[async_trait]
impl TcpSocketSerializer<OrderbookTcpContract> for SourceFeedSerializer {
    fn serialize(&self, contract: OrderbookTcpContract) -> Vec<u8> {
        let mut result = Vec::with_capacity(MAX_PACKET_CAPACITY);
        contract.serialize(&mut result);
        result.extend_from_slice(CLCR);
        result
    }

    fn serialize_ref(&self, contract: &OrderbookTcpContract) -> Vec<u8> {
        let mut result = Vec::with_capacity(MAX_PACKET_CAPACITY);
        contract.serialize(&mut result);
        result.extend_from_slice(CLCR);
        result
    }

    fn get_ping(&self) -> OrderbookTcpContract {
        return OrderbookTcpContract::Ping;
    }
    
    async fn deserialize<TSocketReader: Send + Sync + 'static + SocketReader>(
        &mut self,
        socket_reader: &mut TSocketReader,
    ) -> Result<OrderbookTcpContract, ReadingTcpContractFail> {
        let result = socket_reader
            .read_until_end_marker(&mut self.read_buffer, CLCR)
            .await?;

        let result = std::str::from_utf8(&result[..result.len() - CLCR.len()]).unwrap();

        Ok(OrderbookTcpContract::parse(result))
    }

    fn apply_packet(&mut self, _contract: &OrderbookTcpContract) -> bool {
        false
    }
}
