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
        let mut app = App::new("androidmanagement1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230123")
            .about("The Android Management API provides remote enterprise management of Android devices and apps.")
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
        let mut enterprises0 = SubCommand::with_name("enterprises")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about(
                "Creates an enterprise. This is the last step in the enterprise signup flow.",
            );
            enterprises0 = enterprises0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes an enterprise. Only available for EMM-managed enterprises.");
            enterprises0 = enterprises0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an enterprise.");
            enterprises0 = enterprises0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists EMM-managed enterprises. Only BASIC fields are returned.");
            enterprises0 = enterprises0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an enterprise.");
            enterprises0 = enterprises0.subcommand(mcmd);
        }
        let mut signup_urls0 = SubCommand::with_name("signup_urls")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an enterprise signup URL.");
            signup_urls0 = signup_urls0.subcommand(mcmd);
        }
        let mut applications1 = SubCommand::with_name("applications")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets info about an application.");
            applications1 = applications1.subcommand(mcmd);
        }
        let mut devices1 = SubCommand::with_name("devices")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, issue_command, list and patch");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a device. This operation wipes the device. Deleted devices do not show up in enterprises.devices.list calls and a 404 is returned from enterprises.devices.get.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets a device. Deleted devices will respond with a 404 error.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("issue_command").about("Issues a command to a device. The Operation resource returned contains a Command in its metadata field. Use the get operation method to get the status of the command.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists devices for a given enterprise. Deleted devices are not returned in the response.");
            devices1 = devices1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a device.");
            devices1 = devices1.subcommand(mcmd);
        }
        let mut enrollment_tokens1 = SubCommand::with_name("enrollment_tokens")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an enrollment token for a given enterprise. It's up to the caller's responsibility to manage the lifecycle of newly created tokens and deleting them when they're not intended to be used anymore. Once an enrollment token has been created, it's not possible to retrieve the token's content anymore using AM API. It is recommended for EMMs to securely store the token if it's intended to be reused.");
            enrollment_tokens1 = enrollment_tokens1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an enrollment token. This operation invalidates the token, preventing its future use.");
            enrollment_tokens1 = enrollment_tokens1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an active, unexpired enrollment token. Only a partial view of EnrollmentToken is returned: all the fields but name and expiration_timestamp are empty. This method is meant to help manage active enrollment tokens lifecycle. For security reasons, it's recommended to delete active enrollment tokens as soon as they're not intended to be used anymore.");
            enrollment_tokens1 = enrollment_tokens1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists active, unexpired enrollment tokens for a given enterprise. The list items contain only a partial view of EnrollmentToken: all the fields but name and expiration_timestamp are empty. This method is meant to help manage active enrollment tokens lifecycle. For security reasons, it's recommended to delete active enrollment tokens as soon as they're not intended to be used anymore.");
            enrollment_tokens1 = enrollment_tokens1.subcommand(mcmd);
        }
        let mut policies1 = SubCommand::with_name("policies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a policy. This operation is only permitted if no devices are currently referencing the policy.");
            policies1 = policies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a policy.");
            policies1 = policies1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists policies for a given enterprise.");
            policies1 = policies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates or creates a policy.");
            policies1 = policies1.subcommand(mcmd);
        }
        let mut web_apps1 = SubCommand::with_name("web_apps")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a web app.");
            web_apps1 = web_apps1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a web app.");
            web_apps1 = web_apps1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a web app.");
            web_apps1 = web_apps1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists web apps for a given enterprise.");
            web_apps1 = web_apps1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a web app.");
            web_apps1 = web_apps1.subcommand(mcmd);
        }
        let mut web_tokens1 = SubCommand::with_name("web_tokens")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a web token to access an embeddable managed Google Play web UI for a given enterprise.");
            web_tokens1 = web_tokens1.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, delete, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to Code.CANCELLED.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns google.rpc.Code.UNIMPLEMENTED.");
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
        devices1 = devices1.subcommand(operations2);
        enterprises0 = enterprises0.subcommand(web_tokens1);
        enterprises0 = enterprises0.subcommand(web_apps1);
        enterprises0 = enterprises0.subcommand(policies1);
        enterprises0 = enterprises0.subcommand(enrollment_tokens1);
        enterprises0 = enterprises0.subcommand(devices1);
        enterprises0 = enterprises0.subcommand(applications1);
        app = app.subcommand(signup_urls0);
        app = app.subcommand(enterprises0);

        Self { app }
    }
}
use google_androidmanagement1 as api;

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
