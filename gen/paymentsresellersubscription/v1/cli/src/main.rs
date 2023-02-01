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
        let mut app = App::new("paymentsresellersubscription1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230131")
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
        let mut partners0 = SubCommand::with_name("partners")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: products, promotions and subscriptions");
        let mut products1 = SubCommand::with_name("products")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("To retrieve the products that can be resold by the partner. It should be autenticated with a service account.");
            products1 = products1.subcommand(mcmd);
        }
        let mut promotions1 = SubCommand::with_name("promotions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: find_eligible and list");
        {
            let mcmd = SubCommand::with_name("find_eligible").about("To find eligible promotions for the current user. The API requires user authorization via OAuth. The user is inferred from the authenticated OAuth credential.");
            promotions1 = promotions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("To retrieve the promotions, such as free trial, that can be used by the partner. It should be autenticated with a service account.");
            promotions1 = promotions1.subcommand(mcmd);
        }
        let mut subscriptions1 = SubCommand::with_name("subscriptions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, create, entitle, extend, get, provision and undo_cancel");
        {
            let mcmd = SubCommand::with_name("cancel").about("Used by partners to cancel a subscription service either immediately or by the end of the current billing cycle for their customers. It should be called directly by the partner using service accounts.");
            subscriptions1 = subscriptions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Used by partners to create a subscription for their customers. The created subscription is associated with the end user inferred from the end user credentials. This API must be authorized by the end user using OAuth.");
            subscriptions1 = subscriptions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("entitle").about("Used by partners to entitle a previously provisioned subscription to the current end user. The end user identity is inferred from the authorized credential of the request. This API must be authorized by the end user using OAuth.");
            subscriptions1 = subscriptions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("extend").about("[Deprecated] New partners should be on auto-extend by default. Used by partners to extend a subscription service for their customers on an ongoing basis for the subscription to remain active and renewable. It should be called directly by the partner using service accounts.");
            subscriptions1 = subscriptions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Used by partners to get a subscription by id. It should be called directly by the partner using service accounts.");
            subscriptions1 = subscriptions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("provision").about("Used by partners to provision a subscription for their customers. This creates a subscription without associating it with the end user account. EntitleSubscription must be called separately using OAuth in order for the end user account to be associated with the subscription. It should be called directly by the partner using service accounts.");
            subscriptions1 = subscriptions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("undo_cancel").about("Used by partners to revoke the pending cancellation of a subscription, which is currently in `STATE_CANCEL_AT_END_OF_CYCLE` state. If the subscription is already cancelled, the request will fail. It should be called directly by the partner using service accounts.");
            subscriptions1 = subscriptions1.subcommand(mcmd);
        }
        partners0 = partners0.subcommand(subscriptions1);
        partners0 = partners0.subcommand(promotions1);
        partners0 = partners0.subcommand(products1);
        app = app.subcommand(partners0);

        Self { app }
    }
}
use google_paymentsresellersubscription1 as api;

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
