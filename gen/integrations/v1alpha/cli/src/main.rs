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
        let mut app = App::new("integrations1_alpha")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230124")
            .about("")
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
        let mut callback0 = SubCommand::with_name("callback")
            .setting(AppSettings::ColoredHelp)
            .about("methods: generate_token");
        {
            let mcmd = SubCommand::with_name("generate_token").about("Receives the auth code and auth config id to combine that with the client id and secret to retrieve access tokens from the token endpoint. Returns either a success or error message when it's done.");
            callback0 = callback0.subcommand(mcmd);
        }
        let mut connector_platform_regions0 = SubCommand::with_name("connector_platform_regions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: enumerate");
        {
            let mcmd = SubCommand::with_name("enumerate")
                .about("Enumerates the regions for which Connector Platform is provisioned.");
            connector_platform_regions0 = connector_platform_regions0.subcommand(mcmd);
        }
        let mut projects0 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut locations1 = SubCommand::with_name("locations")
                        .setting(AppSettings::ColoredHelp)
                        .about("sub-resources: apps_script_projects, auth_configs, certificates, connections, integrations, products and sfdc_instances");
        let mut apps_script_projects2 = SubCommand::with_name("apps_script_projects")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create and link");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an Apps Script project.");
            apps_script_projects2 = apps_script_projects2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("link").about("Links a existing Apps Script project.");
            apps_script_projects2 = apps_script_projects2.subcommand(mcmd);
        }
        let mut auth_configs2 = SubCommand::with_name("auth_configs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an auth config record. Fetch corresponding credentials for specific auth types, e.g. access token for OAuth 2.0, JWT token for JWT. Encrypt the auth config with Cloud KMS and store the encrypted credentials in Spanner. Returns the encrypted auth config.");
            auth_configs2 = auth_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an auth config.");
            auth_configs2 = auth_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a complete auth config. If the auth config doesn't exist, Code.NOT_FOUND exception will be thrown. Returns the decrypted auth config.");
            auth_configs2 = auth_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all auth configs that match the filter. Restrict to auth configs belong to the current client only.");
            auth_configs2 = auth_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an auth config. If credential is updated, fetch the encrypted auth config from Spanner, decrypt with Cloud KMS key, update the credential fields, re-encrypt with Cloud KMS key and update the Spanner record. For other fields, directly update the Spanner record. Returns the encrypted auth config.");
            auth_configs2 = auth_configs2.subcommand(mcmd);
        }
        let mut certificates2 = SubCommand::with_name("certificates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd =
                SubCommand::with_name("get").about("Get a certificates in the specified project.");
            certificates2 = certificates2.subcommand(mcmd);
        }
        let mut connections2 = SubCommand::with_name("connections")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_connection_schema_metadata and list");
        {
            let mcmd = SubCommand::with_name("get_connection_schema_metadata")
                .about("Lists the available entities and actions associated with a Connection.");
            connections2 = connections2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists Connections in a given project and location.");
            connections2 = connections2.subcommand(mcmd);
        }
        let mut integrations2 = SubCommand::with_name("integrations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, execute, list and schedule");
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Delete the selected integration and all versions inside");
            integrations2 = integrations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("execute").about("Executes integrations synchronously by passing the trigger id in the request body. The request is not returned until the requested executions are either fulfilled or experienced an error. If the integration name is not specified (passing `-`), all of the associated integration under the given trigger_id will be executed. Otherwise only the specified integration for the given `trigger_id` is executed. This is helpful for execution the integration from UI.");
            integrations2 = integrations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all integrations in the specified project.");
            integrations2 = integrations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("schedule").about("Schedules an integration for execution by passing the trigger id and the scheduled time in the request body.");
            integrations2 = integrations2.subcommand(mcmd);
        }
        let mut products2 = SubCommand::with_name("products")
                        .setting(AppSettings::ColoredHelp)
                        .about("sub-resources: auth_configs, certificates, integrations, integrationtemplates and sfdc_instances");
        let mut sfdc_instances2 = SubCommand::with_name("sfdc_instances")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an sfdc instance record. Store the sfdc instance in Spanner. Returns the sfdc instance.");
            sfdc_instances2 = sfdc_instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an sfdc instance.");
            sfdc_instances2 = sfdc_instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an sfdc instance. If the instance doesn't exist, Code.NOT_FOUND exception will be thrown.");
            sfdc_instances2 = sfdc_instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all sfdc instances that match the filter. Restrict to sfdc instances belonging to the current client only.");
            sfdc_instances2 = sfdc_instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an sfdc instance. Updates the sfdc instance in spanner. Returns the sfdc instance.");
            sfdc_instances2 = sfdc_instances2.subcommand(mcmd);
        }
        let mut runtime_action_schemas3 = SubCommand::with_name("runtime_action_schemas")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists the JSON schemas for the inputs and outputs of actions, filtered by action name.");
            runtime_action_schemas3 = runtime_action_schemas3.subcommand(mcmd);
        }
        let mut runtime_entity_schemas3 = SubCommand::with_name("runtime_entity_schemas")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists the JSON schemas for the properties of runtime entities, filtered by entity name.");
            runtime_entity_schemas3 = runtime_entity_schemas3.subcommand(mcmd);
        }
        let mut executions3 = SubCommand::with_name("executions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists the results of all the integration executions. The response includes the same information as the [execution log](https://cloud.google.com/application-integration/docs/viewing-logs) in the Integration UI.");
            executions3 = executions3.subcommand(mcmd);
        }
        let mut versions3 = SubCommand::with_name("versions")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, download, get, list, patch, publish, takeover_edit_lock, unpublish and upload");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Create a integration with a draft version in the specified project.");
            versions3 = versions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Soft-deletes the integration. Changes the status of the integration to ARCHIVED. If the integration being ARCHIVED is tagged as \"HEAD\", the tag is removed from this snapshot and set to the previous non-ARCHIVED snapshot. The PUBLISH_REQUESTED, DUE_FOR_DELETION tags are removed too. This RPC throws an exception if the version being deleted is DRAFT, and if the `locked_by` user is not the same as the user performing the Delete. Audit fields updated include last_modified_timestamp, last_modified_by. Any existing lock is released when Deleting a integration. Currently, there is no undelete mechanism.");
            versions3 = versions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("download").about("Downloads an integration. Retrieves the `IntegrationVersion` for a given `integration_id` and returns the response as a string.");
            versions3 = versions3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Get a integration in the specified project.");
            versions3 = versions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all integration versions in the specified project.");
            versions3 = versions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Update a integration with a draft version in the specified project.");
            versions3 = versions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("publish").about("This RPC throws an exception if the integration is in ARCHIVED or ACTIVE state. This RPC throws an exception if the version being published is DRAFT, and if the `locked_by` user is not the same as the user performing the Publish. Audit fields updated include last_published_timestamp, last_published_by, last_modified_timestamp, last_modified_by. Any existing lock is on this integration is released.");
            versions3 = versions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("takeover_edit_lock").about("Clears the `locked_by` and `locked_at_timestamp`in the DRAFT version of this integration. It then performs the same action as the CreateDraftIntegrationVersion (i.e., copies the DRAFT version of the integration as a SNAPSHOT and then creates a new DRAFT version with the `locked_by` set to the `user_taking_over` and the `locked_at_timestamp` set to the current timestamp). Both the `locked_by` and `user_taking_over` are notified via email about the takeover. This RPC throws an exception if the integration is not in DRAFT status or if the `locked_by` and `locked_at_timestamp` fields are not set.The TakeoverEdit lock is treated the same as an edit of the integration, and hence shares ACLs with edit. Audit fields updated include last_modified_timestamp, last_modified_by.");
            versions3 = versions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("unpublish").about("Sets the status of the ACTIVE integration to SNAPSHOT with a new tag \"PREVIOUSLY_PUBLISHED\" after validating it. The \"HEAD\" and \"PUBLISH_REQUESTED\" tags do not change. This RPC throws an exception if the version being snapshot is not ACTIVE. Audit fields added include action, action_by, action_timestamp.");
            versions3 = versions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("upload").about("Uploads an integration. The content can be a previously downloaded integration. Performs the same function as CreateDraftIntegrationVersion, but accepts input in a string format, which holds the complete representation of the IntegrationVersion content.");
            versions3 = versions3.subcommand(mcmd);
        }
        let mut auth_configs3 = SubCommand::with_name("auth_configs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an auth config record. Fetch corresponding credentials for specific auth types, e.g. access token for OAuth 2.0, JWT token for JWT. Encrypt the auth config with Cloud KMS and store the encrypted credentials in Spanner. Returns the encrypted auth config.");
            auth_configs3 = auth_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an auth config.");
            auth_configs3 = auth_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a complete auth config. If the auth config doesn't exist, Code.NOT_FOUND exception will be thrown. Returns the decrypted auth config.");
            auth_configs3 = auth_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all auth configs that match the filter. Restrict to auth configs belong to the current client only.");
            auth_configs3 = auth_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an auth config. If credential is updated, fetch the encrypted auth config from Spanner, decrypt with Cloud KMS key, update the credential fields, re-encrypt with Cloud KMS key and update the Spanner record. For other fields, directly update the Spanner record. Returns the encrypted auth config.");
            auth_configs3 = auth_configs3.subcommand(mcmd);
        }
        let mut certificates3 = SubCommand::with_name("certificates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new certificate. The certificate will be registered to the trawler service and will be encrypted using cloud KMS and stored in Spanner Returns the certificate.");
            certificates3 = certificates3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a certificate");
            certificates3 = certificates3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Get a certificates in the specified project.");
            certificates3 = certificates3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List all the certificates that match the filter. Restrict to certificate of current client only.");
            certificates3 = certificates3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the certificate by id. If new certificate file is updated, it will register with the trawler service, re-encrypt with cloud KMS and update the Spanner record. Other fields will directly update the Spanner record. Returns the Certificate.");
            certificates3 = certificates3.subcommand(mcmd);
        }
        let mut integrations3 = SubCommand::with_name("integrations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: execute, list and schedule");
        {
            let mcmd = SubCommand::with_name("execute").about("Executes integrations synchronously by passing the trigger id in the request body. The request is not returned until the requested executions are either fulfilled or experienced an error. If the integration name is not specified (passing `-`), all of the associated integration under the given trigger_id will be executed. Otherwise only the specified integration for the given `trigger_id` is executed. This is helpful for execution the integration from UI.");
            integrations3 = integrations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all integrations in the specified project.");
            integrations3 = integrations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("schedule").about("Schedules an integration for execution by passing the trigger id and the scheduled time in the request body.");
            integrations3 = integrations3.subcommand(mcmd);
        }
        let mut integrationtemplates3 = SubCommand::with_name("integrationtemplates")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: versions");
        let mut sfdc_instances3 = SubCommand::with_name("sfdc_instances")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an sfdc instance record. Store the sfdc instance in Spanner. Returns the sfdc instance.");
            sfdc_instances3 = sfdc_instances3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an sfdc instance.");
            sfdc_instances3 = sfdc_instances3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an sfdc instance. If the instance doesn't exist, Code.NOT_FOUND exception will be thrown.");
            sfdc_instances3 = sfdc_instances3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all sfdc instances that match the filter. Restrict to sfdc instances belonging to the current client only.");
            sfdc_instances3 = sfdc_instances3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an sfdc instance. Updates the sfdc instance in spanner. Returns the sfdc instance.");
            sfdc_instances3 = sfdc_instances3.subcommand(mcmd);
        }
        let mut sfdc_channels3 = SubCommand::with_name("sfdc_channels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an sfdc channel record. Store the sfdc channel in Spanner. Returns the sfdc channel.");
            sfdc_channels3 = sfdc_channels3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an sfdc channel.");
            sfdc_channels3 = sfdc_channels3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an sfdc channel. If the channel doesn't exist, Code.NOT_FOUND exception will be thrown.");
            sfdc_channels3 = sfdc_channels3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all sfdc channels that match the filter. Restrict to sfdc channels belonging to the current client only.");
            sfdc_channels3 = sfdc_channels3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an sfdc channel. Updates the sfdc channel in spanner. Returns the sfdc channel.");
            sfdc_channels3 = sfdc_channels3.subcommand(mcmd);
        }
        let mut suspensions4 = SubCommand::with_name("suspensions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: lift, list and resolve");
        {
            let mcmd = SubCommand::with_name("lift").about("* Lifts suspension for advanced suspension task. Fetch corresponding suspension with provided suspension Id, resolve suspension, and set up suspension result for the Suspension Task.");
            suspensions4 = suspensions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("* Lists suspensions associated with a specific execution. Only those with permissions to resolve the relevant suspensions will be able to view them.");
            suspensions4 = suspensions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("resolve").about("* Resolves (lifts/rejects) any number of suspensions. If the integration is already running, only the status of the suspension is updated. Otherwise, the suspended integration will begin execution again.");
            suspensions4 = suspensions4.subcommand(mcmd);
        }
        let mut executions4 = SubCommand::with_name("executions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Cancellation of an execution");
            executions4 = executions4.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Get an execution in the specified project.");
            executions4 = executions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the results of all the integration executions. The response includes the same information as the [execution log](https://cloud.google.com/application-integration/docs/viewing-logs) in the Integration UI.");
            executions4 = executions4.subcommand(mcmd);
        }
        let mut versions4 = SubCommand::with_name("versions")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, download, get, list, patch, publish, takeover_edit_lock, unpublish and upload");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Create a integration with a draft version in the specified project.");
            versions4 = versions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Soft-deletes the integration. Changes the status of the integration to ARCHIVED. If the integration being ARCHIVED is tagged as \"HEAD\", the tag is removed from this snapshot and set to the previous non-ARCHIVED snapshot. The PUBLISH_REQUESTED, DUE_FOR_DELETION tags are removed too. This RPC throws an exception if the version being deleted is DRAFT, and if the `locked_by` user is not the same as the user performing the Delete. Audit fields updated include last_modified_timestamp, last_modified_by. Any existing lock is released when Deleting a integration. Currently, there is no undelete mechanism.");
            versions4 = versions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("download").about("Downloads an integration. Retrieves the `IntegrationVersion` for a given `integration_id` and returns the response as a string.");
            versions4 = versions4.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Get a integration in the specified project.");
            versions4 = versions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all integration versions in the specified project.");
            versions4 = versions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Update a integration with a draft version in the specified project.");
            versions4 = versions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("publish").about("This RPC throws an exception if the integration is in ARCHIVED or ACTIVE state. This RPC throws an exception if the version being published is DRAFT, and if the `locked_by` user is not the same as the user performing the Publish. Audit fields updated include last_published_timestamp, last_published_by, last_modified_timestamp, last_modified_by. Any existing lock is on this integration is released.");
            versions4 = versions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("takeover_edit_lock").about("Clears the `locked_by` and `locked_at_timestamp`in the DRAFT version of this integration. It then performs the same action as the CreateDraftIntegrationVersion (i.e., copies the DRAFT version of the integration as a SNAPSHOT and then creates a new DRAFT version with the `locked_by` set to the `user_taking_over` and the `locked_at_timestamp` set to the current timestamp). Both the `locked_by` and `user_taking_over` are notified via email about the takeover. This RPC throws an exception if the integration is not in DRAFT status or if the `locked_by` and `locked_at_timestamp` fields are not set.The TakeoverEdit lock is treated the same as an edit of the integration, and hence shares ACLs with edit. Audit fields updated include last_modified_timestamp, last_modified_by.");
            versions4 = versions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("unpublish").about("Sets the status of the ACTIVE integration to SNAPSHOT with a new tag \"PREVIOUSLY_PUBLISHED\" after validating it. The \"HEAD\" and \"PUBLISH_REQUESTED\" tags do not change. This RPC throws an exception if the version being snapshot is not ACTIVE. Audit fields added include action, action_by, action_timestamp.");
            versions4 = versions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("upload").about("Uploads an integration. The content can be a previously downloaded integration. Performs the same function as CreateDraftIntegrationVersion, but accepts input in a string format, which holds the complete representation of the IntegrationVersion content.");
            versions4 = versions4.subcommand(mcmd);
        }
        let mut versions4 = SubCommand::with_name("versions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get and list");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates an IntegrationTemplateVersion.");
            versions4 = versions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Returns an IntegrationTemplateVersion in the specified project.");
            versions4 = versions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Returns the list of all IntegrationTemplateVersions in the specified project.",
            );
            versions4 = versions4.subcommand(mcmd);
        }
        let mut sfdc_channels4 = SubCommand::with_name("sfdc_channels")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an sfdc channel record. Store the sfdc channel in Spanner. Returns the sfdc channel.");
            sfdc_channels4 = sfdc_channels4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an sfdc channel.");
            sfdc_channels4 = sfdc_channels4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an sfdc channel. If the channel doesn't exist, Code.NOT_FOUND exception will be thrown.");
            sfdc_channels4 = sfdc_channels4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all sfdc channels that match the filter. Restrict to sfdc channels belonging to the current client only.");
            sfdc_channels4 = sfdc_channels4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an sfdc channel. Updates the sfdc channel in spanner. Returns the sfdc channel.");
            sfdc_channels4 = sfdc_channels4.subcommand(mcmd);
        }
        let mut suspensions5 = SubCommand::with_name("suspensions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: lift, list and resolve");
        {
            let mcmd = SubCommand::with_name("lift").about("* Lifts suspension for advanced suspension task. Fetch corresponding suspension with provided suspension Id, resolve suspension, and set up suspension result for the Suspension Task.");
            suspensions5 = suspensions5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("* Lists suspensions associated with a specific execution. Only those with permissions to resolve the relevant suspensions will be able to view them.");
            suspensions5 = suspensions5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("resolve").about("* Resolves (lifts/rejects) any number of suspensions. If the integration is already running, only the status of the suspension is updated. Otherwise, the suspended integration will begin execution again.");
            suspensions5 = suspensions5.subcommand(mcmd);
        }
        executions4 = executions4.subcommand(suspensions5);
        sfdc_instances3 = sfdc_instances3.subcommand(sfdc_channels4);
        integrationtemplates3 = integrationtemplates3.subcommand(versions4);
        integrations3 = integrations3.subcommand(versions4);
        integrations3 = integrations3.subcommand(executions4);
        executions3 = executions3.subcommand(suspensions4);
        sfdc_instances2 = sfdc_instances2.subcommand(sfdc_channels3);
        products2 = products2.subcommand(sfdc_instances3);
        products2 = products2.subcommand(integrationtemplates3);
        products2 = products2.subcommand(integrations3);
        products2 = products2.subcommand(certificates3);
        products2 = products2.subcommand(auth_configs3);
        integrations2 = integrations2.subcommand(versions3);
        integrations2 = integrations2.subcommand(executions3);
        connections2 = connections2.subcommand(runtime_entity_schemas3);
        connections2 = connections2.subcommand(runtime_action_schemas3);
        locations1 = locations1.subcommand(sfdc_instances2);
        locations1 = locations1.subcommand(products2);
        locations1 = locations1.subcommand(integrations2);
        locations1 = locations1.subcommand(connections2);
        locations1 = locations1.subcommand(certificates2);
        locations1 = locations1.subcommand(auth_configs2);
        locations1 = locations1.subcommand(apps_script_projects2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);
        app = app.subcommand(connector_platform_regions0);
        app = app.subcommand(callback0);

        Self { app }
    }
}
use google_integrations1_alpha as api;

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
