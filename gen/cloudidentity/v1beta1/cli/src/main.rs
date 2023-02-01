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
        let mut app = App::new("cloudidentity1_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230124")
            .about("API for provisioning and managing identity resources.")
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
        let mut customers0 = SubCommand::with_name("customers")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: userinvitations");
        let mut devices0 = SubCommand::with_name("devices")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel_wipe, create, delete, get, list and wipe");
        {
            let mcmd = SubCommand::with_name("cancel_wipe").about("Cancels an unfinished device wipe. This operation can be used to cancel device wipe in the gap between the wipe operation returning success and the device being wiped.");
            devices0 = devices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a device. Only company-owned device may be created. **Note**: This method is available only to customers who have one of the following SKUs: Enterprise Standard, Enterprise Plus, Enterprise for Education, and Cloud Identity Premium");
            devices0 = devices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified device.");
            devices0 = devices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified device.");
            devices0 = devices0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists/Searches devices.");
            devices0 = devices0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("wipe").about("Wipes all data on the specified device.");
            devices0 = devices0.subcommand(mcmd);
        }
        let mut groups0 = SubCommand::with_name("groups")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_security_settings, list, lookup, patch, search and update_security_settings");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a `Group`.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a `Group`.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a `Group`.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get_security_settings").about("Get Security Settings");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the `Group` resources under a customer or namespace.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("lookup").about("Looks up the [resource name](https://cloud.google.com/apis/design/resource_names) of a `Group` by its `EntityKey`.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a `Group`.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search")
                .about("Searches for `Group` resources matching a specified query.");
            groups0 = groups0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("update_security_settings").about("Update Security Settings");
            groups0 = groups0.subcommand(mcmd);
        }
        let mut inbound_saml_sso_profiles0 = SubCommand::with_name("inbound_saml_sso_profiles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Creates an InboundSamlSsoProfile for a customer.");
            inbound_saml_sso_profiles0 = inbound_saml_sso_profiles0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an InboundSamlSsoProfile.");
            inbound_saml_sso_profiles0 = inbound_saml_sso_profiles0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an InboundSamlSsoProfile.");
            inbound_saml_sso_profiles0 = inbound_saml_sso_profiles0.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists InboundSamlSsoProfiles for a customer.");
            inbound_saml_sso_profiles0 = inbound_saml_sso_profiles0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an InboundSamlSsoProfile.");
            inbound_saml_sso_profiles0 = inbound_saml_sso_profiles0.subcommand(mcmd);
        }
        let mut inbound_sso_assignments0 = SubCommand::with_name("inbound_sso_assignments")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates an InboundSsoAssignment for users and devices in a `Customer` under a given `Group` or `OrgUnit`.");
            inbound_sso_assignments0 = inbound_sso_assignments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an InboundSsoAssignment. To disable SSO, Create (or Update) an assignment that has `sso_mode` == `SSO_OFF`.");
            inbound_sso_assignments0 = inbound_sso_assignments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an InboundSsoAssignment.");
            inbound_sso_assignments0 = inbound_sso_assignments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the InboundSsoAssignments for a `Customer`.");
            inbound_sso_assignments0 = inbound_sso_assignments0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an InboundSsoAssignment. The body of this request is the `inbound_sso_assignment` field and the `update_mask` is relative to that. For example: a PATCH to `/v1beta1/inboundSsoAssignments/0abcdefg1234567&update_mask=rank` with a body of `{ \"rank\": 1 }` moves that (presumably group-targeted) SSO assignment to the highest priority and shifts any other group-targeted assignments down in priority.");
            inbound_sso_assignments0 = inbound_sso_assignments0.subcommand(mcmd);
        }
        let mut org_units0 = SubCommand::with_name("org_units")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: memberships");
        let mut userinvitations1 = SubCommand::with_name("userinvitations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, get, is_invitable_user, list and send");
        {
            let mcmd = SubCommand::with_name("cancel")
                .about("Cancels a UserInvitation that was already sent.");
            userinvitations1 = userinvitations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a UserInvitation resource. **Note:** New consumer accounts with the customer's verified domain created within the previous 48 hours will not appear in the result. This delay also applies to newly-verified domains.");
            userinvitations1 = userinvitations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("is_invitable_user").about("Verifies whether a user account is eligible to receive a UserInvitation (is an unmanaged account). Eligibility is based on the following criteria: * the email address is a consumer account and it's the primary email address of the account, and * the domain of the email address matches an existing verified Google Workspace or Cloud Identity domain If both conditions are met, the user is eligible. **Note:** This method is not supported for Workspace Essentials customers.");
            userinvitations1 = userinvitations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Retrieves a list of UserInvitation resources. **Note:** New consumer accounts with the customer's verified domain created within the previous 48 hours will not appear in the result. This delay also applies to newly-verified domains.");
            userinvitations1 = userinvitations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("send").about("Sends a UserInvitation to email. If the `UserInvitation` does not exist for this request and it is a valid request, the request creates a `UserInvitation`. **Note:** The `get` and `list` methods have a 48-hour delay where newly-created consumer accounts will not appear in the results. You can still send a `UserInvitation` to those accounts if you know the unmanaged email address and IsInvitableUser==True.");
            userinvitations1 = userinvitations1.subcommand(mcmd);
        }
        let mut device_users1 = SubCommand::with_name("device_users")
            .setting(AppSettings::ColoredHelp)
            .about("methods: approve, block, cancel_wipe, delete, get, list, lookup and wipe");
        {
            let mcmd =
                SubCommand::with_name("approve").about("Approves device to access user data.");
            device_users1 = device_users1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("block").about("Blocks device from accessing user data");
            device_users1 = device_users1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("cancel_wipe").about("Cancels an unfinished user account wipe. This operation can be used to cancel device wipe in the gap between the wipe operation returning success and the device being wiped.");
            device_users1 = device_users1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified DeviceUser. This also revokes the user's access to device data.");
            device_users1 = device_users1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves the specified DeviceUser");
            device_users1 = device_users1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists/Searches DeviceUsers.");
            device_users1 = device_users1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("lookup").about("Looks up resource names of the DeviceUsers associated with the caller's credentials, as well as the properties provided in the request. This method must be called with end-user credentials with the scope: https://www.googleapis.com/auth/cloud-identity.devices.lookup If multiple properties are provided, only DeviceUsers having all of these properties are considered as matches - i.e. the query behaves like an AND. Different platforms require different amounts of information from the caller to ensure that the DeviceUser is uniquely identified. - iOS: No properties need to be passed, the caller's credentials are sufficient to identify the corresponding DeviceUser. - Android: Specifying the 'android_id' field is required. - Desktop: Specifying the 'raw_resource_id' field is required.");
            device_users1 = device_users1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("wipe").about("Wipes the user's account on a device.");
            device_users1 = device_users1.subcommand(mcmd);
        }
        let mut memberships1 = SubCommand::with_name("memberships")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: check_transitive_membership, create, delete, get, get_membership_graph, list, lookup, modify_membership_roles, search_transitive_groups and search_transitive_memberships");
        {
            let mcmd = SubCommand::with_name("check_transitive_membership").about("Check a potential member for membership in a group. **Note:** This feature is only available to Google Workspace Enterprise Standard, Enterprise Plus, and Enterprise for Education; and Cloud Identity Premium accounts. A member has membership to a group as long as there is a single viewable transitive membership between the group and the member. The actor must have view permissions to at least one transitive membership between the member and group.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a `Membership`.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a `Membership`.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves a `Membership`.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_membership_graph").about("Get a membership graph of just a member or both a member and a group. **Note:** This feature is only available to Google Workspace Enterprise Standard, Enterprise Plus, and Enterprise for Education; and Cloud Identity Premium accounts. Given a member, the response will contain all membership paths from the member. Given both a group and a member, the response will contain all membership paths between the group and the member.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists the `Membership`s within a `Group`.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("lookup").about("Looks up the [resource name](https://cloud.google.com/apis/design/resource_names) of a `Membership` by its `EntityKey`.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("modify_membership_roles")
                .about("Modifies the `MembershipRole`s of a `Membership`.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_transitive_groups").about("Search transitive groups of a member. **Note:** This feature is only available to Google Workspace Enterprise Standard, Enterprise Plus, and Enterprise for Education; and Cloud Identity Premium accounts. A transitive group is any group that has a direct or indirect membership to the member. Actor must have view permissions all transitive groups.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_transitive_memberships").about("Search transitive memberships of a group. **Note:** This feature is only available to Google Workspace Enterprise Standard, Enterprise Plus, and Enterprise for Education; and Cloud Identity Premium accounts. A transitive membership is any direct or indirect membership of a group. Actor must have view permissions to all transitive memberships.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        let mut idp_credentials1 = SubCommand::with_name("idp_credentials")
            .setting(AppSettings::ColoredHelp)
            .about("methods: add, delete, get and list");
        {
            let mcmd = SubCommand::with_name("add")
                .about("Adds an IdpCredential. Up to 2 credentials are allowed.");
            idp_credentials1 = idp_credentials1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an IdpCredential.");
            idp_credentials1 = idp_credentials1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets an IdpCredential.");
            idp_credentials1 = idp_credentials1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns a list of IdpCredentials in an InboundSamlSsoProfile.");
            idp_credentials1 = idp_credentials1.subcommand(mcmd);
        }
        let mut memberships1 = SubCommand::with_name("memberships")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list and r#move");
        {
            let mcmd = SubCommand::with_name("list").about("List OrgMembership resources in an OrgUnit treated as 'parent'. Parent format: orgUnits/{$orgUnitId} where `$orgUnitId` is the `orgUnitId` from the [Admin SDK `OrgUnit` resource](https://developers.google.com/admin-sdk/directory/reference/rest/v1/orgunits)");
            memberships1 = memberships1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("r#move").about("Move an OrgMembership to a new OrgUnit. NOTE: This is an atomic copy-and-delete. The resource will have a new copy under the destination OrgUnit and be deleted from the source OrgUnit. The resource can only be searched under the destination OrgUnit afterwards.");
            memberships1 = memberships1.subcommand(mcmd);
        }
        let mut client_states2 = SubCommand::with_name("client_states")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and patch");
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets the client state for the device user");
            client_states2 = client_states2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the client state for the device user **Note**: This method is available only to customers who have one of the following SKUs: Enterprise Standard, Enterprise Plus, Enterprise for Education, and Cloud Identity Premium");
            client_states2 = client_states2.subcommand(mcmd);
        }
        device_users1 = device_users1.subcommand(client_states2);
        org_units0 = org_units0.subcommand(memberships1);
        inbound_saml_sso_profiles0 = inbound_saml_sso_profiles0.subcommand(idp_credentials1);
        groups0 = groups0.subcommand(memberships1);
        devices0 = devices0.subcommand(device_users1);
        customers0 = customers0.subcommand(userinvitations1);
        app = app.subcommand(org_units0);
        app = app.subcommand(inbound_sso_assignments0);
        app = app.subcommand(inbound_saml_sso_profiles0);
        app = app.subcommand(groups0);
        app = app.subcommand(devices0);
        app = app.subcommand(customers0);

        Self { app }
    }
}
use google_cloudidentity1_beta1 as api;

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
