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
        let mut app = App::new("recaptchaenterprise1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20220415")
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
        let mut projects0 = SubCommand::with_name("projects")
                        .setting(AppSettings::ColoredHelp)
                        .about("sub-resources: assessments, keys, relatedaccountgroupmemberships and relatedaccountgroups");
        let mut assessments1 = SubCommand::with_name("assessments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: annotate and create");
        {
            let mcmd = SubCommand::with_name("annotate").about("Annotates a previously created Assessment to provide additional information on whether the event turned out to be authentic or fraudulent.");
            assessments1 = assessments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates an Assessment of the likelihood an event is legitimate.");
            assessments1 = assessments1.subcommand(mcmd);
        }
        let mut keys1 = SubCommand::with_name("keys")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, get_metrics, list, migrate and patch");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a new reCAPTCHA Enterprise key.");
            keys1 = keys1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified key.");
            keys1 = keys1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified key.");
            keys1 = keys1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_metrics").about(
                "Get some aggregated metrics for a Key. This data can be used to build dashboards.",
            );
            keys1 = keys1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all keys that belong to a project.");
            keys1 = keys1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("migrate").about("Migrates an existing key from reCAPTCHA to reCAPTCHA Enterprise. Once a key is migrated, it can be used from either product. SiteVerify requests are billed as CreateAssessment calls. You must be authenticated as one of the current owners of the reCAPTCHA Site Key, and your user must have the reCAPTCHA Enterprise Admin IAM role in the destination project.");
            keys1 = keys1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified key.");
            keys1 = keys1.subcommand(mcmd);
        }
        let mut relatedaccountgroupmemberships1 =
            SubCommand::with_name("relatedaccountgroupmemberships")
                .setting(AppSettings::ColoredHelp)
                .about("methods: search");
        {
            let mcmd = SubCommand::with_name("search")
                .about("Search group memberships related to a given account.");
            relatedaccountgroupmemberships1 = relatedaccountgroupmemberships1.subcommand(mcmd);
        }
        let mut relatedaccountgroups1 = SubCommand::with_name("relatedaccountgroups")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List groups of related accounts.");
            relatedaccountgroups1 = relatedaccountgroups1.subcommand(mcmd);
        }
        let mut memberships2 = SubCommand::with_name("memberships")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Get the memberships in a group of related accounts.");
            memberships2 = memberships2.subcommand(mcmd);
        }
        relatedaccountgroups1 = relatedaccountgroups1.subcommand(memberships2);
        projects0 = projects0.subcommand(relatedaccountgroups1);
        projects0 = projects0.subcommand(relatedaccountgroupmemberships1);
        projects0 = projects0.subcommand(keys1);
        projects0 = projects0.subcommand(assessments1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_recaptchaenterprise1 as api;

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
