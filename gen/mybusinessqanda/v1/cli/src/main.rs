use clap::{App, AppSettings, Arg, SubCommand};
use default_boxed::DefaultBoxed;

#[derive(DefaultBoxed)]
struct Outer<'a, 'b> {
    inner: HeapApp<'a, 'b>,
}

struct HeapApp<'a, 'b> {
    app: App<'a, 'b>,
}

impl<'a, 'b> Default for HeapApp<'a, 'b> {
    fn default() -> Self {
        let mut app = App::new("mybusinessqanda1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20220426")
            .about("The My Business Q&A API allows questions and answers to be posted for specific listings.")
            .after_help("All documentation details can be found at <TODO figure out URL>")
            .arg(Arg::with_name("scope")
                .long("scope")
                .help("Specify the authentication method should be executed in. Each scope requires the user to grant this application permission to use it. If unset, it defaults to the shortest scope url for a particular method.")
                .multiple(true)
                .takes_value(true))
            .arg(Arg::with_name("folder")
                .long("config-dir")
                .help("A directory into which we will store our persistent data. Defaults to a user-writable directory that we will create during the first invocation." )
                .multiple(false)
                .takes_value(true))
            .arg(Arg::with_name("debug")
                .long("debug")
                .help("Provide more output to aid with debugging")
                .multiple(false)
                .takes_value(false));
        let mut locations0 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: questions");
        let mut questions1 = SubCommand::with_name("questions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Adds a question for the specified location.");
            questions1 = questions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes a specific question written by the current user.");
            questions1 = questions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns the paginated list of questions and some of its answers for a specified location. This operation is only valid if the specified location is verified.");
            questions1 = questions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates a specific question written by the current user.");
            questions1 = questions1.subcommand(mcmd);
        }
        let mut answers2 = SubCommand::with_name("answers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, list and upsert");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the answer written by the current user to a question.");
            answers2 = answers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the paginated list of answers for a specified question.");
            answers2 = answers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("upsert").about("Creates an answer or updates the existing answer written by the user for the specified question. A user can only create one answer per question.");
            answers2 = answers2.subcommand(mcmd);
        }
        questions1 = questions1.subcommand(answers2);
        locations0 = locations0.subcommand(questions1);
        app = app.subcommand(locations0);

        Self { app }
    }
}
use google_mybusinessqanda1 as api;

fn main() {
    // TODO: set homedir afterwards, once the address is unmovable, or use Pin for the very first time
    // to allow a self-referential structure :D!
    let _home_dir = dirs::config_dir()
        .expect("configuration directory can be obtained")
        .join("google-service-cli");
    let outer = Outer::default_boxed();
    let app = outer.inner.app;
    let _matches = app.get_matches();
}
