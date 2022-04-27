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
        let mut app = App::new("forms1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20220421")
            .about("Reads and writes Google Forms and responses.")
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
        let mut forms0 = SubCommand::with_name("forms")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_update, create and get");
        {
            let mcmd = SubCommand::with_name("batch_update")
                .about("Change the form with a batch of updates.");
            forms0 = forms0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Create a new form using the title given in the provided form message in the request. *Important:* Only the form.info.title and form.info.document_title fields are copied to the new form. All other fields including the form description, items and settings are disallowed. To create a new form and add items, you must first call forms.create to create an empty form with a title and (optional) document title, and then call forms.update to add the items.");
            forms0 = forms0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get a form.");
            forms0 = forms0.subcommand(mcmd);
        }
        let mut responses1 = SubCommand::with_name("responses")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Get one response from the form.");
            responses1 = responses1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List a form's responses.");
            responses1 = responses1.subcommand(mcmd);
        }
        let mut watches1 = SubCommand::with_name("watches")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, list and renew");
        {
            let mcmd = SubCommand::with_name("create").about("Create a new watch. If a watch ID is provided, it must be unused. For each invoking project, the per form limit is one watch per Watch.EventType. A watch expires seven days after it is created (see Watch.expire_time).");
            watches1 = watches1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a watch.");
            watches1 = watches1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Return a list of the watches owned by the invoking project. The maximum number of watches is two: For each invoker, the limit is one for each event type per form.");
            watches1 = watches1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("renew").about("Renew an existing watch for seven days. The state of the watch after renewal is `ACTIVE`, and the `expire_time` is seven days from the renewal. Renewing a watch in an error state (e.g. `SUSPENDED`) succeeds if the error is no longer present, but fail otherwise. After a watch has expired, RenewWatch returns `NOT_FOUND`.");
            watches1 = watches1.subcommand(mcmd);
        }
        forms0 = forms0.subcommand(watches1);
        forms0 = forms0.subcommand(responses1);
        app = app.subcommand(forms0);

        Self { app }
    }
}
use google_forms1 as api;

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
