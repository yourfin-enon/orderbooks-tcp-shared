use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum OrderbookTcpContract {
    Ping,
    Pong,
    Orderbook(OrderbookTcpModel),
    OrderbookUpdate(OrderbookTcpModel),
}

impl OrderbookTcpContract {
    pub fn is_ping(&self) -> bool {
        match self {
            OrderbookTcpContract::Ping => true,
            _ => false,
        }
    }

    pub fn parse(src: &str) -> Self {
        if src == "PING" {
            return Self::Ping;
        }
        if src == "PONG" {
            return Self::Pong;
        }

        Self::Orderbook(OrderbookTcpModel::parse(src).unwrap())
    }

    pub fn serialize(&self, dest: &mut Vec<u8>) {
        match self {
            OrderbookTcpContract::Ping => dest.extend_from_slice(b"PING"),
            OrderbookTcpContract::Pong => dest.extend_from_slice(b"PONG"),
            OrderbookTcpContract::Orderbook(data) => data.serialize(dest),
            OrderbookTcpContract::OrderbookUpdate(data) => data.serialize(dest),
        }
    }

    pub fn is_orderbook(&self) -> bool {
        match self {
            OrderbookTcpContract::Ping => false,
            OrderbookTcpContract::Pong => false,
            OrderbookTcpContract::Orderbook(_) => true,
            OrderbookTcpContract::OrderbookUpdate(_) => true,
        }
    }
}

impl my_tcp_sockets::tcp_connection::TcpContract for OrderbookTcpContract {
    fn is_pong(&self) -> bool {
        match self {
            OrderbookTcpContract::Pong => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderbookTcpModel {
    pub market: String,
    pub bids: Vec<(f64, f64)>,
    pub asks: Vec<(f64, f64)>,
    pub ts: i64,
}

impl OrderbookTcpModel {
    pub fn parse(src: &str) -> Option<Self> {
        let result: Result<OrderbookTcpModel, _> = serde_json::from_str(src);

        if let Ok(orderbook) = result {
            return Some(orderbook);
        }

        return None;
    }

    pub fn serialize(&self, dest: &mut Vec<u8>) {
        let result = serde_json::to_vec(self);
        let value = result.unwrap();
        dest.extend_from_slice(&value);        
    }
}