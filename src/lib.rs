mod tcp_contract;
mod tcp_serializer;

pub use tcp_contract::*;
pub use tcp_serializer::*;

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use chrono::{Utc};
    use crate::OrderbookTcpModel;

    #[test]
    fn test_parse() {
        let orderbook = OrderbookTcpModel{
            market: "test".to_string(),
            ts: Utc::now().timestamp_micros(),
            bids: BTreeMap::from([("123.4".to_string(), "123.5".to_string())]),
            asks:  BTreeMap::from([("100.1".to_string(), "100.2".to_string())]),
        };
        let json = serde_json::to_string(&orderbook).unwrap();
        let result = OrderbookTcpModel::parse(&json).unwrap();

        assert_eq!(result.market, orderbook.market);
        assert_eq!(result.ts, orderbook.ts);
        assert_eq!(result.bids, orderbook.bids);
        assert_eq!(result.asks, orderbook.asks);
    }
}
