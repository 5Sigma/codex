mod server;

use anyhow::Result;
use core::Project;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug, Clone)]
enum RootCommands {
    /// Run a local webserver
    #[command()]
    Serve,
    /// Build a static version of the project
    #[command()]
    Build,
    /// Generate scaffolding for a new project
    #[command()]
    Init { path: String },
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Path to the root of the project
    #[arg(short, long, default_value = ".")]
    root_path: String,

    #[command(subcommand)]
    command: RootCommands,
}

fn main() {
    let args = Args::parse();
    match args.command {
        RootCommands::Serve => {
            server::serve(args.root_path);
        }
        RootCommands::Build => {
            if let Err(e) = build_project(args.root_path) {
                eprintln!("Error: {}", e);
            } else {
                println!("Build successful");
            }
        }
        RootCommands::Init { path } => {
            let p = PathBuf::from(path);
            if p.exists() {
                println!("Path already exists");
                std::process::exit(1);
            } else {
                if let Err(e) = std::fs::create_dir_all(&p) {
                    eprintln!("Couldn't create project path: {}", e);
                    std::process::exit(1);
                }

                let config_path = p.join("codex.yml");
                if let Err(e) =
                    std::fs::write(config_path, core::assets::get_bytes("scaffold_config.yml"))
                {
                    eprintln!("Couldn't create project configuration: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

fn build_project(root_path: String) -> Result<()> {
    let root_path = PathBuf::from(root_path);
    let project = Project::load(&root_path, false)?;
    let build_path = root_path.join("dist");
    if !build_path.exists() {
        std::fs::create_dir(&build_path)?;
    }

    build_folder(&project, &project.root_folder)?;

    for file in core::assets::static_files(&project)? {
        println!("Writing {}", file.root_url());
        file.write(
            &PathBuf::from(&project.details.build_path),
            PathBuf::from("static"),
        )?;
    }

    Ok(())
}

fn build_folder(project: &Project, folder: &core::Folder) -> Result<()> {
    for folder in folder.folders.iter() {
        build_folder(project, folder)?;
    }
    for document in folder.documents.iter() {
        build_document(project, document)?;
    }
    Ok(())
}

fn build_document(project: &Project, doc: &core::Document) -> Result<()> {
    let content = doc.page_content(project)?;
    let file_path = if doc.file_path.is_index() {
        doc.file_path
            .relative_to(
                &project
                    .path
                    .disk_path()
                    .join(PathBuf::from(project.details.build_path.clone())),
            )
            .with_extension("html")
    } else {
        doc.file_path
            .relative_to(
                &project
                    .path
                    .disk_path()
                    .join(PathBuf::from(project.details.build_path.clone())),
            )
            .with_extension("")
            .join("index.html")
    };

    if !file_path.parent().unwrap().exists() {
        std::fs::create_dir_all(file_path.parent().unwrap())?;
    }

    println!("Writing {}", file_path.display());
    std::fs::write(file_path, content)?;
    Ok(())
}
