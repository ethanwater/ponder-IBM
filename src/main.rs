//IBM Research | Ponder This February Rust Solution - Ethan Water
//
//The code below assumes that no match of rounds can result in a tie.
//The percentage the system outputs is in accordance to Alice's probability
//of winning a game in both the original N=13 and the bonus N=300 variations.
//To further enhance the readability of the code, I have provided comments
//describing the methods used.

use rand::Rng;

//roll() simulates the sum of rolling all 6 die using a lazily-initialized
//thread-local random number generator, seeded by the system upon. ensuring a
//pure generation on each call. It returns the sum of the rolled die.
fn roll() -> u32 {
    let die_vector = vec![4, 6, 8, 9, 12, 20];
    let mut thread = rand::thread_rng();
    let mut sum: u32 = 0;

    for &die in &die_vector {
        if die == 9 {
            sum += thread.gen_range(0..=die);
            continue;
        }
        sum += thread.gen_range(1..=die);
    }

    return sum;
}

//is_prime() determines primality using the trial division method. It
//returns wether the number is prime (true) or not prime (false).
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1
    }
    return true;
}

//play() simulates a game, based on 'n: the required amount of wins to end the game',
//and 'bonus_state: if true, the bonus problem is considered where Bob requires a nonprime
//odd number to win the round- if false, the original problem is considered where Bob
//requires a nonprime even number to win the round.
//
//the play() method disregards ties for practical purposes.
fn play(n: u32, bonus_state: bool) -> u32 {
    //on game start: initialize alices and bobs points as 0.
    let (mut alice_pts, mut bob_pts): (u32, u32) = (0, 0);

    loop {
        //for each round we roll the six die
        let roll = roll();

        //if the sum of the die is prime, alice recieves a point for winning the round.
        if is_prime(roll) {
            alice_pts += 1;
            if alice_pts >= n {
                return 1; //Alice wins
            }
            continue;
        } else if !is_prime(roll) && roll % 2 == bonus_state as u32 {
            //if the sum of the die is nonprime (and even or odd based on the 'bonus_state'), Bob
            //recieves a point for winning the round.
            bob_pts += 1;
            if bob_pts >= n {
                return 0; //Bob wins
            }
            continue;
        }
    }
}

//the RUNS constant is used to simulate the designated amount of simulations.
const RUNS: i32 = 100_000;

fn main() {
    let (mut alice_wins, mut alice_bonus_wins): (u32, u32) = (0, 0);

    //for each 'run' we collect the results of Alices wins in both the original and bonus
    //problem sets.
    for _ in 0..RUNS {
        alice_wins += play(13, false);
        alice_bonus_wins += play(300, true);
    }

    //convert and format the results to a percentage.
    let alice_probability: f64 = (100.0 / RUNS as f64) * alice_wins as f64;
    let alice_probability_bonus: f64 = (100.0 / RUNS as f64) * alice_bonus_wins as f64;

    println!("alice normal win probability: %{alice_probability}");
    println!("alice bonus  win probability: %{alice_probability_bonus}");
}
