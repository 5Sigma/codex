mod server;

use anyhow::Result;
use console::style;
use core::{assets::EmbeddedAsset, Project};
use human_repr::{HumanCount, HumanDuration};
use std::path::PathBuf;

use clap::{
    builder::{
        styling::{AnsiColor, Effects},
        Styles,
    },
    Parser, Subcommand,
};

#[derive(Subcommand, Debug, Clone)]
enum RootCommands {
    /// The serve command starts a local web server to preview the site
    ///
    /// The web server will automatically rescan the project folder and rebuild
    /// the site with every request.
    #[command()]
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
    /// Build a static version of the site
    ///
    /// The static site will be placed in the `dist` folder in the project root.
    /// This folder can be served by any web server.
    ///
    /// It is recommended to use a continuous deployment system to automatically
    /// build and deploy the site, using this command.
    #[command()]
    Build,
    /// Generate scaffolding for a new project.
    ///
    /// This will create a new folder with a basic configuration file.
    /// The configuration file can be customized to your needs.
    ///
    /// The specified path must not exist when running this command. It will be
    /// created and populated with the necessary files.
    ///
    /// To completely customize the site see the `eject` command.
    ///
    #[command()]
    Init { path: String },
    /// Eject the static files from the binary
    ///
    /// This will extract all template files needed to build the site into the
    /// project. These files can then be customized in any way.
    #[command()]
    Eject,
}

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Red.on_default() | Effects::BOLD)
        .usage(AnsiColor::Red.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Green.on_default())
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, styles=styles())]
pub struct Args {
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
        RootCommands::Serve { .. } => {
            server::serve(&args);
        }
        RootCommands::Build => {
            if let Err(e) = build_project(&args) {
                eprintln!("Error: {}", e);
            } else {
                println!("{}", style("Build complete").bold().green());
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
                if let Err(e) = std::fs::write(
                    config_path,
                    core::assets::get_bytes("_internal/templates/scaffold_config.yml"),
                ) {
                    eprintln!("Couldn't create project configuration: {}", e);
                    std::process::exit(1);
                }
            }
        }
        RootCommands::Eject => {
            if let Err(e) = eject_static_files(args.root_path) {
                eprintln!("Error: {}", e);
            } else {
                println!("Eject successful");
            }
        }
    }
}

/// Eject the static files from the binary
fn eject_static_files(root_path: String) -> Result<()> {
    let root_path = PathBuf::from(root_path);
    for f_path in EmbeddedAsset::iter().filter(|f| f != "_internal/templates/scaffold_config.yml") {
        let file = EmbeddedAsset::get(&f_path).unwrap();
        let file_path = root_path.join(&*f_path);
        if let Some(parent) = file_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
        println!("Writing {}", file_path.display());
        std::fs::write(file_path, file.data)?;
    }

    Ok(())
}

fn build_project(args: &Args) -> Result<()> {
    let root_path = PathBuf::from(&args.root_path);
    let project = Project::load(&root_path, false)?;
    let build_path = root_path.join("dist");
    if !build_path.exists() {
        std::fs::create_dir(&build_path)?;
    }

    let now = std::time::Instant::now();
    let doc_count = build_folder(args, &project, &project.root_folder)?;
    let doc_time = now.elapsed();
    let now = std::time::Instant::now();
    let mut static_count = 0;
    let mut total_static_size = 0;
    for file in core::assets::static_files(&project)? {
        static_count += 1;
        let static_size = file.write(
            &PathBuf::from(&project.details.build_path),
            PathBuf::from("static"),
        )?;
        if args.verbose {
            println!(
                "Built {} [{}]",
                file.root_url(),
                static_size.human_count_bytes()
            );
        }
        total_static_size += static_size;
    }
    let static_time = now.elapsed();

    if args.verbose {
        println!("\n");
    }
    let mut out = String::new();
    out.push_str(&style("Built ").dim().to_string());
    out.push_str(
        &style(doc_count.0.human_count(" documents"))
            .bold()
            .to_string(),
    );
    out.push_str(&style(" in ").dim().to_string());
    out.push_str(&style(doc_time.human_duration()).bold().to_string());
    out.push_str(&style(" [").dim().to_string());
    out.push_str(&style(doc_count.1.human_count_bytes()).bold().to_string());
    out.push_str(&style("] ").dim().to_string());
    println!("{}", out);

    let mut out = String::new();
    out.push_str(&style("Built ").dim().to_string());
    out.push_str(
        &style(static_count.human_count(" static files"))
            .bold()
            .to_string(),
    );
    out.push_str(&style(" in ").dim().to_string());
    out.push_str(&style(static_time.human_duration()).bold().to_string());
    out.push_str(&style(" [").dim().to_string());
    out.push_str(
        &style(total_static_size.human_count_bytes())
            .bold()
            .to_string(),
    );
    out.push_str(&style("] ").dim().to_string());
    println!("{}", out);

    Ok(())
}

fn build_folder(args: &Args, project: &Project, folder: &core::Folder) -> Result<(usize, usize)> {
    let mut count = 0;
    let mut size = 0;
    for folder in folder.folders.iter() {
        let (c, s) = build_folder(args, project, folder)?;
        count += c;
        size += s;
    }
    for document in folder.documents.iter() {
        count += 1;
        size += build_document(args, project, document)?;
    }
    Ok((count, size))
}

fn build_document(args: &Args, project: &Project, doc: &core::Document) -> Result<usize> {
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

    let l = content.len();
    if args.verbose {
        println!("Built {} [{}]", file_path.display(), l.human_count_bytes());
    }
    std::fs::write(file_path, content)?;
    Ok(l)
}
