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
        let mut app = App::new("drivelabels2_beta")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230125")
            .about("An API for managing Drive Labels")
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
        let mut labels0 = SubCommand::with_name("labels")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: create, delete, delta, disable, enable, get, list, publish, update_label_copy_mode and update_permissions");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new Label.");
            labels0 = labels0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Permanently deletes a Label and related metadata on Drive Items. Once deleted, the Label and related Drive item metadata will be deleted. Only draft Labels, and disabled Labels may be deleted.");
            labels0 = labels0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delta").about("Updates a single Label by applying a set of update requests resulting in a new draft revision. The batch update is all-or-nothing: If any of the update requests are invalid, no changes are applied. The resulting draft revision must be published before the changes may be used with Drive Items.");
            labels0 = labels0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("disable").about("Disable a published Label. Disabling a Label will result in a new disabled published revision based on the current published revision. If there is a draft revision, a new disabled draft revision will be created based on the latest draft revision. Older draft revisions will be deleted. Once disabled, a label may be deleted with `DeleteLabel`.");
            labels0 = labels0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("enable").about("Enable a disabled Label and restore it to its published state. This will result in a new published revision based on the current disabled published revision. If there is an existing disabled draft revision, a new revision will be created based on that draft and will be enabled.");
            labels0 = labels0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get a label by its resource name. Resource name may be any of: * `labels/{id}` - See `labels/{id}@latest` * `labels/{id}@latest` - Gets the latest revision of the label. * `labels/{id}@published` - Gets the current published revision of the label. * `labels/{id}@{revision_id}` - Gets the label at the specified revision ID.");
            labels0 = labels0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List labels.");
            labels0 = labels0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("publish").about("Publish all draft changes to the Label. Once published, the Label may not return to its draft state. See `google.apps.drive.labels.v2.Lifecycle` for more information. Publishing a Label will result in a new published revision. All previous draft revisions will be deleted. Previous published revisions will be kept but are subject to automated deletion as needed. Once published, some changes are no longer permitted. Generally, any change that would invalidate or cause new restrictions on existing metadata related to the Label will be rejected. For example, the following changes to a Label will be rejected after the Label is published: * The label cannot be directly deleted. It must be disabled first, then deleted. * Field.FieldType cannot be changed. * Changes to Field validation options cannot reject something that was previously accepted. * Reducing the max entries.");
            labels0 = labels0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_label_copy_mode").about("Updates a Label's `CopyMode`. Changes to this policy are not revisioned, do not require publishing, and take effect immediately.");
            labels0 = labels0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_permissions").about("Updates a Label's permissions. If a permission for the indicated principal doesn't exist, a new Label Permission is created, otherwise the existing permission is updated. Permissions affect the Label resource as a whole, are not revisioned, and do not require publishing.");
            labels0 = labels0.subcommand(mcmd);
        }
        let mut limits0 = SubCommand::with_name("limits")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_label");
        {
            let mcmd = SubCommand::with_name("get_label").about("Get the constraints on the structure of a Label; such as, the maximum number of Fields allowed and maximum length of the label title.");
            limits0 = limits0.subcommand(mcmd);
        }
        let mut users0 = SubCommand::with_name("users")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get_capabilities");
        {
            let mcmd =
                SubCommand::with_name("get_capabilities").about("Gets the user capabilities.");
            users0 = users0.subcommand(mcmd);
        }
        let mut locks1 = SubCommand::with_name("locks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists the LabelLocks on a Label.");
            locks1 = locks1.subcommand(mcmd);
        }
        let mut permissions1 = SubCommand::with_name("permissions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_delete, batch_update, create, delete and list");
        {
            let mcmd = SubCommand::with_name("batch_delete").about("Deletes Label permissions. Permissions affect the Label resource as a whole, are not revisioned, and do not require publishing.");
            permissions1 = permissions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_update").about("Updates Label permissions. If a permission for the indicated principal doesn't exist, a new Label Permission is created, otherwise the existing permission is updated. Permissions affect the Label resource as a whole, are not revisioned, and do not require publishing.");
            permissions1 = permissions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Updates a Label's permissions. If a permission for the indicated principal doesn't exist, a new Label Permission is created, otherwise the existing permission is updated. Permissions affect the Label resource as a whole, are not revisioned, and do not require publishing.");
            permissions1 = permissions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a Label's permission. Permissions affect the Label resource as a whole, are not revisioned, and do not require publishing.");
            permissions1 = permissions1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists a Label's permissions.");
            permissions1 = permissions1.subcommand(mcmd);
        }
        let mut revisions1 = SubCommand::with_name("revisions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: update_permissions");
        {
            let mcmd = SubCommand::with_name("update_permissions").about("Updates a Label's permissions. If a permission for the indicated principal doesn't exist, a new Label Permission is created, otherwise the existing permission is updated. Permissions affect the Label resource as a whole, are not revisioned, and do not require publishing.");
            revisions1 = revisions1.subcommand(mcmd);
        }
        let mut locks2 = SubCommand::with_name("locks")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists the LabelLocks on a Label.");
            locks2 = locks2.subcommand(mcmd);
        }
        let mut permissions2 = SubCommand::with_name("permissions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_delete, batch_update, create, delete and list");
        {
            let mcmd = SubCommand::with_name("batch_delete").about("Deletes Label permissions. Permissions affect the Label resource as a whole, are not revisioned, and do not require publishing.");
            permissions2 = permissions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_update").about("Updates Label permissions. If a permission for the indicated principal doesn't exist, a new Label Permission is created, otherwise the existing permission is updated. Permissions affect the Label resource as a whole, are not revisioned, and do not require publishing.");
            permissions2 = permissions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Updates a Label's permissions. If a permission for the indicated principal doesn't exist, a new Label Permission is created, otherwise the existing permission is updated. Permissions affect the Label resource as a whole, are not revisioned, and do not require publishing.");
            permissions2 = permissions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a Label's permission. Permissions affect the Label resource as a whole, are not revisioned, and do not require publishing.");
            permissions2 = permissions2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists a Label's permissions.");
            permissions2 = permissions2.subcommand(mcmd);
        }
        revisions1 = revisions1.subcommand(permissions2);
        revisions1 = revisions1.subcommand(locks2);
        labels0 = labels0.subcommand(revisions1);
        labels0 = labels0.subcommand(permissions1);
        labels0 = labels0.subcommand(locks1);
        app = app.subcommand(users0);
        app = app.subcommand(limits0);
        app = app.subcommand(labels0);

        Self { app }
    }
}
use google_drivelabels2_beta as api;

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
