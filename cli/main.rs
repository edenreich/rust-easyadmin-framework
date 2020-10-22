use clap::{App, Arg, ArgMatches};
use std::process::Command;

fn create_new_project(project_name: &str) {
  println!("==> Creating new project, {}...", project_name);
  Command::new("/bin/sh")
    .arg("-c")
    .arg(format!(
      "git clone https://github.com/edenreich/rust-easyadmin.git {} && rm -rf ./{}/.git",
      project_name, project_name,
    ))
    .output()
    .expect("failed to execute process");
}

fn make_new_controller(options: &ArgMatches) {
  let controllerName = options.value_of("controller").unwrap();
  println!("==> Generating new controller, {}", controllerName);

  //@todo generate a new controller
}

fn make_new_model(options: &ArgMatches) {
  let modelName = options.value_of("model").unwrap();
  println!("==> Generating new model, {}", modelName);

  //@todo generate a new model
}

fn make_new_migration(options: &ArgMatches) {
  let migrationName = options.value_of("migration").unwrap();
  println!("==> Generating new migration, {}", migrationName);

  //@todo generate a new migration
}

fn main() {
  let matches = App::new("Rust EasyAdmin")
    .version("1.0")
    .author("Eden Reich <eden.reich@gmail.com>")
    .about("An admin panel made easy written in rust.")
    .subcommand(
      App::new("make:project").about("Create a new project").arg(
        Arg::new("project")
          .required(true)
          .about("The name of the project"),
      ),
    )
    .subcommand(
      App::new("make:controller")
        .about("Create a new controller")
        .arg(
          Arg::new("controller")
            .required(true)
            .about("The name of the controller"),
        )
        .arg("-c, --crud   'Create CRUD controller'"),
    )
    .subcommand(
      App::new("make:model").about("Create a new model").arg(
        Arg::new("model")
          .required(true)
          .about("The name of the model"),
      ),
    )
    .subcommand(
      App::new("make:migration")
        .about("Create a new migration")
        .arg(
          Arg::new("migration")
            .required(true)
            .about("The name of the migration"),
        ),
    )
    .get_matches();

  match matches.subcommand_name() {
    Some("make:project") => create_new_project(
      matches
        .subcommand_matches("make:project")
        .unwrap()
        .value_of("project")
        .unwrap(),
    ),
    Some("make:controller") => {
      make_new_controller(matches.subcommand_matches("make:controller").unwrap())
    }
    Some("make:model") => make_new_model(matches.subcommand_matches("make:model").unwrap()),
    Some("make:migration") => {
      make_new_migration(matches.subcommand_matches("make:migration").unwrap())
    }
    None => println!("No subcommand was used"),
    _ => unreachable!(),
  }
}
