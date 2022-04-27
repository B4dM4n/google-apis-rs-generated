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
        let mut app = App::new("ideahub1_beta")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20220425")
            .about("This is an invitation-only API.")
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
        let mut platforms0 = SubCommand::with_name("platforms")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: properties");
        let mut properties1 = SubCommand::with_name("properties")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: idea_activities, idea_states, ideas, locales and topic_states");
        let mut idea_activities2 = SubCommand::with_name("idea_activities")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an idea activity entry.");
            idea_activities2 = idea_activities2.subcommand(mcmd);
        }
        let mut idea_states2 = SubCommand::with_name("idea_states")
            .setting(AppSettings::ColoredHelp)
            .about("methods: patch");
        {
            let mcmd = SubCommand::with_name("patch").about("Update an idea state resource.");
            idea_states2 = idea_states2.subcommand(mcmd);
        }
        let mut ideas2 = SubCommand::with_name("ideas")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("List ideas for a given Creator and filter and sort options.");
            ideas2 = ideas2.subcommand(mcmd);
        }
        let mut locales2 = SubCommand::with_name("locales")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns which locales ideas are available in for a given Creator.");
            locales2 = locales2.subcommand(mcmd);
        }
        let mut topic_states2 = SubCommand::with_name("topic_states")
            .setting(AppSettings::ColoredHelp)
            .about("methods: patch");
        {
            let mcmd = SubCommand::with_name("patch").about("Update a topic state resource.");
            topic_states2 = topic_states2.subcommand(mcmd);
        }
        properties1 = properties1.subcommand(topic_states2);
        properties1 = properties1.subcommand(locales2);
        properties1 = properties1.subcommand(ideas2);
        properties1 = properties1.subcommand(idea_states2);
        properties1 = properties1.subcommand(idea_activities2);
        platforms0 = platforms0.subcommand(properties1);
        app = app.subcommand(platforms0);

        Self { app }
    }
}
use google_ideahub1_beta as api;

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
