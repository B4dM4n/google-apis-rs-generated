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
        let mut app = App::new("analyticsadmin1_beta")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230131")
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
        let mut account_summaries0 = SubCommand::with_name("account_summaries")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns summaries of all accounts accessible by the caller.");
            account_summaries0 = account_summaries0.subcommand(mcmd);
        }
        let mut accounts0 = SubCommand::with_name("accounts")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: delete, get, get_data_sharing_settings, list, patch, provision_account_ticket and search_change_history_events");
        {
            let mcmd = SubCommand::with_name("delete").about("Marks target Account as soft-deleted (ie: \"trashed\") and returns it. This API does not have a method to restore soft-deleted accounts. However, they can be restored using the Trash Can UI. If the accounts are not restored before the expiration time, the account and all child resources (eg: Properties, GoogleAdsLinks, Streams, UserLinks) will be permanently purged. https://support.google.com/analytics/answer/6154772 Returns an error if the target is not found.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Lookup for a single Account.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_data_sharing_settings").about(
                "Get data sharing settings on an account. Data sharing settings are singletons.",
            );
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns all accounts accessible by the caller. Note that these accounts might not currently have GA4 properties. Soft-deleted (ie: \"trashed\") accounts are excluded by default. Returns an empty list if no relevant accounts are found.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an account.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("provision_account_ticket")
                .about("Requests a ticket for creating an account.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_change_history_events").about("Searches through all changes to an account or its children given the specified set of filters.");
            accounts0 = accounts0.subcommand(mcmd);
        }
        let mut properties0 = SubCommand::with_name("properties")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: acknowledge_user_data_collection, create, delete, get, get_data_retention_settings, list, patch and update_data_retention_settings");
        {
            let mcmd = SubCommand::with_name("acknowledge_user_data_collection").about("Acknowledges the terms of user data collection for the specified property. This acknowledgement must be completed (either in the Google Analytics UI or via this API) before MeasurementProtocolSecret resources may be created.");
            properties0 = properties0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates an \"GA4\" property with the specified location and attributes.");
            properties0 = properties0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Marks target Property as soft-deleted (ie: \"trashed\") and returns it. This API does not have a method to restore soft-deleted properties. However, they can be restored using the Trash Can UI. If the properties are not restored before the expiration time, the Property and all child resources (eg: GoogleAdsLinks, Streams, UserLinks) will be permanently purged. https://support.google.com/analytics/answer/6154772 Returns an error if the target is not found, or is not a GA4 Property.");
            properties0 = properties0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Lookup for a single \"GA4\" Property.");
            properties0 = properties0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_data_retention_settings")
                .about("Returns the singleton data retention settings for this property.");
            properties0 = properties0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns child Properties under the specified parent Account. Only \"GA4\" properties will be returned. Properties will be excluded if the caller does not have access. Soft-deleted (ie: \"trashed\") properties are excluded by default. Returns an empty list if no relevant properties are found.");
            properties0 = properties0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a property.");
            properties0 = properties0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_data_retention_settings")
                .about("Updates the singleton data retention settings for this property.");
            properties0 = properties0.subcommand(mcmd);
        }
        let mut conversion_events1 = SubCommand::with_name("conversion_events")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a conversion event with the specified attributes.");
            conversion_events1 = conversion_events1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes a conversion event in a property.");
            conversion_events1 = conversion_events1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieve a single conversion event.");
            conversion_events1 = conversion_events1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns a list of conversion events in the specified parent property. Returns an empty list if no conversion events are found.");
            conversion_events1 = conversion_events1.subcommand(mcmd);
        }
        let mut custom_dimensions1 = SubCommand::with_name("custom_dimensions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: archive, create, get, list and patch");
        {
            let mcmd =
                SubCommand::with_name("archive").about("Archives a CustomDimension on a property.");
            custom_dimensions1 = custom_dimensions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a CustomDimension.");
            custom_dimensions1 = custom_dimensions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Lookup for a single CustomDimension.");
            custom_dimensions1 = custom_dimensions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists CustomDimensions on a property.");
            custom_dimensions1 = custom_dimensions1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates a CustomDimension on a property.");
            custom_dimensions1 = custom_dimensions1.subcommand(mcmd);
        }
        let mut custom_metrics1 = SubCommand::with_name("custom_metrics")
            .setting(AppSettings::ColoredHelp)
            .about("methods: archive, create, get, list and patch");
        {
            let mcmd =
                SubCommand::with_name("archive").about("Archives a CustomMetric on a property.");
            custom_metrics1 = custom_metrics1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a CustomMetric.");
            custom_metrics1 = custom_metrics1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Lookup for a single CustomMetric.");
            custom_metrics1 = custom_metrics1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists CustomMetrics on a property.");
            custom_metrics1 = custom_metrics1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates a CustomMetric on a property.");
            custom_metrics1 = custom_metrics1.subcommand(mcmd);
        }
        let mut data_streams1 = SubCommand::with_name("data_streams")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a DataStream.");
            data_streams1 = data_streams1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a DataStream on a property.");
            data_streams1 = data_streams1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Lookup for a single DataStream.");
            data_streams1 = data_streams1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists DataStreams on a property.");
            data_streams1 = data_streams1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a DataStream on a property.");
            data_streams1 = data_streams1.subcommand(mcmd);
        }
        let mut firebase_links1 = SubCommand::with_name("firebase_links")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete and list");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a FirebaseLink. Properties can have at most one FirebaseLink.");
            firebase_links1 = firebase_links1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes a FirebaseLink on a property");
            firebase_links1 = firebase_links1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists FirebaseLinks on a property. Properties can have at most one FirebaseLink.",
            );
            firebase_links1 = firebase_links1.subcommand(mcmd);
        }
        let mut google_ads_links1 = SubCommand::with_name("google_ads_links")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a GoogleAdsLink.");
            google_ads_links1 = google_ads_links1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes a GoogleAdsLink on a property");
            google_ads_links1 = google_ads_links1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists GoogleAdsLinks on a property.");
            google_ads_links1 = google_ads_links1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates a GoogleAdsLink on a property");
            google_ads_links1 = google_ads_links1.subcommand(mcmd);
        }
        let mut measurement_protocol_secrets2 =
            SubCommand::with_name("measurement_protocol_secrets")
                .setting(AppSettings::ColoredHelp)
                .about("methods: create, delete, get, list and patch");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a measurement protocol secret.");
            measurement_protocol_secrets2 = measurement_protocol_secrets2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes target MeasurementProtocolSecret.");
            measurement_protocol_secrets2 = measurement_protocol_secrets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Lookup for a single \"GA4\" MeasurementProtocolSecret.");
            measurement_protocol_secrets2 = measurement_protocol_secrets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Returns child MeasurementProtocolSecrets under the specified parent Property.",
            );
            measurement_protocol_secrets2 = measurement_protocol_secrets2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates a measurement protocol secret.");
            measurement_protocol_secrets2 = measurement_protocol_secrets2.subcommand(mcmd);
        }
        data_streams1 = data_streams1.subcommand(measurement_protocol_secrets2);
        properties0 = properties0.subcommand(google_ads_links1);
        properties0 = properties0.subcommand(firebase_links1);
        properties0 = properties0.subcommand(data_streams1);
        properties0 = properties0.subcommand(custom_metrics1);
        properties0 = properties0.subcommand(custom_dimensions1);
        properties0 = properties0.subcommand(conversion_events1);
        app = app.subcommand(properties0);
        app = app.subcommand(accounts0);
        app = app.subcommand(account_summaries0);

        Self { app }
    }
}
use google_analyticsadmin1_beta as api;

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
