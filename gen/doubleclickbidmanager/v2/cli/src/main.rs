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
        let mut app = App::new("doubleclickbidmanager2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230124")
            .about("DoubleClick Bid Manager API allows users to manage and create campaigns and reports.")
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
        let mut queries0 = SubCommand::with_name("queries")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and run");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a query.");
            queries0 = queries0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes a query as well as the associated reports.");
            queries0 = queries0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a query.");
            queries0 = queries0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists queries created by the current user.");
            queries0 = queries0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("run").about("Runs a stored query to generate a report.");
            queries0 = queries0.subcommand(mcmd);
        }
        let mut reports1 = SubCommand::with_name("reports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a report.");
            reports1 = reports1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists reports associated with a query.");
            reports1 = reports1.subcommand(mcmd);
        }
        queries0 = queries0.subcommand(reports1);
        app = app.subcommand(queries0);

        Self { app }
    }
}
use google_doubleclickbidmanager2 as api;

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
