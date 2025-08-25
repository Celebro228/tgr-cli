use clap::{Parser, Subcommand};
use std::process::Command as Console;
use std::fs;



/*

TODO:
[#] Создание проекта
[?] Запуск проекта
[] Экспорт на винду
[] Экспорт на андроид
[] Экспорт на линукс

*/



#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    New {
        path: String,

        #[arg(short, long)]
        name: Option<String>,
    },
    Build {
        #[arg(short, long)]
        windows: bool,
    },
}


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { path, name } => {
            let mut cargo = Console::new("cargo");
            cargo.args(["new", &path]);

            let mut cargo_name = path.clone();
            if let Some(name) = name {
                cargo.args(["--name", &name]);
                cargo_name = name;
            }
            cargo.output().expect("Cargo new project error");

            let path = format!("{}/Cargo.toml", path);
            let toml = toml(&cargo_name);
            fs::write(path, toml).expect("Toml write error");

            println!("Project {} created!", cargo_name);
        }

        
        Commands::Build { windows } => {

        }
    }
}


fn toml(name: &str) -> String {
    format!(
                "[package]
name = \"{}\"
version = \"0.1.0\"
edition = \"2024\"

[dependencies]
tgr2 = \"1.*\"


# Optimizations
[profile.dev]
opt-level = 3

[profile.dev.package.\"*\"]
opt-level = 3

[profile.release]
lto = true
panic = \"abort\"
debug = false
opt-level = 3
overflow-checks = false
debug-assertions = false
incremental = false
rpath = false
codegen-units = 1
strip = true

[profile.release.package.\"*\"]
opt-level = 3",
                name
            )
}