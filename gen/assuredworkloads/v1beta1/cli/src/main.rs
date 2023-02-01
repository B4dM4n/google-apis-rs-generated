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
        let mut app = App::new("assuredworkloads1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230127")
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
        let mut organizations0 = SubCommand::with_name("organizations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: organizations");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: operations and workloads");
        let mut organizations1 = SubCommand::with_name("organizations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut workloads2 = SubCommand::with_name("workloads")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and restrict_allowed_resources");
        {
            let mcmd = SubCommand::with_name("create").about("Creates Assured Workload.");
            workloads2 = workloads2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the workload. Make sure that workload's direct children are already in a deleted state, otherwise the request will fail with a FAILED_PRECONDITION error. In addition to assuredworkloads.workload.delete permission, the user should also have orgpolicy.policy.set permission on the deleted folder to remove Assured Workloads OrgPolicies.");
            workloads2 = workloads2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets Assured Workload associated with a CRM Node");
            workloads2 = workloads2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists Assured Workloads under a CRM Node.");
            workloads2 = workloads2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing workload. Currently allows updating of workload display_name and labels. For force updates don't set etag field in the Workload. Only one update operation per workload can be in progress.");
            workloads2 = workloads2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("restrict_allowed_resources").about("Restrict the list of resources allowed in the Workload environment. The current list of allowed products can be found at https://cloud.google.com/assured-workloads/docs/supported-products In addition to assuredworkloads.workload.update permission, the user should also have orgpolicy.policy.set permission on the folder resource to use this functionality.");
            workloads2 = workloads2.subcommand(mcmd);
        }
        let mut locations2 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: workloads");
        let mut organizations3 = SubCommand::with_name("organizations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut violations3 = SubCommand::with_name("violations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: acknowledge, get and list");
        {
            let mcmd = SubCommand::with_name("acknowledge").about("Acknowledges an existing violation. By acknowledging a violation, users acknowledge the existence of a compliance violation in their workload and decide to ignore it due to a valid business justification. Acknowledgement is a permanent operation and it cannot be reverted.");
            violations3 = violations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves Assured Workload Violation based on ID.");
            violations3 = violations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the Violations in the AssuredWorkload Environment. Callers may also choose to read across multiple Workloads as per [AIP-159](https://google.aip.dev/159) by using '-' (the hyphen or dash character) as a wildcard character instead of workload-id in the parent. Format `organizations/{org_id}/locations/{location}/workloads/-`");
            violations3 = violations3.subcommand(mcmd);
        }
        let mut workloads3 = SubCommand::with_name("workloads")
            .setting(AppSettings::ColoredHelp)
            .about("methods: analyze_workload_move");
        {
            let mcmd = SubCommand::with_name("analyze_workload_move").about("Analyzes a hypothetical move of a source project or project-based workload to a target (destination) folder-based workload.");
            workloads3 = workloads3.subcommand(mcmd);
        }
        let mut locations4 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: workloads");
        let mut workloads5 = SubCommand::with_name("workloads")
            .setting(AppSettings::ColoredHelp)
            .about("methods: analyze_workload_move");
        {
            let mcmd = SubCommand::with_name("analyze_workload_move").about("Analyzes a hypothetical move of a source project or project-based workload to a target (destination) folder-based workload.");
            workloads5 = workloads5.subcommand(mcmd);
        }
        locations4 = locations4.subcommand(workloads5);
        organizations3 = organizations3.subcommand(locations4);
        locations2 = locations2.subcommand(workloads3);
        workloads2 = workloads2.subcommand(violations3);
        workloads2 = workloads2.subcommand(organizations3);
        organizations1 = organizations1.subcommand(locations2);
        locations1 = locations1.subcommand(workloads2);
        locations1 = locations1.subcommand(operations2);
        projects0 = projects0.subcommand(organizations1);
        organizations0 = organizations0.subcommand(locations1);
        app = app.subcommand(projects0);
        app = app.subcommand(organizations0);

        Self { app }
    }
}
use google_assuredworkloads1_beta1 as api;

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
