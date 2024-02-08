mod server;

use anyhow::Result;
use console::style;
use core::{assets::EmbeddedAsset, HtmlRenderer, LatexRenderer, Project, Renderer};
use human_repr::{HumanCount, HumanDuration};
use std::{io::Write, path::PathBuf};

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
    /// The web server will automatically re-scan the project folder and rebuild
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

    /// Generate a LaTeX document
    #[command()]
    Latex,
}

/// Custom styles for clap
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
        RootCommands::Serve { .. } => handle_command(server::serve),
        RootCommands::Build => handle_command(command_build),
        RootCommands::Init { .. } => handle_command(command_init),
        RootCommands::Eject => handle_command(eject_static_files),
        RootCommands::Latex => handle_command(command_latex),
    }
}

fn command_init(args: &Args) -> Result<()> {
    let RootCommands::Init { path } = &args.command else {
        return Err(anyhow::anyhow!("Invalid command"));
    };

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
        std::fs::write(
            config_path,
            core::assets::get_bytes("_internal/templates/scaffold_config.yml"),
        )?;
    }
    Ok(())
}

/// Cinvience function to handle commands
fn handle_command(f: impl Fn(&Args) -> Result<()>) {
    let args = Args::parse();
    if let Err(e) = f(&args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

/// internal command to generate a LaTeX document
pub fn command_latex(args: &Args) -> Result<()> {
    let now = std::time::Instant::now();
    let root_path = PathBuf::from(&args.root_path);
    let project = Project::load(root_path, false)?;
    let latex = build_latext(&project)?;

    let build_path = project.path.disk_path().join("dist");
    if !build_path.exists() {
        std::fs::create_dir(&build_path)?;
    }

    let mut f = std::fs::File::create(build_path.join("main.tex"))?;
    let size = latex.len();
    f.write_all(latex.as_bytes())?;
    print_file_built("main.tex", size, now.elapsed());
    Ok(())
}

/// internal command to build the site
fn command_build(args: &Args) -> Result<()> {
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
        let static_now = std::time::Instant::now();
        let static_size = file.write(
            &PathBuf::from(&project.details.build_path),
            PathBuf::from("static"),
        )?;
        if args.verbose {
            print_file_built(
                file.disk_path().file_name().unwrap().to_str().unwrap(),
                static_size,
                static_now.elapsed(),
            );
        }
        total_static_size += static_size;
    }
    let static_time = now.elapsed();

    print_file_built(&format!("{} documents", doc_count.0), doc_count.1, doc_time);
    print_file_built(
        &format!("{} static_files", static_count),
        total_static_size,
        static_time,
    );

    Ok(())
}

/// Eject the static files from the binary
fn eject_static_files(args: &Args) -> Result<()> {
    let root_path = PathBuf::from(&args.root_path);
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

/// Build static site files for a folder and all its sub folders and documents.
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

/// Build static site files for a document.
fn build_document(args: &Args, project: &Project, doc: &core::Document) -> Result<usize> {
    let now = std::time::Instant::now();
    let renderer = HtmlRenderer {
        render_context: core::RenderContext {
            project,
            document: doc,
        },
    };
    let content = renderer.render()?;
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
        print_file_built(
            file_path
                .parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
            l,
            now.elapsed(),
        );
    }
    std::fs::write(file_path, content)?;
    Ok(l)
}

/// Build a LaTeX document from the project
fn build_latext(project: &Project) -> Result<String> {
    let mut output = String::new();
    for document in project
        .root_folder
        .iter_all_documents()
        .filter(|d| !d.frontmatter.pdf_exclude)
    {
        let renderer = LatexRenderer {
            render_context: core::RenderContext { project, document },
        };
        let res = renderer.render()?;

        let slug = renderer.slug(&document.url.trim_matches('/').replace(['/', '#', '_'], "-"));

        if let Some(subtitle) = &document.frontmatter.subtitle {
            output.push_str(&format!(
                "\\section[{}]{{{}{{\\hfill\\normalsize\\color{{subtitle}} {}}}}}\\label{{sec:{}}}\n",
                document.frontmatter.title, document.frontmatter.title, subtitle, slug
            ));
        } else {
            output.push_str(&format!(
                "\\section{{{}}}\\label{{sec:{}}}\n",
                document.frontmatter.title, slug
            ));
        }
        output.push_str(&res);
        output.push_str("\\pagebreak\n");
    }

    let prelude = String::from_utf8(
        project
            .path
            .new_path("_internal/templates/prelude.tex")
            .read()?
            .to_vec(),
    )?
    .replace(
        "--AUTHOR--",
        &project.details.author.clone().unwrap_or_default(),
    )
    .replace(
        "--TITLE--",
        &format!("\\title{{{}}}", &project.details.name),
    );

    let mut buffer = String::new();

    buffer.push_str(&prelude);
    buffer.push_str(&output);
    buffer.push_str("\\end{document}");

    Ok(buffer)
}

/// Generate styled string for size and time
fn size_in_time(size: usize, time: std::time::Duration) -> String {
    let mut out = String::new();
    out.push_str(&style(" in ").dim().to_string());
    out.push_str(&style(time.human_duration()).bold().to_string());
    out.push_str(&style(" [").dim().to_string());
    out.push_str(&style(size.human_count_bytes()).bold().to_string());
    out.push_str(&style("] ").dim().to_string());
    out
}

/// Print a file built message
fn print_file_built(s: &str, size: usize, time: std::time::Duration) {
    let mut out = String::new();
    out.push_str(&style("Built ").dim().to_string());
    out.push_str(&style(s).bold().to_string());
    out.push_str(&size_in_time(size, time));
    println!("{}", out);
}
