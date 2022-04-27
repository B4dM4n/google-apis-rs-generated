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
        let mut app = App::new("vault1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20220413")
            .about("Retention and eDiscovery for Google Workspace. To work with Vault resources, the account must have the [required Vault privileges](https://support.google.com/vault/answer/2799699) and access to the matter. To access a matter, the account must have created the matter, have the matter shared with them, or have the **View All Matters** privilege. For example, to download an export, an account needs the **Manage Exports** privilege and the matter shared with them. ")
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
        let mut matters0 = SubCommand::with_name("matters")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: add_permissions, close, count, create, delete, get, list, remove_permissions, reopen, undelete and update");
        {
            let mcmd = SubCommand::with_name("add_permissions")
                .about("Adds an account as a matter collaborator.");
            matters0 = matters0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("close")
                .about("Closes the specified matter. Returns the matter with updated state.");
            matters0 = matters0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("count")
                .about("Counts the accounts processed by the specified query.");
            matters0 = matters0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a matter with the given name and description. The initial state is open, and the owner is the method caller. Returns the created matter with default view.");
            matters0 = matters0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes the specified matter. Returns the matter with updated state.");
            matters0 = matters0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified matter.");
            matters0 = matters0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists matters the requestor has access to.");
            matters0 = matters0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_permissions")
                .about("Removes an account as a matter collaborator.");
            matters0 = matters0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reopen")
                .about("Reopens the specified matter. Returns the matter with updated state.");
            matters0 = matters0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undelete")
                .about("Undeletes the specified matter. Returns the matter with updated state.");
            matters0 = matters0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the specified matter. This updates only the name and description of the matter, identified by matter ID. Changes to any other fields are ignored. Returns the default view of the matter.");
            matters0 = matters0.subcommand(mcmd);
        }
        let mut operations0 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, delete, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations0 = operations0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations0 = operations0.subcommand(mcmd);
        }
        let mut exports1 = SubCommand::with_name("exports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an export.");
            exports1 = exports1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an export.");
            exports1 = exports1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an export.");
            exports1 = exports1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists details about the exports in the specified matter.");
            exports1 = exports1.subcommand(mcmd);
        }
        let mut holds1 = SubCommand::with_name("holds")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: add_held_accounts, create, delete, get, list, remove_held_accounts and update");
        {
            let mcmd = SubCommand::with_name("add_held_accounts").about("Adds accounts to a hold. Returns a list of accounts that have been successfully added. Accounts can be added only to an existing account-based hold.");
            holds1 = holds1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a hold in the specified matter.");
            holds1 = holds1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Removes the specified hold and releases the accounts or organizational unit covered by the hold. If the data is not preserved by another hold or retention rule, it might be purged.");
            holds1 = holds1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the specified hold.");
            holds1 = holds1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the holds in a matter.");
            holds1 = holds1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_held_accounts").about("Removes the specified accounts from a hold. Returns a list of statuses in the same order as the request.");
            holds1 = holds1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates the scope (organizational unit or accounts) and query parameters of a hold. You cannot add accounts to a hold that covers an organizational unit, nor can you add organizational units to a hold that covers individual accounts. If you try, the unsupported values are ignored.");
            holds1 = holds1.subcommand(mcmd);
        }
        let mut saved_queries1 = SubCommand::with_name("saved_queries")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a saved query.");
            saved_queries1 = saved_queries1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified saved query.");
            saved_queries1 = saved_queries1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified saved query.");
            saved_queries1 = saved_queries1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the saved queries in a matter.");
            saved_queries1 = saved_queries1.subcommand(mcmd);
        }
        let mut accounts2 = SubCommand::with_name("accounts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete and list");
        {
            let mcmd = SubCommand::with_name("create").about("Adds an account to a hold. Accounts can be added only to a hold that does not have an organizational unit set. If you try to add an account to an organizational unit-based hold, an error is returned.");
            accounts2 = accounts2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Removes an account from a hold.");
            accounts2 = accounts2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the accounts covered by a hold. This can list only individually-specified accounts covered by the hold. If the hold covers an organizational unit, use the [Admin SDK](https://developers.google.com/admin-sdk/). to list the members of the organizational unit on hold.");
            accounts2 = accounts2.subcommand(mcmd);
        }
        holds1 = holds1.subcommand(accounts2);
        matters0 = matters0.subcommand(saved_queries1);
        matters0 = matters0.subcommand(holds1);
        matters0 = matters0.subcommand(exports1);
        app = app.subcommand(operations0);
        app = app.subcommand(matters0);

        Self { app }
    }
}
use google_vault1 as api;

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
