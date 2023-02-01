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
        let mut app = App::new("workstations1_beta")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230113")
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
            .about("sub-resources: operations and workstation_clusters");
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, delete, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.");
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
        let mut workstation_clusters2 = SubCommand::with_name("workstation_clusters")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new workstation cluster.");
            workstation_clusters2 = workstation_clusters2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the specified workstation cluster.");
            workstation_clusters2 = workstation_clusters2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Returns the requested workstation cluster.");
            workstation_clusters2 = workstation_clusters2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns all workstation clusters in the specified location.");
            workstation_clusters2 = workstation_clusters2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates an existing workstation cluster.");
            workstation_clusters2 = workstation_clusters2.subcommand(mcmd);
        }
        let mut workstation_configs3 = SubCommand::with_name("workstation_configs")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, list_usable, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a new workstation configuration.");
            workstation_configs3 = workstation_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified workstation configuration.");
            workstation_configs3 = workstation_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns the requested workstation configuration.");
            workstation_configs3 = workstation_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            workstation_configs3 = workstation_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns all workstation configurations in the specified cluster.");
            workstation_configs3 = workstation_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_usable").about("Returns all workstation configurations in the specified cluster on which the caller has the \"workstations.workstation.create\" permission.");
            workstation_configs3 = workstation_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates an existing workstation configuration.");
            workstation_configs3 = workstation_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            workstation_configs3 = workstation_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            workstation_configs3 = workstation_configs3.subcommand(mcmd);
        }
        let mut workstations4 = SubCommand::with_name("workstations")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, generate_access_token, get, get_iam_policy, list, list_usable, patch, set_iam_policy, start, stop and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new workstation.");
            workstations4 = workstations4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified workstation.");
            workstations4 = workstations4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("generate_access_token").about("Returns a short-lived credential that can be used to send authenticated and authorized traffic to a workstation.");
            workstations4 = workstations4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the requested workstation.");
            workstations4 = workstations4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            workstations4 = workstations4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns all Workstations using the specified config.");
            workstations4 = workstations4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_usable").about("Returns all Workstations using the specified config on which the caller has the \"workstations.workstations.use\" permission.");
            workstations4 = workstations4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing workstation.");
            workstations4 = workstations4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            workstations4 = workstations4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("start")
                .about("Starts running a workstation so that users can connect to it.");
            workstations4 = workstations4.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("stop").about("Stops running a workstation, reducing costs.");
            workstations4 = workstations4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            workstations4 = workstations4.subcommand(mcmd);
        }
        workstation_configs3 = workstation_configs3.subcommand(workstations4);
        workstation_clusters2 = workstation_clusters2.subcommand(workstation_configs3);
        locations1 = locations1.subcommand(workstation_clusters2);
        locations1 = locations1.subcommand(operations2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_workstations1_beta as api;

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
