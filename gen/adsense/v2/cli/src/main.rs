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
        let mut app = App::new("adsense2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20220427")
            .about("The AdSense Management API allows publishers to access their inventory and run earnings and performance reports.")
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
        let mut accounts0 = SubCommand::with_name("accounts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list and list_child_accounts");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets information about the selected AdSense account.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all accounts available to this user.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_child_accounts")
                .about("Lists all accounts directly managed by the given AdSense account.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        let mut adclients1 = SubCommand::with_name("adclients")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_adcode and list");
        {
            let mcmd = SubCommand::with_name("get_adcode").about("Gets the AdSense code for a given ad client. This returns what was previously known as the 'auto ad code'. This is only supported for ad clients with a product_code of AFC. For more information, see [About the AdSense code](https://support.google.com/adsense/answer/9274634).");
            adclients1 = adclients1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the ad clients available in an account.");
            adclients1 = adclients1.subcommand(mcmd);
        }
        let mut alerts1 = SubCommand::with_name("alerts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the alerts available in an account.");
            alerts1 = alerts1.subcommand(mcmd);
        }
        let mut payments1 = SubCommand::with_name("payments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the payments available for an account.");
            payments1 = payments1.subcommand(mcmd);
        }
        let mut reports1 = SubCommand::with_name("reports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: generate and generate_csv");
        {
            let mcmd = SubCommand::with_name("generate").about("Generates an ad hoc report.");
            reports1 = reports1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("generate_csv")
                .about("Generates a csv formatted ad hoc report.");
            reports1 = reports1.subcommand(mcmd);
        }
        let mut sites1 = SubCommand::with_name("sites")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets information about the selected site.");
            sites1 = sites1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all the sites available in an account.");
            sites1 = sites1.subcommand(mcmd);
        }
        let mut adunits2 = SubCommand::with_name("adunits")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, get_adcode, list and list_linked_custom_channels");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets an ad unit from a specified account and ad client.");
            adunits2 = adunits2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_adcode")
                .about("Gets the AdSense code for a given ad unit.");
            adunits2 = adunits2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all ad units under a specified account and ad client.");
            adunits2 = adunits2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_linked_custom_channels")
                .about("Lists all the custom channels available for an ad unit.");
            adunits2 = adunits2.subcommand(mcmd);
        }
        let mut customchannels2 = SubCommand::with_name("customchannels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list and list_linked_ad_units");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets information about the selected custom channel.");
            customchannels2 = customchannels2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the custom channels available in an ad client.");
            customchannels2 = customchannels2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_linked_ad_units")
                .about("Lists all the ad units available for a custom channel.");
            customchannels2 = customchannels2.subcommand(mcmd);
        }
        let mut urlchannels2 = SubCommand::with_name("urlchannels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists active url channels.");
            urlchannels2 = urlchannels2.subcommand(mcmd);
        }
        let mut saved2 = SubCommand::with_name("saved")
            .setting(AppSettings::ColoredHelp)
            .about("methods: generate, generate_csv and list");
        {
            let mcmd = SubCommand::with_name("generate").about("Generates a saved report.");
            saved2 = saved2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("generate_csv")
                .about("Generates a csv formatted saved report.");
            saved2 = saved2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists saved reports.");
            saved2 = saved2.subcommand(mcmd);
        }
        reports1 = reports1.subcommand(saved2);
        adclients1 = adclients1.subcommand(urlchannels2);
        adclients1 = adclients1.subcommand(customchannels2);
        adclients1 = adclients1.subcommand(adunits2);
        accounts0 = accounts0.subcommand(sites1);
        accounts0 = accounts0.subcommand(reports1);
        accounts0 = accounts0.subcommand(payments1);
        accounts0 = accounts0.subcommand(alerts1);
        accounts0 = accounts0.subcommand(adclients1);
        app = app.subcommand(accounts0);

        Self { app }
    }
}
use google_adsense2 as api;

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
