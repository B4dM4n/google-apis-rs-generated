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
        let mut app = App::new("cloudbilling1_beta")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20221206")
            .about("Allows developers to manage billing for their Google Cloud Platform projects programmatically.")
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
        let mut billing_accounts0 = SubCommand::with_name("billing_accounts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: estimate_cost_scenario");
        {
            let mcmd = SubCommand::with_name("estimate_cost_scenario").about("Use custom pricing in the estimate, using a `CostScenario` with a defined `billingAccount`.");
            billing_accounts0 = billing_accounts0.subcommand(mcmd);
        }
        let mut v_1beta0 = SubCommand::with_name("v_1beta")
            .setting(AppSettings::ColoredHelp)
            .about("methods: estimate_cost_scenario");
        {
            let mcmd = SubCommand::with_name("estimate_cost_scenario").about(
                "Estimate list prices using a `CostScenario` without a defined `billingAccount`.",
            );
            v_1beta0 = v_1beta0.subcommand(mcmd);
        }
        app = app.subcommand(v_1beta0);
        app = app.subcommand(billing_accounts0);

        Self { app }
    }
}
use google_cloudbilling1_beta as api;

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
