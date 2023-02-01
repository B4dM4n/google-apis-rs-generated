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
        let mut app = App::new("dataplex1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230120")
            .about("Dataplex API is used to manage the lifecycle of data lakes.")
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
        let mut data_attribute_bindings2 = SubCommand::with_name("data_attribute_bindings")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd =
                SubCommand::with_name("create").about("Create a DataAttributeBinding resource.");
            data_attribute_bindings2 = data_attribute_bindings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a DataAttributeBinding resource. All attributes within the DataAttributeBinding must be deleted before the DataAttributeBinding can be deleted.");
            data_attribute_bindings2 = data_attribute_bindings2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Retrieves a DataAttributeBinding resource.");
            data_attribute_bindings2 = data_attribute_bindings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            data_attribute_bindings2 = data_attribute_bindings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists DataAttributeBinding resources in a project and location.");
            data_attribute_bindings2 = data_attribute_bindings2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates a DataAttributeBinding resource.");
            data_attribute_bindings2 = data_attribute_bindings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.");
            data_attribute_bindings2 = data_attribute_bindings2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            data_attribute_bindings2 = data_attribute_bindings2.subcommand(mcmd);
        }
        let mut data_scans2 = SubCommand::with_name("data_scans")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, run, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a DataScan resource.");
            data_scans2 = data_scans2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a DataScan resource.");
            data_scans2 = data_scans2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a DataScan resource.");
            data_scans2 = data_scans2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            data_scans2 = data_scans2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists DataScans.");
            data_scans2 = data_scans2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a DataScan resource.");
            data_scans2 = data_scans2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("run").about("Runs an on-demand execution of a DataScan");
            data_scans2 = data_scans2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.");
            data_scans2 = data_scans2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            data_scans2 = data_scans2.subcommand(mcmd);
        }
        let mut data_taxonomies2 = SubCommand::with_name("data_taxonomies")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Create a DataTaxonomy resource.");
            data_taxonomies2 = data_taxonomies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a DataTaxonomy resource. All attributes within the DataTaxonomy must be deleted before the DataTaxonomy can be deleted.");
            data_taxonomies2 = data_taxonomies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a DataTaxonomy resource.");
            data_taxonomies2 = data_taxonomies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            data_taxonomies2 = data_taxonomies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists DataTaxonomy resources in a project and location.");
            data_taxonomies2 = data_taxonomies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a DataTaxonomy resource.");
            data_taxonomies2 = data_taxonomies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.");
            data_taxonomies2 = data_taxonomies2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            data_taxonomies2 = data_taxonomies2.subcommand(mcmd);
        }
        let mut lakes2 = SubCommand::with_name("lakes")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a lake resource.");
            lakes2 = lakes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a lake resource. All zones within the lake must be deleted before the lake can be deleted.");
            lakes2 = lakes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a lake resource.");
            lakes2 = lakes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            lakes2 = lakes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists lake resources in a project and location.");
            lakes2 = lakes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a lake resource.");
            lakes2 = lakes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.");
            lakes2 = lakes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            lakes2 = lakes2.subcommand(mcmd);
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
        let mut jobs3 = SubCommand::with_name("jobs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets a DataScanJob resource.");
            jobs3 = jobs3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists DataScanJobs under the given DataScan.");
            jobs3 = jobs3.subcommand(mcmd);
        }
        let mut attributes3 = SubCommand::with_name("attributes")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Create a DataAttribute resource.");
            attributes3 = attributes3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a Data Attribute resource.");
            attributes3 = attributes3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a Data Attribute resource.");
            attributes3 = attributes3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            attributes3 = attributes3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists Data Attribute resources in a DataTaxonomy.");
            attributes3 = attributes3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a DataAttribute resource.");
            attributes3 = attributes3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.");
            attributes3 = attributes3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            attributes3 = attributes3.subcommand(mcmd);
        }
        let mut actions3 = SubCommand::with_name("actions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists action resources in a lake.");
            actions3 = actions3.subcommand(mcmd);
        }
        let mut content3 = SubCommand::with_name("content")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Create a content.");
            content3 = content3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a content.");
            content3 = content3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get a content resource.");
            content3 = content3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a contentitem resource. A NOT_FOUND error is returned if the resource does not exist. An empty policy is returned if the resource exists but does not have a policy set on it.Caller must have Google IAM dataplex.content.getIamPolicy permission on the resource.");
            content3 = content3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List content.");
            content3 = content3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Update a content. Only supports full resource update.");
            content3 = content3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified contentitem resource. Replaces any existing policy.Caller must have Google IAM dataplex.content.setIamPolicy permission on the resource.");
            content3 = content3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns the caller's permissions on a resource. If the resource does not exist, an empty set of permissions is returned (a NOT_FOUND error is not returned).A caller is not required to have Google IAM permission to make this request.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            content3 = content3.subcommand(mcmd);
        }
        let mut contentitems3 = SubCommand::with_name("contentitems")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Create a content.");
            contentitems3 = contentitems3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a content.");
            contentitems3 = contentitems3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get a content resource.");
            contentitems3 = contentitems3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a contentitem resource. A NOT_FOUND error is returned if the resource does not exist. An empty policy is returned if the resource exists but does not have a policy set on it.Caller must have Google IAM dataplex.content.getIamPolicy permission on the resource.");
            contentitems3 = contentitems3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List content.");
            contentitems3 = contentitems3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Update a content. Only supports full resource update.");
            contentitems3 = contentitems3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified contentitem resource. Replaces any existing policy.Caller must have Google IAM dataplex.content.setIamPolicy permission on the resource.");
            contentitems3 = contentitems3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns the caller's permissions on a resource. If the resource does not exist, an empty set of permissions is returned (a NOT_FOUND error is not returned).A caller is not required to have Google IAM permission to make this request.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            contentitems3 = contentitems3.subcommand(mcmd);
        }
        let mut environments3 = SubCommand::with_name("environments")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Create an environment resource.");
            environments3 = environments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete the environment resource. All the child resources must have been deleted before environment deletion can be initiated.");
            environments3 = environments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get environment resource.");
            environments3 = environments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            environments3 = environments3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists environments under the given lake.");
            environments3 = environments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update the environment resource.");
            environments3 = environments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.");
            environments3 = environments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            environments3 = environments3.subcommand(mcmd);
        }
        let mut tasks3 = SubCommand::with_name("tasks")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, run, set_iam_policy and test_iam_permissions");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a task resource within a lake.");
            tasks3 = tasks3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete the task resource.");
            tasks3 = tasks3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get task resource.");
            tasks3 = tasks3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            tasks3 = tasks3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists tasks under the given lake.");
            tasks3 = tasks3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update the task resource.");
            tasks3 = tasks3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("run").about("Run an on demand execution of a Task.");
            tasks3 = tasks3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.");
            tasks3 = tasks3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            tasks3 = tasks3.subcommand(mcmd);
        }
        let mut zones3 = SubCommand::with_name("zones")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd =
                SubCommand::with_name("create").about("Creates a zone resource within a lake.");
            zones3 = zones3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a zone resource. All assets within a zone must be deleted before the zone can be deleted.");
            zones3 = zones3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a zone resource.");
            zones3 = zones3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            zones3 = zones3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists zone resources in a lake.");
            zones3 = zones3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a zone resource.");
            zones3 = zones3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.");
            zones3 = zones3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            zones3 = zones3.subcommand(mcmd);
        }
        let mut sessions4 = SubCommand::with_name("sessions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists session resources in an environment.");
            sessions4 = sessions4.subcommand(mcmd);
        }
        let mut jobs4 = SubCommand::with_name("jobs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, get and list");
        {
            let mcmd =
                SubCommand::with_name("cancel").about("Cancel jobs running for the task resource.");
            jobs4 = jobs4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get job resource.");
            jobs4 = jobs4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists Jobs under the given task.");
            jobs4 = jobs4.subcommand(mcmd);
        }
        let mut actions4 = SubCommand::with_name("actions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists action resources in a zone.");
            actions4 = actions4.subcommand(mcmd);
        }
        let mut assets4 = SubCommand::with_name("assets")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an asset resource.");
            assets4 = assets4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an asset resource. The referenced storage resource is detached (default) or deleted based on the associated Lifecycle policy.");
            assets4 = assets4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves an asset resource.");
            assets4 = assets4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            assets4 = assets4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists asset resources in a zone.");
            assets4 = assets4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an asset resource.");
            assets4 = assets4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy.Can return NOT_FOUND, INVALID_ARGUMENT, and PERMISSION_DENIED errors.");
            assets4 = assets4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a NOT_FOUND error.Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            assets4 = assets4.subcommand(mcmd);
        }
        let mut entities4 = SubCommand::with_name("entities")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("create").about("Create a metadata entity.");
            entities4 = entities4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a metadata entity.");
            entities4 = entities4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get a metadata entity.");
            entities4 = entities4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List metadata entities in a zone.");
            entities4 = entities4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update")
                .about("Update a metadata entity. Only supports full resource update.");
            entities4 = entities4.subcommand(mcmd);
        }
        let mut actions5 = SubCommand::with_name("actions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists action resources in an asset.");
            actions5 = actions5.subcommand(mcmd);
        }
        let mut partitions5 = SubCommand::with_name("partitions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Create a metadata partition.");
            partitions5 = partitions5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a metadata partition.");
            partitions5 = partitions5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get a metadata partition of an entity.");
            partitions5 = partitions5.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("List metadata partitions of an entity.");
            partitions5 = partitions5.subcommand(mcmd);
        }
        entities4 = entities4.subcommand(partitions5);
        assets4 = assets4.subcommand(actions5);
        zones3 = zones3.subcommand(entities4);
        zones3 = zones3.subcommand(assets4);
        zones3 = zones3.subcommand(actions4);
        tasks3 = tasks3.subcommand(jobs4);
        environments3 = environments3.subcommand(sessions4);
        lakes2 = lakes2.subcommand(zones3);
        lakes2 = lakes2.subcommand(tasks3);
        lakes2 = lakes2.subcommand(environments3);
        lakes2 = lakes2.subcommand(contentitems3);
        lakes2 = lakes2.subcommand(content3);
        lakes2 = lakes2.subcommand(actions3);
        data_taxonomies2 = data_taxonomies2.subcommand(attributes3);
        data_scans2 = data_scans2.subcommand(jobs3);
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(lakes2);
        locations1 = locations1.subcommand(data_taxonomies2);
        locations1 = locations1.subcommand(data_scans2);
        locations1 = locations1.subcommand(data_attribute_bindings2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_dataplex1 as api;

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
