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
        let mut app = App::new("area120tables1_alpha1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230130")
            .about("")
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
        let mut tables0 = SubCommand::with_name("tables")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets a table. Returns NOT_FOUND if the table does not exist.");
            tables0 = tables0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists tables for the user.");
            tables0 = tables0.subcommand(mcmd);
        }
        let mut workspaces0 = SubCommand::with_name("workspaces")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets a workspace. Returns NOT_FOUND if the workspace does not exist.");
            workspaces0 = workspaces0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists workspaces for the user.");
            workspaces0 = workspaces0.subcommand(mcmd);
        }
        let mut rows1 = SubCommand::with_name("rows")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: batch_create, batch_delete, batch_update, create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("batch_create").about("Creates multiple rows.");
            rows1 = rows1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_delete").about("Deletes multiple rows.");
            rows1 = rows1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_update").about("Updates multiple rows.");
            rows1 = rows1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a row.");
            rows1 = rows1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a row.");
            rows1 = rows1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets a row. Returns NOT_FOUND if the row does not exist in the table.");
            rows1 = rows1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists rows in a table. Returns NOT_FOUND if the table does not exist.");
            rows1 = rows1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a row.");
            rows1 = rows1.subcommand(mcmd);
        }
        tables0 = tables0.subcommand(rows1);
        app = app.subcommand(workspaces0);
        app = app.subcommand(tables0);

        Self { app }
    }
}
use google_area120tables1_alpha1 as api;

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
