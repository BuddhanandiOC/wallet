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
        vals
            .iter()
            .map(|n| match n {
                1 => Some(1),
                5 => Some(5),
                10 => Some(10),
                25 => Some(25),
                50 => Some(50),
                100 => Some(100),
                _ => None,
            })
            .filter(|n| n.is_some())
            .map(|n| match n {
                Some(1) => Coins::Penny,
                Some(5) => Coins::Nickel,
                Some(10) => Coins::Dime,
                Some(25) => Coins::Quarter,
                Some(50) => Coins::HalfDollar,
                Some(100) => Coins::Dollar,
                _ => Coins::Penny, // TODO: find a way to get rid of this since None was filtered
            }).collect()
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
        vals
            .iter()
            .map(|n| match n {
                1 => Some(1),
                2 => Some(2),
                5 => Some(5),
                10 => Some(10),
                20 => Some(20),
                50 => Some(50),
                100 => Some(100),
                _ => None,
            })
            .filter(|n| n.is_some())
            .map(|n| match n {
                Some(1) => Bills::One,
                Some(2) => Bills::Two,
                Some(5) => Bills::Five,
                Some(10) => Bills::Ten,
                Some(20) => Bills::Twenty,
                Some(50) => Bills::Fifty,
                Some(100) => Bills::Hundred,
                _ => Bills::One, // TODO: find a way to get rid of this since None was filtered
            }).collect()
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
        Bills::from(vec![1, 1, 2, 5, 5, 10, 20, 50, 100, 100]),
        Coins::from(vec![1, 1, 1, 5, 10, 25, 25, 50, 100, 100])
    );

    println!("Total value: {}", wallet.total());
    println!("Total value (prop): {}", wallet.total_value);
}