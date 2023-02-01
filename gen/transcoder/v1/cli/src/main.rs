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
        let mut app = App::new("transcoder1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230125")
            .about("This API converts video files into formats suitable for consumer distribution. For more information, see the Transcoder API overview. ")
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
            .about("sub-resources: job_templates and jobs");
        let mut job_templates2 = SubCommand::with_name("job_templates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a job template in the specified region.");
            job_templates2 = job_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a job template.");
            job_templates2 = job_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the job template data.");
            job_templates2 = job_templates2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists job templates in the specified region.");
            job_templates2 = job_templates2.subcommand(mcmd);
        }
        let mut jobs2 = SubCommand::with_name("jobs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a job in the specified region.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a job.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the job data.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists jobs in the specified region.");
            jobs2 = jobs2.subcommand(mcmd);
        }
        locations1 = locations1.subcommand(jobs2);
        locations1 = locations1.subcommand(job_templates2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_transcoder1 as api;

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
