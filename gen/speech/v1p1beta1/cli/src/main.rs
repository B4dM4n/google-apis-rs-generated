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
        let mut app = App::new("speech1_p1beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230119")
            .about("Converts audio to text by applying powerful neural network models.")
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
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut speech0 = SubCommand::with_name("speech")
            .setting(AppSettings::ColoredHelp)
            .about("methods: longrunningrecognize and recognize");
        {
            let mcmd = SubCommand::with_name("longrunningrecognize").about("Performs asynchronous speech recognition: receive results via the google.longrunning.Operations interface. Returns either an `Operation.error` or an `Operation.response` which contains a `LongRunningRecognizeResponse` message. For more information on asynchronous speech recognition, see the [how-to](https://cloud.google.com/speech-to-text/docs/async-recognize).");
            speech0 = speech0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("recognize").about("Performs synchronous speech recognition: receive results after all audio has been sent and processed.");
            speech0 = speech0.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: custom_classes and phrase_sets");
        let mut custom_classes2 = SubCommand::with_name("custom_classes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Create a custom class.");
            custom_classes2 = custom_classes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a custom class.");
            custom_classes2 = custom_classes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get a custom class.");
            custom_classes2 = custom_classes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List custom classes.");
            custom_classes2 = custom_classes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update a custom class.");
            custom_classes2 = custom_classes2.subcommand(mcmd);
        }
        let mut phrase_sets2 = SubCommand::with_name("phrase_sets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Create a set of phrase hints. Each item in the set can be a single word or a multi-word phrase. The items in the PhraseSet are favored by the recognition model when you send a call that includes the PhraseSet.");
            phrase_sets2 = phrase_sets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a phrase set.");
            phrase_sets2 = phrase_sets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get a phrase set.");
            phrase_sets2 = phrase_sets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List phrase sets.");
            phrase_sets2 = phrase_sets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update a phrase set.");
            phrase_sets2 = phrase_sets2.subcommand(mcmd);
        }
        locations1 = locations1.subcommand(phrase_sets2);
        locations1 = locations1.subcommand(custom_classes2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(speech0);
        app = app.subcommand(projects0);
        app = app.subcommand(operations0);

        Self { app }
    }
}
use google_speech1_p1beta1 as api;

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
