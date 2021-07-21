enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

enum Bills {
    One,
    Two,
    Five,
    Ten,
    Twenty,
    Fifty,
    Hundred,
}

struct Wallet {
    total_value: f32,
    coins: Vec<Coins>,
    bills: Vec<Bills>,
}

impl Wallet {
    fn total(&mut self) -> f32 {
        let mut total = 0.0;

        // sum coins
        self.coins.iter().for_each(|coin| match coin {
            Coins::Penny => total += 0.01,
            Coins::Nickel => total += 0.05,
            Coins::Dime => total += 0.10,
            Coins::Quarter => total += 0.25,
        });

        // sum bills
        self.bills.iter().for_each(|bill| match bill {
            Bills::One => total += 1.0,
            Bills::Two => total += 2.0,
            Bills::Five => total += 5.0,
            Bills::Ten => total += 10.0,
            Bills::Twenty => total += 20.0,
            Bills::Fifty => total += 50.0,
            Bills::Hundred => total += 100.0,
        });

        // update struct
        self.total_value = total;

        total
    }
}

fn main() {
    let mut wallet = Wallet {
        total_value: 0.0,
        coins: vec![
            Coins::Penny,
            Coins::Dime,
            Coins::Quarter,
            Coins::Quarter,
        ],
        bills: vec![
            Bills::One,
            Bills::One,
            Bills::Two,
            Bills::Twenty,
            Bills::Twenty,
            Bills::Hundred,
        ]
    };

    println!("Total value: {}", wallet.total());
    println!("Total value (prop): {}", wallet.total_value);
}