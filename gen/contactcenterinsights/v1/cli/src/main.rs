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
        let mut app = App::new("contactcenterinsights1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230121")
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
            .about("sub-resources: locations");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_settings and update_settings");
        {
            let mcmd = SubCommand::with_name("get_settings").about("Gets project-level settings.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update_settings").about("Updates project-level settings.");
            locations1 = locations1.subcommand(mcmd);
        }
        let mut conversations2 = SubCommand::with_name("conversations")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: bulk_analyze, calculate_stats, create, delete, get, ingest, list and patch");
        {
            let mcmd = SubCommand::with_name("bulk_analyze")
                .about("Analyzes multiple conversations in a single request.");
            conversations2 = conversations2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("calculate_stats").about("Gets conversation statistics.");
            conversations2 = conversations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a conversation.");
            conversations2 = conversations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a conversation.");
            conversations2 = conversations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a conversation.");
            conversations2 = conversations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("ingest").about(
                "Imports conversations and processes them according to the user's configuration.",
            );
            conversations2 = conversations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists conversations.");
            conversations2 = conversations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a conversation.");
            conversations2 = conversations2.subcommand(mcmd);
        }
        let mut insightsdata2 = SubCommand::with_name("insightsdata")
            .setting(AppSettings::ColoredHelp)
            .about("methods: export");
        {
            let mcmd = SubCommand::with_name("export")
                .about("Export insights data to a destination defined in the request body.");
            insightsdata2 = insightsdata2.subcommand(mcmd);
        }
        let mut issue_models2 = SubCommand::with_name("issue_models")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: calculate_issue_model_stats, create, delete, deploy, get, list, patch and undeploy");
        {
            let mcmd = SubCommand::with_name("calculate_issue_model_stats")
                .about("Gets an issue model's statistics.");
            issue_models2 = issue_models2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates an issue model.");
            issue_models2 = issue_models2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an issue model.");
            issue_models2 = issue_models2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("deploy").about("Deploys an issue model. Returns an error if a model is already deployed. An issue model can only be used in analysis after it has been deployed.");
            issue_models2 = issue_models2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an issue model.");
            issue_models2 = issue_models2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists issue models.");
            issue_models2 = issue_models2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an issue model.");
            issue_models2 = issue_models2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undeploy").about("Undeploys an issue model. An issue model can not be used in analysis after it has been undeployed.");
            issue_models2 = issue_models2.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut phrase_matchers2 = SubCommand::with_name("phrase_matchers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a phrase matcher.");
            phrase_matchers2 = phrase_matchers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a phrase matcher.");
            phrase_matchers2 = phrase_matchers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a phrase matcher.");
            phrase_matchers2 = phrase_matchers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists phrase matchers.");
            phrase_matchers2 = phrase_matchers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a phrase matcher.");
            phrase_matchers2 = phrase_matchers2.subcommand(mcmd);
        }
        let mut views2 = SubCommand::with_name("views")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a view.");
            views2 = views2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a view.");
            views2 = views2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a view.");
            views2 = views2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists views.");
            views2 = views2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a view.");
            views2 = views2.subcommand(mcmd);
        }
        let mut analyses3 = SubCommand::with_name("analyses")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an analysis. The long running operation is done when the analysis has completed.");
            analyses3 = analyses3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an analysis.");
            analyses3 = analyses3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an analysis.");
            analyses3 = analyses3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists analyses.");
            analyses3 = analyses3.subcommand(mcmd);
        }
        let mut issues3 = SubCommand::with_name("issues")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an issue.");
            issues3 = issues3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an issue.");
            issues3 = issues3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists issues.");
            issues3 = issues3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an issue.");
            issues3 = issues3.subcommand(mcmd);
        }
        issue_models2 = issue_models2.subcommand(issues3);
        conversations2 = conversations2.subcommand(analyses3);
        locations1 = locations1.subcommand(views2);
        locations1 = locations1.subcommand(phrase_matchers2);
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(issue_models2);
        locations1 = locations1.subcommand(insightsdata2);
        locations1 = locations1.subcommand(conversations2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_contactcenterinsights1 as api;

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
