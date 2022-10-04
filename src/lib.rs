mod tcp_contract;
mod tcp_serializer;

pub use tcp_contract::*;
pub use tcp_serializer::*;

#[cfg(test)]
mod tests {
    use chrono::{Utc};
    use crate::Orderbook;

    #[test]
    fn test_parse() {
        let orderbook = Orderbook{
            market: "test".to_string(),
            ts: Utc::now().timestamp_micros(),
            bids: vec![(123.4, 123.5)],
            asks: vec![(100.1, 100.2)],
        };
        let json = serde_json::to_string(&orderbook).unwrap();
        let result = Orderbook::parse(&json).unwrap();

        assert_eq!(result.market, orderbook.market);
        assert_eq!(result.ts, orderbook.ts);
        assert_eq!(result.bids, orderbook.bids);
        assert_eq!(result.asks, orderbook.asks);
    }
}
