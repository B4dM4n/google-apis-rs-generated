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
        let mut app = App::new("chromemanagement1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20220425")
            .about("The Chrome Management API is a suite of services that allows Chrome administrators to view, manage and gain insights on their Chrome OS and Chrome Browser devices.")
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
        let mut customers0 = SubCommand::with_name("customers")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: apps, reports and telemetry");
        let mut apps1 = SubCommand::with_name("apps")
            .setting(AppSettings::ColoredHelp)
            .about("methods: count_chrome_app_requests");
        {
            let mcmd = SubCommand::with_name("count_chrome_app_requests")
                .about("Generate summary of app installation requests.");
            apps1 = apps1.subcommand(mcmd);
        }
        let mut reports1 = SubCommand::with_name("reports")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: count_chrome_versions, count_installed_apps and find_installed_app_devices");
        {
            let mcmd = SubCommand::with_name("count_chrome_versions")
                .about("Generate report of installed Chrome versions.");
            reports1 = reports1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("count_installed_apps")
                .about("Generate report of app installations.");
            reports1 = reports1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("find_installed_app_devices")
                .about("Generate report of devices that have a specified app installed.");
            reports1 = reports1.subcommand(mcmd);
        }
        let mut telemetry1 = SubCommand::with_name("telemetry")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: devices");
        let mut android2 = SubCommand::with_name("android")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Get a specific app for a customer by its resource name.");
            android2 = android2.subcommand(mcmd);
        }
        let mut chrome2 = SubCommand::with_name("chrome")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Get a specific app for a customer by its resource name.");
            chrome2 = chrome2.subcommand(mcmd);
        }
        let mut web2 = SubCommand::with_name("web")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Get a specific app for a customer by its resource name.");
            web2 = web2.subcommand(mcmd);
        }
        let mut devices2 = SubCommand::with_name("devices")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List all telemetry devices.");
            devices2 = devices2.subcommand(mcmd);
        }
        telemetry1 = telemetry1.subcommand(devices2);
        apps1 = apps1.subcommand(web2);
        apps1 = apps1.subcommand(chrome2);
        apps1 = apps1.subcommand(android2);
        customers0 = customers0.subcommand(telemetry1);
        customers0 = customers0.subcommand(reports1);
        customers0 = customers0.subcommand(apps1);
        app = app.subcommand(customers0);

        Self { app }
    }
}
use google_chromemanagement1 as api;

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
