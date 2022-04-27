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
        let mut app = App::new("privateca1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20220406")
            .about("The Certificate Authority Service API is a highly-available, scalable service that enables you to simplify and automate the management of private certificate authorities (CAs) while staying in control of your private keys. ")
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
        let mut ca_pools2 = SubCommand::with_name("ca_pools")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, fetch_ca_certs, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Create a CaPool.");
            ca_pools2 = ca_pools2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a CaPool.");
            ca_pools2 = ca_pools2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("fetch_ca_certs").about("FetchCaCerts returns the current trust anchor for the CaPool. This will include CA certificate chains for all ACTIVE CertificateAuthority resources in the CaPool.");
            ca_pools2 = ca_pools2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a CaPool.");
            ca_pools2 = ca_pools2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            ca_pools2 = ca_pools2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists CaPools.");
            ca_pools2 = ca_pools2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update a CaPool.");
            ca_pools2 = ca_pools2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            ca_pools2 = ca_pools2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            ca_pools2 = ca_pools2.subcommand(mcmd);
        }
        let mut certificate_templates2 = SubCommand::with_name("certificate_templates")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions");
        {
            let mcmd = SubCommand::with_name("create")
                .about("Create a new CertificateTemplate in a given Project and Location.");
            certificate_templates2 = certificate_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete")
                .about("DeleteCertificateTemplate deletes a CertificateTemplate.");
            certificate_templates2 = certificate_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a CertificateTemplate.");
            certificate_templates2 = certificate_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            certificate_templates2 = certificate_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists CertificateTemplates.");
            certificate_templates2 = certificate_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update a CertificateTemplate.");
            certificate_templates2 = certificate_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            certificate_templates2 = certificate_templates2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            certificate_templates2 = certificate_templates2.subcommand(mcmd);
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
        let mut certificate_authorities3 = SubCommand::with_name("certificate_authorities")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: activate, create, delete, disable, enable, fetch, get, list, patch and undelete");
        {
            let mcmd = SubCommand::with_name("activate").about("Activate a CertificateAuthority that is in state AWAITING_USER_ACTIVATION and is of type SUBORDINATE. After the parent Certificate Authority signs a certificate signing request from FetchCertificateAuthorityCsr, this method can complete the activation process.");
            certificate_authorities3 = certificate_authorities3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create")
                .about("Create a new CertificateAuthority in a given Project and Location.");
            certificate_authorities3 = certificate_authorities3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete a CertificateAuthority.");
            certificate_authorities3 = certificate_authorities3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("disable").about("Disable a CertificateAuthority.");
            certificate_authorities3 = certificate_authorities3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("enable").about("Enable a CertificateAuthority.");
            certificate_authorities3 = certificate_authorities3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("fetch").about("Fetch a certificate signing request (CSR) from a CertificateAuthority that is in state AWAITING_USER_ACTIVATION and is of type SUBORDINATE. The CSR must then be signed by the desired parent Certificate Authority, which could be another CertificateAuthority resource, or could be an on-prem certificate authority. See also ActivateCertificateAuthority.");
            certificate_authorities3 = certificate_authorities3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a CertificateAuthority.");
            certificate_authorities3 = certificate_authorities3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists CertificateAuthorities.");
            certificate_authorities3 = certificate_authorities3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update a CertificateAuthority.");
            certificate_authorities3 = certificate_authorities3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undelete")
                .about("Undelete a CertificateAuthority that has been deleted.");
            certificate_authorities3 = certificate_authorities3.subcommand(mcmd);
        }
        let mut certificates3 = SubCommand::with_name("certificates")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get, list, patch and revoke");
        {
            let mcmd = SubCommand::with_name("create").about(
                "Create a new Certificate in a given Project, Location from a particular CaPool.",
            );
            certificates3 = certificates3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns a Certificate.");
            certificates3 = certificates3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists Certificates.");
            certificates3 = certificates3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update a Certificate. Currently, the only field you can update is the labels field.");
            certificates3 = certificates3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("revoke").about("Revoke a Certificate.");
            certificates3 = certificates3.subcommand(mcmd);
        }
        let mut certificate_revocation_lists4 = SubCommand::with_name(
            "certificate_revocation_lists",
        )
        .setting(AppSettings::ColoredHelp)
        .about(
            "methods: get, get_iam_policy, list, patch, set_iam_policy and test_iam_permissions",
        );
        {
            let mcmd = SubCommand::with_name("get").about("Returns a CertificateRevocationList.");
            certificate_revocation_lists4 = certificate_revocation_lists4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_iam_policy").about("Gets the access control policy for a resource. Returns an empty policy if the resource exists and does not have a policy set.");
            certificate_revocation_lists4 = certificate_revocation_lists4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists CertificateRevocationLists.");
            certificate_revocation_lists4 = certificate_revocation_lists4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Update a CertificateRevocationList.");
            certificate_revocation_lists4 = certificate_revocation_lists4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_iam_policy").about("Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.");
            certificate_revocation_lists4 = certificate_revocation_lists4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("test_iam_permissions").about("Returns permissions that a caller has on the specified resource. If the resource does not exist, this will return an empty set of permissions, not a `NOT_FOUND` error. Note: This operation is designed to be used for building permission-aware UIs and command-line tools, not for authorization checking. This operation may \"fail open\" without warning.");
            certificate_revocation_lists4 = certificate_revocation_lists4.subcommand(mcmd);
        }
        certificate_authorities3 =
            certificate_authorities3.subcommand(certificate_revocation_lists4);
        ca_pools2 = ca_pools2.subcommand(certificates3);
        ca_pools2 = ca_pools2.subcommand(certificate_authorities3);
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(certificate_templates2);
        locations1 = locations1.subcommand(ca_pools2);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_privateca1 as api;

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
