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
        let mut app = App::new("analyticshub1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230123")
            .about("Exchange data and analytics assets securely and efficiently.")
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
        let mut organizations0 = SubCommand::with_name("organizations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: data_exchanges");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: data_exchanges");
        let mut data_exchanges2 = SubCommand::with_name("data_exchanges")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists all data exchanges from projects in a given organization and location.",
            );
            data_exchanges2 = data_exchanges2.subcommand(mcmd);
        }
        let mut data_exchanges2 = SubCommand::with_name("data_exchanges")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new data exchange.");
            data_exchanges2 = data_exchanges2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an existing data exchange.");
            data_exchanges2 = data_exchanges2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the details of a data exchange.");
            data_exchanges2 = data_exchanges2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the IAM policy.");
            data_exchanges2 = data_exchanges2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all data exchanges in a given project and location.");
            data_exchanges2 = data_exchanges2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing data exchange.");
            data_exchanges2 = data_exchanges2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the IAM policy.");
            data_exchanges2 = data_exchanges2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Returns the permissions that a caller has.");
            data_exchanges2 = data_exchanges2.subcommand(mcmd);
        }
        let mut listings3 = SubCommand::with_name("listings")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy, subscribe and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new listing.");
            listings3 = listings3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a listing.");
            listings3 = listings3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the details of a listing.");
            listings3 = listings3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the IAM policy.");
            listings3 = listings3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all listings in a given project and location.");
            listings3 = listings3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing listing.");
            listings3 = listings3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the IAM policy.");
            listings3 = listings3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("subscribe").about("Subscribes to a listing. Currently, with Analytics Hub, you can create listings that reference only BigQuery datasets. Upon subscription to a listing for a BigQuery dataset, Analytics Hub creates a linked dataset in the subscriber's project.");
            listings3 = listings3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions")
                .about("Returns the permissions that a caller has.");
            listings3 = listings3.subcommand(mcmd);
        }
        data_exchanges2 = data_exchanges2.subcommand(listings3);
        locations1 = locations1.subcommand(data_exchanges2);
        locations1 = locations1.subcommand(data_exchanges2);
        projects0 = projects0.subcommand(locations1);
        organizations0 = organizations0.subcommand(locations1);
        app = app.subcommand(projects0);
        app = app.subcommand(organizations0);

        Self { app }
    }
}
use google_analyticshub1 as api;

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
