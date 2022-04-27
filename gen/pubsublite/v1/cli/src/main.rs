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
        let mut app = App::new("pubsublite1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20220415")
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
        let mut admin0 = SubCommand::with_name("admin")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: projects");
        let mut cursor0 = SubCommand::with_name("cursor")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: projects");
        let mut topic_stats0 = SubCommand::with_name("topic_stats")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: projects");
        let mut projects1 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut projects1 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut projects1 = SubCommand::with_name("projects")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: locations");
        let mut locations2 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: operations, reservations, subscriptions and topics");
        let mut locations2 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: subscriptions");
        let mut locations2 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: topics");
        let mut operations3 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: cancel, delete, get and list");
        {
            let mcmd = SubCommand::with_name("cancel").about("Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.");
            operations3 = operations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a long-running operation. This method indicates that the client is no longer interested in the operation result. It does not cancel the operation. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`.");
            operations3 = operations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations3 = operations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations3 = operations3.subcommand(mcmd);
        }
        let mut reservations3 = SubCommand::with_name("reservations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new reservation.");
            reservations3 = reservations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified reservation.");
            reservations3 = reservations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the reservation configuration.");
            reservations3 = reservations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of reservations for the given project.");
            reservations3 = reservations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates properties of the specified reservation.");
            reservations3 = reservations3.subcommand(mcmd);
        }
        let mut subscriptions3 = SubCommand::with_name("subscriptions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list, patch and seek");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new subscription.");
            subscriptions3 = subscriptions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified subscription.");
            subscriptions3 = subscriptions3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Returns the subscription configuration.");
            subscriptions3 = subscriptions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of subscriptions for the given project.");
            subscriptions3 = subscriptions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch")
                .about("Updates properties of the specified subscription.");
            subscriptions3 = subscriptions3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("seek").about("Performs an out-of-band seek for a subscription to a specified target, which may be timestamps or named positions within the message backlog. Seek translates these targets to cursors for each partition and orchestrates subscribers to start consuming messages from these seek cursors. If an operation is returned, the seek has been registered and subscribers will eventually receive messages from the seek cursors (i.e. eventual consistency), as long as they are using a minimum supported client library version and not a system that tracks cursors independently of Pub/Sub Lite (e.g. Apache Beam, Dataflow, Spark). The seek operation will fail for unsupported clients. If clients would like to know when subscribers react to the seek (or not), they can poll the operation. The seek operation will succeed and complete once subscribers are ready to receive messages from the seek cursors for all partitions of the topic. This means that the seek operation will not complete until all subscribers come online. If the previous seek operation has not yet completed, it will be aborted and the new invocation of seek will supersede it.");
            subscriptions3 = subscriptions3.subcommand(mcmd);
        }
        let mut topics3 = SubCommand::with_name("topics")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, get_partitions, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new topic.");
            topics3 = topics3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes the specified topic.");
            topics3 = topics3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Returns the topic configuration.");
            topics3 = topics3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_partitions")
                .about("Returns the partition information for the requested topic.");
            topics3 = topics3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns the list of topics for the given project.");
            topics3 = topics3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("patch").about("Updates properties of the specified topic.");
            topics3 = topics3.subcommand(mcmd);
        }
        let mut subscriptions3 = SubCommand::with_name("subscriptions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: commit_cursor");
        {
            let mcmd =
                SubCommand::with_name("commit_cursor").about("Updates the committed cursor.");
            subscriptions3 = subscriptions3.subcommand(mcmd);
        }
        let mut topics3 = SubCommand::with_name("topics")
            .setting(AppSettings::ColoredHelp)
            .about("methods: compute_head_cursor, compute_message_stats and compute_time_cursor");
        {
            let mcmd = SubCommand::with_name("compute_head_cursor").about("Compute the head cursor for the partition. The head cursor's offset is guaranteed to be less than or equal to all messages which have not yet been acknowledged as published, and greater than the offset of any message whose publish has already been acknowledged. It is zero if there have never been messages in the partition.");
            topics3 = topics3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("compute_message_stats").about(
                "Compute statistics about a range of messages in a given topic and partition.",
            );
            topics3 = topics3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("compute_time_cursor").about("Compute the corresponding cursor for a publish or event time in a topic partition.");
            topics3 = topics3.subcommand(mcmd);
        }
        let mut topics4 = SubCommand::with_name("topics")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the topics attached to the specified reservation.");
            topics4 = topics4.subcommand(mcmd);
        }
        let mut subscriptions4 = SubCommand::with_name("subscriptions")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists the subscriptions attached to the specified topic.");
            subscriptions4 = subscriptions4.subcommand(mcmd);
        }
        let mut cursors4 = SubCommand::with_name("cursors")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Returns all committed cursor information for a subscription.");
            cursors4 = cursors4.subcommand(mcmd);
        }
        subscriptions3 = subscriptions3.subcommand(cursors4);
        topics3 = topics3.subcommand(subscriptions4);
        reservations3 = reservations3.subcommand(topics4);
        locations2 = locations2.subcommand(topics3);
        locations2 = locations2.subcommand(subscriptions3);
        locations2 = locations2.subcommand(topics3);
        locations2 = locations2.subcommand(subscriptions3);
        locations2 = locations2.subcommand(reservations3);
        locations2 = locations2.subcommand(operations3);
        projects1 = projects1.subcommand(locations2);
        projects1 = projects1.subcommand(locations2);
        projects1 = projects1.subcommand(locations2);
        topic_stats0 = topic_stats0.subcommand(projects1);
        cursor0 = cursor0.subcommand(projects1);
        admin0 = admin0.subcommand(projects1);
        app = app.subcommand(topic_stats0);
        app = app.subcommand(cursor0);
        app = app.subcommand(admin0);

        Self { app }
    }
}
use google_pubsublite1 as api;

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
