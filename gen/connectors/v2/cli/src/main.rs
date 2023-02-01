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
        let mut app = App::new("connectors2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230125")
            .about("Enables users to create and manage connections to Google Cloud services and third-party business applications using the Connectors interface.")
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
            .about("sub-resources: connections");
        let mut connections2 = SubCommand::with_name("connections")
            .setting(AppSettings::ColoredHelp)
            .about("methods: execute_sql_query");
        {
            let mcmd = SubCommand::with_name("execute_sql_query").about("Executes a SQL statement specified in the body of the request. An example of this SQL statement in the case of Salesforce connector would be 'select * from Account a, Order o where a.Id = o.AccountId'.");
            connections2 = connections2.subcommand(mcmd);
        }
        let mut actions3 = SubCommand::with_name("actions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: execute and list");
        {
            let mcmd = SubCommand::with_name("execute").about("Executes an action with the name specified in the request. The input parameters for executing the action are passed through the body of the ExecuteAction request.");
            actions3 = actions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Gets the schema of all the actions supported by the connector.");
            actions3 = actions3.subcommand(mcmd);
        }
        let mut entity_types3 = SubCommand::with_name("entity_types")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists metadata related to all entity types present in the external system.",
            );
            entity_types3 = entity_types3.subcommand(mcmd);
        }
        let mut entities4 = SubCommand::with_name("entities")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, delete_entities_with_conditions, get, list, patch and update_entities_with_conditions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new entity row of the specified entity type in the external system. The field values for creating the row are contained in the body of the request. The response message contains a `Entity` message object returned as a response by the external system.");
            entities4 = entities4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an existing entity row matching the entity type and entity id specified in the request.");
            entities4 = entities4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete_entities_with_conditions").about("Deletes entities based on conditions specified in the request and not on entity id.");
            entities4 = entities4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a single entity row matching the entity type and entity id specified in the request.");
            entities4 = entities4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists entity rows of a particular entity type contained in the request. Note: 1. Currently, only max of one 'sort_by' column is supported. 2. If no 'sort_by' column is provided, the primary key of the table is used. If zero or more than one primary key is available, we default to the unpaginated list entities logic which only returns the first page. 3. The values of the 'sort_by' columns must uniquely identify an entity row, otherwise undefined behaviors may be observed during pagination. 4. Since transactions are not supported, any updates, inserts or deletes during pagination can lead to stale data being returned or other unexpected behaviors.");
            entities4 = entities4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing entity row matching the entity type and entity id specified in the request. The fields in the entity row that need to be modified are contained in the body of the request. All unspecified fields are left unchanged. The response message contains a `Entity` message object returned as a response by the external system.");
            entities4 = entities4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_entities_with_conditions").about("Updates entities based on conditions specified in the request and not on entity id.");
            entities4 = entities4.subcommand(mcmd);
        }
        entity_types3 = entity_types3.subcommand(entities4);
        connections2 = connections2.subcommand(entity_types3);
        connections2 = connections2.subcommand(actions3);
        locations1 = locations1.subcommand(connections2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_connectors2 as api;

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
