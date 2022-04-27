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
        let mut app = App::new("logging2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20220421")
            .about("Writes log entries and manages your Cloud Logging configuration.")
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
        let mut billing_accounts0 = SubCommand::with_name("billing_accounts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_cmek_settings and get_settings");
        {
            let mcmd = SubCommand::with_name("get_cmek_settings").about("Gets the Logging CMEK settings for the given resource.Note: CMEK for the Log Router can be configured for Google Cloud projects, folders, organizations and billing accounts. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.");
            billing_accounts0 = billing_accounts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_settings").about("Gets the Log Router settings for the given resource.Note: Settings for the Log Router can be get for Google Cloud projects, folders, organizations and billing accounts. Currently it can only be configured for organizations. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.");
            billing_accounts0 = billing_accounts0.subcommand(mcmd);
        }
        let mut entries0 = SubCommand::with_name("entries")
            .setting(AppSettings::ColoredHelp)
            .about("methods: copy, list, tail and write");
        {
            let mcmd = SubCommand::with_name("copy")
                .about("Copies a set of log entries from a log bucket to a Cloud Storage bucket.");
            entries0 = entries0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists log entries. Use this method to retrieve log entries that originated from a project/folder/organization/billing account. For ways to export log entries, see Exporting Logs (https://cloud.google.com/logging/docs/export).");
            entries0 = entries0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("tail").about("Streaming read of log entries as they are ingested. Until the stream is terminated, it will continue reading logs.");
            entries0 = entries0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("write").about("Writes log entries to Logging. This API method is the only way to send log entries to Logging. This method is used, directly or indirectly, by the Logging agent (fluentd) and all logging libraries configured to use Logging. A single request may contain log entries for a maximum of 1000 different resources (projects, organizations, billing accounts or folders)");
            entries0 = entries0.subcommand(mcmd);
        }
        let mut exclusions0 = SubCommand::with_name("exclusions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new exclusion in the _Default sink in a specified parent resource. Only log entries belonging to that resource can be excluded. You can have up to 10 exclusions in a resource.");
            exclusions0 = exclusions0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes an exclusion in the _Default sink.");
            exclusions0 = exclusions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the description of an exclusion in the _Default sink.");
            exclusions0 = exclusions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the exclusions on the _Default sink in a parent resource.");
            exclusions0 = exclusions0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Changes one or more properties of an existing exclusion in the _Default sink.",
            );
            exclusions0 = exclusions0.subcommand(mcmd);
        }
        let mut folders0 = SubCommand::with_name("folders")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_cmek_settings, get_settings and update_settings");
        {
            let mcmd = SubCommand::with_name("get_cmek_settings").about("Gets the Logging CMEK settings for the given resource.Note: CMEK for the Log Router can be configured for Google Cloud projects, folders, organizations and billing accounts. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_settings").about("Gets the Log Router settings for the given resource.Note: Settings for the Log Router can be get for Google Cloud projects, folders, organizations and billing accounts. Currently it can only be configured for organizations. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.");
            folders0 = folders0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_settings").about("Updates the Log Router settings for the given resource.Note: Settings for the Log Router can currently only be configured for Google Cloud organizations. Once configured, it applies to all projects and folders in the Google Cloud organization.UpdateSettings will fail if 1) kms_key_name is invalid, or 2) the associated service account does not have the required roles/cloudkms.cryptoKeyEncrypterDecrypter role assigned for the key, or 3) access to the key is disabled. 4) location_id is not supported by Logging. 5) location_id violate OrgPolicy.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.");
            folders0 = folders0.subcommand(mcmd);
        }
        let mut locations0 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a location.");
            locations0 = locations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists information about the supported locations for this service.");
            locations0 = locations0.subcommand(mcmd);
        }
        let mut logs0 = SubCommand::with_name("logs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes all the log entries in a log for the _Default Log Bucket. The log reappears if it receives new entries. Log entries written shortly before the delete operation might not be deleted. Entries received after the delete operation with a timestamp before the operation will be deleted.");
            logs0 = logs0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the logs in projects, organizations, folders, or billing accounts. Only logs that have entries are listed.");
            logs0 = logs0.subcommand(mcmd);
        }
        let mut monitored_resource_descriptors0 =
            SubCommand::with_name("monitored_resource_descriptors")
                .setting(AppSettings::ColoredHelp)
                .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the descriptors for monitored resource types used by Logging.");
            monitored_resource_descriptors0 = monitored_resource_descriptors0.subcommand(mcmd);
        }
        let mut organizations0 = SubCommand::with_name("organizations")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: get_cmek_settings, get_settings, update_cmek_settings and update_settings");
        {
            let mcmd = SubCommand::with_name("get_cmek_settings").about("Gets the Logging CMEK settings for the given resource.Note: CMEK for the Log Router can be configured for Google Cloud projects, folders, organizations and billing accounts. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_settings").about("Gets the Log Router settings for the given resource.Note: Settings for the Log Router can be get for Google Cloud projects, folders, organizations and billing accounts. Currently it can only be configured for organizations. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_cmek_settings").about("Updates the Log Router CMEK settings for the given resource.Note: CMEK for the Log Router can currently only be configured for Google Cloud organizations. Once configured, it applies to all projects and folders in the Google Cloud organization.UpdateCmekSettings will fail if 1) kms_key_name is invalid, or 2) the associated service account does not have the required roles/cloudkms.cryptoKeyEncrypterDecrypter role assigned for the key, or 3) access to the key is disabled.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_settings").about("Updates the Log Router settings for the given resource.Note: Settings for the Log Router can currently only be configured for Google Cloud organizations. Once configured, it applies to all projects and folders in the Google Cloud organization.UpdateSettings will fail if 1) kms_key_name is invalid, or 2) the associated service account does not have the required roles/cloudkms.cryptoKeyEncrypterDecrypter role assigned for the key, or 3) access to the key is disabled. 4) location_id is not supported by Logging. 5) location_id violate OrgPolicy.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.");
            organizations0 = organizations0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_cmek_settings and get_settings");
        {
            let mcmd = SubCommand::with_name("get_cmek_settings").about("Gets the Logging CMEK settings for the given resource.Note: CMEK for the Log Router can be configured for Google Cloud projects, folders, organizations and billing accounts. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_settings").about("Gets the Log Router settings for the given resource.Note: Settings for the Log Router can be get for Google Cloud projects, folders, organizations and billing accounts. Currently it can only be configured for organizations. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut sinks0 = SubCommand::with_name("sinks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a sink that exports specified log entries to a destination. The export of newly-ingested log entries begins immediately, unless the sink's writer_identity is not permitted to write to the destination. A sink can export log entries only from the resource owning the sink.");
            sinks0 = sinks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a sink. If the sink has a unique writer_identity, then that service account is also deleted.");
            sinks0 = sinks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a sink.");
            sinks0 = sinks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists sinks.");
            sinks0 = sinks0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a sink. This method replaces the following fields in the existing sink with values from the new sink: destination, and filter.The updated sink might also have a new writer_identity; see the unique_writer_identity field.");
            sinks0 = sinks0.subcommand(mcmd);
        }
        let mut v_20 = SubCommand::with_name("v_2")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: get_cmek_settings, get_settings, update_cmek_settings and update_settings");
        {
            let mcmd = SubCommand::with_name("get_cmek_settings").about("Gets the Logging CMEK settings for the given resource.Note: CMEK for the Log Router can be configured for Google Cloud projects, folders, organizations and billing accounts. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.");
            v_20 = v_20.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_settings").about("Gets the Log Router settings for the given resource.Note: Settings for the Log Router can be get for Google Cloud projects, folders, organizations and billing accounts. Currently it can only be configured for organizations. Once configured for an organization, it applies to all projects and folders in the Google Cloud organization.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.");
            v_20 = v_20.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_cmek_settings").about("Updates the Log Router CMEK settings for the given resource.Note: CMEK for the Log Router can currently only be configured for Google Cloud organizations. Once configured, it applies to all projects and folders in the Google Cloud organization.UpdateCmekSettings will fail if 1) kms_key_name is invalid, or 2) the associated service account does not have the required roles/cloudkms.cryptoKeyEncrypterDecrypter role assigned for the key, or 3) access to the key is disabled.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.");
            v_20 = v_20.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_settings").about("Updates the Log Router settings for the given resource.Note: Settings for the Log Router can currently only be configured for Google Cloud organizations. Once configured, it applies to all projects and folders in the Google Cloud organization.UpdateSettings will fail if 1) kms_key_name is invalid, or 2) the associated service account does not have the required roles/cloudkms.cryptoKeyEncrypterDecrypter role assigned for the key, or 3) access to the key is disabled. 4) location_id is not supported by Logging. 5) location_id violate OrgPolicy.See Enabling CMEK for Log Router (https://cloud.google.com/logging/docs/routing/managed-encryption) for more information.");
            v_20 = v_20.subcommand(mcmd);
        }
        let mut buckets1 = SubCommand::with_name("buckets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a log bucket.");
            buckets1 = buckets1.subcommand(mcmd);
        }
        let mut exclusions1 = SubCommand::with_name("exclusions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new exclusion in the _Default sink in a specified parent resource. Only log entries belonging to that resource can be excluded. You can have up to 10 exclusions in a resource.");
            exclusions1 = exclusions1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes an exclusion in the _Default sink.");
            exclusions1 = exclusions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the description of an exclusion in the _Default sink.");
            exclusions1 = exclusions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the exclusions on the _Default sink in a parent resource.");
            exclusions1 = exclusions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Changes one or more properties of an existing exclusion in the _Default sink.",
            );
            exclusions1 = exclusions1.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a location.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists information about the supported locations for this service.");
            locations1 = locations1.subcommand(mcmd);
        }
        let mut logs1 = SubCommand::with_name("logs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes all the log entries in a log for the _Default Log Bucket. The log reappears if it receives new entries. Log entries written shortly before the delete operation might not be deleted. Entries received after the delete operation with a timestamp before the operation will be deleted.");
            logs1 = logs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the logs in projects, organizations, folders, or billing accounts. Only logs that have entries are listed.");
            logs1 = logs1.subcommand(mcmd);
        }
        let mut operations1 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations1 = operations1.subcommand(mcmd);
        }
        let mut sinks1 = SubCommand::with_name("sinks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a sink that exports specified log entries to a destination. The export of newly-ingested log entries begins immediately, unless the sink's writer_identity is not permitted to write to the destination. A sink can export log entries only from the resource owning the sink.");
            sinks1 = sinks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a sink. If the sink has a unique writer_identity, then that service account is also deleted.");
            sinks1 = sinks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a sink.");
            sinks1 = sinks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists sinks.");
            sinks1 = sinks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a sink. This method replaces the following fields in the existing sink with values from the new sink: destination, and filter.The updated sink might also have a new writer_identity; see the unique_writer_identity field.");
            sinks1 = sinks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a sink. This method replaces the following fields in the existing sink with values from the new sink: destination, and filter.The updated sink might also have a new writer_identity; see the unique_writer_identity field.");
            sinks1 = sinks1.subcommand(mcmd);
        }
        let mut exclusions1 = SubCommand::with_name("exclusions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new exclusion in the _Default sink in a specified parent resource. Only log entries belonging to that resource can be excluded. You can have up to 10 exclusions in a resource.");
            exclusions1 = exclusions1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes an exclusion in the _Default sink.");
            exclusions1 = exclusions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the description of an exclusion in the _Default sink.");
            exclusions1 = exclusions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the exclusions on the _Default sink in a parent resource.");
            exclusions1 = exclusions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Changes one or more properties of an existing exclusion in the _Default sink.",
            );
            exclusions1 = exclusions1.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a location.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists information about the supported locations for this service.");
            locations1 = locations1.subcommand(mcmd);
        }
        let mut logs1 = SubCommand::with_name("logs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes all the log entries in a log for the _Default Log Bucket. The log reappears if it receives new entries. Log entries written shortly before the delete operation might not be deleted. Entries received after the delete operation with a timestamp before the operation will be deleted.");
            logs1 = logs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the logs in projects, organizations, folders, or billing accounts. Only logs that have entries are listed.");
            logs1 = logs1.subcommand(mcmd);
        }
        let mut sinks1 = SubCommand::with_name("sinks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a sink that exports specified log entries to a destination. The export of newly-ingested log entries begins immediately, unless the sink's writer_identity is not permitted to write to the destination. A sink can export log entries only from the resource owning the sink.");
            sinks1 = sinks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a sink. If the sink has a unique writer_identity, then that service account is also deleted.");
            sinks1 = sinks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a sink.");
            sinks1 = sinks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists sinks.");
            sinks1 = sinks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a sink. This method replaces the following fields in the existing sink with values from the new sink: destination, and filter.The updated sink might also have a new writer_identity; see the unique_writer_identity field.");
            sinks1 = sinks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a sink. This method replaces the following fields in the existing sink with values from the new sink: destination, and filter.The updated sink might also have a new writer_identity; see the unique_writer_identity field.");
            sinks1 = sinks1.subcommand(mcmd);
        }
        let mut buckets1 = SubCommand::with_name("buckets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and undelete");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a log bucket that can be used to store log entries. After a bucket has been created, the bucket's location cannot be changed.");
            buckets1 = buckets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a log bucket.Changes the bucket's lifecycle_state to the DELETE_REQUESTED state. After 7 days, the bucket will be purged and all log entries in the bucket will be permanently deleted.");
            buckets1 = buckets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a log bucket.");
            buckets1 = buckets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists log buckets.");
            buckets1 = buckets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a log bucket. This method replaces the following fields in the existing bucket with values from the new bucket: retention_periodIf the retention period is decreased and the bucket is locked, FAILED_PRECONDITION will be returned.If the bucket has a lifecycle_state of DELETE_REQUESTED, then FAILED_PRECONDITION will be returned.After a bucket has been created, the bucket's location cannot be changed.");
            buckets1 = buckets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undelete").about("Undeletes a log bucket. A bucket that has been deleted can be undeleted within the grace period of 7 days.");
            buckets1 = buckets1.subcommand(mcmd);
        }
        let mut operations1 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED.");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns UNIMPLEMENTED.NOTE: the name binding allows API services to override the binding to use different resource name schemes, such as users/*/operations. To override the binding, API services can add a binding such as \"/v1/{name=users/*}/operations\" to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations1 = operations1.subcommand(mcmd);
        }
        let mut exclusions1 = SubCommand::with_name("exclusions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new exclusion in the _Default sink in a specified parent resource. Only log entries belonging to that resource can be excluded. You can have up to 10 exclusions in a resource.");
            exclusions1 = exclusions1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes an exclusion in the _Default sink.");
            exclusions1 = exclusions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the description of an exclusion in the _Default sink.");
            exclusions1 = exclusions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the exclusions on the _Default sink in a parent resource.");
            exclusions1 = exclusions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Changes one or more properties of an existing exclusion in the _Default sink.",
            );
            exclusions1 = exclusions1.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a location.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists information about the supported locations for this service.");
            locations1 = locations1.subcommand(mcmd);
        }
        let mut logs1 = SubCommand::with_name("logs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes all the log entries in a log for the _Default Log Bucket. The log reappears if it receives new entries. Log entries written shortly before the delete operation might not be deleted. Entries received after the delete operation with a timestamp before the operation will be deleted.");
            logs1 = logs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the logs in projects, organizations, folders, or billing accounts. Only logs that have entries are listed.");
            logs1 = logs1.subcommand(mcmd);
        }
        let mut sinks1 = SubCommand::with_name("sinks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a sink that exports specified log entries to a destination. The export of newly-ingested log entries begins immediately, unless the sink's writer_identity is not permitted to write to the destination. A sink can export log entries only from the resource owning the sink.");
            sinks1 = sinks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a sink. If the sink has a unique writer_identity, then that service account is also deleted.");
            sinks1 = sinks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a sink.");
            sinks1 = sinks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists sinks.");
            sinks1 = sinks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a sink. This method replaces the following fields in the existing sink with values from the new sink: destination, and filter.The updated sink might also have a new writer_identity; see the unique_writer_identity field.");
            sinks1 = sinks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a sink. This method replaces the following fields in the existing sink with values from the new sink: destination, and filter.The updated sink might also have a new writer_identity; see the unique_writer_identity field.");
            sinks1 = sinks1.subcommand(mcmd);
        }
        let mut exclusions1 = SubCommand::with_name("exclusions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new exclusion in the _Default sink in a specified parent resource. Only log entries belonging to that resource can be excluded. You can have up to 10 exclusions in a resource.");
            exclusions1 = exclusions1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes an exclusion in the _Default sink.");
            exclusions1 = exclusions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets the description of an exclusion in the _Default sink.");
            exclusions1 = exclusions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the exclusions on the _Default sink in a parent resource.");
            exclusions1 = exclusions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about(
                "Changes one or more properties of an existing exclusion in the _Default sink.",
            );
            exclusions1 = exclusions1.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a location.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists information about the supported locations for this service.");
            locations1 = locations1.subcommand(mcmd);
        }
        let mut logs1 = SubCommand::with_name("logs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete and list");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes all the log entries in a log for the _Default Log Bucket. The log reappears if it receives new entries. Log entries written shortly before the delete operation might not be deleted. Entries received after the delete operation with a timestamp before the operation will be deleted.");
            logs1 = logs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the logs in projects, organizations, folders, or billing accounts. Only logs that have entries are listed.");
            logs1 = logs1.subcommand(mcmd);
        }
        let mut metrics1 = SubCommand::with_name("metrics")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a logs-based metric.");
            metrics1 = metrics1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a logs-based metric.");
            metrics1 = metrics1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a logs-based metric.");
            metrics1 = metrics1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists logs-based metrics.");
            metrics1 = metrics1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update").about("Creates or updates a logs-based metric.");
            metrics1 = metrics1.subcommand(mcmd);
        }
        let mut sinks1 = SubCommand::with_name("sinks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a sink that exports specified log entries to a destination. The export of newly-ingested log entries begins immediately, unless the sink's writer_identity is not permitted to write to the destination. A sink can export log entries only from the resource owning the sink.");
            sinks1 = sinks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a sink. If the sink has a unique writer_identity, then that service account is also deleted.");
            sinks1 = sinks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a sink.");
            sinks1 = sinks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists sinks.");
            sinks1 = sinks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a sink. This method replaces the following fields in the existing sink with values from the new sink: destination, and filter.The updated sink might also have a new writer_identity; see the unique_writer_identity field.");
            sinks1 = sinks1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a sink. This method replaces the following fields in the existing sink with values from the new sink: destination, and filter.The updated sink might also have a new writer_identity; see the unique_writer_identity field.");
            sinks1 = sinks1.subcommand(mcmd);
        }
        let mut views2 = SubCommand::with_name("views")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a view on a log bucket..");
            views2 = views2.subcommand(mcmd);
        }
        let mut buckets2 = SubCommand::with_name("buckets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, list, patch and undelete");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a log bucket that can be used to store log entries. After a bucket has been created, the bucket's location cannot be changed.");
            buckets2 = buckets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a log bucket.Changes the bucket's lifecycle_state to the DELETE_REQUESTED state. After 7 days, the bucket will be purged and all log entries in the bucket will be permanently deleted.");
            buckets2 = buckets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists log buckets.");
            buckets2 = buckets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a log bucket. This method replaces the following fields in the existing bucket with values from the new bucket: retention_periodIf the retention period is decreased and the bucket is locked, FAILED_PRECONDITION will be returned.If the bucket has a lifecycle_state of DELETE_REQUESTED, then FAILED_PRECONDITION will be returned.After a bucket has been created, the bucket's location cannot be changed.");
            buckets2 = buckets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undelete").about("Undeletes a log bucket. A bucket that has been deleted can be undeleted within the grace period of 7 days.");
            buckets2 = buckets2.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns UNIMPLEMENTED.NOTE: the name binding allows API services to override the binding to use different resource name schemes, such as users/*/operations. To override the binding, API services can add a binding such as \"/v1/{name=users/*}/operations\" to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut buckets2 = SubCommand::with_name("buckets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and undelete");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a log bucket that can be used to store log entries. After a bucket has been created, the bucket's location cannot be changed.");
            buckets2 = buckets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a log bucket.Changes the bucket's lifecycle_state to the DELETE_REQUESTED state. After 7 days, the bucket will be purged and all log entries in the bucket will be permanently deleted.");
            buckets2 = buckets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a log bucket.");
            buckets2 = buckets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists log buckets.");
            buckets2 = buckets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a log bucket. This method replaces the following fields in the existing bucket with values from the new bucket: retention_periodIf the retention period is decreased and the bucket is locked, FAILED_PRECONDITION will be returned.If the bucket has a lifecycle_state of DELETE_REQUESTED, then FAILED_PRECONDITION will be returned.After a bucket has been created, the bucket's location cannot be changed.");
            buckets2 = buckets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undelete").about("Undeletes a log bucket. A bucket that has been deleted can be undeleted within the grace period of 7 days.");
            buckets2 = buckets2.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns UNIMPLEMENTED.NOTE: the name binding allows API services to override the binding to use different resource name schemes, such as users/*/operations. To override the binding, API services can add a binding such as \"/v1/{name=users/*}/operations\" to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut views2 = SubCommand::with_name("views")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a view over log entries in a log bucket. A bucket may contain a maximum of 30 views.");
            views2 = views2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a view on a log bucket. If an UNAVAILABLE error is returned, this indicates that system is not in a state where it can delete the view. If this occurs, please try again in a few minutes.");
            views2 = views2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a view on a log bucket..");
            views2 = views2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists views on a log bucket.");
            views2 = views2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a view on a log bucket. This method replaces the following fields in the existing view with values from the new view: filter. If an UNAVAILABLE error is returned, this indicates that system is not in a state where it can update the view. If this occurs, please try again in a few minutes.");
            views2 = views2.subcommand(mcmd);
        }
        let mut buckets2 = SubCommand::with_name("buckets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and undelete");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a log bucket that can be used to store log entries. After a bucket has been created, the bucket's location cannot be changed.");
            buckets2 = buckets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a log bucket.Changes the bucket's lifecycle_state to the DELETE_REQUESTED state. After 7 days, the bucket will be purged and all log entries in the bucket will be permanently deleted.");
            buckets2 = buckets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a log bucket.");
            buckets2 = buckets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists log buckets.");
            buckets2 = buckets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a log bucket. This method replaces the following fields in the existing bucket with values from the new bucket: retention_periodIf the retention period is decreased and the bucket is locked, FAILED_PRECONDITION will be returned.If the bucket has a lifecycle_state of DELETE_REQUESTED, then FAILED_PRECONDITION will be returned.After a bucket has been created, the bucket's location cannot be changed.");
            buckets2 = buckets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undelete").about("Undeletes a log bucket. A bucket that has been deleted can be undeleted within the grace period of 7 days.");
            buckets2 = buckets2.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns UNIMPLEMENTED.NOTE: the name binding allows API services to override the binding to use different resource name schemes, such as users/*/operations. To override the binding, API services can add a binding such as \"/v1/{name=users/*}/operations\" to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut buckets2 = SubCommand::with_name("buckets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and undelete");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a log bucket that can be used to store log entries. After a bucket has been created, the bucket's location cannot be changed.");
            buckets2 = buckets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a log bucket.Changes the bucket's lifecycle_state to the DELETE_REQUESTED state. After 7 days, the bucket will be purged and all log entries in the bucket will be permanently deleted.");
            buckets2 = buckets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a log bucket.");
            buckets2 = buckets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists log buckets.");
            buckets2 = buckets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a log bucket. This method replaces the following fields in the existing bucket with values from the new bucket: retention_periodIf the retention period is decreased and the bucket is locked, FAILED_PRECONDITION will be returned.If the bucket has a lifecycle_state of DELETE_REQUESTED, then FAILED_PRECONDITION will be returned.After a bucket has been created, the bucket's location cannot be changed.");
            buckets2 = buckets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undelete").about("Undeletes a log bucket. A bucket that has been deleted can be undeleted within the grace period of 7 days.");
            buckets2 = buckets2.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns UNIMPLEMENTED.NOTE: the name binding allows API services to override the binding to use different resource name schemes, such as users/*/operations. To override the binding, API services can add a binding such as \"/v1/{name=users/*}/operations\" to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut views3 = SubCommand::with_name("views")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a view over log entries in a log bucket. A bucket may contain a maximum of 30 views.");
            views3 = views3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a view on a log bucket. If an UNAVAILABLE error is returned, this indicates that system is not in a state where it can delete the view. If this occurs, please try again in a few minutes.");
            views3 = views3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists views on a log bucket.");
            views3 = views3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a view on a log bucket. This method replaces the following fields in the existing view with values from the new view: filter. If an UNAVAILABLE error is returned, this indicates that system is not in a state where it can update the view. If this occurs, please try again in a few minutes.");
            views3 = views3.subcommand(mcmd);
        }
        let mut views3 = SubCommand::with_name("views")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a view over log entries in a log bucket. A bucket may contain a maximum of 30 views.");
            views3 = views3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a view on a log bucket. If an UNAVAILABLE error is returned, this indicates that system is not in a state where it can delete the view. If this occurs, please try again in a few minutes.");
            views3 = views3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a view on a log bucket..");
            views3 = views3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists views on a log bucket.");
            views3 = views3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a view on a log bucket. This method replaces the following fields in the existing view with values from the new view: filter. If an UNAVAILABLE error is returned, this indicates that system is not in a state where it can update the view. If this occurs, please try again in a few minutes.");
            views3 = views3.subcommand(mcmd);
        }
        let mut views3 = SubCommand::with_name("views")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a view over log entries in a log bucket. A bucket may contain a maximum of 30 views.");
            views3 = views3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a view on a log bucket. If an UNAVAILABLE error is returned, this indicates that system is not in a state where it can delete the view. If this occurs, please try again in a few minutes.");
            views3 = views3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a view on a log bucket..");
            views3 = views3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists views on a log bucket.");
            views3 = views3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a view on a log bucket. This method replaces the following fields in the existing view with values from the new view: filter. If an UNAVAILABLE error is returned, this indicates that system is not in a state where it can update the view. If this occurs, please try again in a few minutes.");
            views3 = views3.subcommand(mcmd);
        }
        let mut views3 = SubCommand::with_name("views")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a view over log entries in a log bucket. A bucket may contain a maximum of 30 views.");
            views3 = views3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a view on a log bucket. If an UNAVAILABLE error is returned, this indicates that system is not in a state where it can delete the view. If this occurs, please try again in a few minutes.");
            views3 = views3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a view on a log bucket..");
            views3 = views3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists views on a log bucket.");
            views3 = views3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a view on a log bucket. This method replaces the following fields in the existing view with values from the new view: filter. If an UNAVAILABLE error is returned, this indicates that system is not in a state where it can update the view. If this occurs, please try again in a few minutes.");
            views3 = views3.subcommand(mcmd);
        }
        let mut logs4 = SubCommand::with_name("logs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists the logs in projects, organizations, folders, or billing accounts. Only logs that have entries are listed.");
            logs4 = logs4.subcommand(mcmd);
        }
        let mut logs4 = SubCommand::with_name("logs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists the logs in projects, organizations, folders, or billing accounts. Only logs that have entries are listed.");
            logs4 = logs4.subcommand(mcmd);
        }
        let mut logs4 = SubCommand::with_name("logs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists the logs in projects, organizations, folders, or billing accounts. Only logs that have entries are listed.");
            logs4 = logs4.subcommand(mcmd);
        }
        let mut logs4 = SubCommand::with_name("logs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists the logs in projects, organizations, folders, or billing accounts. Only logs that have entries are listed.");
            logs4 = logs4.subcommand(mcmd);
        }
        views3 = views3.subcommand(logs4);
        views3 = views3.subcommand(logs4);
        views3 = views3.subcommand(logs4);
        views3 = views3.subcommand(logs4);
        buckets2 = buckets2.subcommand(views3);
        buckets2 = buckets2.subcommand(views3);
        buckets2 = buckets2.subcommand(views3);
        buckets2 = buckets2.subcommand(views3);
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(buckets2);
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(buckets2);
        buckets1 = buckets1.subcommand(views2);
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(buckets2);
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(buckets2);
        buckets1 = buckets1.subcommand(views2);
        projects0 = projects0.subcommand(sinks1);
        projects0 = projects0.subcommand(metrics1);
        projects0 = projects0.subcommand(logs1);
        projects0 = projects0.subcommand(locations1);
        projects0 = projects0.subcommand(exclusions1);
        organizations0 = organizations0.subcommand(sinks1);
        organizations0 = organizations0.subcommand(logs1);
        organizations0 = organizations0.subcommand(locations1);
        organizations0 = organizations0.subcommand(exclusions1);
        locations0 = locations0.subcommand(operations1);
        locations0 = locations0.subcommand(buckets1);
        folders0 = folders0.subcommand(sinks1);
        folders0 = folders0.subcommand(logs1);
        folders0 = folders0.subcommand(locations1);
        folders0 = folders0.subcommand(exclusions1);
        billing_accounts0 = billing_accounts0.subcommand(sinks1);
        billing_accounts0 = billing_accounts0.subcommand(operations1);
        billing_accounts0 = billing_accounts0.subcommand(logs1);
        billing_accounts0 = billing_accounts0.subcommand(locations1);
        billing_accounts0 = billing_accounts0.subcommand(exclusions1);
        billing_accounts0 = billing_accounts0.subcommand(buckets1);
        app = app.subcommand(v_20);
        app = app.subcommand(sinks0);
        app = app.subcommand(projects0);
        app = app.subcommand(organizations0);
        app = app.subcommand(monitored_resource_descriptors0);
        app = app.subcommand(logs0);
        app = app.subcommand(locations0);
        app = app.subcommand(folders0);
        app = app.subcommand(exclusions0);
        app = app.subcommand(entries0);
        app = app.subcommand(billing_accounts0);

        Self { app }
    }
}
use google_logging2 as api;

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
