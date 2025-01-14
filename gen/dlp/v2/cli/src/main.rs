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
        let mut app = App::new("dlp2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230129")
            .about("Provides methods for detection, risk analysis, and de-identification of privacy-sensitive fragments in text, images, and Google Cloud Platform storage repositories.")
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
        let mut info_types0 = SubCommand::with_name("info_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Returns a list of the sensitive information types that DLP API supports. See https://cloud.google.com/dlp/docs/infotypes-reference to learn more.");
            info_types0 = info_types0.subcommand(mcmd);
        }
        let mut locations0 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: info_types");
        let mut organizations0 = SubCommand::with_name("organizations")
                        .setting(AppSettings::ColoredHelp)
                        .about("sub-resources: deidentify_templates, inspect_templates, locations and stored_info_types");
        let mut projects0 = SubCommand::with_name("projects")
                        .setting(AppSettings::ColoredHelp)
                        .about("sub-resources: content, deidentify_templates, dlp_jobs, image, inspect_templates, job_triggers, locations and stored_info_types");
        let mut info_types1 = SubCommand::with_name("info_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Returns a list of the sensitive information types that DLP API supports. See https://cloud.google.com/dlp/docs/infotypes-reference to learn more.");
            info_types1 = info_types1.subcommand(mcmd);
        }
        let mut deidentify_templates1 = SubCommand::with_name("deidentify_templates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a DeidentifyTemplate for reusing frequently used configuration for de-identifying content, images, and storage. See https://cloud.google.com/dlp/docs/creating-templates-deid to learn more.");
            deidentify_templates1 = deidentify_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a DeidentifyTemplate. See https://cloud.google.com/dlp/docs/creating-templates-deid to learn more.");
            deidentify_templates1 = deidentify_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a DeidentifyTemplate. See https://cloud.google.com/dlp/docs/creating-templates-deid to learn more.");
            deidentify_templates1 = deidentify_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists DeidentifyTemplates. See https://cloud.google.com/dlp/docs/creating-templates-deid to learn more.");
            deidentify_templates1 = deidentify_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the DeidentifyTemplate. See https://cloud.google.com/dlp/docs/creating-templates-deid to learn more.");
            deidentify_templates1 = deidentify_templates1.subcommand(mcmd);
        }
        let mut inspect_templates1 = SubCommand::with_name("inspect_templates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an InspectTemplate for reusing frequently used configuration for inspecting content, images, and storage. See https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates1 = inspect_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an InspectTemplate. See https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates1 = inspect_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an InspectTemplate. See https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates1 = inspect_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists InspectTemplates. See https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates1 = inspect_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the InspectTemplate. See https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates1 = inspect_templates1.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
                        .setting(AppSettings::ColoredHelp)
                        .about("sub-resources: deidentify_templates, dlp_jobs, inspect_templates, job_triggers and stored_info_types");
        let mut stored_info_types1 = SubCommand::with_name("stored_info_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a pre-built stored infoType to be used for inspection. See https://cloud.google.com/dlp/docs/creating-stored-infotypes to learn more.");
            stored_info_types1 = stored_info_types1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a stored infoType. See https://cloud.google.com/dlp/docs/creating-stored-infotypes to learn more.");
            stored_info_types1 = stored_info_types1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a stored infoType. See https://cloud.google.com/dlp/docs/creating-stored-infotypes to learn more.");
            stored_info_types1 = stored_info_types1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists stored infoTypes. See https://cloud.google.com/dlp/docs/creating-stored-infotypes to learn more.");
            stored_info_types1 = stored_info_types1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the stored infoType by creating a new version. The existing version will continue to be used until the new version is ready. See https://cloud.google.com/dlp/docs/creating-stored-infotypes to learn more.");
            stored_info_types1 = stored_info_types1.subcommand(mcmd);
        }
        let mut content1 = SubCommand::with_name("content")
            .setting(AppSettings::ColoredHelp)
            .about("methods: deidentify, inspect and reidentify");
        {
            let mcmd = SubCommand::with_name("deidentify").about("De-identifies potentially sensitive info from a ContentItem. This method has limits on input size and output size. See https://cloud.google.com/dlp/docs/deidentify-sensitive-data to learn more. When no InfoTypes or CustomInfoTypes are specified in this request, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated.");
            content1 = content1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("inspect").about("Finds potentially sensitive info in content. This method has limits on input size, processing time, and output size. When no InfoTypes or CustomInfoTypes are specified in this request, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated. For how to guides, see https://cloud.google.com/dlp/docs/inspecting-images and https://cloud.google.com/dlp/docs/inspecting-text,");
            content1 = content1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reidentify").about("Re-identifies content that has been de-identified. See https://cloud.google.com/dlp/docs/pseudonymization#re-identification_in_free_text_code_example to learn more.");
            content1 = content1.subcommand(mcmd);
        }
        let mut deidentify_templates1 = SubCommand::with_name("deidentify_templates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a DeidentifyTemplate for reusing frequently used configuration for de-identifying content, images, and storage. See https://cloud.google.com/dlp/docs/creating-templates-deid to learn more.");
            deidentify_templates1 = deidentify_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a DeidentifyTemplate. See https://cloud.google.com/dlp/docs/creating-templates-deid to learn more.");
            deidentify_templates1 = deidentify_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a DeidentifyTemplate. See https://cloud.google.com/dlp/docs/creating-templates-deid to learn more.");
            deidentify_templates1 = deidentify_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists DeidentifyTemplates. See https://cloud.google.com/dlp/docs/creating-templates-deid to learn more.");
            deidentify_templates1 = deidentify_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the DeidentifyTemplate. See https://cloud.google.com/dlp/docs/creating-templates-deid to learn more.");
            deidentify_templates1 = deidentify_templates1.subcommand(mcmd);
        }
        let mut dlp_jobs1 = SubCommand::with_name("dlp_jobs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running DlpJob. The server makes a best effort to cancel the DlpJob, but success is not guaranteed. See https://cloud.google.com/dlp/docs/inspecting-storage and https://cloud.google.com/dlp/docs/compute-risk-analysis to learn more.");
            dlp_jobs1 = dlp_jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new job to inspect storage or calculate risk metrics. See https://cloud.google.com/dlp/docs/inspecting-storage and https://cloud.google.com/dlp/docs/compute-risk-analysis to learn more. When no InfoTypes or CustomInfoTypes are specified in inspect jobs, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated.");
            dlp_jobs1 = dlp_jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running DlpJob. This method indicates that the client is no longer interested in the DlpJob result. The job will be canceled if possible. See https://cloud.google.com/dlp/docs/inspecting-storage and https://cloud.google.com/dlp/docs/compute-risk-analysis to learn more.");
            dlp_jobs1 = dlp_jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running DlpJob. See https://cloud.google.com/dlp/docs/inspecting-storage and https://cloud.google.com/dlp/docs/compute-risk-analysis to learn more.");
            dlp_jobs1 = dlp_jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists DlpJobs that match the specified filter in the request. See https://cloud.google.com/dlp/docs/inspecting-storage and https://cloud.google.com/dlp/docs/compute-risk-analysis to learn more.");
            dlp_jobs1 = dlp_jobs1.subcommand(mcmd);
        }
        let mut image1 = SubCommand::with_name("image")
            .setting(AppSettings::ColoredHelp)
            .about("methods: redact");
        {
            let mcmd = SubCommand::with_name("redact").about("Redacts potentially sensitive info from an image. This method has limits on input size, processing time, and output size. See https://cloud.google.com/dlp/docs/redacting-sensitive-data-images to learn more. When no InfoTypes or CustomInfoTypes are specified in this request, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated.");
            image1 = image1.subcommand(mcmd);
        }
        let mut inspect_templates1 = SubCommand::with_name("inspect_templates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an InspectTemplate for reusing frequently used configuration for inspecting content, images, and storage. See https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates1 = inspect_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an InspectTemplate. See https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates1 = inspect_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an InspectTemplate. See https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates1 = inspect_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists InspectTemplates. See https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates1 = inspect_templates1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the InspectTemplate. See https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates1 = inspect_templates1.subcommand(mcmd);
        }
        let mut job_triggers1 = SubCommand::with_name("job_triggers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: activate, create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("activate").about("Activate a job trigger. Causes the immediate execute of a trigger instead of waiting on the trigger event to occur.");
            job_triggers1 = job_triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a job trigger to run DLP actions such as scanning storage for sensitive information on a set schedule. See https://cloud.google.com/dlp/docs/creating-job-triggers to learn more.");
            job_triggers1 = job_triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a job trigger. See https://cloud.google.com/dlp/docs/creating-job-triggers to learn more.");
            job_triggers1 = job_triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a job trigger. See https://cloud.google.com/dlp/docs/creating-job-triggers to learn more.");
            job_triggers1 = job_triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists job triggers. See https://cloud.google.com/dlp/docs/creating-job-triggers to learn more.");
            job_triggers1 = job_triggers1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a job trigger. See https://cloud.google.com/dlp/docs/creating-job-triggers to learn more.");
            job_triggers1 = job_triggers1.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
                        .setting(AppSettings::ColoredHelp)
                        .about("sub-resources: content, deidentify_templates, dlp_jobs, image, inspect_templates, job_triggers and stored_info_types");
        let mut stored_info_types1 = SubCommand::with_name("stored_info_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a pre-built stored infoType to be used for inspection. See https://cloud.google.com/dlp/docs/creating-stored-infotypes to learn more.");
            stored_info_types1 = stored_info_types1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a stored infoType. See https://cloud.google.com/dlp/docs/creating-stored-infotypes to learn more.");
            stored_info_types1 = stored_info_types1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a stored infoType. See https://cloud.google.com/dlp/docs/creating-stored-infotypes to learn more.");
            stored_info_types1 = stored_info_types1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists stored infoTypes. See https://cloud.google.com/dlp/docs/creating-stored-infotypes to learn more.");
            stored_info_types1 = stored_info_types1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the stored infoType by creating a new version. The existing version will continue to be used until the new version is ready. See https://cloud.google.com/dlp/docs/creating-stored-infotypes to learn more.");
            stored_info_types1 = stored_info_types1.subcommand(mcmd);
        }
        let mut deidentify_templates2 = SubCommand::with_name("deidentify_templates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a DeidentifyTemplate for reusing frequently used configuration for de-identifying content, images, and storage. See https://cloud.google.com/dlp/docs/creating-templates-deid to learn more.");
            deidentify_templates2 = deidentify_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a DeidentifyTemplate. See https://cloud.google.com/dlp/docs/creating-templates-deid to learn more.");
            deidentify_templates2 = deidentify_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a DeidentifyTemplate. See https://cloud.google.com/dlp/docs/creating-templates-deid to learn more.");
            deidentify_templates2 = deidentify_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists DeidentifyTemplates. See https://cloud.google.com/dlp/docs/creating-templates-deid to learn more.");
            deidentify_templates2 = deidentify_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the DeidentifyTemplate. See https://cloud.google.com/dlp/docs/creating-templates-deid to learn more.");
            deidentify_templates2 = deidentify_templates2.subcommand(mcmd);
        }
        let mut dlp_jobs2 = SubCommand::with_name("dlp_jobs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists DlpJobs that match the specified filter in the request. See https://cloud.google.com/dlp/docs/inspecting-storage and https://cloud.google.com/dlp/docs/compute-risk-analysis to learn more.");
            dlp_jobs2 = dlp_jobs2.subcommand(mcmd);
        }
        let mut inspect_templates2 = SubCommand::with_name("inspect_templates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an InspectTemplate for reusing frequently used configuration for inspecting content, images, and storage. See https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates2 = inspect_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an InspectTemplate. See https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates2 = inspect_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an InspectTemplate. See https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates2 = inspect_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists InspectTemplates. See https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates2 = inspect_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the InspectTemplate. See https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates2 = inspect_templates2.subcommand(mcmd);
        }
        let mut job_triggers2 = SubCommand::with_name("job_triggers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a job trigger to run DLP actions such as scanning storage for sensitive information on a set schedule. See https://cloud.google.com/dlp/docs/creating-job-triggers to learn more.");
            job_triggers2 = job_triggers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a job trigger. See https://cloud.google.com/dlp/docs/creating-job-triggers to learn more.");
            job_triggers2 = job_triggers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a job trigger. See https://cloud.google.com/dlp/docs/creating-job-triggers to learn more.");
            job_triggers2 = job_triggers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists job triggers. See https://cloud.google.com/dlp/docs/creating-job-triggers to learn more.");
            job_triggers2 = job_triggers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a job trigger. See https://cloud.google.com/dlp/docs/creating-job-triggers to learn more.");
            job_triggers2 = job_triggers2.subcommand(mcmd);
        }
        let mut stored_info_types2 = SubCommand::with_name("stored_info_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a pre-built stored infoType to be used for inspection. See https://cloud.google.com/dlp/docs/creating-stored-infotypes to learn more.");
            stored_info_types2 = stored_info_types2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a stored infoType. See https://cloud.google.com/dlp/docs/creating-stored-infotypes to learn more.");
            stored_info_types2 = stored_info_types2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a stored infoType. See https://cloud.google.com/dlp/docs/creating-stored-infotypes to learn more.");
            stored_info_types2 = stored_info_types2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists stored infoTypes. See https://cloud.google.com/dlp/docs/creating-stored-infotypes to learn more.");
            stored_info_types2 = stored_info_types2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the stored infoType by creating a new version. The existing version will continue to be used until the new version is ready. See https://cloud.google.com/dlp/docs/creating-stored-infotypes to learn more.");
            stored_info_types2 = stored_info_types2.subcommand(mcmd);
        }
        let mut content2 = SubCommand::with_name("content")
            .setting(AppSettings::ColoredHelp)
            .about("methods: deidentify, inspect and reidentify");
        {
            let mcmd = SubCommand::with_name("deidentify").about("De-identifies potentially sensitive info from a ContentItem. This method has limits on input size and output size. See https://cloud.google.com/dlp/docs/deidentify-sensitive-data to learn more. When no InfoTypes or CustomInfoTypes are specified in this request, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated.");
            content2 = content2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("inspect").about("Finds potentially sensitive info in content. This method has limits on input size, processing time, and output size. When no InfoTypes or CustomInfoTypes are specified in this request, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated. For how to guides, see https://cloud.google.com/dlp/docs/inspecting-images and https://cloud.google.com/dlp/docs/inspecting-text,");
            content2 = content2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reidentify").about("Re-identifies content that has been de-identified. See https://cloud.google.com/dlp/docs/pseudonymization#re-identification_in_free_text_code_example to learn more.");
            content2 = content2.subcommand(mcmd);
        }
        let mut deidentify_templates2 = SubCommand::with_name("deidentify_templates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a DeidentifyTemplate for reusing frequently used configuration for de-identifying content, images, and storage. See https://cloud.google.com/dlp/docs/creating-templates-deid to learn more.");
            deidentify_templates2 = deidentify_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a DeidentifyTemplate. See https://cloud.google.com/dlp/docs/creating-templates-deid to learn more.");
            deidentify_templates2 = deidentify_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a DeidentifyTemplate. See https://cloud.google.com/dlp/docs/creating-templates-deid to learn more.");
            deidentify_templates2 = deidentify_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists DeidentifyTemplates. See https://cloud.google.com/dlp/docs/creating-templates-deid to learn more.");
            deidentify_templates2 = deidentify_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the DeidentifyTemplate. See https://cloud.google.com/dlp/docs/creating-templates-deid to learn more.");
            deidentify_templates2 = deidentify_templates2.subcommand(mcmd);
        }
        let mut dlp_jobs2 = SubCommand::with_name("dlp_jobs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, create, delete, finish, get, hybrid_inspect and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running DlpJob. The server makes a best effort to cancel the DlpJob, but success is not guaranteed. See https://cloud.google.com/dlp/docs/inspecting-storage and https://cloud.google.com/dlp/docs/compute-risk-analysis to learn more.");
            dlp_jobs2 = dlp_jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new job to inspect storage or calculate risk metrics. See https://cloud.google.com/dlp/docs/inspecting-storage and https://cloud.google.com/dlp/docs/compute-risk-analysis to learn more. When no InfoTypes or CustomInfoTypes are specified in inspect jobs, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated.");
            dlp_jobs2 = dlp_jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running DlpJob. This method indicates that the client is no longer interested in the DlpJob result. The job will be canceled if possible. See https://cloud.google.com/dlp/docs/inspecting-storage and https://cloud.google.com/dlp/docs/compute-risk-analysis to learn more.");
            dlp_jobs2 = dlp_jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("finish").about("Finish a running hybrid DlpJob. Triggers the finalization steps and running of any enabled actions that have not yet run.");
            dlp_jobs2 = dlp_jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running DlpJob. See https://cloud.google.com/dlp/docs/inspecting-storage and https://cloud.google.com/dlp/docs/compute-risk-analysis to learn more.");
            dlp_jobs2 = dlp_jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("hybrid_inspect").about("Inspect hybrid content and store findings to a job. To review the findings, inspect the job. Inspection will occur asynchronously.");
            dlp_jobs2 = dlp_jobs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists DlpJobs that match the specified filter in the request. See https://cloud.google.com/dlp/docs/inspecting-storage and https://cloud.google.com/dlp/docs/compute-risk-analysis to learn more.");
            dlp_jobs2 = dlp_jobs2.subcommand(mcmd);
        }
        let mut image2 = SubCommand::with_name("image")
            .setting(AppSettings::ColoredHelp)
            .about("methods: redact");
        {
            let mcmd = SubCommand::with_name("redact").about("Redacts potentially sensitive info from an image. This method has limits on input size, processing time, and output size. See https://cloud.google.com/dlp/docs/redacting-sensitive-data-images to learn more. When no InfoTypes or CustomInfoTypes are specified in this request, the system will automatically choose what detectors to run. By default this may be all types, but may change over time as detectors are updated.");
            image2 = image2.subcommand(mcmd);
        }
        let mut inspect_templates2 = SubCommand::with_name("inspect_templates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an InspectTemplate for reusing frequently used configuration for inspecting content, images, and storage. See https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates2 = inspect_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an InspectTemplate. See https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates2 = inspect_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an InspectTemplate. See https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates2 = inspect_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists InspectTemplates. See https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates2 = inspect_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the InspectTemplate. See https://cloud.google.com/dlp/docs/creating-templates to learn more.");
            inspect_templates2 = inspect_templates2.subcommand(mcmd);
        }
        let mut job_triggers2 = SubCommand::with_name("job_triggers")
            .setting(AppSettings::ColoredHelp)
            .about("methods: activate, create, delete, get, hybrid_inspect, list and patch");
        {
            let mcmd = SubCommand::with_name("activate").about("Activate a job trigger. Causes the immediate execute of a trigger instead of waiting on the trigger event to occur.");
            job_triggers2 = job_triggers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a job trigger to run DLP actions such as scanning storage for sensitive information on a set schedule. See https://cloud.google.com/dlp/docs/creating-job-triggers to learn more.");
            job_triggers2 = job_triggers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a job trigger. See https://cloud.google.com/dlp/docs/creating-job-triggers to learn more.");
            job_triggers2 = job_triggers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a job trigger. See https://cloud.google.com/dlp/docs/creating-job-triggers to learn more.");
            job_triggers2 = job_triggers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("hybrid_inspect").about("Inspect hybrid content and store findings to a trigger. The inspection will be processed asynchronously. To review the findings monitor the jobs within the trigger.");
            job_triggers2 = job_triggers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists job triggers. See https://cloud.google.com/dlp/docs/creating-job-triggers to learn more.");
            job_triggers2 = job_triggers2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a job trigger. See https://cloud.google.com/dlp/docs/creating-job-triggers to learn more.");
            job_triggers2 = job_triggers2.subcommand(mcmd);
        }
        let mut stored_info_types2 = SubCommand::with_name("stored_info_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a pre-built stored infoType to be used for inspection. See https://cloud.google.com/dlp/docs/creating-stored-infotypes to learn more.");
            stored_info_types2 = stored_info_types2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a stored infoType. See https://cloud.google.com/dlp/docs/creating-stored-infotypes to learn more.");
            stored_info_types2 = stored_info_types2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a stored infoType. See https://cloud.google.com/dlp/docs/creating-stored-infotypes to learn more.");
            stored_info_types2 = stored_info_types2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists stored infoTypes. See https://cloud.google.com/dlp/docs/creating-stored-infotypes to learn more.");
            stored_info_types2 = stored_info_types2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the stored infoType by creating a new version. The existing version will continue to be used until the new version is ready. See https://cloud.google.com/dlp/docs/creating-stored-infotypes to learn more.");
            stored_info_types2 = stored_info_types2.subcommand(mcmd);
        }
        locations1 = locations1.subcommand(stored_info_types2);
        locations1 = locations1.subcommand(job_triggers2);
        locations1 = locations1.subcommand(inspect_templates2);
        locations1 = locations1.subcommand(image2);
        locations1 = locations1.subcommand(dlp_jobs2);
        locations1 = locations1.subcommand(deidentify_templates2);
        locations1 = locations1.subcommand(content2);
        locations1 = locations1.subcommand(stored_info_types2);
        locations1 = locations1.subcommand(job_triggers2);
        locations1 = locations1.subcommand(inspect_templates2);
        locations1 = locations1.subcommand(dlp_jobs2);
        locations1 = locations1.subcommand(deidentify_templates2);
        projects0 = projects0.subcommand(stored_info_types1);
        projects0 = projects0.subcommand(locations1);
        projects0 = projects0.subcommand(job_triggers1);
        projects0 = projects0.subcommand(inspect_templates1);
        projects0 = projects0.subcommand(image1);
        projects0 = projects0.subcommand(dlp_jobs1);
        projects0 = projects0.subcommand(deidentify_templates1);
        projects0 = projects0.subcommand(content1);
        organizations0 = organizations0.subcommand(stored_info_types1);
        organizations0 = organizations0.subcommand(locations1);
        organizations0 = organizations0.subcommand(inspect_templates1);
        organizations0 = organizations0.subcommand(deidentify_templates1);
        locations0 = locations0.subcommand(info_types1);
        app = app.subcommand(projects0);
        app = app.subcommand(organizations0);
        app = app.subcommand(locations0);
        app = app.subcommand(info_types0);

        Self { app }
    }
}
use google_dlp2 as api;

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
