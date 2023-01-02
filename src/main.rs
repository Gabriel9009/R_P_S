use rand::Rng;
use std::fmt::{Display, Formatter, Error};

enum Status{ Loss, Win, Draw}

#[derive(Clone)]
struct Hand{ name:String, weekness:String }
impl Hand{
    pub fn new(name: &str, weekness: &str)->Self{
        return Hand{ name:String::from(name), weekness:String::from(weekness) };
    }
    
    pub fn compare(&self, hand: Hand)->Status{
        if self.name.eq_ignore_ascii_case(hand.name.as_str()){
            return Status::Draw;
        }else if self.weekness.eq_ignore_ascii_case(hand.name.as_str()){
            return Status::Loss;
        }
        return Status::Win;
    }
}

impl Display for Hand{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
        write!(f, "{}", self.name)
    }
}

fn main() {
    println!("This is a Game of Rock, Paper,  Scissor\n");
    
    let mut rng = rand::thread_rng();
    let hands:[Hand; 3] = [ Hand::new("Rock", "Paper"), Hand::new("Paper", "Scissors"), Hand::new("Scissors", "Rock") ];
    
    let (mut player_one, mut player_two, mut index) = (0, 0, 0);
    while index < 3 || player_one == player_two{
        let (player_one_hand, player_two_hand) = (hands[rng.gen_range(0..3)].clone(), hands[rng.gen_range(0..3)].clone());
        println!("Player 1: {} vs Player 2: {}", player_one_hand, player_two_hand);
        
        match player_one_hand.compare(player_two_hand){
            Status::Win => { 
                println!("Point goes to Player 1\n");
                player_one += 1; },
            Status::Loss => { 
                println!("Point goes to Player 2\n");
                player_two += 1; },
            Status::Draw => println!("it's a tie\n"),
        }
        index += 1;
    }
    
    if player_one > player_two{
        println!("Player 1 Wins");
    }else{
        println!("Player 2 Wins");
    }
}
