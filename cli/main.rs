use clap::{App, Arg, ArgMatches};
use std::process::Command;

fn create_new_project(project_name: &str) {
  println!("==> Creating new project, {}...", project_name);
  Command::new("/bin/sh")
    .arg("-c")
    .arg(format!("git clone https://github.com/edenreich/rust-easyadmin.git {}", project_name))
    .output()
    .expect("failed to execute process");

  // @todo add a basic project structure, something like:
  // app/
  //  ├── bootstrap.rs
  // database/
  //  ├── migrations/
  // routes/
  //  ├── admin.rs
  //  ├── frontend.rs
  // controllers/
  //  ├── admin
  //  ├── frontend
  // models/
  // public/
  // views/
  //  ├── admin/
  //    ├── index.html // maybe use https://crates.io/crates/handlebars
  //  ├── frontend/
  //    ├── index.html
}

fn make_new_controller(options: &ArgMatches) {
  println!(
    "==> Generating new controller, {}",
    options.value_of("controller").unwrap()
  );
}

fn make_new_model(options: &ArgMatches) {
  println!(
    "==> Generating new model, {}",
    options.value_of("model").unwrap()
  );
}

fn make_new_migration(options: &ArgMatches) {
  println!(
    "==> Generating new migration, {}",
    options.value_of("migration").unwrap()
  );
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
