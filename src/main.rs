use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum Fruit {
    #[serde(rename = "apple")]
    Apple(Apple),
    #[serde(rename = "pear")]
    Pear(Pear),
    #[serde(rename = "cherry")]
    Cherry(Cherry),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Apple {
    size: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pear {
    size: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cherry {
    size: f64,
    insects: Vec<Insects>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insects {
    pub name: String,
    pub size: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method")]
pub enum WebsocketMessage2 {
    #[serde(rename = "internal/get-best-bid-ask-quotes")]
    InternalGetBestBidAskQuotes(InternalGetBestBidAskQuotes),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalGetBestBidAskQuotes {
    pub best_bid_ask_quotes: Vec<BestBidAskQuotes>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestBidAskQuotes {
    pub amount: f64,
    pub ask_price: f64,
    pub bid_price: f64,
    pub ask_route_text: String,
    pub bid_route_text: String,
}

fn main() {}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{BestBidAskQuotes, Cherry, Fruit, Insects, Pear, WebsocketMessage2};

    #[test]
    fn test_serde_bug() {
        // Apple
        let apple = json!(
            {
                "name": "apple".to_owned(),
                "size": 1.23
            }
        );
        let apple_string = serde_json::to_string(&apple).unwrap();
        let apple = serde_json::from_str::<Fruit>(&apple_string).unwrap();
        println!("{:?}", apple);

        // pear
        let pear = json!(
            {
                "name": "pear".to_owned(),
                "size": 1.23,

            }
        );
        let pear_string = serde_json::to_string(&pear).unwrap();
        let pear = serde_json::from_str::<Fruit>(&pear_string).unwrap();
        println!("{:?}", pear);

        // Cherry
        let cherry = json!(
            {
                "name": "cherry".to_owned(),
                "size": 1.23,
                "insects": vec![Insects { name: "Cockroach".to_owned(), size: 1.23 }]
            }
        );
        let cherry_string = serde_json::to_string(&cherry).unwrap();
        let cherry = serde_json::from_str::<Fruit>(&cherry_string).unwrap();
        println!("{:?}", cherry);

        //Cherry
        let cherry = Fruit::Cherry(Cherry {
            size: 1.23,
            insects: vec![Insects {
                name: "Cockroach".to_owned(),
                size: 1.23,
            }],
        });
        let cherry_string = serde_json::to_string(&cherry).unwrap();
        let cherry = serde_json::from_str::<Fruit>(&cherry_string).unwrap();
        println!("Cherry 2 {:?}", cherry);

        let message =
            WebsocketMessage2::InternalGetBestBidAskQuotes(crate::InternalGetBestBidAskQuotes {
                best_bid_ask_quotes: vec![BestBidAskQuotes {
                    amount: 1.23,
                    ask_price: 4.56,
                    bid_price: 7.89,
                    ask_route_text: "Hi".to_owned(),
                    bid_route_text: "There".to_owned(),
                }],
            });
        let message_string = serde_json::to_string(&message).unwrap();
        let message = serde_json::from_str::<WebsocketMessage2>(&message_string).unwrap();
        println!("Parsed: {:?}", message);
    }
}
