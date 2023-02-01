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
        let mut app = App::new("datapipelines1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230121")
            .about("Data Pipelines provides an interface for creating, updating, and managing recurring Data Analytics jobs.")
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
            .about("sub-resources: pipelines");
        let mut pipelines2 = SubCommand::with_name("pipelines")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch, run and stop");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a pipeline. For a batch pipeline, you can pass scheduler information. Data Pipelines uses the scheduler information to create an internal scheduler that runs jobs periodically. If the internal scheduler is not configured, you can use RunPipeline to run jobs.");
            pipelines2 = pipelines2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a pipeline. If a scheduler job is attached to the pipeline, it will be deleted.");
            pipelines2 = pipelines2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Looks up a single pipeline. Returns a \"NOT_FOUND\" error if no such pipeline exists. Returns a \"FORBIDDEN\" error if the caller doesn't have permission to access it.");
            pipelines2 = pipelines2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists pipelines. Returns a \"FORBIDDEN\" error if the caller doesn't have permission to access it.");
            pipelines2 = pipelines2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a pipeline. If successful, the updated Pipeline is returned. Returns `NOT_FOUND` if the pipeline doesn't exist. If UpdatePipeline does not return successfully, you can retry the UpdatePipeline request until you receive a successful response.");
            pipelines2 = pipelines2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("run").about("Creates a job for the specified pipeline directly. You can use this method when the internal scheduler is not configured and you want to trigger the job directly or through an external system. Returns a \"NOT_FOUND\" error if the pipeline doesn't exist. Returns a \"FORBIDDEN\" error if the user doesn't have permission to access the pipeline or run jobs for the pipeline.");
            pipelines2 = pipelines2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("stop").about("Freezes pipeline execution permanently. If there's a corresponding scheduler entry, it's deleted, and the pipeline state is changed to \"ARCHIVED\". However, pipeline metadata is retained.");
            pipelines2 = pipelines2.subcommand(mcmd);
        }
        let mut jobs3 = SubCommand::with_name("jobs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists jobs for a given pipeline. Throws a \"FORBIDDEN\" error if the caller doesn't have permission to access it.");
            jobs3 = jobs3.subcommand(mcmd);
        }
        pipelines2 = pipelines2.subcommand(jobs3);
        locations1 = locations1.subcommand(pipelines2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_datapipelines1 as api;

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
