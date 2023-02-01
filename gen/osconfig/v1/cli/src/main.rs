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
        let mut app = App::new("osconfig1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230129")
            .about("OS management tools that can be used for patch management, patch compliance, and configuration management on VM instances.")
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
            .about("sub-resources: locations, patch_deployments and patch_jobs");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: instances and os_policy_assignments");
        let mut patch_deployments1 = SubCommand::with_name("patch_deployments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch, pause and resume");
        {
            let mcmd =
                SubCommand::with_name("create").about("Create an OS Config patch deployment.");
            patch_deployments1 = patch_deployments1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Delete an OS Config patch deployment.");
            patch_deployments1 = patch_deployments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get an OS Config patch deployment.");
            patch_deployments1 = patch_deployments1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Get a page of OS Config patch deployments.");
            patch_deployments1 = patch_deployments1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Update an OS Config patch deployment.");
            patch_deployments1 = patch_deployments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("pause").about("Change state of patch deployment to \"PAUSED\". Patch deployment in paused state doesn't generate patch jobs.");
            patch_deployments1 = patch_deployments1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("resume").about("Change state of patch deployment back to \"ACTIVE\". Patch deployment in active state continues to generate patch jobs.");
            patch_deployments1 = patch_deployments1.subcommand(mcmd);
        }
        let mut patch_jobs1 = SubCommand::with_name("patch_jobs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, execute, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Cancel a patch job. The patch job must be active. Canceled patch jobs cannot be restarted.");
            patch_jobs1 = patch_jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("execute")
                .about("Patch VM instances by creating and running a patch job.");
            patch_jobs1 = patch_jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get the patch job. This can be used to track the progress of an ongoing patch job or review the details of completed jobs.");
            patch_jobs1 = patch_jobs1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Get a list of patch jobs.");
            patch_jobs1 = patch_jobs1.subcommand(mcmd);
        }
        let mut instances2 = SubCommand::with_name("instances")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: inventories, os_policy_assignments and vulnerability_reports");
        let mut os_policy_assignments2 = SubCommand::with_name("os_policy_assignments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, list_revisions and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Create an OS policy assignment. This method also creates the first revision of the OS policy assignment. This method returns a long running operation (LRO) that contains the rollout details. The rollout can be cancelled by cancelling the LRO. For more information, see [Method: projects.locations.osPolicyAssignments.operations.cancel](https://cloud.google.com/compute/docs/osconfig/rest/v1/projects.locations.osPolicyAssignments.operations/cancel).");
            os_policy_assignments2 = os_policy_assignments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete the OS policy assignment. This method creates a new revision of the OS policy assignment. This method returns a long running operation (LRO) that contains the rollout details. The rollout can be cancelled by cancelling the LRO. If the LRO completes and is not cancelled, all revisions associated with the OS policy assignment are deleted. For more information, see [Method: projects.locations.osPolicyAssignments.operations.cancel](https://cloud.google.com/compute/docs/osconfig/rest/v1/projects.locations.osPolicyAssignments.operations/cancel).");
            os_policy_assignments2 = os_policy_assignments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieve an existing OS policy assignment. This method always returns the latest revision. In order to retrieve a previous revision of the assignment, also provide the revision ID in the `name` parameter.");
            os_policy_assignments2 = os_policy_assignments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List the OS policy assignments under the parent resource. For each OS policy assignment, the latest revision is returned.");
            os_policy_assignments2 = os_policy_assignments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_revisions")
                .about("List the OS policy assignment revisions for a given OS policy assignment.");
            os_policy_assignments2 = os_policy_assignments2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update an existing OS policy assignment. This method creates a new revision of the OS policy assignment. This method returns a long running operation (LRO) that contains the rollout details. The rollout can be cancelled by cancelling the LRO. For more information, see [Method: projects.locations.osPolicyAssignments.operations.cancel](https://cloud.google.com/compute/docs/osconfig/rest/v1/projects.locations.osPolicyAssignments.operations/cancel).");
            os_policy_assignments2 = os_policy_assignments2.subcommand(mcmd);
        }
        let mut instance_details2 = SubCommand::with_name("instance_details")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Get a list of instance details for a given patch job.");
            instance_details2 = instance_details2.subcommand(mcmd);
        }
        let mut inventories3 = SubCommand::with_name("inventories")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Get inventory data for the specified VM instance. If the VM has no associated inventory, the message `NOT_FOUND` is returned.");
            inventories3 = inventories3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List inventory data for all VM instances in the specified zone.");
            inventories3 = inventories3.subcommand(mcmd);
        }
        let mut os_policy_assignments3 = SubCommand::with_name("os_policy_assignments")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: reports");
        let mut vulnerability_reports3 = SubCommand::with_name("vulnerability_reports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the vulnerability report for the specified VM instance. Only VMs with inventory data have vulnerability reports associated with them.");
            vulnerability_reports3 = vulnerability_reports3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List vulnerability reports for all VM instances in the specified zone.");
            vulnerability_reports3 = vulnerability_reports3.subcommand(mcmd);
        }
        let mut operations3 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel and get");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
            operations3 = operations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations3 = operations3.subcommand(mcmd);
        }
        let mut reports4 = SubCommand::with_name("reports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Get the OS policy asssignment report for the specified Compute Engine VM instance.");
            reports4 = reports4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List OS policy asssignment reports for all Compute Engine VM instances in the specified zone.");
            reports4 = reports4.subcommand(mcmd);
        }
        os_policy_assignments3 = os_policy_assignments3.subcommand(reports4);
        os_policy_assignments2 = os_policy_assignments2.subcommand(operations3);
        instances2 = instances2.subcommand(vulnerability_reports3);
        instances2 = instances2.subcommand(os_policy_assignments3);
        instances2 = instances2.subcommand(inventories3);
        patch_jobs1 = patch_jobs1.subcommand(instance_details2);
        locations1 = locations1.subcommand(os_policy_assignments2);
        locations1 = locations1.subcommand(instances2);
        projects0 = projects0.subcommand(patch_jobs1);
        projects0 = projects0.subcommand(patch_deployments1);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_osconfig1 as api;

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
