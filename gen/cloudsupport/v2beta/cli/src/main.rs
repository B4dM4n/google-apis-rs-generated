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
        let mut app = App::new("cloudsupport2_beta")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230131")
            .about("Manages Google Cloud technical support cases for Customer Care support offerings. ")
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
        let mut attachments0 = SubCommand::with_name("attachments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create");
        {
            let mcmd = SubCommand::with_name("create").about("Create a file attachment on a case or Cloud resource. The attachment object must have the following fields set: filename.");
            attachments0 = attachments0.subcommand(mcmd);
        }
        let mut case_classifications0 = SubCommand::with_name("case_classifications")
            .setting(AppSettings::ColoredHelp)
            .about("methods: search");
        {
            let mcmd = SubCommand::with_name("search").about("Retrieve valid classifications to be used when creating a support case. The classications are hierarchical, with each classification containing all levels of the hierarchy, separated by \" > \". For example \"Technical Issue > Compute > Compute Engine\".");
            case_classifications0 = case_classifications0.subcommand(mcmd);
        }
        let mut cases0 = SubCommand::with_name("cases")
            .setting(AppSettings::ColoredHelp)
            .about("methods: close, create, escalate, get, list, patch and search");
        {
            let mcmd = SubCommand::with_name("close").about("Close the specified case.");
            cases0 = cases0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Create a new case and associate it with the given Cloud resource. The case object must have the following fields set: display_name, description, classification, and severity.");
            cases0 = cases0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("escalate").about("Escalate a case. Escalating a case will initiate the Cloud Support escalation management process. This operation is only available to certain Customer Care tiers. Go to https://cloud.google.com/support and look for 'Technical support escalations' in the feature list to find out which tiers are able to perform escalations.");
            cases0 = cases0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieve the specified case.");
            cases0 = cases0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieve all cases under the specified parent. Note: Listing cases under an Organization returns only the cases directly parented by that organization. To retrieve all cases under an organization, including cases parented by projects under that organization, use `cases.search`.");
            cases0 = cases0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Update the specified case. Only a subset of fields can be updated.");
            cases0 = cases0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("search").about("Search cases using the specified query.");
            cases0 = cases0.subcommand(mcmd);
        }
        let mut media0 = SubCommand::with_name("media")
            .setting(AppSettings::ColoredHelp)
            .about("methods: download and upload");
        {
            let mcmd = SubCommand::with_name("download").about("Download a file attachment on a case. Note: HTTP requests must append \"?alt=media\" to the URL.");
            media0 = media0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("upload").about("Create a file attachment on a case or Cloud resource. The attachment object must have the following fields set: filename.");
            media0 = media0.subcommand(mcmd);
        }
        let mut attachments1 = SubCommand::with_name("attachments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieve all attachments associated with a support case.");
            attachments1 = attachments1.subcommand(mcmd);
        }
        let mut comments1 = SubCommand::with_name("comments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create and list");
        {
            let mcmd = SubCommand::with_name("create").about("Add a new comment to the specified Case. The comment object must have the following fields set: body.");
            comments1 = comments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Retrieve all Comments associated with the Case object.");
            comments1 = comments1.subcommand(mcmd);
        }
        cases0 = cases0.subcommand(comments1);
        cases0 = cases0.subcommand(attachments1);
        app = app.subcommand(media0);
        app = app.subcommand(cases0);
        app = app.subcommand(case_classifications0);
        app = app.subcommand(attachments0);

        Self { app }
    }
}
use google_cloudsupport2_beta as api;

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
