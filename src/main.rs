#![allow(unused_imports, dead_code, unused_variables, unreachable_code)]
use rand::rngs::ThreadRng;
use rand::Rng;
use std::error::Error;
use std::io::Write;

mod user_input;
use user_input::{BuySell, WholeNumber};

#[derive(Default, Debug, Clone, Copy)]
pub struct City {
    year: i32,
    people: i32,
    land: i32,
    grain: i32,
    starved: i32,
    plague_victims: i32,
    births: i32,
    rats_ate: i32,
    harvest_yield: i32,
    total_starved: i32,
    total_plague_deaths: i32,
    total_births: i32,
}

impl City {
    fn clear_events(&mut self) {
        self.starved = 0;
        self.plague_victims = 0;
        self.births = 0;
        self.harvest_yield = 0;
    }
}

enum TurnResult {
    Continue(City),
    Starved(City),
    Quit(City),
}

fn warn_insufficient_land(land: i32) {
    println!("Hammurabi: Think again. You own only {land} acres. Now then,");
}

fn warn_insufficient_grain(grain: i32) {
    println!("Hammurabi: Think again. You have only {grain} bushels of grain. Now then,")
}

fn warn_optimal_grain(grain: i32, optimum: i32) {
    println!("Hammurabi: The people need at least {optimum} bushels of grain.");
    println!("You have only {grain} bushels. Now then,");
}

fn report_allocated_optimum_grain(optimum: i32) {
    println!("I have allocated {optimum} bushels of grain in your name.");
}

fn report_planted_maximum_land(land: i32) {
    println!("I have ordered {land} acres to be planted in your name.");
}

fn warn_insufficient_labor(labor: i32) {
    println!("But you have only {labor} people to tend the fields.");
}

fn steward_quits() {
    println!("Hammurabi, I cannot do what you wish.");
    println!("Get yourself another steward!");
}

fn warn_invalid_input() {
    println!("I did not understand your command!");
}

fn prompt_for_number(prompt: &str) -> i32 {
    loop {
        let mut b = String::new();
        print!("\n{prompt}? ");
        #[allow(unused_must_use)]
        {
            std::io::stdout().flush();
            std::io::stdin().read_line(&mut b);
        }
        let i = WholeNumber::parse_maybe(&b);
        match i {
            Some(WholeNumber::Number(x)) => return x,
            _ => warn_invalid_input(),
        }
    }
}

fn prompt_for_number_with_default(prompt: &str, default: i32) -> i32 {
    loop {
        let mut b = String::new();
        print!("\n{prompt} (enter={default})? ");
        #[allow(unused_must_use)]
        {
            std::io::stdout().flush();
            std::io::stdin().read_line(&mut b);
        }
        let i = WholeNumber::parse_maybe(&b);
        match i {
            Some(WholeNumber::Number(x)) => return x,
            Some(WholeNumber::Default) => return default,
            _ => warn_invalid_input(),
        }
    }
}
fn prompt_for_buysell(prompt: &str) -> BuySell {
    loop {
        let mut b = String::new();
        print!("\n{prompt} (buy #/sell #/enter=0)? ");
        #[allow(unused_must_use)]
        {
            std::io::stdout().flush();
            std::io::stdin().read_line(&mut b);
        }
        let i = BuySell::parse_maybe(&b);
        match i {
            None => warn_invalid_input(),
            Some(x) => return x,
        }
    }
}

fn roll_birth_fraction(rng: &mut ThreadRng) -> i32 {
    rng.gen_range(2..=5)
}

fn roll_land_price(rng: &mut ThreadRng) -> i32 {
    rng.gen_range(17..=26)
    // rng.generate_range(17..=27)
}

fn roll_harvest_yield(rng: &mut ThreadRng) -> i32 {
    rng.gen_range(1..=5)
}

fn roll_for_plague(rng: &mut ThreadRng) -> bool {
    let r = rng.gen_range(1..=100);
    r <= 15
}

fn roll_rats_fraction(rng: &mut ThreadRng) -> i32 {
    rng.gen_range(2..=6)
}

