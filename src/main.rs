mod server;

use anyhow::Result;
use core::Project;
use std::path::{Path, PathBuf};

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
    let project = Project::load(&root_path)?;
    let build_path = root_path.join("dist");
    if !build_path.exists() {
        std::fs::create_dir(&build_path)?;
    }

    build_folder(&project, &project.root_folder)?;

    for file in core::assets::static_files() {
        let pb = PathBuf::from(&*file);
        let path = pb.strip_prefix("static").unwrap();

        if let Some(parent) = path.parent() {
            let parent_path = project
                .path
                .join(project.details.build_path.clone())
                .join(parent);
            if !parent_path.exists() {
                std::fs::create_dir_all(parent_path)?;
            }
        }
        let file_path = project
            .path
            .join(project.details.build_path.clone())
            .join(path);
        println!("Writing {}", file_path.display());
        std::fs::write(file_path, core::assets::get_bytes(&file))?;
    }

    Ok(())
}

fn build_folder(project: &Project, folder: &core::Folder) -> Result<()> {
    for folder in folder.folders.iter() {
        build_folder(project, folder)?;
    }
    for document in folder.documents.iter() {
        build_document(&folder.path, project, document)?;
    }
    Ok(())
}

fn build_document(path: &Path, project: &Project, doc: &core::Document) -> Result<()> {
    let content = doc.page_content(project, core::ContentMode::Build)?;
    let file_path = if doc
        .file_path
        .file_stem()
        .map(|s| s == "index")
        .unwrap_or(false)
    {
        PathBuf::from(&project.path)
            .join(PathBuf::from(&project.details.build_path))
            .join(path.strip_prefix(&project.root_folder.path)?)
            .join("index.html")
    } else {
        PathBuf::from(&project.path)
            .join(PathBuf::from(&project.details.build_path))
            .join(path.strip_prefix(&project.root_folder.path)?)
            .join(doc.file_path.file_stem().unwrap())
            .join("index.html")
    };

    if !file_path.parent().unwrap().exists() {
        std::fs::create_dir_all(file_path.parent().unwrap())?;
    }

    println!("Writing {}", file_path.display());
    std::fs::write(file_path, content)?;
    Ok(())
}
