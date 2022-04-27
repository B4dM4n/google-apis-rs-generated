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
        let mut app = App::new("datalabeling1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20220409")
            .about("Public API for Google Cloud AI Data Labeling Service.")
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
                        .about("sub-resources: annotation_spec_sets, datasets, evaluation_jobs, evaluations, instructions and operations");
        let mut annotation_spec_sets1 = SubCommand::with_name("annotation_spec_sets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates an annotation spec set by providing a set of labels.");
            annotation_spec_sets1 = annotation_spec_sets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes an annotation spec set by resource name.");
            annotation_spec_sets1 = annotation_spec_sets1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets an annotation spec set by resource name.");
            annotation_spec_sets1 = annotation_spec_sets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists annotation spec sets for a project. Pagination is supported.");
            annotation_spec_sets1 = annotation_spec_sets1.subcommand(mcmd);
        }
        let mut datasets1 = SubCommand::with_name("datasets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, export_data, get, import_data and list");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates dataset. If success return a Dataset resource.");
            datasets1 = datasets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a dataset by resource name.");
            datasets1 = datasets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("export_data")
                .about("Exports data and annotations from dataset.");
            datasets1 = datasets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets dataset by resource name.");
            datasets1 = datasets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import_data").about("Imports data into dataset based on source locations defined in request. It can be called multiple times for the same dataset. Each dataset can only have one long running operation running on it. For example, no labeling task (also long running operation) can be started while importing is still ongoing. Vice versa.");
            datasets1 = datasets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists datasets under a project. Pagination is supported.");
            datasets1 = datasets1.subcommand(mcmd);
        }
        let mut evaluation_jobs1 = SubCommand::with_name("evaluation_jobs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch, pause and resume");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an evaluation job.");
            evaluation_jobs1 = evaluation_jobs1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Stops and deletes an evaluation job.");
            evaluation_jobs1 = evaluation_jobs1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets an evaluation job by resource name.");
            evaluation_jobs1 = evaluation_jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all evaluation jobs within a project with possible filters. Pagination is supported.");
            evaluation_jobs1 = evaluation_jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an evaluation job. You can only update certain fields of the job's EvaluationJobConfig: `humanAnnotationConfig.instruction`, `exampleCount`, and `exampleSamplePercentage`. If you want to change any other aspect of the evaluation job, you must delete the job and create a new one.");
            evaluation_jobs1 = evaluation_jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("pause").about("Pauses an evaluation job. Pausing an evaluation job that is already in a `PAUSED` state is a no-op.");
            evaluation_jobs1 = evaluation_jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("resume").about("Resumes a paused evaluation job. A deleted evaluation job can't be resumed. Resuming a running or scheduled evaluation job is a no-op.");
            evaluation_jobs1 = evaluation_jobs1.subcommand(mcmd);
        }
        let mut evaluations1 = SubCommand::with_name("evaluations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: search");
        {
            let mcmd =
                SubCommand::with_name("search").about("Searches evaluations within a project.");
            evaluations1 = evaluations1.subcommand(mcmd);
        }
        let mut instructions1 = SubCommand::with_name("instructions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates an instruction for how data should be labeled.");
            instructions1 = instructions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes an instruction object by resource name.");
            instructions1 = instructions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an instruction by resource name.");
            instructions1 = instructions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists instructions for a project. Pagination is supported.");
            instructions1 = instructions1.subcommand(mcmd);
        }
        let mut operations1 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, delete, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations1 = operations1.subcommand(mcmd);
        }
        let mut annotated_datasets2 = SubCommand::with_name("annotated_datasets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get and list");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes an annotated dataset by resource name.");
            annotated_datasets2 = annotated_datasets2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets an annotated dataset by resource name.");
            annotated_datasets2 = annotated_datasets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists annotated datasets for a dataset. Pagination is supported.");
            annotated_datasets2 = annotated_datasets2.subcommand(mcmd);
        }
        let mut data_items2 = SubCommand::with_name("data_items")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a data item in a dataset by resource name. This API can be called after data are imported into dataset.");
            data_items2 = data_items2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists data items in a dataset. This API can be called after data are imported into dataset. Pagination is supported.");
            data_items2 = data_items2.subcommand(mcmd);
        }
        let mut evaluations2 = SubCommand::with_name("evaluations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about(
                "Gets an evaluation by resource name (to search, use projects.evaluations.search).",
            );
            evaluations2 = evaluations2.subcommand(mcmd);
        }
        let mut image2 = SubCommand::with_name("image")
            .setting(AppSettings::ColoredHelp)
            .about("methods: label");
        {
            let mcmd = SubCommand::with_name("label").about("Starts a labeling task for image. The type of image labeling task is configured by feature in the request.");
            image2 = image2.subcommand(mcmd);
        }
        let mut text2 = SubCommand::with_name("text")
            .setting(AppSettings::ColoredHelp)
            .about("methods: label");
        {
            let mcmd = SubCommand::with_name("label").about("Starts a labeling task for text. The type of text labeling task is configured by feature in the request.");
            text2 = text2.subcommand(mcmd);
        }
        let mut video2 = SubCommand::with_name("video")
            .setting(AppSettings::ColoredHelp)
            .about("methods: label");
        {
            let mcmd = SubCommand::with_name("label").about("Starts a labeling task for video. The type of video labeling task is configured by feature in the request.");
            video2 = video2.subcommand(mcmd);
        }
        let mut data_items3 = SubCommand::with_name("data_items")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a data item in a dataset by resource name. This API can be called after data are imported into dataset.");
            data_items3 = data_items3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists data items in a dataset. This API can be called after data are imported into dataset. Pagination is supported.");
            data_items3 = data_items3.subcommand(mcmd);
        }
        let mut examples3 = SubCommand::with_name("examples")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets an example by resource name, including both data and annotation.");
            examples3 = examples3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists examples in an annotated dataset. Pagination is supported.");
            examples3 = examples3.subcommand(mcmd);
        }
        let mut feedback_threads3 = SubCommand::with_name("feedback_threads")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a FeedbackThread.");
            feedback_threads3 = feedback_threads3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get a FeedbackThread object.");
            feedback_threads3 = feedback_threads3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List FeedbackThreads with pagination.");
            feedback_threads3 = feedback_threads3.subcommand(mcmd);
        }
        let mut example_comparisons3 = SubCommand::with_name("example_comparisons")
            .setting(AppSettings::ColoredHelp)
            .about("methods: search");
        {
            let mcmd = SubCommand::with_name("search").about("Searches example comparisons from an evaluation. The return format is a list of example comparisons that show ground truth and prediction(s) for a single input. Search by providing an evaluation ID.");
            example_comparisons3 = example_comparisons3.subcommand(mcmd);
        }
        let mut feedback_messages4 = SubCommand::with_name("feedback_messages")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Create a FeedbackMessage object.");
            feedback_messages4 = feedback_messages4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a FeedbackMessage.");
            feedback_messages4 = feedback_messages4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get a FeedbackMessage object.");
            feedback_messages4 = feedback_messages4.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("List FeedbackMessages with pagination.");
            feedback_messages4 = feedback_messages4.subcommand(mcmd);
        }
        feedback_threads3 = feedback_threads3.subcommand(feedback_messages4);
        evaluations2 = evaluations2.subcommand(example_comparisons3);
        annotated_datasets2 = annotated_datasets2.subcommand(feedback_threads3);
        annotated_datasets2 = annotated_datasets2.subcommand(examples3);
        annotated_datasets2 = annotated_datasets2.subcommand(data_items3);
        datasets1 = datasets1.subcommand(video2);
        datasets1 = datasets1.subcommand(text2);
        datasets1 = datasets1.subcommand(image2);
        datasets1 = datasets1.subcommand(evaluations2);
        datasets1 = datasets1.subcommand(data_items2);
        datasets1 = datasets1.subcommand(annotated_datasets2);
        projects0 = projects0.subcommand(operations1);
        projects0 = projects0.subcommand(instructions1);
        projects0 = projects0.subcommand(evaluations1);
        projects0 = projects0.subcommand(evaluation_jobs1);
        projects0 = projects0.subcommand(datasets1);
        projects0 = projects0.subcommand(annotation_spec_sets1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_datalabeling1_beta1 as api;

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
