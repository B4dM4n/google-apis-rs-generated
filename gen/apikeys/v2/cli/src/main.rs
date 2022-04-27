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
        let mut app = App::new("apikeys2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20220421")
            .about("Manages the API keys associated with developer projects.")
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
        let mut keys0 = SubCommand::with_name("keys")
            .setting(AppSettings::ColoredHelp)
            .about("methods: lookup_key");
        {
            let mcmd = SubCommand::with_name("lookup_key").about("Find the parent project and resource name of the API key that matches the key string in the request. If the API key has been purged, resource name will not be set. The service account must have the `apikeys.keys.lookup` permission on the parent project.");
            keys0 = keys0.subcommand(mcmd);
        }
        let mut operations0 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: keys");
        let mut keys2 = SubCommand::with_name("keys")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, get_key_string, list, patch and undelete");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new API key. NOTE: Key is a global resource; hence the only supported value for location is `global`.");
            keys2 = keys2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an API key. Deleted key can be retrieved within 30 days of deletion. Afterward, key will be purged from the project. NOTE: Key is a global resource; hence the only supported value for location is `global`.");
            keys2 = keys2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the metadata for an API key. The key string of the API key isn't included in the response. NOTE: Key is a global resource; hence the only supported value for location is `global`.");
            keys2 = keys2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_key_string").about("Get the key string for an API key. NOTE: Key is a global resource; hence the only supported value for location is `global`.");
            keys2 = keys2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the API keys owned by a project. The key string of the API key isn't included in the response. NOTE: Key is a global resource; hence the only supported value for location is `global`.");
            keys2 = keys2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Patches the modifiable fields of an API key. The key string of the API key isn't included in the response. NOTE: Key is a global resource; hence the only supported value for location is `global`.");
            keys2 = keys2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undelete").about("Undeletes an API key which was deleted within 30 days. NOTE: Key is a global resource; hence the only supported value for location is `global`.");
            keys2 = keys2.subcommand(mcmd);
        }
        locations1 = locations1.subcommand(keys2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);
        app = app.subcommand(operations0);
        app = app.subcommand(keys0);

        Self { app }
    }
}
use google_apikeys2 as api;

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
