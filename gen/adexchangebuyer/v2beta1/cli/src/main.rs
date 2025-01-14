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
        let mut app = App::new("adexchangebuyer2_beta1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20230131")
            .about("Accesses the latest features for managing Authorized Buyers accounts, Real-Time Bidding configurations and auction metrics, and Marketplace programmatic deals.")
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
        let mut accounts0 = SubCommand::with_name("accounts")
                        .setting(AppSettings::ColoredHelp)
                        .about("sub-resources: clients, creatives, finalized_proposals, products, proposals and publisher_profiles");
        let mut bidders0 = SubCommand::with_name("bidders")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: accounts and filter_sets");
        let mut clients1 = SubCommand::with_name("clients")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get, list and update");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new client buyer.");
            clients1 = clients1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets a client buyer with a given client account ID.");
            clients1 = clients1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the clients for the current sponsor buyer.");
            clients1 = clients1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates an existing client buyer.");
            clients1 = clients1.subcommand(mcmd);
        }
        let mut creatives1 = SubCommand::with_name("creatives")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get, list, stop_watching, update and watch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a creative.");
            creatives1 = creatives1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a creative.");
            creatives1 = creatives1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists creatives.");
            creatives1 = creatives1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("stop_watching").about("Stops watching a creative. Will stop push notifications being sent to the topics when the creative changes status.");
            creatives1 = creatives1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Updates a creative.");
            creatives1 = creatives1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("watch").about("Watches a creative. Will result in push notifications being sent to the topic when the creative changes status.");
            creatives1 = creatives1.subcommand(mcmd);
        }
        let mut finalized_proposals1 = SubCommand::with_name("finalized_proposals")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list, pause and resume");
        {
            let mcmd = SubCommand::with_name("list").about("List finalized proposals, regardless if a proposal is being renegotiated. A filter expression (PQL query) may be specified to filter the results. The notes will not be returned.");
            finalized_proposals1 = finalized_proposals1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("pause").about("Update given deals to pause serving. This method will set the `DealServingMetadata.DealPauseStatus.has_buyer_paused` bit to true for all listed deals in the request. Currently, this method only applies to PG and PD deals. For PA deals, call accounts.proposals.pause endpoint. It is a no-op to pause already-paused deals. It is an error to call PauseProposalDeals for deals which are not part of the proposal of proposal_id or which are not finalized or renegotiating.");
            finalized_proposals1 = finalized_proposals1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("resume").about("Update given deals to resume serving. This method will set the `DealServingMetadata.DealPauseStatus.has_buyer_paused` bit to false for all listed deals in the request. Currently, this method only applies to PG and PD deals. For PA deals, call accounts.proposals.resume endpoint. It is a no-op to resume running deals or deals paused by the other party. It is an error to call ResumeProposalDeals for deals which are not part of the proposal of proposal_id or which are not finalized or renegotiating.");
            finalized_proposals1 = finalized_proposals1.subcommand(mcmd);
        }
        let mut products1 = SubCommand::with_name("products")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the requested product by ID.");
            products1 = products1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List all products visible to the buyer (optionally filtered by the specified PQL query).");
            products1 = products1.subcommand(mcmd);
        }
        let mut proposals1 = SubCommand::with_name("proposals")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: accept, add_note, cancel_negotiation, complete_setup, create, get, list, pause, resume and update");
        {
            let mcmd = SubCommand::with_name("accept").about("Mark the proposal as accepted at the given revision number. If the number does not match the server's revision number an `ABORTED` error message will be returned. This call updates the proposal_state from `PROPOSED` to `BUYER_ACCEPTED`, or from `SELLER_ACCEPTED` to `FINALIZED`. Upon calling this endpoint, the buyer implicitly agrees to the terms and conditions optionally set within the proposal by the publisher.");
            proposals1 = proposals1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("add_note").about("Create a new note and attach it to the proposal. The note is assigned a unique ID by the server. The proposal revision number will not increase when associated with a new note.");
            proposals1 = proposals1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("cancel_negotiation").about("Cancel an ongoing negotiation on a proposal. This does not cancel or end serving for the deals if the proposal has been finalized, but only cancels a negotiation unilaterally.");
            proposals1 = proposals1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("complete_setup").about("You can opt-in to manually update proposals to indicate that setup is complete. By default, proposal setup is automatically completed after their deals are finalized. Contact your Technical Account Manager to opt in. Buyers can call this method when the proposal has been finalized, and all the required creatives have been uploaded using the Creatives API. This call updates the `is_setup_completed` field on the deals in the proposal, and notifies the seller. The server then advances the revision number of the most recent proposal. To mark an individual deal as ready to serve, call `buyers.finalizedDeals.setReadyToServe` in the Marketplace API.");
            proposals1 = proposals1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Create the given proposal. Each created proposal and any deals it contains are assigned a unique ID by the server.");
            proposals1 = proposals1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about(
                "Gets a proposal given its ID. The proposal is returned at its head revision.",
            );
            proposals1 = proposals1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List proposals. A filter expression (PQL query) may be specified to filter the results. To retrieve all finalized proposals, regardless if a proposal is being renegotiated, see the FinalizedProposals resource. Note that Bidder/ChildSeat relationships differ from the usual behavior. A Bidder account can only see its child seats' proposals by specifying the ChildSeat's accountId in the request path.");
            proposals1 = proposals1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("pause").about("Update the given proposal to pause serving. This method will set the `DealServingMetadata.DealPauseStatus.has_buyer_paused` bit to true for all deals in the proposal. It is a no-op to pause an already-paused proposal. It is an error to call PauseProposal for a proposal that is not finalized or renegotiating.");
            proposals1 = proposals1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("resume").about("Update the given proposal to resume serving. This method will set the `DealServingMetadata.DealPauseStatus.has_buyer_paused` bit to false for all deals in the proposal. Note that if the `has_seller_paused` bit is also set, serving will not resume until the seller also resumes. It is a no-op to resume an already-running proposal. It is an error to call ResumeProposal for a proposal that is not finalized or renegotiating.");
            proposals1 = proposals1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about("Update the given proposal at the client known revision number. If the server revision has advanced since the passed-in `proposal.proposal_revision`, an `ABORTED` error message will be returned. Only the buyer-modifiable fields of the proposal will be updated. Note that the deals in the proposal will be updated to match the passed-in copy. If a passed-in deal does not have a `deal_id`, the server will assign a new unique ID and create the deal. If passed-in deal has a `deal_id`, it will be updated to match the passed-in copy. Any existing deals not present in the passed-in proposal will be deleted. It is an error to pass in a deal with a `deal_id` not present at head.");
            proposals1 = proposals1.subcommand(mcmd);
        }
        let mut publisher_profiles1 = SubCommand::with_name("publisher_profiles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets the requested publisher profile by id.");
            publisher_profiles1 = publisher_profiles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List all publisher profiles visible to the buyer");
            publisher_profiles1 = publisher_profiles1.subcommand(mcmd);
        }
        let mut accounts1 = SubCommand::with_name("accounts")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: filter_sets");
        let mut filter_sets1 = SubCommand::with_name("filter_sets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about(
                "Creates the specified filter set for the account with the given account ID.",
            );
            filter_sets1 = filter_sets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about(
                "Deletes the requested filter set from the account with the given account ID.",
            );
            filter_sets1 = filter_sets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about(
                "Retrieves the requested filter set for the account with the given account ID.",
            );
            filter_sets1 = filter_sets1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all filter sets for the account with the given account ID.");
            filter_sets1 = filter_sets1.subcommand(mcmd);
        }
        let mut invitations2 = SubCommand::with_name("invitations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, get and list");
        {
            let mcmd = SubCommand::with_name("create").about("Creates and sends out an email invitation to access an Ad Exchange client buyer account.");
            invitations2 = invitations2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Retrieves an existing client user invitation.");
            invitations2 = invitations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists all the client users invitations for a client with a given account ID.",
            );
            invitations2 = invitations2.subcommand(mcmd);
        }
        let mut users2 = SubCommand::with_name("users")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get, list and update");
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves an existing client user.");
            users2 = users2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about(
                "Lists all the known client users for a specified sponsor buyer account ID.",
            );
            users2 = users2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update").about(
                "Updates an existing client user. Only the user status can be changed on update.",
            );
            users2 = users2.subcommand(mcmd);
        }
        let mut deal_associations2 = SubCommand::with_name("deal_associations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: add, list and remove");
        {
            let mcmd =
                SubCommand::with_name("add").about("Associate an existing deal with a creative.");
            deal_associations2 = deal_associations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("List all creative-deal associations.");
            deal_associations2 = deal_associations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove")
                .about("Remove the association between a deal and a creative.");
            deal_associations2 = deal_associations2.subcommand(mcmd);
        }
        let mut filter_sets2 = SubCommand::with_name("filter_sets")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get and list");
        {
            let mcmd = SubCommand::with_name("create").about(
                "Creates the specified filter set for the account with the given account ID.",
            );
            filter_sets2 = filter_sets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about(
                "Deletes the requested filter set from the account with the given account ID.",
            );
            filter_sets2 = filter_sets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about(
                "Retrieves the requested filter set for the account with the given account ID.",
            );
            filter_sets2 = filter_sets2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all filter sets for the account with the given account ID.");
            filter_sets2 = filter_sets2.subcommand(mcmd);
        }
        let mut bid_metrics2 = SubCommand::with_name("bid_metrics")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all metrics that are measured in terms of number of bids.");
            bid_metrics2 = bid_metrics2.subcommand(mcmd);
        }
        let mut bid_response_errors2 = SubCommand::with_name("bid_response_errors")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List all errors that occurred in bid responses, with the number of bid responses affected for each reason.");
            bid_response_errors2 = bid_response_errors2.subcommand(mcmd);
        }
        let mut bid_responses_without_bids2 = SubCommand::with_name("bid_responses_without_bids")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List all reasons for which bid responses were considered to have no applicable bids, with the number of bid responses affected for each reason.");
            bid_responses_without_bids2 = bid_responses_without_bids2.subcommand(mcmd);
        }
        let mut filtered_bid_requests2 = SubCommand::with_name("filtered_bid_requests")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List all reasons that caused a bid request not to be sent for an impression, with the number of bid requests not sent for each reason.");
            filtered_bid_requests2 = filtered_bid_requests2.subcommand(mcmd);
        }
        let mut filtered_bids2 = SubCommand::with_name("filtered_bids")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List all reasons for which bids were filtered, with the number of bids filtered for each reason.");
            filtered_bids2 = filtered_bids2.subcommand(mcmd);
        }
        let mut impression_metrics2 = SubCommand::with_name("impression_metrics")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all metrics that are measured in terms of number of impressions.");
            impression_metrics2 = impression_metrics2.subcommand(mcmd);
        }
        let mut losing_bids2 = SubCommand::with_name("losing_bids")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List all reasons for which bids lost in the auction, with the number of bids that lost for each reason.");
            losing_bids2 = losing_bids2.subcommand(mcmd);
        }
        let mut non_billable_winning_bids2 = SubCommand::with_name("non_billable_winning_bids")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List all reasons for which winning bids were not billable, with the number of bids not billed for each reason.");
            non_billable_winning_bids2 = non_billable_winning_bids2.subcommand(mcmd);
        }
        let mut bid_metrics3 = SubCommand::with_name("bid_metrics")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all metrics that are measured in terms of number of bids.");
            bid_metrics3 = bid_metrics3.subcommand(mcmd);
        }
        let mut bid_response_errors3 = SubCommand::with_name("bid_response_errors")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List all errors that occurred in bid responses, with the number of bid responses affected for each reason.");
            bid_response_errors3 = bid_response_errors3.subcommand(mcmd);
        }
        let mut bid_responses_without_bids3 = SubCommand::with_name("bid_responses_without_bids")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List all reasons for which bid responses were considered to have no applicable bids, with the number of bid responses affected for each reason.");
            bid_responses_without_bids3 = bid_responses_without_bids3.subcommand(mcmd);
        }
        let mut filtered_bid_requests3 = SubCommand::with_name("filtered_bid_requests")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List all reasons that caused a bid request not to be sent for an impression, with the number of bid requests not sent for each reason.");
            filtered_bid_requests3 = filtered_bid_requests3.subcommand(mcmd);
        }
        let mut filtered_bids3 = SubCommand::with_name("filtered_bids")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List all reasons for which bids were filtered, with the number of bids filtered for each reason.");
            filtered_bids3 = filtered_bids3.subcommand(mcmd);
        }
        let mut impression_metrics3 = SubCommand::with_name("impression_metrics")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all metrics that are measured in terms of number of impressions.");
            impression_metrics3 = impression_metrics3.subcommand(mcmd);
        }
        let mut losing_bids3 = SubCommand::with_name("losing_bids")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List all reasons for which bids lost in the auction, with the number of bids that lost for each reason.");
            losing_bids3 = losing_bids3.subcommand(mcmd);
        }
        let mut non_billable_winning_bids3 = SubCommand::with_name("non_billable_winning_bids")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List all reasons for which winning bids were not billable, with the number of bids not billed for each reason.");
            non_billable_winning_bids3 = non_billable_winning_bids3.subcommand(mcmd);
        }
        let mut creatives3 = SubCommand::with_name("creatives")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List all creatives associated with a specific reason for which bids were filtered, with the number of bids filtered for each creative.");
            creatives3 = creatives3.subcommand(mcmd);
        }
        let mut details3 = SubCommand::with_name("details")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List all details associated with a specific reason for which bids were filtered, with the number of bids filtered for each detail.");
            details3 = details3.subcommand(mcmd);
        }
        let mut creatives4 = SubCommand::with_name("creatives")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List all creatives associated with a specific reason for which bids were filtered, with the number of bids filtered for each creative.");
            creatives4 = creatives4.subcommand(mcmd);
        }
        let mut details4 = SubCommand::with_name("details")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("List all details associated with a specific reason for which bids were filtered, with the number of bids filtered for each detail.");
            details4 = details4.subcommand(mcmd);
        }
        filtered_bids3 = filtered_bids3.subcommand(details4);
        filtered_bids3 = filtered_bids3.subcommand(creatives4);
        filtered_bids2 = filtered_bids2.subcommand(details3);
        filtered_bids2 = filtered_bids2.subcommand(creatives3);
        filter_sets2 = filter_sets2.subcommand(non_billable_winning_bids3);
        filter_sets2 = filter_sets2.subcommand(losing_bids3);
        filter_sets2 = filter_sets2.subcommand(impression_metrics3);
        filter_sets2 = filter_sets2.subcommand(filtered_bids3);
        filter_sets2 = filter_sets2.subcommand(filtered_bid_requests3);
        filter_sets2 = filter_sets2.subcommand(bid_responses_without_bids3);
        filter_sets2 = filter_sets2.subcommand(bid_response_errors3);
        filter_sets2 = filter_sets2.subcommand(bid_metrics3);
        filter_sets1 = filter_sets1.subcommand(non_billable_winning_bids2);
        filter_sets1 = filter_sets1.subcommand(losing_bids2);
        filter_sets1 = filter_sets1.subcommand(impression_metrics2);
        filter_sets1 = filter_sets1.subcommand(filtered_bids2);
        filter_sets1 = filter_sets1.subcommand(filtered_bid_requests2);
        filter_sets1 = filter_sets1.subcommand(bid_responses_without_bids2);
        filter_sets1 = filter_sets1.subcommand(bid_response_errors2);
        filter_sets1 = filter_sets1.subcommand(bid_metrics2);
        accounts1 = accounts1.subcommand(filter_sets2);
        creatives1 = creatives1.subcommand(deal_associations2);
        clients1 = clients1.subcommand(users2);
        clients1 = clients1.subcommand(invitations2);
        bidders0 = bidders0.subcommand(filter_sets1);
        bidders0 = bidders0.subcommand(accounts1);
        accounts0 = accounts0.subcommand(publisher_profiles1);
        accounts0 = accounts0.subcommand(proposals1);
        accounts0 = accounts0.subcommand(products1);
        accounts0 = accounts0.subcommand(finalized_proposals1);
        accounts0 = accounts0.subcommand(creatives1);
        accounts0 = accounts0.subcommand(clients1);
        app = app.subcommand(bidders0);
        app = app.subcommand(accounts0);

        Self { app }
    }
}
use google_adexchangebuyer2_beta1 as api;

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
