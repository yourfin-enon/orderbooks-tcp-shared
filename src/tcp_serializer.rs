use async_trait::async_trait;
use my_tcp_sockets::{
    socket_reader::{ReadBuffer, ReadingTcpContractFail, SocketReader},
    TcpSocketSerializer,
};

use super::tcp_contract::OrderbookTcpContract;

static CLCR: &[u8] = &[13u8, 10u8];

pub struct OrderbookTcpSerializer {
    read_buffer: ReadBuffer,
}

impl OrderbookTcpSerializer {
    pub fn new() -> Self {
        Self {
            read_buffer: ReadBuffer::new(1024 * 24),
        }
    }
}

#[async_trait]
impl TcpSocketSerializer<OrderbookTcpContract> for OrderbookTcpSerializer {
    const PING_PACKET_IS_SINGLETONE: bool = true;
    
    fn serialize(&self, contract: OrderbookTcpContract) -> Vec<u8> {
        let mut result = Vec::new();
        contract.serialize(&mut result);
        result.extend_from_slice(CLCR);
        result
    }

    fn serialize_ref(&self, contract: &OrderbookTcpContract) -> Vec<u8> {
        let mut result = Vec::new();
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
