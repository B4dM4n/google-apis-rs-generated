#![doc = "# Resources and Methods\n    * [projects](resources/projects/struct.ProjectsActions.html)\n      * [locations](resources/projects/locations/struct.LocationsActions.html)\n        * [catalogs](resources/projects/locations/catalogs/struct.CatalogsActions.html)\n          * [*list*](resources/projects/locations/catalogs/struct.ListRequestBuilder.html), [*patch*](resources/projects/locations/catalogs/struct.PatchRequestBuilder.html)\n          * [catalog_items](resources/projects/locations/catalogs/catalog_items/struct.CatalogItemsActions.html)\n            * [*create*](resources/projects/locations/catalogs/catalog_items/struct.CreateRequestBuilder.html), [*delete*](resources/projects/locations/catalogs/catalog_items/struct.DeleteRequestBuilder.html), [*get*](resources/projects/locations/catalogs/catalog_items/struct.GetRequestBuilder.html), [*import*](resources/projects/locations/catalogs/catalog_items/struct.ImportRequestBuilder.html), [*list*](resources/projects/locations/catalogs/catalog_items/struct.ListRequestBuilder.html), [*patch*](resources/projects/locations/catalogs/catalog_items/struct.PatchRequestBuilder.html)\n          * [event_stores](resources/projects/locations/catalogs/event_stores/struct.EventStoresActions.html)\n            * [operations](resources/projects/locations/catalogs/event_stores/operations/struct.OperationsActions.html)\n              * [*get*](resources/projects/locations/catalogs/event_stores/operations/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/catalogs/event_stores/operations/struct.ListRequestBuilder.html)\n            * [placements](resources/projects/locations/catalogs/event_stores/placements/struct.PlacementsActions.html)\n              * [*predict*](resources/projects/locations/catalogs/event_stores/placements/struct.PredictRequestBuilder.html)\n            * [prediction_api_key_registrations](resources/projects/locations/catalogs/event_stores/prediction_api_key_registrations/struct.PredictionApiKeyRegistrationsActions.html)\n              * [*create*](resources/projects/locations/catalogs/event_stores/prediction_api_key_registrations/struct.CreateRequestBuilder.html), [*delete*](resources/projects/locations/catalogs/event_stores/prediction_api_key_registrations/struct.DeleteRequestBuilder.html), [*list*](resources/projects/locations/catalogs/event_stores/prediction_api_key_registrations/struct.ListRequestBuilder.html)\n            * [user_events](resources/projects/locations/catalogs/event_stores/user_events/struct.UserEventsActions.html)\n              * [*collect*](resources/projects/locations/catalogs/event_stores/user_events/struct.CollectRequestBuilder.html), [*import*](resources/projects/locations/catalogs/event_stores/user_events/struct.ImportRequestBuilder.html), [*list*](resources/projects/locations/catalogs/event_stores/user_events/struct.ListRequestBuilder.html), [*purge*](resources/projects/locations/catalogs/event_stores/user_events/struct.PurgeRequestBuilder.html), [*rejoin*](resources/projects/locations/catalogs/event_stores/user_events/struct.RejoinRequestBuilder.html), [*write*](resources/projects/locations/catalogs/event_stores/user_events/struct.WriteRequestBuilder.html)\n          * [operations](resources/projects/locations/catalogs/operations/struct.OperationsActions.html)\n            * [*get*](resources/projects/locations/catalogs/operations/struct.GetRequestBuilder.html), [*list*](resources/projects/locations/catalogs/operations/struct.ListRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
}
pub mod schemas {
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleApiHttpBody {
        #[doc = "The HTTP Content-Type header value specifying the content type of the body."]
        #[serde(
            rename = "contentType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_type: ::std::option::Option<String>,
        #[doc = "The HTTP request/response body as raw binary."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Application specific response metadata. Must be set in the first response for streaming APIs."]
        #[serde(
            rename = "extensions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub extensions:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleApiHttpBody {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleApiHttpBody {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1AlphaRejoinCatalogMetadata {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1AlphaRejoinCatalogMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1AlphaRejoinCatalogMetadata
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1AlphaRejoinCatalogResponse {
        #[doc = "Number of user events that were joined with latest catalog items."]
        #[serde(
            rename = "rejoinedUserEventsCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub rejoined_user_events_count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1AlphaRejoinCatalogResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1AlphaRejoinCatalogResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1AlphaTuningMetadata {
        #[doc = "The resource name of the recommendation model that this tune applies to. Format: projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/eventStores/{event_store_id}/recommendationModels/{recommendation_model_id}"]
        #[serde(
            rename = "recommendationModel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recommendation_model: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1AlphaTuningMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecommendationengineV1AlphaTuningMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1AlphaTuningResponse {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1AlphaTuningResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecommendationengineV1AlphaTuningResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1BigQuerySource {
        #[doc = "Optional. The schema to use when parsing the data from the source. Supported values for catalog imports: 1: \"catalog_recommendations_ai\" using https://cloud.google.com/recommendations-ai/docs/upload-catalog#json (Default for catalogItems.import) 2: \"catalog_merchant_center\" using https://cloud.google.com/recommendations-ai/docs/upload-catalog#mc Supported values for user event imports: 1: \"user_events_recommendations_ai\" using https://cloud.google.com/recommendations-ai/docs/manage-user-events#import (Default for userEvents.import) 2. \"user_events_ga360\" using https://support.google.com/analytics/answer/3437719?hl=en"]
        #[serde(
            rename = "dataSchema",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_schema: ::std::option::Option<String>,
        #[doc = "Required. The BigQuery data set to copy the data from."]
        #[serde(
            rename = "datasetId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dataset_id: ::std::option::Option<String>,
        #[doc = "Optional. Intermediate Cloud Storage directory used for the import. Can be specified if one wants to have the BigQuery export to a specific Cloud Storage directory."]
        #[serde(
            rename = "gcsStagingDir",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs_staging_dir: ::std::option::Option<String>,
        #[doc = "Optional. The project id (can be project # or id) that the BigQuery source is in. If not specified, inherits the project id from the parent request."]
        #[serde(
            rename = "projectId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project_id: ::std::option::Option<String>,
        #[doc = "Required. The BigQuery table to copy the data from."]
        #[serde(
            rename = "tableId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub table_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1BigQuerySource
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecommendationengineV1Beta1BigQuerySource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1Catalog {
        #[doc = "Required. The catalog item level configuration."]
        #[serde(
            rename = "catalogItemLevelConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub catalog_item_level_config: ::std::option::Option<
            crate::schemas::GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfig,
        >,
        #[doc = "Required. The ID of the default event store."]
        #[serde(
            rename = "defaultEventStoreId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_event_store_id: ::std::option::Option<String>,
        #[doc = "Required. The catalog display name."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The fully qualified resource name of the catalog."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudRecommendationengineV1Beta1Catalog {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecommendationengineV1Beta1Catalog {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1CatalogInlineSource {
        #[doc = "Optional. A list of catalog items to update/create. Recommended max of 10k items."]
        #[serde(
            rename = "catalogItems",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub catalog_items: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudRecommendationengineV1Beta1CatalogItem>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1CatalogInlineSource
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1CatalogInlineSource
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1CatalogItem {
        #[doc = "Required. Catalog item categories. This field is repeated for supporting one catalog item belonging to several parallel category hierarchies. For example, if a shoes product belongs to both [\"Shoes & Accessories\" -> \"Shoes\"] and [\"Sports & Fitness\" -> \"Athletic Clothing\" -> \"Shoes\"], it could be represented as: \"categoryHierarchies\": [ { \"categories\": [\"Shoes & Accessories\", \"Shoes\"]}, { \"categories\": [\"Sports & Fitness\", \"Athletic Clothing\", \"Shoes\"] } ]"]
        #[serde(
            rename = "categoryHierarchies",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category_hierarchies: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudRecommendationengineV1Beta1CatalogItemCategoryHierarchy>,
        >,
        #[doc = "Optional. Catalog item description. UTF-8 encoded string with a length limit of 5 KiB."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Required. Catalog item identifier. UTF-8 encoded string with a length limit of 128 bytes. This id must be unique among all catalog items within the same catalog. It should also be used when logging user events in order for the user events to be joined with the Catalog."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Optional. Highly encouraged. Extra catalog item attributes to be included in the recommendation model. For example, for retail products, this could include the store name, vendor, style, color, etc. These are very strong signals for recommendation model, thus we highly recommend providing the item attributes here."]
        #[serde(
            rename = "itemAttributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub item_attributes:
            ::std::option::Option<crate::schemas::GoogleCloudRecommendationengineV1Beta1FeatureMap>,
        #[doc = "Optional. Variant group identifier for prediction results. UTF-8 encoded string with a length limit of 128 bytes. This field must be enabled before it can be used. [Learn more](/recommendations-ai/docs/catalog#item-group-id)."]
        #[serde(
            rename = "itemGroupId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub item_group_id: ::std::option::Option<String>,
        #[doc = "Optional. Deprecated. The model automatically detects the text language. Your catalog can include text in different languages, but duplicating catalog items to provide text in multiple languages can result in degraded model performance."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Optional. Metadata specific to retail products."]
        #[serde(
            rename = "productMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub product_metadata: ::std::option::Option<
            crate::schemas::GoogleCloudRecommendationengineV1Beta1ProductCatalogItem,
        >,
        #[doc = "Optional. Filtering tags associated with the catalog item. Each tag should be a UTF-8 encoded string with a length limit of 1 KiB. This tag can be used for filtering recommendation results by passing the tag as part of the predict request filter."]
        #[serde(
            rename = "tags",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tags: ::std::option::Option<Vec<String>>,
        #[doc = "Required. Catalog item title. UTF-8 encoded string with a length limit of 1 KiB."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudRecommendationengineV1Beta1CatalogItem {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecommendationengineV1Beta1CatalogItem {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1CatalogItemCategoryHierarchy {
        #[doc = "Required. Catalog item categories. Each category should be a UTF-8 encoded string with a length limit of 2 KiB. Note that the order in the list denotes the specificity (from least to most specific)."]
        #[serde(
            rename = "categories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub categories: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1CatalogItemCategoryHierarchy
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1CatalogItemCategoryHierarchy
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfig { # [doc = "Optional. Level of the catalog at which events are uploaded. See https://cloud.google.com/recommendations-ai/docs/catalog#catalog-levels for more details."] # [serde (rename = "eventItemLevel" , default , skip_serializing_if = "std::option::Option::is_none")] pub event_item_level : :: std :: option :: Option < crate :: schemas :: GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigEventItemLevel > , # [doc = "Optional. Level of the catalog at which predictions are made. See https://cloud.google.com/recommendations-ai/docs/catalog#catalog-levels for more details."] # [serde (rename = "predictItemLevel" , default , skip_serializing_if = "std::option::Option::is_none")] pub predict_item_level : :: std :: option :: Option < crate :: schemas :: GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigPredictItemLevel > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfig
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfig
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigEventItemLevel {
        #[doc = "Unknown value - should never be used."]
        CatalogItemLevelUnspecified,
        #[doc = "Catalog items are at master level."]
        Master,
        #[doc = "Catalog items are at variant level."]
        Variant,
    }
    impl GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigEventItemLevel {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigEventItemLevel :: CatalogItemLevelUnspecified => "CATALOG_ITEM_LEVEL_UNSPECIFIED" , GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigEventItemLevel :: Master => "MASTER" , GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigEventItemLevel :: Variant => "VARIANT" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigEventItemLevel
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigEventItemLevel
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigEventItemLevel,
            (),
        > {
            Ok (match s { "CATALOG_ITEM_LEVEL_UNSPECIFIED" => GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigEventItemLevel :: CatalogItemLevelUnspecified , "MASTER" => GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigEventItemLevel :: Master , "VARIANT" => GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigEventItemLevel :: Variant , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigEventItemLevel
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigEventItemLevel
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigEventItemLevel
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CATALOG_ITEM_LEVEL_UNSPECIFIED" => GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigEventItemLevel :: CatalogItemLevelUnspecified , "MASTER" => GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigEventItemLevel :: Master , "VARIANT" => GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigEventItemLevel :: Variant , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigEventItemLevel
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigEventItemLevel
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigPredictItemLevel {
        #[doc = "Unknown value - should never be used."]
        CatalogItemLevelUnspecified,
        #[doc = "Catalog items are at master level."]
        Master,
        #[doc = "Catalog items are at variant level."]
        Variant,
    }
    impl GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigPredictItemLevel {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigPredictItemLevel :: CatalogItemLevelUnspecified => "CATALOG_ITEM_LEVEL_UNSPECIFIED" , GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigPredictItemLevel :: Master => "MASTER" , GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigPredictItemLevel :: Variant => "VARIANT" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigPredictItemLevel
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigPredictItemLevel
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigPredictItemLevel,
            (),
        > {
            Ok (match s { "CATALOG_ITEM_LEVEL_UNSPECIFIED" => GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigPredictItemLevel :: CatalogItemLevelUnspecified , "MASTER" => GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigPredictItemLevel :: Master , "VARIANT" => GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigPredictItemLevel :: Variant , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigPredictItemLevel
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigPredictItemLevel
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigPredictItemLevel
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CATALOG_ITEM_LEVEL_UNSPECIFIED" => GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigPredictItemLevel :: CatalogItemLevelUnspecified , "MASTER" => GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigPredictItemLevel :: Master , "VARIANT" => GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigPredictItemLevel :: Variant , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigPredictItemLevel
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1CatalogItemLevelConfigPredictItemLevel
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1CreatePredictionApiKeyRegistrationRequest {
        #[doc = "Required. The prediction API key registration."]
        #[serde(
            rename = "predictionApiKeyRegistration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub prediction_api_key_registration: ::std::option::Option<
            crate::schemas::GoogleCloudRecommendationengineV1Beta1PredictionApiKeyRegistration,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1CreatePredictionApiKeyRegistrationRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1CreatePredictionApiKeyRegistrationRequest
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1EventDetail {
        #[doc = "Optional. Extra user event features to include in the recommendation model. For product recommendation, an example of extra user information is traffic_channel, i.e. how user arrives at the site. Users can arrive at the site by coming to the site directly, or coming through Google search, and etc."]
        #[serde(
            rename = "eventAttributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub event_attributes:
            ::std::option::Option<crate::schemas::GoogleCloudRecommendationengineV1Beta1FeatureMap>,
        #[doc = "Optional. A list of identifiers for the independent experiment groups this user event belongs to. This is used to distinguish between user events associated with different experiment setups (e.g. using Recommendation Engine system, using different recommendation models)."]
        #[serde(
            rename = "experimentIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub experiment_ids: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. A unique id of a web page view. This should be kept the same for all user events triggered from the same pageview. For example, an item detail page view could trigger multiple events as the user is browsing the page. The `pageViewId` property should be kept the same for all these events so that they can be grouped together properly. This `pageViewId` will be automatically generated if using the JavaScript pixel."]
        #[serde(
            rename = "pageViewId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_view_id: ::std::option::Option<String>,
        #[doc = "Optional. Recommendation token included in the recommendation prediction response. This field enables accurate attribution of recommendation model performance. This token enables us to accurately attribute page view or purchase back to the event and the particular predict response containing this clicked/purchased item. If user clicks on product K in the recommendation results, pass the `PredictResponse.recommendationToken` property as a url parameter to product K's page. When recording events on product K's page, log the PredictResponse.recommendation_token to this field. Optional, but highly encouraged for user events that are the result of a recommendation prediction query."]
        #[serde(
            rename = "recommendationToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recommendation_token: ::std::option::Option<String>,
        #[doc = "Optional. The referrer url of the current page. When using the JavaScript pixel, this value is filled in automatically."]
        #[serde(
            rename = "referrerUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub referrer_uri: ::std::option::Option<String>,
        #[doc = "Optional. Complete url (window.location.href) of the user's current page. When using the JavaScript pixel, this value is filled in automatically. Maximum length 5KB."]
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudRecommendationengineV1Beta1EventDetail {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecommendationengineV1Beta1EventDetail {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1FeatureMap {
        #[doc = "Categorical features that can take on one of a limited number of possible values. Some examples would be the brand/maker of a product, or country of a customer. Feature names and values must be UTF-8 encoded strings. For example: `{ \"colors\": {\"value\": [\"yellow\", \"green\"]}, \"sizes\": {\"value\":[\"S\", \"M\"]}`"]
        #[serde(
            rename = "categoricalFeatures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub categorical_features: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                crate::schemas::GoogleCloudRecommendationengineV1Beta1FeatureMapStringList,
            >,
        >,
        #[doc = "Numerical features. Some examples would be the height/weight of a product, or age of a customer. Feature names must be UTF-8 encoded strings. For example: `{ \"lengths_cm\": {\"value\":[2.3, 15.4]}, \"heights_cm\": {\"value\":[8.1, 6.4]} }`"]
        #[serde(
            rename = "numericalFeatures",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub numerical_features: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                crate::schemas::GoogleCloudRecommendationengineV1Beta1FeatureMapFloatList,
            >,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudRecommendationengineV1Beta1FeatureMap {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecommendationengineV1Beta1FeatureMap {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1FeatureMapFloatList {
        #[doc = "Float feature value."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<Vec<f32>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1FeatureMapFloatList
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1FeatureMapFloatList
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1FeatureMapStringList {
        #[doc = "String feature value with a length limit of 128 bytes."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1FeatureMapStringList
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1FeatureMapStringList
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1GcsSource {
        #[doc = "Required. Google Cloud Storage URIs to input files. URI can be up to 2000 characters long. URIs can match the full object path (for example, `gs://bucket/directory/object.json`) or a pattern matching one or more files, such as `gs://bucket/directory/*.json`. A request can contain at most 100 files, and each file can be up to 2 GB. See [Importing catalog information](/recommendations-ai/docs/upload-catalog) for the expected file format and setup instructions."]
        #[serde(
            rename = "inputUris",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_uris: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The schema to use when parsing the data from the source. Supported values for catalog imports: 1: \"catalog_recommendations_ai\" using https://cloud.google.com/recommendations-ai/docs/upload-catalog#json (Default for catalogItems.import) 2: \"catalog_merchant_center\" using https://cloud.google.com/recommendations-ai/docs/upload-catalog#mc Supported values for user events imports: 1: \"user_events_recommendations_ai\" using https://cloud.google.com/recommendations-ai/docs/manage-user-events#import (Default for userEvents.import) 2. \"user_events_ga360\" using https://support.google.com/analytics/answer/3437719?hl=en"]
        #[serde(
            rename = "jsonSchema",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub json_schema: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudRecommendationengineV1Beta1GcsSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecommendationengineV1Beta1GcsSource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1Image {
        #[doc = "Optional. Height of the image in number of pixels."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[doc = "Required. URL of the image with a length limit of 5 KiB."]
        #[serde(
            rename = "uri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri: ::std::option::Option<String>,
        #[doc = "Optional. Width of the image in number of pixels."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudRecommendationengineV1Beta1Image {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecommendationengineV1Beta1Image {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1ImportCatalogItemsRequest {
        #[doc = "Optional. The desired location of errors incurred during the Import."]
        #[serde(
            rename = "errorsConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub errors_config: ::std::option::Option<
            crate::schemas::GoogleCloudRecommendationengineV1Beta1ImportErrorsConfig,
        >,
        #[doc = "Required. The desired input location of the data."]
        #[serde(
            rename = "inputConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_config: ::std::option::Option<
            crate::schemas::GoogleCloudRecommendationengineV1Beta1InputConfig,
        >,
        #[doc = "Optional. Unique identifier provided by client, within the ancestor dataset scope. Ensures idempotency and used for request deduplication. Server-generated if unspecified. Up to 128 characters long. This is returned as google.longrunning.Operation.name in the response."]
        #[serde(
            rename = "requestId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_id: ::std::option::Option<String>,
        #[doc = "Optional. Indicates which fields in the provided imported 'items' to update. If not set, will by default update all fields."]
        #[serde(
            rename = "updateMask",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_mask: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1ImportCatalogItemsRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1ImportCatalogItemsRequest
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudRecommendationengineV1Beta1ImportCatalogItemsResponse {
        #[doc = "A sample of errors encountered while processing the request."]
        #[serde(
            rename = "errorSamples",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_samples: ::std::option::Option<Vec<crate::schemas::GoogleRpcStatus>>,
        #[doc = "Echoes the destination for the complete errors in the request if set."]
        #[serde(
            rename = "errorsConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub errors_config: ::std::option::Option<
            crate::schemas::GoogleCloudRecommendationengineV1Beta1ImportErrorsConfig,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1ImportCatalogItemsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1ImportCatalogItemsResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1ImportErrorsConfig {
        #[doc = "Google Cloud Storage path for import errors. This must be an empty, existing Cloud Storage bucket. Import errors will be written to a file in this bucket, one per line, as a JSON-encoded `google.rpc.Status` message."]
        #[serde(
            rename = "gcsPrefix",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs_prefix: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1ImportErrorsConfig
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1ImportErrorsConfig
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1ImportMetadata {
        #[doc = "Operation create time."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Count of entries that encountered errors while processing."]
        #[serde(
            rename = "failureCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub failure_count: ::std::option::Option<i64>,
        #[doc = "Name of the operation."]
        #[serde(
            rename = "operationName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation_name: ::std::option::Option<String>,
        #[doc = "Id of the request / operation. This is parroting back the requestId that was passed in the request."]
        #[serde(
            rename = "requestId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_id: ::std::option::Option<String>,
        #[doc = "Count of entries that were processed successfully."]
        #[serde(
            rename = "successCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub success_count: ::std::option::Option<i64>,
        #[doc = "Operation last update time. If the operation is done, this is also the finish time."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1ImportMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecommendationengineV1Beta1ImportMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1ImportUserEventsRequest {
        #[doc = "Optional. The desired location of errors incurred during the Import."]
        #[serde(
            rename = "errorsConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub errors_config: ::std::option::Option<
            crate::schemas::GoogleCloudRecommendationengineV1Beta1ImportErrorsConfig,
        >,
        #[doc = "Required. The desired input location of the data."]
        #[serde(
            rename = "inputConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_config: ::std::option::Option<
            crate::schemas::GoogleCloudRecommendationengineV1Beta1InputConfig,
        >,
        #[doc = "Optional. Unique identifier provided by client, within the ancestor dataset scope. Ensures idempotency for expensive long running operations. Server-generated if unspecified. Up to 128 characters long. This is returned as google.longrunning.Operation.name in the response. Note that this field must not be set if the desired input config is catalog_inline_source."]
        #[serde(
            rename = "requestId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1ImportUserEventsRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1ImportUserEventsRequest
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudRecommendationengineV1Beta1ImportUserEventsResponse {
        #[doc = "A sample of errors encountered while processing the request."]
        #[serde(
            rename = "errorSamples",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_samples: ::std::option::Option<Vec<crate::schemas::GoogleRpcStatus>>,
        #[doc = "Echoes the destination for the complete errors if this field was set in the request."]
        #[serde(
            rename = "errorsConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub errors_config: ::std::option::Option<
            crate::schemas::GoogleCloudRecommendationengineV1Beta1ImportErrorsConfig,
        >,
        #[doc = "Aggregated statistics of user event import status."]
        #[serde(
            rename = "importSummary",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub import_summary: ::std::option::Option<
            crate::schemas::GoogleCloudRecommendationengineV1Beta1UserEventImportSummary,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1ImportUserEventsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1ImportUserEventsResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1InputConfig {
        #[doc = "BigQuery input source."]
        #[serde(
            rename = "bigQuerySource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub big_query_source: ::std::option::Option<
            crate::schemas::GoogleCloudRecommendationengineV1Beta1BigQuerySource,
        >,
        #[doc = "The Inline source for the input content for Catalog items."]
        #[serde(
            rename = "catalogInlineSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub catalog_inline_source: ::std::option::Option<
            crate::schemas::GoogleCloudRecommendationengineV1Beta1CatalogInlineSource,
        >,
        #[doc = "Google Cloud Storage location for the input content."]
        #[serde(
            rename = "gcsSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gcs_source:
            ::std::option::Option<crate::schemas::GoogleCloudRecommendationengineV1Beta1GcsSource>,
        #[doc = "The Inline source for the input content for UserEvents."]
        #[serde(
            rename = "userEventInlineSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_event_inline_source: ::std::option::Option<
            crate::schemas::GoogleCloudRecommendationengineV1Beta1UserEventInlineSource,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudRecommendationengineV1Beta1InputConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecommendationengineV1Beta1InputConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1ListCatalogItemsResponse {
        #[doc = "The catalog items."]
        #[serde(
            rename = "catalogItems",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub catalog_items: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudRecommendationengineV1Beta1CatalogItem>,
        >,
        #[doc = "If empty, the list is complete. If nonempty, the token to pass to the next request's ListCatalogItemRequest.page_token."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1ListCatalogItemsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1ListCatalogItemsResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1ListCatalogsResponse {
        #[doc = "Output only. All the customer's catalogs."]
        #[serde(
            rename = "catalogs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub catalogs: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudRecommendationengineV1Beta1Catalog>,
        >,
        #[doc = "Pagination token, if not returned indicates the last page."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1ListCatalogsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1ListCatalogsResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1ListPredictionApiKeyRegistrationsResponse {
        #[doc = "If empty, the list is complete. If nonempty, pass the token to the next request's `ListPredictionApiKeysRegistrationsRequest.pageToken`."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The list of registered API keys."]
        #[serde(
            rename = "predictionApiKeyRegistrations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub prediction_api_key_registrations: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudRecommendationengineV1Beta1PredictionApiKeyRegistration>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1ListPredictionApiKeyRegistrationsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1ListPredictionApiKeyRegistrationsResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1ListUserEventsResponse {
        #[doc = "If empty, the list is complete. If nonempty, the token to pass to the next request's ListUserEvents.page_token."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The user events."]
        #[serde(
            rename = "userEvents",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_events: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudRecommendationengineV1Beta1UserEvent>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1ListUserEventsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1ListUserEventsResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudRecommendationengineV1Beta1PredictRequest {
        #[doc = "Optional. Use dryRun mode for this prediction query. If set to true, a fake model will be used that returns arbitrary catalog items. Note that the dryRun mode should only be used for testing the API, or if the model is not ready."]
        #[serde(
            rename = "dryRun",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dry_run: ::std::option::Option<bool>,
        #[doc = "Optional. Filter for restricting prediction results. Accepts values for tags and the `filterOutOfStockItems` flag. * Tag expressions. Restricts predictions to items that match all of the specified tags. Boolean operators `OR` and `NOT` are supported if the expression is enclosed in parentheses, and must be separated from the tag values by a space. `-\"tagA\"` is also supported and is equivalent to `NOT \"tagA\"`. Tag values must be double quoted UTF-8 encoded strings with a size limit of 1 KiB. * filterOutOfStockItems. Restricts predictions to items that do not have a stockState value of OUT_OF_STOCK. Examples: * tag=(\"Red\" OR \"Blue\") tag=\"New-Arrival\" tag=(NOT \"promotional\") * filterOutOfStockItems tag=(-\"promotional\") * filterOutOfStockItems If your filter blocks all prediction results, nothing will be returned. If you want generic (unfiltered) popular items to be returned instead, set `strictFiltering` to false in `PredictRequest.params`."]
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter: ::std::option::Option<String>,
        #[doc = "Optional. The labels for the predict request. * Label keys can contain lowercase letters, digits and hyphens, must start with a letter, and must end with a letter or digit. * Non-zero label values can contain lowercase letters, digits and hyphens, must start with a letter, and must end with a letter or digit. * No more than 64 labels can be associated with a given request. See https://goo.gl/xmQnxf for more information on and examples of labels."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Optional. Maximum number of results to return per page. Set this property to the number of prediction results required. If zero, the service will choose a reasonable default."]
        #[serde(
            rename = "pageSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_size: ::std::option::Option<i32>,
        #[doc = "Optional. The previous PredictResponse.next_page_token."]
        #[serde(
            rename = "pageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_token: ::std::option::Option<String>,
        #[doc = "Optional. Additional domain specific parameters for the predictions. Allowed values: * `returnCatalogItem`: Boolean. If set to true, the associated catalogItem object will be returned in the `PredictResponse.PredictionResult.itemMetadata` object in the method response. * `returnItemScore`: Boolean. If set to true, the prediction 'score' corresponding to each returned item will be set in the `metadata` field in the prediction response. The given 'score' indicates the probability of an item being clicked/purchased given the user's context and history. * `strictFiltering`: Boolean. True by default. If set to false, the service will return generic (unfiltered) popular items instead of empty if your filter blocks all prediction results. * `priceRerankLevel`: String. Default empty. If set to be non-empty, then it needs to be one of {'no-price-reranking', 'low-price-reranking', 'medium-price-reranking', 'high-price-reranking'}. This gives request level control and adjust prediction results based on product price. * `diversityLevel`: String. Default empty. If set to be non-empty, then it needs to be one of {'no-diversity', 'low-diversity', 'medium-diversity', 'high-diversity', 'auto-diversity'}. This gives request level control and adjust prediction results based on product category."]
        #[serde(
            rename = "params",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub params:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Required. Context about the user, what they are looking at and what action they took to trigger the predict request. Note that this user event detail won't be ingested to userEvent logs. Thus, a separate userEvent write request is required for event logging. Don't set UserInfo.visitor_id or UserInfo.user_id to the same fixed ID for different users. If you are trying to receive non-personalized recommendations (not recommended; this can negatively impact model performance), instead set UserInfo.visitor_id to a random unique ID and leave UserInfo.user_id unset."]
        #[serde(
            rename = "userEvent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_event:
            ::std::option::Option<crate::schemas::GoogleCloudRecommendationengineV1Beta1UserEvent>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1PredictRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecommendationengineV1Beta1PredictRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudRecommendationengineV1Beta1PredictResponse { # [doc = "True if the dryRun property was set in the request."] # [serde (rename = "dryRun" , default , skip_serializing_if = "std::option::Option::is_none")] pub dry_run : :: std :: option :: Option < bool > , # [doc = "IDs of items in the request that were missing from the catalog."] # [serde (rename = "itemsMissingInCatalog" , default , skip_serializing_if = "std::option::Option::is_none")] pub items_missing_in_catalog : :: std :: option :: Option < Vec < String > > , # [doc = "Additional domain specific prediction response metadata."] # [serde (rename = "metadata" , default , skip_serializing_if = "std::option::Option::is_none")] pub metadata : :: std :: option :: Option < :: std :: collections :: BTreeMap < String , :: serde_json :: Value > > , # [doc = "If empty, the list is complete. If nonempty, the token to pass to the next request's PredictRequest.page_token."] # [serde (rename = "nextPageToken" , default , skip_serializing_if = "std::option::Option::is_none")] pub next_page_token : :: std :: option :: Option < String > , # [doc = "A unique recommendation token. This should be included in the user event logs resulting from this recommendation, which enables accurate attribution of recommendation model performance."] # [serde (rename = "recommendationToken" , default , skip_serializing_if = "std::option::Option::is_none")] pub recommendation_token : :: std :: option :: Option < String > , # [doc = "A list of recommended items. The order represents the ranking (from the most relevant item to the least)."] # [serde (rename = "results" , default , skip_serializing_if = "std::option::Option::is_none")] pub results : :: std :: option :: Option < Vec < crate :: schemas :: GoogleCloudRecommendationengineV1Beta1PredictResponsePredictionResult > > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1PredictResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1PredictResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleCloudRecommendationengineV1Beta1PredictResponsePredictionResult {
        #[doc = "ID of the recommended catalog item"]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Additional item metadata / annotations. Possible values: * `catalogItem`: JSON representation of the catalogItem. Will be set if `returnCatalogItem` is set to true in `PredictRequest.params`. * `score`: Prediction score in double value. Will be set if `returnItemScore` is set to true in `PredictRequest.params`."]
        #[serde(
            rename = "itemMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub item_metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1PredictResponsePredictionResult
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1PredictResponsePredictionResult
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1PredictionApiKeyRegistration {
        #[doc = "The API key."]
        #[serde(
            rename = "apiKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub api_key: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1PredictionApiKeyRegistration
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1PredictionApiKeyRegistration
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1ProductCatalogItem {
        #[doc = "Optional. The available quantity of the item."]
        #[serde(
            rename = "availableQuantity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub available_quantity: ::std::option::Option<i64>,
        #[doc = "Optional. Canonical URL directly linking to the item detail page with a length limit of 5 KiB.."]
        #[serde(
            rename = "canonicalProductUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub canonical_product_uri: ::std::option::Option<String>,
        #[doc = "Optional. A map to pass the costs associated with the product. For example: {\"manufacturing\": 45.5} The profit of selling this item is computed like so: * If 'exactPrice' is provided, profit = displayPrice - sum(costs) * If 'priceRange' is provided, profit = minPrice - sum(costs)"]
        #[serde(
            rename = "costs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub costs: ::std::option::Option<::std::collections::BTreeMap<String, f32>>,
        #[doc = "Optional. Only required if the price is set. Currency code for price/costs. Use three-character ISO-4217 code."]
        #[serde(
            rename = "currencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency_code: ::std::option::Option<String>,
        #[doc = "Optional. The exact product price."]
        #[serde(
            rename = "exactPrice",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exact_price: ::std::option::Option<
            crate::schemas::GoogleCloudRecommendationengineV1Beta1ProductCatalogItemExactPrice,
        >,
        #[doc = "Optional. Product images for the catalog item."]
        #[serde(
            rename = "images",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub images:
            ::std::option::Option<Vec<crate::schemas::GoogleCloudRecommendationengineV1Beta1Image>>,
        #[doc = "Optional. The product price range."]
        #[serde(
            rename = "priceRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub price_range: ::std::option::Option<
            crate::schemas::GoogleCloudRecommendationengineV1Beta1ProductCatalogItemPriceRange,
        >,
        #[doc = "Optional. Online stock state of the catalog item. Default is `IN_STOCK`."]
        #[serde(
            rename = "stockState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stock_state: ::std::option::Option<
            crate::schemas::GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1ProductCatalogItem
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1ProductCatalogItem
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState {
        #[doc = "Item that is back-ordered (i.e. temporarily out of stock)."]
        Backorder,
        #[doc = "Item in stock."]
        InStock,
        #[doc = "Item out of stock."]
        OutOfStock,
        #[doc = "Item that is in pre-order state."]
        Preorder,
        #[doc = "Default item stock status. Should never be used."]
        StockStateUnspecified,
    }
    impl GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState :: Backorder => "BACKORDER" , GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState :: InStock => "IN_STOCK" , GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState :: OutOfStock => "OUT_OF_STOCK" , GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState :: Preorder => "PREORDER" , GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState :: StockStateUnspecified => "STOCK_STATE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState,
            (),
        > {
            Ok (match s { "BACKORDER" => GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState :: Backorder , "IN_STOCK" => GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState :: InStock , "OUT_OF_STOCK" => GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState :: OutOfStock , "PREORDER" => GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState :: Preorder , "STOCK_STATE_UNSPECIFIED" => GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState :: StockStateUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "BACKORDER" => GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState :: Backorder , "IN_STOCK" => GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState :: InStock , "OUT_OF_STOCK" => GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState :: OutOfStock , "PREORDER" => GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState :: Preorder , "STOCK_STATE_UNSPECIFIED" => GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState :: StockStateUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1ProductCatalogItemStockState
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1ProductCatalogItemExactPrice {
        #[doc = "Optional. Display price of the product."]
        #[serde(
            rename = "displayPrice",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_price: ::std::option::Option<f32>,
        #[doc = "Optional. Price of the product without any discount. If zero, by default set to be the 'displayPrice'."]
        #[serde(
            rename = "originalPrice",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub original_price: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1ProductCatalogItemExactPrice
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1ProductCatalogItemExactPrice
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1ProductCatalogItemPriceRange {
        #[doc = "Required. The maximum product price."]
        #[serde(
            rename = "max",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max: ::std::option::Option<f32>,
        #[doc = "Required. The minimum product price."]
        #[serde(
            rename = "min",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min: ::std::option::Option<f32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1ProductCatalogItemPriceRange
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1ProductCatalogItemPriceRange
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1ProductDetail {
        #[doc = "Optional. Quantity of the products in stock when a user event happens. Optional. If provided, this overrides the available quantity in Catalog for this event. and can only be set if `stock_status` is set to `IN_STOCK`. Note that if an item is out of stock, you must set the `stock_state` field to be `OUT_OF_STOCK`. Leaving this field unspecified / as zero is not sufficient to mark the item out of stock."]
        #[serde(
            rename = "availableQuantity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub available_quantity: ::std::option::Option<i32>,
        #[doc = "Optional. Currency code for price/costs. Use three-character ISO-4217 code. Required only if originalPrice or displayPrice is set."]
        #[serde(
            rename = "currencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency_code: ::std::option::Option<String>,
        #[doc = "Optional. Display price of the product (e.g. discounted price). If provided, this will override the display price in Catalog for this product."]
        #[serde(
            rename = "displayPrice",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_price: ::std::option::Option<f32>,
        #[doc = "Required. Catalog item ID. UTF-8 encoded string with a length limit of 128 characters."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Optional. Extra features associated with a product in the user event."]
        #[serde(
            rename = "itemAttributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub item_attributes:
            ::std::option::Option<crate::schemas::GoogleCloudRecommendationengineV1Beta1FeatureMap>,
        #[doc = "Optional. Original price of the product. If provided, this will override the original price in Catalog for this product."]
        #[serde(
            rename = "originalPrice",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub original_price: ::std::option::Option<f32>,
        #[doc = "Optional. Quantity of the product associated with the user event. For example, this field will be 2 if two products are added to the shopping cart for `add-to-cart` event. Required for `add-to-cart`, `add-to-list`, `remove-from-cart`, `checkout-start`, `purchase-complete`, `refund` event types."]
        #[serde(
            rename = "quantity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quantity: ::std::option::Option<i32>,
        #[doc = "Optional. Item stock state. If provided, this overrides the stock state in Catalog for items in this event."]
        #[serde(
            rename = "stockState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stock_state: ::std::option::Option<
            crate::schemas::GoogleCloudRecommendationengineV1Beta1ProductDetailStockState,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1ProductDetail
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecommendationengineV1Beta1ProductDetail {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudRecommendationengineV1Beta1ProductDetailStockState {
        #[doc = "Item that is back-ordered (i.e. temporarily out of stock)."]
        Backorder,
        #[doc = "Item in stock."]
        InStock,
        #[doc = "Item out of stock."]
        OutOfStock,
        #[doc = "Item that is in pre-order state."]
        Preorder,
        #[doc = "Default item stock status. Should never be used."]
        StockStateUnspecified,
    }
    impl GoogleCloudRecommendationengineV1Beta1ProductDetailStockState {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudRecommendationengineV1Beta1ProductDetailStockState :: Backorder => "BACKORDER" , GoogleCloudRecommendationengineV1Beta1ProductDetailStockState :: InStock => "IN_STOCK" , GoogleCloudRecommendationengineV1Beta1ProductDetailStockState :: OutOfStock => "OUT_OF_STOCK" , GoogleCloudRecommendationengineV1Beta1ProductDetailStockState :: Preorder => "PREORDER" , GoogleCloudRecommendationengineV1Beta1ProductDetailStockState :: StockStateUnspecified => "STOCK_STATE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudRecommendationengineV1Beta1ProductDetailStockState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudRecommendationengineV1Beta1ProductDetailStockState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudRecommendationengineV1Beta1ProductDetailStockState, ()>
        {
            Ok (match s { "BACKORDER" => GoogleCloudRecommendationengineV1Beta1ProductDetailStockState :: Backorder , "IN_STOCK" => GoogleCloudRecommendationengineV1Beta1ProductDetailStockState :: InStock , "OUT_OF_STOCK" => GoogleCloudRecommendationengineV1Beta1ProductDetailStockState :: OutOfStock , "PREORDER" => GoogleCloudRecommendationengineV1Beta1ProductDetailStockState :: Preorder , "STOCK_STATE_UNSPECIFIED" => GoogleCloudRecommendationengineV1Beta1ProductDetailStockState :: StockStateUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudRecommendationengineV1Beta1ProductDetailStockState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudRecommendationengineV1Beta1ProductDetailStockState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudRecommendationengineV1Beta1ProductDetailStockState
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "BACKORDER" => GoogleCloudRecommendationengineV1Beta1ProductDetailStockState :: Backorder , "IN_STOCK" => GoogleCloudRecommendationengineV1Beta1ProductDetailStockState :: InStock , "OUT_OF_STOCK" => GoogleCloudRecommendationengineV1Beta1ProductDetailStockState :: OutOfStock , "PREORDER" => GoogleCloudRecommendationengineV1Beta1ProductDetailStockState :: Preorder , "STOCK_STATE_UNSPECIFIED" => GoogleCloudRecommendationengineV1Beta1ProductDetailStockState :: StockStateUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1ProductDetailStockState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1ProductDetailStockState
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1ProductEventDetail {
        #[doc = "Optional. The id or name of the associated shopping cart. This id is used to associate multiple items added or present in the cart before purchase. This can only be set for `add-to-cart`, `remove-from-cart`, `checkout-start`, `purchase-complete`, or `shopping-cart-page-view` events."]
        #[serde(
            rename = "cartId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cart_id: ::std::option::Option<String>,
        #[doc = "Required for `add-to-list` and `remove-from-list` events. The id or name of the list that the item is being added to or removed from. Other event types should not set this field."]
        #[serde(
            rename = "listId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub list_id: ::std::option::Option<String>,
        #[doc = "Required for `category-page-view` events. At least one of search_query or page_categories is required for `search` events. Other event types should not set this field. The categories associated with a category page. Category pages include special pages such as sales or promotions. For instance, a special sale page may have the category hierarchy: categories : [\"Sales\", \"2017 Black Friday Deals\"]."]
        #[serde(
            rename = "pageCategories",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_categories: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudRecommendationengineV1Beta1CatalogItemCategoryHierarchy>,
        >,
        #[doc = "The main product details related to the event. This field is required for the following event types: * `add-to-cart` * `add-to-list` * `checkout-start` * `detail-page-view` * `purchase-complete` * `refund` * `remove-from-cart` * `remove-from-list` This field is optional for the following event types: * `page-visit` * `shopping-cart-page-view` - note that 'product_details' should be set for this unless the shopping cart is empty. * `search` (highly encouraged) In a `search` event, this field represents the products returned to the end user on the current page (the end user may have not finished broswing the whole page yet). When a new page is returned to the end user, after pagination/filtering/ordering even for the same query, a new SEARCH event with different product_details is desired. The end user may have not finished broswing the whole page yet. This field is not allowed for the following event types: * `category-page-view` * `home-page-view`"]
        #[serde(
            rename = "productDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub product_details: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudRecommendationengineV1Beta1ProductDetail>,
        >,
        #[doc = "Optional. A transaction represents the entire purchase transaction. Required for `purchase-complete` events. Optional for `checkout-start` events. Other event types should not set this field."]
        #[serde(
            rename = "purchaseTransaction",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub purchase_transaction: ::std::option::Option<
            crate::schemas::GoogleCloudRecommendationengineV1Beta1PurchaseTransaction,
        >,
        #[doc = "At least one of search_query or page_categories is required for `search` events. Other event types should not set this field. The user's search query as UTF-8 encoded text with a length limit of 5 KiB."]
        #[serde(
            rename = "searchQuery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_query: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1ProductEventDetail
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1ProductEventDetail
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1PurchaseTransaction {
        #[doc = "Optional. All the costs associated with the product. These can be manufacturing costs, shipping expenses not borne by the end user, or any other costs. Total product cost such that profit = revenue - (sum(taxes) + sum(costs)) If product_cost is not set, then profit = revenue - tax - shipping - sum(CatalogItem.costs). If CatalogItem.cost is not specified for one of the items, CatalogItem.cost based profit *cannot* be calculated for this Transaction."]
        #[serde(
            rename = "costs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub costs: ::std::option::Option<::std::collections::BTreeMap<String, f32>>,
        #[doc = "Required. Currency code. Use three-character ISO-4217 code. This field is not required if the event type is `refund`."]
        #[serde(
            rename = "currencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency_code: ::std::option::Option<String>,
        #[doc = "Optional. The transaction ID with a length limit of 128 bytes."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Required. Total revenue or grand total associated with the transaction. This value include shipping, tax, or other adjustments to total revenue that you want to include as part of your revenue calculations. This field is not required if the event type is `refund`."]
        #[serde(
            rename = "revenue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revenue: ::std::option::Option<f32>,
        #[doc = "Optional. All the taxes associated with the transaction."]
        #[serde(
            rename = "taxes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub taxes: ::std::option::Option<::std::collections::BTreeMap<String, f32>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1PurchaseTransaction
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1PurchaseTransaction
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1PurgeUserEventsMetadata {
        #[doc = "Operation create time."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The ID of the request / operation."]
        #[serde(
            rename = "operationName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1PurgeUserEventsMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1PurgeUserEventsMetadata
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1PurgeUserEventsRequest {
        #[doc = "Required. The filter string to specify the events to be deleted. Empty string filter is not allowed. The eligible fields for filtering are: * `eventType`: UserEvent.eventType field of type string. * `eventTime`: in ISO 8601 \"zulu\" format. * `visitorId`: field of type string. Specifying this will delete all events associated with a visitor. * `userId`: field of type string. Specifying this will delete all events associated with a user. Examples: * Deleting all events in a time range: `eventTime > \"2012-04-23T18:25:43.511Z\" eventTime < \"2012-04-23T18:30:43.511Z\"` * Deleting specific eventType in time range: `eventTime > \"2012-04-23T18:25:43.511Z\" eventType = \"detail-page-view\"` * Deleting all events for a specific visitor: `visitorId = \"visitor1024\"` The filtering fields are assumed to have an implicit AND."]
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter: ::std::option::Option<String>,
        #[doc = "Optional. The default value is false. Override this flag to true to actually perform the purge. If the field is not set to true, a sampling of events to be deleted will be returned."]
        #[serde(
            rename = "force",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub force: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1PurgeUserEventsRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1PurgeUserEventsRequest
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1PurgeUserEventsResponse {
        #[doc = "The total count of events purged as a result of the operation."]
        #[serde(
            rename = "purgedEventsCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub purged_events_count: ::std::option::Option<i64>,
        #[doc = "A sampling of events deleted (or will be deleted) depending on the `force` property in the request. Max of 500 items will be returned."]
        #[serde(
            rename = "userEventsSample",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_events_sample: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudRecommendationengineV1Beta1UserEvent>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1PurgeUserEventsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1PurgeUserEventsResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1RejoinUserEventsMetadata {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1RejoinUserEventsMetadata
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1RejoinUserEventsMetadata
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1RejoinUserEventsRequest { # [doc = "Required. The type of the catalog rejoin to define the scope and range of the user events to be rejoined with catalog items."] # [serde (rename = "userEventRejoinScope" , default , skip_serializing_if = "std::option::Option::is_none")] pub user_event_rejoin_scope : :: std :: option :: Option < crate :: schemas :: GoogleCloudRecommendationengineV1Beta1RejoinUserEventsRequestUserEventRejoinScope > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1RejoinUserEventsRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1RejoinUserEventsRequest
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudRecommendationengineV1Beta1RejoinUserEventsRequestUserEventRejoinScope {
        #[doc = "Only rejoin catalogs with joined events."]
        JoinedEvents,
        #[doc = "Only rejoin catalogs with unjoined events."]
        UnjoinedEvents,
        #[doc = "Rejoin catalogs with all events including both joined events and unjoined events."]
        UserEventRejoinScopeUnspecified,
    }
    impl GoogleCloudRecommendationengineV1Beta1RejoinUserEventsRequestUserEventRejoinScope {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudRecommendationengineV1Beta1RejoinUserEventsRequestUserEventRejoinScope :: JoinedEvents => "JOINED_EVENTS" , GoogleCloudRecommendationengineV1Beta1RejoinUserEventsRequestUserEventRejoinScope :: UnjoinedEvents => "UNJOINED_EVENTS" , GoogleCloudRecommendationengineV1Beta1RejoinUserEventsRequestUserEventRejoinScope :: UserEventRejoinScopeUnspecified => "USER_EVENT_REJOIN_SCOPE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudRecommendationengineV1Beta1RejoinUserEventsRequestUserEventRejoinScope
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudRecommendationengineV1Beta1RejoinUserEventsRequestUserEventRejoinScope
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudRecommendationengineV1Beta1RejoinUserEventsRequestUserEventRejoinScope,
            (),
        > {
            Ok (match s { "JOINED_EVENTS" => GoogleCloudRecommendationengineV1Beta1RejoinUserEventsRequestUserEventRejoinScope :: JoinedEvents , "UNJOINED_EVENTS" => GoogleCloudRecommendationengineV1Beta1RejoinUserEventsRequestUserEventRejoinScope :: UnjoinedEvents , "USER_EVENT_REJOIN_SCOPE_UNSPECIFIED" => GoogleCloudRecommendationengineV1Beta1RejoinUserEventsRequestUserEventRejoinScope :: UserEventRejoinScopeUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudRecommendationengineV1Beta1RejoinUserEventsRequestUserEventRejoinScope
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudRecommendationengineV1Beta1RejoinUserEventsRequestUserEventRejoinScope
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudRecommendationengineV1Beta1RejoinUserEventsRequestUserEventRejoinScope
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "JOINED_EVENTS" => GoogleCloudRecommendationengineV1Beta1RejoinUserEventsRequestUserEventRejoinScope :: JoinedEvents , "UNJOINED_EVENTS" => GoogleCloudRecommendationengineV1Beta1RejoinUserEventsRequestUserEventRejoinScope :: UnjoinedEvents , "USER_EVENT_REJOIN_SCOPE_UNSPECIFIED" => GoogleCloudRecommendationengineV1Beta1RejoinUserEventsRequestUserEventRejoinScope :: UserEventRejoinScopeUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1RejoinUserEventsRequestUserEventRejoinScope
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1RejoinUserEventsRequestUserEventRejoinScope
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1RejoinUserEventsResponse {
        #[doc = "Number of user events that were joined with latest catalog items."]
        #[serde(
            rename = "rejoinedUserEventsCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub rejoined_user_events_count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1RejoinUserEventsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1RejoinUserEventsResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1UserEvent {
        #[doc = "Optional. User event detailed information common across different recommendation types."]
        #[serde(
            rename = "eventDetail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub event_detail: ::std::option::Option<
            crate::schemas::GoogleCloudRecommendationengineV1Beta1EventDetail,
        >,
        #[doc = "Optional. This field should *not* be set when using JavaScript pixel or the Recommendations AI Tag. Defaults to `EVENT_SOURCE_UNSPECIFIED`."]
        #[serde(
            rename = "eventSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub event_source: ::std::option::Option<
            crate::schemas::GoogleCloudRecommendationengineV1Beta1UserEventEventSource,
        >,
        #[doc = "Optional. Only required for ImportUserEvents method. Timestamp of user event created."]
        #[serde(
            rename = "eventTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub event_time: ::std::option::Option<String>,
        #[doc = "Required. User event type. Allowed values are: * `add-to-cart` Products being added to cart. * `add-to-list` Items being added to a list (shopping list, favorites etc). * `category-page-view` Special pages such as sale or promotion pages viewed. * `checkout-start` User starting a checkout process. * `detail-page-view` Products detail page viewed. * `home-page-view` Homepage viewed. * `page-visit` Generic page visits not included in the event types above. * `purchase-complete` User finishing a purchase. * `refund` Purchased items being refunded or returned. * `remove-from-cart` Products being removed from cart. * `remove-from-list` Items being removed from a list. * `search` Product search. * `shopping-cart-page-view` User viewing a shopping cart. * `impression` List of items displayed. Used by Google Tag Manager."]
        #[serde(
            rename = "eventType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub event_type: ::std::option::Option<String>,
        #[doc = "Optional. Retail product specific user event metadata. This field is required for the following event types: * `add-to-cart` * `add-to-list` * `category-page-view` * `checkout-start` * `detail-page-view` * `purchase-complete` * `refund` * `remove-from-cart` * `remove-from-list` * `search` This field is optional for the following event types: * `page-visit` * `shopping-cart-page-view` - note that 'product_event_detail' should be set for this unless the shopping cart is empty. This field is not allowed for the following event types: * `home-page-view`"]
        #[serde(
            rename = "productEventDetail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub product_event_detail: ::std::option::Option<
            crate::schemas::GoogleCloudRecommendationengineV1Beta1ProductEventDetail,
        >,
        #[doc = "Required. User information."]
        #[serde(
            rename = "userInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_info:
            ::std::option::Option<crate::schemas::GoogleCloudRecommendationengineV1Beta1UserInfo>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudRecommendationengineV1Beta1UserEvent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecommendationengineV1Beta1UserEvent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudRecommendationengineV1Beta1UserEventEventSource {
        #[doc = "The event is ingested via a javascript pixel or Recommendations AI Tag through automl datalayer or JS Macros."]
        Automl,
        #[doc = "The event is ingested via Import user events API."]
        BatchUpload,
        #[doc = "The event is ingested via Recommendations AI Tag through Enhanced Ecommerce datalayer."]
        Ecommerce,
        #[doc = "Unspecified event source."]
        EventSourceUnspecified,
    }
    impl GoogleCloudRecommendationengineV1Beta1UserEventEventSource {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudRecommendationengineV1Beta1UserEventEventSource :: Automl => "AUTOML" , GoogleCloudRecommendationengineV1Beta1UserEventEventSource :: BatchUpload => "BATCH_UPLOAD" , GoogleCloudRecommendationengineV1Beta1UserEventEventSource :: Ecommerce => "ECOMMERCE" , GoogleCloudRecommendationengineV1Beta1UserEventEventSource :: EventSourceUnspecified => "EVENT_SOURCE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudRecommendationengineV1Beta1UserEventEventSource {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudRecommendationengineV1Beta1UserEventEventSource {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudRecommendationengineV1Beta1UserEventEventSource, ()>
        {
            Ok (match s { "AUTOML" => GoogleCloudRecommendationengineV1Beta1UserEventEventSource :: Automl , "BATCH_UPLOAD" => GoogleCloudRecommendationengineV1Beta1UserEventEventSource :: BatchUpload , "ECOMMERCE" => GoogleCloudRecommendationengineV1Beta1UserEventEventSource :: Ecommerce , "EVENT_SOURCE_UNSPECIFIED" => GoogleCloudRecommendationengineV1Beta1UserEventEventSource :: EventSourceUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudRecommendationengineV1Beta1UserEventEventSource {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudRecommendationengineV1Beta1UserEventEventSource {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudRecommendationengineV1Beta1UserEventEventSource {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "AUTOML" => GoogleCloudRecommendationengineV1Beta1UserEventEventSource :: Automl , "BATCH_UPLOAD" => GoogleCloudRecommendationengineV1Beta1UserEventEventSource :: BatchUpload , "ECOMMERCE" => GoogleCloudRecommendationengineV1Beta1UserEventEventSource :: Ecommerce , "EVENT_SOURCE_UNSPECIFIED" => GoogleCloudRecommendationengineV1Beta1UserEventEventSource :: EventSourceUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1UserEventEventSource
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1UserEventEventSource
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1UserEventImportSummary {
        #[doc = "Count of user events imported with complete existing catalog information."]
        #[serde(
            rename = "joinedEventsCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub joined_events_count: ::std::option::Option<i64>,
        #[doc = "Count of user events imported, but with catalog information not found in the imported catalog."]
        #[serde(
            rename = "unjoinedEventsCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub unjoined_events_count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1UserEventImportSummary
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1UserEventImportSummary
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1UserEventInlineSource {
        #[doc = "Optional. A list of user events to import. Recommended max of 10k items."]
        #[serde(
            rename = "userEvents",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_events: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudRecommendationengineV1Beta1UserEvent>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudRecommendationengineV1Beta1UserEventInlineSource
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudRecommendationengineV1Beta1UserEventInlineSource
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudRecommendationengineV1Beta1UserInfo {
        #[doc = "Optional. Indicates if the request is made directly from the end user in which case the user_agent and ip_address fields can be populated from the HTTP request. This should *not* be set when using the javascript pixel. This flag should be set only if the API request is made directly from the end user such as a mobile app (and not if a gateway or a server is processing and pushing the user events)."]
        #[serde(
            rename = "directUserRequest",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub direct_user_request: ::std::option::Option<bool>,
        #[doc = "Optional. IP address of the user. This could be either IPv4 (e.g. 104.133.9.80) or IPv6 (e.g. 2001:0db8:85a3:0000:0000:8a2e:0370:7334). This should *not* be set when using the javascript pixel or if `direct_user_request` is set. Used to extract location information for personalization."]
        #[serde(
            rename = "ipAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ip_address: ::std::option::Option<String>,
        #[doc = "Optional. User agent as included in the HTTP header. UTF-8 encoded string with a length limit of 1 KiB. This should *not* be set when using the JavaScript pixel or if `directUserRequest` is set."]
        #[serde(
            rename = "userAgent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_agent: ::std::option::Option<String>,
        #[doc = "Optional. Unique identifier for logged-in user with a length limit of 128 bytes. Required only for logged-in users. Don't set for anonymous users. Don't set the field to the same fixed ID for different users. This mixes the event history of those users together, which results in degraded model quality."]
        #[serde(
            rename = "userId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_id: ::std::option::Option<String>,
        #[doc = "Required. A unique identifier for tracking visitors with a length limit of 128 bytes. For example, this could be implemented with an HTTP cookie, which should be able to uniquely identify a visitor on a single device. This unique identifier should not change if the visitor logs in or out of the website. Maximum length 128 bytes. Cannot be empty. Don't set the field to the same fixed ID for different users. This mixes the event history of those users together, which results in degraded model quality."]
        #[serde(
            rename = "visitorId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub visitor_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudRecommendationengineV1Beta1UserInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudRecommendationengineV1Beta1UserInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleLongrunningListOperationsResponse {
        #[doc = "The standard List next-page token."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "A list of operations that matches the specified filter in the request."]
        #[serde(
            rename = "operations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operations: ::std::option::Option<Vec<crate::schemas::GoogleLongrunningOperation>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleLongrunningListOperationsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleLongrunningListOperationsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleLongrunningOperation {
        #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
        #[serde(
            rename = "done",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub done: ::std::option::Option<bool>,
        #[doc = "The error result of the operation in case of failure or cancellation."]
        #[serde(
            rename = "error",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
        #[doc = "Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`."]
        #[serde(
            rename = "response",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleLongrunningOperation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleLongrunningOperation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleProtobufEmpty {}
    impl ::google_field_selector::FieldSelector for GoogleProtobufEmpty {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleProtobufEmpty {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleRpcStatus {
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<i32>,
        #[doc = "A list of messages that carry the error details. There is a common set of message types for APIs to use."]
        #[serde(
            rename = "details",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub details:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleRpcStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleRpcStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
}
pub mod params {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Alt {
        #[doc = "Responses with Content-Type of application/json"]
        Json,
        #[doc = "Media download with context-dependent Content-Type"]
        Media,
        #[doc = "Responses with Content-Type of application/x-protobuf"]
        Proto,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
                Alt::Media => "media",
                Alt::Proto => "proto",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Alt {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Alt {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Alt, ()> {
            Ok(match s {
                "json" => Alt::Json,
                "media" => Alt::Media,
                "proto" => Alt::Proto,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Alt {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Alt {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Alt {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "json" => Alt::Json,
                "media" => Alt::Media,
                "proto" => Alt::Proto,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Alt {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Alt {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Xgafv {
        #[doc = "v1 error format"]
        _1,
        #[doc = "v2 error format"]
        _2,
    }
    impl Xgafv {
        pub fn as_str(self) -> &'static str {
            match self {
                Xgafv::_1 => "1",
                Xgafv::_2 => "2",
            }
        }
    }
    impl ::std::convert::AsRef<str> for Xgafv {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for Xgafv {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<Xgafv, ()> {
            Ok(match s {
                "1" => Xgafv::_1,
                "2" => Xgafv::_2,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for Xgafv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Xgafv {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Xgafv {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "1" => Xgafv::_1,
                "2" => Xgafv::_2,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for Xgafv {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Xgafv {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
}
pub struct Client {
    reqwest: ::reqwest::Client,
    auth: Box<dyn ::google_api_auth::GetAccessToken>,
}
impl Client {
    pub fn new<A>(auth: A) -> Self
    where
        A: ::google_api_auth::GetAccessToken + 'static,
    {
        Client::with_reqwest_client(auth, ::reqwest::Client::builder().build().unwrap())
    }
    pub fn with_reqwest_client<A>(auth: A, reqwest: ::reqwest::Client) -> Self
    where
        A: ::google_api_auth::GetAccessToken + 'static,
    {
        Client {
            reqwest,
            auth: Box::new(auth),
        }
    }
    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
        self.auth.as_ref()
    }
    #[doc = "Actions that can be performed on the projects resource"]
    pub fn projects(&self) -> crate::resources::projects::ProjectsActions {
        crate::resources::projects::ProjectsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod projects {
        pub mod params {}
        pub struct ProjectsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ProjectsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the locations resource"]
            pub fn locations(&self) -> crate::resources::projects::locations::LocationsActions {
                crate::resources::projects::locations::LocationsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod locations {
            pub mod params {}
            pub struct LocationsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> LocationsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Actions that can be performed on the catalogs resource"]
                pub fn catalogs(
                    &self,
                ) -> crate::resources::projects::locations::catalogs::CatalogsActions
                {
                    crate::resources::projects::locations::catalogs::CatalogsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            pub mod catalogs {
                pub mod params {}
                pub struct CatalogsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> CatalogsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Lists all the catalog configurations associated with the project."]
                    pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            parent: parent.into(),
                            page_size: None,
                            page_token: None,
                        }
                    }
                    #[doc = "Updates the catalog configuration."]
                    pub fn patch(
                        &self,
                        request: crate::schemas::GoogleCloudRecommendationengineV1Beta1Catalog,
                        name: impl Into<String>,
                    ) -> PatchRequestBuilder {
                        PatchRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: self.auth_ref(),
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            name: name.into(),
                            update_mask: None,
                        }
                    }
                    #[doc = "Actions that can be performed on the catalog_items resource"]                    pub fn catalog_items (& self) -> crate :: resources :: projects :: locations :: catalogs :: catalog_items :: CatalogItemsActions{
                        crate :: resources :: projects :: locations :: catalogs :: catalog_items :: CatalogItemsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                    }
                    #[doc = "Actions that can be performed on the event_stores resource"]                    pub fn event_stores (& self) -> crate :: resources :: projects :: locations :: catalogs :: event_stores :: EventStoresActions{
                        crate :: resources :: projects :: locations :: catalogs :: event_stores :: EventStoresActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                    }
                    #[doc = "Actions that can be performed on the operations resource"]                    pub fn operations (& self) -> crate :: resources :: projects :: locations :: catalogs :: operations :: OperationsActions{
                        crate :: resources :: projects :: locations :: catalogs :: operations :: OperationsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                    }
                }
                #[doc = "Created via [CatalogsActions::list()](struct.CatalogsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    parent: String,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> ListRequestBuilder<'a> {
                    #[doc = "Optional. Maximum number of results to return. If unspecified, defaults to 50. Max allowed value is 1000."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "Optional. A page token, received from a previous `ListCatalogs` call. Provide this to retrieve the subsequent page."]
                    pub fn page_token(mut self, value: impl Into<String>) -> Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub async fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields).await
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub async fn execute_with_default_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudRecommendationengineV1Beta1ListCatalogsResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudRecommendationengineV1Beta1ListCatalogsResponse,
                        crate::Error,
                    > {
                        self.execute_with_fields(Some("*")).await
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub async fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute().await
                    }
                    async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path()).await?;
                        Ok(req.send().await?.error_for_status()?.json().await?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://recommendationengine.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/catalogs");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[("pageSize", &self.page_size)]);
                        req = req.query(&[("pageToken", &self.page_token)]);
                        req = req.query(&[("access_token", &self.access_token)]);
                        req = req.query(&[("alt", &self.alt)]);
                        req = req.query(&[("callback", &self.callback)]);
                        req = req.query(&[("fields", &self.fields)]);
                        req = req.query(&[("key", &self.key)]);
                        req = req.query(&[("oauth_token", &self.oauth_token)]);
                        req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        req = req.query(&[("quotaUser", &self.quota_user)]);
                        req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        req = req.query(&[("uploadType", &self.upload_type)]);
                        req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let access_token = self
                            .auth
                            .access_token()
                            .await
                            .map_err(|err| crate::Error::OAuth2(err))?;
                        req = req.bearer_auth(access_token);
                        Ok(req)
                    }
                }
                #[doc = "Created via [CatalogsActions::patch()](struct.CatalogsActions.html#method.patch)"]
                #[derive(Debug, Clone)]
                pub struct PatchRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::GoogleCloudRecommendationengineV1Beta1Catalog,
                    name: String,
                    update_mask: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a> PatchRequestBuilder<'a> {
                    #[doc = "Optional. Indicates which fields in the provided 'catalog' to update. If not set, will only update the catalog_item_level_config field. Currently only fields that can be updated are catalog_item_level_config."]
                    pub fn update_mask(mut self, value: impl Into<String>) -> Self {
                        self.update_mask = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub async fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields).await
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub async fn execute_with_default_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudRecommendationengineV1Beta1Catalog,
                        crate::Error,
                    > {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<
                        crate::schemas::GoogleCloudRecommendationengineV1Beta1Catalog,
                        crate::Error,
                    > {
                        self.execute_with_fields(Some("*")).await
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub async fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute().await
                    }
                    async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path()).await?;
                        let req = req.json(&self.request);
                        Ok(req.send().await?.error_for_status()?.json().await?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://recommendationengine.googleapis.com/".to_owned();
                        output.push_str("v1beta1/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::PATCH, path);
                        req = req.query(&[("updateMask", &self.update_mask)]);
                        req = req.query(&[("access_token", &self.access_token)]);
                        req = req.query(&[("alt", &self.alt)]);
                        req = req.query(&[("callback", &self.callback)]);
                        req = req.query(&[("fields", &self.fields)]);
                        req = req.query(&[("key", &self.key)]);
                        req = req.query(&[("oauth_token", &self.oauth_token)]);
                        req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        req = req.query(&[("quotaUser", &self.quota_user)]);
                        req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        req = req.query(&[("uploadType", &self.upload_type)]);
                        req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let access_token = self
                            .auth
                            .access_token()
                            .await
                            .map_err(|err| crate::Error::OAuth2(err))?;
                        req = req.bearer_auth(access_token);
                        Ok(req)
                    }
                }
                pub mod catalog_items {
                    pub mod params {}
                    pub struct CatalogItemsActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> CatalogItemsActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Creates a catalog item."]
                        pub fn create(
                            &self,
                            request : crate :: schemas :: GoogleCloudRecommendationengineV1Beta1CatalogItem,
                            parent: impl Into<String>,
                        ) -> CreateRequestBuilder {
                            CreateRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                            }
                        }
                        #[doc = "Deletes a catalog item."]
                        pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder {
                            DeleteRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                            }
                        }
                        #[doc = "Gets a specific catalog item."]
                        pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                            GetRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                            }
                        }
                        #[doc = "Bulk import of multiple catalog items. Request processing may be synchronous. No partial updating supported. Non-existing items will be created. Operation.response is of type ImportResponse. Note that it is possible for a subset of the items to be successfully updated."]
                        pub fn import(
                            &self,
                            request : crate :: schemas :: GoogleCloudRecommendationengineV1Beta1ImportCatalogItemsRequest,
                            parent: impl Into<String>,
                        ) -> ImportRequestBuilder {
                            ImportRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                            }
                        }
                        #[doc = "Gets a list of catalog items."]
                        pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
                            ListRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                parent: parent.into(),
                                filter: None,
                                page_size: None,
                                page_token: None,
                            }
                        }
                        #[doc = "Updates a catalog item. Partial updating is supported. Non-existing items will be created."]
                        pub fn patch(
                            &self,
                            request : crate :: schemas :: GoogleCloudRecommendationengineV1Beta1CatalogItem,
                            name: impl Into<String>,
                        ) -> PatchRequestBuilder {
                            PatchRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                                update_mask: None,
                            }
                        }
                    }
                    #[doc = "Created via [CatalogItemsActions::create()](struct.CatalogItemsActions.html#method.create)"]
                    #[derive(Debug, Clone)]
                    pub struct CreateRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::GoogleCloudRecommendationengineV1Beta1CatalogItem,
                        parent: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> CreateRequestBuilder<'a> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub async fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields).await
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub async fn execute_with_default_fields(
                            self,
                        ) -> Result<
                            crate::schemas::GoogleCloudRecommendationengineV1Beta1CatalogItem,
                            crate::Error,
                        > {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<
                            crate::schemas::GoogleCloudRecommendationengineV1Beta1CatalogItem,
                            crate::Error,
                        > {
                            self.execute_with_fields(Some("*")).await
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub async fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute().await
                        }
                        async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path()).await?;
                            let req = req.json(&self.request);
                            Ok(req.send().await?.error_for_status()?.json().await?)
                        }
                        fn _path(&self) -> String {
                            let mut output =
                                "https://recommendationengine.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/catalogItems");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                            req = req.query(&[("access_token", &self.access_token)]);
                            req = req.query(&[("alt", &self.alt)]);
                            req = req.query(&[("callback", &self.callback)]);
                            req = req.query(&[("fields", &self.fields)]);
                            req = req.query(&[("key", &self.key)]);
                            req = req.query(&[("oauth_token", &self.oauth_token)]);
                            req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            req = req.query(&[("quotaUser", &self.quota_user)]);
                            req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            req = req.query(&[("uploadType", &self.upload_type)]);
                            req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let access_token = self
                                .auth
                                .access_token()
                                .await
                                .map_err(|err| crate::Error::OAuth2(err))?;
                            req = req.bearer_auth(access_token);
                            Ok(req)
                        }
                    }
                    #[doc = "Created via [CatalogItemsActions::delete()](struct.CatalogItemsActions.html#method.delete)"]
                    #[derive(Debug, Clone)]
                    pub struct DeleteRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> DeleteRequestBuilder<'a> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub async fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields).await
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub async fn execute_with_default_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                        {
                            self.execute_with_fields(Some("*")).await
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub async fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute().await
                        }
                        async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path()).await?;
                            Ok(req.send().await?.error_for_status()?.json().await?)
                        }
                        fn _path(&self) -> String {
                            let mut output =
                                "https://recommendationengine.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::DELETE, path);
                            req = req.query(&[("access_token", &self.access_token)]);
                            req = req.query(&[("alt", &self.alt)]);
                            req = req.query(&[("callback", &self.callback)]);
                            req = req.query(&[("fields", &self.fields)]);
                            req = req.query(&[("key", &self.key)]);
                            req = req.query(&[("oauth_token", &self.oauth_token)]);
                            req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            req = req.query(&[("quotaUser", &self.quota_user)]);
                            req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            req = req.query(&[("uploadType", &self.upload_type)]);
                            req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let access_token = self
                                .auth
                                .access_token()
                                .await
                                .map_err(|err| crate::Error::OAuth2(err))?;
                            req = req.bearer_auth(access_token);
                            Ok(req)
                        }
                    }
                    #[doc = "Created via [CatalogItemsActions::get()](struct.CatalogItemsActions.html#method.get)"]
                    #[derive(Debug, Clone)]
                    pub struct GetRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> GetRequestBuilder<'a> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub async fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields).await
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub async fn execute_with_default_fields(
                            self,
                        ) -> Result<
                            crate::schemas::GoogleCloudRecommendationengineV1Beta1CatalogItem,
                            crate::Error,
                        > {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<
                            crate::schemas::GoogleCloudRecommendationengineV1Beta1CatalogItem,
                            crate::Error,
                        > {
                            self.execute_with_fields(Some("*")).await
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub async fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute().await
                        }
                        async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path()).await?;
                            Ok(req.send().await?.error_for_status()?.json().await?)
                        }
                        fn _path(&self) -> String {
                            let mut output =
                                "https://recommendationengine.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                            req = req.query(&[("access_token", &self.access_token)]);
                            req = req.query(&[("alt", &self.alt)]);
                            req = req.query(&[("callback", &self.callback)]);
                            req = req.query(&[("fields", &self.fields)]);
                            req = req.query(&[("key", &self.key)]);
                            req = req.query(&[("oauth_token", &self.oauth_token)]);
                            req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            req = req.query(&[("quotaUser", &self.quota_user)]);
                            req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            req = req.query(&[("uploadType", &self.upload_type)]);
                            req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let access_token = self
                                .auth
                                .access_token()
                                .await
                                .map_err(|err| crate::Error::OAuth2(err))?;
                            req = req.bearer_auth(access_token);
                            Ok(req)
                        }
                    }
                    #[doc = "Created via [CatalogItemsActions::import()](struct.CatalogItemsActions.html#method.import)"]
                    #[derive(Debug, Clone)]
                    pub struct ImportRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , request : crate :: schemas :: GoogleCloudRecommendationengineV1Beta1ImportCatalogItemsRequest , parent : String , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
                    impl<'a> ImportRequestBuilder<'a> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub async fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields).await
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub async fn execute_with_default_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(Some("*")).await
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub async fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute().await
                        }
                        async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path()).await?;
                            let req = req.json(&self.request);
                            Ok(req.send().await?.error_for_status()?.json().await?)
                        }
                        fn _path(&self) -> String {
                            let mut output =
                                "https://recommendationengine.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/catalogItems:import");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                            req = req.query(&[("access_token", &self.access_token)]);
                            req = req.query(&[("alt", &self.alt)]);
                            req = req.query(&[("callback", &self.callback)]);
                            req = req.query(&[("fields", &self.fields)]);
                            req = req.query(&[("key", &self.key)]);
                            req = req.query(&[("oauth_token", &self.oauth_token)]);
                            req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            req = req.query(&[("quotaUser", &self.quota_user)]);
                            req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            req = req.query(&[("uploadType", &self.upload_type)]);
                            req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let access_token = self
                                .auth
                                .access_token()
                                .await
                                .map_err(|err| crate::Error::OAuth2(err))?;
                            req = req.bearer_auth(access_token);
                            Ok(req)
                        }
                    }
                    #[doc = "Created via [CatalogItemsActions::list()](struct.CatalogItemsActions.html#method.list)"]
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        parent: String,
                        filter: Option<String>,
                        page_size: Option<i32>,
                        page_token: Option<String>,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> ListRequestBuilder<'a> {
                        #[doc = "Optional. Use of this field is not supported by version v1beta1."]
                        pub fn filter(mut self, value: impl Into<String>) -> Self {
                            self.filter = Some(value.into());
                            self
                        }
                        #[doc = "Optional. Maximum number of results to return per page. If zero, the service will choose a reasonable default."]
                        pub fn page_size(mut self, value: i32) -> Self {
                            self.page_size = Some(value);
                            self
                        }
                        #[doc = "Optional. The previous ListCatalogItemsResponse.next_page_token."]
                        pub fn page_token(mut self, value: impl Into<String>) -> Self {
                            self.page_token = Some(value.into());
                            self
                        }
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub async fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields).await
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]                        pub async fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleCloudRecommendationengineV1Beta1ListCatalogItemsResponse , crate :: Error >{
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]                        pub async fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleCloudRecommendationengineV1Beta1ListCatalogItemsResponse , crate :: Error >{
                            self.execute_with_fields(Some("*")).await
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub async fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute().await
                        }
                        async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path()).await?;
                            Ok(req.send().await?.error_for_status()?.json().await?)
                        }
                        fn _path(&self) -> String {
                            let mut output =
                                "https://recommendationengine.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/catalogItems");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                            req = req.query(&[("filter", &self.filter)]);
                            req = req.query(&[("pageSize", &self.page_size)]);
                            req = req.query(&[("pageToken", &self.page_token)]);
                            req = req.query(&[("access_token", &self.access_token)]);
                            req = req.query(&[("alt", &self.alt)]);
                            req = req.query(&[("callback", &self.callback)]);
                            req = req.query(&[("fields", &self.fields)]);
                            req = req.query(&[("key", &self.key)]);
                            req = req.query(&[("oauth_token", &self.oauth_token)]);
                            req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            req = req.query(&[("quotaUser", &self.quota_user)]);
                            req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            req = req.query(&[("uploadType", &self.upload_type)]);
                            req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let access_token = self
                                .auth
                                .access_token()
                                .await
                                .map_err(|err| crate::Error::OAuth2(err))?;
                            req = req.bearer_auth(access_token);
                            Ok(req)
                        }
                    }
                    #[doc = "Created via [CatalogItemsActions::patch()](struct.CatalogItemsActions.html#method.patch)"]
                    #[derive(Debug, Clone)]
                    pub struct PatchRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        request: crate::schemas::GoogleCloudRecommendationengineV1Beta1CatalogItem,
                        name: String,
                        update_mask: Option<String>,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> PatchRequestBuilder<'a> {
                        #[doc = "Optional. Indicates which fields in the provided 'item' to update. If not set, will by default update all fields."]
                        pub fn update_mask(mut self, value: impl Into<String>) -> Self {
                            self.update_mask = Some(value.into());
                            self
                        }
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub async fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields).await
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub async fn execute_with_default_fields(
                            self,
                        ) -> Result<
                            crate::schemas::GoogleCloudRecommendationengineV1Beta1CatalogItem,
                            crate::Error,
                        > {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<
                            crate::schemas::GoogleCloudRecommendationengineV1Beta1CatalogItem,
                            crate::Error,
                        > {
                            self.execute_with_fields(Some("*")).await
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub async fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute().await
                        }
                        async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path()).await?;
                            let req = req.json(&self.request);
                            Ok(req.send().await?.error_for_status()?.json().await?)
                        }
                        fn _path(&self) -> String {
                            let mut output =
                                "https://recommendationengine.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::PATCH, path);
                            req = req.query(&[("updateMask", &self.update_mask)]);
                            req = req.query(&[("access_token", &self.access_token)]);
                            req = req.query(&[("alt", &self.alt)]);
                            req = req.query(&[("callback", &self.callback)]);
                            req = req.query(&[("fields", &self.fields)]);
                            req = req.query(&[("key", &self.key)]);
                            req = req.query(&[("oauth_token", &self.oauth_token)]);
                            req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            req = req.query(&[("quotaUser", &self.quota_user)]);
                            req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            req = req.query(&[("uploadType", &self.upload_type)]);
                            req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let access_token = self
                                .auth
                                .access_token()
                                .await
                                .map_err(|err| crate::Error::OAuth2(err))?;
                            req = req.bearer_auth(access_token);
                            Ok(req)
                        }
                    }
                }
                pub mod event_stores {
                    pub mod params {}
                    pub struct EventStoresActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> EventStoresActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Actions that can be performed on the operations resource"]                        pub fn operations (& self) -> crate :: resources :: projects :: locations :: catalogs :: event_stores :: operations :: OperationsActions{
                            crate :: resources :: projects :: locations :: catalogs :: event_stores :: operations :: OperationsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                        }
                        #[doc = "Actions that can be performed on the placements resource"]                        pub fn placements (& self) -> crate :: resources :: projects :: locations :: catalogs :: event_stores :: placements :: PlacementsActions{
                            crate :: resources :: projects :: locations :: catalogs :: event_stores :: placements :: PlacementsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                        }
                        #[doc = "Actions that can be performed on the prediction_api_key_registrations resource"]                        pub fn prediction_api_key_registrations (& self) -> crate :: resources :: projects :: locations :: catalogs :: event_stores :: prediction_api_key_registrations :: PredictionApiKeyRegistrationsActions{
                            crate :: resources :: projects :: locations :: catalogs :: event_stores :: prediction_api_key_registrations :: PredictionApiKeyRegistrationsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                        }
                        #[doc = "Actions that can be performed on the user_events resource"]                        pub fn user_events (& self) -> crate :: resources :: projects :: locations :: catalogs :: event_stores :: user_events :: UserEventsActions{
                            crate :: resources :: projects :: locations :: catalogs :: event_stores :: user_events :: UserEventsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                        }
                    }
                    pub mod operations {
                        pub mod params {}
                        pub struct OperationsActions<'a> {
                            pub(crate) reqwest: &'a reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        }
                        impl<'a> OperationsActions<'a> {
                            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                                self.auth
                            }
                            #[doc = "Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."]
                            pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                                GetRequestBuilder {
                                    reqwest: &self.reqwest,
                                    auth: self.auth_ref(),
                                    access_token: None,
                                    alt: None,
                                    callback: None,
                                    fields: None,
                                    key: None,
                                    oauth_token: None,
                                    pretty_print: None,
                                    quota_user: None,
                                    upload_protocol: None,
                                    upload_type: None,
                                    xgafv: None,
                                    name: name.into(),
                                }
                            }
                            #[doc = "Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id."]
                            pub fn list(&self, name: impl Into<String>) -> ListRequestBuilder {
                                ListRequestBuilder {
                                    reqwest: &self.reqwest,
                                    auth: self.auth_ref(),
                                    access_token: None,
                                    alt: None,
                                    callback: None,
                                    fields: None,
                                    key: None,
                                    oauth_token: None,
                                    pretty_print: None,
                                    quota_user: None,
                                    upload_protocol: None,
                                    upload_type: None,
                                    xgafv: None,
                                    name: name.into(),
                                    filter: None,
                                    page_size: None,
                                    page_token: None,
                                }
                            }
                        }
                        #[doc = "Created via [OperationsActions::get()](struct.OperationsActions.html#method.get)"]
                        #[derive(Debug, Clone)]
                        pub struct GetRequestBuilder<'a> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                            name: String,
                            access_token: Option<String>,
                            alt: Option<crate::params::Alt>,
                            callback: Option<String>,
                            fields: Option<String>,
                            key: Option<String>,
                            oauth_token: Option<String>,
                            pretty_print: Option<bool>,
                            quota_user: Option<String>,
                            upload_protocol: Option<String>,
                            upload_type: Option<String>,
                            xgafv: Option<crate::params::Xgafv>,
                        }
                        impl<'a> GetRequestBuilder<'a> {
                            #[doc = "OAuth access token."]
                            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                                self.access_token = Some(value.into());
                                self
                            }
                            #[doc = "JSONP"]
                            pub fn callback(mut self, value: impl Into<String>) -> Self {
                                self.callback = Some(value.into());
                                self
                            }
                            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                            pub fn key(mut self, value: impl Into<String>) -> Self {
                                self.key = Some(value.into());
                                self
                            }
                            #[doc = "OAuth 2.0 token for the current user."]
                            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                                self.oauth_token = Some(value.into());
                                self
                            }
                            #[doc = "Returns response with indentations and line breaks."]
                            pub fn pretty_print(mut self, value: bool) -> Self {
                                self.pretty_print = Some(value);
                                self
                            }
                            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                                self.quota_user = Some(value.into());
                                self
                            }
                            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                                self.upload_protocol = Some(value.into());
                                self
                            }
                            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                                self.upload_type = Some(value.into());
                                self
                            }
                            #[doc = "V1 error format."]
                            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                                self.xgafv = Some(value);
                                self
                            }
                            #[doc = r" Execute the given operation. The fields requested are"]
                            #[doc = r" determined by the FieldSelector attribute of the return type."]
                            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                            #[doc = r" are not generic over the return type and deserialize the"]
                            #[doc = r" response into an auto-generated struct will all possible"]
                            #[doc = r" fields."]
                            pub async fn execute<T>(self) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned
                                    + ::google_field_selector::FieldSelector,
                            {
                                let fields = ::google_field_selector::to_string::<T>();
                                let fields: Option<String> = if fields.is_empty() {
                                    None
                                } else {
                                    Some(fields)
                                };
                                self.execute_with_fields(fields).await
                            }
                            #[doc = r" Execute the given operation. This will not provide any"]
                            #[doc = r" `fields` selector indicating that the server will determine"]
                            #[doc = r" the fields returned. This typically includes the most common"]
                            #[doc = r" fields, but it will not include every possible attribute of"]
                            #[doc = r" the response resource."]
                            pub async fn execute_with_default_fields(
                                self,
                            ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                            {
                                self.execute_with_fields(Some("*")).await
                            }
                            #[doc = r" Execute the given operation. This will use the `fields`"]
                            #[doc = r" selector provided and will deserialize the response into"]
                            #[doc = r" whatever return value is provided."]
                            pub async fn execute_with_fields<T, F>(
                                mut self,
                                fields: Option<F>,
                            ) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned,
                                F: Into<String>,
                            {
                                self.fields = fields.map(Into::into);
                                self._execute().await
                            }
                            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned,
                            {
                                let req = self._request(&self._path()).await?;
                                Ok(req.send().await?.error_for_status()?.json().await?)
                            }
                            fn _path(&self) -> String {
                                let mut output =
                                    "https://recommendationengine.googleapis.com/".to_owned();
                                output.push_str("v1beta1/");
                                {
                                    let var_as_str = &self.name;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::RESERVED,
                                    ));
                                }
                                output
                            }
                            async fn _request(
                                &self,
                                path: &str,
                            ) -> Result<::reqwest::RequestBuilder, crate::Error>
                            {
                                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                                req = req.query(&[("access_token", &self.access_token)]);
                                req = req.query(&[("alt", &self.alt)]);
                                req = req.query(&[("callback", &self.callback)]);
                                req = req.query(&[("fields", &self.fields)]);
                                req = req.query(&[("key", &self.key)]);
                                req = req.query(&[("oauth_token", &self.oauth_token)]);
                                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                req = req.query(&[("quotaUser", &self.quota_user)]);
                                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                                req = req.query(&[("uploadType", &self.upload_type)]);
                                req = req.query(&[("$.xgafv", &self.xgafv)]);
                                let access_token = self
                                    .auth
                                    .access_token()
                                    .await
                                    .map_err(|err| crate::Error::OAuth2(err))?;
                                req = req.bearer_auth(access_token);
                                Ok(req)
                            }
                        }
                        #[doc = "Created via [OperationsActions::list()](struct.OperationsActions.html#method.list)"]
                        #[derive(Debug, Clone)]
                        pub struct ListRequestBuilder<'a> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                            name: String,
                            filter: Option<String>,
                            page_size: Option<i32>,
                            page_token: Option<String>,
                            access_token: Option<String>,
                            alt: Option<crate::params::Alt>,
                            callback: Option<String>,
                            fields: Option<String>,
                            key: Option<String>,
                            oauth_token: Option<String>,
                            pretty_print: Option<bool>,
                            quota_user: Option<String>,
                            upload_protocol: Option<String>,
                            upload_type: Option<String>,
                            xgafv: Option<crate::params::Xgafv>,
                        }
                        impl<'a> ListRequestBuilder<'a> {
                            #[doc = "The standard list filter."]
                            pub fn filter(mut self, value: impl Into<String>) -> Self {
                                self.filter = Some(value.into());
                                self
                            }
                            #[doc = "The standard list page size."]
                            pub fn page_size(mut self, value: i32) -> Self {
                                self.page_size = Some(value);
                                self
                            }
                            #[doc = "The standard list page token."]
                            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                                self.page_token = Some(value.into());
                                self
                            }
                            #[doc = "OAuth access token."]
                            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                                self.access_token = Some(value.into());
                                self
                            }
                            #[doc = "JSONP"]
                            pub fn callback(mut self, value: impl Into<String>) -> Self {
                                self.callback = Some(value.into());
                                self
                            }
                            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                            pub fn key(mut self, value: impl Into<String>) -> Self {
                                self.key = Some(value.into());
                                self
                            }
                            #[doc = "OAuth 2.0 token for the current user."]
                            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                                self.oauth_token = Some(value.into());
                                self
                            }
                            #[doc = "Returns response with indentations and line breaks."]
                            pub fn pretty_print(mut self, value: bool) -> Self {
                                self.pretty_print = Some(value);
                                self
                            }
                            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                                self.quota_user = Some(value.into());
                                self
                            }
                            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                                self.upload_protocol = Some(value.into());
                                self
                            }
                            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                                self.upload_type = Some(value.into());
                                self
                            }
                            #[doc = "V1 error format."]
                            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                                self.xgafv = Some(value);
                                self
                            }
                            #[doc = r" Execute the given operation. The fields requested are"]
                            #[doc = r" determined by the FieldSelector attribute of the return type."]
                            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                            #[doc = r" are not generic over the return type and deserialize the"]
                            #[doc = r" response into an auto-generated struct will all possible"]
                            #[doc = r" fields."]
                            pub async fn execute<T>(self) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned
                                    + ::google_field_selector::FieldSelector,
                            {
                                let fields = ::google_field_selector::to_string::<T>();
                                let fields: Option<String> = if fields.is_empty() {
                                    None
                                } else {
                                    Some(fields)
                                };
                                self.execute_with_fields(fields).await
                            }
                            #[doc = r" Execute the given operation. This will not provide any"]
                            #[doc = r" `fields` selector indicating that the server will determine"]
                            #[doc = r" the fields returned. This typically includes the most common"]
                            #[doc = r" fields, but it will not include every possible attribute of"]
                            #[doc = r" the response resource."]
                            pub async fn execute_with_default_fields(
                                self,
                            ) -> Result<
                                crate::schemas::GoogleLongrunningListOperationsResponse,
                                crate::Error,
                            > {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<
                                crate::schemas::GoogleLongrunningListOperationsResponse,
                                crate::Error,
                            > {
                                self.execute_with_fields(Some("*")).await
                            }
                            #[doc = r" Execute the given operation. This will use the `fields`"]
                            #[doc = r" selector provided and will deserialize the response into"]
                            #[doc = r" whatever return value is provided."]
                            pub async fn execute_with_fields<T, F>(
                                mut self,
                                fields: Option<F>,
                            ) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned,
                                F: Into<String>,
                            {
                                self.fields = fields.map(Into::into);
                                self._execute().await
                            }
                            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned,
                            {
                                let req = self._request(&self._path()).await?;
                                Ok(req.send().await?.error_for_status()?.json().await?)
                            }
                            fn _path(&self) -> String {
                                let mut output =
                                    "https://recommendationengine.googleapis.com/".to_owned();
                                output.push_str("v1beta1/");
                                {
                                    let var_as_str = &self.name;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::RESERVED,
                                    ));
                                }
                                output.push_str("/operations");
                                output
                            }
                            async fn _request(
                                &self,
                                path: &str,
                            ) -> Result<::reqwest::RequestBuilder, crate::Error>
                            {
                                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                                req = req.query(&[("filter", &self.filter)]);
                                req = req.query(&[("pageSize", &self.page_size)]);
                                req = req.query(&[("pageToken", &self.page_token)]);
                                req = req.query(&[("access_token", &self.access_token)]);
                                req = req.query(&[("alt", &self.alt)]);
                                req = req.query(&[("callback", &self.callback)]);
                                req = req.query(&[("fields", &self.fields)]);
                                req = req.query(&[("key", &self.key)]);
                                req = req.query(&[("oauth_token", &self.oauth_token)]);
                                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                req = req.query(&[("quotaUser", &self.quota_user)]);
                                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                                req = req.query(&[("uploadType", &self.upload_type)]);
                                req = req.query(&[("$.xgafv", &self.xgafv)]);
                                let access_token = self
                                    .auth
                                    .access_token()
                                    .await
                                    .map_err(|err| crate::Error::OAuth2(err))?;
                                req = req.bearer_auth(access_token);
                                Ok(req)
                            }
                        }
                    }
                    pub mod placements {
                        pub mod params {}
                        pub struct PlacementsActions<'a> {
                            pub(crate) reqwest: &'a reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        }
                        impl<'a> PlacementsActions<'a> {
                            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                                self.auth
                            }
                            #[doc = "Makes a recommendation prediction. If using API Key based authentication, the API Key must be registered using the PredictionApiKeyRegistry service. [Learn more](https://cloud.google.com/recommendations-ai/docs/setting-up#register-key)."]
                            pub fn predict(
                                &self,
                                request : crate :: schemas :: GoogleCloudRecommendationengineV1Beta1PredictRequest,
                                name: impl Into<String>,
                            ) -> PredictRequestBuilder {
                                PredictRequestBuilder {
                                    reqwest: &self.reqwest,
                                    auth: self.auth_ref(),
                                    request,
                                    access_token: None,
                                    alt: None,
                                    callback: None,
                                    fields: None,
                                    key: None,
                                    oauth_token: None,
                                    pretty_print: None,
                                    quota_user: None,
                                    upload_protocol: None,
                                    upload_type: None,
                                    xgafv: None,
                                    name: name.into(),
                                }
                            }
                        }
                        #[doc = "Created via [PlacementsActions::predict()](struct.PlacementsActions.html#method.predict)"]
                        #[derive(Debug, Clone)]
                        pub struct PredictRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , request : crate :: schemas :: GoogleCloudRecommendationengineV1Beta1PredictRequest , name : String , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
                        impl<'a> PredictRequestBuilder<'a> {
                            #[doc = "OAuth access token."]
                            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                                self.access_token = Some(value.into());
                                self
                            }
                            #[doc = "JSONP"]
                            pub fn callback(mut self, value: impl Into<String>) -> Self {
                                self.callback = Some(value.into());
                                self
                            }
                            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                            pub fn key(mut self, value: impl Into<String>) -> Self {
                                self.key = Some(value.into());
                                self
                            }
                            #[doc = "OAuth 2.0 token for the current user."]
                            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                                self.oauth_token = Some(value.into());
                                self
                            }
                            #[doc = "Returns response with indentations and line breaks."]
                            pub fn pretty_print(mut self, value: bool) -> Self {
                                self.pretty_print = Some(value);
                                self
                            }
                            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                                self.quota_user = Some(value.into());
                                self
                            }
                            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                                self.upload_protocol = Some(value.into());
                                self
                            }
                            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                                self.upload_type = Some(value.into());
                                self
                            }
                            #[doc = "V1 error format."]
                            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                                self.xgafv = Some(value);
                                self
                            }
                            #[doc = r" Execute the given operation. The fields requested are"]
                            #[doc = r" determined by the FieldSelector attribute of the return type."]
                            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                            #[doc = r" are not generic over the return type and deserialize the"]
                            #[doc = r" response into an auto-generated struct will all possible"]
                            #[doc = r" fields."]
                            pub async fn execute<T>(self) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned
                                    + ::google_field_selector::FieldSelector,
                            {
                                let fields = ::google_field_selector::to_string::<T>();
                                let fields: Option<String> = if fields.is_empty() {
                                    None
                                } else {
                                    Some(fields)
                                };
                                self.execute_with_fields(fields).await
                            }
                            #[doc = r" Execute the given operation. This will not provide any"]
                            #[doc = r" `fields` selector indicating that the server will determine"]
                            #[doc = r" the fields returned. This typically includes the most common"]
                            #[doc = r" fields, but it will not include every possible attribute of"]
                            #[doc = r" the response resource."]                            pub async fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleCloudRecommendationengineV1Beta1PredictResponse , crate :: Error >{
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]                            pub async fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleCloudRecommendationengineV1Beta1PredictResponse , crate :: Error >{
                                self.execute_with_fields(Some("*")).await
                            }
                            #[doc = r" Execute the given operation. This will use the `fields`"]
                            #[doc = r" selector provided and will deserialize the response into"]
                            #[doc = r" whatever return value is provided."]
                            pub async fn execute_with_fields<T, F>(
                                mut self,
                                fields: Option<F>,
                            ) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned,
                                F: Into<String>,
                            {
                                self.fields = fields.map(Into::into);
                                self._execute().await
                            }
                            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned,
                            {
                                let req = self._request(&self._path()).await?;
                                let req = req.json(&self.request);
                                Ok(req.send().await?.error_for_status()?.json().await?)
                            }
                            fn _path(&self) -> String {
                                let mut output =
                                    "https://recommendationengine.googleapis.com/".to_owned();
                                output.push_str("v1beta1/");
                                {
                                    let var_as_str = &self.name;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::RESERVED,
                                    ));
                                }
                                output.push_str(":predict");
                                output
                            }
                            async fn _request(
                                &self,
                                path: &str,
                            ) -> Result<::reqwest::RequestBuilder, crate::Error>
                            {
                                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                                req = req.query(&[("access_token", &self.access_token)]);
                                req = req.query(&[("alt", &self.alt)]);
                                req = req.query(&[("callback", &self.callback)]);
                                req = req.query(&[("fields", &self.fields)]);
                                req = req.query(&[("key", &self.key)]);
                                req = req.query(&[("oauth_token", &self.oauth_token)]);
                                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                req = req.query(&[("quotaUser", &self.quota_user)]);
                                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                                req = req.query(&[("uploadType", &self.upload_type)]);
                                req = req.query(&[("$.xgafv", &self.xgafv)]);
                                let access_token = self
                                    .auth
                                    .access_token()
                                    .await
                                    .map_err(|err| crate::Error::OAuth2(err))?;
                                req = req.bearer_auth(access_token);
                                Ok(req)
                            }
                        }
                    }
                    pub mod prediction_api_key_registrations {
                        pub mod params {}
                        pub struct PredictionApiKeyRegistrationsActions<'a> {
                            pub(crate) reqwest: &'a reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        }
                        impl<'a> PredictionApiKeyRegistrationsActions<'a> {
                            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                                self.auth
                            }
                            #[doc = "Register an API key for use with predict method."]
                            pub fn create(
                                &self,
                                request : crate :: schemas :: GoogleCloudRecommendationengineV1Beta1CreatePredictionApiKeyRegistrationRequest,
                                parent: impl Into<String>,
                            ) -> CreateRequestBuilder {
                                CreateRequestBuilder {
                                    reqwest: &self.reqwest,
                                    auth: self.auth_ref(),
                                    request,
                                    access_token: None,
                                    alt: None,
                                    callback: None,
                                    fields: None,
                                    key: None,
                                    oauth_token: None,
                                    pretty_print: None,
                                    quota_user: None,
                                    upload_protocol: None,
                                    upload_type: None,
                                    xgafv: None,
                                    parent: parent.into(),
                                }
                            }
                            #[doc = "Unregister an apiKey from using for predict method."]
                            pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder {
                                DeleteRequestBuilder {
                                    reqwest: &self.reqwest,
                                    auth: self.auth_ref(),
                                    access_token: None,
                                    alt: None,
                                    callback: None,
                                    fields: None,
                                    key: None,
                                    oauth_token: None,
                                    pretty_print: None,
                                    quota_user: None,
                                    upload_protocol: None,
                                    upload_type: None,
                                    xgafv: None,
                                    name: name.into(),
                                }
                            }
                            #[doc = "List the registered apiKeys for use with predict method."]
                            pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
                                ListRequestBuilder {
                                    reqwest: &self.reqwest,
                                    auth: self.auth_ref(),
                                    access_token: None,
                                    alt: None,
                                    callback: None,
                                    fields: None,
                                    key: None,
                                    oauth_token: None,
                                    pretty_print: None,
                                    quota_user: None,
                                    upload_protocol: None,
                                    upload_type: None,
                                    xgafv: None,
                                    parent: parent.into(),
                                    page_size: None,
                                    page_token: None,
                                }
                            }
                        }
                        #[doc = "Created via [PredictionApiKeyRegistrationsActions::create()](struct.PredictionApiKeyRegistrationsActions.html#method.create)"]
                        #[derive(Debug, Clone)]
                        pub struct CreateRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , request : crate :: schemas :: GoogleCloudRecommendationengineV1Beta1CreatePredictionApiKeyRegistrationRequest , parent : String , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
                        impl<'a> CreateRequestBuilder<'a> {
                            #[doc = "OAuth access token."]
                            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                                self.access_token = Some(value.into());
                                self
                            }
                            #[doc = "JSONP"]
                            pub fn callback(mut self, value: impl Into<String>) -> Self {
                                self.callback = Some(value.into());
                                self
                            }
                            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                            pub fn key(mut self, value: impl Into<String>) -> Self {
                                self.key = Some(value.into());
                                self
                            }
                            #[doc = "OAuth 2.0 token for the current user."]
                            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                                self.oauth_token = Some(value.into());
                                self
                            }
                            #[doc = "Returns response with indentations and line breaks."]
                            pub fn pretty_print(mut self, value: bool) -> Self {
                                self.pretty_print = Some(value);
                                self
                            }
                            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                                self.quota_user = Some(value.into());
                                self
                            }
                            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                                self.upload_protocol = Some(value.into());
                                self
                            }
                            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                                self.upload_type = Some(value.into());
                                self
                            }
                            #[doc = "V1 error format."]
                            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                                self.xgafv = Some(value);
                                self
                            }
                            #[doc = r" Execute the given operation. The fields requested are"]
                            #[doc = r" determined by the FieldSelector attribute of the return type."]
                            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                            #[doc = r" are not generic over the return type and deserialize the"]
                            #[doc = r" response into an auto-generated struct will all possible"]
                            #[doc = r" fields."]
                            pub async fn execute<T>(self) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned
                                    + ::google_field_selector::FieldSelector,
                            {
                                let fields = ::google_field_selector::to_string::<T>();
                                let fields: Option<String> = if fields.is_empty() {
                                    None
                                } else {
                                    Some(fields)
                                };
                                self.execute_with_fields(fields).await
                            }
                            #[doc = r" Execute the given operation. This will not provide any"]
                            #[doc = r" `fields` selector indicating that the server will determine"]
                            #[doc = r" the fields returned. This typically includes the most common"]
                            #[doc = r" fields, but it will not include every possible attribute of"]
                            #[doc = r" the response resource."]                            pub async fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleCloudRecommendationengineV1Beta1PredictionApiKeyRegistration , crate :: Error >{
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]                            pub async fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleCloudRecommendationengineV1Beta1PredictionApiKeyRegistration , crate :: Error >{
                                self.execute_with_fields(Some("*")).await
                            }
                            #[doc = r" Execute the given operation. This will use the `fields`"]
                            #[doc = r" selector provided and will deserialize the response into"]
                            #[doc = r" whatever return value is provided."]
                            pub async fn execute_with_fields<T, F>(
                                mut self,
                                fields: Option<F>,
                            ) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned,
                                F: Into<String>,
                            {
                                self.fields = fields.map(Into::into);
                                self._execute().await
                            }
                            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned,
                            {
                                let req = self._request(&self._path()).await?;
                                let req = req.json(&self.request);
                                Ok(req.send().await?.error_for_status()?.json().await?)
                            }
                            fn _path(&self) -> String {
                                let mut output =
                                    "https://recommendationengine.googleapis.com/".to_owned();
                                output.push_str("v1beta1/");
                                {
                                    let var_as_str = &self.parent;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::RESERVED,
                                    ));
                                }
                                output.push_str("/predictionApiKeyRegistrations");
                                output
                            }
                            async fn _request(
                                &self,
                                path: &str,
                            ) -> Result<::reqwest::RequestBuilder, crate::Error>
                            {
                                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                                req = req.query(&[("access_token", &self.access_token)]);
                                req = req.query(&[("alt", &self.alt)]);
                                req = req.query(&[("callback", &self.callback)]);
                                req = req.query(&[("fields", &self.fields)]);
                                req = req.query(&[("key", &self.key)]);
                                req = req.query(&[("oauth_token", &self.oauth_token)]);
                                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                req = req.query(&[("quotaUser", &self.quota_user)]);
                                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                                req = req.query(&[("uploadType", &self.upload_type)]);
                                req = req.query(&[("$.xgafv", &self.xgafv)]);
                                let access_token = self
                                    .auth
                                    .access_token()
                                    .await
                                    .map_err(|err| crate::Error::OAuth2(err))?;
                                req = req.bearer_auth(access_token);
                                Ok(req)
                            }
                        }
                        #[doc = "Created via [PredictionApiKeyRegistrationsActions::delete()](struct.PredictionApiKeyRegistrationsActions.html#method.delete)"]
                        #[derive(Debug, Clone)]
                        pub struct DeleteRequestBuilder<'a> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                            name: String,
                            access_token: Option<String>,
                            alt: Option<crate::params::Alt>,
                            callback: Option<String>,
                            fields: Option<String>,
                            key: Option<String>,
                            oauth_token: Option<String>,
                            pretty_print: Option<bool>,
                            quota_user: Option<String>,
                            upload_protocol: Option<String>,
                            upload_type: Option<String>,
                            xgafv: Option<crate::params::Xgafv>,
                        }
                        impl<'a> DeleteRequestBuilder<'a> {
                            #[doc = "OAuth access token."]
                            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                                self.access_token = Some(value.into());
                                self
                            }
                            #[doc = "JSONP"]
                            pub fn callback(mut self, value: impl Into<String>) -> Self {
                                self.callback = Some(value.into());
                                self
                            }
                            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                            pub fn key(mut self, value: impl Into<String>) -> Self {
                                self.key = Some(value.into());
                                self
                            }
                            #[doc = "OAuth 2.0 token for the current user."]
                            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                                self.oauth_token = Some(value.into());
                                self
                            }
                            #[doc = "Returns response with indentations and line breaks."]
                            pub fn pretty_print(mut self, value: bool) -> Self {
                                self.pretty_print = Some(value);
                                self
                            }
                            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                                self.quota_user = Some(value.into());
                                self
                            }
                            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                                self.upload_protocol = Some(value.into());
                                self
                            }
                            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                                self.upload_type = Some(value.into());
                                self
                            }
                            #[doc = "V1 error format."]
                            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                                self.xgafv = Some(value);
                                self
                            }
                            #[doc = r" Execute the given operation. The fields requested are"]
                            #[doc = r" determined by the FieldSelector attribute of the return type."]
                            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                            #[doc = r" are not generic over the return type and deserialize the"]
                            #[doc = r" response into an auto-generated struct will all possible"]
                            #[doc = r" fields."]
                            pub async fn execute<T>(self) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned
                                    + ::google_field_selector::FieldSelector,
                            {
                                let fields = ::google_field_selector::to_string::<T>();
                                let fields: Option<String> = if fields.is_empty() {
                                    None
                                } else {
                                    Some(fields)
                                };
                                self.execute_with_fields(fields).await
                            }
                            #[doc = r" Execute the given operation. This will not provide any"]
                            #[doc = r" `fields` selector indicating that the server will determine"]
                            #[doc = r" the fields returned. This typically includes the most common"]
                            #[doc = r" fields, but it will not include every possible attribute of"]
                            #[doc = r" the response resource."]
                            pub async fn execute_with_default_fields(
                                self,
                            ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::GoogleProtobufEmpty, crate::Error>
                            {
                                self.execute_with_fields(Some("*")).await
                            }
                            #[doc = r" Execute the given operation. This will use the `fields`"]
                            #[doc = r" selector provided and will deserialize the response into"]
                            #[doc = r" whatever return value is provided."]
                            pub async fn execute_with_fields<T, F>(
                                mut self,
                                fields: Option<F>,
                            ) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned,
                                F: Into<String>,
                            {
                                self.fields = fields.map(Into::into);
                                self._execute().await
                            }
                            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned,
                            {
                                let req = self._request(&self._path()).await?;
                                Ok(req.send().await?.error_for_status()?.json().await?)
                            }
                            fn _path(&self) -> String {
                                let mut output =
                                    "https://recommendationengine.googleapis.com/".to_owned();
                                output.push_str("v1beta1/");
                                {
                                    let var_as_str = &self.name;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::RESERVED,
                                    ));
                                }
                                output
                            }
                            async fn _request(
                                &self,
                                path: &str,
                            ) -> Result<::reqwest::RequestBuilder, crate::Error>
                            {
                                let mut req = self.reqwest.request(::reqwest::Method::DELETE, path);
                                req = req.query(&[("access_token", &self.access_token)]);
                                req = req.query(&[("alt", &self.alt)]);
                                req = req.query(&[("callback", &self.callback)]);
                                req = req.query(&[("fields", &self.fields)]);
                                req = req.query(&[("key", &self.key)]);
                                req = req.query(&[("oauth_token", &self.oauth_token)]);
                                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                req = req.query(&[("quotaUser", &self.quota_user)]);
                                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                                req = req.query(&[("uploadType", &self.upload_type)]);
                                req = req.query(&[("$.xgafv", &self.xgafv)]);
                                let access_token = self
                                    .auth
                                    .access_token()
                                    .await
                                    .map_err(|err| crate::Error::OAuth2(err))?;
                                req = req.bearer_auth(access_token);
                                Ok(req)
                            }
                        }
                        #[doc = "Created via [PredictionApiKeyRegistrationsActions::list()](struct.PredictionApiKeyRegistrationsActions.html#method.list)"]
                        #[derive(Debug, Clone)]
                        pub struct ListRequestBuilder<'a> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                            parent: String,
                            page_size: Option<i32>,
                            page_token: Option<String>,
                            access_token: Option<String>,
                            alt: Option<crate::params::Alt>,
                            callback: Option<String>,
                            fields: Option<String>,
                            key: Option<String>,
                            oauth_token: Option<String>,
                            pretty_print: Option<bool>,
                            quota_user: Option<String>,
                            upload_protocol: Option<String>,
                            upload_type: Option<String>,
                            xgafv: Option<crate::params::Xgafv>,
                        }
                        impl<'a> ListRequestBuilder<'a> {
                            #[doc = "Optional. Maximum number of results to return per page. If unset, the service will choose a reasonable default."]
                            pub fn page_size(mut self, value: i32) -> Self {
                                self.page_size = Some(value);
                                self
                            }
                            #[doc = "Optional. The previous `ListPredictionApiKeyRegistration.nextPageToken`."]
                            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                                self.page_token = Some(value.into());
                                self
                            }
                            #[doc = "OAuth access token."]
                            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                                self.access_token = Some(value.into());
                                self
                            }
                            #[doc = "JSONP"]
                            pub fn callback(mut self, value: impl Into<String>) -> Self {
                                self.callback = Some(value.into());
                                self
                            }
                            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                            pub fn key(mut self, value: impl Into<String>) -> Self {
                                self.key = Some(value.into());
                                self
                            }
                            #[doc = "OAuth 2.0 token for the current user."]
                            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                                self.oauth_token = Some(value.into());
                                self
                            }
                            #[doc = "Returns response with indentations and line breaks."]
                            pub fn pretty_print(mut self, value: bool) -> Self {
                                self.pretty_print = Some(value);
                                self
                            }
                            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                                self.quota_user = Some(value.into());
                                self
                            }
                            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                                self.upload_protocol = Some(value.into());
                                self
                            }
                            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                                self.upload_type = Some(value.into());
                                self
                            }
                            #[doc = "V1 error format."]
                            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                                self.xgafv = Some(value);
                                self
                            }
                            #[doc = r" Execute the given operation. The fields requested are"]
                            #[doc = r" determined by the FieldSelector attribute of the return type."]
                            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                            #[doc = r" are not generic over the return type and deserialize the"]
                            #[doc = r" response into an auto-generated struct will all possible"]
                            #[doc = r" fields."]
                            pub async fn execute<T>(self) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned
                                    + ::google_field_selector::FieldSelector,
                            {
                                let fields = ::google_field_selector::to_string::<T>();
                                let fields: Option<String> = if fields.is_empty() {
                                    None
                                } else {
                                    Some(fields)
                                };
                                self.execute_with_fields(fields).await
                            }
                            #[doc = r" Execute the given operation. This will not provide any"]
                            #[doc = r" `fields` selector indicating that the server will determine"]
                            #[doc = r" the fields returned. This typically includes the most common"]
                            #[doc = r" fields, but it will not include every possible attribute of"]
                            #[doc = r" the response resource."]                            pub async fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleCloudRecommendationengineV1Beta1ListPredictionApiKeyRegistrationsResponse , crate :: Error >{
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]                            pub async fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleCloudRecommendationengineV1Beta1ListPredictionApiKeyRegistrationsResponse , crate :: Error >{
                                self.execute_with_fields(Some("*")).await
                            }
                            #[doc = r" Execute the given operation. This will use the `fields`"]
                            #[doc = r" selector provided and will deserialize the response into"]
                            #[doc = r" whatever return value is provided."]
                            pub async fn execute_with_fields<T, F>(
                                mut self,
                                fields: Option<F>,
                            ) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned,
                                F: Into<String>,
                            {
                                self.fields = fields.map(Into::into);
                                self._execute().await
                            }
                            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned,
                            {
                                let req = self._request(&self._path()).await?;
                                Ok(req.send().await?.error_for_status()?.json().await?)
                            }
                            fn _path(&self) -> String {
                                let mut output =
                                    "https://recommendationengine.googleapis.com/".to_owned();
                                output.push_str("v1beta1/");
                                {
                                    let var_as_str = &self.parent;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::RESERVED,
                                    ));
                                }
                                output.push_str("/predictionApiKeyRegistrations");
                                output
                            }
                            async fn _request(
                                &self,
                                path: &str,
                            ) -> Result<::reqwest::RequestBuilder, crate::Error>
                            {
                                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                                req = req.query(&[("pageSize", &self.page_size)]);
                                req = req.query(&[("pageToken", &self.page_token)]);
                                req = req.query(&[("access_token", &self.access_token)]);
                                req = req.query(&[("alt", &self.alt)]);
                                req = req.query(&[("callback", &self.callback)]);
                                req = req.query(&[("fields", &self.fields)]);
                                req = req.query(&[("key", &self.key)]);
                                req = req.query(&[("oauth_token", &self.oauth_token)]);
                                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                req = req.query(&[("quotaUser", &self.quota_user)]);
                                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                                req = req.query(&[("uploadType", &self.upload_type)]);
                                req = req.query(&[("$.xgafv", &self.xgafv)]);
                                let access_token = self
                                    .auth
                                    .access_token()
                                    .await
                                    .map_err(|err| crate::Error::OAuth2(err))?;
                                req = req.bearer_auth(access_token);
                                Ok(req)
                            }
                        }
                    }
                    pub mod user_events {
                        pub mod params {}
                        pub struct UserEventsActions<'a> {
                            pub(crate) reqwest: &'a reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        }
                        impl<'a> UserEventsActions<'a> {
                            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                                self.auth
                            }
                            #[doc = "Writes a single user event from the browser. This uses a GET request to due to browser restriction of POST-ing to a 3rd party domain. This method is used only by the Recommendations AI JavaScript pixel. Users should not call this method directly."]
                            pub fn collect(
                                &self,
                                parent: impl Into<String>,
                            ) -> CollectRequestBuilder {
                                CollectRequestBuilder {
                                    reqwest: &self.reqwest,
                                    auth: self.auth_ref(),
                                    access_token: None,
                                    alt: None,
                                    callback: None,
                                    fields: None,
                                    key: None,
                                    oauth_token: None,
                                    pretty_print: None,
                                    quota_user: None,
                                    upload_protocol: None,
                                    upload_type: None,
                                    xgafv: None,
                                    parent: parent.into(),
                                    ets: None,
                                    uri: None,
                                    user_event: None,
                                }
                            }
                            #[doc = "Bulk import of User events. Request processing might be synchronous. Events that already exist are skipped. Use this method for backfilling historical user events. Operation.response is of type ImportResponse. Note that it is possible for a subset of the items to be successfully inserted. Operation.metadata is of type ImportMetadata."]
                            pub fn import(
                                &self,
                                request : crate :: schemas :: GoogleCloudRecommendationengineV1Beta1ImportUserEventsRequest,
                                parent: impl Into<String>,
                            ) -> ImportRequestBuilder {
                                ImportRequestBuilder {
                                    reqwest: &self.reqwest,
                                    auth: self.auth_ref(),
                                    request,
                                    access_token: None,
                                    alt: None,
                                    callback: None,
                                    fields: None,
                                    key: None,
                                    oauth_token: None,
                                    pretty_print: None,
                                    quota_user: None,
                                    upload_protocol: None,
                                    upload_type: None,
                                    xgafv: None,
                                    parent: parent.into(),
                                }
                            }
                            #[doc = "Gets a list of user events within a time range, with potential filtering. The method does not list unjoined user events. Unjoined user event definition: when a user event is ingested from Recommendations AI User Event APIs, the catalog item included in the user event is connected with the current catalog. If a catalog item of the ingested event is not in the current catalog, it could lead to degraded model quality. This is called an unjoined event."]
                            pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder {
                                ListRequestBuilder {
                                    reqwest: &self.reqwest,
                                    auth: self.auth_ref(),
                                    access_token: None,
                                    alt: None,
                                    callback: None,
                                    fields: None,
                                    key: None,
                                    oauth_token: None,
                                    pretty_print: None,
                                    quota_user: None,
                                    upload_protocol: None,
                                    upload_type: None,
                                    xgafv: None,
                                    parent: parent.into(),
                                    filter: None,
                                    page_size: None,
                                    page_token: None,
                                }
                            }
                            #[doc = "Deletes permanently all user events specified by the filter provided. Depending on the number of events specified by the filter, this operation could take hours or days to complete. To test a filter, use the list command first."]
                            pub fn purge(
                                &self,
                                request : crate :: schemas :: GoogleCloudRecommendationengineV1Beta1PurgeUserEventsRequest,
                                parent: impl Into<String>,
                            ) -> PurgeRequestBuilder {
                                PurgeRequestBuilder {
                                    reqwest: &self.reqwest,
                                    auth: self.auth_ref(),
                                    request,
                                    access_token: None,
                                    alt: None,
                                    callback: None,
                                    fields: None,
                                    key: None,
                                    oauth_token: None,
                                    pretty_print: None,
                                    quota_user: None,
                                    upload_protocol: None,
                                    upload_type: None,
                                    xgafv: None,
                                    parent: parent.into(),
                                }
                            }
                            #[doc = "Triggers a user event rejoin operation with latest catalog data. Events will not be annotated with detailed catalog information if catalog item is missing at the time the user event is ingested, and these events are stored as unjoined events with a limited usage on training and serving. This API can be used to trigger a 'join' operation on specified events with latest version of catalog items. It can also be used to correct events joined with wrong catalog items."]
                            pub fn rejoin(
                                &self,
                                request : crate :: schemas :: GoogleCloudRecommendationengineV1Beta1RejoinUserEventsRequest,
                                parent: impl Into<String>,
                            ) -> RejoinRequestBuilder {
                                RejoinRequestBuilder {
                                    reqwest: &self.reqwest,
                                    auth: self.auth_ref(),
                                    request,
                                    access_token: None,
                                    alt: None,
                                    callback: None,
                                    fields: None,
                                    key: None,
                                    oauth_token: None,
                                    pretty_print: None,
                                    quota_user: None,
                                    upload_protocol: None,
                                    upload_type: None,
                                    xgafv: None,
                                    parent: parent.into(),
                                }
                            }
                            #[doc = "Writes a single user event."]
                            pub fn write(
                                &self,
                                request : crate :: schemas :: GoogleCloudRecommendationengineV1Beta1UserEvent,
                                parent: impl Into<String>,
                            ) -> WriteRequestBuilder {
                                WriteRequestBuilder {
                                    reqwest: &self.reqwest,
                                    auth: self.auth_ref(),
                                    request,
                                    access_token: None,
                                    alt: None,
                                    callback: None,
                                    fields: None,
                                    key: None,
                                    oauth_token: None,
                                    pretty_print: None,
                                    quota_user: None,
                                    upload_protocol: None,
                                    upload_type: None,
                                    xgafv: None,
                                    parent: parent.into(),
                                }
                            }
                        }
                        #[doc = "Created via [UserEventsActions::collect()](struct.UserEventsActions.html#method.collect)"]
                        #[derive(Debug, Clone)]
                        pub struct CollectRequestBuilder<'a> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                            parent: String,
                            ets: Option<i64>,
                            uri: Option<String>,
                            user_event: Option<String>,
                            access_token: Option<String>,
                            alt: Option<crate::params::Alt>,
                            callback: Option<String>,
                            fields: Option<String>,
                            key: Option<String>,
                            oauth_token: Option<String>,
                            pretty_print: Option<bool>,
                            quota_user: Option<String>,
                            upload_protocol: Option<String>,
                            upload_type: Option<String>,
                            xgafv: Option<crate::params::Xgafv>,
                        }
                        impl<'a> CollectRequestBuilder<'a> {
                            #[doc = "Optional. The event timestamp in milliseconds. This prevents browser caching of otherwise identical get requests. The name is abbreviated to reduce the payload bytes."]
                            pub fn ets(mut self, value: i64) -> Self {
                                self.ets = Some(value);
                                self
                            }
                            #[doc = "Optional. The url including cgi-parameters but excluding the hash fragment. The URL must be truncated to 1.5K bytes to conservatively be under the 2K bytes. This is often more useful than the referer url, because many browsers only send the domain for 3rd party requests."]
                            pub fn uri(mut self, value: impl Into<String>) -> Self {
                                self.uri = Some(value.into());
                                self
                            }
                            #[doc = "Required. URL encoded UserEvent proto."]
                            pub fn user_event(mut self, value: impl Into<String>) -> Self {
                                self.user_event = Some(value.into());
                                self
                            }
                            #[doc = "OAuth access token."]
                            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                                self.access_token = Some(value.into());
                                self
                            }
                            #[doc = "JSONP"]
                            pub fn callback(mut self, value: impl Into<String>) -> Self {
                                self.callback = Some(value.into());
                                self
                            }
                            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                            pub fn key(mut self, value: impl Into<String>) -> Self {
                                self.key = Some(value.into());
                                self
                            }
                            #[doc = "OAuth 2.0 token for the current user."]
                            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                                self.oauth_token = Some(value.into());
                                self
                            }
                            #[doc = "Returns response with indentations and line breaks."]
                            pub fn pretty_print(mut self, value: bool) -> Self {
                                self.pretty_print = Some(value);
                                self
                            }
                            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                                self.quota_user = Some(value.into());
                                self
                            }
                            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                                self.upload_protocol = Some(value.into());
                                self
                            }
                            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                                self.upload_type = Some(value.into());
                                self
                            }
                            #[doc = "V1 error format."]
                            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                                self.xgafv = Some(value);
                                self
                            }
                            #[doc = r" Execute the given operation. The fields requested are"]
                            #[doc = r" determined by the FieldSelector attribute of the return type."]
                            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                            #[doc = r" are not generic over the return type and deserialize the"]
                            #[doc = r" response into an auto-generated struct will all possible"]
                            #[doc = r" fields."]
                            pub async fn execute<T>(self) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned
                                    + ::google_field_selector::FieldSelector,
                            {
                                let fields = ::google_field_selector::to_string::<T>();
                                let fields: Option<String> = if fields.is_empty() {
                                    None
                                } else {
                                    Some(fields)
                                };
                                self.execute_with_fields(fields).await
                            }
                            #[doc = r" Execute the given operation. This will not provide any"]
                            #[doc = r" `fields` selector indicating that the server will determine"]
                            #[doc = r" the fields returned. This typically includes the most common"]
                            #[doc = r" fields, but it will not include every possible attribute of"]
                            #[doc = r" the response resource."]
                            pub async fn execute_with_default_fields(
                                self,
                            ) -> Result<crate::schemas::GoogleApiHttpBody, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::GoogleApiHttpBody, crate::Error>
                            {
                                self.execute_with_fields(Some("*")).await
                            }
                            #[doc = r" Execute the given operation. This will use the `fields`"]
                            #[doc = r" selector provided and will deserialize the response into"]
                            #[doc = r" whatever return value is provided."]
                            pub async fn execute_with_fields<T, F>(
                                mut self,
                                fields: Option<F>,
                            ) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned,
                                F: Into<String>,
                            {
                                self.fields = fields.map(Into::into);
                                self._execute().await
                            }
                            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned,
                            {
                                let req = self._request(&self._path()).await?;
                                Ok(req.send().await?.error_for_status()?.json().await?)
                            }
                            fn _path(&self) -> String {
                                let mut output =
                                    "https://recommendationengine.googleapis.com/".to_owned();
                                output.push_str("v1beta1/");
                                {
                                    let var_as_str = &self.parent;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::RESERVED,
                                    ));
                                }
                                output.push_str("/userEvents:collect");
                                output
                            }
                            async fn _request(
                                &self,
                                path: &str,
                            ) -> Result<::reqwest::RequestBuilder, crate::Error>
                            {
                                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                                req = req.query(&[("ets", &self.ets)]);
                                req = req.query(&[("uri", &self.uri)]);
                                req = req.query(&[("userEvent", &self.user_event)]);
                                req = req.query(&[("access_token", &self.access_token)]);
                                req = req.query(&[("alt", &self.alt)]);
                                req = req.query(&[("callback", &self.callback)]);
                                req = req.query(&[("fields", &self.fields)]);
                                req = req.query(&[("key", &self.key)]);
                                req = req.query(&[("oauth_token", &self.oauth_token)]);
                                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                req = req.query(&[("quotaUser", &self.quota_user)]);
                                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                                req = req.query(&[("uploadType", &self.upload_type)]);
                                req = req.query(&[("$.xgafv", &self.xgafv)]);
                                let access_token = self
                                    .auth
                                    .access_token()
                                    .await
                                    .map_err(|err| crate::Error::OAuth2(err))?;
                                req = req.bearer_auth(access_token);
                                Ok(req)
                            }
                        }
                        #[doc = "Created via [UserEventsActions::import()](struct.UserEventsActions.html#method.import)"]
                        #[derive(Debug, Clone)]
                        pub struct ImportRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , request : crate :: schemas :: GoogleCloudRecommendationengineV1Beta1ImportUserEventsRequest , parent : String , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
                        impl<'a> ImportRequestBuilder<'a> {
                            #[doc = "OAuth access token."]
                            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                                self.access_token = Some(value.into());
                                self
                            }
                            #[doc = "JSONP"]
                            pub fn callback(mut self, value: impl Into<String>) -> Self {
                                self.callback = Some(value.into());
                                self
                            }
                            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                            pub fn key(mut self, value: impl Into<String>) -> Self {
                                self.key = Some(value.into());
                                self
                            }
                            #[doc = "OAuth 2.0 token for the current user."]
                            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                                self.oauth_token = Some(value.into());
                                self
                            }
                            #[doc = "Returns response with indentations and line breaks."]
                            pub fn pretty_print(mut self, value: bool) -> Self {
                                self.pretty_print = Some(value);
                                self
                            }
                            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                                self.quota_user = Some(value.into());
                                self
                            }
                            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                                self.upload_protocol = Some(value.into());
                                self
                            }
                            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                                self.upload_type = Some(value.into());
                                self
                            }
                            #[doc = "V1 error format."]
                            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                                self.xgafv = Some(value);
                                self
                            }
                            #[doc = r" Execute the given operation. The fields requested are"]
                            #[doc = r" determined by the FieldSelector attribute of the return type."]
                            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                            #[doc = r" are not generic over the return type and deserialize the"]
                            #[doc = r" response into an auto-generated struct will all possible"]
                            #[doc = r" fields."]
                            pub async fn execute<T>(self) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned
                                    + ::google_field_selector::FieldSelector,
                            {
                                let fields = ::google_field_selector::to_string::<T>();
                                let fields: Option<String> = if fields.is_empty() {
                                    None
                                } else {
                                    Some(fields)
                                };
                                self.execute_with_fields(fields).await
                            }
                            #[doc = r" Execute the given operation. This will not provide any"]
                            #[doc = r" `fields` selector indicating that the server will determine"]
                            #[doc = r" the fields returned. This typically includes the most common"]
                            #[doc = r" fields, but it will not include every possible attribute of"]
                            #[doc = r" the response resource."]
                            pub async fn execute_with_default_fields(
                                self,
                            ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                            {
                                self.execute_with_fields(Some("*")).await
                            }
                            #[doc = r" Execute the given operation. This will use the `fields`"]
                            #[doc = r" selector provided and will deserialize the response into"]
                            #[doc = r" whatever return value is provided."]
                            pub async fn execute_with_fields<T, F>(
                                mut self,
                                fields: Option<F>,
                            ) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned,
                                F: Into<String>,
                            {
                                self.fields = fields.map(Into::into);
                                self._execute().await
                            }
                            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned,
                            {
                                let req = self._request(&self._path()).await?;
                                let req = req.json(&self.request);
                                Ok(req.send().await?.error_for_status()?.json().await?)
                            }
                            fn _path(&self) -> String {
                                let mut output =
                                    "https://recommendationengine.googleapis.com/".to_owned();
                                output.push_str("v1beta1/");
                                {
                                    let var_as_str = &self.parent;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::RESERVED,
                                    ));
                                }
                                output.push_str("/userEvents:import");
                                output
                            }
                            async fn _request(
                                &self,
                                path: &str,
                            ) -> Result<::reqwest::RequestBuilder, crate::Error>
                            {
                                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                                req = req.query(&[("access_token", &self.access_token)]);
                                req = req.query(&[("alt", &self.alt)]);
                                req = req.query(&[("callback", &self.callback)]);
                                req = req.query(&[("fields", &self.fields)]);
                                req = req.query(&[("key", &self.key)]);
                                req = req.query(&[("oauth_token", &self.oauth_token)]);
                                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                req = req.query(&[("quotaUser", &self.quota_user)]);
                                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                                req = req.query(&[("uploadType", &self.upload_type)]);
                                req = req.query(&[("$.xgafv", &self.xgafv)]);
                                let access_token = self
                                    .auth
                                    .access_token()
                                    .await
                                    .map_err(|err| crate::Error::OAuth2(err))?;
                                req = req.bearer_auth(access_token);
                                Ok(req)
                            }
                        }
                        #[doc = "Created via [UserEventsActions::list()](struct.UserEventsActions.html#method.list)"]
                        #[derive(Debug, Clone)]
                        pub struct ListRequestBuilder<'a> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                            parent: String,
                            filter: Option<String>,
                            page_size: Option<i32>,
                            page_token: Option<String>,
                            access_token: Option<String>,
                            alt: Option<crate::params::Alt>,
                            callback: Option<String>,
                            fields: Option<String>,
                            key: Option<String>,
                            oauth_token: Option<String>,
                            pretty_print: Option<bool>,
                            quota_user: Option<String>,
                            upload_protocol: Option<String>,
                            upload_type: Option<String>,
                            xgafv: Option<crate::params::Xgafv>,
                        }
                        impl<'a> ListRequestBuilder<'a> {
                            #[doc = "Optional. Filtering expression to specify restrictions over returned events. This is a sequence of terms, where each term applies some kind of a restriction to the returned user events. Use this expression to restrict results to a specific time range, or filter events by eventType. eg: eventTime > \"2012-04-23T18:25:43.511Z\" eventsMissingCatalogItems eventTime<\"2012-04-23T18:25:43.511Z\" eventType=search We expect only 3 types of fields: * eventTime: this can be specified a maximum of 2 times, once with a less than operator and once with a greater than operator. The eventTime restrict should result in one contiguous valid eventTime range. * eventType: only 1 eventType restriction can be specified. * eventsMissingCatalogItems: specififying this will restrict results to events for which catalog items were not found in the catalog. The default behavior is to return only those events for which catalog items were found. Some examples of valid filters expressions: * Example 1: eventTime > \"2012-04-23T18:25:43.511Z\" eventTime < \"2012-04-23T18:30:43.511Z\" * Example 2: eventTime > \"2012-04-23T18:25:43.511Z\" eventType = detail-page-view * Example 3: eventsMissingCatalogItems eventType = search eventTime < \"2018-04-23T18:30:43.511Z\" * Example 4: eventTime > \"2012-04-23T18:25:43.511Z\" * Example 5: eventType = search * Example 6: eventsMissingCatalogItems"]
                            pub fn filter(mut self, value: impl Into<String>) -> Self {
                                self.filter = Some(value.into());
                                self
                            }
                            #[doc = "Optional. Maximum number of results to return per page. If zero, the service will choose a reasonable default."]
                            pub fn page_size(mut self, value: i32) -> Self {
                                self.page_size = Some(value);
                                self
                            }
                            #[doc = "Optional. The previous ListUserEventsResponse.next_page_token."]
                            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                                self.page_token = Some(value.into());
                                self
                            }
                            #[doc = "OAuth access token."]
                            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                                self.access_token = Some(value.into());
                                self
                            }
                            #[doc = "JSONP"]
                            pub fn callback(mut self, value: impl Into<String>) -> Self {
                                self.callback = Some(value.into());
                                self
                            }
                            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                            pub fn key(mut self, value: impl Into<String>) -> Self {
                                self.key = Some(value.into());
                                self
                            }
                            #[doc = "OAuth 2.0 token for the current user."]
                            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                                self.oauth_token = Some(value.into());
                                self
                            }
                            #[doc = "Returns response with indentations and line breaks."]
                            pub fn pretty_print(mut self, value: bool) -> Self {
                                self.pretty_print = Some(value);
                                self
                            }
                            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                                self.quota_user = Some(value.into());
                                self
                            }
                            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                                self.upload_protocol = Some(value.into());
                                self
                            }
                            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                                self.upload_type = Some(value.into());
                                self
                            }
                            #[doc = "V1 error format."]
                            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                                self.xgafv = Some(value);
                                self
                            }
                            #[doc = r" Execute the given operation. The fields requested are"]
                            #[doc = r" determined by the FieldSelector attribute of the return type."]
                            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                            #[doc = r" are not generic over the return type and deserialize the"]
                            #[doc = r" response into an auto-generated struct will all possible"]
                            #[doc = r" fields."]
                            pub async fn execute<T>(self) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned
                                    + ::google_field_selector::FieldSelector,
                            {
                                let fields = ::google_field_selector::to_string::<T>();
                                let fields: Option<String> = if fields.is_empty() {
                                    None
                                } else {
                                    Some(fields)
                                };
                                self.execute_with_fields(fields).await
                            }
                            #[doc = r" Execute the given operation. This will not provide any"]
                            #[doc = r" `fields` selector indicating that the server will determine"]
                            #[doc = r" the fields returned. This typically includes the most common"]
                            #[doc = r" fields, but it will not include every possible attribute of"]
                            #[doc = r" the response resource."]                            pub async fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleCloudRecommendationengineV1Beta1ListUserEventsResponse , crate :: Error >{
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]                            pub async fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleCloudRecommendationengineV1Beta1ListUserEventsResponse , crate :: Error >{
                                self.execute_with_fields(Some("*")).await
                            }
                            #[doc = r" Execute the given operation. This will use the `fields`"]
                            #[doc = r" selector provided and will deserialize the response into"]
                            #[doc = r" whatever return value is provided."]
                            pub async fn execute_with_fields<T, F>(
                                mut self,
                                fields: Option<F>,
                            ) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned,
                                F: Into<String>,
                            {
                                self.fields = fields.map(Into::into);
                                self._execute().await
                            }
                            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned,
                            {
                                let req = self._request(&self._path()).await?;
                                Ok(req.send().await?.error_for_status()?.json().await?)
                            }
                            fn _path(&self) -> String {
                                let mut output =
                                    "https://recommendationengine.googleapis.com/".to_owned();
                                output.push_str("v1beta1/");
                                {
                                    let var_as_str = &self.parent;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::RESERVED,
                                    ));
                                }
                                output.push_str("/userEvents");
                                output
                            }
                            async fn _request(
                                &self,
                                path: &str,
                            ) -> Result<::reqwest::RequestBuilder, crate::Error>
                            {
                                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                                req = req.query(&[("filter", &self.filter)]);
                                req = req.query(&[("pageSize", &self.page_size)]);
                                req = req.query(&[("pageToken", &self.page_token)]);
                                req = req.query(&[("access_token", &self.access_token)]);
                                req = req.query(&[("alt", &self.alt)]);
                                req = req.query(&[("callback", &self.callback)]);
                                req = req.query(&[("fields", &self.fields)]);
                                req = req.query(&[("key", &self.key)]);
                                req = req.query(&[("oauth_token", &self.oauth_token)]);
                                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                req = req.query(&[("quotaUser", &self.quota_user)]);
                                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                                req = req.query(&[("uploadType", &self.upload_type)]);
                                req = req.query(&[("$.xgafv", &self.xgafv)]);
                                let access_token = self
                                    .auth
                                    .access_token()
                                    .await
                                    .map_err(|err| crate::Error::OAuth2(err))?;
                                req = req.bearer_auth(access_token);
                                Ok(req)
                            }
                        }
                        #[doc = "Created via [UserEventsActions::purge()](struct.UserEventsActions.html#method.purge)"]
                        #[derive(Debug, Clone)]
                        pub struct PurgeRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , request : crate :: schemas :: GoogleCloudRecommendationengineV1Beta1PurgeUserEventsRequest , parent : String , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
                        impl<'a> PurgeRequestBuilder<'a> {
                            #[doc = "OAuth access token."]
                            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                                self.access_token = Some(value.into());
                                self
                            }
                            #[doc = "JSONP"]
                            pub fn callback(mut self, value: impl Into<String>) -> Self {
                                self.callback = Some(value.into());
                                self
                            }
                            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                            pub fn key(mut self, value: impl Into<String>) -> Self {
                                self.key = Some(value.into());
                                self
                            }
                            #[doc = "OAuth 2.0 token for the current user."]
                            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                                self.oauth_token = Some(value.into());
                                self
                            }
                            #[doc = "Returns response with indentations and line breaks."]
                            pub fn pretty_print(mut self, value: bool) -> Self {
                                self.pretty_print = Some(value);
                                self
                            }
                            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                                self.quota_user = Some(value.into());
                                self
                            }
                            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                                self.upload_protocol = Some(value.into());
                                self
                            }
                            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                                self.upload_type = Some(value.into());
                                self
                            }
                            #[doc = "V1 error format."]
                            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                                self.xgafv = Some(value);
                                self
                            }
                            #[doc = r" Execute the given operation. The fields requested are"]
                            #[doc = r" determined by the FieldSelector attribute of the return type."]
                            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                            #[doc = r" are not generic over the return type and deserialize the"]
                            #[doc = r" response into an auto-generated struct will all possible"]
                            #[doc = r" fields."]
                            pub async fn execute<T>(self) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned
                                    + ::google_field_selector::FieldSelector,
                            {
                                let fields = ::google_field_selector::to_string::<T>();
                                let fields: Option<String> = if fields.is_empty() {
                                    None
                                } else {
                                    Some(fields)
                                };
                                self.execute_with_fields(fields).await
                            }
                            #[doc = r" Execute the given operation. This will not provide any"]
                            #[doc = r" `fields` selector indicating that the server will determine"]
                            #[doc = r" the fields returned. This typically includes the most common"]
                            #[doc = r" fields, but it will not include every possible attribute of"]
                            #[doc = r" the response resource."]
                            pub async fn execute_with_default_fields(
                                self,
                            ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                            {
                                self.execute_with_fields(Some("*")).await
                            }
                            #[doc = r" Execute the given operation. This will use the `fields`"]
                            #[doc = r" selector provided and will deserialize the response into"]
                            #[doc = r" whatever return value is provided."]
                            pub async fn execute_with_fields<T, F>(
                                mut self,
                                fields: Option<F>,
                            ) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned,
                                F: Into<String>,
                            {
                                self.fields = fields.map(Into::into);
                                self._execute().await
                            }
                            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned,
                            {
                                let req = self._request(&self._path()).await?;
                                let req = req.json(&self.request);
                                Ok(req.send().await?.error_for_status()?.json().await?)
                            }
                            fn _path(&self) -> String {
                                let mut output =
                                    "https://recommendationengine.googleapis.com/".to_owned();
                                output.push_str("v1beta1/");
                                {
                                    let var_as_str = &self.parent;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::RESERVED,
                                    ));
                                }
                                output.push_str("/userEvents:purge");
                                output
                            }
                            async fn _request(
                                &self,
                                path: &str,
                            ) -> Result<::reqwest::RequestBuilder, crate::Error>
                            {
                                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                                req = req.query(&[("access_token", &self.access_token)]);
                                req = req.query(&[("alt", &self.alt)]);
                                req = req.query(&[("callback", &self.callback)]);
                                req = req.query(&[("fields", &self.fields)]);
                                req = req.query(&[("key", &self.key)]);
                                req = req.query(&[("oauth_token", &self.oauth_token)]);
                                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                req = req.query(&[("quotaUser", &self.quota_user)]);
                                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                                req = req.query(&[("uploadType", &self.upload_type)]);
                                req = req.query(&[("$.xgafv", &self.xgafv)]);
                                let access_token = self
                                    .auth
                                    .access_token()
                                    .await
                                    .map_err(|err| crate::Error::OAuth2(err))?;
                                req = req.bearer_auth(access_token);
                                Ok(req)
                            }
                        }
                        #[doc = "Created via [UserEventsActions::rejoin()](struct.UserEventsActions.html#method.rejoin)"]
                        #[derive(Debug, Clone)]
                        pub struct RejoinRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , request : crate :: schemas :: GoogleCloudRecommendationengineV1Beta1RejoinUserEventsRequest , parent : String , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
                        impl<'a> RejoinRequestBuilder<'a> {
                            #[doc = "OAuth access token."]
                            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                                self.access_token = Some(value.into());
                                self
                            }
                            #[doc = "JSONP"]
                            pub fn callback(mut self, value: impl Into<String>) -> Self {
                                self.callback = Some(value.into());
                                self
                            }
                            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                            pub fn key(mut self, value: impl Into<String>) -> Self {
                                self.key = Some(value.into());
                                self
                            }
                            #[doc = "OAuth 2.0 token for the current user."]
                            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                                self.oauth_token = Some(value.into());
                                self
                            }
                            #[doc = "Returns response with indentations and line breaks."]
                            pub fn pretty_print(mut self, value: bool) -> Self {
                                self.pretty_print = Some(value);
                                self
                            }
                            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                                self.quota_user = Some(value.into());
                                self
                            }
                            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                                self.upload_protocol = Some(value.into());
                                self
                            }
                            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                                self.upload_type = Some(value.into());
                                self
                            }
                            #[doc = "V1 error format."]
                            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                                self.xgafv = Some(value);
                                self
                            }
                            #[doc = r" Execute the given operation. The fields requested are"]
                            #[doc = r" determined by the FieldSelector attribute of the return type."]
                            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                            #[doc = r" are not generic over the return type and deserialize the"]
                            #[doc = r" response into an auto-generated struct will all possible"]
                            #[doc = r" fields."]
                            pub async fn execute<T>(self) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned
                                    + ::google_field_selector::FieldSelector,
                            {
                                let fields = ::google_field_selector::to_string::<T>();
                                let fields: Option<String> = if fields.is_empty() {
                                    None
                                } else {
                                    Some(fields)
                                };
                                self.execute_with_fields(fields).await
                            }
                            #[doc = r" Execute the given operation. This will not provide any"]
                            #[doc = r" `fields` selector indicating that the server will determine"]
                            #[doc = r" the fields returned. This typically includes the most common"]
                            #[doc = r" fields, but it will not include every possible attribute of"]
                            #[doc = r" the response resource."]
                            pub async fn execute_with_default_fields(
                                self,
                            ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                            {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                            {
                                self.execute_with_fields(Some("*")).await
                            }
                            #[doc = r" Execute the given operation. This will use the `fields`"]
                            #[doc = r" selector provided and will deserialize the response into"]
                            #[doc = r" whatever return value is provided."]
                            pub async fn execute_with_fields<T, F>(
                                mut self,
                                fields: Option<F>,
                            ) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned,
                                F: Into<String>,
                            {
                                self.fields = fields.map(Into::into);
                                self._execute().await
                            }
                            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned,
                            {
                                let req = self._request(&self._path()).await?;
                                let req = req.json(&self.request);
                                Ok(req.send().await?.error_for_status()?.json().await?)
                            }
                            fn _path(&self) -> String {
                                let mut output =
                                    "https://recommendationengine.googleapis.com/".to_owned();
                                output.push_str("v1beta1/");
                                {
                                    let var_as_str = &self.parent;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::RESERVED,
                                    ));
                                }
                                output.push_str("/userEvents:rejoin");
                                output
                            }
                            async fn _request(
                                &self,
                                path: &str,
                            ) -> Result<::reqwest::RequestBuilder, crate::Error>
                            {
                                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                                req = req.query(&[("access_token", &self.access_token)]);
                                req = req.query(&[("alt", &self.alt)]);
                                req = req.query(&[("callback", &self.callback)]);
                                req = req.query(&[("fields", &self.fields)]);
                                req = req.query(&[("key", &self.key)]);
                                req = req.query(&[("oauth_token", &self.oauth_token)]);
                                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                req = req.query(&[("quotaUser", &self.quota_user)]);
                                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                                req = req.query(&[("uploadType", &self.upload_type)]);
                                req = req.query(&[("$.xgafv", &self.xgafv)]);
                                let access_token = self
                                    .auth
                                    .access_token()
                                    .await
                                    .map_err(|err| crate::Error::OAuth2(err))?;
                                req = req.bearer_auth(access_token);
                                Ok(req)
                            }
                        }
                        #[doc = "Created via [UserEventsActions::write()](struct.UserEventsActions.html#method.write)"]
                        #[derive(Debug, Clone)]
                        pub struct WriteRequestBuilder<'a> {
                            pub(crate) reqwest: &'a ::reqwest::Client,
                            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                            request:
                                crate::schemas::GoogleCloudRecommendationengineV1Beta1UserEvent,
                            parent: String,
                            access_token: Option<String>,
                            alt: Option<crate::params::Alt>,
                            callback: Option<String>,
                            fields: Option<String>,
                            key: Option<String>,
                            oauth_token: Option<String>,
                            pretty_print: Option<bool>,
                            quota_user: Option<String>,
                            upload_protocol: Option<String>,
                            upload_type: Option<String>,
                            xgafv: Option<crate::params::Xgafv>,
                        }
                        impl<'a> WriteRequestBuilder<'a> {
                            #[doc = "OAuth access token."]
                            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                                self.access_token = Some(value.into());
                                self
                            }
                            #[doc = "JSONP"]
                            pub fn callback(mut self, value: impl Into<String>) -> Self {
                                self.callback = Some(value.into());
                                self
                            }
                            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                            pub fn key(mut self, value: impl Into<String>) -> Self {
                                self.key = Some(value.into());
                                self
                            }
                            #[doc = "OAuth 2.0 token for the current user."]
                            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                                self.oauth_token = Some(value.into());
                                self
                            }
                            #[doc = "Returns response with indentations and line breaks."]
                            pub fn pretty_print(mut self, value: bool) -> Self {
                                self.pretty_print = Some(value);
                                self
                            }
                            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                                self.quota_user = Some(value.into());
                                self
                            }
                            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                                self.upload_protocol = Some(value.into());
                                self
                            }
                            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                                self.upload_type = Some(value.into());
                                self
                            }
                            #[doc = "V1 error format."]
                            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                                self.xgafv = Some(value);
                                self
                            }
                            #[doc = r" Execute the given operation. The fields requested are"]
                            #[doc = r" determined by the FieldSelector attribute of the return type."]
                            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                            #[doc = r" are not generic over the return type and deserialize the"]
                            #[doc = r" response into an auto-generated struct will all possible"]
                            #[doc = r" fields."]
                            pub async fn execute<T>(self) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned
                                    + ::google_field_selector::FieldSelector,
                            {
                                let fields = ::google_field_selector::to_string::<T>();
                                let fields: Option<String> = if fields.is_empty() {
                                    None
                                } else {
                                    Some(fields)
                                };
                                self.execute_with_fields(fields).await
                            }
                            #[doc = r" Execute the given operation. This will not provide any"]
                            #[doc = r" `fields` selector indicating that the server will determine"]
                            #[doc = r" the fields returned. This typically includes the most common"]
                            #[doc = r" fields, but it will not include every possible attribute of"]
                            #[doc = r" the response resource."]
                            pub async fn execute_with_default_fields(
                                self,
                            ) -> Result<
                                crate::schemas::GoogleCloudRecommendationengineV1Beta1UserEvent,
                                crate::Error,
                            > {
                                self.execute_with_fields(None::<&str>).await
                            }
                            #[doc = r" Execute the given operation. This will provide a `fields`"]
                            #[doc = r" selector of `*`. This will include every attribute of the"]
                            #[doc = r" response resource and should be limited to use during"]
                            #[doc = r" development or debugging."]
                            pub async fn execute_with_all_fields(
                                self,
                            ) -> Result<
                                crate::schemas::GoogleCloudRecommendationengineV1Beta1UserEvent,
                                crate::Error,
                            > {
                                self.execute_with_fields(Some("*")).await
                            }
                            #[doc = r" Execute the given operation. This will use the `fields`"]
                            #[doc = r" selector provided and will deserialize the response into"]
                            #[doc = r" whatever return value is provided."]
                            pub async fn execute_with_fields<T, F>(
                                mut self,
                                fields: Option<F>,
                            ) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned,
                                F: Into<String>,
                            {
                                self.fields = fields.map(Into::into);
                                self._execute().await
                            }
                            async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                            where
                                T: ::serde::de::DeserializeOwned,
                            {
                                let req = self._request(&self._path()).await?;
                                let req = req.json(&self.request);
                                Ok(req.send().await?.error_for_status()?.json().await?)
                            }
                            fn _path(&self) -> String {
                                let mut output =
                                    "https://recommendationengine.googleapis.com/".to_owned();
                                output.push_str("v1beta1/");
                                {
                                    let var_as_str = &self.parent;
                                    output.extend(::percent_encoding::utf8_percent_encode(
                                        &var_as_str,
                                        crate::RESERVED,
                                    ));
                                }
                                output.push_str("/userEvents:write");
                                output
                            }
                            async fn _request(
                                &self,
                                path: &str,
                            ) -> Result<::reqwest::RequestBuilder, crate::Error>
                            {
                                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                                req = req.query(&[("access_token", &self.access_token)]);
                                req = req.query(&[("alt", &self.alt)]);
                                req = req.query(&[("callback", &self.callback)]);
                                req = req.query(&[("fields", &self.fields)]);
                                req = req.query(&[("key", &self.key)]);
                                req = req.query(&[("oauth_token", &self.oauth_token)]);
                                req = req.query(&[("prettyPrint", &self.pretty_print)]);
                                req = req.query(&[("quotaUser", &self.quota_user)]);
                                req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                                req = req.query(&[("uploadType", &self.upload_type)]);
                                req = req.query(&[("$.xgafv", &self.xgafv)]);
                                let access_token = self
                                    .auth
                                    .access_token()
                                    .await
                                    .map_err(|err| crate::Error::OAuth2(err))?;
                                req = req.bearer_auth(access_token);
                                Ok(req)
                            }
                        }
                    }
                }
                pub mod operations {
                    pub mod params {}
                    pub struct OperationsActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> OperationsActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service."]
                        pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder {
                            GetRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                            }
                        }
                        #[doc = "Lists operations that match the specified filter in the request. If the server doesn't support this method, it returns `UNIMPLEMENTED`. NOTE: the `name` binding allows API services to override the binding to use different resource name schemes, such as `users/*/operations`. To override the binding, API services can add a binding such as `\"/v1/{name=users/*}/operations\"` to their service configuration. For backwards compatibility, the default name includes the operations collection id, however overriding users must ensure the name binding is the parent resource, without the operations collection id."]
                        pub fn list(&self, name: impl Into<String>) -> ListRequestBuilder {
                            ListRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: self.auth_ref(),
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                name: name.into(),
                                filter: None,
                                page_size: None,
                                page_token: None,
                            }
                        }
                    }
                    #[doc = "Created via [OperationsActions::get()](struct.OperationsActions.html#method.get)"]
                    #[derive(Debug, Clone)]
                    pub struct GetRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        name: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> GetRequestBuilder<'a> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub async fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields).await
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub async fn execute_with_default_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::GoogleLongrunningOperation, crate::Error>
                        {
                            self.execute_with_fields(Some("*")).await
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub async fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute().await
                        }
                        async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path()).await?;
                            Ok(req.send().await?.error_for_status()?.json().await?)
                        }
                        fn _path(&self) -> String {
                            let mut output =
                                "https://recommendationengine.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                            req = req.query(&[("access_token", &self.access_token)]);
                            req = req.query(&[("alt", &self.alt)]);
                            req = req.query(&[("callback", &self.callback)]);
                            req = req.query(&[("fields", &self.fields)]);
                            req = req.query(&[("key", &self.key)]);
                            req = req.query(&[("oauth_token", &self.oauth_token)]);
                            req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            req = req.query(&[("quotaUser", &self.quota_user)]);
                            req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            req = req.query(&[("uploadType", &self.upload_type)]);
                            req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let access_token = self
                                .auth
                                .access_token()
                                .await
                                .map_err(|err| crate::Error::OAuth2(err))?;
                            req = req.bearer_auth(access_token);
                            Ok(req)
                        }
                    }
                    #[doc = "Created via [OperationsActions::list()](struct.OperationsActions.html#method.list)"]
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        name: String,
                        filter: Option<String>,
                        page_size: Option<i32>,
                        page_token: Option<String>,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a> ListRequestBuilder<'a> {
                        #[doc = "The standard list filter."]
                        pub fn filter(mut self, value: impl Into<String>) -> Self {
                            self.filter = Some(value.into());
                            self
                        }
                        #[doc = "The standard list page size."]
                        pub fn page_size(mut self, value: i32) -> Self {
                            self.page_size = Some(value);
                            self
                        }
                        #[doc = "The standard list page token."]
                        pub fn page_token(mut self, value: impl Into<String>) -> Self {
                            self.page_token = Some(value.into());
                            self
                        }
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub async fn execute<T>(self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned
                                + ::google_field_selector::FieldSelector,
                        {
                            let fields = ::google_field_selector::to_string::<T>();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_with_fields(fields).await
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub async fn execute_with_default_fields(
                            self,
                        ) -> Result<
                            crate::schemas::GoogleLongrunningListOperationsResponse,
                            crate::Error,
                        > {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<
                            crate::schemas::GoogleLongrunningListOperationsResponse,
                            crate::Error,
                        > {
                            self.execute_with_fields(Some("*")).await
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub async fn execute_with_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute().await
                        }
                        async fn _execute<T>(&mut self) -> Result<T, crate::Error>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path()).await?;
                            Ok(req.send().await?.error_for_status()?.json().await?)
                        }
                        fn _path(&self) -> String {
                            let mut output =
                                "https://recommendationengine.googleapis.com/".to_owned();
                            output.push_str("v1beta1/");
                            {
                                let var_as_str = &self.name;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/operations");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                            req = req.query(&[("filter", &self.filter)]);
                            req = req.query(&[("pageSize", &self.page_size)]);
                            req = req.query(&[("pageToken", &self.page_token)]);
                            req = req.query(&[("access_token", &self.access_token)]);
                            req = req.query(&[("alt", &self.alt)]);
                            req = req.query(&[("callback", &self.callback)]);
                            req = req.query(&[("fields", &self.fields)]);
                            req = req.query(&[("key", &self.key)]);
                            req = req.query(&[("oauth_token", &self.oauth_token)]);
                            req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            req = req.query(&[("quotaUser", &self.quota_user)]);
                            req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            req = req.query(&[("uploadType", &self.upload_type)]);
                            req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let access_token = self
                                .auth
                                .access_token()
                                .await
                                .map_err(|err| crate::Error::OAuth2(err))?;
                            req = req.bearer_auth(access_token);
                            Ok(req)
                        }
                    }
                }
            }
        }
    }
}
#[derive(Debug)]
pub enum Error {
    OAuth2(Box<dyn ::std::error::Error + Send + Sync>),
    JSON(::serde_json::Error),
    Reqwest {
        reqwest_err: ::reqwest::Error,
        body: Option<String>,
    },
    IO(std::io::Error),
    Other(Box<dyn ::std::error::Error + Send + Sync>),
}

