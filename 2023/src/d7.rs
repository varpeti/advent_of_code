use crate::d7::m_day7_b::show_card;

mod m_day7_a {
    use itertools::Itertools;
    use std::cmp::Ordering;

    type Card = u8;

    fn to_card(c: char) -> Card {
        match c {
            '2'..='9' => c as Card - 50,
            'T' => 8,
            'J' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            err => panic!("Invalid card: {}", err),
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Hand {
        pub cards: [Card; 5],
        pub bet: u32,
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    enum CType {
        HighCard,
        OnePair,
        TwoPair,
        ThreeKind,
        FullHouse,
        FourKind,
        FiveKind,
    }

    impl Hand {
        fn get_ctype(&self) -> CType {
            let counts = self.cards.iter().counts();
            let mut is_threekind = false;
            let mut num_onepair: u8 = 0;
            for value in counts.values() {
                match *value {
                    5 => return CType::FiveKind,
                    4 => return CType::FourKind,
                    3 => is_threekind = true,
                    2 => num_onepair += 1,
                    _ => (),
                }
            }
            match (is_threekind, num_onepair) {
                (true, 1) => CType::FullHouse,
                (true, 0) => CType::ThreeKind,
                (false, 2) => CType::TwoPair,
                (false, 1) => CType::OnePair,
                (false, 0) => CType::HighCard,
                _ => unreachable!(),
            }
        }

        fn my_cmp(&self, other: &Self) -> Ordering {
            match self.get_ctype().cmp(&other.get_ctype()) {
                Ordering::Equal => self
                    .cards
                    .iter()
                    .zip(other.cards.iter())
                    .find_map(|(a, b)| match a.cmp(b) {
                        Ordering::Equal => None,
                        ord => Some(ord),
                    })
                    .unwrap_or(Ordering::Equal),
                ord => ord,
            }
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.my_cmp(other))
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            self.my_cmp(other)
        }
    }

    pub fn to_hand(str: &str) -> Hand {
        let mut str = str.split_whitespace();
        let cards = str
            .next()
            .expect("hand")
            .chars()
            .map(to_card)
            .collect::<Vec<Card>>()
            .try_into()
            .expect("[]hand");
        let bet = str.next().expect("bet").parse::<u32>().expect("pbet");
        Hand { cards, bet }
    }
}

pub fn day7_a(input: &String) -> String {
    let mut hands = input
        .lines()
        .map(m_day7_a::to_hand)
        .collect::<Vec<m_day7_a::Hand>>();
    hands.sort();
    format!(
        "{}",
        hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (i as u32 + 1) * hand.bet)
            .sum::<u32>()
    )
}

////////////////////////////////////////B//////////////////////////////////////

mod m_day7_b {
    use itertools::Itertools;
    use std::cmp::Ordering;

    type Card = u8;

    fn to_card(c: char) -> Card {
        match c {
            'J' => 0,
            '2'..='9' => c as Card - 49,
            'T' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            err => panic!("Invalid card: {}", err),
        }
    }

    pub fn show_card(c: Card) -> char {
        match c {
            0 => 'J',
            1..=8 => (c + 49) as char,
            9 => 'T',
            10 => 'Q',
            11 => 'K',
            12 => 'A',
            err => panic!("Invalid card: {}", err),
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Hand {
        pub cards: [Card; 5],
        pub bet: usize,
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum CType {
        HighCard,
        OnePair,
        TwoPair,
        ThreeKind,
        FullHouse,
        FourKind,
        FiveKind,
    }

    impl Hand {
        pub fn get_ctype(&self) -> CType {
            // println!("AAA5 {:?}", self);
            let counts = self.cards.iter().counts();
            let mut joker = match counts.get(&0) {
                Some(num) => *num,
                None => 0,
            };
            let mut is_threekind = false;
            let mut num_onepair: u8 = 0;
            for (card, count) in counts.iter().sorted_by(|a, b| a.1.cmp(b.1)).rev() {
                // println!("{}: {}", show_card(**card), count);
                match (**card == 0, *count == 5) {
                    (true, true) => return CType::FiveKind,
                    (true, false) => continue,
                    _ => (),
                }
                match (*count, joker) {
                    (5, 0) => return CType::FiveKind,
                    (4, 0) => return CType::FourKind,
                    (3, 0) => is_threekind = true,
                    (2, 0) => num_onepair += 1,

                    (4, 1) => return CType::FiveKind,
                    (3, 1) => return CType::FourKind,
                    (2, 1) => {
                        is_threekind = true;
                        joker -= 1
                    }
                    (1, 1) => {
                        num_onepair += 1;
                        joker -= 1;
                    }

                    (3, 2) => return CType::FiveKind,
                    (2, 2) => return CType::FourKind,
                    (1, 2) => {
                        is_threekind = true;
                        joker -= 2
                    }

                    (2, 3) => return CType::FiveKind,
                    (1, 3) => return CType::FourKind,

                    (1, 4) => return CType::FiveKind,
                    _ => (),
                }
            }
            match (is_threekind, num_onepair) {
                (true, 1) => CType::FullHouse,
                (true, 0) => CType::ThreeKind,
                (false, 2) => CType::TwoPair,
                (false, 1) => CType::OnePair,
                (false, 0) => CType::HighCard,
                _ => unreachable!(),
            }
        }

        fn my_cmp(&self, other: &Self) -> Ordering {
            match self.get_ctype().cmp(&other.get_ctype()) {
                Ordering::Equal => self
                    .cards
                    .iter()
                    .zip(other.cards.iter())
                    .find_map(|(a, b)| match a.cmp(b) {
                        Ordering::Equal => None,
                        ord => Some(ord),
                    })
                    .unwrap_or(Ordering::Equal),
                ord => ord,
            }
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.my_cmp(other))
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            self.my_cmp(other)
        }
    }

    pub fn to_hand(str: &str) -> Hand {
        let mut str = str.split_whitespace();
        let cards = str
            .next()
            .expect("hand")
            .chars()
            .map(to_card)
            .collect::<Vec<Card>>()
            .try_into()
            .expect("[]hand");
        let bet = str.next().expect("bet").parse::<usize>().expect("pbet");
        Hand { cards, bet }
    }
}

pub fn day7_b(input: &String) -> String {
    let mut hands = input
        .lines()
        .map(m_day7_b::to_hand)
        .collect::<Vec<m_day7_b::Hand>>();
    hands.sort();
    // for hand in hands.iter() {
    //     for card in hand.cards {
    //         print!("{}", show_card(card))
    //     }
    //     println!(" {:?} {}", hand.get_ctype(), hand.bet)
    // }
    format!(
        "{}",
        hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (i + 1) * hand.bet)
            .sum::<usize>()
    )
}
