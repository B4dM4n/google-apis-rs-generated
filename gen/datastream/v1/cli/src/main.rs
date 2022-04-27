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
        let mut app = App::new("datastream1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20220405")
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
            .about("methods: fetch_static_ips, get and list");
        {
            let mcmd = SubCommand::with_name("fetch_static_ips").about(
                "The FetchStaticIps API call exposes the static IP addresses used by Datastream.",
            );
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a location.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists information about the supported locations for this service.");
            locations1 = locations1.subcommand(mcmd);
        }
        let mut connection_profiles2 = SubCommand::with_name("connection_profiles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, discover, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Use this method to create a connection profile in a project and location.");
            connection_profiles2 = connection_profiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Use this method to delete a connection profile.");
            connection_profiles2 = connection_profiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("discover").about("Use this method to discover a connection profile. The discover API call exposes the data objects and metadata belonging to the profile. Typically, a request returns children data objects of a parent data object that's optionally supplied in the request.");
            connection_profiles2 = connection_profiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Use this method to get details about a connection profile.");
            connection_profiles2 = connection_profiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Use this method to list connection profiles created in a project and location.",
            );
            connection_profiles2 = connection_profiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Use this method to update the parameters of a connection profile.");
            connection_profiles2 = connection_profiles2.subcommand(mcmd);
        }
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
        let mut private_connections2 = SubCommand::with_name("private_connections")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Use this method to create a private connectivity configuration.");
            private_connections2 = private_connections2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Use this method to delete a private connectivity configuration.");
            private_connections2 = private_connections2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about(
                "Use this method to get details about a private connectivity configuration.",
            );
            private_connections2 = private_connections2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Use this method to list private connectivity configurations in a project and location.");
            private_connections2 = private_connections2.subcommand(mcmd);
        }
        let mut streams2 = SubCommand::with_name("streams")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Use this method to create a stream.");
            streams2 = streams2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Use this method to delete a stream.");
            streams2 = streams2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Use this method to get details about a stream.");
            streams2 = streams2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Use this method to list streams in a project and location.");
            streams2 = streams2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Use this method to update the configuration of a stream.");
            streams2 = streams2.subcommand(mcmd);
        }
        let mut routes3 = SubCommand::with_name("routes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Use this method to create a route for a private connectivity configuration in a project and location.");
            routes3 = routes3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Use this method to delete a route.");
            routes3 = routes3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Use this method to get details about a route.");
            routes3 = routes3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Use this method to list routes created for a private connectivity configuration in a project and location.");
            routes3 = routes3.subcommand(mcmd);
        }
        let mut objects3 = SubCommand::with_name("objects")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list, lookup, start_backfill_job and stop_backfill_job");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Use this method to get details about a stream object.");
            objects3 = objects3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Use this method to list the objects of a specific stream.");
            objects3 = objects3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("lookup").about(
                "Use this method to look up a stream object by its source object identifier.",
            );
            objects3 = objects3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("start_backfill_job")
                .about("Use this method to start a backfill job for the specified stream object.");
            objects3 = objects3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("stop_backfill_job")
                .about("Use this method to stop a backfill job for the specified stream object.");
            objects3 = objects3.subcommand(mcmd);
        }
        streams2 = streams2.subcommand(objects3);
        private_connections2 = private_connections2.subcommand(routes3);
        locations1 = locations1.subcommand(streams2);
        locations1 = locations1.subcommand(private_connections2);
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(connection_profiles2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_datastream1 as api;

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
