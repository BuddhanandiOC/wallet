enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter,
    HalfDollar,
    Dollar,
}

impl Coins {
    fn from(vals: Vec<u8>) -> Vec<Coins> {
        vals.iter()
            .filter_map(|n| match n {
                1 => Some(Coins::Penny),
                5 => Some(Coins::Nickel),
                10 => Some(Coins::Dime),
                25 => Some(Coins::Quarter),
                50 => Some(Coins::HalfDollar),
                100 => Some(Coins::Dollar),
                _ => None,
            })
            .collect()
    }
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

impl Bills {
    fn from(vals: Vec<u8>) -> Vec<Bills> {
        vals.iter()
            .filter_map(|n| match n {
                1 => Some(Bills::One),
                2 => Some(Bills::Two),
                5 => Some(Bills::Five),
                10 => Some(Bills::Ten),
                20 => Some(Bills::Twenty),
                50 => Some(Bills::Fifty),
                100 => Some(Bills::Hundred),
                _ => None,
            })
            .collect()
    }
}

struct Wallet {
    total_value: f32,
    coins: Vec<Coins>,
    bills: Vec<Bills>,
}

impl Wallet {
    fn from(bills: Vec<Bills>, coins: Vec<Coins>) -> Wallet {
        let mut wallet = Wallet {
            total_value: 0.0,
            bills,
            coins,
        };

        wallet.total();
        wallet
    }

    fn total(&mut self) -> f32 {
        let mut total = 0.0;

        // sum coins
        self.coins.iter().for_each(|coin| match coin {
            Coins::Penny => total += 0.01,
            Coins::Nickel => total += 0.05,
            Coins::Dime => total += 0.10,
            Coins::Quarter => total += 0.25,
            Coins::HalfDollar => total += 0.50,
            Coins::Dollar => total += 1.00,
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
    let mut wallet = Wallet::from(
        Bills::from(vec![1, 1, 2, 5, 5, 10, 11, 20, 50, 100, 100]),
        Coins::from(vec![1, 1, 1, 5, 10, 11, 25, 25, 50, 100, 100]),
    );

    println!("Total value: {}", wallet.total());
    println!("Total value (prop): {}", wallet.total_value);

    assert_eq!(297.18, wallet.total());
    assert_eq!(297.18, wallet.total_value);
}
