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
        let mut app = App::new("discoveryengine1_alpha")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230126")
            .about("Discovery Engine API.")
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
            .about("sub-resources: locations and operations");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: data_stores and operations");
        let mut operations1 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations1 = operations1.subcommand(mcmd);
        }
        let mut data_stores2 = SubCommand::with_name("data_stores")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: branches, models, operations, serving_configs and user_events");
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
        let mut branches3 = SubCommand::with_name("branches")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: documents and operations");
        let mut models3 = SubCommand::with_name("models")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: operations");
        let mut operations3 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations3 = operations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations3 = operations3.subcommand(mcmd);
        }
        let mut serving_configs3 = SubCommand::with_name("serving_configs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: recommend");
        {
            let mcmd = SubCommand::with_name("recommend")
                .about("Makes a recommendation, which requires a contextual user event.");
            serving_configs3 = serving_configs3.subcommand(mcmd);
        }
        let mut user_events3 = SubCommand::with_name("user_events")
            .setting(AppSettings::ColoredHelp)
            .about("methods: collect, import and write");
        {
            let mcmd = SubCommand::with_name("collect").about("Writes a single user event from the browser. This uses a GET request to due to browser restriction of POST-ing to a 3rd party domain. This method is used only by the Discovery Engine API JavaScript pixel and Google Tag Manager. Users should not call this method directly.");
            user_events3 = user_events3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Bulk import of User events. Request processing might be synchronous. Events that already exist are skipped. Use this method for backfilling historical user events. Operation.response is of type ImportResponse. Note that it is possible for a subset of the items to be successfully inserted. Operation.metadata is of type ImportMetadata.");
            user_events3 = user_events3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("write").about("Writes a single user event.");
            user_events3 = user_events3.subcommand(mcmd);
        }
        let mut documents4 = SubCommand::with_name("documents")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, import, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a Document.");
            documents4 = documents4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a Document.");
            documents4 = documents4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a Document.");
            documents4 = documents4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Bulk import of multiple Documents. Request processing may be synchronous. Non-existing items will be created. Note: It is possible for a subset of the Documents to be successfully updated.");
            documents4 = documents4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Gets a list of Documents.");
            documents4 = documents4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a Document.");
            documents4 = documents4.subcommand(mcmd);
        }
        let mut operations4 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations4 = operations4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations4 = operations4.subcommand(mcmd);
        }
        let mut operations4 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations4 = operations4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations4 = operations4.subcommand(mcmd);
        }
        models3 = models3.subcommand(operations4);
        branches3 = branches3.subcommand(operations4);
        branches3 = branches3.subcommand(documents4);
        data_stores2 = data_stores2.subcommand(user_events3);
        data_stores2 = data_stores2.subcommand(serving_configs3);
        data_stores2 = data_stores2.subcommand(operations3);
        data_stores2 = data_stores2.subcommand(models3);
        data_stores2 = data_stores2.subcommand(branches3);
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(data_stores2);
        projects0 = projects0.subcommand(operations1);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_discoveryengine1_alpha as api;

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
