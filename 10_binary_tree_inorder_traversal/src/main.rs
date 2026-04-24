fn main() {
    println!("Hello, world!");
}

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    for (buy_day, buy_price) in prices.iter().enumerate() {
        let sellable_day = buy_day + 1;
        for sell_price in &prices[sellable_day..] {
            let profit = sell_price - buy_price;
            if profit > max_profit {
                max_profit = profit
            }
        }
    }

    max_profit
}
