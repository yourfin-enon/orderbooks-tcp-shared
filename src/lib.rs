mod tcp_contract;
mod tcp_serializer;

pub use tcp_contract::*;
pub use tcp_serializer::*;

#[cfg(test)]
mod tests {
    use std::collections::{HashMap};
    use chrono::{Utc};
    use crate::OrderbookTcpModel;

    #[test]
    fn test_ser_der() {
        let orderbook = OrderbookTcpModel {
            is_update: false,
            market: "test".to_string(),
            ts: Utc::now().timestamp_micros(),
            bids: HashMap::from([("123.4".to_string(), "123.5".to_string())]),
            asks: HashMap::from([("100.1".to_string(), "100.2".to_string())]),
        };
        let mut result = Vec::new();
        orderbook.serialize(&mut result);
        let result = OrderbookTcpModel::deserialize(&result);

        assert_eq!(result.market, orderbook.market);
        assert_eq!(result.ts, orderbook.ts);
        assert_eq!(result.bids, orderbook.bids);
        assert_eq!(result.asks, orderbook.asks);
    }
}
