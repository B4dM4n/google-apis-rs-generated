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
        let mut app = App::new("accesscontextmanager1_beta")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230126")
            .about("An API for setting attribute based access control to requests to GCP services.")
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
        let mut access_policies0 = SubCommand::with_name("access_policies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Create an `AccessPolicy`. Fails if this organization already has a `AccessPolicy`. The longrunning Operation will have a successful status once the `AccessPolicy` has propagated to long-lasting storage. Syntactic and basic semantic errors will be returned in `metadata` as a BadRequest proto.");
            access_policies0 = access_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete an AccessPolicy by resource name. The longrunning Operation will have a successful status once the AccessPolicy has been removed from long-lasting storage.");
            access_policies0 = access_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get an AccessPolicy by name.");
            access_policies0 = access_policies0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("List all AccessPolicies under a container.");
            access_policies0 = access_policies0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update an AccessPolicy. The longrunning Operation from this RPC will have a successful status once the changes to the AccessPolicy have propagated to long-lasting storage. Syntactic and basic semantic errors will be returned in `metadata` as a BadRequest proto.");
            access_policies0 = access_policies0.subcommand(mcmd);
        }
        let mut operations0 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut access_levels1 = SubCommand::with_name("access_levels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Create an Access Level. The longrunning operation from this RPC will have a successful status once the Access Level has propagated to long-lasting storage. Access Levels containing errors will result in an error response for the first error encountered.");
            access_levels1 = access_levels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete an Access Level by resource name. The longrunning operation from this RPC will have a successful status once the Access Level has been removed from long-lasting storage.");
            access_levels1 = access_levels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get an Access Level by resource name.");
            access_levels1 = access_levels1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("List all Access Levels for an access policy.");
            access_levels1 = access_levels1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update an Access Level. The longrunning operation from this RPC will have a successful status once the changes to the Access Level have propagated to long-lasting storage. Access Levels containing errors will result in an error response for the first error encountered.");
            access_levels1 = access_levels1.subcommand(mcmd);
        }
        let mut service_perimeters1 = SubCommand::with_name("service_perimeters")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Create a Service Perimeter. The longrunning operation from this RPC will have a successful status once the Service Perimeter has propagated to long-lasting storage. Service Perimeters containing errors will result in an error response for the first error encountered.");
            service_perimeters1 = service_perimeters1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a Service Perimeter by resource name. The longrunning operation from this RPC will have a successful status once the Service Perimeter has been removed from long-lasting storage.");
            service_perimeters1 = service_perimeters1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Get a Service Perimeter by resource name.");
            service_perimeters1 = service_perimeters1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all Service Perimeters for an access policy.");
            service_perimeters1 = service_perimeters1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update a Service Perimeter. The longrunning operation from this RPC will have a successful status once the changes to the Service Perimeter have propagated to long-lasting storage. Service Perimeter containing errors will result in an error response for the first error encountered.");
            service_perimeters1 = service_perimeters1.subcommand(mcmd);
        }
        access_policies0 = access_policies0.subcommand(service_perimeters1);
        access_policies0 = access_policies0.subcommand(access_levels1);
        app = app.subcommand(operations0);
        app = app.subcommand(access_policies0);

        Self { app }
    }
}
use google_accesscontextmanager1_beta as api;

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
