use std::io;
use rand::Rng;
use std::cmp::Ordering;



fn main() {
    println!("----------");
    println!("Kyro's Rust Practice Library");
    println!("");
    println!("What would you like to see?");
    println!("Number Guesser:    [1]");
    println!("BlackJack:         [2]");
    println!("Placeholder:       [3]");
    println!("See SRC at github.com/kyrofx/rustpractice");
    let game = get_num();
    if game == 1 {
        number_guesser();
    }
    else if game == 2 {
        blackjack();
    }
    //else if game == 3 {
        
    //}
    else{
        println!("You did not enter a valid response.");
        main();
    }
}


fn blackjack() -> () {
    println!("Welcome to BlackJack!");
    let mut score = 0;
    let mut round = 0;
    let mut previous_cards: [String; 0] = [];
    loop {

        let mut card = generate_card();
        if card == "1" {
            card = ace_1or11(score);
        }
        previous_cards[round] = card.clone();
        let temporary_var_1 = print_card(card.clone());
        let temporary_var_2 = card_to_number(card.clone());
        println!("");
        println!("");
        println!("");
        println!("------------------------------");
        println!("Your card: {temporary_var_1}");
        score = score + temporary_var_2;
        println!("Your score: {score}");








    }
}
fn ace_1or11(score: u32) -> String {
    if score + 11 > 21 {
        // hard total
        return ("1").to_string();
    }
    else if score + 11 == 21 {
        return ("11").to_string();
    }
    println!("You got an ACE! You can either take 1 or 11 in this case. Which will you take? [1 / 11]");
    let mut ace_score = String::new();
            io::stdin()
                .read_line(&mut ace_score)
                .expect("Error: Failed to read line");
            ace_score = ace_score.trim().to_string();
    if ace_score == "1" {
        return ("14").to_string();
    }
    else if ace_score == "11" {
        return ("15").to_string(); 
    }
    else {
        println!("Your entry was not valid. Only '1' or '11' is an allowed answer");
        ace_1or11(score)
    }
        

}
fn print_card(card: String) -> String{
    let u32_card: u32 = card.parse().unwrap();
    match u32_card {
        2  => return ("🂢").to_string(),
        3  => return ("🂣").to_string(),
        4  => return ("🂤").to_string(),
        5  => return ("🂥").to_string(),
        6  => return ("🂦").to_string(),
        7  => return ("🂧").to_string(),
        8  => return ("🂨").to_string(),
        9  => return ("🂩").to_string(),
        10 => return ("🂪").to_string(),
        11 => return ("🂫").to_string(),
        12 => return ("🂭").to_string(),
        13 => return ("🂮").to_string(),
        14 => return ("🂡").to_string(),
        15 => return ("🂡").to_string(),
        _ => return ("Big Problem :3").to_string(),

    }
}
fn card_to_number(card: String) -> u32 {
    let u32_card: u32 = card.parse().unwrap();
    if (u32_card >= 2 && u32_card <= 15) {
        match u32_card {
            2  => return 2,
            3  => return 3,
            4  => return 4,
            5  => return 5,
            6  => return 6,
            7  => return 7,
            8  => return 8,
            9  => return 9,
            10 => return 10,
            11 => return 10,
            12 => return 10,
            13 => return 10,
            14 => return 1,
            15 => return 11,
            _ => return 0,
        }
    }
    else {
        println!("debug: illegal card: less than or exceeds 2 or 15.");
        return 0;
    }
}
fn generate_card() -> (String) {
    // Card Values:
    // 1: 1 or 11, Ace
    // 2: 2
    // 3: 3
    // 4: 4
    // 5: 5
    // 6: 6
    // 7: 7
    // 8: 8
    // 9: 9
    // 10: 10
    // 11: 10, Jack
    // 12: 10, Queen
    // 13: 10, King
    // 14: Ace 1
    // 15: Ace 11
    let rand = rand::thread_rng().gen_range(1..=13);
    match rand{
        1=> {
            // ACE
            return ("ace").to_string();
        }
        2=> {
            return ("2").to_string();
        }
        3=> {
            return ("3").to_string();
        }
        4=> {
            return ("4").to_string();
        }
        5=> {
            return ("5").to_string();
        }
        6=> {
            return ("6").to_string();
        }
        7=> {
            return ("7").to_string();
        }
        8=> {
            return ("8").to_string();
        }
        9=> {
            return ("9").to_string();
        }
        10=> {
            return ("10").to_string();
        }
        11=> {
            return ("jack").to_string();
        }
        12=> {
            return ("queen").to_string();
        }
        13=> {
            return ("king").to_string();
        }
        _=> return ("Big Error :3").to_string(),
    }
}


fn number_guesser() -> () {
    //println!("debug: define var 'num'");
    //let num = override_tester();
    let num = gen_num();
    let mut playcount: u32 = 1;
    println!("Welcome to Number Guesser!");
    println!("");
    //println!("debug: start loop");
    
    loop{
    println!("Guess the number: ");
    let ans = get_num(); 
    println!("You guessed: {ans}");
    let compared = compare_num((ans, num));
    if compared == 0{
        println!("Your guess was less! Try again.");
            playcount = playcount + 1; 
        }
    else if compared == 1{
        println!("Your guess was greater! Try again.");
            playcount = playcount + 1;
        }
    else if compared == 2{
            if playcount == 1 {
                println!("Your guess of {ans} was correct! You won with {playcount} try. Play again? [y/n]");
            }
            else {
                println!("Your guess of {ans} was correct! You won with {playcount} tries. Play again? [y/n]");

            }
            let mut play_again = String::new();
            io::stdin()
                .read_line(&mut play_again)
                .expect("Error: Failed to read line");
            play_again = play_again.trim().to_string();
            //println!("debug: play_again  defined as {play_again}");
            if play_again == "y" {
                //println!("debug: play_again defined as True");
                main();
            }
            else {
                println!("Bye!");
                break;
            }
        }
    else {

        }
    }
} 
fn get_num() -> u32 {
    loop {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Error: Failed to read line");
    
    //println!("debug: entering var guess loop");
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    return guess;
    }
}
fn gen_num() -> u32 {
    let rand = rand::thread_rng().gen_range(1..=100);
    return rand;
}
fn compare_num((guess, rand): (u32, u32)) -> u32 {
match guess.cmp(&rand){
        Ordering::Less => return 0,
        Ordering::Greater => return 1,
        Ordering::Equal => return 2,
    }
}
fn override_tester() -> u32 {
    return 1;
}
