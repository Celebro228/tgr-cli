use clap::{Parser, Subcommand};



/*

TODO:
[] Создание проекта
[] Запуск проекта
[] Экспорт на винду
[] Экспорт на андроид
[] Экспорт на линукс

*/



#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand)]
enum Commands {
    Build {
        #[arg(short, long)]
        windows: bool
    },
    New {
        name: String,
    },
}


fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli.name);
}
