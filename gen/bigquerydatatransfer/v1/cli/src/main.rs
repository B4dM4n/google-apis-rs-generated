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
        let mut app = App::new("bigquerydatatransfer1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230128")
            .about("Schedule queries or transfer external data from SaaS applications to Google BigQuery on a regular basis.")
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
            .about("methods: enroll_data_sources");
        {
            let mcmd = SubCommand::with_name("enroll_data_sources").about("Enroll data sources in a user project. This allows users to create transfer configurations for these data sources. They will also appear in the ListDataSources RPC and as such, will appear in the [BigQuery UI](https://console.cloud.google.com/bigquery), and the documents can be found in the public guide for [BigQuery Web UI](https://cloud.google.com/bigquery/bigquery-web-ui) and [Data Transfer Service](https://cloud.google.com/bigquery/docs/working-with-transfers).");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut data_sources1 = SubCommand::with_name("data_sources")
            .setting(AppSettings::ColoredHelp)
            .about("methods: check_valid_creds, get and list");
        {
            let mcmd = SubCommand::with_name("check_valid_creds").about("Returns true if valid credentials exist for the given data source and requesting user.");
            data_sources1 = data_sources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves a supported data source and returns its settings.");
            data_sources1 = data_sources1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists supported data sources and returns their settings.");
            data_sources1 = data_sources1.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: enroll_data_sources, get and list");
        {
            let mcmd = SubCommand::with_name("enroll_data_sources").about("Enroll data sources in a user project. This allows users to create transfer configurations for these data sources. They will also appear in the ListDataSources RPC and as such, will appear in the [BigQuery UI](https://console.cloud.google.com/bigquery), and the documents can be found in the public guide for [BigQuery Web UI](https://cloud.google.com/bigquery/bigquery-web-ui) and [Data Transfer Service](https://cloud.google.com/bigquery/docs/working-with-transfers).");
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
        let mut transfer_configs1 = SubCommand::with_name("transfer_configs")
            .setting(AppSettings::ColoredHelp)
            .about(
                "methods: create, delete, get, list, patch, schedule_runs and start_manual_runs",
            );
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a new data transfer configuration.");
            transfer_configs1 = transfer_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a data transfer configuration, including any associated transfer runs and logs.");
            transfer_configs1 = transfer_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns information about a data transfer config.");
            transfer_configs1 = transfer_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns information about all transfer configs owned by a project in the specified location.");
            transfer_configs1 = transfer_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a data transfer configuration. All fields must be set, even if they are not updated.");
            transfer_configs1 = transfer_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("schedule_runs").about("Creates transfer runs for a time range [start_time, end_time]. For each date - or whatever granularity the data source supports - in the range, one transfer run is created. Note that runs are created per UTC time in the time range. DEPRECATED: use StartManualTransferRuns instead.");
            transfer_configs1 = transfer_configs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("start_manual_runs").about("Start manual transfer runs to be executed now with schedule_time equal to current time. The transfer runs can be created for a time range where the run_time is between start_time (inclusive) and end_time (exclusive), or for a specific run_time.");
            transfer_configs1 = transfer_configs1.subcommand(mcmd);
        }
        let mut data_sources2 = SubCommand::with_name("data_sources")
            .setting(AppSettings::ColoredHelp)
            .about("methods: check_valid_creds, get and list");
        {
            let mcmd = SubCommand::with_name("check_valid_creds").about("Returns true if valid credentials exist for the given data source and requesting user.");
            data_sources2 = data_sources2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Retrieves a supported data source and returns its settings.");
            data_sources2 = data_sources2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists supported data sources and returns their settings.");
            data_sources2 = data_sources2.subcommand(mcmd);
        }
        let mut transfer_configs2 = SubCommand::with_name("transfer_configs")
            .setting(AppSettings::ColoredHelp)
            .about(
                "methods: create, delete, get, list, patch, schedule_runs and start_manual_runs",
            );
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a new data transfer configuration.");
            transfer_configs2 = transfer_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a data transfer configuration, including any associated transfer runs and logs.");
            transfer_configs2 = transfer_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns information about a data transfer config.");
            transfer_configs2 = transfer_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns information about all transfer configs owned by a project in the specified location.");
            transfer_configs2 = transfer_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a data transfer configuration. All fields must be set, even if they are not updated.");
            transfer_configs2 = transfer_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("schedule_runs").about("Creates transfer runs for a time range [start_time, end_time]. For each date - or whatever granularity the data source supports - in the range, one transfer run is created. Note that runs are created per UTC time in the time range. DEPRECATED: use StartManualTransferRuns instead.");
            transfer_configs2 = transfer_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("start_manual_runs").about("Start manual transfer runs to be executed now with schedule_time equal to current time. The transfer runs can be created for a time range where the run_time is between start_time (inclusive) and end_time (exclusive), or for a specific run_time.");
            transfer_configs2 = transfer_configs2.subcommand(mcmd);
        }
        let mut runs2 = SubCommand::with_name("runs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified transfer run.");
            runs2 = runs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns information about the particular transfer run.");
            runs2 = runs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns information about running and completed transfer runs.");
            runs2 = runs2.subcommand(mcmd);
        }
        let mut runs3 = SubCommand::with_name("runs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified transfer run.");
            runs3 = runs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns information about the particular transfer run.");
            runs3 = runs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns information about running and completed transfer runs.");
            runs3 = runs3.subcommand(mcmd);
        }
        let mut transfer_logs3 = SubCommand::with_name("transfer_logs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd =
                SubCommand::with_name("list").about("Returns log messages for the transfer run.");
            transfer_logs3 = transfer_logs3.subcommand(mcmd);
        }
        let mut transfer_logs4 = SubCommand::with_name("transfer_logs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd =
                SubCommand::with_name("list").about("Returns log messages for the transfer run.");
            transfer_logs4 = transfer_logs4.subcommand(mcmd);
        }
        runs3 = runs3.subcommand(transfer_logs4);
        runs2 = runs2.subcommand(transfer_logs3);
        transfer_configs2 = transfer_configs2.subcommand(runs3);
        transfer_configs1 = transfer_configs1.subcommand(runs2);
        locations1 = locations1.subcommand(transfer_configs2);
        locations1 = locations1.subcommand(data_sources2);
        projects0 = projects0.subcommand(transfer_configs1);
        projects0 = projects0.subcommand(locations1);
        projects0 = projects0.subcommand(data_sources1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_bigquerydatatransfer1 as api;

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
