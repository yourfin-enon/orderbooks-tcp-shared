use std::collections::{HashMap};

#[derive(Debug, Clone)]
pub enum OrderbookTcpContract {
    Ping,
    Pong,
    Orderbook(OrderbookTcpModel),
}

impl OrderbookTcpContract {
    pub fn is_ping(&self) -> bool {
        matches!(self, OrderbookTcpContract::Ping)
    }

    pub fn deserialize(bytes: &[u8]) -> Self {
        if bytes == b"PING" {
            return Self::Ping;
        }
        if bytes == b"PONG" {
            return Self::Pong;
        }

        Self::Orderbook(OrderbookTcpModel::deserialize(bytes))
    }

    pub fn serialize(&self, dest: &mut Vec<u8>) {
        match self {
            OrderbookTcpContract::Ping => dest.extend_from_slice(b"PING"),
            OrderbookTcpContract::Pong => dest.extend_from_slice(b"PONG"),
            OrderbookTcpContract::Orderbook(data) => data.serialize(dest),
        }
    }

    pub fn is_orderbook(&self) -> bool {
        match self {
            OrderbookTcpContract::Ping => false,
            OrderbookTcpContract::Pong => false,
            OrderbookTcpContract::Orderbook(_) => true,
        }
    }
}

impl my_tcp_sockets::tcp_connection::TcpContract for OrderbookTcpContract {
    fn is_pong(&self) -> bool {
        matches!(self, OrderbookTcpContract::Pong)
    }
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderbookTcpModel {
    #[prost(string, tag = "1")]
    pub market: String,
    #[prost(map = "string, string", tag = "2")]
    pub bids: HashMap<String, String>,
    #[prost(map = "string, string", tag = "3")]
    pub asks: HashMap<String, String>,
    #[prost(int64, tag = "4")]
    pub ts: i64,
    #[prost(bool, tag = "5")]
    pub is_update: bool,
}

impl OrderbookTcpModel {
    pub fn deserialize(bytes: &[u8]) -> Self {
        prost::Message::decode(bytes).expect("Failed to proto decode")
    }

    pub fn serialize(&self, dest: &mut Vec<u8>) {
        prost::Message::encode(self, dest).expect("Failed to proto encode");
    }
}