use rand::RngExt;

pub const QUOTES: [&str; 10] = [
    "But first, coffee.",
    "Live, laugh, love.",
    "Too glam to give a damn.",
    "Good vibes only.",
    "Messy bun and getting stuff done.",
    "Stressed, blessed, and coffee obsessed.",
    "Sippin' on sunshine.",
    "You can't sip with us.",
    "Namast'ay in bed.",
    "Pumpkin spice and everything nice.",
];

pub fn get_random_quote() -> String {
    let rand_num: u32 = rand::rng().random();
    let rand_idx: usize = rand_num as usize % QUOTES.len();
    String::from(QUOTES[rand_idx])
}
