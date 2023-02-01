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
        let mut app = App::new("dialogflow2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230130")
            .about("Builds conversational interfaces (for example, chatbots, and voice-powered apps and devices).")
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
            .about("methods: delete_agent, get_agent and set_agent");
        {
            let mcmd = SubCommand::with_name("delete_agent").about("Deletes the specified agent.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_agent").about("Retrieves the specified agent.");
            projects0 = projects0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_agent").about("Creates/updates the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            projects0 = projects0.subcommand(mcmd);
        }
        let mut agent1 = SubCommand::with_name("agent")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: export, get_fulfillment, get_validation_result, import, restore, search, train and update_fulfillment");
        {
            let mcmd = SubCommand::with_name("export").about("Exports the specified agent to a ZIP file. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: ExportAgentResponse");
            agent1 = agent1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_fulfillment").about("Retrieves the fulfillment.");
            agent1 = agent1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_validation_result").about("Gets agent validation result. Agent validation is performed during training time and is updated automatically when training is completed.");
            agent1 = agent1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Imports the specified agent from a ZIP file. Uploads new intents and entity types without deleting the existing ones. Intents and entity types with the same name are replaced with the new versions from ImportAgentRequest. After the import, the imported draft agent will be trained automatically (unless disabled in agent settings). However, once the import is done, training may not be completed yet. Please call TrainAgent and wait for the operation it returns in order to train explicitly. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) The operation only tracks when importing is complete, not when it is done training. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            agent1 = agent1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("restore").about("Restores the specified agent from a ZIP file. Replaces the current agent version with a new one. All the intents and entity types in the older version are deleted. After the restore, the restored draft agent will be trained automatically (unless disabled in agent settings). However, once the restore is done, training may not be completed yet. Please call TrainAgent and wait for the operation it returns in order to train explicitly. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) The operation only tracks when restoring is complete, not when it is done training. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            agent1 = agent1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search").about("Returns the list of agents. Since there is at most one conversational agent per project, this method is useful primarily for listing all agents across projects the caller has access to. One can achieve that with a wildcard project collection id \"-\". Refer to [List Sub-Collections](https://cloud.google.com/apis/design/design_patterns#list_sub-collections).");
            agent1 = agent1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("train").about("Trains the specified agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            agent1 = agent1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update_fulfillment").about("Updates the fulfillment.");
            agent1 = agent1.subcommand(mcmd);
        }
        let mut answer_records1 = SubCommand::with_name("answer_records")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list and patch");
        {
            let mcmd = SubCommand::with_name("list").about("Returns the list of all answer records in the specified project in reverse chronological order.");
            answer_records1 = answer_records1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified answer record.");
            answer_records1 = answer_records1.subcommand(mcmd);
        }
        let mut conversation_datasets1 = SubCommand::with_name("conversation_datasets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, import_conversation_data and list");
        {
            let mcmd =
                SubCommand::with_name("get").about("Retrieves the specified conversation dataset.");
            conversation_datasets1 = conversation_datasets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import_conversation_data").about("Import data into the specified conversation dataset. Note that it is not allowed to import data to a conversation dataset that already has data in it. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: ImportConversationDataOperationMetadata - `response`: ImportConversationDataOperationResponse");
            conversation_datasets1 = conversation_datasets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns the list of all conversation datasets in the specified project and location.");
            conversation_datasets1 = conversation_datasets1.subcommand(mcmd);
        }
        let mut conversation_models1 = SubCommand::with_name("conversation_models")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, deploy, get, list and undeploy");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a model. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: CreateConversationModelOperationMetadata - `response`: ConversationModel");
            conversation_models1 = conversation_models1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a model. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: DeleteConversationModelOperationMetadata - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty)");
            conversation_models1 = conversation_models1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("deploy").about("Deploys a model. If a model is already deployed, deploying it has no effect. A model can only serve prediction requests after it gets deployed. For article suggestion, custom model will not be used unless it is deployed. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: DeployConversationModelOperationMetadata - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty)");
            conversation_models1 = conversation_models1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets conversation model.");
            conversation_models1 = conversation_models1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists conversation models.");
            conversation_models1 = conversation_models1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undeploy").about("Undeploys a model. If the model is not deployed this method has no effect. If the model is currently being used: - For article suggestion, article suggestion will fallback to the default model if model is undeployed. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: UndeployConversationModelOperationMetadata - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty)");
            conversation_models1 = conversation_models1.subcommand(mcmd);
        }
        let mut conversation_profiles1 = SubCommand::with_name("conversation_profiles")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: clear_suggestion_feature_config, create, delete, get, list, patch and set_suggestion_feature_config");
        {
            let mcmd = SubCommand::with_name("clear_suggestion_feature_config").about("Clears a suggestion feature from a conversation profile for the given participant role. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: ClearSuggestionFeatureConfigOperationMetadata - `response`: ConversationProfile");
            conversation_profiles1 = conversation_profiles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a conversation profile in the specified project. ConversationProfile.CreateTime and ConversationProfile.UpdateTime aren't populated in the response. You can retrieve them via GetConversationProfile API.");
            conversation_profiles1 = conversation_profiles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified conversation profile.");
            conversation_profiles1 = conversation_profiles1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Retrieves the specified conversation profile.");
            conversation_profiles1 = conversation_profiles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all conversation profiles in the specified project.");
            conversation_profiles1 = conversation_profiles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified conversation profile. ConversationProfile.CreateTime and ConversationProfile.UpdateTime aren't populated in the response. You can retrieve them via GetConversationProfile API.");
            conversation_profiles1 = conversation_profiles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_suggestion_feature_config").about("Adds or updates a suggestion feature in a conversation profile. If the conversation profile contains the type of suggestion feature for the participant role, it will update it. Otherwise it will insert the suggestion feature. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: SetSuggestionFeatureConfigOperationMetadata - `response`: ConversationProfile If a long running operation to add or update suggestion feature config for the same conversation profile, participant role and suggestion feature type exists, please cancel the existing long running operation before sending such request, otherwise the request will be rejected.");
            conversation_profiles1 = conversation_profiles1.subcommand(mcmd);
        }
        let mut conversations1 = SubCommand::with_name("conversations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: complete, create, get and list");
        {
            let mcmd = SubCommand::with_name("complete").about("Completes the specified conversation. Finished conversations are purged from the database after 30 days.");
            conversations1 = conversations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new conversation. Conversations are auto-completed after 24 hours. Conversation Lifecycle: There are two stages during a conversation: Automated Agent Stage and Assist Stage. For Automated Agent Stage, there will be a dialogflow agent responding to user queries. For Assist Stage, there's no dialogflow agent responding to user queries. But we will provide suggestions which are generated from conversation. If Conversation.conversation_profile is configured for a dialogflow agent, conversation will start from `Automated Agent Stage`, otherwise, it will start from `Assist Stage`. And during `Automated Agent Stage`, once an Intent with Intent.live_agent_handoff is triggered, conversation will transfer to Assist Stage.");
            conversations1 = conversations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specific conversation.");
            conversations1 = conversations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all conversations in the specified project.");
            conversations1 = conversations1.subcommand(mcmd);
        }
        let mut knowledge_bases1 = SubCommand::with_name("knowledge_bases")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a knowledge base.");
            knowledge_bases1 = knowledge_bases1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the specified knowledge base.");
            knowledge_bases1 = knowledge_bases1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Retrieves the specified knowledge base.");
            knowledge_bases1 = knowledge_bases1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all knowledge bases of the specified agent.");
            knowledge_bases1 = knowledge_bases1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates the specified knowledge base.");
            knowledge_bases1 = knowledge_bases1.subcommand(mcmd);
        }
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete_agent, get, get_agent, list and set_agent");
        {
            let mcmd = SubCommand::with_name("delete_agent").about("Deletes the specified agent.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets information about a location.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_agent").about("Retrieves the specified agent.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists information about the supported locations for this service.");
            locations1 = locations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_agent").about("Creates/updates the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            locations1 = locations1.subcommand(mcmd);
        }
        let mut operations1 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
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
        let mut entity_types2 = SubCommand::with_name("entity_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_delete, batch_update, create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("batch_delete").about("Deletes entity types in the specified agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            entity_types2 = entity_types2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_update").about("Updates/Creates multiple entity types in the specified agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: BatchUpdateEntityTypesResponse Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            entity_types2 = entity_types2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates an entity type in the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            entity_types2 = entity_types2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified entity type. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            entity_types2 = entity_types2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified entity type.");
            entity_types2 = entity_types2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all entity types in the specified agent.");
            entity_types2 = entity_types2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified entity type. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            entity_types2 = entity_types2.subcommand(mcmd);
        }
        let mut environments2 = SubCommand::with_name("environments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, get_history, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an agent environment.");
            environments2 = environments2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the specified agent environment.");
            environments2 = environments2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Retrieves the specified agent environment.");
            environments2 = environments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_history")
                .about("Gets the history of the specified environment.");
            environments2 = environments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all non-default environments of the specified agent.");
            environments2 = environments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified agent environment. This method allows you to deploy new agent versions into the environment. When an environment is pointed to a new agent version by setting `environment.agent_version`, the environment is temporarily set to the `LOADING` state. During that time, the environment continues serving the previous version of the agent. After the new agent version is done loading, the environment is set back to the `RUNNING` state. You can use \"-\" as Environment ID in environment name to update an agent version in the default environment. WARNING: this will negate all recent changes to the draft agent and can't be undone. You may want to save the draft agent to a version before calling this method.");
            environments2 = environments2.subcommand(mcmd);
        }
        let mut intents2 = SubCommand::with_name("intents")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_delete, batch_update, create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("batch_delete").about("Deletes intents in the specified agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            intents2 = intents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_update").about("Updates/Creates multiple intents in the specified agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: BatchUpdateIntentsResponse Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            intents2 = intents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates an intent in the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            intents2 = intents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified intent and its direct or indirect followup intents. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            intents2 = intents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified intent.");
            intents2 = intents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all intents in the specified agent.");
            intents2 = intents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified intent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            intents2 = intents2.subcommand(mcmd);
        }
        let mut knowledge_bases2 = SubCommand::with_name("knowledge_bases")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a knowledge base.");
            knowledge_bases2 = knowledge_bases2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the specified knowledge base.");
            knowledge_bases2 = knowledge_bases2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Retrieves the specified knowledge base.");
            knowledge_bases2 = knowledge_bases2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all knowledge bases of the specified agent.");
            knowledge_bases2 = knowledge_bases2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates the specified knowledge base.");
            knowledge_bases2 = knowledge_bases2.subcommand(mcmd);
        }
        let mut sessions2 = SubCommand::with_name("sessions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete_contexts and detect_intent");
        {
            let mcmd = SubCommand::with_name("delete_contexts")
                .about("Deletes all active contexts in the specified session.");
            sessions2 = sessions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("detect_intent").about("Processes a natural language query and returns structured, actionable data as a result. This method is not idempotent, because it may cause contexts and session entity types to be updated, which in turn might affect results of future queries. If you might use [Agent Assist](https://cloud.google.com/dialogflow/docs/#aa) or other CCAI products now or in the future, consider using AnalyzeContent instead of `DetectIntent`. `AnalyzeContent` has additional functionality for Agent Assist and other CCAI products. Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/es/docs/agents-versions).");
            sessions2 = sessions2.subcommand(mcmd);
        }
        let mut versions2 = SubCommand::with_name("versions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an agent version. The new version points to the agent instance in the \"default\" environment.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete the specified agent version.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified agent version.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all versions of the specified agent.");
            versions2 = versions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified agent version. Note that this method does not allow you to update the state of the agent the given version points to. It allows you to update only mutable properties of the version resource.");
            versions2 = versions2.subcommand(mcmd);
        }
        let mut evaluations2 = SubCommand::with_name("evaluations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets an evaluation of conversation model.");
            evaluations2 = evaluations2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists evaluations of a conversation model.");
            evaluations2 = evaluations2.subcommand(mcmd);
        }
        let mut messages2 = SubCommand::with_name("messages")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists messages that belong to a given conversation. `messages` are ordered by `create_time` in descending order. To fetch updates without duplication, send request with filter `create_time_epoch_microseconds > [first item's create_time of previous request]` and empty page_token.");
            messages2 = messages2.subcommand(mcmd);
        }
        let mut participants2 = SubCommand::with_name("participants")
            .setting(AppSettings::ColoredHelp)
            .about("methods: analyze_content, create, get, list and patch");
        {
            let mcmd = SubCommand::with_name("analyze_content").about("Adds a text (chat, for example), or audio (phone recording, for example) message from a participant into the conversation. Note: Always use agent versions for production traffic sent to virtual agents. See [Versions and environments](https://cloud.google.com/dialogflow/es/docs/agents-versions).");
            participants2 = participants2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new participant in a conversation.");
            participants2 = participants2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a conversation participant.");
            participants2 = participants2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all participants in the specified conversation.");
            participants2 = participants2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified participant.");
            participants2 = participants2.subcommand(mcmd);
        }
        let mut suggestions2 = SubCommand::with_name("suggestions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: suggest_conversation_summary");
        {
            let mcmd = SubCommand::with_name("suggest_conversation_summary").about("Suggests summary for a conversation based on specific historical messages. The range of the messages to be used for summary can be specified in the request.");
            suggestions2 = suggestions2.subcommand(mcmd);
        }
        let mut documents2 = SubCommand::with_name("documents")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, export, get, import, list, patch and reload");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new document. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: Document");
            documents2 = documents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified document. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty)");
            documents2 = documents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("export").about("Exports a smart messaging candidate document into the specified destination. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: Document");
            documents2 = documents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified document.");
            documents2 = documents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Creates documents by importing data from external sources. Dialogflow supports up to 350 documents in each request. If you try to import more, Dialogflow will return an error. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: ImportDocumentsResponse");
            documents2 = documents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all documents of the knowledge base.");
            documents2 = documents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified document. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: Document");
            documents2 = documents2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reload").about("Reloads the specified document from its specified source, content_uri or content. The previously loaded content of the document will be deleted. Note: Even when the content of the document has not changed, there still may be side effects because of internal implementation changes. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: Document Note: The `projects.agent.knowledgeBases.documents` resource is deprecated; only use `projects.knowledgeBases.documents`.");
            documents2 = documents2.subcommand(mcmd);
        }
        let mut agent2 = SubCommand::with_name("agent")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: export, get_fulfillment, get_validation_result, import, restore, search, train and update_fulfillment");
        {
            let mcmd = SubCommand::with_name("export").about("Exports the specified agent to a ZIP file. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: ExportAgentResponse");
            agent2 = agent2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_fulfillment").about("Retrieves the fulfillment.");
            agent2 = agent2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_validation_result").about("Gets agent validation result. Agent validation is performed during training time and is updated automatically when training is completed.");
            agent2 = agent2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Imports the specified agent from a ZIP file. Uploads new intents and entity types without deleting the existing ones. Intents and entity types with the same name are replaced with the new versions from ImportAgentRequest. After the import, the imported draft agent will be trained automatically (unless disabled in agent settings). However, once the import is done, training may not be completed yet. Please call TrainAgent and wait for the operation it returns in order to train explicitly. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) The operation only tracks when importing is complete, not when it is done training. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            agent2 = agent2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("restore").about("Restores the specified agent from a ZIP file. Replaces the current agent version with a new one. All the intents and entity types in the older version are deleted. After the restore, the restored draft agent will be trained automatically (unless disabled in agent settings). However, once the restore is done, training may not be completed yet. Please call TrainAgent and wait for the operation it returns in order to train explicitly. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) The operation only tracks when restoring is complete, not when it is done training. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            agent2 = agent2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search").about("Returns the list of agents. Since there is at most one conversational agent per project, this method is useful primarily for listing all agents across projects the caller has access to. One can achieve that with a wildcard project collection id \"-\". Refer to [List Sub-Collections](https://cloud.google.com/apis/design/design_patterns#list_sub-collections).");
            agent2 = agent2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("train").about("Trains the specified agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            agent2 = agent2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update_fulfillment").about("Updates the fulfillment.");
            agent2 = agent2.subcommand(mcmd);
        }
        let mut answer_records2 = SubCommand::with_name("answer_records")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list and patch");
        {
            let mcmd = SubCommand::with_name("list").about("Returns the list of all answer records in the specified project in reverse chronological order.");
            answer_records2 = answer_records2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified answer record.");
            answer_records2 = answer_records2.subcommand(mcmd);
        }
        let mut conversation_datasets2 = SubCommand::with_name("conversation_datasets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, import_conversation_data and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new conversation dataset. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: CreateConversationDatasetOperationMetadata - `response`: ConversationDataset");
            conversation_datasets2 = conversation_datasets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified conversation dataset. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: DeleteConversationDatasetOperationMetadata - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty)");
            conversation_datasets2 = conversation_datasets2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Retrieves the specified conversation dataset.");
            conversation_datasets2 = conversation_datasets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import_conversation_data").about("Import data into the specified conversation dataset. Note that it is not allowed to import data to a conversation dataset that already has data in it. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: ImportConversationDataOperationMetadata - `response`: ImportConversationDataOperationResponse");
            conversation_datasets2 = conversation_datasets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns the list of all conversation datasets in the specified project and location.");
            conversation_datasets2 = conversation_datasets2.subcommand(mcmd);
        }
        let mut conversation_models2 = SubCommand::with_name("conversation_models")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, deploy, get, list and undeploy");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a model. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: CreateConversationModelOperationMetadata - `response`: ConversationModel");
            conversation_models2 = conversation_models2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a model. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: DeleteConversationModelOperationMetadata - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty)");
            conversation_models2 = conversation_models2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("deploy").about("Deploys a model. If a model is already deployed, deploying it has no effect. A model can only serve prediction requests after it gets deployed. For article suggestion, custom model will not be used unless it is deployed. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: DeployConversationModelOperationMetadata - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty)");
            conversation_models2 = conversation_models2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets conversation model.");
            conversation_models2 = conversation_models2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists conversation models.");
            conversation_models2 = conversation_models2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undeploy").about("Undeploys a model. If the model is not deployed this method has no effect. If the model is currently being used: - For article suggestion, article suggestion will fallback to the default model if model is undeployed. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: UndeployConversationModelOperationMetadata - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty)");
            conversation_models2 = conversation_models2.subcommand(mcmd);
        }
        let mut conversation_profiles2 = SubCommand::with_name("conversation_profiles")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: clear_suggestion_feature_config, create, delete, get, list, patch and set_suggestion_feature_config");
        {
            let mcmd = SubCommand::with_name("clear_suggestion_feature_config").about("Clears a suggestion feature from a conversation profile for the given participant role. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: ClearSuggestionFeatureConfigOperationMetadata - `response`: ConversationProfile");
            conversation_profiles2 = conversation_profiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a conversation profile in the specified project. ConversationProfile.CreateTime and ConversationProfile.UpdateTime aren't populated in the response. You can retrieve them via GetConversationProfile API.");
            conversation_profiles2 = conversation_profiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified conversation profile.");
            conversation_profiles2 = conversation_profiles2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Retrieves the specified conversation profile.");
            conversation_profiles2 = conversation_profiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all conversation profiles in the specified project.");
            conversation_profiles2 = conversation_profiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified conversation profile. ConversationProfile.CreateTime and ConversationProfile.UpdateTime aren't populated in the response. You can retrieve them via GetConversationProfile API.");
            conversation_profiles2 = conversation_profiles2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_suggestion_feature_config").about("Adds or updates a suggestion feature in a conversation profile. If the conversation profile contains the type of suggestion feature for the participant role, it will update it. Otherwise it will insert the suggestion feature. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: SetSuggestionFeatureConfigOperationMetadata - `response`: ConversationProfile If a long running operation to add or update suggestion feature config for the same conversation profile, participant role and suggestion feature type exists, please cancel the existing long running operation before sending such request, otherwise the request will be rejected.");
            conversation_profiles2 = conversation_profiles2.subcommand(mcmd);
        }
        let mut conversations2 = SubCommand::with_name("conversations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: complete, create, get and list");
        {
            let mcmd = SubCommand::with_name("complete").about("Completes the specified conversation. Finished conversations are purged from the database after 30 days.");
            conversations2 = conversations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new conversation. Conversations are auto-completed after 24 hours. Conversation Lifecycle: There are two stages during a conversation: Automated Agent Stage and Assist Stage. For Automated Agent Stage, there will be a dialogflow agent responding to user queries. For Assist Stage, there's no dialogflow agent responding to user queries. But we will provide suggestions which are generated from conversation. If Conversation.conversation_profile is configured for a dialogflow agent, conversation will start from `Automated Agent Stage`, otherwise, it will start from `Assist Stage`. And during `Automated Agent Stage`, once an Intent with Intent.live_agent_handoff is triggered, conversation will transfer to Assist Stage.");
            conversations2 = conversations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specific conversation.");
            conversations2 = conversations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all conversations in the specified project.");
            conversations2 = conversations2.subcommand(mcmd);
        }
        let mut knowledge_bases2 = SubCommand::with_name("knowledge_bases")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a knowledge base.");
            knowledge_bases2 = knowledge_bases2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the specified knowledge base.");
            knowledge_bases2 = knowledge_bases2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Retrieves the specified knowledge base.");
            knowledge_bases2 = knowledge_bases2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all knowledge bases of the specified agent.");
            knowledge_bases2 = knowledge_bases2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates the specified knowledge base.");
            knowledge_bases2 = knowledge_bases2.subcommand(mcmd);
        }
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
        let mut entities3 = SubCommand::with_name("entities")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_create, batch_delete and batch_update");
        {
            let mcmd = SubCommand::with_name("batch_create").about("Creates multiple new entities in the specified entity type. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            entities3 = entities3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_delete").about("Deletes entities in the specified entity type. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            entities3 = entities3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_update").about("Updates or creates multiple entities in the specified entity type. This method does not affect entities in the entity type that aren't explicitly specified in the request. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training). ");
            entities3 = entities3.subcommand(mcmd);
        }
        let mut intents3 = SubCommand::with_name("intents")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all intents in the specified agent.");
            intents3 = intents3.subcommand(mcmd);
        }
        let mut users3 = SubCommand::with_name("users")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: sessions");
        let mut documents3 = SubCommand::with_name("documents")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and reload");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new document. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: Document");
            documents3 = documents3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified document. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty)");
            documents3 = documents3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified document.");
            documents3 = documents3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all documents of the knowledge base.");
            documents3 = documents3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified document. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: Document");
            documents3 = documents3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reload").about("Reloads the specified document from its specified source, content_uri or content. The previously loaded content of the document will be deleted. Note: Even when the content of the document has not changed, there still may be side effects because of internal implementation changes. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: Document Note: The `projects.agent.knowledgeBases.documents` resource is deprecated; only use `projects.knowledgeBases.documents`.");
            documents3 = documents3.subcommand(mcmd);
        }
        let mut contexts3 = SubCommand::with_name("contexts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a context. If the specified context already exists, overrides the context.");
            contexts3 = contexts3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified context.");
            contexts3 = contexts3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified context.");
            contexts3 = contexts3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all contexts in the specified session.");
            contexts3 = contexts3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified context.");
            contexts3 = contexts3.subcommand(mcmd);
        }
        let mut entity_types3 = SubCommand::with_name("entity_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a session entity type. If the specified session entity type already exists, overrides the session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.");
            entity_types3 = entity_types3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.");
            entity_types3 = entity_types3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.");
            entity_types3 = entity_types3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns the list of all session entity types in the specified session. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.");
            entity_types3 = entity_types3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.");
            entity_types3 = entity_types3.subcommand(mcmd);
        }
        let mut suggestions3 = SubCommand::with_name("suggestions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: suggest_articles, suggest_faq_answers and suggest_smart_replies");
        {
            let mcmd = SubCommand::with_name("suggest_articles").about(
                "Gets suggested articles for a participant based on specific historical messages.",
            );
            suggestions3 = suggestions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("suggest_faq_answers").about("Gets suggested faq answers for a participant based on specific historical messages.");
            suggestions3 = suggestions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("suggest_smart_replies").about(
                "Gets smart replies for a participant based on specific historical messages.",
            );
            suggestions3 = suggestions3.subcommand(mcmd);
        }
        let mut entity_types3 = SubCommand::with_name("entity_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_delete, batch_update, create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("batch_delete").about("Deletes entity types in the specified agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            entity_types3 = entity_types3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_update").about("Updates/Creates multiple entity types in the specified agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: BatchUpdateEntityTypesResponse Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            entity_types3 = entity_types3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates an entity type in the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            entity_types3 = entity_types3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified entity type. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            entity_types3 = entity_types3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified entity type.");
            entity_types3 = entity_types3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all entity types in the specified agent.");
            entity_types3 = entity_types3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified entity type. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            entity_types3 = entity_types3.subcommand(mcmd);
        }
        let mut environments3 = SubCommand::with_name("environments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, get_history, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an agent environment.");
            environments3 = environments3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes the specified agent environment.");
            environments3 = environments3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Retrieves the specified agent environment.");
            environments3 = environments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_history")
                .about("Gets the history of the specified environment.");
            environments3 = environments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all non-default environments of the specified agent.");
            environments3 = environments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified agent environment. This method allows you to deploy new agent versions into the environment. When an environment is pointed to a new agent version by setting `environment.agent_version`, the environment is temporarily set to the `LOADING` state. During that time, the environment continues serving the previous version of the agent. After the new agent version is done loading, the environment is set back to the `RUNNING` state. You can use \"-\" as Environment ID in environment name to update an agent version in the default environment. WARNING: this will negate all recent changes to the draft agent and can't be undone. You may want to save the draft agent to a version before calling this method.");
            environments3 = environments3.subcommand(mcmd);
        }
        let mut intents3 = SubCommand::with_name("intents")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_delete, batch_update, create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("batch_delete").about("Deletes intents in the specified agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            intents3 = intents3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_update").about("Updates/Creates multiple intents in the specified agent. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: BatchUpdateIntentsResponse Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            intents3 = intents3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates an intent in the specified agent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            intents3 = intents3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified intent and its direct or indirect followup intents. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            intents3 = intents3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified intent.");
            intents3 = intents3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all intents in the specified agent.");
            intents3 = intents3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified intent. Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            intents3 = intents3.subcommand(mcmd);
        }
        let mut sessions3 = SubCommand::with_name("sessions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete_contexts and detect_intent");
        {
            let mcmd = SubCommand::with_name("delete_contexts")
                .about("Deletes all active contexts in the specified session.");
            sessions3 = sessions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("detect_intent").about("Processes a natural language query and returns structured, actionable data as a result. This method is not idempotent, because it may cause contexts and session entity types to be updated, which in turn might affect results of future queries. If you might use [Agent Assist](https://cloud.google.com/dialogflow/docs/#aa) or other CCAI products now or in the future, consider using AnalyzeContent instead of `DetectIntent`. `AnalyzeContent` has additional functionality for Agent Assist and other CCAI products. Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/es/docs/agents-versions).");
            sessions3 = sessions3.subcommand(mcmd);
        }
        let mut versions3 = SubCommand::with_name("versions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an agent version. The new version points to the agent instance in the \"default\" environment.");
            versions3 = versions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete the specified agent version.");
            versions3 = versions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified agent version.");
            versions3 = versions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all versions of the specified agent.");
            versions3 = versions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified agent version. Note that this method does not allow you to update the state of the agent the given version points to. It allows you to update only mutable properties of the version resource.");
            versions3 = versions3.subcommand(mcmd);
        }
        let mut evaluations3 = SubCommand::with_name("evaluations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get and list");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates evaluation of a conversation model.");
            evaluations3 = evaluations3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets an evaluation of conversation model.");
            evaluations3 = evaluations3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists evaluations of a conversation model.");
            evaluations3 = evaluations3.subcommand(mcmd);
        }
        let mut messages3 = SubCommand::with_name("messages")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists messages that belong to a given conversation. `messages` are ordered by `create_time` in descending order. To fetch updates without duplication, send request with filter `create_time_epoch_microseconds > [first item's create_time of previous request]` and empty page_token.");
            messages3 = messages3.subcommand(mcmd);
        }
        let mut participants3 = SubCommand::with_name("participants")
            .setting(AppSettings::ColoredHelp)
            .about("methods: analyze_content, create, get, list and patch");
        {
            let mcmd = SubCommand::with_name("analyze_content").about("Adds a text (chat, for example), or audio (phone recording, for example) message from a participant into the conversation. Note: Always use agent versions for production traffic sent to virtual agents. See [Versions and environments](https://cloud.google.com/dialogflow/es/docs/agents-versions).");
            participants3 = participants3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new participant in a conversation.");
            participants3 = participants3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a conversation participant.");
            participants3 = participants3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all participants in the specified conversation.");
            participants3 = participants3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified participant.");
            participants3 = participants3.subcommand(mcmd);
        }
        let mut suggestions3 = SubCommand::with_name("suggestions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: suggest_conversation_summary");
        {
            let mcmd = SubCommand::with_name("suggest_conversation_summary").about("Suggests summary for a conversation based on specific historical messages. The range of the messages to be used for summary can be specified in the request.");
            suggestions3 = suggestions3.subcommand(mcmd);
        }
        let mut documents3 = SubCommand::with_name("documents")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, export, get, import, list, patch and reload");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new document. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: Document");
            documents3 = documents3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified document. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty)");
            documents3 = documents3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("export").about("Exports a smart messaging candidate document into the specified destination. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: Document");
            documents3 = documents3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified document.");
            documents3 = documents3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Creates documents by importing data from external sources. Dialogflow supports up to 350 documents in each request. If you try to import more, Dialogflow will return an error. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: ImportDocumentsResponse");
            documents3 = documents3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all documents of the knowledge base.");
            documents3 = documents3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified document. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: Document");
            documents3 = documents3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reload").about("Reloads the specified document from its specified source, content_uri or content. The previously loaded content of the document will be deleted. Note: Even when the content of the document has not changed, there still may be side effects because of internal implementation changes. This method is a [long-running operation](https://cloud.google.com/dialogflow/cx/docs/how/long-running-operation). The returned `Operation` type has the following method-specific fields: - `metadata`: KnowledgeOperationMetadata - `response`: Document Note: The `projects.agent.knowledgeBases.documents` resource is deprecated; only use `projects.knowledgeBases.documents`.");
            documents3 = documents3.subcommand(mcmd);
        }
        let mut sessions4 = SubCommand::with_name("sessions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete_contexts and detect_intent");
        {
            let mcmd = SubCommand::with_name("delete_contexts")
                .about("Deletes all active contexts in the specified session.");
            sessions4 = sessions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("detect_intent").about("Processes a natural language query and returns structured, actionable data as a result. This method is not idempotent, because it may cause contexts and session entity types to be updated, which in turn might affect results of future queries. If you might use [Agent Assist](https://cloud.google.com/dialogflow/docs/#aa) or other CCAI products now or in the future, consider using AnalyzeContent instead of `DetectIntent`. `AnalyzeContent` has additional functionality for Agent Assist and other CCAI products. Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/es/docs/agents-versions).");
            sessions4 = sessions4.subcommand(mcmd);
        }
        let mut entities4 = SubCommand::with_name("entities")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_create, batch_delete and batch_update");
        {
            let mcmd = SubCommand::with_name("batch_create").about("Creates multiple new entities in the specified entity type. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            entities4 = entities4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_delete").about("Deletes entities in the specified entity type. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training).");
            entities4 = entities4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_update").about("Updates or creates multiple entities in the specified entity type. This method does not affect entities in the entity type that aren't explicitly specified in the request. This method is a [long-running operation](https://cloud.google.com/dialogflow/es/docs/how/long-running-operations). The returned `Operation` type has the following method-specific fields: - `metadata`: An empty [Struct message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#struct) - `response`: An [Empty message](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#empty) Note: You should always train an agent prior to sending it queries. See the [training documentation](https://cloud.google.com/dialogflow/es/docs/training). ");
            entities4 = entities4.subcommand(mcmd);
        }
        let mut intents4 = SubCommand::with_name("intents")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all intents in the specified agent.");
            intents4 = intents4.subcommand(mcmd);
        }
        let mut users4 = SubCommand::with_name("users")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: sessions");
        let mut contexts4 = SubCommand::with_name("contexts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a context. If the specified context already exists, overrides the context.");
            contexts4 = contexts4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified context.");
            contexts4 = contexts4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified context.");
            contexts4 = contexts4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all contexts in the specified session.");
            contexts4 = contexts4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified context.");
            contexts4 = contexts4.subcommand(mcmd);
        }
        let mut entity_types4 = SubCommand::with_name("entity_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a session entity type. If the specified session entity type already exists, overrides the session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.");
            entity_types4 = entity_types4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.");
            entity_types4 = entity_types4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.");
            entity_types4 = entity_types4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns the list of all session entity types in the specified session. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.");
            entity_types4 = entity_types4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.");
            entity_types4 = entity_types4.subcommand(mcmd);
        }
        let mut suggestions4 = SubCommand::with_name("suggestions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: suggest_articles, suggest_faq_answers and suggest_smart_replies");
        {
            let mcmd = SubCommand::with_name("suggest_articles").about(
                "Gets suggested articles for a participant based on specific historical messages.",
            );
            suggestions4 = suggestions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("suggest_faq_answers").about("Gets suggested faq answers for a participant based on specific historical messages.");
            suggestions4 = suggestions4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("suggest_smart_replies").about(
                "Gets smart replies for a participant based on specific historical messages.",
            );
            suggestions4 = suggestions4.subcommand(mcmd);
        }
        let mut contexts5 = SubCommand::with_name("contexts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a context. If the specified context already exists, overrides the context.");
            contexts5 = contexts5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified context.");
            contexts5 = contexts5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified context.");
            contexts5 = contexts5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all contexts in the specified session.");
            contexts5 = contexts5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified context.");
            contexts5 = contexts5.subcommand(mcmd);
        }
        let mut entity_types5 = SubCommand::with_name("entity_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a session entity type. If the specified session entity type already exists, overrides the session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.");
            entity_types5 = entity_types5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.");
            entity_types5 = entity_types5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.");
            entity_types5 = entity_types5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns the list of all session entity types in the specified session. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.");
            entity_types5 = entity_types5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.");
            entity_types5 = entity_types5.subcommand(mcmd);
        }
        let mut sessions5 = SubCommand::with_name("sessions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete_contexts and detect_intent");
        {
            let mcmd = SubCommand::with_name("delete_contexts")
                .about("Deletes all active contexts in the specified session.");
            sessions5 = sessions5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("detect_intent").about("Processes a natural language query and returns structured, actionable data as a result. This method is not idempotent, because it may cause contexts and session entity types to be updated, which in turn might affect results of future queries. If you might use [Agent Assist](https://cloud.google.com/dialogflow/docs/#aa) or other CCAI products now or in the future, consider using AnalyzeContent instead of `DetectIntent`. `AnalyzeContent` has additional functionality for Agent Assist and other CCAI products. Note: Always use agent versions for production traffic. See [Versions and environments](https://cloud.google.com/dialogflow/es/docs/agents-versions).");
            sessions5 = sessions5.subcommand(mcmd);
        }
        let mut contexts6 = SubCommand::with_name("contexts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a context. If the specified context already exists, overrides the context.");
            contexts6 = contexts6.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified context.");
            contexts6 = contexts6.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified context.");
            contexts6 = contexts6.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of all contexts in the specified session.");
            contexts6 = contexts6.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified context.");
            contexts6 = contexts6.subcommand(mcmd);
        }
        let mut entity_types6 = SubCommand::with_name("entity_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a session entity type. If the specified session entity type already exists, overrides the session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.");
            entity_types6 = entity_types6.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.");
            entity_types6 = entity_types6.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.");
            entity_types6 = entity_types6.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns the list of all session entity types in the specified session. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.");
            entity_types6 = entity_types6.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the specified session entity type. This method doesn't work with Google Assistant integration. Contact Dialogflow support if you need to use session entities with Google Assistant integration.");
            entity_types6 = entity_types6.subcommand(mcmd);
        }
        sessions5 = sessions5.subcommand(entity_types6);
        sessions5 = sessions5.subcommand(contexts6);
        users4 = users4.subcommand(sessions5);
        sessions4 = sessions4.subcommand(entity_types5);
        sessions4 = sessions4.subcommand(contexts5);
        participants3 = participants3.subcommand(suggestions4);
        sessions3 = sessions3.subcommand(entity_types4);
        sessions3 = sessions3.subcommand(contexts4);
        environments3 = environments3.subcommand(users4);
        environments3 = environments3.subcommand(intents4);
        entity_types3 = entity_types3.subcommand(entities4);
        users3 = users3.subcommand(sessions4);
        knowledge_bases2 = knowledge_bases2.subcommand(documents3);
        conversations2 = conversations2.subcommand(suggestions3);
        conversations2 = conversations2.subcommand(participants3);
        conversations2 = conversations2.subcommand(messages3);
        conversation_models2 = conversation_models2.subcommand(evaluations3);
        agent2 = agent2.subcommand(versions3);
        agent2 = agent2.subcommand(sessions3);
        agent2 = agent2.subcommand(intents3);
        agent2 = agent2.subcommand(environments3);
        agent2 = agent2.subcommand(entity_types3);
        participants2 = participants2.subcommand(suggestions3);
        sessions2 = sessions2.subcommand(entity_types3);
        sessions2 = sessions2.subcommand(contexts3);
        knowledge_bases2 = knowledge_bases2.subcommand(documents3);
        environments2 = environments2.subcommand(users3);
        environments2 = environments2.subcommand(intents3);
        entity_types2 = entity_types2.subcommand(entities3);
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(knowledge_bases2);
        locations1 = locations1.subcommand(conversations2);
        locations1 = locations1.subcommand(conversation_profiles2);
        locations1 = locations1.subcommand(conversation_models2);
        locations1 = locations1.subcommand(conversation_datasets2);
        locations1 = locations1.subcommand(answer_records2);
        locations1 = locations1.subcommand(agent2);
        knowledge_bases1 = knowledge_bases1.subcommand(documents2);
        conversations1 = conversations1.subcommand(suggestions2);
        conversations1 = conversations1.subcommand(participants2);
        conversations1 = conversations1.subcommand(messages2);
        conversation_models1 = conversation_models1.subcommand(evaluations2);
        agent1 = agent1.subcommand(versions2);
        agent1 = agent1.subcommand(sessions2);
        agent1 = agent1.subcommand(knowledge_bases2);
        agent1 = agent1.subcommand(intents2);
        agent1 = agent1.subcommand(environments2);
        agent1 = agent1.subcommand(entity_types2);
        projects0 = projects0.subcommand(operations1);
        projects0 = projects0.subcommand(locations1);
        projects0 = projects0.subcommand(knowledge_bases1);
        projects0 = projects0.subcommand(conversations1);
        projects0 = projects0.subcommand(conversation_profiles1);
        projects0 = projects0.subcommand(conversation_models1);
        projects0 = projects0.subcommand(conversation_datasets1);
        projects0 = projects0.subcommand(answer_records1);
        projects0 = projects0.subcommand(agent1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_dialogflow2 as api;

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
