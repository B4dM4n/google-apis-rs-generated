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
        let mut app = App::new("mybusinessbusinessinformation1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20220426")
            .about("The My Business Business Information API provides an interface for managing business information on Google.")
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
        let mut accounts0 = SubCommand::with_name("accounts")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut attributes0 = SubCommand::with_name("attributes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Returns the list of attributes that would be available for a location with the given primary category and country.");
            attributes0 = attributes0.subcommand(mcmd);
        }
        let mut categories0 = SubCommand::with_name("categories")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_get and list");
        {
            let mcmd = SubCommand::with_name("batch_get").about(
                "Returns a list of business categories for the provided language and GConcept ids.",
            );
            categories0 = categories0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns a list of business categories. Search will match the category name but not the category ID. Search only matches the front of a category name (that is, 'food' may return 'Food Court' but not 'Fast Food Restaurant').");
            categories0 = categories0.subcommand(mcmd);
        }
        let mut chains0 = SubCommand::with_name("chains")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and search");
        {
            let mcmd = SubCommand::with_name("get").about(
                "Gets the specified chain. Returns `NOT_FOUND` if the chain does not exist.",
            );
            chains0 = chains0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("search").about("Searches the chain based on chain name.");
            chains0 = chains0.subcommand(mcmd);
        }
        let mut google_locations0 = SubCommand::with_name("google_locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: search");
        {
            let mcmd = SubCommand::with_name("search").about(
                "Search all of the possible locations that are a match to the specified request.",
            );
            google_locations0 = google_locations0.subcommand(mcmd);
        }
        let mut locations0 = SubCommand::with_name("locations")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: associate, clear_location_association, delete, get, get_attributes, get_google_updated, patch and update_attributes");
        {
            let mcmd = SubCommand::with_name("associate").about("Associates a location to a place ID. Any previous association is overwritten. This operation is only valid if the location is unverified. The association must be valid, that is, it appears in the list of `SearchGoogleLocations`.");
            locations0 = locations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("clear_location_association").about("Clears an association between a location and its place ID. This operation is only valid if the location is unverified.");
            locations0 = locations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a location. If this location cannot be deleted using the API and it is marked so in the `google.mybusiness.businessinformation.v1.LocationState`, use the [Google Business Profile](https://business.google.com/manage/) website.");
            locations0 = locations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified location.");
            locations0 = locations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_attributes")
                .about("Looks up all the attributes set for a given location.");
            locations0 = locations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_google_updated")
                .about("Gets the Google-updated version of the specified location.");
            locations0 = locations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified location.");
            locations0 = locations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_attributes")
                .about("Update attributes for a given location.");
            locations0 = locations0.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create and list");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new Location that will be owned by the logged in user.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the locations for the specified account.");
            locations1 = locations1.subcommand(mcmd);
        }
        let mut attributes1 = SubCommand::with_name("attributes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_google_updated");
        {
            let mcmd = SubCommand::with_name("get_google_updated")
                .about("Gets the Google-updated version of the specified location.");
            attributes1 = attributes1.subcommand(mcmd);
        }
        locations0 = locations0.subcommand(attributes1);
        accounts0 = accounts0.subcommand(locations1);
        app = app.subcommand(locations0);
        app = app.subcommand(google_locations0);
        app = app.subcommand(chains0);
        app = app.subcommand(categories0);
        app = app.subcommand(attributes0);
        app = app.subcommand(accounts0);

        Self { app }
    }
}
use google_mybusinessbusinessinformation1 as api;

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
