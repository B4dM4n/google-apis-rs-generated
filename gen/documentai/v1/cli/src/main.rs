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
        let mut app = App::new("documentai1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20220409")
            .about("Service to parse structured information from unstructured or semi-structured documents using state-of-the-art Google AI such as natural language, computer vision, translation, and AutoML.")
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
        let mut operations0 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations and operations");
        let mut uiv_1beta_30 = SubCommand::with_name("uiv_1beta_3")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: projects");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: fetch_processor_types, get and list");
        {
            let mcmd = SubCommand::with_name("fetch_processor_types").about("Fetches processor types. Note that we do not use ListProcessorTypes here because it is not paginated.");
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
        let mut operations1 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations1 = operations1.subcommand(mcmd);
        }
        let mut projects1 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut processors2 = SubCommand::with_name("processors")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: batch_process, create, delete, disable, enable, get, list, process and set_default_processor_version");
        {
            let mcmd = SubCommand::with_name("batch_process").about("LRO endpoint to batch process many documents. The output is written to Cloud Storage as JSON in the [Document] format.");
            processors2 = processors2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a processor from the type processor that the user chose. The processor will be at \"ENABLED\" state by default after its creation.");
            processors2 = processors2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the processor, unloads all deployed model artifacts if it was enabled and then deletes all artifacts associated with this processor.");
            processors2 = processors2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("disable").about("Disables a processor");
            processors2 = processors2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("enable").about("Enables a processor");
            processors2 = processors2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a processor detail.");
            processors2 = processors2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all processors which belong to this project.");
            processors2 = processors2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("process").about("Processes a single document.");
            processors2 = processors2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_default_processor_version").about("Set the default (active) version of a Processor that will be used in ProcessDocument and BatchProcessDocuments.");
            processors2 = processors2.subcommand(mcmd);
        }
        let mut locations2 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a location.");
            locations2 = locations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists information about the supported locations for this service.");
            locations2 = locations2.subcommand(mcmd);
        }
        let mut human_review_config3 = SubCommand::with_name("human_review_config")
            .setting(AppSettings::ColoredHelp)
            .about("methods: review_document");
        {
            let mcmd = SubCommand::with_name("review_document").about("Send a document for Human Review. The input document should be processed by the specified processor.");
            human_review_config3 = human_review_config3.subcommand(mcmd);
        }
        let mut processor_versions3 = SubCommand::with_name("processor_versions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_process, delete, deploy, get, list, process and undeploy");
        {
            let mcmd = SubCommand::with_name("batch_process").about("LRO endpoint to batch process many documents. The output is written to Cloud Storage as JSON in the [Document] format.");
            processor_versions3 = processor_versions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the processor version, all artifacts under the processor version will be deleted.");
            processor_versions3 = processor_versions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("deploy").about("Deploys the processor version.");
            processor_versions3 = processor_versions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a processor version detail.");
            processor_versions3 = processor_versions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all versions of a processor.");
            processor_versions3 = processor_versions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("process").about("Processes a single document.");
            processor_versions3 = processor_versions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undeploy").about("Undeploys the processor version.");
            processor_versions3 = processor_versions3.subcommand(mcmd);
        }
        let mut operations3 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
            operations3 = operations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations3 = operations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations3 = operations3.subcommand(mcmd);
        }
        locations2 = locations2.subcommand(operations3);
        processors2 = processors2.subcommand(processor_versions3);
        processors2 = processors2.subcommand(human_review_config3);
        projects1 = projects1.subcommand(locations2);
        locations1 = locations1.subcommand(processors2);
        locations1 = locations1.subcommand(operations2);
        uiv_1beta_30 = uiv_1beta_30.subcommand(projects1);
        projects0 = projects0.subcommand(operations1);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(uiv_1beta_30);
        app = app.subcommand(projects0);
        app = app.subcommand(operations0);

        Self { app }
    }
}
use google_documentai1 as api;

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
