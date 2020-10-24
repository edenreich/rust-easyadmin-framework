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

fn key_generate() {
  if !Path::new(".env").exists() {
    fs::copy(".env.dist", ".env").unwrap();
  }

  let output: Output = Command::new("/bin/sh")
    .arg("-c")
    .arg(format!("openssl rand -base64 32"))
    .output()
    .expect("failed to execute process");

  let base64string: String = String::from_utf8(output.stdout).unwrap();
  let sed_command: String = format!("sed -i -e 's/^ROCKET_SECRET_KEY=$/ROCKET_SECRET_KEY={}/g' .env", base64string.escape_default()); 

  Command::new("/bin/sh")
    .arg("-c")
    .arg(sed_command)
    .output()
    .expect("failed to execute process");

  print_info(format!("random base64 key has been generated.", base64string));
}

fn make_project(project_name: &str) {
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
    "project {} has been created. Run: cd {} && docker-compose up -d",
    project_name, project_name
  ));
  std::process::exit(0);
}

fn make_controller(options: &ArgMatches) {
  let controller_name = options.value_of("controller").unwrap();
  println!("==> Generating new controller, {}", controller_name);

  //@todo generate a new controller
}

fn make_model(options: &ArgMatches) {
  let model_name = options.value_of("model").unwrap();
  println!("==> Generating new model, {}", model_name);

  //@todo generate a new model
}

fn make_migration(options: &ArgMatches) {
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

fn run_migrations() {
  let output: Output = Command::new("/bin/sh")
    .arg("-c")
    .arg("diesel migration --migration-dir database/migrations run")
    .output()
    .expect("failed to execute process");

  if !output.status.success() {
    if output.stderr.is_empty() {
      print_error_and_exit(String::from_utf8(output.stdout.clone()).unwrap());
    } else {
      print_error_and_exit(String::from_utf8(output.stderr).unwrap());
    }
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
    .subcommand(App::new("key:generate").about("Generate a base64 key for the rocket server"))
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
    .subcommand(App::new("run:migrations").about("Run the migrations"))
    .get_matches();

  match matches.subcommand_name() {
    Some("make:project") => make_project(
      matches
        .subcommand_matches("make:project")
        .unwrap()
        .value_of("project")
        .unwrap(),
    ),
    Some("make:controller") => {
      make_controller(matches.subcommand_matches("make:controller").unwrap())
    }
    Some("make:model") => make_model(matches.subcommand_matches("make:model").unwrap()),
    Some("make:migration") => make_migration(matches.subcommand_matches("make:migration").unwrap()),
    Some("run:migrations") => run_migrations(),
    Some("key:generate") => key_generate(),
    None => println!("No subcommand was used"),
    _ => unreachable!(),
  }
}
