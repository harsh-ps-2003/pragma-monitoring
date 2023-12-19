use std::collections::HashMap;

use crate::{constants::COINGECKO_IDS, error::MonitoringError, models::SpotEntry};

/// Data Transfer Object for Defillama API
/// e.g
///{
//   "coins": {
//     "coingecko:bitcoin": {
//       "price": 42220,
//       "symbol": "BTC",
//       "timestamp": 1702677632,
//       "confidence": 0.99
//     }
//   }
// }
#[derive(serde::Deserialize, Debug)]
struct CoinPricesDTO {
    coins: HashMap<String, CoinPriceDTO>,
}

#[allow(unused)]
#[derive(serde::Deserialize, Debug)]
struct CoinPriceDTO {
    price: f64,
    symbol: String,
    timestamp: u64,
    confidence: f64,
}

/// Calculates the deviation of the price from a trusted API (Coingecko)
pub async fn price_deviation(
    query: &SpotEntry,
    normalized_price: f64,
) -> Result<f64, MonitoringError> {
    let ids = &COINGECKO_IDS;

    let pair_id = query.pair_id.to_string();
    let coingecko_id = *ids.get(&pair_id).expect("Failed to get coingecko id");

    let request_url = format!(
        "https://coins.llama.fi/prices/historical/{timestamp}/coingecko:{id}",
        timestamp = query.timestamp.timestamp(),
        id = coingecko_id,
    );

    let response = reqwest::get(&request_url)
        .await
        .map_err(|e| MonitoringError::Api(e.to_string()))?;

    let coins_prices: CoinPricesDTO = response
        .json()
        .await
        .map_err(|e| MonitoringError::Api(e.to_string()))?;

    let api_id = format!("coingecko:{}", coingecko_id);

    let reference_price = coins_prices
        .coins
        .get(&api_id)
        .ok_or(MonitoringError::Api(format!(
            "Failed to get coingecko price for id {:?}",
            coingecko_id
        )))?
        .price;

    Ok((normalized_price - reference_price) / reference_price)
}