impl Error {
    pub fn json_error(&self) -> Option<&::serde_json::Error> {
        match self {
            Error::OAuth2(_) => None,
            Error::JSON(err) => Some(err),
            Error::Reqwest { .. } => None,
            Error::IO(_) => None,
            Error::Other(_) => None,
        }
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Error::OAuth2(err) => write!(f, "OAuth2 Error: {}", err),
            Error::JSON(err) => write!(f, "JSON Error: {}", err),
            Error::Reqwest { reqwest_err, body } => {
                write!(f, "Reqwest Error: {}", reqwest_err)?;
                if let Some(body) = body {
                    write!(f, ": {}", body)?;
                }
                Ok(())
            }
            Error::IO(err) => write!(f, "IO Error: {}", err),
            Error::Other(err) => write!(f, "Uknown Error: {}", err),
        }
    }
}

impl ::std::error::Error for Error {}

impl From<::serde_json::Error> for Error {
    fn from(err: ::serde_json::Error) -> Error {
        Error::JSON(err)
    }
}

impl From<::reqwest::Error> for Error {
    fn from(reqwest_err: ::reqwest::Error) -> Error {
        Error::Reqwest {
            reqwest_err,
            body: None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO(err)
    }
}
#[allow(dead_code)]
const SIMPLE: &::percent_encoding::AsciiSet = &::percent_encoding::NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

#[allow(dead_code)]
const RESERVED: &::percent_encoding::AsciiSet = &SIMPLE
    .remove(b'%')
    .remove(b':')
    .remove(b'/')
    .remove(b'?')
    .remove(b'#')
    .remove(b'[')
    .remove(b']')
    .remove(b'@')
    .remove(b'!')
    .remove(b'$')
    .remove(b'&')
    .remove(b'\'')
    .remove(b'(')
    .remove(b')')
    .remove(b'*')
    .remove(b'+')
    .remove(b',')
    .remove(b';')
    .remove(b'=');
#[allow(dead_code)]
mod multipart {
    pub(crate) struct RelatedMultiPart {
        parts: Vec<Part>,
        boundary: String,
    }

