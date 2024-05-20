use clap::{Arg, Command};
mod commands {
    pub mod ls;
    pub mod add;
    pub mod remove;
    pub mod import;
    pub mod default;
}
fn main() {
    let matches = Command::new("assetm")
        .version("1.0")
        .author("Halill Focic <halil.focic@gmail.com>")
        .about("Simple asset manager for projects")
        .subcommand(Command::new("ls").about("Lists all added projects"))
        .subcommand(
            Command::new("add")
                .about("Adds a project and its path to list of projects")
                .arg(Arg::new("project_name").required(true).index(1))
                .arg(Arg::new("path_to_project").required(true).index(2)),
        )
        .subcommand(
            Command::new("remove")
                .about("Removes a project")
                .arg(Arg::new("project_name").required(true).index(1)),
        )
        .subcommand(
            Command::new("default")
                .about("Sets the default path from where to pull the asset")
                .arg(Arg::new("path").required(true).index(1)),
        )
        .subcommand(
            Command::new("import")
                .about("Imports a file to a project")
                .arg(Arg::new("project_name").required(true).index(1))
                .arg(Arg::new("file_name").required(true).index(2)),
        )
        .get_matches();
    // if command is ls, use handle_ls from ls.rs
    if let Some(_) = matches.subcommand_matches("ls") {
        commands::ls::handle_ls();
    }
    if let Some(matches) = matches.subcommand_matches("add") {
        commands::add::handle_add();
    }
    if let Some(matches) = matches.subcommand_matches("remove") {
        commands::remove::handle_remove();
    }
    if let Some(matches) = matches.subcommand_matches("default") {
        commands::default::handle_default();
    }
    if let Some(matches) = matches.subcommand_matches("import") {
        commands::import::handle_import();
    }

}
