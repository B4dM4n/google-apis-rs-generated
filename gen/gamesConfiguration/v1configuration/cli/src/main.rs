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
        let mut app = App::new("gamesConfiguration1_configuration")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230120")
            .about("The Google Play Game Services Publishing API allows developers to configure their games in Game Services.")
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
        let mut achievement_configurations0 = SubCommand::with_name("achievement_configurations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list and update");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Delete the achievement configuration with the given ID.");
            achievement_configurations0 = achievement_configurations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about(
                "Retrieves the metadata of the achievement configuration with the given ID.",
            );
            achievement_configurations0 = achievement_configurations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Insert a new achievement configuration in this application.");
            achievement_configurations0 = achievement_configurations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns a list of the achievement configurations in this application.");
            achievement_configurations0 = achievement_configurations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Update the metadata of the achievement configuration with the given ID.");
            achievement_configurations0 = achievement_configurations0.subcommand(mcmd);
        }
        let mut leaderboard_configurations0 = SubCommand::with_name("leaderboard_configurations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, insert, list and update");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Delete the leaderboard configuration with the given ID.");
            leaderboard_configurations0 = leaderboard_configurations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about(
                "Retrieves the metadata of the leaderboard configuration with the given ID.",
            );
            leaderboard_configurations0 = leaderboard_configurations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("insert")
                .about("Insert a new leaderboard configuration in this application.");
            leaderboard_configurations0 = leaderboard_configurations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns a list of the leaderboard configurations in this application.");
            leaderboard_configurations0 = leaderboard_configurations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Update the metadata of the leaderboard configuration with the given ID.");
            leaderboard_configurations0 = leaderboard_configurations0.subcommand(mcmd);
        }
        app = app.subcommand(leaderboard_configurations0);
        app = app.subcommand(achievement_configurations0);

        Self { app }
    }
}
use google_gamesConfiguration1_configuration as api;

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
