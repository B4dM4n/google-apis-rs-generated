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
        let mut app = App::new("people1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230130")
            .about("Provides access to information about profiles and contacts.")
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
        let mut contact_groups0 = SubCommand::with_name("contact_groups")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_get, create, delete, get, list and update");
        {
            let mcmd = SubCommand::with_name("batch_get").about("Get a list of contact groups owned by the authenticated user by specifying a list of contact group resource names.");
            contact_groups0 = contact_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Create a new contact group owned by the authenticated user. Created contact group names must be unique to the users contact groups. Attempting to create a group with a duplicate name will return a HTTP 409 error. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.");
            contact_groups0 = contact_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Delete an existing contact group owned by the authenticated user by specifying a contact group resource name. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.");
            contact_groups0 = contact_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Get a specific contact group owned by the authenticated user by specifying a contact group resource name.");
            contact_groups0 = contact_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List all contact groups owned by the authenticated user. Members of the contact groups are not populated.");
            contact_groups0 = contact_groups0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Update the name of an existing contact group owned by the authenticated user. Updated contact group names must be unique to the users contact groups. Attempting to create a group with a duplicate name will return a HTTP 409 error. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.");
            contact_groups0 = contact_groups0.subcommand(mcmd);
        }
        let mut other_contacts0 = SubCommand::with_name("other_contacts")
            .setting(AppSettings::ColoredHelp)
            .about("methods: copy_other_contact_to_my_contacts_group, list and search");
        {
            let mcmd = SubCommand::with_name("copy_other_contact_to_my_contacts_group").about("Copies an \"Other contact\" to a new contact in the user's \"myContacts\" group Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.");
            other_contacts0 = other_contacts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List all \"Other contacts\", that is contacts that are not in a contact group. \"Other contacts\" are typically auto created contacts from interactions. Sync tokens expire 7 days after the full sync. A request with an expired sync token will get an error with an [google.rpc.ErrorInfo](https://cloud.google.com/apis/design/errors#error_info) with reason \"EXPIRED_SYNC_TOKEN\". In the case of such an error clients should make a full sync request without a `sync_token`. The first page of a full sync request has an additional quota. If the quota is exceeded, a 429 error will be returned. This quota is fixed and can not be increased. When the `sync_token` is specified, resources deleted since the last sync will be returned as a person with `PersonMetadata.deleted` set to true. When the `page_token` or `sync_token` is specified, all other request parameters must match the first call. Writes may have a propagation delay of several minutes for sync requests. Incremental syncs are not intended for read-after-write use cases. See example usage at [List the user's other contacts that have changed](/people/v1/other-contacts#list_the_users_other_contacts_that_have_changed).");
            other_contacts0 = other_contacts0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search").about("Provides a list of contacts in the authenticated user's other contacts that matches the search query. The query matches on a contact's `names`, `emailAddresses`, and `phoneNumbers` fields that are from the OTHER_CONTACT source. **IMPORTANT**: Before searching, clients should send a warmup request with an empty query to update the cache. See https://developers.google.com/people/v1/other-contacts#search_the_users_other_contacts");
            other_contacts0 = other_contacts0.subcommand(mcmd);
        }
        let mut people0 = SubCommand::with_name("people")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: batch_create_contacts, batch_delete_contacts, batch_update_contacts, create_contact, delete_contact, delete_contact_photo, get, get_batch_get, list_directory_people, search_contacts, search_directory_people, update_contact and update_contact_photo");
        {
            let mcmd = SubCommand::with_name("batch_create_contacts").about("Create a batch of new contacts and return the PersonResponses for the newly Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.");
            people0 = people0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_delete_contacts").about("Delete a batch of contacts. Any non-contact data will not be deleted. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.");
            people0 = people0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("batch_update_contacts").about("Update a batch of contacts and return a map of resource names to PersonResponses for the updated contacts. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.");
            people0 = people0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create_contact").about("Create a new contact and return the person resource for that contact. The request returns a 400 error if more than one field is specified on a field that is a singleton for contact sources: * biographies * birthdays * genders * names Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.");
            people0 = people0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete_contact").about("Delete a contact person. Any non-contact data will not be deleted. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.");
            people0 = people0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete_contact_photo").about("Delete a contact's photo. Mutate requests for the same user should be done sequentially to avoid // lock contention.");
            people0 = people0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Provides information about a person by specifying a resource name. Use `people/me` to indicate the authenticated user. The request returns a 400 error if 'personFields' is not specified.");
            people0 = people0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_batch_get").about("Provides information about a list of specific people by specifying a list of requested resource names. Use `people/me` to indicate the authenticated user. The request returns a 400 error if 'personFields' is not specified.");
            people0 = people0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list_directory_people").about("Provides a list of domain profiles and domain contacts in the authenticated user's domain directory. When the `sync_token` is specified, resources deleted since the last sync will be returned as a person with `PersonMetadata.deleted` set to true. When the `page_token` or `sync_token` is specified, all other request parameters must match the first call. Writes may have a propagation delay of several minutes for sync requests. Incremental syncs are not intended for read-after-write use cases. See example usage at [List the directory people that have changed](/people/v1/directory#list_the_directory_people_that_have_changed).");
            people0 = people0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_contacts").about("Provides a list of contacts in the authenticated user's grouped contacts that matches the search query. The query matches on a contact's `names`, `nickNames`, `emailAddresses`, `phoneNumbers`, and `organizations` fields that are from the CONTACT source. **IMPORTANT**: Before searching, clients should send a warmup request with an empty query to update the cache. See https://developers.google.com/people/v1/contacts#search_the_users_contacts");
            people0 = people0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search_directory_people").about("Provides a list of domain profiles and domain contacts in the authenticated user's domain directory that match the search query.");
            people0 = people0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_contact").about("Update contact data for an existing contact person. Any non-contact data will not be modified. Any non-contact data in the person to update will be ignored. All fields specified in the `update_mask` will be replaced. The server returns a 400 error if `person.metadata.sources` is not specified for the contact to be updated or if there is no contact source. The server returns a 400 error with reason `\"failedPrecondition\"` if `person.metadata.sources.etag` is different than the contact's etag, which indicates the contact has changed since its data was read. Clients should get the latest person and merge their updates into the latest person. The server returns a 400 error if `memberships` are being updated and there are no contact group memberships specified on the person. The server returns a 400 error if more than one field is specified on a field that is a singleton for contact sources: * biographies * birthdays * genders * names Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.");
            people0 = people0.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_contact_photo").about("Update a contact's photo. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.");
            people0 = people0.subcommand(mcmd);
        }
        let mut members1 = SubCommand::with_name("members")
            .setting(AppSettings::ColoredHelp)
            .about("methods: modify");
        {
            let mcmd = SubCommand::with_name("modify").about("Modify the members of a contact group owned by the authenticated user. The only system contact groups that can have members added are `contactGroups/myContacts` and `contactGroups/starred`. Other system contact groups are deprecated and can only have contacts removed.");
            members1 = members1.subcommand(mcmd);
        }
        let mut connections1 = SubCommand::with_name("connections")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Provides a list of the authenticated user's contacts. Sync tokens expire 7 days after the full sync. A request with an expired sync token will get an error with an [google.rpc.ErrorInfo](https://cloud.google.com/apis/design/errors#error_info) with reason \"EXPIRED_SYNC_TOKEN\". In the case of such an error clients should make a full sync request without a `sync_token`. The first page of a full sync request has an additional quota. If the quota is exceeded, a 429 error will be returned. This quota is fixed and can not be increased. When the `sync_token` is specified, resources deleted since the last sync will be returned as a person with `PersonMetadata.deleted` set to true. When the `page_token` or `sync_token` is specified, all other request parameters must match the first call. Writes may have a propagation delay of several minutes for sync requests. Incremental syncs are not intended for read-after-write use cases. See example usage at [List the user's contacts that have changed](/people/v1/contacts#list_the_users_contacts_that_have_changed).");
            connections1 = connections1.subcommand(mcmd);
        }
        people0 = people0.subcommand(connections1);
        contact_groups0 = contact_groups0.subcommand(members1);
        app = app.subcommand(people0);
        app = app.subcommand(other_contacts0);
        app = app.subcommand(contact_groups0);

        Self { app }
    }
}
use google_people1 as api;

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
