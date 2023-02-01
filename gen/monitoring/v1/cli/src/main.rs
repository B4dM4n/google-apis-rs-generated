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
        let mut app = App::new("monitoring1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230123")
            .about("Manages your Cloud Monitoring data and configurations.")
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
        let mut locations0 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: global");
        let mut operations0 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: dashboards and location");
        let mut global1 = SubCommand::with_name("global")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: metrics_scopes");
        let mut dashboards1 = SubCommand::with_name("dashboards")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new custom dashboard. For examples on how you can use this API to create dashboards, see Managing dashboards by API (https://cloud.google.com/monitoring/dashboards/api-dashboard). This method requires the monitoring.dashboards.create permission on the specified project. For more information about permissions, see Cloud Identity and Access Management (https://cloud.google.com/iam).");
            dashboards1 = dashboards1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an existing custom dashboard.This method requires the monitoring.dashboards.delete permission on the specified dashboard. For more information, see Cloud Identity and Access Management (https://cloud.google.com/iam).");
            dashboards1 = dashboards1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Fetches a specific dashboard.This method requires the monitoring.dashboards.get permission on the specified dashboard. For more information, see Cloud Identity and Access Management (https://cloud.google.com/iam).");
            dashboards1 = dashboards1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the existing dashboards.This method requires the monitoring.dashboards.list permission on the specified project. For more information, see Cloud Identity and Access Management (https://cloud.google.com/iam).");
            dashboards1 = dashboards1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Replaces an existing custom dashboard with a new definition.This method requires the monitoring.dashboards.update permission on the specified dashboard. For more information, see Cloud Identity and Access Management (https://cloud.google.com/iam).");
            dashboards1 = dashboards1.subcommand(mcmd);
        }
        let mut location1 = SubCommand::with_name("location")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: prometheus");
        let mut metrics_scopes2 = SubCommand::with_name("metrics_scopes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list_metrics_scopes_by_monitored_project");
        {
            let mcmd = SubCommand::with_name("get").about("Returns a specific Metrics Scope, including the list of projects monitored by the specified Metrics Scope.");
            metrics_scopes2 = metrics_scopes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_metrics_scopes_by_monitored_project").about("Returns a list of every Metrics Scope that a specific MonitoredProject has been added to. The metrics scope representing the specified monitored project will always be the first entry in the response.");
            metrics_scopes2 = metrics_scopes2.subcommand(mcmd);
        }
        let mut prometheus2 = SubCommand::with_name("prometheus")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: api");
        let mut projects3 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create and delete");
        {
            let mcmd = SubCommand::with_name("create").about(
                "Adds a MonitoredProject with the given project ID to the specified Metrics Scope.",
            );
            projects3 = projects3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes a MonitoredProject from the specified Metrics Scope.");
            projects3 = projects3.subcommand(mcmd);
        }
        let mut api3 = SubCommand::with_name("api")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: v_1");
        let mut v_14 = SubCommand::with_name("v_1")
            .setting(AppSettings::ColoredHelp)
            .about("methods: labels_method, query, query_range and series");
        {
            let mcmd = SubCommand::with_name("labels_method").about("Lists labels for metrics.");
            v_14 = v_14.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("query")
                .about("Evaluate a PromQL query at a single point in time.");
            v_14 = v_14.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("query_range")
                .about("Evaluate a PromQL query with start, end time range.");
            v_14 = v_14.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("series").about("Lists metadata for metrics.");
            v_14 = v_14.subcommand(mcmd);
        }
        let mut label5 = SubCommand::with_name("label")
            .setting(AppSettings::ColoredHelp)
            .about("methods: values");
        {
            let mcmd = SubCommand::with_name("values")
                .about("Lists possible values for a given label name.");
            label5 = label5.subcommand(mcmd);
        }
        let mut labels5 = SubCommand::with_name("labels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists labels for metrics.");
            labels5 = labels5.subcommand(mcmd);
        }
        let mut metadata5 = SubCommand::with_name("metadata")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists metadata for metrics.");
            metadata5 = metadata5.subcommand(mcmd);
        }
        v_14 = v_14.subcommand(metadata5);
        v_14 = v_14.subcommand(labels5);
        v_14 = v_14.subcommand(label5);
        api3 = api3.subcommand(v_14);
        prometheus2 = prometheus2.subcommand(api3);
        metrics_scopes2 = metrics_scopes2.subcommand(projects3);
        location1 = location1.subcommand(prometheus2);
        global1 = global1.subcommand(metrics_scopes2);
        projects0 = projects0.subcommand(location1);
        projects0 = projects0.subcommand(dashboards1);
        locations0 = locations0.subcommand(global1);
        app = app.subcommand(projects0);
        app = app.subcommand(operations0);
        app = app.subcommand(locations0);

        Self { app }
    }
}
use google_monitoring1 as api;

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
