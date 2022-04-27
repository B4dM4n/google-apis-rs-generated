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
        let mut app = App::new("playdeveloperreporting1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20220425")
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
        let mut anomalies0 = SubCommand::with_name("anomalies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists anomalies in any of the datasets.");
            anomalies0 = anomalies0.subcommand(mcmd);
        }
        let mut vitals0 = SubCommand::with_name("vitals")
                        .setting(AppSettings::ColoredHelp)
                        .about("sub-resources: anrrate, crashrate, excessivewakeuprate and stuckbackgroundwakelockrate");
        let mut anrrate1 = SubCommand::with_name("anrrate")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and query");
        {
            let mcmd =
                SubCommand::with_name("get").about("Describes the properties of the metric set.");
            anrrate1 = anrrate1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("query").about("Queries the metrics in the metric set.");
            anrrate1 = anrrate1.subcommand(mcmd);
        }
        let mut crashrate1 = SubCommand::with_name("crashrate")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and query");
        {
            let mcmd =
                SubCommand::with_name("get").about("Describes the properties of the metric set.");
            crashrate1 = crashrate1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("query").about("Queries the metrics in the metric set.");
            crashrate1 = crashrate1.subcommand(mcmd);
        }
        let mut excessivewakeuprate1 = SubCommand::with_name("excessivewakeuprate")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and query");
        {
            let mcmd =
                SubCommand::with_name("get").about("Describes the properties of the metric set.");
            excessivewakeuprate1 = excessivewakeuprate1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("query").about("Queries the metrics in the metric set.");
            excessivewakeuprate1 = excessivewakeuprate1.subcommand(mcmd);
        }
        let mut stuckbackgroundwakelockrate1 = SubCommand::with_name("stuckbackgroundwakelockrate")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and query");
        {
            let mcmd =
                SubCommand::with_name("get").about("Describes the properties of the metric set.");
            stuckbackgroundwakelockrate1 = stuckbackgroundwakelockrate1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("query").about("Queries the metrics in the metric set.");
            stuckbackgroundwakelockrate1 = stuckbackgroundwakelockrate1.subcommand(mcmd);
        }
        vitals0 = vitals0.subcommand(stuckbackgroundwakelockrate1);
        vitals0 = vitals0.subcommand(excessivewakeuprate1);
        vitals0 = vitals0.subcommand(crashrate1);
        vitals0 = vitals0.subcommand(anrrate1);
        app = app.subcommand(vitals0);
        app = app.subcommand(anomalies0);

        Self { app }
    }
}
use google_playdeveloperreporting1_beta1 as api;

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