fn play_turn(rng: &mut ThreadRng, mut city: City) -> TurnResult {
    println!("Hammurabi, I beg to report to you, in year {},", city.year);
    if city.plague_victims > 0 {
        println!("A horrible plague struck! Half the people died.");
    }
    println!(
        "{} people starved, {} came to the city",
        city.starved, city.births
    );
    println!("The population is now {}", city.people);
    println!("The city owns {} acres of land", city.land);
    println!("You harvested {} bushels per acre", city.harvest_yield);
    println!("Rats ate {} bushels", city.rats_ate);
    println!("You now have {} bushels in store", city.grain);
    city.clear_events();
    // Trade land
    let land_price = roll_land_price(rng);
    println!("Land is trading at {land_price} bushels per acre");
    println!("grain: {} land: {}", city.grain, city.land);
    loop {
        let bs = prompt_for_buysell("How many acres do you wish to trade");
        match bs {
            BuySell::Buy(x) => {
                if x * land_price > city.grain {
                    warn_insufficient_grain(city.grain);
                    continue;
                }
                city.grain -= land_price * x;
                city.land += x;
                break;
            }
            BuySell::Sell(x) => {
                if x > city.land {
                    warn_insufficient_land(city.land);
                    continue;
                }
                city.grain += land_price * x;
                city.land -= x;
                break;
            }
            BuySell::Default => {
                break;
            }
        };
    }
    // Feed the people
    println!("grain: {} land: {}", city.grain, city.land);
    let feed = loop {
        let default = (city.people * 20).min(city.grain);
        let feed = prompt_for_number_with_default(
            "How many bushels do you wish to feed your people",
            default,
        );
        if feed <= city.grain {
            break feed;
        }
        warn_insufficient_grain(city.grain);
    };
    city.grain -= feed;
    // Plant the fields
    println!("grain: {} land: {}", city.grain, city.land);
    let plant = loop {
        let default = (city.people * 10).min(city.land).min(city.grain);
        let plant = prompt_for_number_with_default("How many acres do you wish to plant", default);
        if plant > city.land {
            warn_insufficient_land(city.land);
            continue;
        }
        if plant > city.people * 10 {
            warn_insufficient_labor(city.people);
            continue;
        }
        if plant > city.grain {
            warn_insufficient_grain(city.grain);
            continue;
        }
        break plant;
    };
    // Plant one bushel per acre
    city.grain -= plant;
    // Harvest
    city.harvest_yield = roll_harvest_yield(rng);
    city.grain += city.harvest_yield * plant;
    // Rats
    let r = roll_rats_fraction(rng);
    city.rats_ate = city.grain / r;
    city.grain -= city.rats_ate;
    // The people eat
    city.starved = (city.people - feed / 20).min(city.people);
    city.people -= city.starved;
    city.total_starved += city.starved;
    // Births
    let r = roll_birth_fraction(rng) as f32; // base birth rate
    let sf = 1.0 - city.starved as f32 / city.people as f32; // starvation factor
    let pf = 1.0 - city.people as f32 / 1000.0; // population factor
    let br = r * sf * pf;
    city.births = (city.people as f32 / br) as i32;
    city.people += city.births;
    city.total_births += city.births;
    // Plague
    if roll_for_plague(rng) {
        city.plague_victims = city.people / 2;
        city.people -= city.plague_victims;
        city.total_plague_deaths += city.plague_victims;
    }
    // Starve too many people?
    if city.starved as f32 > city.people as f32 * 0.45 {
        return TurnResult::Starved(city);
    }
    TurnResult::Continue(city)
}

fn play() {
    let mut rng = rand::thread_rng();
    let mut city = City {
        people: 100,
        land: 1000,
        grain: 2800,
        rats_ate: 200,
        births: 5,
        harvest_yield: 2,
        ..City::default()
    };
    for year in 1..=10 {
        city.year = year;
        let x = play_turn(&mut rng, city);
        println!();
        match x {
            TurnResult::Continue(x) => city = x,
            TurnResult::Starved(x) => {
                println!("You starved {} people in one year!", x.starved);
                eval_message_fink();
                return;
            }
            TurnResult::Quit(x) => {
                println!("You quit!");
                city = x;
                break;
            }
        }
    }
    eval_term_of_office(&mut rng, city);
}

fn eval_term_of_office(rng: &mut ThreadRng, city: City) {
    print!("You served {} ", city.year);
    if city.year > 1 {
        print!("years");
    } else {
        print!("year");
    }
    println!(" of your 10 year term of office.");

    println!("Under your rule,");
    let starved = city.total_starved;
    let avg_starved = starved / city.year;
    println!("{} came to the city,", city.total_births);
    println!("{starved} people starved ({avg_starved} people per year, on average),");
    println!(
        "and {} people succumbed to the plague.",
        city.total_plague_deaths
    );
    let avg_acres = city.land / city.people;
    println!(
        "The city began with 10 acres per person, and ended with {} acres per person.",
        avg_acres
    );
    if avg_starved > 33 {
        eval_message_fink();
        return;
    }
    if avg_starved > 10 || avg_acres < 9 {
        eval_message_heavy_handed();
        return;
    }
    if avg_starved > 3 {
        eval_message_fair(rng, city.people);
        return;
    }
    eval_message_good();
}

fn eval_message_fink() {
    println!("Due to this extreme mismanagement, not only have you been impeached and thrown out of office,");
    println!("but you have been declared 'National Fink'!!!");
}

fn eval_message_heavy_handed() {
    println!("Your heavy-handed performance smacks of Nero and Ivan IV.");
    println!("The (remaining) people find you an unpleasant ruler, and,");
    println!("frankly, hate your guts!");
}

fn eval_message_fair(rng: &mut ThreadRng, p: i32) {
    println!("Your performance could have been somewhat better, but really");
    println!("wasn't too bad at all.");
    let x: f32 = rng.gen::<f32>() / f32::MAX * 0.8 * (p as f32);
    println!(
        "{} people would like to see you assassinated, but we all have",
        x as i32
    );
    println!("our trivial problems.");
}

fn eval_message_good() {
    println!("A fantastic performance! Charlamagne, Disraeli, and Jefferson combined");
    println!("could not have done better!");
}

fn main() -> Result<(), Box<dyn Error>> {
    play();
    Ok(())
}
