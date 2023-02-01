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
        let mut app = App::new("kmsinventory1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230129")
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
        let mut organizations0 = SubCommand::with_name("organizations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: protected_resources");
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: crypto_keys and locations");
        let mut protected_resources1 = SubCommand::with_name("protected_resources")
            .setting(AppSettings::ColoredHelp)
            .about("methods: search");
        {
            let mcmd = SubCommand::with_name("search").about("Returns metadata about the resources protected by the given Cloud KMS CryptoKey in the given Cloud organization.");
            protected_resources1 = protected_resources1.subcommand(mcmd);
        }
        let mut crypto_keys1 = SubCommand::with_name("crypto_keys")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Returns cryptographic keys managed by Cloud KMS in a given Cloud project. Note that this data is sourced from snapshots, meaning it may not completely reflect the actual state of key metadata at call time.");
            crypto_keys1 = crypto_keys1.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: key_rings");
        let mut key_rings2 = SubCommand::with_name("key_rings")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: crypto_keys");
        let mut crypto_keys3 = SubCommand::with_name("crypto_keys")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_protected_resources_summary");
        {
            let mcmd = SubCommand::with_name("get_protected_resources_summary").about("Returns aggregate information about the resources protected by the given Cloud KMS CryptoKey. Only resources within the same Cloud organization as the key will be returned. The project that holds the key must be part of an organization in order for this call to succeed.");
            crypto_keys3 = crypto_keys3.subcommand(mcmd);
        }
        key_rings2 = key_rings2.subcommand(crypto_keys3);
        locations1 = locations1.subcommand(key_rings2);
        projects0 = projects0.subcommand(locations1);
        projects0 = projects0.subcommand(crypto_keys1);
        organizations0 = organizations0.subcommand(protected_resources1);
        app = app.subcommand(projects0);
        app = app.subcommand(organizations0);

        Self { app }
    }
}
use google_kmsinventory1 as api;

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
