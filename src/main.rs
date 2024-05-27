use clap::{Arg, Command};
mod commands {
    pub mod add;
    pub mod default;
    pub mod import;
    pub mod ls;
    pub mod remove;
}
mod project_state;
fn main() {
    let matches = Command::new("assetm")
        .version("1.0")
        .author("Halill Focic <halil.focic@gmail.com>")
        .about("Simple asset manager for projects")
        .subcommand(Command::new("ls").about("Lists all added projects"))
        .subcommand(
            Command::new("add")
                .about("Adds a project and its path to list of projects")
                .arg(Arg::new("project_name").required(true).index(1)),
        )
        .subcommand(
            Command::new("remove")
                .about("Removes a project")
                .arg(Arg::new("project_name").required(true).index(1)),
        )
        .subcommand(
            Command::new("set-default").about("Sets the default path from where to pull the asset"),
        )
        .subcommand(
            Command::new("import")
                .about("Imports a file to a project")
                .arg(Arg::new("project_name").required(true).index(1))
                .arg(Arg::new("file_name").required(true).index(2)),
        )
        .subcommand(Command::new("default").about("Shows the default path for assets"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("ls") {
        commands::ls::handle_ls();
    }
    if let Some(add_match) = matches.subcommand_matches("add") {
        let project_name = add_match.get_one::<String>("project_name").unwrap();
        commands::add::handle_add(&project_name);
    }
    if let Some(remove_match) = matches.subcommand_matches("remove") {
        let project_name = remove_match.get_one::<String>("project_name").unwrap();
        commands::remove::handle_remove(&project_name);
    }
    if let Some(_) = matches.subcommand_matches("set-default") {
        commands::default::handle_default();
    }
    if let Some(_) = matches.subcommand_matches("default") {
        commands::default::print_default();
    }
    if let Some(import_match) = matches.subcommand_matches("import") {
        let project_name = import_match.get_one::<String>("project_name").unwrap();
        let file_name = import_match.get_one::<String>("file_name").unwrap();
        commands::import::handle_import(&project_name, &file_name);
    }
}
