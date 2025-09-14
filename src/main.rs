use bevy::prelude::*;
use clap::Parser;
#[cfg(feature = "dev")]
use rogue::inspectors::InspectorPlugins;
use rogue::{app::AppPlugins, placeholders::placeholder_plugin};

#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
struct Args {
    /// Start in fullscreen mode.
    #[arg(short, long)]
    fullscreen: bool,
}

fn main() {
    let args = Args::parse();

    let mut app = App::new();

    app.add_plugins(AppPlugins {
        fullscreen: args.fullscreen,
    })
    .add_plugins(placeholder_plugin);

    #[cfg(feature = "dev")]
    app.add_plugins(InspectorPlugins);

    app.run();
}
