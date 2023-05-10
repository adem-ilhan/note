use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;
use chrono::{ Utc};

#[derive(Debug, StructOpt)]
#[structopt(name = "take-note", about = "A command-line note-taking app")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "add", about = "Add a new note")]
    Add {
        #[structopt(short, long, help = "Title of the note")]
        title: String,
        #[structopt(help = "Content of the note")]
        content: String,
    },
    #[structopt(name = "list", about = "List all notes")]
    List,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    match opt.cmd {
        Command::Add { title, content } => add_note(title, content),
        Command::List => list_notes(),
    }
}

fn add_note(title: String, content: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(notes_file())?;

    let timestamp = Utc::now();
    let note = format!("{}\n{}\n{}\n\n", title, content, timestamp);
    file.write_all(note.as_bytes())?;

    // println!("Note added successfully.");

    Ok(())
}

fn list_notes() -> std::io::Result<()> {
    let mut file = OpenOptions::new().read(true).open(notes_file())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    if contents.is_empty() {
        println!("No notes found.");
    } else {
        println!("{}", contents);
    }

    Ok(())
}

fn notes_file() -> PathBuf {
    dirs::home_dir().unwrap().join(".notes.txt")
}
