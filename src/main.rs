use clap::{Parser,Subcommand};
use inquire::Select;

fn main() {
    let cli = Cli::parse();

    match cli.command {
      Command::Add { description } => println!("Description: {:?}", description),
      Command::List => println!("list"),
      Command::Done { id: Some(id) } => println!("Marking as done {:?}", id),
      Command::Done { id: _ } => {
        let choice = Select::new("Gimme:", vec!["a", "b", "c"]).prompt().unwrap();
        println!("Choice is {}", choice)
      }
    }
}

#[derive(Parser)]
#[command(name = "task")]
#[command(about = "Cli for interacting with linear", long_about = None)]
struct Cli {
  #[command(subcommand)]
  command: Command
}

#[derive(Subcommand)]
enum Command {
  Add {
    #[arg(short, long)]
    description: Option<String>
  },
  List,
  Done {
    id: Option<usize>
  }
}
