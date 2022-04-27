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
        let mut app = App::new("authorizedbuyersmarketplace1")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20220426")
            .about("The Authorized Buyers Marketplace API lets buyers programmatically discover inventory; propose, retrieve and negotiate deals with publishers.")
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
        let mut bidders0 = SubCommand::with_name("bidders")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: finalized_deals");
        let mut buyers0 = SubCommand::with_name("buyers")
                        .setting(AppSettings::ColoredHelp)
                        .about("sub-resources: auction_packages, clients, finalized_deals, proposals and publisher_profiles");
        let mut finalized_deals1 = SubCommand::with_name("finalized_deals")
            .setting(AppSettings::ColoredHelp)
            .about("methods: list");
        {
            let mcmd = SubCommand::with_name("list").about("Lists finalized deals. Use the URL path \"/v1/buyers/{accountId}/finalizedDeals\" to list finalized deals for the current buyer and its clients. Bidders can use the URL path \"/v1/bidders/{accountId}/finalizedDeals\" to list finalized deals for the bidder, its buyers and all their clients.");
            finalized_deals1 = finalized_deals1.subcommand(mcmd);
        }
        let mut auction_packages1 = SubCommand::with_name("auction_packages")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: get, list, subscribe, subscribe_clients, unsubscribe and unsubscribe_clients");
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets an auction package given its name.");
            auction_packages1 = auction_packages1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("List the auction packages subscribed by a buyer and its clients.");
            auction_packages1 = auction_packages1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("subscribe").about("Subscribe to the auction package for the specified buyer. Once subscribed, the bidder will receive a call out for inventory matching the auction package targeting criteria with the auction package deal ID and the specified buyer.");
            auction_packages1 = auction_packages1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("subscribe_clients").about("Subscribe the specified clients of the buyer to the auction package. If a client in the list does not belong to the buyer, an error response will be returned, and all of the following clients in the list will not be subscribed. Subscribing an already subscribed client will have no effect.");
            auction_packages1 = auction_packages1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("unsubscribe").about("Unsubscribe from the auction package for the specified buyer. Once unsubscribed, the bidder will no longer receive a call out for the auction package deal ID and the specified buyer.");
            auction_packages1 = auction_packages1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("unsubscribe_clients").about("Unsubscribe from the auction package for the specified clients of the buyer. Unsubscribing a client that is not subscribed will have no effect.");
            auction_packages1 = auction_packages1.subcommand(mcmd);
        }
        let mut clients1 = SubCommand::with_name("clients")
            .setting(AppSettings::ColoredHelp)
            .about("methods: activate, create, deactivate, get, list and patch");
        {
            let mcmd = SubCommand::with_name("activate").about("Activates an existing client. The state of the client will be updated to \"ACTIVE\". This method has no effect if the client is already in \"ACTIVE\" state.");
            clients1 = clients1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new client.");
            clients1 = clients1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("deactivate").about("Deactivates an existing client. The state of the client will be updated to \"INACTIVE\". This method has no effect if the client is already in \"INACTIVE\" state.");
            clients1 = clients1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets a client with a given resource name.");
            clients1 = clients1.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all the clients for the current buyer.");
            clients1 = clients1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates an existing client.");
            clients1 = clients1.subcommand(mcmd);
        }
        let mut finalized_deals1 = SubCommand::with_name("finalized_deals")
            .setting(AppSettings::ColoredHelp)
            .about("methods: add_creative, get, list, pause, resume and set_ready_to_serve");
        {
            let mcmd = SubCommand::with_name("add_creative").about("Add creative to be used in the bidding process for a finalized deal. For programmatic guaranteed deals, it's recommended that you associate at least one approved creative with the deal before calling SetReadyToServe, to help reduce the number of bid responses filtered because they don't contain approved creatives. Creatives successfully added to a deal can be found in the Realtime-bidding Creatives API creative.deal_ids. This method only applies to programmatic guaranteed deals. Maximum number of 1000 creatives can be added to a finalized deal.");
            finalized_deals1 = finalized_deals1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a finalized deal given its name.");
            finalized_deals1 = finalized_deals1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists finalized deals. Use the URL path \"/v1/buyers/{accountId}/finalizedDeals\" to list finalized deals for the current buyer and its clients. Bidders can use the URL path \"/v1/bidders/{accountId}/finalizedDeals\" to list finalized deals for the bidder, its buyers and all their clients.");
            finalized_deals1 = finalized_deals1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("pause").about("Pauses serving of the given finalized deal. This call only pauses the serving status, and does not affect other fields of the finalized deal. Calling this method for an already paused deal has no effect. This method only applies to programmatic guaranteed deals.");
            finalized_deals1 = finalized_deals1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("resume").about("Resumes serving of the given finalized deal. Calling this method for an running deal has no effect. If a deal is initially paused by the seller, calling this method will not resume serving of the deal until the seller also resumes the deal. This method only applies to programmatic guaranteed deals.");
            finalized_deals1 = finalized_deals1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_ready_to_serve").about("Sets the given finalized deal as ready to serve. By default, deals are ready to serve as soon as they're finalized. A bidder can opt out of this feature by asking to be included in an allowlist. Once opted out, finalized deals belonging to the bidder and its child seats will not start serving until this method is called. This method is useful to the bidders who prefer to not receive bid requests before the creative is ready. This method only applies to programmatic guaranteed deals.");
            finalized_deals1 = finalized_deals1.subcommand(mcmd);
        }
        let mut proposals1 = SubCommand::with_name("proposals")
            .setting(AppSettings::ColoredHelp)
            .about("methods: accept, add_note, cancel_negotiation, get, list, patch and send_rfp");
        {
            let mcmd = SubCommand::with_name("accept").about("Accepts the proposal at the given revision number. If the revision number in the request is behind the latest from the server, an error message will be returned. This call updates the Proposal.state from `BUYER_ACCEPTANCE_REQUESTED` to `FINALIZED`; it has no side effect if the Proposal.state is already `FINALIZED` and throws exception if the Proposal.state is not either `BUYER_ACCEPTANCE_REQUESTED` or `FINALIZED`. Accepting a proposal means the buyer understands and accepts the Proposal.terms_and_conditions proposed by the seller.");
            proposals1 = proposals1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("add_note")
                .about("Creates a note for this proposal and sends to the seller.");
            proposals1 = proposals1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("cancel_negotiation").about("Cancels an ongoing negotiation on a proposal. This does not cancel or end serving for the deals if the proposal has been finalized. If the proposal has not been finalized before, calling this method will set the Proposal.state to `TERMINATED` and increment the Proposal.proposal_revision. If the proposal has been finalized before and is under renegotiation now, calling this method will reset the Proposal.state to `FINALIZED` and increment the Proposal.proposal_revision. This method does not support private auction proposals whose Proposal.deal_type is 'PRIVATE_AUCTION'.");
            proposals1 = proposals1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a proposal using its name. The proposal is returned at most recent revision. revision.");
            proposals1 = proposals1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists proposals. A filter expression (list filter syntax) may be specified to filter the results. This will not list finalized versions of proposals that are being renegotiated; to retrieve these use the finalizedProposals resource.");
            proposals1 = proposals1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the proposal at the given revision number. If the revision number in the request is behind the latest from the server, an error message will be returned. See FieldMask for how to use FieldMask. Only fields specified in the UpdateProposalRequest.update_mask will be updated; Fields noted as 'Immutable' or 'Output only' yet specified in the UpdateProposalRequest.update_mask will be ignored and left unchanged. Updating a private auction proposal is not allowed and will result in an error.");
            proposals1 = proposals1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("send_rfp").about("Sends a request for proposal (RFP) to a publisher to initiate the negotiation regarding certain inventory. In the RFP, buyers can specify the deal type, deal terms, start and end dates, targeting, and a message to the publisher. Once the RFP is sent, a proposal in `SELLER_REVIEW_REQUESTED` state will be created and returned in the response. The publisher may review your request and respond with detailed deals in the proposal.");
            proposals1 = proposals1.subcommand(mcmd);
        }
        let mut publisher_profiles1 = SubCommand::with_name("publisher_profiles")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd =
                SubCommand::with_name("get").about("Gets the requested publisher profile by name.");
            publisher_profiles1 = publisher_profiles1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists publisher profiles");
            publisher_profiles1 = publisher_profiles1.subcommand(mcmd);
        }
        let mut users2 = SubCommand::with_name("users")
            .setting(AppSettings::ColoredHelp)
            .about("methods: activate, create, deactivate, delete, get and list");
        {
            let mcmd = SubCommand::with_name("activate").about("Activates an existing client user. The state of the client user will be updated from \"INACTIVE\" to \"ACTIVE\". This method has no effect if the client user is already in \"ACTIVE\" state. An error will be returned if the client user to activate is still in \"INVITED\" state.");
            users2 = users2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a new client user in \"INVITED\" state. An email invitation will be sent to the new user, once accepted the user will become active.");
            users2 = users2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("deactivate").about("Deactivates an existing client user. The state of the client user will be updated from \"ACTIVE\" to \"INACTIVE\". This method has no effect if the client user is already in \"INACTIVE\" state. An error will be returned if the client user to deactivate is still in \"INVITED\" state.");
            users2 = users2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes an existing client user. The client user will lose access to the Authorized Buyers UI. Note that if a client user is deleted, the user's access to the UI can't be restored unless a new client user is created and activated.");
            users2 = users2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Retrieves an existing client user.");
            users2 = users2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all client users for a specified client.");
            users2 = users2.subcommand(mcmd);
        }
        let mut deals2 = SubCommand::with_name("deals")
            .setting(AppSettings::ColoredHelp)
            .about("methods: batch_update, get, list and patch");
        {
            let mcmd = SubCommand::with_name("batch_update")
                .about("Batch updates multiple deals in the same proposal.");
            deals2 = deals2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get")
                .about("Gets a deal given its name. The deal is returned at its head revision.");
            deals2 = deals2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists all deals in a proposal. To retrieve only the finalized revision deals regardless if a deal is being renegotiated, see the FinalizedDeals resource.");
            deals2 = deals2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the given deal at the buyer known revision number. If the server revision has advanced since the passed-in proposal.proposal_revision an ABORTED error message will be returned. The revision number is incremented by the server whenever the proposal or its constituent deals are updated. Note: The revision number is kept at a proposal level. The buyer of the API is expected to keep track of the revision number after the last update operation and send it in as part of the next update request. This way, if there are further changes on the server (for example, seller making new updates), then the server can detect conflicts and reject the proposed changes.");
            deals2 = deals2.subcommand(mcmd);
        }
        proposals1 = proposals1.subcommand(deals2);
        clients1 = clients1.subcommand(users2);
        buyers0 = buyers0.subcommand(publisher_profiles1);
        buyers0 = buyers0.subcommand(proposals1);
        buyers0 = buyers0.subcommand(finalized_deals1);
        buyers0 = buyers0.subcommand(clients1);
        buyers0 = buyers0.subcommand(auction_packages1);
        bidders0 = bidders0.subcommand(finalized_deals1);
        app = app.subcommand(buyers0);
        app = app.subcommand(bidders0);

        Self { app }
    }
}
use google_authorizedbuyersmarketplace1 as api;

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
