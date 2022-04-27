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
        let mut app = App::new("recommendationengine1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20220407")
            .about("Note that we now highly recommend new customers to use Retail API, which incorporates the GA version of the Recommendations AI funtionalities. To enable Retail API, please visit https://console.cloud.google.com/apis/library/retail.googleapis.com. The Recommendations AI service enables customers to build end-to-end personalized recommendation systems without requiring a high level of expertise in machine learning, recommendation system, or Google Cloud.")
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
            .about("sub-resources: catalogs");
        let mut catalogs2 = SubCommand::with_name("catalogs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list and patch");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the catalog configurations associated with the project.");
            catalogs2 = catalogs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the catalog configuration.");
            catalogs2 = catalogs2.subcommand(mcmd);
        }
        let mut catalog_items3 = SubCommand::with_name("catalog_items")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, import, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a catalog item.");
            catalog_items3 = catalog_items3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a catalog item.");
            catalog_items3 = catalog_items3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a specific catalog item.");
            catalog_items3 = catalog_items3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Bulk import of multiple catalog items. Request processing may be synchronous. No partial updating supported. Non-existing items will be created. Operation.response is of type ImportResponse. Note that it is possible for a subset of the items to be successfully updated.");
            catalog_items3 = catalog_items3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Gets a list of catalog items.");
            catalog_items3 = catalog_items3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a catalog item. Partial updating is supported. Non-existing items will be created.");
            catalog_items3 = catalog_items3.subcommand(mcmd);
        }
        let mut event_stores3 = SubCommand::with_name("event_stores")
                        .setting(AppSettings::ColoredHelp)
                        .about("sub-resources: operations, placements, prediction_api_key_registrations and user_events");
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
        let mut placements4 = SubCommand::with_name("placements")
            .setting(AppSettings::ColoredHelp)
            .about("methods: predict");
        {
            let mcmd = SubCommand::with_name("predict").about("Makes a recommendation prediction. If using API Key based authentication, the API Key must be registered using the PredictionApiKeyRegistry service. [Learn more](https://cloud.google.com/recommendations-ai/docs/setting-up#register-key).");
            placements4 = placements4.subcommand(mcmd);
        }
        let mut prediction_api_key_registrations4 =
            SubCommand::with_name("prediction_api_key_registrations")
                .setting(AppSettings::ColoredHelp)
                .about("methods: create, delete and list");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Register an API key for use with predict method.");
            prediction_api_key_registrations4 = prediction_api_key_registrations4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Unregister an apiKey from using for predict method.");
            prediction_api_key_registrations4 = prediction_api_key_registrations4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List the registered apiKeys for use with predict method.");
            prediction_api_key_registrations4 = prediction_api_key_registrations4.subcommand(mcmd);
        }
        let mut user_events4 = SubCommand::with_name("user_events")
            .setting(AppSettings::ColoredHelp)
            .about("methods: collect, import, list, purge, rejoin and write");
        {
            let mcmd = SubCommand::with_name("collect").about("Writes a single user event from the browser. This uses a GET request to due to browser restriction of POST-ing to a 3rd party domain. This method is used only by the Recommendations AI JavaScript pixel. Users should not call this method directly.");
            user_events4 = user_events4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Bulk import of User events. Request processing might be synchronous. Events that already exist are skipped. Use this method for backfilling historical user events. Operation.response is of type ImportResponse. Note that it is possible for a subset of the items to be successfully inserted. Operation.metadata is of type ImportMetadata.");
            user_events4 = user_events4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Gets a list of user events within a time range, with potential filtering. The method does not list unjoined user events. Unjoined user event definition: when a user event is ingested from Recommendations AI User Event APIs, the catalog item included in the user event is connected with the current catalog. If a catalog item of the ingested event is not in the current catalog, it could lead to degraded model quality. This is called an unjoined event.");
            user_events4 = user_events4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("purge").about("Deletes permanently all user events specified by the filter provided. Depending on the number of events specified by the filter, this operation could take hours or days to complete. To test a filter, use the list command first.");
            user_events4 = user_events4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("rejoin").about("Triggers a user event rejoin operation with latest catalog data. Events will not be annotated with detailed catalog information if catalog item is missing at the time the user event is ingested, and these events are stored as unjoined events with a limited usage on training and serving. This API can be used to trigger a 'join' operation on specified events with latest version of catalog items. It can also be used to correct events joined with wrong catalog items.");
            user_events4 = user_events4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("write").about("Writes a single user event.");
            user_events4 = user_events4.subcommand(mcmd);
        }
        event_stores3 = event_stores3.subcommand(user_events4);
        event_stores3 = event_stores3.subcommand(prediction_api_key_registrations4);
        event_stores3 = event_stores3.subcommand(placements4);
        event_stores3 = event_stores3.subcommand(operations4);
        catalogs2 = catalogs2.subcommand(operations3);
        catalogs2 = catalogs2.subcommand(event_stores3);
        catalogs2 = catalogs2.subcommand(catalog_items3);
        locations1 = locations1.subcommand(catalogs2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_recommendationengine1_beta1 as api;

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
