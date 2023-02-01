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
        let mut app = App::new("baremetalsolution2")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230113")
            .about("Provides ways to manage Bare Metal Solution hardware installed in a regional extension located near a Google Cloud data center.")
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
        let mut instance_provisioning_settings2 =
            SubCommand::with_name("instance_provisioning_settings")
                .setting(AppSettings::ColoredHelp)
                .about("methods: fetch");
        {
            let mcmd = SubCommand::with_name("fetch").about("Get instance provisioning settings for a given project. This is hidden method used by UI only.");
            instance_provisioning_settings2 = instance_provisioning_settings2.subcommand(mcmd);
        }
        let mut instances2 = SubCommand::with_name("instances")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, detach_lun, disable_interactive_serial_console, enable_interactive_serial_console, get, list, patch, reset, start and stop");
        {
            let mcmd = SubCommand::with_name("create").about("Create an Instance.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("detach_lun").about("Detach LUN from Instance.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("disable_interactive_serial_console")
                .about("Disable the interactive serial console feature on an instance.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("enable_interactive_serial_console")
                .about("Enable the interactive serial console feature on an instance.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get details about a single server.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List servers in a given project and location.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update details of a single server.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reset").about("Perform an ungraceful, hard reset on a server. Equivalent to shutting the power off and then turning it back on.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("start").about("Starts a server that was shutdown.");
            instances2 = instances2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("stop").about("Stop a running server.");
            instances2 = instances2.subcommand(mcmd);
        }
        let mut networks2 = SubCommand::with_name("networks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list, list_network_usage and patch");
        {
            let mcmd = SubCommand::with_name("get").about("Get details of a single network.");
            networks2 = networks2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List network in a given project and location.");
            networks2 = networks2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_network_usage").about("List all Networks (and used IPs for each Network) in the vendor account associated with the specified project.");
            networks2 = networks2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update details of a single network.");
            networks2 = networks2.subcommand(mcmd);
        }
        let mut nfs_shares2 = SubCommand::with_name("nfs_shares")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Create an NFS share.");
            nfs_shares2 = nfs_shares2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Delete an NFS share. The underlying volume is automatically deleted.");
            nfs_shares2 = nfs_shares2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get details of a single NFS share.");
            nfs_shares2 = nfs_shares2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List NFS shares.");
            nfs_shares2 = nfs_shares2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Update details of a single NFS share.");
            nfs_shares2 = nfs_shares2.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Get details about an operation. This method used only to work around CCFE lack of passthrough LRO support (b/221498758).");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut provisioning_configs2 = SubCommand::with_name("provisioning_configs")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get, patch and submit");
        {
            let mcmd = SubCommand::with_name("create").about("Create new ProvisioningConfig.");
            provisioning_configs2 = provisioning_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get ProvisioningConfig by name.");
            provisioning_configs2 = provisioning_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update existing ProvisioningConfig.");
            provisioning_configs2 = provisioning_configs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("submit")
                .about("Submit a provisiong configuration for a given project.");
            provisioning_configs2 = provisioning_configs2.subcommand(mcmd);
        }
        let mut provisioning_quotas2 = SubCommand::with_name("provisioning_quotas")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("List the budget details to provision resources on a given project.");
            provisioning_quotas2 = provisioning_quotas2.subcommand(mcmd);
        }
        let mut ssh_keys2 = SubCommand::with_name("ssh_keys")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete and list");
        {
            let mcmd = SubCommand::with_name("create").about("Register a public SSH key in the specified project for use with the interactive serial console feature.");
            ssh_keys2 = ssh_keys2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("Deletes a public SSH key registered in the specified project.");
            ssh_keys2 = ssh_keys2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the public SSH keys registered for the specified project. These SSH keys are used only for the interactive serial console feature.");
            ssh_keys2 = ssh_keys2.subcommand(mcmd);
        }
        let mut volumes2 = SubCommand::with_name("volumes")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list, patch and resize");
        {
            let mcmd =
                SubCommand::with_name("get").about("Get details of a single storage volume.");
            volumes2 = volumes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List storage volumes in a given project and location.");
            volumes2 = volumes2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Update details of a single storage volume.");
            volumes2 = volumes2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("resize").about("Emergency Volume resize.");
            volumes2 = volumes2.subcommand(mcmd);
        }
        let mut luns3 = SubCommand::with_name("luns")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get")
                .about("Get details of a single storage logical unit number(LUN).");
            luns3 = luns3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List storage volume luns for given storage volume.");
            luns3 = luns3.subcommand(mcmd);
        }
        let mut snapshots3 = SubCommand::with_name("snapshots")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and restore_volume_snapshot");
        {
            let mcmd = SubCommand::with_name("create").about("Takes a snapshot of a boot volume. Returns INVALID_ARGUMENT if called for a non-boot volume.");
            snapshots3 = snapshots3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a volume snapshot. Returns INVALID_ARGUMENT if called for a non-boot volume.");
            snapshots3 = snapshots3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the specified snapshot resource. Returns INVALID_ARGUMENT if called for a non-boot volume.");
            snapshots3 = snapshots3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves the list of snapshots for the specified volume. Returns a response with an empty list of snapshots if called for a non-boot volume.");
            snapshots3 = snapshots3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("restore_volume_snapshot").about("Uses the specified snapshot to restore its parent volume. Returns INVALID_ARGUMENT if called for a non-boot volume.");
            snapshots3 = snapshots3.subcommand(mcmd);
        }
        volumes2 = volumes2.subcommand(snapshots3);
        volumes2 = volumes2.subcommand(luns3);
        locations1 = locations1.subcommand(volumes2);
        locations1 = locations1.subcommand(ssh_keys2);
        locations1 = locations1.subcommand(provisioning_quotas2);
        locations1 = locations1.subcommand(provisioning_configs2);
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(nfs_shares2);
        locations1 = locations1.subcommand(networks2);
        locations1 = locations1.subcommand(instances2);
        locations1 = locations1.subcommand(instance_provisioning_settings2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_baremetalsolution2 as api;

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