    impl RelatedMultiPart {
        pub(crate) fn new() -> Self {
            RelatedMultiPart {
                parts: Vec::new(),
                boundary: ::textnonce::TextNonce::sized(68).unwrap().0,
            }
        }

        pub(crate) fn new_part(&mut self, part: Part) {
            self.parts.push(part);
        }

        pub(crate) fn boundary(&self) -> &str {
            &self.boundary
        }

        pub(crate) fn into_reader(self) -> RelatedMultiPartReader {
            let boundary_marker = boundary_marker(&self.boundary);
            RelatedMultiPartReader {
                state: RelatedMultiPartReaderState::WriteBoundary {
                    start: 0,
                    boundary: format!("{}\r\n", &boundary_marker),
                },
                boundary: boundary_marker,
                next_body: None,
                parts: self.parts.into_iter(),
            }
        }
    }

    pub(crate) struct Part {
        content_type: ::mime::Mime,
        body: Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>,
    }

    impl Part {
        pub(crate) fn new(
            content_type: ::mime::Mime,
            body: Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>,
        ) -> Part {
            Part { content_type, body }
        }
    }

    pub(crate) struct RelatedMultiPartReader {
        state: RelatedMultiPartReaderState,
        boundary: String,
        next_body: Option<Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>>,
        parts: std::vec::IntoIter<Part>,
    }

