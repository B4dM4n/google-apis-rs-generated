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
        let mut app = App::new("osconfig1_beta")
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
            .about("sub-resources: guest_policies, patch_deployments, patch_jobs and zones");
        let mut guest_policies1 = SubCommand::with_name("guest_policies")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Create an OS Config guest policy.");
            guest_policies1 = guest_policies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete an OS Config guest policy.");
            guest_policies1 = guest_policies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get an OS Config guest policy.");
            guest_policies1 = guest_policies1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Get a page of OS Config guest policies.");
            guest_policies1 = guest_policies1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update an OS Config guest policy.");
            guest_policies1 = guest_policies1.subcommand(mcmd);
        }
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
        let mut zones1 = SubCommand::with_name("zones")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: instances");
        let mut instance_details2 = SubCommand::with_name("instance_details")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Get a list of instance details for a given patch job.");
            instance_details2 = instance_details2.subcommand(mcmd);
        }
        let mut instances2 = SubCommand::with_name("instances")
            .setting(AppSettings::ColoredHelp)
            .about("methods: lookup_effective_guest_policy");
        {
            let mcmd = SubCommand::with_name("lookup_effective_guest_policy").about("Lookup the effective guest policy that applies to a VM instance. This lookup merges all policies that are assigned to the instance ancestry.");
            instances2 = instances2.subcommand(mcmd);
        }
        zones1 = zones1.subcommand(instances2);
        patch_jobs1 = patch_jobs1.subcommand(instance_details2);
        projects0 = projects0.subcommand(zones1);
        projects0 = projects0.subcommand(patch_jobs1);
        projects0 = projects0.subcommand(patch_deployments1);
        projects0 = projects0.subcommand(guest_policies1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_osconfig1_beta as api;

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
