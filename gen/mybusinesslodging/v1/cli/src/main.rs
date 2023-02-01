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
        let mut app = App::new("mybusinesslodging1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230131")
            .about("The My Business Lodging API enables managing lodging business information on Google. Note - If you have a quota of 0 after enabling the API, please request for GBP API access.")
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
            .about("methods: get_lodging and update_lodging");
        {
            let mcmd = SubCommand::with_name("get_lodging")
                .about("Returns the Lodging of a specific location.");
            locations0 = locations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_lodging")
                .about("Updates the Lodging of a specific location.");
            locations0 = locations0.subcommand(mcmd);
        }
        let mut lodging1 = SubCommand::with_name("lodging")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_google_updated");
        {
            let mcmd = SubCommand::with_name("get_google_updated")
                .about("Returns the Google updated Lodging of a specific location.");
            lodging1 = lodging1.subcommand(mcmd);
        }
        locations0 = locations0.subcommand(lodging1);
        app = app.subcommand(locations0);

        Self { app }
    }
}
use google_mybusinesslodging1 as api;

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
