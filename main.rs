use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut drafting: bool = true;
    let mut turn = 0;
    // game loop
    loop {
        // loops twice. The first time is the player's information, the second time is the opponent information
        let mut cards: Vec<card::Card> = Vec::new();
        let mut players: Vec<player::Player> = Vec::new();
        
        // create current player statuses
        for i in 0..2 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let player_health = parse_input!(inputs[0], i32);
            let player_mana = parse_input!(inputs[1], i32);
            let player_deck = parse_input!(inputs[2], i32);
            let player_rune = parse_input!(inputs[3], i32);
            let player_draw = parse_input!(inputs[4], i32);
            players.push(player::Player {
                player_health,
                player_mana,
                player_deck,
                player_rune,
                player_draw
            });
        }

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let opponent_hand = parse_input!(inputs[0], i32);
        let opponent_actions = parse_input!(inputs[1], i32);
        for i in 0..opponent_actions as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let card_number_and_action = input_line.trim_matches('\n').to_string();
        }
        
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let card_count = parse_input!(input_line, i32);
        for i in 0..card_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let card_number = parse_input!(inputs[0], i32);
            let instance_id = parse_input!(inputs[1], i32);
            let location = parse_input!(inputs[2], i32);
            let card_type = parse_input!(inputs[3], i32);
            let cost = parse_input!(inputs[4], i32);
            let attack = parse_input!(inputs[5], i32);
            let defense = parse_input!(inputs[6], i32);
            let abilities = inputs[7].trim().to_string();
            let my_health_change = parse_input!(inputs[8], i32);
            let opponent_health_change = parse_input!(inputs[9], i32);
            let card_draw = parse_input!(inputs[10], i32);
            cards.push(card::Card {
                card_number,
                instance_id,
                location,
                card_type,
                cost,
                attack,
                defense,
                abilities,
                my_health_change,
                opponent_health_change,
                card_draw,
                pick_id: i,
            })
        }
        
        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");
        if (turn == 30) {
            drafting = false;
            eprintln!("Draft Complete");
            game_turn::start(players, cards);
        } else if (drafting) {
            draft(cards);
        } else {
            game_turn::start(players, cards);
        }

        turn = turn + 1;
    }
}

// during the draft, the size of the vector will always be 3
fn draft(hand: Vec<card::Card>) {
    let mut efficiencies = vec![hand[0].raw_efficiency(), hand[1].raw_efficiency(), hand[2].raw_efficiency()];
    let mut max: (usize, f64) = (0, efficiencies[0]);
    for i in 1..efficiencies.len() as usize {
        if (efficiencies[i] > max.1) {
            max = (i, efficiencies[i]);
        }
    }
    println!("PICK {}", max.0);
}

mod card {
    use std::cmp;
    
    #[derive(Debug)]
    pub struct Card {
        pub card_number: i32,
        pub instance_id: i32,
        pub location: i32,
        pub card_type: i32,
        pub cost: i32,
        pub attack: i32,
        pub defense: i32,
        pub abilities: String,
        pub my_health_change: i32,
        pub opponent_health_change: i32,
        pub card_draw: i32,
        pub pick_id: usize, // TODO: Remove this
    }
    
    impl Card {
        pub fn play(&self) -> String {
            return format!("SUMMON {}", self.instance_id);
        }
        
        pub fn attack(&self, target: i32) -> String {
            return format!("ATTACK {} {}", self.instance_id, target);
        }
        
        pub fn attack_player(&self) -> String {
            return format!("ATTACK {} -1", self.instance_id);
        }
        
        pub fn raw_efficiency(&self) -> f64 {
            return ((self.attack + self.defense) / cmp::max(self.cost, 1)) as f64;
        }
    }

    trait Iterator {
        fn next(&mut self) -> Option<Card>;
    }
}

mod player {
    #[derive(Debug)]
    pub struct Player {
        pub player_health: i32,
        pub player_mana: i32,
        pub player_deck: i32,
        pub player_rune: i32,
        pub player_draw: i32
    }
}

mod game_turn {
    use crate::player;
    use crate::card;

    pub fn start(players: Vec<player::Player>, cards: Vec<card::Card>) {
        let mut opponent_board: Vec<&card::Card> =  cards.iter().filter(|card| card.location == -1).collect();
        let mut board: Vec<&card::Card> = cards.iter().filter(|card| card.location == 1).collect();
        let mut hand: Vec<&card::Card> = cards.iter().filter(|card| card.location == 0).collect();

        let user = &players[0];
        let opponent = &players[1];

        let mut play = String::new();

        let playable: Vec<&card::Card> = hand
            .into_iter()
            .filter(|card| card.cost <= user.player_mana)
            .collect::<Vec<&card::Card>>();
        play.push_str(&format!("{}", pure_efficiency_play(&players[0], playable)));
        play.push_str(&format!("{}", full_face_attack(board)));

        if (play.len() > 0) {
            println!("{}", play);
        } else {
            println!("PASS");
        }
    }

    fn pure_efficiency_play(player: &player::Player, playable: Vec<&card::Card>) -> String {
        let mut summon_string = String::new();
        let mut current_mana = player.player_mana;
        
        while (current_mana >= 0) {
            for card in &playable {
                if (current_mana == card.cost) {
                    summon_string.push_str(&format!("{};", card.play()));
                    current_mana = current_mana - card.cost;
                    break;
                } else {
                    current_mana = current_mana - 1;
                }
            }
        }

        return summon_string;
        // loop over the playables, remove the one with the highest efficiency, add to the summon string
        // continue until there are no available plays and return the summoning string
    }

    fn full_face_attack(board: Vec<&card::Card>) -> String {
        let mut attack_string = String::new();
        for card in board {
            attack_string.push_str(&format!("{};", card.attack(-1)));
        }
        return attack_string;
    }
}

