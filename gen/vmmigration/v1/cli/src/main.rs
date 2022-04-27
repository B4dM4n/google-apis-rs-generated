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
        let mut app = App::new("vmmigration1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20220408")
            .about("Use the Migrate for Compute Engine API to programmatically migrate workloads. ")
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
        let mut groups2 = SubCommand::with_name("groups")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: add_group_migration, create, delete, get, list, patch and remove_group_migration");
        {
            let mcmd = SubCommand::with_name("add_group_migration")
                .about("Adds a MigratingVm to a Group.");
            groups2 = groups2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new Group in a given project and location.");
            groups2 = groups2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a single Group.");
            groups2 = groups2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets details of a single Group.");
            groups2 = groups2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists Groups in a given project and location.");
            groups2 = groups2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates the parameters of a single Group.");
            groups2 = groups2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_group_migration")
                .about("Removes a MigratingVm from a Group.");
            groups2 = groups2.subcommand(mcmd);
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
        let mut sources2 = SubCommand::with_name("sources")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, fetch_inventory, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new Source in a given project and location.");
            sources2 = sources2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a single Source.");
            sources2 = sources2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("fetch_inventory").about("List remote source's inventory of VMs. The remote source is the onprem vCenter (remote in the sense it's not in Compute Engine). The inventory describes the list of existing VMs in that source. Note that this operation lists the VMs on the remote source, as opposed to listing the MigratingVms resources in the vmmigration service.");
            sources2 = sources2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets details of a single Source.");
            sources2 = sources2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists Sources in a given project and location.");
            sources2 = sources2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates the parameters of a single Source.");
            sources2 = sources2.subcommand(mcmd);
        }
        let mut target_projects2 = SubCommand::with_name("target_projects")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new TargetProject in a given project. NOTE: TargetProject is a global resource; hence the only supported value for location is `global`.");
            target_projects2 = target_projects2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a single TargetProject. NOTE: TargetProject is a global resource; hence the only supported value for location is `global`.");
            target_projects2 = target_projects2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets details of a single TargetProject. NOTE: TargetProject is a global resource; hence the only supported value for location is `global`.");
            target_projects2 = target_projects2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists TargetProjects in a given project. NOTE: TargetProject is a global resource; hence the only supported value for location is `global`.");
            target_projects2 = target_projects2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the parameters of a single TargetProject. NOTE: TargetProject is a global resource; hence the only supported value for location is `global`.");
            target_projects2 = target_projects2.subcommand(mcmd);
        }
        let mut datacenter_connectors3 = SubCommand::with_name("datacenter_connectors")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and upgrade_appliance");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new DatacenterConnector in a given Source.");
            datacenter_connectors3 = datacenter_connectors3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes a single DatacenterConnector.");
            datacenter_connectors3 = datacenter_connectors3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets details of a single DatacenterConnector.");
            datacenter_connectors3 = datacenter_connectors3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists DatacenterConnectors in a given Source.");
            datacenter_connectors3 = datacenter_connectors3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("upgrade_appliance").about("Upgrades the appliance relate to this DatacenterConnector to the in-place updateable version.");
            datacenter_connectors3 = datacenter_connectors3.subcommand(mcmd);
        }
        let mut migrating_vms3 = SubCommand::with_name("migrating_vms")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, finalize_migration, get, list, patch, pause_migration, resume_migration and start_migration");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates a new MigratingVm in a given Source.");
            migrating_vms3 = migrating_vms3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a single MigratingVm.");
            migrating_vms3 = migrating_vms3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("finalize_migration").about("Marks a migration as completed, deleting migration resources that are no longer being used. Only applicable after cutover is done.");
            migrating_vms3 = migrating_vms3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets details of a single MigratingVm.");
            migrating_vms3 = migrating_vms3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists MigratingVms in a given Source.");
            migrating_vms3 = migrating_vms3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates the parameters of a single MigratingVm.");
            migrating_vms3 = migrating_vms3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("pause_migration").about("Pauses a migration for a VM. If cycle tasks are running they will be cancelled, preserving source task data. Further replication cycles will not be triggered while the VM is paused.");
            migrating_vms3 = migrating_vms3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("resume_migration").about("Resumes a migration for a VM. When called on a paused migration, will start the process of uploading data and creating snapshots; when called on a completed cut-over migration, will update the migration to active state and start the process of uploading data and creating snapshots.");
            migrating_vms3 = migrating_vms3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("start_migration").about("Starts migration for a VM. Starts the process of uploading data and creating snapshots, in replication cycles scheduled by the policy.");
            migrating_vms3 = migrating_vms3.subcommand(mcmd);
        }
        let mut utilization_reports3 = SubCommand::with_name("utilization_reports")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new UtilizationReport.");
            utilization_reports3 = utilization_reports3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("delete").about("Deletes a single Utilization Report.");
            utilization_reports3 = utilization_reports3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a single Utilization Report.");
            utilization_reports3 = utilization_reports3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists Utilization Reports of the given Source.");
            utilization_reports3 = utilization_reports3.subcommand(mcmd);
        }
        let mut clone_jobs4 = SubCommand::with_name("clone_jobs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, create, get and list");
        {
            let mcmd = SubCommand::with_name("cancel")
                .about("Initiates the cancellation of a running clone job.");
            clone_jobs4 = clone_jobs4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create")
                .about("Initiates a Clone of a specific migrating VM.");
            clone_jobs4 = clone_jobs4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets details of a single CloneJob.");
            clone_jobs4 = clone_jobs4.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists CloneJobs of a given migrating VM.");
            clone_jobs4 = clone_jobs4.subcommand(mcmd);
        }
        let mut cutover_jobs4 = SubCommand::with_name("cutover_jobs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, create, get and list");
        {
            let mcmd = SubCommand::with_name("cancel")
                .about("Initiates the cancellation of a running cutover job.");
            cutover_jobs4 = cutover_jobs4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Initiates a Cutover of a specific migrating VM. The returned LRO is completed when the cutover job resource is created and the job is initiated.");
            cutover_jobs4 = cutover_jobs4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets details of a single CutoverJob.");
            cutover_jobs4 = cutover_jobs4.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists CutoverJobs of a given migrating VM.");
            cutover_jobs4 = cutover_jobs4.subcommand(mcmd);
        }
        migrating_vms3 = migrating_vms3.subcommand(cutover_jobs4);
        migrating_vms3 = migrating_vms3.subcommand(clone_jobs4);
        sources2 = sources2.subcommand(utilization_reports3);
        sources2 = sources2.subcommand(migrating_vms3);
        sources2 = sources2.subcommand(datacenter_connectors3);
        locations1 = locations1.subcommand(target_projects2);
        locations1 = locations1.subcommand(sources2);
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(groups2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_vmmigration1 as api;

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