    enum RelatedMultiPartReaderState {
        WriteBoundary {
            start: usize,
            boundary: String,
        },
        WriteContentType {
            start: usize,
            content_type: Vec<u8>,
        },
        WriteBody {
            body: Box<dyn futures::io::AsyncRead + std::marker::Unpin + Send>,
        },
    }

    impl futures::io::AsyncRead for RelatedMultiPartReader {
        fn poll_read(
            mut self: std::pin::Pin<&mut Self>,
            ctx: &mut futures::task::Context,
            buf: &mut [u8],
        ) -> futures::task::Poll<Result<usize, futures::io::Error>> {
            use RelatedMultiPartReaderState::*;

            let mut bytes_written: usize = 0;
            loop {
                let rem_buf = &mut buf[bytes_written..];
                match &mut self.state {
                    WriteBoundary { start, boundary } => {
                        let bytes_to_copy = std::cmp::min(boundary.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&boundary.as_bytes()[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == boundary.len() {
                            let next_part = match self.parts.next() {
                                None => break,
                                Some(part) => part,
                            };
                            self.next_body = Some(next_part.body);
                            self.state = WriteContentType {
                                start: 0,
                                content_type: format!(
                                    "Content-Type: {}\r\n\r\n",
                                    next_part.content_type
                                )
                                .into_bytes(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteContentType {
                        start,
                        content_type,
                    } => {
                        let bytes_to_copy =
                            std::cmp::min(content_type.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&content_type[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == content_type.len() {
                            self.state = WriteBody {
                                body: self.next_body.take().unwrap(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteBody { body } => {
                        let body = std::pin::Pin::new(body);
                        let written = match futures::io::AsyncRead::poll_read(body, ctx, rem_buf) {
                            futures::task::Poll::Ready(Ok(n)) => n,
                            futures::task::Poll::Ready(Err(err)) => {
                                return futures::task::Poll::Ready(Err(err));
                            }
                            futures::task::Poll::Pending => return futures::task::Poll::Pending,
                        };
                        bytes_written += written;
                        if written == 0 {
                            self.state = WriteBoundary {
                                start: 0,
                                boundary: format!("\r\n{}\r\n", &self.boundary),
                            };
                        } else {
                            break;
                        }
                    }
                }
            }

            futures::task::Poll::Ready(Ok(bytes_written))
        }
    }

    fn boundary_marker(boundary: &str) -> String {
        let mut marker = String::with_capacity(boundary.len() + 2);
        marker.push_str("--");
        marker.push_str(boundary);
        marker
    }
}
// A serde helper module that can be used with the `with` attribute
// to deserialize any string to a FromStr type and serialize any
// Display type to a String. Google API's encode i64, u64 values as
// strings.
#[allow(dead_code)]
mod parsed_string {
    pub fn serialize<T, S>(
        value: &Option<T>,
        serializer: S,
    ) -> ::std::result::Result<S::Ok, S::Error>
    where
        T: ::std::fmt::Display,
        S: ::serde::Serializer,
    {
        use ::serde::Serialize;
        value.as_ref().map(|x| x.to_string()).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> ::std::result::Result<Option<T>, D::Error>
    where
        T: ::std::str::FromStr,
        T::Err: ::std::fmt::Display,
        D: ::serde::de::Deserializer<'de>,
    {
        use ::serde::Deserialize;
        match Option::<String>::deserialize(deserializer)? {
            Some(x) => Ok(Some(x.parse().map_err(::serde::de::Error::custom)?)),
            None => Ok(None),
        }
    }
}
