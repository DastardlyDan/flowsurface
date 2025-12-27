use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChartData {
    pub symbol: String,
    pub timeframe: String,
    pub data: Vec<Candlestick>,
    pub indicators: Option<Indicators>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Candlestick {
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Indicators {
    pub sma_50: Option<Vec<f64>>,
    pub rsi: Option<Vec<f64>>,
    // Add more indicators as needed
}

impl ChartData {
    pub fn new(symbol: String, timeframe: String, data: Vec<Candlestick>, indicators: Option<Indicators>) -> Self {
        Self {
            symbol,
            timeframe,
            data,
            indicators,
        }
    }
}