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
        let mut app = App::new("apigeeregistry1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230127")
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
        let mut apis2 = SubCommand::with_name("apis")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a specified API.");
            apis2 = apis2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Removes a specified API and all of the resources that it owns.");
            apis2 = apis2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a specified API.");
            apis2 = apis2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            apis2 = apis2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns matching APIs.");
            apis2 = apis2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Used to modify a specified API.");
            apis2 = apis2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            apis2 = apis2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            apis2 = apis2.subcommand(mcmd);
        }
        let mut artifacts2 = SubCommand::with_name("artifacts")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_contents, get_iam_policy, list, replace_artifact, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a specified artifact.");
            artifacts2 = artifacts2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Removes a specified artifact.");
            artifacts2 = artifacts2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a specified artifact.");
            artifacts2 = artifacts2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_contents").about("Returns the contents of a specified artifact. If artifacts are stored with GZip compression, the default behavior is to return the artifact uncompressed (the mime_type response field indicates the exact format returned).");
            artifacts2 = artifacts2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            artifacts2 = artifacts2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns matching artifacts.");
            artifacts2 = artifacts2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("replace_artifact")
                .about("Used to replace a specified artifact.");
            artifacts2 = artifacts2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            artifacts2 = artifacts2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            artifacts2 = artifacts2.subcommand(mcmd);
        }
        let mut instances2 = SubCommand::with_name("instances")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Provisions instance resources for the Registry.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the Registry instance.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets details of a single Instance.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            instances2 = instances2.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, delete, get and list");
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
        let mut runtime2 = SubCommand::with_name("runtime")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_iam_policy, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            runtime2 = runtime2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            runtime2 = runtime2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            runtime2 = runtime2.subcommand(mcmd);
        }
        let mut artifacts3 = SubCommand::with_name("artifacts")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_contents, get_iam_policy, list, replace_artifact, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a specified artifact.");
            artifacts3 = artifacts3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Removes a specified artifact.");
            artifacts3 = artifacts3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a specified artifact.");
            artifacts3 = artifacts3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_contents").about("Returns the contents of a specified artifact. If artifacts are stored with GZip compression, the default behavior is to return the artifact uncompressed (the mime_type response field indicates the exact format returned).");
            artifacts3 = artifacts3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            artifacts3 = artifacts3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns matching artifacts.");
            artifacts3 = artifacts3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("replace_artifact")
                .about("Used to replace a specified artifact.");
            artifacts3 = artifacts3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            artifacts3 = artifacts3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            artifacts3 = artifacts3.subcommand(mcmd);
        }
        let mut deployments3 = SubCommand::with_name("deployments")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, delete_revision, get, get_iam_policy, list, list_revisions, patch, rollback, set_iam_policy, tag_revision and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a specified deployment.");
            deployments3 = deployments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Removes a specified deployment, all revisions, and all child resources (e.g., artifacts).");
            deployments3 = deployments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete_revision")
                .about("Deletes a revision of a deployment.");
            deployments3 = deployments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a specified deployment.");
            deployments3 = deployments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            deployments3 = deployments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns matching deployments.");
            deployments3 = deployments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_revisions").about("Lists all revisions of a deployment. Revisions are returned in descending order of revision creation time.");
            deployments3 = deployments3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Used to modify a specified deployment.");
            deployments3 = deployments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("rollback").about("Sets the current revision to a specified prior revision. Note that this creates a new revision with a new revision ID.");
            deployments3 = deployments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            deployments3 = deployments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("tag_revision")
                .about("Adds a tag to a specified revision of a deployment.");
            deployments3 = deployments3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            deployments3 = deployments3.subcommand(mcmd);
        }
        let mut versions3 = SubCommand::with_name("versions")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a specified version.");
            versions3 = versions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Removes a specified version and all of the resources that it owns.");
            versions3 = versions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a specified version.");
            versions3 = versions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            versions3 = versions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns matching versions.");
            versions3 = versions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Used to modify a specified version.");
            versions3 = versions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            versions3 = versions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            versions3 = versions3.subcommand(mcmd);
        }
        let mut artifacts4 = SubCommand::with_name("artifacts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, get_contents, list and replace_artifact");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a specified artifact.");
            artifacts4 = artifacts4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Removes a specified artifact.");
            artifacts4 = artifacts4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a specified artifact.");
            artifacts4 = artifacts4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_contents").about("Returns the contents of a specified artifact. If artifacts are stored with GZip compression, the default behavior is to return the artifact uncompressed (the mime_type response field indicates the exact format returned).");
            artifacts4 = artifacts4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns matching artifacts.");
            artifacts4 = artifacts4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("replace_artifact")
                .about("Used to replace a specified artifact.");
            artifacts4 = artifacts4.subcommand(mcmd);
        }
        let mut artifacts4 = SubCommand::with_name("artifacts")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_contents, get_iam_policy, list, replace_artifact, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a specified artifact.");
            artifacts4 = artifacts4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Removes a specified artifact.");
            artifacts4 = artifacts4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a specified artifact.");
            artifacts4 = artifacts4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_contents").about("Returns the contents of a specified artifact. If artifacts are stored with GZip compression, the default behavior is to return the artifact uncompressed (the mime_type response field indicates the exact format returned).");
            artifacts4 = artifacts4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            artifacts4 = artifacts4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns matching artifacts.");
            artifacts4 = artifacts4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("replace_artifact")
                .about("Used to replace a specified artifact.");
            artifacts4 = artifacts4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            artifacts4 = artifacts4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            artifacts4 = artifacts4.subcommand(mcmd);
        }
        let mut specs4 = SubCommand::with_name("specs")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, delete_revision, get, get_contents, get_iam_policy, list, list_revisions, patch, rollback, set_iam_policy, tag_revision and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a specified spec.");
            specs4 = specs4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Removes a specified spec, all revisions, and all child resources (e.g., artifacts).");
            specs4 = specs4.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete_revision").about("Deletes a revision of a spec.");
            specs4 = specs4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a specified spec.");
            specs4 = specs4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_contents").about("Returns the contents of a specified spec. If specs are stored with GZip compression, the default behavior is to return the spec uncompressed (the mime_type response field indicates the exact format returned).");
            specs4 = specs4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            specs4 = specs4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns matching specs.");
            specs4 = specs4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_revisions").about("Lists all revisions of a spec. Revisions are returned in descending order of revision creation time.");
            specs4 = specs4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Used to modify a specified spec.");
            specs4 = specs4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("rollback").about("Sets the current revision to a specified prior revision. Note that this creates a new revision with a new revision ID.");
            specs4 = specs4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            specs4 = specs4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("tag_revision")
                .about("Adds a tag to a specified revision of a spec.");
            specs4 = specs4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            specs4 = specs4.subcommand(mcmd);
        }
        let mut artifacts5 = SubCommand::with_name("artifacts")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_contents, get_iam_policy, list, replace_artifact, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a specified artifact.");
            artifacts5 = artifacts5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Removes a specified artifact.");
            artifacts5 = artifacts5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a specified artifact.");
            artifacts5 = artifacts5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_contents").about("Returns the contents of a specified artifact. If artifacts are stored with GZip compression, the default behavior is to return the artifact uncompressed (the mime_type response field indicates the exact format returned).");
            artifacts5 = artifacts5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            artifacts5 = artifacts5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Returns matching artifacts.");
            artifacts5 = artifacts5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("replace_artifact")
                .about("Used to replace a specified artifact.");
            artifacts5 = artifacts5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            artifacts5 = artifacts5.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            artifacts5 = artifacts5.subcommand(mcmd);
        }
        specs4 = specs4.subcommand(artifacts5);
        versions3 = versions3.subcommand(specs4);
        versions3 = versions3.subcommand(artifacts4);
        deployments3 = deployments3.subcommand(artifacts4);
        apis2 = apis2.subcommand(versions3);
        apis2 = apis2.subcommand(deployments3);
        apis2 = apis2.subcommand(artifacts3);
        locations1 = locations1.subcommand(runtime2);
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(instances2);
        locations1 = locations1.subcommand(artifacts2);
        locations1 = locations1.subcommand(apis2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_apigeeregistry1 as api;

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
