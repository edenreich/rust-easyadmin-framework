use clap::{App, Arg};

fn main() {
  let matches = App::new("Rust EasyAdmin")
    .version("1.0")
    .author("Eden Reich <eden.reich@gmail.com>")
    .about("An admin panel made easy written in rust.")
    .subcommand(
      App::new("new-project").about("Create a new project").arg(
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
      App::new("make:migration").about("Create a new migration").arg(
        Arg::new("migration")
          .required(true)
          .about("The name of the migration"),
      ),
    )
    .get_matches();

  match matches.subcommand_name() {
    Some("new-project") => println!("==> creating new project.."),
    Some("make:controller") => println!("==> creating new controller.."),
    Some("make:model") => println!("==> creating new model.."),
    Some("make:migration") => println!("==> creating new migration.."),
    None => println!("No subcommand was used"),
    _ => unreachable!(),
  }
}
