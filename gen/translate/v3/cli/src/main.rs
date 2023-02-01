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
        let mut app = App::new("translate3")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230120")
            .about("Integrates text translation into your website or application.")
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
            .about("methods: detect_language, get_supported_languages and translate_text");
        {
            let mcmd = SubCommand::with_name("detect_language")
                .about("Detects the language of text within a request.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_supported_languages")
                .about("Returns a list of supported languages for translation.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("translate_text")
                .about("Translates input text and returns translated text.");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: batch_translate_document, batch_translate_text, detect_language, get, get_supported_languages, list, translate_document and translate_text");
        {
            let mcmd = SubCommand::with_name("batch_translate_document").about("Translates a large volume of document in asynchronous batch mode. This function provides real-time output as the inputs are being processed. If caller cancels a request, the partial results (for an input file, it's all or nothing) may still be available on the specified output location. This call returns immediately and you can use google.longrunning.Operation.name to poll the status of the call.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_translate_text").about("Translates a large volume of text in asynchronous batch mode. This function provides real-time output as the inputs are being processed. If caller cancels a request, the partial results (for an input file, it's all or nothing) may still be available on the specified output location. This call returns immediately and you can use google.longrunning.Operation.name to poll the status of the call.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("detect_language")
                .about("Detects the language of text within a request.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a location.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_supported_languages")
                .about("Returns a list of supported languages for translation.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists information about the supported locations for this service.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("translate_document")
                .about("Translates documents in synchronous mode.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("translate_text")
                .about("Translates input text and returns translated text.");
            locations1 = locations1.subcommand(mcmd);
        }
        let mut datasets2 = SubCommand::with_name("datasets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, export_data, get, import_data and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a Dataset.");
            datasets2 = datasets2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes a dataset and all of its contents.");
            datasets2 = datasets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("export_data")
                .about("Exports dataset's data to the provided output location.");
            datasets2 = datasets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a Dataset.");
            datasets2 = datasets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import_data")
                .about("Import sentence pairs into translation Dataset.");
            datasets2 = datasets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists datasets.");
            datasets2 = datasets2.subcommand(mcmd);
        }
        let mut glossaries2 = SubCommand::with_name("glossaries")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a glossary and returns the long-running operation. Returns NOT_FOUND, if the project doesn't exist.");
            glossaries2 = glossaries2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a glossary, or cancels glossary construction if the glossary isn't created yet. Returns NOT_FOUND, if the glossary doesn't exist.");
            glossaries2 = glossaries2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets a glossary. Returns NOT_FOUND, if the glossary doesn't exist.");
            glossaries2 = glossaries2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists glossaries in a project. Returns NOT_FOUND, if the project doesn't exist.",
            );
            glossaries2 = glossaries2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a glossary. A LRO is used since the update can be async if the glossary's entry file is updated.");
            glossaries2 = glossaries2.subcommand(mcmd);
        }
        let mut models2 = SubCommand::with_name("models")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a Model.");
            models2 = models2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a model.");
            models2 = models2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a model.");
            models2 = models2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists models.");
            models2 = models2.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, delete, get, list and wait");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.");
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
        {
            let mcmd = SubCommand::with_name("wait").about("Waits until the specified long-running operation is done or reaches at most a specified timeout, returning the latest state. If the operation is already done, the latest state is immediately returned. If the timeout specified is greater than the default HTTP/RPC timeout, the HTTP/RPC timeout is used. If the server does not support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Note that this method is on a best-effort basis. It may return the latest state before the specified timeout (including immediately), meaning even an immediate response is no guarantee that the operation is done.");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut examples3 = SubCommand::with_name("examples")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists sentence pairs in the dataset.");
            examples3 = examples3.subcommand(mcmd);
        }
        let mut glossary_entries3 = SubCommand::with_name("glossary_entries")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a glossary entry.");
            glossary_entries3 = glossary_entries3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes a single entry from the glossary");
            glossary_entries3 = glossary_entries3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets a single glossary entry by the given id.");
            glossary_entries3 = glossary_entries3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List the entries for the glossary.");
            glossary_entries3 = glossary_entries3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a glossary entry.");
            glossary_entries3 = glossary_entries3.subcommand(mcmd);
        }
        glossaries2 = glossaries2.subcommand(glossary_entries3);
        datasets2 = datasets2.subcommand(examples3);
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(models2);
        locations1 = locations1.subcommand(glossaries2);
        locations1 = locations1.subcommand(datasets2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_translate3 as api;

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
