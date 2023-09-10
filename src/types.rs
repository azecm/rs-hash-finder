use clap_derive::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'N', long, default_value = "1")]
    pub numbers: usize,
    #[arg(short = 'F', long, default_value = "1")]
    pub find: usize,
}