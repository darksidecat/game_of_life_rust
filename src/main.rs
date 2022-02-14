use clap::Parser;

mod game_of_life;
mod game_render;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    /// Game field width
    #[clap(short, long, default_value_t = 120_i32)]
    width: i32,
    /// Game field height
    #[clap(short, long, default_value_t = 40_i32)]
    height: i32,
    /// Sleep time before screen update, in milliseconds
    #[clap(short, long, default_value_t = 30_u64)]
    sleep_time: u64,
    /// Initial field filling percent
    #[clap(short, long, default_value_t = 30_u8)]
    filling: u8,
}


fn main() {
    let args = Cli::parse();

    let mut game = game_of_life::GameField::new(args.width, args.height);
    game.fill_random(args.filling);

    game_render::prepare_console();
    loop {
        game_render::render_field(&mut game, args.sleep_time);
    }
}
