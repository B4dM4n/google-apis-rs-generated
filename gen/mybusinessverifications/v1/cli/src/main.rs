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
        let mut app = App::new("mybusinessverifications1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230131")
            .about("The My Business Verifications API provides an interface for taking verifications related actions for locations.")
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
            .about("methods: fetch_verification_options, get_voice_of_merchant_state and verify");
        {
            let mcmd = SubCommand::with_name("fetch_verification_options").about(
                "Reports all eligible verification options for a location in a specific language.",
            );
            locations0 = locations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_voice_of_merchant_state")
                .about("Gets the VoiceOfMerchant state.");
            locations0 = locations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("verify")
                .about("Starts the verification process for a location.");
            locations0 = locations0.subcommand(mcmd);
        }
        let mut verification_tokens0 = SubCommand::with_name("verification_tokens")
            .setting(AppSettings::ColoredHelp)
            .about("methods: generate");
        {
            let mcmd = SubCommand::with_name("generate").about("Generates a token for the provided location data as a vetted [partner](https://support.google.com/business/answer/7674102). Throws PERMISSION_DENIED if the caller is not a vetted partner account. Throws FAILED_PRECONDITION if the caller's VettedStatus is INVALID.");
            verification_tokens0 = verification_tokens0.subcommand(mcmd);
        }
        let mut verifications1 = SubCommand::with_name("verifications")
            .setting(AppSettings::ColoredHelp)
            .about("methods: complete and list");
        {
            let mcmd = SubCommand::with_name("complete").about("Completes a `PENDING` verification. It is only necessary for non `AUTO` verification methods. `AUTO` verification request is instantly `VERIFIED` upon creation.");
            verifications1 = verifications1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List verifications of a location, ordered by create time.");
            verifications1 = verifications1.subcommand(mcmd);
        }
        locations0 = locations0.subcommand(verifications1);
        app = app.subcommand(verification_tokens0);
        app = app.subcommand(locations0);

        Self { app }
    }
}
use google_mybusinessverifications1 as api;

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
