use clap::{App, AppSettings, Arg, ArgMatches};
use std::process::Command;
use std::process::Output;
use std::{fs, path::Path};
use termion::{color, style};

fn print_info<T: std::fmt::Display>(message: T) {
  println!(
    "{}{}info:{} {}",
    color::Fg(color::Blue),
    style::Bold,
    style::Reset,
    message
  );
}

fn print_error_and_exit<T: std::fmt::Display>(message: T) {
  println!(
    "{}{}error:{} {}",
    color::Fg(color::Red),
    style::Bold,
    style::Reset,
    message
  );
  std::process::exit(1);
}

fn create_new_project(project_name: &str) {
  if Path::new(project_name).exists() {
    print_error_and_exit(format!(
      "project {} already exists, use a different name.",
      project_name
    ));
  }
  println!("==> Creating new project, {}...", project_name);
  Command::new("/bin/sh")
    .arg("-c")
    .arg(format!(
      "git clone https://github.com/edenreich/rust-easyadmin.git {} && rm -rf ./{}/.git",
      project_name, project_name,
    ))
    .output()
    .expect("failed to execute process");
  print_info(format!(
    "project {} has been created. Run: cd {}.",
    project_name, project_name
  ));
  std::process::exit(0);
}

fn make_new_controller(options: &ArgMatches) {
  let controller_name = options.value_of("controller").unwrap();
  println!("==> Generating new controller, {}", controller_name);

  //@todo generate a new controller
}

fn make_new_model(options: &ArgMatches) {
  let model_name = options.value_of("model").unwrap();
  println!("==> Generating new model, {}", model_name);

  //@todo generate a new model
}

fn make_new_migration(options: &ArgMatches) {
  let migration_name = options.value_of("migration").unwrap();
  println!("==> Generating new migration, {}", migration_name);

  if !Path::new("database/migrations").exists() {
    match fs::create_dir_all("database/migrations") {
      Ok(_) => print_info("created database/migrations directory because it was not exists."),
      Err(_) => print_error_and_exit("could not create database/migrations directory."),
    }
  }

  let output: Output = Command::new("/bin/sh")
    .arg("-c")
    .arg(format!(
      "diesel migration --migration-dir database/migrations generate {}",
      migration_name,
    ))
    .output()
    .expect("failed to execute process");

  if !output.status.success() {
    print_error_and_exit(String::from_utf8(output.stderr).unwrap());
  }

  println!("{}", String::from_utf8(output.stdout).unwrap());
  std::process::exit(0);
}

fn main() {
  let matches = App::new("Rust EasyAdmin")
    .setting(AppSettings::ArgRequiredElseHelp)
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
