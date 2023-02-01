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
        let mut app = App::new("firebasestorage1_beta")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230120")
            .about("The Cloud Storage for Firebase API enables programmatic management of Cloud Storage buckets for use in Firebase projects")
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
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: buckets");
        let mut buckets1 = SubCommand::with_name("buckets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: add_firebase, get, list and remove_firebase");
        {
            let mcmd = SubCommand::with_name("add_firebase")
                .about("Links a Google Cloud Storage bucket to a Firebase project.");
            buckets1 = buckets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a single linked storage bucket.");
            buckets1 = buckets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the linked storage buckets for a project.");
            buckets1 = buckets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_firebase")
                .about("Unlinks a linked Google Cloud Storage bucket from a Firebase project.");
            buckets1 = buckets1.subcommand(mcmd);
        }
        projects0 = projects0.subcommand(buckets1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_firebasestorage1_beta as api;

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
