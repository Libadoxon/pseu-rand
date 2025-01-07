use clap::{Parser, Subcommand};
use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Seed to use
    seed: u64,
    /// Length of the output in characters/digits
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

fn gen_num(length: u8, mut rng: StdRng) -> u32 {
    let length = length as u32;
    let min = u32::pow(10, length - 1);
    let max = u32::pow(10, length) - 1;
    rng.gen_range(min..max)
}

fn gen_ascii(length: &u8, mut rng: StdRng) -> String {
    let mut result: Vec<u8> = Vec::new();
    for _ in 0..*length {
        result.push(rng.gen_range(32..=126))
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
        let num = rng.gen_range(0..=61);
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
