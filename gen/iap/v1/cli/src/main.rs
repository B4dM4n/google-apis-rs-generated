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
        let mut app = App::new("iap1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230127")
            .about("Controls access to cloud applications running on Google Cloud Platform.")
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
            .about("sub-resources: brands and iap_tunnel");
        let mut v_10 = SubCommand::with_name("v_1")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: get_iam_policy, get_iap_settings, set_iam_policy, test_iam_permissions and update_iap_settings");
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for an Identity-Aware Proxy protected resource. More information about managing access via IAP can be found at: https://cloud.google.com/iap/docs/managing-access#managing_access_via_the_api");
            v_10 = v_10.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iap_settings")
                .about("Gets the IAP settings on a particular IAP protected resource.");
            v_10 = v_10.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy for an Identity-Aware Proxy protected resource. Replaces any existing policy. More information about managing access via IAP can be found at: https://cloud.google.com/iap/docs/managing-access#managing_access_via_the_api");
            v_10 = v_10.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the Identity-Aware Proxy protected resource. More information about managing access via IAP can be found at: https://cloud.google.com/iap/docs/managing-access#managing_access_via_the_api");
            v_10 = v_10.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_iap_settings").about("Updates the IAP settings on a particular IAP protected resource. It replaces all fields unless the `update_mask` is set.");
            v_10 = v_10.subcommand(mcmd);
        }
        let mut brands1 = SubCommand::with_name("brands")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Constructs a new OAuth brand for the project if one does not exist. The created brand is \"internal only\", meaning that OAuth clients created under it only accept requests from users who belong to the same Google Workspace organization as the project. The brand is created in an un-reviewed status. NOTE: The \"internal only\" status can be manually changed in the Google Cloud Console. Requires that a brand does not already exist for the project, and that the specified support email is owned by the caller.");
            brands1 = brands1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Retrieves the OAuth brand of the project.");
            brands1 = brands1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists the existing brands for the project.");
            brands1 = brands1.subcommand(mcmd);
        }
        let mut iap_tunnel1 = SubCommand::with_name("iap_tunnel")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut identity_aware_proxy_clients2 =
            SubCommand::with_name("identity_aware_proxy_clients")
                .setting(AppSettings::ColoredHelp)
                .about("methods: create, delete, get, list and reset_secret");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an Identity Aware Proxy (IAP) OAuth client. The client is owned by IAP. Requires that the brand for the project exists and that it is set for internal-only use.");
            identity_aware_proxy_clients2 = identity_aware_proxy_clients2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an Identity Aware Proxy (IAP) OAuth client. Useful for removing obsolete clients, managing the number of clients in a given project, and cleaning up after tests. Requires that the client is owned by IAP.");
            identity_aware_proxy_clients2 = identity_aware_proxy_clients2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves an Identity Aware Proxy (IAP) OAuth client. Requires that the client is owned by IAP.");
            identity_aware_proxy_clients2 = identity_aware_proxy_clients2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists the existing clients for the brand.");
            identity_aware_proxy_clients2 = identity_aware_proxy_clients2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("reset_secret").about("Resets an Identity Aware Proxy (IAP) OAuth client secret. Useful if the secret was compromised. Requires that the client is owned by IAP.");
            identity_aware_proxy_clients2 = identity_aware_proxy_clients2.subcommand(mcmd);
        }
        let mut locations2 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: dest_groups");
        let mut dest_groups3 = SubCommand::with_name("dest_groups")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new TunnelDestGroup.");
            dest_groups3 = dest_groups3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a TunnelDestGroup.");
            dest_groups3 = dest_groups3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves an existing TunnelDestGroup.");
            dest_groups3 = dest_groups3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists the existing TunnelDestGroups. To group across all locations, use a `-` as the location ID. For example: `/v1/projects/123/iap_tunnel/locations/-/destGroups`");
            dest_groups3 = dest_groups3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a TunnelDestGroup.");
            dest_groups3 = dest_groups3.subcommand(mcmd);
        }
        locations2 = locations2.subcommand(dest_groups3);
        iap_tunnel1 = iap_tunnel1.subcommand(locations2);
        brands1 = brands1.subcommand(identity_aware_proxy_clients2);
        projects0 = projects0.subcommand(iap_tunnel1);
        projects0 = projects0.subcommand(brands1);
        app = app.subcommand(v_10);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_iap1 as api;

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
