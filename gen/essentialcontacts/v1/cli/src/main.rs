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
        let mut app = App::new("essentialcontacts1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230131")
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
        let mut folders0 = SubCommand::with_name("folders")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: contacts");
        let mut organizations0 = SubCommand::with_name("organizations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: contacts");
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: contacts");
        let mut contacts1 = SubCommand::with_name("contacts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: compute, create, delete, get, list, patch and send_test_message");
        {
            let mcmd = SubCommand::with_name("compute").about("Lists all contacts for the resource that are subscribed to the specified notification categories, including contacts inherited from any parent resources.");
            contacts1 = contacts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Adds a new contact for a resource.");
            contacts1 = contacts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a contact.");
            contacts1 = contacts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a single contact.");
            contacts1 = contacts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the contacts that have been set on a resource.");
            contacts1 = contacts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates a contact. Note: A contact's email address cannot be changed.");
            contacts1 = contacts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("send_test_message").about("Allows a contact admin to send a test message to contact to verify that it has been configured correctly.");
            contacts1 = contacts1.subcommand(mcmd);
        }
        let mut contacts1 = SubCommand::with_name("contacts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: compute, create, delete, get, list, patch and send_test_message");
        {
            let mcmd = SubCommand::with_name("compute").about("Lists all contacts for the resource that are subscribed to the specified notification categories, including contacts inherited from any parent resources.");
            contacts1 = contacts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Adds a new contact for a resource.");
            contacts1 = contacts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a contact.");
            contacts1 = contacts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a single contact.");
            contacts1 = contacts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the contacts that have been set on a resource.");
            contacts1 = contacts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates a contact. Note: A contact's email address cannot be changed.");
            contacts1 = contacts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("send_test_message").about("Allows a contact admin to send a test message to contact to verify that it has been configured correctly.");
            contacts1 = contacts1.subcommand(mcmd);
        }
        let mut contacts1 = SubCommand::with_name("contacts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: compute, create, delete, get, list, patch and send_test_message");
        {
            let mcmd = SubCommand::with_name("compute").about("Lists all contacts for the resource that are subscribed to the specified notification categories, including contacts inherited from any parent resources.");
            contacts1 = contacts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Adds a new contact for a resource.");
            contacts1 = contacts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a contact.");
            contacts1 = contacts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a single contact.");
            contacts1 = contacts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the contacts that have been set on a resource.");
            contacts1 = contacts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates a contact. Note: A contact's email address cannot be changed.");
            contacts1 = contacts1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("send_test_message").about("Allows a contact admin to send a test message to contact to verify that it has been configured correctly.");
            contacts1 = contacts1.subcommand(mcmd);
        }
        projects0 = projects0.subcommand(contacts1);
        organizations0 = organizations0.subcommand(contacts1);
        folders0 = folders0.subcommand(contacts1);
        app = app.subcommand(projects0);
        app = app.subcommand(organizations0);
        app = app.subcommand(folders0);

        Self { app }
    }
}
use google_essentialcontacts1 as api;

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
