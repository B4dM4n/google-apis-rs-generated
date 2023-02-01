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
        let mut app = App::new("searchads0")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20221208")
            .about("The Search Ads 360 API allows developers to automate downloading reports from Search Ads 360.")
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
        let mut customers0 = SubCommand::with_name("customers")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: custom_columns and search_ads_360");
        let mut search_ads_360_fields0 = SubCommand::with_name("search_ads_360_fields")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and search");
        {
            let mcmd = SubCommand::with_name("get").about("Returns just the requested field. List of thrown errors: [AuthenticationError]() [AuthorizationError]() [HeaderError]() [InternalError]() [QuotaError]() [RequestError]()");
            search_ads_360_fields0 = search_ads_360_fields0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search").about("Returns all fields that match the search query. List of thrown errors: [AuthenticationError]() [AuthorizationError]() [HeaderError]() [InternalError]() [QueryError]() [QuotaError]() [RequestError]()");
            search_ads_360_fields0 = search_ads_360_fields0.subcommand(mcmd);
        }
        let mut custom_columns1 = SubCommand::with_name("custom_columns")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns the requested custom column in full detail.");
            custom_columns1 = custom_columns1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Returns all the custom columns associated with the customer in full detail.",
            );
            custom_columns1 = custom_columns1.subcommand(mcmd);
        }
        let mut search_ads_3601 = SubCommand::with_name("search_ads_360")
            .setting(AppSettings::ColoredHelp)
            .about("methods: search and search_stream");
        {
            let mcmd = SubCommand::with_name("search").about("Returns all rows that match the search query. List of thrown errors: [AuthenticationError]() [AuthorizationError]() [HeaderError]() [InternalError]() [QueryError]() [QuotaError]() [RequestError]()");
            search_ads_3601 = search_ads_3601.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_stream").about("Returns all rows that match the search stream query. List of thrown errors: [AuthenticationError]() [AuthorizationError]() [HeaderError]() [InternalError]() [QueryError]() [QuotaError]() [RequestError]()");
            search_ads_3601 = search_ads_3601.subcommand(mcmd);
        }
        customers0 = customers0.subcommand(search_ads_3601);
        customers0 = customers0.subcommand(custom_columns1);
        app = app.subcommand(search_ads_360_fields0);
        app = app.subcommand(customers0);

        Self { app }
    }
}
use google_searchads0 as api;

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
