use bevy::prelude::*;
use clap::Parser;
use rogue::{core::CorePlugins, inspectors::InspectorPlugins, placeholders::placeholder_plugin};

#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
struct Args {
    /// Start in fullscreen mode.
    #[arg(short, long)]
    fullscreen: bool,
}

fn main() {
    let args = Args::parse();

    App::new()
        .add_plugins(CorePlugins {
            fullscreen: args.fullscreen,
        })
        .add_plugins(placeholder_plugin)
        .add_plugins(InspectorPlugins)
        .run();
}
