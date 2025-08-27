use clap::{Parser, Subcommand};
use rand::{Rng, SeedableRng, rngs::StdRng};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Seed to use
    seed: u64,
    /// Length of the output in characters/digits (maximal 38)
    #[arg(value_parser = clap::value_parser!(u8).range(0..=38))]
    length: u8,
}

#[derive(Subcommand)]
enum Commands {
    /// Gives valid ASCII output minus the control characters
    Ascii {},
    /// Gives pseudo random output that only contains a-zA-Z0-9
    Text {},
    /// Gives a whole number as output
    Num {
        #[arg(long, action = clap::ArgAction::SetTrue)]
        hex: bool,
    },
}

fn gen_num(length: u8, mut rng: StdRng) -> u128 {
    let length = length as u32;

    let mut result: u128 = 0;

    for _ in 0..length {
        let rng_num = rng.random_range(0..9);
        result = result * 10 + rng_num as u128;
    }

    result
}

fn gen_ascii(length: &u8, mut rng: StdRng) -> String {
    let mut result: Vec<u8> = Vec::new();
    for _ in 0..*length {
        result.push(rng.random_range(32..=126))
    }
    String::from_utf8(result).expect("Couldn't encode char")
}

const VALID_CHARS: [char; 62] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9',
];

fn gen_text(length: &u8, mut rng: StdRng) -> String {
    let mut result: Vec<char> = Vec::new();
    for _ in 0..*length {
        let num = rng.random_range(0..=61);
        result.push(VALID_CHARS[num])
    }
    let result: String = result.into_iter().collect();
    result
}

fn main() {
    let cli = Cli::parse();
    let rng: StdRng = SeedableRng::seed_from_u64(cli.seed + cli.length as u64);
    let out = match &cli.command {
        Commands::Num { hex } => {
            let num = gen_num(cli.length, rng);
            if *hex {
                format!("{:02X}", num)
            } else {
                format!("{}", num)
            }
        }
        Commands::Text {} => gen_text(&cli.length, rng),
        Commands::Ascii {} => gen_ascii(&cli.length, rng),
    };
    print!("{}", out);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_num_1() {
        let rng = SeedableRng::seed_from_u64(787 + 5);
        assert_eq!(gen_num(5, rng), 84531)
    }

    #[test]
    fn test_gen_num_2() {
        let rng = SeedableRng::seed_from_u64(238094721521236520 + 11);
        assert_eq!(gen_num(11, rng), 43185815040)
    }

    #[test]
    fn test_gen_ascii_1() {
        let rng = SeedableRng::seed_from_u64(1337 + 13);
        assert_eq!(gen_ascii(&13, rng), "51h|dwTo,\\>|~")
    }

    #[test]
    fn test_gen_ascii_2() {
        let rng = SeedableRng::seed_from_u64(4721398401298347 + 2);
        assert_eq!(gen_ascii(&2, rng), "g^")
    }

    #[test]
    fn test_gen_text_1() {
        let rng = SeedableRng::seed_from_u64(44 + 8);
        assert_eq!(gen_text(&8, rng), "nmZnzCvv")
    }

    #[test]
    fn test_gen_text_2() {
        let rng = SeedableRng::seed_from_u64(1239214354653452417 + 21);
        assert_eq!(gen_text(&21, rng), "PxQgiWCYclxWMOJyzBDMe")
    }
}
