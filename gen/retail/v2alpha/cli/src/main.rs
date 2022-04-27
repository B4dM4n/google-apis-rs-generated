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
        let mut app = App::new("retail2_alpha")
            .setting(clap::AppSettings::ColoredHelp)
            .author("Sebastian Thiel <byronimo@gmail.com>")
            .version("0.1.0-20220414")
            .about("Cloud Retail service enables customers to build end-to-end personalized recommendation systems without requiring a high level of expertise in machine learning, recommendation system, or Google Cloud.")
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
            .about("sub-resources: locations and operations");
        let mut locations1 = SubCommand::with_name("locations")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: catalogs and operations");
        let mut operations1 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations1 = operations1.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations1 = operations1.subcommand(mcmd);
        }
        let mut catalogs2 = SubCommand::with_name("catalogs")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: complete_query, get_attributes_config, get_completion_config, get_default_branch, list, patch, set_default_branch, update_attributes_config and update_completion_config");
        {
            let mcmd = SubCommand::with_name("complete_query").about("Completes the specified prefix with keyword suggestions. This feature is only available for users who have Retail Search enabled. Please enable Retail Search on Cloud Console before using this feature.");
            catalogs2 = catalogs2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get_attributes_config").about("Gets an AttributesConfig.");
            catalogs2 = catalogs2.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("get_completion_config").about("Gets a CompletionConfig.");
            catalogs2 = catalogs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get_default_branch").about("Get which branch is currently default branch set by CatalogService.SetDefaultBranch method under a specified parent catalog.");
            catalogs2 = catalogs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all the Catalogs associated with the project.");
            catalogs2 = catalogs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates the Catalogs.");
            catalogs2 = catalogs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_default_branch").about("Set a specified branch id as default branch. API methods such as SearchService.Search, ProductService.GetProduct, ProductService.ListProducts will treat requests using \"default_branch\" to the actual branch id set as default. For example, if `projects/*/locations/*/catalogs/*/branches/1` is set as default, setting SearchRequest.branch to `projects/*/locations/*/catalogs/*/branches/default_branch` is equivalent to setting SearchRequest.branch to `projects/*/locations/*/catalogs/*/branches/1`. Using multiple branches can be useful when developers would like to have a staging branch to test and verify for future usage. When it becomes ready, developers switch on the staging branch using this API while keeping using `projects/*/locations/*/catalogs/*/branches/default_branch` as SearchRequest.branch to route the traffic to this staging branch. CAUTION: If you have live predict/search traffic, switching the default branch could potentially cause outages if the ID space of the new branch is very different from the old one. More specifically: * PredictionService will only return product IDs from branch {newBranch}. * SearchService will only return product IDs from branch {newBranch} (if branch is not explicitly set). * UserEventService will only join events with products from branch {newBranch}.");
            catalogs2 = catalogs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_attributes_config").about("Updates the AttributesConfig. The catalog attributes in the request will be updated in the catalog, or inserted if they do not exist. Existing catalog attributes not included in the request will remain unchanged. Attributes that are assigned to products, but do not exist at the catalog level, are always included in the response. The product attribute is assigned default values for missing catalog attribute fields, e.g., searchable and dynamic facetable options.");
            catalogs2 = catalogs2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("update_completion_config")
                .about("Updates the CompletionConfigs.");
            catalogs2 = catalogs2.subcommand(mcmd);
        }
        let mut operations2 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations2 = operations2.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations2 = operations2.subcommand(mcmd);
        }
        let mut attributes_config3 = SubCommand::with_name("attributes_config")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: add_catalog_attribute, remove_catalog_attribute and replace_catalog_attribute");
        {
            let mcmd = SubCommand::with_name("add_catalog_attribute").about("Adds the specified CatalogAttribute to the AttributesConfig. If the CatalogAttribute to add already exists, an ALREADY_EXISTS error is returned.");
            attributes_config3 = attributes_config3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_catalog_attribute").about("Removes the specified CatalogAttribute from the AttributesConfig. If the CatalogAttribute to remove does not exist, a NOT_FOUND error is returned.");
            attributes_config3 = attributes_config3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("replace_catalog_attribute").about("Replaces the specified CatalogAttribute in the AttributesConfig by updating the catalog attribute with the same CatalogAttribute.key. If the CatalogAttribute to replace does not exist, a NOT_FOUND error is returned.");
            attributes_config3 = attributes_config3.subcommand(mcmd);
        }
        let mut branches3 = SubCommand::with_name("branches")
            .setting(AppSettings::ColoredHelp)
            .about("sub-resources: operations and products");
        let mut completion_data3 = SubCommand::with_name("completion_data")
            .setting(AppSettings::ColoredHelp)
            .about("methods: import");
        {
            let mcmd = SubCommand::with_name("import").about("Bulk import of processed completion dataset. Request processing is asynchronous. Partial updating is not supported. The operation is successfully finished only after the imported suggestions are indexed successfully and ready for serving. The process takes hours. This feature is only available for users who have Retail Search enabled. Please enable Retail Search on Cloud Console before using this feature.");
            completion_data3 = completion_data3.subcommand(mcmd);
        }
        let mut controls3 = SubCommand::with_name("controls")
            .setting(AppSettings::ColoredHelp)
            .about("methods: create, delete, get, list and patch");
        {
            let mcmd = SubCommand::with_name("create").about("Creates a Control. If the Control to create already exists, an ALREADY_EXISTS error is returned.");
            controls3 = controls3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a Control. If the Control to delete does not exist, a NOT_FOUND error is returned.");
            controls3 = controls3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a Control.");
            controls3 = controls3.subcommand(mcmd);
        }
        {
            let mcmd =
                SubCommand::with_name("list").about("Lists all Controls linked to this catalog.");
            controls3 = controls3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a Control. Control cannot be set to a different oneof field, if so an INVALID_ARGUMENT is returned. If the Control to delete does not exist, a NOT_FOUND error is returned.");
            controls3 = controls3.subcommand(mcmd);
        }
        let mut operations3 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get and list");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations3 = operations3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id.");
            operations3 = operations3.subcommand(mcmd);
        }
        let mut placements3 = SubCommand::with_name("placements")
            .setting(AppSettings::ColoredHelp)
            .about("methods: predict and search");
        {
            let mcmd = SubCommand::with_name("predict").about("Makes a recommendation prediction.");
            placements3 = placements3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search").about("Performs a search. This feature is only available for users who have Retail Search enabled. Please enable Retail Search on Cloud Console before using this feature.");
            placements3 = placements3.subcommand(mcmd);
        }
        let mut serving_configs3 = SubCommand::with_name("serving_configs")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: add_control, create, delete, get, list, patch, predict, remove_control and search");
        {
            let mcmd = SubCommand::with_name("add_control").about("Enables a Control on the specified ServingConfig. The control is added in the last position of the list of controls it belongs to (e.g. if it's a facet spec control it will be applied in the last position of servingConfig.facetSpecIds) Returns a ALREADY_EXISTS error if the control has already been applied. Returns a FAILED_PRECONDITION error if the addition could exceed maximum number of control allowed for that type of control.");
            serving_configs3 = serving_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a ServingConfig. A maximum of 100 ServingConfigs are allowed in a Catalog, otherwise a FAILED_PRECONDITION error is returned.");
            serving_configs3 = serving_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a ServingConfig. Returns a NotFound error if the ServingConfig does not exist.");
            serving_configs3 = serving_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a ServingConfig. Returns a NotFound error if the ServingConfig does not exist.");
            serving_configs3 = serving_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list")
                .about("Lists all ServingConfigs linked to this catalog.");
            serving_configs3 = serving_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a ServingConfig.");
            serving_configs3 = serving_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("predict").about("Makes a recommendation prediction.");
            serving_configs3 = serving_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_control").about("Disables a Control on the specified ServingConfig. The control is removed from the ServingConfig. Returns a NOT_FOUND error if the Control is not enabled for the ServingConfig.");
            serving_configs3 = serving_configs3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("search").about("Performs a search. This feature is only available for users who have Retail Search enabled. Please enable Retail Search on Cloud Console before using this feature.");
            serving_configs3 = serving_configs3.subcommand(mcmd);
        }
        let mut user_events3 = SubCommand::with_name("user_events")
            .setting(AppSettings::ColoredHelp)
            .about("methods: collect, import, purge, rejoin and write");
        {
            let mcmd = SubCommand::with_name("collect").about("Writes a single user event from the browser. This uses a GET request to due to browser restriction of POST-ing to a 3rd party domain. This method is used only by the Retail API JavaScript pixel and Google Tag Manager. Users should not call this method directly.");
            user_events3 = user_events3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Bulk import of User events. Request processing might be synchronous. Events that already exist are skipped. Use this method for backfilling historical user events. Operation.response is of type ImportResponse. Note that it is possible for a subset of the items to be successfully inserted. Operation.metadata is of type ImportMetadata.");
            user_events3 = user_events3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("purge").about("Deletes permanently all user events specified by the filter provided. Depending on the number of events specified by the filter, this operation could take hours or days to complete. To test a filter, use the list command first.");
            user_events3 = user_events3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("rejoin").about("Starts a user event rejoin operation with latest product catalog. Events will not be annotated with detailed product information if product is missing from the catalog at the time the user event is ingested, and these events are stored as unjoined events with a limited usage on training and serving. This method can be used to start a join operation on specified events with latest version of product catalog. It can also be used to correct events joined with the wrong product catalog. A rejoin operation can take hours or days to complete.");
            user_events3 = user_events3.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("write").about("Writes a single user event.");
            user_events3 = user_events3.subcommand(mcmd);
        }
        let mut operations4 = SubCommand::with_name("operations")
            .setting(AppSettings::ColoredHelp)
            .about("methods: get");
        {
            let mcmd = SubCommand::with_name("get").about("Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.");
            operations4 = operations4.subcommand(mcmd);
        }
        let mut products4 = SubCommand::with_name("products")
                        .setting(AppSettings::ColoredHelp)
                        .about("methods: add_fulfillment_places, add_local_inventories, create, delete, get, import, list, patch, purge, remove_fulfillment_places, remove_local_inventories and set_inventory");
        {
            let mcmd = SubCommand::with_name("add_fulfillment_places").about("Incrementally adds place IDs to Product.fulfillment_info.place_ids. This process is asynchronous and does not require the Product to exist before updating fulfillment information. If the request is valid, the update will be enqueued and processed downstream. As a consequence, when a response is returned, the added place IDs are not immediately manifested in the Product queried by GetProduct or ListProducts. This feature is only available for users who have Retail Search enabled. Please enable Retail Search on Cloud Console before using this feature.");
            products4 = products4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("add_local_inventories").about("Updates local inventory information for a Product at a list of places, while respecting the last update timestamps of each inventory field. This process is asynchronous and does not require the Product to exist before updating inventory information. If the request is valid, the update will be enqueued and processed downstream. As a consequence, when a response is returned, updates are not immediately manifested in the Product queried by GetProduct or ListProducts. Local inventory information can only be modified using this method. CreateProduct and UpdateProduct has no effect on local inventories. This feature is only available for users who have Retail Search enabled. Please enable Retail Search on Cloud Console before using this feature.");
            products4 = products4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("create").about("Creates a Product.");
            products4 = products4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("delete").about("Deletes a Product.");
            products4 = products4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("get").about("Gets a Product.");
            products4 = products4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("import").about("Bulk import of multiple Products. Request processing may be synchronous. No partial updating is supported. Non-existing items are created. Note that it is possible for a subset of the Products to be successfully updated.");
            products4 = products4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("list").about("Gets a list of Products.");
            products4 = products4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("patch").about("Updates a Product.");
            products4 = products4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("purge").about("Permanently deletes all selected Products under a branch. This process is asynchronous. If the request is valid, the removal will be enqueued and processed offline. Depending on the number of Products, this operation could take hours to complete. Before the operation completes, some Products may still be returned by GetProduct or ListProducts. Depending on the number of Products, this operation could take hours to complete. To get a sample of Products that would be deleted, set PurgeProductsRequest.force to false.");
            products4 = products4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_fulfillment_places").about("Incrementally removes place IDs from a Product.fulfillment_info.place_ids. This process is asynchronous and does not require the Product to exist before updating fulfillment information. If the request is valid, the update will be enqueued and processed downstream. As a consequence, when a response is returned, the removed place IDs are not immediately manifested in the Product queried by GetProduct or ListProducts. This feature is only available for users who have Retail Search enabled. Please enable Retail Search on Cloud Console before using this feature.");
            products4 = products4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("remove_local_inventories").about("Remove local inventory information for a Product at a list of places at a removal timestamp. This process is asynchronous. If the request is valid, the removal will be enqueued and processed downstream. As a consequence, when a response is returned, removals are not immediately manifested in the Product queried by GetProduct or ListProducts. Local inventory information can only be removed using this method. CreateProduct and UpdateProduct has no effect on local inventories. This feature is only available for users who have Retail Search enabled. Please enable Retail Search on Cloud Console before using this feature.");
            products4 = products4.subcommand(mcmd);
        }
        {
            let mcmd = SubCommand::with_name("set_inventory").about("Updates inventory information for a Product while respecting the last update timestamps of each inventory field. This process is asynchronous and does not require the Product to exist before updating fulfillment information. If the request is valid, the update will be enqueued and processed downstream. As a consequence, when a response is returned, updates are not immediately manifested in the Product queried by GetProduct or ListProducts. When inventory is updated with CreateProduct and UpdateProduct, the specified inventory field value(s) will overwrite any existing value(s) while ignoring the last update time for this field. Furthermore, the last update time for the specified inventory fields will be overwritten to the time of the CreateProduct or UpdateProduct request. If no inventory fields are set in CreateProductRequest.product, then any pre-existing inventory information for this product will be used. If no inventory fields are set in SetInventoryRequest.set_mask, then any existing inventory information will be preserved. Pre-existing inventory information can only be updated with SetInventory, AddFulfillmentPlaces, and RemoveFulfillmentPlaces. This feature is only available for users who have Retail Search enabled. Please enable Retail Search on Cloud Console before using this feature.");
            products4 = products4.subcommand(mcmd);
        }
        branches3 = branches3.subcommand(products4);
        branches3 = branches3.subcommand(operations4);
        catalogs2 = catalogs2.subcommand(user_events3);
        catalogs2 = catalogs2.subcommand(serving_configs3);
        catalogs2 = catalogs2.subcommand(placements3);
        catalogs2 = catalogs2.subcommand(operations3);
        catalogs2 = catalogs2.subcommand(controls3);
        catalogs2 = catalogs2.subcommand(completion_data3);
        catalogs2 = catalogs2.subcommand(branches3);
        catalogs2 = catalogs2.subcommand(attributes_config3);
        locations1 = locations1.subcommand(operations2);
        locations1 = locations1.subcommand(catalogs2);
        projects0 = projects0.subcommand(operations1);
        projects0 = projects0.subcommand(locations1);
        app = app.subcommand(projects0);

        Self { app }
    }
}
use google_retail2_alpha as api;

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
