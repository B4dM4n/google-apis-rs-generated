#![doc = "# Resources and Methods\n    * [debug](resources/debug/struct.DebugActions.html)\n      * [datasources](resources/debug/datasources/struct.DatasourcesActions.html)\n        * [items](resources/debug/datasources/items/struct.ItemsActions.html)\n          * [*checkAccess*](resources/debug/datasources/items/struct.CheckAccessRequestBuilder.html), [*searchByViewUrl*](resources/debug/datasources/items/struct.SearchByViewUrlRequestBuilder.html)\n          * [unmappedids](resources/debug/datasources/items/unmappedids/struct.UnmappedidsActions.html)\n            * [*list*](resources/debug/datasources/items/unmappedids/struct.ListRequestBuilder.html)\n      * [identitysources](resources/debug/identitysources/struct.IdentitysourcesActions.html)\n        * [items](resources/debug/identitysources/items/struct.ItemsActions.html)\n          * [*listForunmappedidentity*](resources/debug/identitysources/items/struct.ListForunmappedidentityRequestBuilder.html)\n        * [unmappedids](resources/debug/identitysources/unmappedids/struct.UnmappedidsActions.html)\n          * [*list*](resources/debug/identitysources/unmappedids/struct.ListRequestBuilder.html)\n    * [indexing](resources/indexing/struct.IndexingActions.html)\n      * [datasources](resources/indexing/datasources/struct.DatasourcesActions.html)\n        * [*deleteSchema*](resources/indexing/datasources/struct.DeleteSchemaRequestBuilder.html), [*getSchema*](resources/indexing/datasources/struct.GetSchemaRequestBuilder.html), [*updateSchema*](resources/indexing/datasources/struct.UpdateSchemaRequestBuilder.html)\n        * [items](resources/indexing/datasources/items/struct.ItemsActions.html)\n          * [*delete*](resources/indexing/datasources/items/struct.DeleteRequestBuilder.html), [*deleteQueueItems*](resources/indexing/datasources/items/struct.DeleteQueueItemsRequestBuilder.html), [*get*](resources/indexing/datasources/items/struct.GetRequestBuilder.html), [*index*](resources/indexing/datasources/items/struct.IndexRequestBuilder.html), [*list*](resources/indexing/datasources/items/struct.ListRequestBuilder.html), [*poll*](resources/indexing/datasources/items/struct.PollRequestBuilder.html), [*push*](resources/indexing/datasources/items/struct.PushRequestBuilder.html), [*unreserve*](resources/indexing/datasources/items/struct.UnreserveRequestBuilder.html), [*upload*](resources/indexing/datasources/items/struct.UploadRequestBuilder.html)\n    * [media](resources/media/struct.MediaActions.html)\n      * [*upload*](resources/media/struct.UploadRequestBuilder.html)\n    * [operations](resources/operations/struct.OperationsActions.html)\n      * [*get*](resources/operations/struct.GetRequestBuilder.html)\n      * [lro](resources/operations/lro/struct.LroActions.html)\n        * [*list*](resources/operations/lro/struct.ListRequestBuilder.html)\n    * [query](resources/query/struct.QueryActions.html)\n      * [*search*](resources/query/struct.SearchRequestBuilder.html), [*suggest*](resources/query/struct.SuggestRequestBuilder.html)\n      * [sources](resources/query/sources/struct.SourcesActions.html)\n        * [*list*](resources/query/sources/struct.ListRequestBuilder.html)\n    * [settings](resources/settings/struct.SettingsActions.html)\n      * [*getCustomer*](resources/settings/struct.GetCustomerRequestBuilder.html), [*updateCustomer*](resources/settings/struct.UpdateCustomerRequestBuilder.html)\n      * [datasources](resources/settings/datasources/struct.DatasourcesActions.html)\n        * [*create*](resources/settings/datasources/struct.CreateRequestBuilder.html), [*delete*](resources/settings/datasources/struct.DeleteRequestBuilder.html), [*get*](resources/settings/datasources/struct.GetRequestBuilder.html), [*list*](resources/settings/datasources/struct.ListRequestBuilder.html), [*update*](resources/settings/datasources/struct.UpdateRequestBuilder.html)\n      * [searchapplications](resources/settings/searchapplications/struct.SearchapplicationsActions.html)\n        * [*create*](resources/settings/searchapplications/struct.CreateRequestBuilder.html), [*delete*](resources/settings/searchapplications/struct.DeleteRequestBuilder.html), [*get*](resources/settings/searchapplications/struct.GetRequestBuilder.html), [*list*](resources/settings/searchapplications/struct.ListRequestBuilder.html), [*reset*](resources/settings/searchapplications/struct.ResetRequestBuilder.html), [*update*](resources/settings/searchapplications/struct.UpdateRequestBuilder.html)\n    * [stats](resources/stats/struct.StatsActions.html)\n      * [*getIndex*](resources/stats/struct.GetIndexRequestBuilder.html), [*getQuery*](resources/stats/struct.GetQueryRequestBuilder.html), [*getSearchapplication*](resources/stats/struct.GetSearchapplicationRequestBuilder.html), [*getSession*](resources/stats/struct.GetSessionRequestBuilder.html), [*getUser*](resources/stats/struct.GetUserRequestBuilder.html)\n      * [index](resources/stats/index/struct.IndexActions.html)\n        * [datasources](resources/stats/index/datasources/struct.DatasourcesActions.html)\n          * [*get*](resources/stats/index/datasources/struct.GetRequestBuilder.html)\n      * [query](resources/stats/query/struct.QueryActions.html)\n        * [searchapplications](resources/stats/query/searchapplications/struct.SearchapplicationsActions.html)\n          * [*get*](resources/stats/query/searchapplications/struct.GetRequestBuilder.html)\n      * [session](resources/stats/session/struct.SessionActions.html)\n        * [searchapplications](resources/stats/session/searchapplications/struct.SearchapplicationsActions.html)\n          * [*get*](resources/stats/session/searchapplications/struct.GetRequestBuilder.html)\n      * [user](resources/stats/user/struct.UserActions.html)\n        * [searchapplications](resources/stats/user/searchapplications/struct.SearchapplicationsActions.html)\n          * [*get*](resources/stats/user/searchapplications/struct.GetRequestBuilder.html)\n    * [v_1](resources/v_1/struct.V1Actions.html)\n      * [*initializeCustomer*](resources/v_1/struct.InitializeCustomerRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "Index and serve your organization's data with Cloud Search\n\n`https://www.googleapis.com/auth/cloud_search`"]
    pub const CLOUD_SEARCH: &str = "https://www.googleapis.com/auth/cloud_search";
    #[doc = "Index and serve your organization's data with Cloud Search\n\n`https://www.googleapis.com/auth/cloud_search.debug`"]
    pub const CLOUD_SEARCH_DEBUG: &str = "https://www.googleapis.com/auth/cloud_search.debug";
    #[doc = "Index and serve your organization's data with Cloud Search\n\n`https://www.googleapis.com/auth/cloud_search.indexing`"]
    pub const CLOUD_SEARCH_INDEXING: &str = "https://www.googleapis.com/auth/cloud_search.indexing";
    #[doc = "Search your organization's data in the Cloud Search index\n\n`https://www.googleapis.com/auth/cloud_search.query`"]
    pub const CLOUD_SEARCH_QUERY: &str = "https://www.googleapis.com/auth/cloud_search.query";
    #[doc = "Index and serve your organization's data with Cloud Search\n\n`https://www.googleapis.com/auth/cloud_search.settings`"]
    pub const CLOUD_SEARCH_SETTINGS: &str = "https://www.googleapis.com/auth/cloud_search.settings";
    #[doc = "Index and serve your organization's data with Cloud Search\n\n`https://www.googleapis.com/auth/cloud_search.settings.indexing`"]
    pub const CLOUD_SEARCH_SETTINGS_INDEXING: &str =
        "https://www.googleapis.com/auth/cloud_search.settings.indexing";
    #[doc = "Index and serve your organization's data with Cloud Search\n\n`https://www.googleapis.com/auth/cloud_search.settings.query`"]
    pub const CLOUD_SEARCH_SETTINGS_QUERY: &str =
        "https://www.googleapis.com/auth/cloud_search.settings.query";
    #[doc = "Index and serve your organization's data with Cloud Search\n\n`https://www.googleapis.com/auth/cloud_search.stats`"]
    pub const CLOUD_SEARCH_STATS: &str = "https://www.googleapis.com/auth/cloud_search.stats";
    #[doc = "Index and serve your organization's data with Cloud Search\n\n`https://www.googleapis.com/auth/cloud_search.stats.indexing`"]
    pub const CLOUD_SEARCH_STATS_INDEXING: &str =
        "https://www.googleapis.com/auth/cloud_search.stats.indexing";
}
pub mod schemas {
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
    pub struct AclInfo {
        #[doc = "Number of groups which have at least read access to the document."]
        #[serde(
            rename = "groupsCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub groups_count: ::std::option::Option<i32>,
        #[doc = "The scope to which the content was shared."]
        #[serde(
            rename = "scope",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scope: ::std::option::Option<crate::schemas::AclInfoScope>,
        #[doc = "Number of users which have at least read access to the document."]
        #[serde(
            rename = "usersCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub users_count: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for AclInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AclInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AclInfoScope {
        #[doc = "Now it works only for google.com. Anybody at the same domain. Now it works only"]
        DasherDomain,
        #[doc = "Anybody at the same domain with the link."]
        DasherDomainWithLink,
        #[doc = "Explicit set of people and groups."]
        Limited,
        #[doc = "Anybody."]
        Public,
        #[doc = "for google.com. Anybody with the link."]
        PublicWithLink,
        #[doc = "Special tag to indicate TeamDrive scope."]
        TeamDrive,
    }
    impl AclInfoScope {
        pub fn as_str(self) -> &'static str {
            match self {
                AclInfoScope::DasherDomain => "DASHER_DOMAIN",
                AclInfoScope::DasherDomainWithLink => "DASHER_DOMAIN_WITH_LINK",
                AclInfoScope::Limited => "LIMITED",
                AclInfoScope::Public => "PUBLIC",
                AclInfoScope::PublicWithLink => "PUBLIC_WITH_LINK",
                AclInfoScope::TeamDrive => "TEAM_DRIVE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AclInfoScope {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AclInfoScope {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AclInfoScope, ()> {
            Ok(match s {
                "DASHER_DOMAIN" => AclInfoScope::DasherDomain,
                "DASHER_DOMAIN_WITH_LINK" => AclInfoScope::DasherDomainWithLink,
                "LIMITED" => AclInfoScope::Limited,
                "PUBLIC" => AclInfoScope::Public,
                "PUBLIC_WITH_LINK" => AclInfoScope::PublicWithLink,
                "TEAM_DRIVE" => AclInfoScope::TeamDrive,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AclInfoScope {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AclInfoScope {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AclInfoScope {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DASHER_DOMAIN" => AclInfoScope::DasherDomain,
                "DASHER_DOMAIN_WITH_LINK" => AclInfoScope::DasherDomainWithLink,
                "LIMITED" => AclInfoScope::Limited,
                "PUBLIC" => AclInfoScope::Public,
                "PUBLIC_WITH_LINK" => AclInfoScope::PublicWithLink,
                "TEAM_DRIVE" => AclInfoScope::TeamDrive,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AclInfoScope {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AclInfoScope {
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
    pub struct AppId {
        #[doc = "Enum indicating the type of App this is."]
        #[serde(
            rename = "appType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_type: ::std::option::Option<crate::schemas::AppIdAppType>,
        #[doc = "Enum indicating which 1P App this is when app_type is GSUITE_APP. Determined & set by the 1P API as a convenience for all users of this identifier(Eg. clients, chime, backend etc.) to map to 1P properties."]
        #[serde(
            rename = "gsuiteAppType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gsuite_app_type: ::std::option::Option<crate::schemas::AppIdGsuiteAppType>,
        #[doc = "Numeric identifier of the App. Set to Project number for 1/3P Apps. For Webhook, this is WebhookId. Determined & set by the 1P API from App credentials on the side channel."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub id: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for AppId {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AppId {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AppIdAppType {
        #[doc = "3P APP eg. external Bots(Asana Bot), 1P Bots(Drive Bot)."]
        App,
        AppTypeUnspecified,
        #[doc = "1P APP eg. Tasks, Meet, Docs, Calendar.."]
        GsuiteApp,
        #[doc = "Asynchronous messages via an incoming webhook."]
        IncomingWebhook,
    }
    impl AppIdAppType {
        pub fn as_str(self) -> &'static str {
            match self {
                AppIdAppType::App => "APP",
                AppIdAppType::AppTypeUnspecified => "APP_TYPE_UNSPECIFIED",
                AppIdAppType::GsuiteApp => "GSUITE_APP",
                AppIdAppType::IncomingWebhook => "INCOMING_WEBHOOK",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AppIdAppType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AppIdAppType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AppIdAppType, ()> {
            Ok(match s {
                "APP" => AppIdAppType::App,
                "APP_TYPE_UNSPECIFIED" => AppIdAppType::AppTypeUnspecified,
                "GSUITE_APP" => AppIdAppType::GsuiteApp,
                "INCOMING_WEBHOOK" => AppIdAppType::IncomingWebhook,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AppIdAppType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AppIdAppType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AppIdAppType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APP" => AppIdAppType::App,
                "APP_TYPE_UNSPECIFIED" => AppIdAppType::AppTypeUnspecified,
                "GSUITE_APP" => AppIdAppType::GsuiteApp,
                "INCOMING_WEBHOOK" => AppIdAppType::IncomingWebhook,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AppIdAppType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AppIdAppType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AppIdGsuiteAppType {
        ActivityFeedApp,
        #[doc = "Powered by Bullseye"]
        AssistiveSuggestionApp,
        CalendarApp,
        ContactsApp,
        DocsApp,
        DriveApp,
        GsuiteAppTypeUnspecified,
        MeetApp,
        SheetsApp,
        SlidesApp,
        TasksApp,
    }
    impl AppIdGsuiteAppType {
        pub fn as_str(self) -> &'static str {
            match self {
                AppIdGsuiteAppType::ActivityFeedApp => "ACTIVITY_FEED_APP",
                AppIdGsuiteAppType::AssistiveSuggestionApp => "ASSISTIVE_SUGGESTION_APP",
                AppIdGsuiteAppType::CalendarApp => "CALENDAR_APP",
                AppIdGsuiteAppType::ContactsApp => "CONTACTS_APP",
                AppIdGsuiteAppType::DocsApp => "DOCS_APP",
                AppIdGsuiteAppType::DriveApp => "DRIVE_APP",
                AppIdGsuiteAppType::GsuiteAppTypeUnspecified => "GSUITE_APP_TYPE_UNSPECIFIED",
                AppIdGsuiteAppType::MeetApp => "MEET_APP",
                AppIdGsuiteAppType::SheetsApp => "SHEETS_APP",
                AppIdGsuiteAppType::SlidesApp => "SLIDES_APP",
                AppIdGsuiteAppType::TasksApp => "TASKS_APP",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AppIdGsuiteAppType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AppIdGsuiteAppType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AppIdGsuiteAppType, ()> {
            Ok(match s {
                "ACTIVITY_FEED_APP" => AppIdGsuiteAppType::ActivityFeedApp,
                "ASSISTIVE_SUGGESTION_APP" => AppIdGsuiteAppType::AssistiveSuggestionApp,
                "CALENDAR_APP" => AppIdGsuiteAppType::CalendarApp,
                "CONTACTS_APP" => AppIdGsuiteAppType::ContactsApp,
                "DOCS_APP" => AppIdGsuiteAppType::DocsApp,
                "DRIVE_APP" => AppIdGsuiteAppType::DriveApp,
                "GSUITE_APP_TYPE_UNSPECIFIED" => AppIdGsuiteAppType::GsuiteAppTypeUnspecified,
                "MEET_APP" => AppIdGsuiteAppType::MeetApp,
                "SHEETS_APP" => AppIdGsuiteAppType::SheetsApp,
                "SLIDES_APP" => AppIdGsuiteAppType::SlidesApp,
                "TASKS_APP" => AppIdGsuiteAppType::TasksApp,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AppIdGsuiteAppType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AppIdGsuiteAppType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AppIdGsuiteAppType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVITY_FEED_APP" => AppIdGsuiteAppType::ActivityFeedApp,
                "ASSISTIVE_SUGGESTION_APP" => AppIdGsuiteAppType::AssistiveSuggestionApp,
                "CALENDAR_APP" => AppIdGsuiteAppType::CalendarApp,
                "CONTACTS_APP" => AppIdGsuiteAppType::ContactsApp,
                "DOCS_APP" => AppIdGsuiteAppType::DocsApp,
                "DRIVE_APP" => AppIdGsuiteAppType::DriveApp,
                "GSUITE_APP_TYPE_UNSPECIFIED" => AppIdGsuiteAppType::GsuiteAppTypeUnspecified,
                "MEET_APP" => AppIdGsuiteAppType::MeetApp,
                "SHEETS_APP" => AppIdGsuiteAppType::SheetsApp,
                "SLIDES_APP" => AppIdGsuiteAppType::SlidesApp,
                "TASKS_APP" => AppIdGsuiteAppType::TasksApp,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AppIdGsuiteAppType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AppIdGsuiteAppType {
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
    pub struct AuditLoggingSettings {
        #[doc = "Indicates whether audit logging is on/off for admin activity read APIs i.e. Get/List DataSources, Get/List SearchApplications etc."]
        #[serde(
            rename = "logAdminReadActions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub log_admin_read_actions: ::std::option::Option<bool>,
        #[doc = "Indicates whether audit logging is on/off for data access read APIs i.e. ListItems, GetItem etc."]
        #[serde(
            rename = "logDataReadActions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub log_data_read_actions: ::std::option::Option<bool>,
        #[doc = "Indicates whether audit logging is on/off for data access write APIs i.e. IndexItem etc."]
        #[serde(
            rename = "logDataWriteActions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub log_data_write_actions: ::std::option::Option<bool>,
        #[doc = "The resource name of the GCP Project to store audit logs. Cloud audit logging will be enabled after project_name has been updated through CustomerService. Format: projects/{project_id}"]
        #[serde(
            rename = "project",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AuditLoggingSettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AuditLoggingSettings {
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
    pub struct AvatarInfo {
        #[serde(
            rename = "emoji",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub emoji: ::std::option::Option<crate::schemas::Emoji>,
    }
    impl ::google_field_selector::FieldSelector for AvatarInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AvatarInfo {
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
    pub struct BooleanOperatorOptions {
        #[doc = "Indicates the operator name required in the query in order to isolate the boolean property. For example, if operatorName is *closed* and the property's name is *isClosed*, then queries like *closed:<value>* show results only where the value of the property named *isClosed* matches *<value>*. By contrast, a search that uses the same *<value>* without an operator returns all items where *<value>* matches the value of any String properties or text within the content field for the item. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
        #[serde(
            rename = "operatorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BooleanOperatorOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BooleanOperatorOptions {
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
    pub struct BooleanPropertyOptions {
        #[doc = "If set, describes how the boolean should be used as a search operator."]
        #[serde(
            rename = "operatorOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator_options: ::std::option::Option<crate::schemas::BooleanOperatorOptions>,
    }
    impl ::google_field_selector::FieldSelector for BooleanPropertyOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BooleanPropertyOptions {
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
    pub struct CheckAccessResponse {
        #[doc = "Returns true if principal has access. Returns false otherwise."]
        #[serde(
            rename = "hasAccess",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub has_access: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for CheckAccessResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CheckAccessResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CompositeFilter {
        #[doc = "The logic operator of the sub filter."]
        #[serde(
            rename = "logicOperator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub logic_operator: ::std::option::Option<crate::schemas::CompositeFilterLogicOperator>,
        #[doc = "Sub filters."]
        #[serde(
            rename = "subFilters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sub_filters: ::std::option::Option<Vec<crate::schemas::Filter>>,
    }
    impl ::google_field_selector::FieldSelector for CompositeFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CompositeFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CompositeFilterLogicOperator {
        #[doc = "Logical operators, which can only be applied to sub filters."]
        And,
        #[doc = "NOT can only be applied on a single sub filter."]
        Not,
        Or,
    }
    impl CompositeFilterLogicOperator {
        pub fn as_str(self) -> &'static str {
            match self {
                CompositeFilterLogicOperator::And => "AND",
                CompositeFilterLogicOperator::Not => "NOT",
                CompositeFilterLogicOperator::Or => "OR",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CompositeFilterLogicOperator {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CompositeFilterLogicOperator {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CompositeFilterLogicOperator, ()> {
            Ok(match s {
                "AND" => CompositeFilterLogicOperator::And,
                "NOT" => CompositeFilterLogicOperator::Not,
                "OR" => CompositeFilterLogicOperator::Or,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CompositeFilterLogicOperator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CompositeFilterLogicOperator {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CompositeFilterLogicOperator {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AND" => CompositeFilterLogicOperator::And,
                "NOT" => CompositeFilterLogicOperator::Not,
                "OR" => CompositeFilterLogicOperator::Or,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CompositeFilterLogicOperator {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CompositeFilterLogicOperator {
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
    pub struct ContextAttribute {
        #[doc = "The name of the attribute. It should not be empty. The maximum length is 32 characters. The name must start with a letter and can only contain letters (A-Z, a-z) or numbers (0-9). The name will be normalized (lower-cased) before being matched."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Text values of the attribute. The maximum number of elements is 10. The maximum length of an element in the array is 32 characters. The value will be normalized (lower-cased) before being matched."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for ContextAttribute {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContextAttribute {
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
    pub struct CustomEmoji {
        #[doc = "ID for the underlying image data in Blobstore. This field should *only* be present in Spanner or within the server, but should not be exposed in public APIs."]
        #[serde(
            rename = "blobId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub blob_id: ::std::option::Option<String>,
        #[doc = "Time when the Emoji was created, in microseconds. This field may be present in Spanner, within the server, or in public APIs."]
        #[serde(
            rename = "createTimeMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub create_time_micros: ::std::option::Option<i64>,
        #[doc = "This field should *never* be persisted to Spanner."]
        #[serde(
            rename = "creatorUserId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub creator_user_id: ::std::option::Option<crate::schemas::UserId>,
        #[doc = "This field should *never* be persisted to Spanner."]
        #[serde(
            rename = "ownerCustomerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub owner_customer_id: ::std::option::Option<crate::schemas::CustomerId>,
        #[doc = "Opaque token that clients use to construct the URL for accessing the custom emoji’s image data. This field is intended for API consumption, and should *never* be persisted to Spanner."]
        #[serde(
            rename = "readToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub read_token: ::std::option::Option<String>,
        #[doc = "User-provided, human-readable ID for the custom emoji. Users are expected to observe this field in the UI instead of the UUID. This shortcode should be unique within an organization, but has no global uniqueness guarantees, unlike the UUID. This field should *never* be persisted to Spanner."]
        #[serde(
            rename = "shortcode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub shortcode: ::std::option::Option<String>,
        #[doc = "Snapshot of the current state of the emoji, which may differ from the source-of-truth in the CustomEmojis table. This field should *never* be persisted to Spanner."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::CustomEmojiState>,
        #[serde(
            rename = "updateTimeMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub update_time_micros: ::std::option::Option<i64>,
        #[doc = "Unique key for a custom emoji resource. Required. This field is *always* populated."]
        #[serde(
            rename = "uuid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uuid: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CustomEmoji {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomEmoji {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CustomEmojiState {
        #[doc = "Emoji is removed everywhere and is not available to end-users."]
        EmojiDeleted,
        #[doc = "Emoji is visible and available to be used, subject to access control requirements."]
        EmojiEnabled,
        #[doc = "Emoji is hidden from pickers, so new usages are not allowed, but is not removed from existing embeddings."]
        EmojiHidden,
        EmojiStateUnspecified,
        #[doc = "Emoji can no longer be used (e.g. due to a shortcode conflict), but is not removed from existing embeddings."]
        EmojiSystemDisabled,
    }
    impl CustomEmojiState {
        pub fn as_str(self) -> &'static str {
            match self {
                CustomEmojiState::EmojiDeleted => "EMOJI_DELETED",
                CustomEmojiState::EmojiEnabled => "EMOJI_ENABLED",
                CustomEmojiState::EmojiHidden => "EMOJI_HIDDEN",
                CustomEmojiState::EmojiStateUnspecified => "EMOJI_STATE_UNSPECIFIED",
                CustomEmojiState::EmojiSystemDisabled => "EMOJI_SYSTEM_DISABLED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CustomEmojiState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CustomEmojiState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CustomEmojiState, ()> {
            Ok(match s {
                "EMOJI_DELETED" => CustomEmojiState::EmojiDeleted,
                "EMOJI_ENABLED" => CustomEmojiState::EmojiEnabled,
                "EMOJI_HIDDEN" => CustomEmojiState::EmojiHidden,
                "EMOJI_STATE_UNSPECIFIED" => CustomEmojiState::EmojiStateUnspecified,
                "EMOJI_SYSTEM_DISABLED" => CustomEmojiState::EmojiSystemDisabled,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CustomEmojiState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CustomEmojiState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CustomEmojiState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EMOJI_DELETED" => CustomEmojiState::EmojiDeleted,
                "EMOJI_ENABLED" => CustomEmojiState::EmojiEnabled,
                "EMOJI_HIDDEN" => CustomEmojiState::EmojiHidden,
                "EMOJI_STATE_UNSPECIFIED" => CustomEmojiState::EmojiStateUnspecified,
                "EMOJI_SYSTEM_DISABLED" => CustomEmojiState::EmojiSystemDisabled,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CustomEmojiState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomEmojiState {
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
    pub struct CustomerId {
        #[serde(
            rename = "customerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub customer_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CustomerId {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomerId {
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
    pub struct CustomerIndexStats {
        #[doc = "Date for which statistics were calculated."]
        #[serde(
            rename = "date",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "Number of items aggregrated by status code."]
        #[serde(
            rename = "itemCountByStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub item_count_by_status: ::std::option::Option<Vec<crate::schemas::ItemCountByStatus>>,
    }
    impl ::google_field_selector::FieldSelector for CustomerIndexStats {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomerIndexStats {
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
    pub struct CustomerQueryStats {
        #[doc = "Date for which query stats were calculated. Stats calculated on the next day close to midnight are returned."]
        #[serde(
            rename = "date",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date: ::std::option::Option<crate::schemas::Date>,
        #[serde(
            rename = "queryCountByStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query_count_by_status: ::std::option::Option<Vec<crate::schemas::QueryCountByStatus>>,
    }
    impl ::google_field_selector::FieldSelector for CustomerQueryStats {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomerQueryStats {
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
    pub struct CustomerSearchApplicationStats {
        #[doc = "The count of search applications for the date."]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub count: ::std::option::Option<i64>,
        #[doc = "Date for which search application stats were calculated."]
        #[serde(
            rename = "date",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date: ::std::option::Option<crate::schemas::Date>,
    }
    impl ::google_field_selector::FieldSelector for CustomerSearchApplicationStats {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomerSearchApplicationStats {
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
    pub struct CustomerSessionStats {
        #[doc = "Date for which session stats were calculated. Stats are calculated on the following day, close to midnight PST, and then returned."]
        #[serde(
            rename = "date",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "The count of search sessions on the day"]
        #[serde(
            rename = "searchSessionsCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub search_sessions_count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for CustomerSessionStats {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomerSessionStats {
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
    pub struct CustomerSettings {
        #[doc = "Audit Logging settings for the customer. If update_mask is empty then this field will be updated based on UpdateCustomerSettings request."]
        #[serde(
            rename = "auditLoggingSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audit_logging_settings: ::std::option::Option<crate::schemas::AuditLoggingSettings>,
        #[doc = "VPC SC settings for the customer. If update_mask is empty then this field will be updated based on UpdateCustomerSettings request."]
        #[serde(
            rename = "vpcSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vpc_settings: ::std::option::Option<crate::schemas::Vpcsettings>,
    }
    impl ::google_field_selector::FieldSelector for CustomerSettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomerSettings {
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
    pub struct CustomerUserStats {
        #[doc = "Date for which session stats were calculated. Stats calculated on the next day close to midnight are returned."]
        #[serde(
            rename = "date",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "The count of unique active users in the past one day"]
        #[serde(
            rename = "oneDayActiveUsersCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub one_day_active_users_count: ::std::option::Option<i64>,
        #[doc = "The count of unique active users in the past seven days"]
        #[serde(
            rename = "sevenDaysActiveUsersCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub seven_days_active_users_count: ::std::option::Option<i64>,
        #[doc = "The count of unique active users in the past thirty days"]
        #[serde(
            rename = "thirtyDaysActiveUsersCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub thirty_days_active_users_count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for CustomerUserStats {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomerUserStats {
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
    pub struct DataSource {
        #[doc = "If true, sets the datasource to read-only mode. In read-only mode, the Indexing API rejects any requests to index or delete items in this source. Enabling read-only mode does not stop the processing of previously accepted data."]
        #[serde(
            rename = "disableModifications",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_modifications: ::std::option::Option<bool>,
        #[doc = "Disable serving any search or assist results."]
        #[serde(
            rename = "disableServing",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_serving: ::std::option::Option<bool>,
        #[doc = "Required. Display name of the datasource The maximum length is 300 characters."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "List of service accounts that have indexing access."]
        #[serde(
            rename = "indexingServiceAccounts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub indexing_service_accounts: ::std::option::Option<Vec<String>>,
        #[doc = "This field restricts visibility to items at the datasource level. Items within the datasource are restricted to the union of users and groups included in this field. Note that, this does not ensure access to a specific item, as users need to have ACL permissions on the contained items. This ensures a high level access on the entire datasource, and that the individual items are not shared outside this visibility."]
        #[serde(
            rename = "itemsVisibility",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items_visibility: ::std::option::Option<Vec<crate::schemas::GsuitePrincipal>>,
        #[doc = "Name of the datasource resource. Format: datasources/{source_id}. The name is ignored when creating a datasource."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "IDs of the Long Running Operations (LROs) currently running for this schema."]
        #[serde(
            rename = "operationIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation_ids: ::std::option::Option<Vec<String>>,
        #[doc = "Can a user request to get thumbnail URI for Items indexed in this data source."]
        #[serde(
            rename = "returnThumbnailUrls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub return_thumbnail_urls: ::std::option::Option<bool>,
        #[doc = "A short name or alias for the source. This value will be used to match the 'source' operator. For example, if the short name is *<value>* then queries like *source:<value>* will only return results for this source. The value must be unique across all datasources. The value must only contain alphanumeric characters (a-zA-Z0-9). The value cannot start with 'google' and cannot be one of the following: mail, gmail, docs, drive, groups, sites, calendar, hangouts, gplus, keep, people, teams. Its maximum length is 32 characters."]
        #[serde(
            rename = "shortName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub short_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DataSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DataSource {
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
    pub struct DataSourceIndexStats {
        #[doc = "Date for which index stats were calculated. If the date of request is not the current date then stats calculated on the next day are returned. Stats are calculated close to mid night in this case. If date of request is current date, then real time stats are returned."]
        #[serde(
            rename = "date",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "Number of items aggregrated by status code."]
        #[serde(
            rename = "itemCountByStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub item_count_by_status: ::std::option::Option<Vec<crate::schemas::ItemCountByStatus>>,
    }
    impl ::google_field_selector::FieldSelector for DataSourceIndexStats {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DataSourceIndexStats {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DataSourceRestriction {
        #[doc = "Filter options restricting the results. If multiple filters are present, they are grouped by object type before joining. Filters with the same object type are joined conjunctively, then the resulting expressions are joined disjunctively. The maximum number of elements is 20. NOTE: Suggest API supports only few filters at the moment: \"objecttype\", \"type\" and \"mimetype\". For now, schema specific filters cannot be used to filter suggestions."]
        #[serde(
            rename = "filterOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter_options: ::std::option::Option<Vec<crate::schemas::FilterOptions>>,
        #[doc = "The source of restriction."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<crate::schemas::Source>,
    }
    impl ::google_field_selector::FieldSelector for DataSourceRestriction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DataSourceRestriction {
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
    pub struct Date {
        #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
        #[serde(
            rename = "day",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub day: ::std::option::Option<i32>,
        #[doc = "Month of date. Must be from 1 to 12."]
        #[serde(
            rename = "month",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub month: ::std::option::Option<i32>,
        #[doc = "Year of date. Must be from 1 to 9999."]
        #[serde(
            rename = "year",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub year: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Date {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Date {
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
    pub struct DateOperatorOptions {
        #[doc = "Indicates the operator name required in the query in order to isolate the date property using the greater-than operator. For example, if greaterThanOperatorName is *closedafter* and the property's name is *closeDate*, then queries like *closedafter:<value>* show results only where the value of the property named *closeDate* is later than *<value>*. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
        #[serde(
            rename = "greaterThanOperatorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub greater_than_operator_name: ::std::option::Option<String>,
        #[doc = "Indicates the operator name required in the query in order to isolate the date property using the less-than operator. For example, if lessThanOperatorName is *closedbefore* and the property's name is *closeDate*, then queries like *closedbefore:<value>* show results only where the value of the property named *closeDate* is earlier than *<value>*. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
        #[serde(
            rename = "lessThanOperatorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub less_than_operator_name: ::std::option::Option<String>,
        #[doc = "Indicates the actual string required in the query in order to isolate the date property. For example, suppose an issue tracking schema object has a property named *closeDate* that specifies an operator with an operatorName of *closedon*. For searches on that data, queries like *closedon:<value>* show results only where the value of the *closeDate* property matches *<value>*. By contrast, a search that uses the same *<value>* without an operator returns all items where *<value>* matches the value of any String properties or text within the content field for the indexed datasource. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
        #[serde(
            rename = "operatorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DateOperatorOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DateOperatorOptions {
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
    pub struct DatePropertyOptions {
        #[doc = "If set, describes how the date should be used as a search operator."]
        #[serde(
            rename = "operatorOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator_options: ::std::option::Option<crate::schemas::DateOperatorOptions>,
    }
    impl ::google_field_selector::FieldSelector for DatePropertyOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DatePropertyOptions {
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
    pub struct DateValues {
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<crate::schemas::Date>>,
    }
    impl ::google_field_selector::FieldSelector for DateValues {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DateValues {
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
    pub struct DebugOptions {
        #[doc = "If you are asked by Google to help with debugging, set this field. Otherwise, ignore this field."]
        #[serde(
            rename = "enableDebugging",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_debugging: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for DebugOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DebugOptions {
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
    pub struct DeleteQueueItemsRequest {
        #[doc = "Name of connector making this call. Format: datasources/{source_id}/connectors/{ID}"]
        #[serde(
            rename = "connectorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub connector_name: ::std::option::Option<String>,
        #[doc = "Common debug options."]
        #[serde(
            rename = "debugOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub debug_options: ::std::option::Option<crate::schemas::DebugOptions>,
        #[doc = "Name of a queue to delete items from."]
        #[serde(
            rename = "queue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub queue: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DeleteQueueItemsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeleteQueueItemsRequest {
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
    pub struct DisplayedProperty {
        #[doc = "The name of the top-level property as defined in a property definition for the object. If the name is not a defined property in the schema, an error is given when attempting to update the schema."]
        #[serde(
            rename = "propertyName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub property_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DisplayedProperty {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DisplayedProperty {
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
    pub struct DmId {
        #[doc = "Unique server assigned Id, per Direct Message Space."]
        #[serde(
            rename = "dmId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dm_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DmId {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DmId {
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
    pub struct DoubleOperatorOptions {
        #[doc = "Indicates the operator name required in the query in order to use the double property in sorting or as a facet. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
        #[serde(
            rename = "operatorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DoubleOperatorOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DoubleOperatorOptions {
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
    pub struct DoublePropertyOptions {
        #[doc = "If set, describes how the double should be used as a search operator."]
        #[serde(
            rename = "operatorOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator_options: ::std::option::Option<crate::schemas::DoubleOperatorOptions>,
    }
    impl ::google_field_selector::FieldSelector for DoublePropertyOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DoublePropertyOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DoubleValues {
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<f64>>,
    }
    impl ::google_field_selector::FieldSelector for DoubleValues {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DoubleValues {
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
    pub struct DriveFollowUpRestrict {
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::DriveFollowUpRestrictType>,
    }
    impl ::google_field_selector::FieldSelector for DriveFollowUpRestrict {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DriveFollowUpRestrict {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DriveFollowUpRestrictType {
        FollowupActionItems,
        FollowupSuggestions,
        Unspecified,
    }
    impl DriveFollowUpRestrictType {
        pub fn as_str(self) -> &'static str {
            match self {
                DriveFollowUpRestrictType::FollowupActionItems => "FOLLOWUP_ACTION_ITEMS",
                DriveFollowUpRestrictType::FollowupSuggestions => "FOLLOWUP_SUGGESTIONS",
                DriveFollowUpRestrictType::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DriveFollowUpRestrictType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DriveFollowUpRestrictType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DriveFollowUpRestrictType, ()> {
            Ok(match s {
                "FOLLOWUP_ACTION_ITEMS" => DriveFollowUpRestrictType::FollowupActionItems,
                "FOLLOWUP_SUGGESTIONS" => DriveFollowUpRestrictType::FollowupSuggestions,
                "UNSPECIFIED" => DriveFollowUpRestrictType::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DriveFollowUpRestrictType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DriveFollowUpRestrictType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DriveFollowUpRestrictType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FOLLOWUP_ACTION_ITEMS" => DriveFollowUpRestrictType::FollowupActionItems,
                "FOLLOWUP_SUGGESTIONS" => DriveFollowUpRestrictType::FollowupSuggestions,
                "UNSPECIFIED" => DriveFollowUpRestrictType::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DriveFollowUpRestrictType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DriveFollowUpRestrictType {
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
    pub struct DriveLocationRestrict {
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::DriveLocationRestrictType>,
    }
    impl ::google_field_selector::FieldSelector for DriveLocationRestrict {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DriveLocationRestrict {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DriveLocationRestrictType {
        Starred,
        Trashed,
        Unspecified,
    }
    impl DriveLocationRestrictType {
        pub fn as_str(self) -> &'static str {
            match self {
                DriveLocationRestrictType::Starred => "STARRED",
                DriveLocationRestrictType::Trashed => "TRASHED",
                DriveLocationRestrictType::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DriveLocationRestrictType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DriveLocationRestrictType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DriveLocationRestrictType, ()> {
            Ok(match s {
                "STARRED" => DriveLocationRestrictType::Starred,
                "TRASHED" => DriveLocationRestrictType::Trashed,
                "UNSPECIFIED" => DriveLocationRestrictType::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DriveLocationRestrictType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DriveLocationRestrictType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DriveLocationRestrictType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "STARRED" => DriveLocationRestrictType::Starred,
                "TRASHED" => DriveLocationRestrictType::Trashed,
                "UNSPECIFIED" => DriveLocationRestrictType::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DriveLocationRestrictType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DriveLocationRestrictType {
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
    pub struct DriveMimeTypeRestrict {
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::DriveMimeTypeRestrictType>,
    }
    impl ::google_field_selector::FieldSelector for DriveMimeTypeRestrict {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DriveMimeTypeRestrict {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DriveMimeTypeRestrictType {
        Archive,
        Audio,
        Document,
        Drawing,
        Folder,
        Form,
        Image,
        Map,
        Pdf,
        Presentation,
        Script,
        Site,
        Spreadsheet,
        Unspecified,
        Video,
    }
    impl DriveMimeTypeRestrictType {
        pub fn as_str(self) -> &'static str {
            match self {
                DriveMimeTypeRestrictType::Archive => "ARCHIVE",
                DriveMimeTypeRestrictType::Audio => "AUDIO",
                DriveMimeTypeRestrictType::Document => "DOCUMENT",
                DriveMimeTypeRestrictType::Drawing => "DRAWING",
                DriveMimeTypeRestrictType::Folder => "FOLDER",
                DriveMimeTypeRestrictType::Form => "FORM",
                DriveMimeTypeRestrictType::Image => "IMAGE",
                DriveMimeTypeRestrictType::Map => "MAP",
                DriveMimeTypeRestrictType::Pdf => "PDF",
                DriveMimeTypeRestrictType::Presentation => "PRESENTATION",
                DriveMimeTypeRestrictType::Script => "SCRIPT",
                DriveMimeTypeRestrictType::Site => "SITE",
                DriveMimeTypeRestrictType::Spreadsheet => "SPREADSHEET",
                DriveMimeTypeRestrictType::Unspecified => "UNSPECIFIED",
                DriveMimeTypeRestrictType::Video => "VIDEO",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DriveMimeTypeRestrictType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DriveMimeTypeRestrictType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DriveMimeTypeRestrictType, ()> {
            Ok(match s {
                "ARCHIVE" => DriveMimeTypeRestrictType::Archive,
                "AUDIO" => DriveMimeTypeRestrictType::Audio,
                "DOCUMENT" => DriveMimeTypeRestrictType::Document,
                "DRAWING" => DriveMimeTypeRestrictType::Drawing,
                "FOLDER" => DriveMimeTypeRestrictType::Folder,
                "FORM" => DriveMimeTypeRestrictType::Form,
                "IMAGE" => DriveMimeTypeRestrictType::Image,
                "MAP" => DriveMimeTypeRestrictType::Map,
                "PDF" => DriveMimeTypeRestrictType::Pdf,
                "PRESENTATION" => DriveMimeTypeRestrictType::Presentation,
                "SCRIPT" => DriveMimeTypeRestrictType::Script,
                "SITE" => DriveMimeTypeRestrictType::Site,
                "SPREADSHEET" => DriveMimeTypeRestrictType::Spreadsheet,
                "UNSPECIFIED" => DriveMimeTypeRestrictType::Unspecified,
                "VIDEO" => DriveMimeTypeRestrictType::Video,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DriveMimeTypeRestrictType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DriveMimeTypeRestrictType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DriveMimeTypeRestrictType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ARCHIVE" => DriveMimeTypeRestrictType::Archive,
                "AUDIO" => DriveMimeTypeRestrictType::Audio,
                "DOCUMENT" => DriveMimeTypeRestrictType::Document,
                "DRAWING" => DriveMimeTypeRestrictType::Drawing,
                "FOLDER" => DriveMimeTypeRestrictType::Folder,
                "FORM" => DriveMimeTypeRestrictType::Form,
                "IMAGE" => DriveMimeTypeRestrictType::Image,
                "MAP" => DriveMimeTypeRestrictType::Map,
                "PDF" => DriveMimeTypeRestrictType::Pdf,
                "PRESENTATION" => DriveMimeTypeRestrictType::Presentation,
                "SCRIPT" => DriveMimeTypeRestrictType::Script,
                "SITE" => DriveMimeTypeRestrictType::Site,
                "SPREADSHEET" => DriveMimeTypeRestrictType::Spreadsheet,
                "UNSPECIFIED" => DriveMimeTypeRestrictType::Unspecified,
                "VIDEO" => DriveMimeTypeRestrictType::Video,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DriveMimeTypeRestrictType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DriveMimeTypeRestrictType {
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
    pub struct DriveTimeSpanRestrict {
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::DriveTimeSpanRestrictType>,
    }
    impl ::google_field_selector::FieldSelector for DriveTimeSpanRestrict {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DriveTimeSpanRestrict {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DriveTimeSpanRestrictType {
        #[doc = "Not Enabled"]
        Last30Days,
        Last7Days,
        #[doc = "Not Enabled"]
        Last90Days,
        Today,
        Unspecified,
        Yesterday,
    }
    impl DriveTimeSpanRestrictType {
        pub fn as_str(self) -> &'static str {
            match self {
                DriveTimeSpanRestrictType::Last30Days => "LAST_30_DAYS",
                DriveTimeSpanRestrictType::Last7Days => "LAST_7_DAYS",
                DriveTimeSpanRestrictType::Last90Days => "LAST_90_DAYS",
                DriveTimeSpanRestrictType::Today => "TODAY",
                DriveTimeSpanRestrictType::Unspecified => "UNSPECIFIED",
                DriveTimeSpanRestrictType::Yesterday => "YESTERDAY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DriveTimeSpanRestrictType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DriveTimeSpanRestrictType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DriveTimeSpanRestrictType, ()> {
            Ok(match s {
                "LAST_30_DAYS" => DriveTimeSpanRestrictType::Last30Days,
                "LAST_7_DAYS" => DriveTimeSpanRestrictType::Last7Days,
                "LAST_90_DAYS" => DriveTimeSpanRestrictType::Last90Days,
                "TODAY" => DriveTimeSpanRestrictType::Today,
                "UNSPECIFIED" => DriveTimeSpanRestrictType::Unspecified,
                "YESTERDAY" => DriveTimeSpanRestrictType::Yesterday,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DriveTimeSpanRestrictType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DriveTimeSpanRestrictType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DriveTimeSpanRestrictType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LAST_30_DAYS" => DriveTimeSpanRestrictType::Last30Days,
                "LAST_7_DAYS" => DriveTimeSpanRestrictType::Last7Days,
                "LAST_90_DAYS" => DriveTimeSpanRestrictType::Last90Days,
                "TODAY" => DriveTimeSpanRestrictType::Today,
                "UNSPECIFIED" => DriveTimeSpanRestrictType::Unspecified,
                "YESTERDAY" => DriveTimeSpanRestrictType::Yesterday,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DriveTimeSpanRestrictType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DriveTimeSpanRestrictType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DynamiteSpacesScoringInfo {
        #[serde(
            rename = "affinityScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub affinity_score: ::std::option::Option<f64>,
        #[serde(
            rename = "commonContactCountAffinityScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub common_contact_count_affinity_score: ::std::option::Option<f64>,
        #[serde(
            rename = "contactsIntersectionCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub contacts_intersection_count: ::std::option::Option<f64>,
        #[serde(
            rename = "finalScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub final_score: ::std::option::Option<f64>,
        #[serde(
            rename = "freshnessScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub freshness_score: ::std::option::Option<f64>,
        #[serde(
            rename = "joinedSpacesAffinityScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub joined_spaces_affinity_score: ::std::option::Option<f64>,
        #[serde(
            rename = "lastMessagePostedTimestampMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub last_message_posted_timestamp_micros: ::std::option::Option<i64>,
        #[serde(
            rename = "memberMetadataCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub member_metadata_count: ::std::option::Option<f64>,
        #[serde(
            rename = "messageScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message_score: ::std::option::Option<f64>,
        #[serde(
            rename = "numAucContacts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub num_auc_contacts: ::std::option::Option<i64>,
        #[serde(
            rename = "smallContactListAffinityScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub small_contact_list_affinity_score: ::std::option::Option<f64>,
        #[serde(
            rename = "smallUnjoinedSpacesAffinityScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub small_unjoined_spaces_affinity_score: ::std::option::Option<f64>,
        #[serde(
            rename = "spaceAgeInDays",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub space_age_in_days: ::std::option::Option<f64>,
        #[serde(
            rename = "spaceCreationTimestampMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub space_creation_timestamp_micros: ::std::option::Option<i64>,
        #[serde(
            rename = "topicalityScore",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub topicality_score: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for DynamiteSpacesScoringInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DynamiteSpacesScoringInfo {
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
    pub struct EmailAddress {
        #[doc = "The email address."]
        #[serde(
            rename = "emailAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email_address: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EmailAddress {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EmailAddress {
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
    pub struct Emoji {
        #[doc = "A custom emoji."]
        #[serde(
            rename = "customEmoji",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_emoji: ::std::option::Option<crate::schemas::CustomEmoji>,
        #[doc = "A basic emoji represented by a unicode string."]
        #[serde(
            rename = "unicode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unicode: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Emoji {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Emoji {
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
    pub struct EnumOperatorOptions {
        #[doc = "Indicates the operator name required in the query in order to isolate the enum property. For example, if operatorName is *priority* and the property's name is *priorityVal*, then queries like *priority:<value>* show results only where the value of the property named *priorityVal* matches *<value>*. By contrast, a search that uses the same *<value>* without an operator returns all items where *<value>* matches the value of any String properties or text within the content field for the item. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
        #[serde(
            rename = "operatorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EnumOperatorOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnumOperatorOptions {
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
    pub struct EnumPropertyOptions {
        #[doc = "If set, describes how the enum should be used as a search operator."]
        #[serde(
            rename = "operatorOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator_options: ::std::option::Option<crate::schemas::EnumOperatorOptions>,
        #[doc = "Used to specify the ordered ranking for the enumeration that determines how the integer values provided in the possible EnumValuePairs are used to rank results. If specified, integer values must be provided for all possible EnumValuePair values given for this property. Can only be used if isRepeatable is false."]
        #[serde(
            rename = "orderedRanking",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ordered_ranking:
            ::std::option::Option<crate::schemas::EnumPropertyOptionsOrderedRanking>,
        #[doc = "The list of possible values for the enumeration property. All EnumValuePairs must provide a string value. If you specify an integer value for one EnumValuePair, then all possible EnumValuePairs must provide an integer value. Both the string value and integer value must be unique over all possible values. Once set, possible values cannot be removed or modified. If you supply an ordered ranking and think you might insert additional enum values in the future, leave gaps in the initial integer values to allow adding a value in between previously registered values. The maximum number of elements is 100."]
        #[serde(
            rename = "possibleValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub possible_values: ::std::option::Option<Vec<crate::schemas::EnumValuePair>>,
    }
    impl ::google_field_selector::FieldSelector for EnumPropertyOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnumPropertyOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EnumPropertyOptionsOrderedRanking {
        #[doc = "This property is ranked in ascending order. Lower values indicate lower ranking."]
        Ascending,
        #[doc = "This property is ranked in descending order. Lower values indicate higher ranking."]
        Descending,
        #[doc = "There is no ranking order for the property. Results aren't adjusted by this property's value."]
        NoOrder,
    }
    impl EnumPropertyOptionsOrderedRanking {
        pub fn as_str(self) -> &'static str {
            match self {
                EnumPropertyOptionsOrderedRanking::Ascending => "ASCENDING",
                EnumPropertyOptionsOrderedRanking::Descending => "DESCENDING",
                EnumPropertyOptionsOrderedRanking::NoOrder => "NO_ORDER",
            }
        }
    }
    impl ::std::convert::AsRef<str> for EnumPropertyOptionsOrderedRanking {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for EnumPropertyOptionsOrderedRanking {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<EnumPropertyOptionsOrderedRanking, ()> {
            Ok(match s {
                "ASCENDING" => EnumPropertyOptionsOrderedRanking::Ascending,
                "DESCENDING" => EnumPropertyOptionsOrderedRanking::Descending,
                "NO_ORDER" => EnumPropertyOptionsOrderedRanking::NoOrder,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for EnumPropertyOptionsOrderedRanking {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EnumPropertyOptionsOrderedRanking {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EnumPropertyOptionsOrderedRanking {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ASCENDING" => EnumPropertyOptionsOrderedRanking::Ascending,
                "DESCENDING" => EnumPropertyOptionsOrderedRanking::Descending,
                "NO_ORDER" => EnumPropertyOptionsOrderedRanking::NoOrder,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for EnumPropertyOptionsOrderedRanking {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnumPropertyOptionsOrderedRanking {
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
    pub struct EnumValuePair {
        #[doc = "The integer value of the EnumValuePair which must be non-negative. Optional."]
        #[serde(
            rename = "integerValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub integer_value: ::std::option::Option<i32>,
        #[doc = "The string value of the EnumValuePair. The maximum length is 32 characters."]
        #[serde(
            rename = "stringValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub string_value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EnumValuePair {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnumValuePair {
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
    pub struct EnumValues {
        #[doc = "The maximum allowable length for string values is 32 characters."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for EnumValues {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EnumValues {
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
    pub struct ErrorInfo {
        #[serde(
            rename = "errorMessages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_messages: ::std::option::Option<Vec<crate::schemas::ErrorMessage>>,
    }
    impl ::google_field_selector::FieldSelector for ErrorInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ErrorInfo {
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
    pub struct ErrorMessage {
        #[serde(
            rename = "errorMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_message: ::std::option::Option<String>,
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<crate::schemas::Source>,
    }
    impl ::google_field_selector::FieldSelector for ErrorMessage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ErrorMessage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct FacetBucket {
        #[doc = "Number of results that match the bucket value. Counts are only returned for searches when count accuracy is ensured. Cloud Search does not guarantee facet counts for any query and facet counts might be present only intermittently, even for identical queries. Do not build dependencies on facet count existence; instead use facet ount percentages which are always returned."]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub count: ::std::option::Option<i32>,
        #[doc = "Percent of results that match the bucket value. The returned value is between (0-100], and is rounded down to an integer if fractional. If the value is not explicitly returned, it represents a percentage value that rounds to 0. Percentages are returned for all searches, but are an estimate. Because percentages are always returned, you should render percentages instead of counts."]
        #[serde(
            rename = "percentage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub percentage: ::std::option::Option<i32>,
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<crate::schemas::Value>,
    }
    impl ::google_field_selector::FieldSelector for FacetBucket {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FacetBucket {
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
    pub struct FacetOptions {
        #[doc = "Maximum number of facet buckets that should be returned for this facet. Defaults to 10. Maximum value is 100."]
        #[serde(
            rename = "numFacetBuckets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub num_facet_buckets: ::std::option::Option<i32>,
        #[doc = "If object_type is set, only those objects of that type will be used to compute facets. If empty, then all objects will be used to compute facets."]
        #[serde(
            rename = "objectType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_type: ::std::option::Option<String>,
        #[doc = "Name of the operator chosen for faceting. @see cloudsearch.SchemaPropertyOptions"]
        #[serde(
            rename = "operatorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator_name: ::std::option::Option<String>,
        #[doc = "Source name to facet on. Format: datasources/{source_id} If empty, all data sources will be used."]
        #[serde(
            rename = "sourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for FacetOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FacetOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct FacetResult {
        #[doc = "FacetBuckets for values in response containing at least a single result."]
        #[serde(
            rename = "buckets",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub buckets: ::std::option::Option<Vec<crate::schemas::FacetBucket>>,
        #[doc = "Object type for which facet results are returned. Can be empty."]
        #[serde(
            rename = "objectType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_type: ::std::option::Option<String>,
        #[doc = "Name of the operator chosen for faceting. @see cloudsearch.SchemaPropertyOptions"]
        #[serde(
            rename = "operatorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator_name: ::std::option::Option<String>,
        #[doc = "Source name for which facet results are returned. Will not be empty."]
        #[serde(
            rename = "sourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for FacetResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FacetResult {
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
    pub struct FieldViolation {
        #[doc = "Description of the error."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Path of field with violation."]
        #[serde(
            rename = "field",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for FieldViolation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FieldViolation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Filter {
        #[serde(
            rename = "compositeFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub composite_filter: ::std::option::Option<crate::schemas::CompositeFilter>,
        #[serde(
            rename = "valueFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value_filter: ::std::option::Option<crate::schemas::ValueFilter>,
    }
    impl ::google_field_selector::FieldSelector for Filter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Filter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct FilterOptions {
        #[doc = "Generic filter to restrict the search, such as `lang:en`, `site:xyz`."]
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter: ::std::option::Option<crate::schemas::Filter>,
        #[doc = "If object_type is set, only objects of that type are returned. This should correspond to the name of the object that was registered within the definition of schema. The maximum length is 256 characters."]
        #[serde(
            rename = "objectType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for FilterOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FilterOptions {
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
    pub struct FreshnessOptions {
        #[doc = "The duration after which an object should be considered stale. The default value is 180 days (in seconds)."]
        #[serde(
            rename = "freshnessDuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub freshness_duration: ::std::option::Option<String>,
        #[doc = "This property indicates the freshness level of the object in the index. If set, this property must be a top-level property within the property definitions and it must be a timestamp type or date type. Otherwise, the Indexing API uses updateTime as the freshness indicator. The maximum length is 256 characters. When a property is used to calculate freshness, the value defaults to 2 years from the current time."]
        #[serde(
            rename = "freshnessProperty",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub freshness_property: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for FreshnessOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FreshnessOptions {
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
    pub struct GetCustomerIndexStatsResponse {
        #[doc = "Average item count for the given date range for which billing is done."]
        #[serde(
            rename = "averageIndexedItemCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub average_indexed_item_count: ::std::option::Option<i64>,
        #[doc = "Summary of indexed item counts, one for each day in the requested range."]
        #[serde(
            rename = "stats",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stats: ::std::option::Option<Vec<crate::schemas::CustomerIndexStats>>,
    }
    impl ::google_field_selector::FieldSelector for GetCustomerIndexStatsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetCustomerIndexStatsResponse {
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
    pub struct GetCustomerQueryStatsResponse {
        #[serde(
            rename = "stats",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stats: ::std::option::Option<Vec<crate::schemas::CustomerQueryStats>>,
        #[doc = "Total successful query count (status code 200) for the given date range."]
        #[serde(
            rename = "totalQueryCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_query_count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GetCustomerQueryStatsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetCustomerQueryStatsResponse {
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
    pub struct GetCustomerSearchApplicationStatsResponse {
        #[doc = "Average search application count for the given date range."]
        #[serde(
            rename = "averageSearchApplicationCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub average_search_application_count: ::std::option::Option<i64>,
        #[doc = "Search application stats by date."]
        #[serde(
            rename = "stats",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stats: ::std::option::Option<Vec<crate::schemas::CustomerSearchApplicationStats>>,
    }
    impl ::google_field_selector::FieldSelector for GetCustomerSearchApplicationStatsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetCustomerSearchApplicationStatsResponse {
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
    pub struct GetCustomerSessionStatsResponse {
        #[serde(
            rename = "stats",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stats: ::std::option::Option<Vec<crate::schemas::CustomerSessionStats>>,
    }
    impl ::google_field_selector::FieldSelector for GetCustomerSessionStatsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetCustomerSessionStatsResponse {
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
    pub struct GetCustomerUserStatsResponse {
        #[serde(
            rename = "stats",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stats: ::std::option::Option<Vec<crate::schemas::CustomerUserStats>>,
    }
    impl ::google_field_selector::FieldSelector for GetCustomerUserStatsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetCustomerUserStatsResponse {
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
    pub struct GetDataSourceIndexStatsResponse {
        #[doc = "Average item count for the given date range for which billing is done."]
        #[serde(
            rename = "averageIndexedItemCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub average_indexed_item_count: ::std::option::Option<i64>,
        #[doc = "Summary of indexed item counts, one for each day in the requested range."]
        #[serde(
            rename = "stats",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stats: ::std::option::Option<Vec<crate::schemas::DataSourceIndexStats>>,
    }
    impl ::google_field_selector::FieldSelector for GetDataSourceIndexStatsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetDataSourceIndexStatsResponse {
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
    pub struct GetSearchApplicationQueryStatsResponse {
        #[doc = "Query stats per date for a search application."]
        #[serde(
            rename = "stats",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stats: ::std::option::Option<Vec<crate::schemas::SearchApplicationQueryStats>>,
        #[doc = "Total successful query count (status code 200) for the given date range."]
        #[serde(
            rename = "totalQueryCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_query_count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GetSearchApplicationQueryStatsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetSearchApplicationQueryStatsResponse {
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
    pub struct GetSearchApplicationSessionStatsResponse {
        #[serde(
            rename = "stats",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stats: ::std::option::Option<Vec<crate::schemas::SearchApplicationSessionStats>>,
    }
    impl ::google_field_selector::FieldSelector for GetSearchApplicationSessionStatsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetSearchApplicationSessionStatsResponse {
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
    pub struct GetSearchApplicationUserStatsResponse {
        #[serde(
            rename = "stats",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub stats: ::std::option::Option<Vec<crate::schemas::SearchApplicationUserStats>>,
    }
    impl ::google_field_selector::FieldSelector for GetSearchApplicationUserStatsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetSearchApplicationUserStatsResponse {
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
    pub struct GoogleDocsMetadata {
        #[doc = "Contains number of users and groups which can access the document."]
        #[serde(
            rename = "aclInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub acl_info: ::std::option::Option<crate::schemas::AclInfo>,
        #[doc = "The conceptual type (presentation, document, etc.) of this document."]
        #[serde(
            rename = "documentType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub document_type: ::std::option::Option<crate::schemas::GoogleDocsMetadataDocumentType>,
        #[doc = "The file extension of the document. NOTE: As of October 2018 this field is not backfilled for old documents."]
        #[serde(
            rename = "fileExtension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_extension: ::std::option::Option<String>,
        #[doc = "The last time this document was modified, in seconds since epoch. Only counts content modifications."]
        #[serde(
            rename = "lastContentModifiedTimestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub last_content_modified_timestamp: ::std::option::Option<i64>,
        #[doc = "Additional per-result information, akin to Gmail's SingleThreadResponse. Note: GWS no longer seems to use this field, but there's still one reference to it for Scribe, so we can't remove it."]
        #[serde(
            rename = "resultInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub result_info: ::std::option::Option<crate::schemas::GoogleDocsResultInfo>,
        #[doc = "Contains additional information about the document depending on its type."]
        #[serde(
            rename = "typeInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub type_info: ::std::option::Option<crate::schemas::TypeInfo>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDocsMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDocsMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleDocsMetadataDocumentType {
        #[doc = "Fall-back for unknown Gdrive types."]
        BinaryBlob,
        #[doc = "Writely, Word, etc."]
        Document,
        #[doc = "For Atari page and site drafts"]
        DraftSite,
        DraftSitePage,
        Drawing,
        Folder,
        Form,
        FusionTable,
        Image,
        #[doc = "Jamboard Jams (go/jam)"]
        Jam,
        #[doc = "File types for Gdrive objects are below."]
        Pdf,
        #[doc = "Presently, PowerPoint, etc."]
        Presentation,
        Script,
        #[doc = "Drive Shortcuts (go/shortcuts)"]
        Shortcut,
        #[doc = "Trix, Excel, etc."]
        Spreadsheet,
        #[doc = "If the type is unknown or not represented in this enum."]
        Unknown,
        Video,
    }
    impl GoogleDocsMetadataDocumentType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleDocsMetadataDocumentType::BinaryBlob => "BINARY_BLOB",
                GoogleDocsMetadataDocumentType::Document => "DOCUMENT",
                GoogleDocsMetadataDocumentType::DraftSite => "DRAFT_SITE",
                GoogleDocsMetadataDocumentType::DraftSitePage => "DRAFT_SITE_PAGE",
                GoogleDocsMetadataDocumentType::Drawing => "DRAWING",
                GoogleDocsMetadataDocumentType::Folder => "FOLDER",
                GoogleDocsMetadataDocumentType::Form => "FORM",
                GoogleDocsMetadataDocumentType::FusionTable => "FUSION_TABLE",
                GoogleDocsMetadataDocumentType::Image => "IMAGE",
                GoogleDocsMetadataDocumentType::Jam => "JAM",
                GoogleDocsMetadataDocumentType::Pdf => "PDF",
                GoogleDocsMetadataDocumentType::Presentation => "PRESENTATION",
                GoogleDocsMetadataDocumentType::Script => "SCRIPT",
                GoogleDocsMetadataDocumentType::Shortcut => "SHORTCUT",
                GoogleDocsMetadataDocumentType::Spreadsheet => "SPREADSHEET",
                GoogleDocsMetadataDocumentType::Unknown => "UNKNOWN",
                GoogleDocsMetadataDocumentType::Video => "VIDEO",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleDocsMetadataDocumentType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleDocsMetadataDocumentType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleDocsMetadataDocumentType, ()> {
            Ok(match s {
                "BINARY_BLOB" => GoogleDocsMetadataDocumentType::BinaryBlob,
                "DOCUMENT" => GoogleDocsMetadataDocumentType::Document,
                "DRAFT_SITE" => GoogleDocsMetadataDocumentType::DraftSite,
                "DRAFT_SITE_PAGE" => GoogleDocsMetadataDocumentType::DraftSitePage,
                "DRAWING" => GoogleDocsMetadataDocumentType::Drawing,
                "FOLDER" => GoogleDocsMetadataDocumentType::Folder,
                "FORM" => GoogleDocsMetadataDocumentType::Form,
                "FUSION_TABLE" => GoogleDocsMetadataDocumentType::FusionTable,
                "IMAGE" => GoogleDocsMetadataDocumentType::Image,
                "JAM" => GoogleDocsMetadataDocumentType::Jam,
                "PDF" => GoogleDocsMetadataDocumentType::Pdf,
                "PRESENTATION" => GoogleDocsMetadataDocumentType::Presentation,
                "SCRIPT" => GoogleDocsMetadataDocumentType::Script,
                "SHORTCUT" => GoogleDocsMetadataDocumentType::Shortcut,
                "SPREADSHEET" => GoogleDocsMetadataDocumentType::Spreadsheet,
                "UNKNOWN" => GoogleDocsMetadataDocumentType::Unknown,
                "VIDEO" => GoogleDocsMetadataDocumentType::Video,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleDocsMetadataDocumentType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleDocsMetadataDocumentType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleDocsMetadataDocumentType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BINARY_BLOB" => GoogleDocsMetadataDocumentType::BinaryBlob,
                "DOCUMENT" => GoogleDocsMetadataDocumentType::Document,
                "DRAFT_SITE" => GoogleDocsMetadataDocumentType::DraftSite,
                "DRAFT_SITE_PAGE" => GoogleDocsMetadataDocumentType::DraftSitePage,
                "DRAWING" => GoogleDocsMetadataDocumentType::Drawing,
                "FOLDER" => GoogleDocsMetadataDocumentType::Folder,
                "FORM" => GoogleDocsMetadataDocumentType::Form,
                "FUSION_TABLE" => GoogleDocsMetadataDocumentType::FusionTable,
                "IMAGE" => GoogleDocsMetadataDocumentType::Image,
                "JAM" => GoogleDocsMetadataDocumentType::Jam,
                "PDF" => GoogleDocsMetadataDocumentType::Pdf,
                "PRESENTATION" => GoogleDocsMetadataDocumentType::Presentation,
                "SCRIPT" => GoogleDocsMetadataDocumentType::Script,
                "SHORTCUT" => GoogleDocsMetadataDocumentType::Shortcut,
                "SPREADSHEET" => GoogleDocsMetadataDocumentType::Spreadsheet,
                "UNKNOWN" => GoogleDocsMetadataDocumentType::Unknown,
                "VIDEO" => GoogleDocsMetadataDocumentType::Video,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleDocsMetadataDocumentType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDocsMetadataDocumentType {
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
    pub struct GoogleDocsResultInfo {
        #[doc = "The SHA1 hash of the object in Drive, if any."]
        #[serde(
            rename = "attachmentSha1",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attachment_sha_1: ::std::option::Option<String>,
        #[doc = "The storage identifier for the object in Cosmo. This field is intended to used by Stratus/Moonshine integration only. It should not be exposed externally (please refer to encrypted_id for that purpose)."]
        #[serde(
            rename = "cosmoId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cosmo_id: ::std::option::Option<crate::schemas::Id>,
        #[doc = "For Cosmo objects, the Cosmo namespace the object was in. This allows downstream clients to identify whether a document was created in Writely or Kix, Presently or Punch, or whether it was uploaded from GDrive. See storage_cosmo.Id.NAME_SPACE for a list of all Cosmo name spaces."]
        #[serde(
            rename = "cosmoNameSpace",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cosmo_name_space: ::std::option::Option<i32>,
        #[doc = "The encrypted (user-visible) id of this object. Knowing the id is sufficient to create a canonical URL for this document."]
        #[serde(
            rename = "encryptedId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub encrypted_id: ::std::option::Option<String>,
        #[doc = "The mimetype of the document."]
        #[serde(
            rename = "mimeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mime_type: ::std::option::Option<String>,
        #[doc = "The visibility indicator in the UI will be based upon this."]
        #[serde(
            rename = "shareScope",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub share_scope: ::std::option::Option<crate::schemas::ShareScope>,
    }
    impl ::google_field_selector::FieldSelector for GoogleDocsResultInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleDocsResultInfo {
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
    pub struct GroupId {
        #[doc = "Unique, immutable ID of the Direct Message Space"]
        #[serde(
            rename = "dmId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dm_id: ::std::option::Option<crate::schemas::DmId>,
        #[doc = "Unique, immutable ID of the Space"]
        #[serde(
            rename = "spaceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub space_id: ::std::option::Option<crate::schemas::SpaceId>,
    }
    impl ::google_field_selector::FieldSelector for GroupId {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GroupId {
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
    pub struct GsuitePrincipal {
        #[doc = "This principal represents all users of the G Suite domain of the customer."]
        #[serde(
            rename = "gsuiteDomain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gsuite_domain: ::std::option::Option<bool>,
        #[doc = "This principal references a G Suite group account"]
        #[serde(
            rename = "gsuiteGroupEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gsuite_group_email: ::std::option::Option<String>,
        #[doc = "This principal references a G Suite user account"]
        #[serde(
            rename = "gsuiteUserEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gsuite_user_email: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GsuitePrincipal {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GsuitePrincipal {
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
    pub struct HtmlOperatorOptions {
        #[doc = "Indicates the operator name required in the query in order to isolate the html property. For example, if operatorName is *subject* and the property's name is *subjectLine*, then queries like *subject:<value>* show results only where the value of the property named *subjectLine* matches *<value>*. By contrast, a search that uses the same *<value>* without an operator return all items where *<value>* matches the value of any html properties or text within the content field for the item. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
        #[serde(
            rename = "operatorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for HtmlOperatorOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HtmlOperatorOptions {
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
    pub struct HtmlPropertyOptions {
        #[doc = "If set, describes how the property should be used as a search operator."]
        #[serde(
            rename = "operatorOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator_options: ::std::option::Option<crate::schemas::HtmlOperatorOptions>,
        #[doc = "Indicates the search quality importance of the tokens within the field when used for retrieval. Can only be set to DEFAULT or NONE."]
        #[serde(
            rename = "retrievalImportance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub retrieval_importance: ::std::option::Option<crate::schemas::RetrievalImportance>,
    }
    impl ::google_field_selector::FieldSelector for HtmlPropertyOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HtmlPropertyOptions {
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
    pub struct HtmlValues {
        #[doc = "The maximum allowable length for html values is 2048 characters."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for HtmlValues {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HtmlValues {
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
    pub struct Id {
        #[doc = "The User account in which the DirEntry was originally created. If name_space==GAIA, then it's the gaia_id of the user this id is referring to."]
        #[serde(
            rename = "creatorUserId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub creator_user_id: ::std::option::Option<u64>,
        #[doc = "The local identifier for the DirEntry (local to the creator's account). local_id + app_name is guaranteed to be unique within the creator account, but not across all User accounts. The string is case sensitive. Ignore if name_space==GAIA. NB For name_space==COSMO, all local_id's should be defined in google3/java/com/google/storage/cosmo/server/api/SpecialObjectIds.java as they have a special predefined meaning. See cosmo.client.CosmoIdFactory.createObjectId(long,String) for IMPORTANT recommendations when generating IDs."]
        #[serde(
            rename = "localId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub local_id: ::std::option::Option<String>,
        #[doc = "The name space in which this id is unique (typically the application that created it). Values should be drawn from the above enum, but for experimentation, use values greater than 1000."]
        #[serde(
            rename = "nameSpace",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name_space: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for Id {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Id {
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
    pub struct IndexItemOptions {
        #[doc = "Specifies if the index request should allow gsuite principals that do not exist or are deleted in the index request."]
        #[serde(
            rename = "allowUnknownGsuitePrincipals",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allow_unknown_gsuite_principals: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for IndexItemOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IndexItemOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct IndexItemRequest {
        #[doc = "Name of connector making this call. Format: datasources/{source_id}/connectors/{ID}"]
        #[serde(
            rename = "connectorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub connector_name: ::std::option::Option<String>,
        #[doc = "Common debug options."]
        #[serde(
            rename = "debugOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub debug_options: ::std::option::Option<crate::schemas::DebugOptions>,
        #[serde(
            rename = "indexItemOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub index_item_options: ::std::option::Option<crate::schemas::IndexItemOptions>,
        #[doc = "Name of the item. Format: datasources/{source_id}/items/{item_id}"]
        #[serde(
            rename = "item",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub item: ::std::option::Option<crate::schemas::Item>,
        #[doc = "Required. The RequestMode for this request."]
        #[serde(
            rename = "mode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mode: ::std::option::Option<crate::schemas::IndexItemRequestMode>,
    }
    impl ::google_field_selector::FieldSelector for IndexItemRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IndexItemRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum IndexItemRequestMode {
        #[doc = "For changes that are executed after the response is sent back to the caller."]
        Asynchronous,
        #[doc = "For real-time updates."]
        Synchronous,
        #[doc = "Priority is not specified in the update request. Leaving priority unspecified results in an update failure."]
        Unspecified,
    }
    impl IndexItemRequestMode {
        pub fn as_str(self) -> &'static str {
            match self {
                IndexItemRequestMode::Asynchronous => "ASYNCHRONOUS",
                IndexItemRequestMode::Synchronous => "SYNCHRONOUS",
                IndexItemRequestMode::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for IndexItemRequestMode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for IndexItemRequestMode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<IndexItemRequestMode, ()> {
            Ok(match s {
                "ASYNCHRONOUS" => IndexItemRequestMode::Asynchronous,
                "SYNCHRONOUS" => IndexItemRequestMode::Synchronous,
                "UNSPECIFIED" => IndexItemRequestMode::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for IndexItemRequestMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for IndexItemRequestMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for IndexItemRequestMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ASYNCHRONOUS" => IndexItemRequestMode::Asynchronous,
                "SYNCHRONOUS" => IndexItemRequestMode::Synchronous,
                "UNSPECIFIED" => IndexItemRequestMode::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for IndexItemRequestMode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IndexItemRequestMode {
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
    pub struct InitializeCustomerRequest {}
    impl ::google_field_selector::FieldSelector for InitializeCustomerRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InitializeCustomerRequest {
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
    pub struct IntegerOperatorOptions {
        #[doc = "Indicates the operator name required in the query in order to isolate the integer property using the greater-than operator. For example, if greaterThanOperatorName is *priorityabove* and the property's name is *priorityVal*, then queries like *priorityabove:<value>* show results only where the value of the property named *priorityVal* is greater than *<value>*. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
        #[serde(
            rename = "greaterThanOperatorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub greater_than_operator_name: ::std::option::Option<String>,
        #[doc = "Indicates the operator name required in the query in order to isolate the integer property using the less-than operator. For example, if lessThanOperatorName is *prioritybelow* and the property's name is *priorityVal*, then queries like *prioritybelow:<value>* show results only where the value of the property named *priorityVal* is less than *<value>*. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
        #[serde(
            rename = "lessThanOperatorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub less_than_operator_name: ::std::option::Option<String>,
        #[doc = "Indicates the operator name required in the query in order to isolate the integer property. For example, if operatorName is *priority* and the property's name is *priorityVal*, then queries like *priority:<value>* show results only where the value of the property named *priorityVal* matches *<value>*. By contrast, a search that uses the same *<value>* without an operator returns all items where *<value>* matches the value of any String properties or text within the content field for the item. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
        #[serde(
            rename = "operatorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for IntegerOperatorOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IntegerOperatorOptions {
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
    pub struct IntegerPropertyOptions {
        #[doc = "The maximum value of the property. The minimum and maximum values for the property are used to rank results according to the ordered ranking. Indexing requests with values greater than the maximum are accepted and ranked with the same weight as items indexed with the maximum value."]
        #[serde(
            rename = "maximumValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub maximum_value: ::std::option::Option<i64>,
        #[doc = "The minimum value of the property. The minimum and maximum values for the property are used to rank results according to the ordered ranking. Indexing requests with values less than the minimum are accepted and ranked with the same weight as items indexed with the minimum value."]
        #[serde(
            rename = "minimumValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub minimum_value: ::std::option::Option<i64>,
        #[doc = "If set, describes how the integer should be used as a search operator."]
        #[serde(
            rename = "operatorOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator_options: ::std::option::Option<crate::schemas::IntegerOperatorOptions>,
        #[doc = "Used to specify the ordered ranking for the integer. Can only be used if isRepeatable is false."]
        #[serde(
            rename = "orderedRanking",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ordered_ranking:
            ::std::option::Option<crate::schemas::IntegerPropertyOptionsOrderedRanking>,
    }
    impl ::google_field_selector::FieldSelector for IntegerPropertyOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IntegerPropertyOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum IntegerPropertyOptionsOrderedRanking {
        #[doc = "This property is ranked in ascending order. Lower values indicate lower ranking."]
        Ascending,
        #[doc = "This property is ranked in descending order. Lower values indicate higher ranking."]
        Descending,
        #[doc = "There is no ranking order for the property. Results are not adjusted by this property's value."]
        NoOrder,
    }
    impl IntegerPropertyOptionsOrderedRanking {
        pub fn as_str(self) -> &'static str {
            match self {
                IntegerPropertyOptionsOrderedRanking::Ascending => "ASCENDING",
                IntegerPropertyOptionsOrderedRanking::Descending => "DESCENDING",
                IntegerPropertyOptionsOrderedRanking::NoOrder => "NO_ORDER",
            }
        }
    }
    impl ::std::convert::AsRef<str> for IntegerPropertyOptionsOrderedRanking {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for IntegerPropertyOptionsOrderedRanking {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<IntegerPropertyOptionsOrderedRanking, ()> {
            Ok(match s {
                "ASCENDING" => IntegerPropertyOptionsOrderedRanking::Ascending,
                "DESCENDING" => IntegerPropertyOptionsOrderedRanking::Descending,
                "NO_ORDER" => IntegerPropertyOptionsOrderedRanking::NoOrder,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for IntegerPropertyOptionsOrderedRanking {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for IntegerPropertyOptionsOrderedRanking {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for IntegerPropertyOptionsOrderedRanking {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ASCENDING" => IntegerPropertyOptionsOrderedRanking::Ascending,
                "DESCENDING" => IntegerPropertyOptionsOrderedRanking::Descending,
                "NO_ORDER" => IntegerPropertyOptionsOrderedRanking::NoOrder,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for IntegerPropertyOptionsOrderedRanking {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IntegerPropertyOptionsOrderedRanking {
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
    pub struct IntegerValues {
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<i64>>,
    }
    impl ::google_field_selector::FieldSelector for IntegerValues {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IntegerValues {
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
    pub struct Interaction {
        #[doc = "The time when the user acted on the item. If multiple actions of the same type exist for a single user, only the most recent action is recorded."]
        #[serde(
            rename = "interactionTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub interaction_time: ::std::option::Option<String>,
        #[doc = "The user that acted on the item."]
        #[serde(
            rename = "principal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub principal: ::std::option::Option<crate::schemas::Principal>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::InteractionType>,
    }
    impl ::google_field_selector::FieldSelector for Interaction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Interaction {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum InteractionType {
        #[doc = "This interaction indicates the user edited the item."]
        Edit,
        #[doc = "Invalid value."]
        Unspecified,
        #[doc = "This interaction indicates the user viewed the item."]
        View,
    }
    impl InteractionType {
        pub fn as_str(self) -> &'static str {
            match self {
                InteractionType::Edit => "EDIT",
                InteractionType::Unspecified => "UNSPECIFIED",
                InteractionType::View => "VIEW",
            }
        }
    }
    impl ::std::convert::AsRef<str> for InteractionType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for InteractionType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<InteractionType, ()> {
            Ok(match s {
                "EDIT" => InteractionType::Edit,
                "UNSPECIFIED" => InteractionType::Unspecified,
                "VIEW" => InteractionType::View,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for InteractionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for InteractionType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for InteractionType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EDIT" => InteractionType::Edit,
                "UNSPECIFIED" => InteractionType::Unspecified,
                "VIEW" => InteractionType::View,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for InteractionType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InteractionType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Item {
        #[doc = "Access control list for this item."]
        #[serde(
            rename = "acl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub acl: ::std::option::Option<crate::schemas::ItemAcl>,
        #[doc = "Item content to be indexed and made text searchable."]
        #[serde(
            rename = "content",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content: ::std::option::Option<crate::schemas::ItemContent>,
        #[doc = "Type for this item."]
        #[serde(
            rename = "itemType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub item_type: ::std::option::Option<crate::schemas::ItemItemType>,
        #[doc = "Metadata information."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::ItemMetadata>,
        #[doc = "Name of the Item. Format: datasources/{source_id}/items/{item_id} This is a required field. The maximum length is 1536 characters."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Additional state connector can store for this item. The maximum length is 10000 bytes."]
        #[serde(
            rename = "payload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub payload: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Queue this item belongs to. The maximum length is 100 characters."]
        #[serde(
            rename = "queue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub queue: ::std::option::Option<String>,
        #[doc = "Status of the item. Output only field."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::ItemStatus>,
        #[doc = "The structured data for the item that should conform to a registered object definition in the schema for the data source."]
        #[serde(
            rename = "structuredData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub structured_data: ::std::option::Option<crate::schemas::ItemStructuredData>,
        #[doc = "Required. The indexing system stores the version from the datasource as a byte string and compares the Item version in the index to the version of the queued Item using lexical ordering. Cloud Search Indexing won't index or delete any queued item with a version value that is less than or equal to the version of the currently indexed item. The maximum length for this field is 1024 bytes. For information on how item version affects the deletion process, refer to [Handle revisions after manual deletes](https://developers.google.com/cloud-search/docs/guides/operations)."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for Item {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Item {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ItemItemType {
        #[doc = "An item that gets indexed and whose purpose is to supply other items with ACLs and/or contain other items."]
        ContainerItem,
        #[doc = "An item that is indexed for the only purpose of serving information. These items cannot be referred in containerName or inheritAclFrom fields."]
        ContentItem,
        Unspecified,
        #[doc = "An item that does not get indexed, but otherwise has the same purpose as CONTAINER_ITEM."]
        VirtualContainerItem,
    }
    impl ItemItemType {
        pub fn as_str(self) -> &'static str {
            match self {
                ItemItemType::ContainerItem => "CONTAINER_ITEM",
                ItemItemType::ContentItem => "CONTENT_ITEM",
                ItemItemType::Unspecified => "UNSPECIFIED",
                ItemItemType::VirtualContainerItem => "VIRTUAL_CONTAINER_ITEM",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ItemItemType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ItemItemType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ItemItemType, ()> {
            Ok(match s {
                "CONTAINER_ITEM" => ItemItemType::ContainerItem,
                "CONTENT_ITEM" => ItemItemType::ContentItem,
                "UNSPECIFIED" => ItemItemType::Unspecified,
                "VIRTUAL_CONTAINER_ITEM" => ItemItemType::VirtualContainerItem,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ItemItemType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ItemItemType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ItemItemType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONTAINER_ITEM" => ItemItemType::ContainerItem,
                "CONTENT_ITEM" => ItemItemType::ContentItem,
                "UNSPECIFIED" => ItemItemType::Unspecified,
                "VIRTUAL_CONTAINER_ITEM" => ItemItemType::VirtualContainerItem,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ItemItemType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ItemItemType {
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
    pub struct ItemAcl {
        #[doc = "Sets the type of access rules to apply when an item inherits its ACL from a parent. This should always be set in tandem with the inheritAclFrom field. Also, when the inheritAclFrom field is set, this field should be set to a valid AclInheritanceType."]
        #[serde(
            rename = "aclInheritanceType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub acl_inheritance_type: ::std::option::Option<crate::schemas::ItemAclAclInheritanceType>,
        #[doc = "List of principals who are explicitly denied access to the item in search results. While principals are denied access by default, use denied readers to handle exceptions and override the list allowed readers. The maximum number of elements is 100."]
        #[serde(
            rename = "deniedReaders",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub denied_readers: ::std::option::Option<Vec<crate::schemas::Principal>>,
        #[doc = "Name of the item to inherit the Access Permission List (ACL) from. Note: ACL inheritance *only* provides access permissions to child items and does not define structural relationships, nor does it provide convenient ways to delete large groups of items. Deleting an ACL parent from the index only alters the access permissions of child items that reference the parent in the inheritAclFrom field. The item is still in the index, but may not visible in search results. By contrast, deletion of a container item also deletes all items that reference the container via the containerName field. The maximum length for this field is 1536 characters."]
        #[serde(
            rename = "inheritAclFrom",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inherit_acl_from: ::std::option::Option<String>,
        #[doc = "Optional. List of owners for the item. This field has no bearing on document access permissions. It does, however, offer a slight ranking boosts items where the querying user is an owner. The maximum number of elements is 5."]
        #[serde(
            rename = "owners",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub owners: ::std::option::Option<Vec<crate::schemas::Principal>>,
        #[doc = "List of principals who are allowed to see the item in search results. Optional if inheriting permissions from another item or if the item is not intended to be visible, such as virtual containers. The maximum number of elements is 1000."]
        #[serde(
            rename = "readers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub readers: ::std::option::Option<Vec<crate::schemas::Principal>>,
    }
    impl ::google_field_selector::FieldSelector for ItemAcl {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ItemAcl {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ItemAclAclInheritanceType {
        #[doc = "Access is granted only if this item and the parent item specified in the inheritAclFrom field both permit read access."]
        BothPermit,
        #[doc = "During an authorization conflict, the ACL of the child item determines its read access."]
        ChildOverride,
        #[doc = "The default value when this item does not inherit an ACL. Use NOT_APPLICABLE when inheritAclFrom is empty. An item without ACL inheritance can still have ACLs supplied by its own readers and deniedReaders fields."]
        NotApplicable,
        #[doc = "During an authorization conflict, the ACL of the parent item specified in the inheritAclFrom field determines read access."]
        ParentOverride,
    }
    impl ItemAclAclInheritanceType {
        pub fn as_str(self) -> &'static str {
            match self {
                ItemAclAclInheritanceType::BothPermit => "BOTH_PERMIT",
                ItemAclAclInheritanceType::ChildOverride => "CHILD_OVERRIDE",
                ItemAclAclInheritanceType::NotApplicable => "NOT_APPLICABLE",
                ItemAclAclInheritanceType::ParentOverride => "PARENT_OVERRIDE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ItemAclAclInheritanceType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ItemAclAclInheritanceType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ItemAclAclInheritanceType, ()> {
            Ok(match s {
                "BOTH_PERMIT" => ItemAclAclInheritanceType::BothPermit,
                "CHILD_OVERRIDE" => ItemAclAclInheritanceType::ChildOverride,
                "NOT_APPLICABLE" => ItemAclAclInheritanceType::NotApplicable,
                "PARENT_OVERRIDE" => ItemAclAclInheritanceType::ParentOverride,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ItemAclAclInheritanceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ItemAclAclInheritanceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ItemAclAclInheritanceType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOTH_PERMIT" => ItemAclAclInheritanceType::BothPermit,
                "CHILD_OVERRIDE" => ItemAclAclInheritanceType::ChildOverride,
                "NOT_APPLICABLE" => ItemAclAclInheritanceType::NotApplicable,
                "PARENT_OVERRIDE" => ItemAclAclInheritanceType::ParentOverride,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ItemAclAclInheritanceType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ItemAclAclInheritanceType {
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
    pub struct ItemContent {
        #[doc = "Upload reference ID of a previously uploaded content via write method."]
        #[serde(
            rename = "contentDataRef",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_data_ref: ::std::option::Option<crate::schemas::UploadItemRef>,
        #[serde(
            rename = "contentFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_format: ::std::option::Option<crate::schemas::ItemContentContentFormat>,
        #[doc = "Hashing info calculated and provided by the API client for content. Can be used with the items.push method to calculate modified state. The maximum length is 2048 characters."]
        #[serde(
            rename = "hash",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hash: ::std::option::Option<String>,
        #[doc = "Content that is supplied inlined within the update method. The maximum length is 102400 bytes (100 KiB)."]
        #[serde(
            rename = "inlineContent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inline_content: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for ItemContent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ItemContent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ItemContentContentFormat {
        #[doc = "contentFormat is HTML."]
        Html,
        #[doc = "contentFormat is raw bytes."]
        Raw,
        #[doc = "contentFormat is free text."]
        Text,
        #[doc = "Invalid value."]
        Unspecified,
    }
    impl ItemContentContentFormat {
        pub fn as_str(self) -> &'static str {
            match self {
                ItemContentContentFormat::Html => "HTML",
                ItemContentContentFormat::Raw => "RAW",
                ItemContentContentFormat::Text => "TEXT",
                ItemContentContentFormat::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ItemContentContentFormat {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ItemContentContentFormat {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ItemContentContentFormat, ()> {
            Ok(match s {
                "HTML" => ItemContentContentFormat::Html,
                "RAW" => ItemContentContentFormat::Raw,
                "TEXT" => ItemContentContentFormat::Text,
                "UNSPECIFIED" => ItemContentContentFormat::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ItemContentContentFormat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ItemContentContentFormat {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ItemContentContentFormat {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "HTML" => ItemContentContentFormat::Html,
                "RAW" => ItemContentContentFormat::Raw,
                "TEXT" => ItemContentContentFormat::Text,
                "UNSPECIFIED" => ItemContentContentFormat::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ItemContentContentFormat {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ItemContentContentFormat {
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
    pub struct ItemCountByStatus {
        #[doc = "Number of items matching the status code."]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub count: ::std::option::Option<i64>,
        #[doc = "Number of items matching the status code for which billing is done. This excludes virtual container items from the total count. This count would not be applicable for items with ERROR or NEW_ITEM status code."]
        #[serde(
            rename = "indexedItemsCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub indexed_items_count: ::std::option::Option<i64>,
        #[doc = "Status of the items."]
        #[serde(
            rename = "statusCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status_code: ::std::option::Option<crate::schemas::ItemCountByStatusStatusCode>,
    }
    impl ::google_field_selector::FieldSelector for ItemCountByStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ItemCountByStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ItemCountByStatusStatusCode {
        #[doc = "API has accepted the up-to-date data of this item."]
        Accepted,
        #[doc = "Input-only value. Used with Items.list to list all items in the queue, regardless of status."]
        CodeUnspecified,
        #[doc = "Error encountered by Cloud Search while processing this item. Details of the error are in repositoryError."]
        Error,
        #[doc = "Item has been modified in the repository, and is out of date with the version previously accepted into Cloud Search."]
        Modified,
        #[doc = "Item is known to exist in the repository, but is not yet accepted by Cloud Search. An item can be in this state when Items.push has been called for an item of this name that did not exist previously."]
        NewItem,
    }
    impl ItemCountByStatusStatusCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ItemCountByStatusStatusCode::Accepted => "ACCEPTED",
                ItemCountByStatusStatusCode::CodeUnspecified => "CODE_UNSPECIFIED",
                ItemCountByStatusStatusCode::Error => "ERROR",
                ItemCountByStatusStatusCode::Modified => "MODIFIED",
                ItemCountByStatusStatusCode::NewItem => "NEW_ITEM",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ItemCountByStatusStatusCode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ItemCountByStatusStatusCode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ItemCountByStatusStatusCode, ()> {
            Ok(match s {
                "ACCEPTED" => ItemCountByStatusStatusCode::Accepted,
                "CODE_UNSPECIFIED" => ItemCountByStatusStatusCode::CodeUnspecified,
                "ERROR" => ItemCountByStatusStatusCode::Error,
                "MODIFIED" => ItemCountByStatusStatusCode::Modified,
                "NEW_ITEM" => ItemCountByStatusStatusCode::NewItem,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ItemCountByStatusStatusCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ItemCountByStatusStatusCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ItemCountByStatusStatusCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCEPTED" => ItemCountByStatusStatusCode::Accepted,
                "CODE_UNSPECIFIED" => ItemCountByStatusStatusCode::CodeUnspecified,
                "ERROR" => ItemCountByStatusStatusCode::Error,
                "MODIFIED" => ItemCountByStatusStatusCode::Modified,
                "NEW_ITEM" => ItemCountByStatusStatusCode::NewItem,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ItemCountByStatusStatusCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ItemCountByStatusStatusCode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ItemMetadata {
        #[doc = "The name of the container for this item. Deletion of the container item leads to automatic deletion of this item. Note: ACLs are not inherited from a container item. To provide ACL inheritance for an item, use the inheritAclFrom field. The maximum length is 1536 characters."]
        #[serde(
            rename = "containerName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub container_name: ::std::option::Option<String>,
        #[doc = "The BCP-47 language code for the item, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier. The maximum length is 32 characters."]
        #[serde(
            rename = "contentLanguage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_language: ::std::option::Option<String>,
        #[doc = "A set of named attributes associated with the item. This can be used for influencing the ranking of the item based on the context in the request. The maximum number of elements is 10."]
        #[serde(
            rename = "contextAttributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub context_attributes: ::std::option::Option<Vec<crate::schemas::ContextAttribute>>,
        #[doc = "The time when the item was created in the source repository."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Hashing value provided by the API caller. This can be used with the items.push method to calculate modified state. The maximum length is 2048 characters."]
        #[serde(
            rename = "hash",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hash: ::std::option::Option<String>,
        #[doc = "A list of interactions for the item. Interactions are used to improve Search quality, but are not exposed to end users. The maximum number of elements is 1000."]
        #[serde(
            rename = "interactions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub interactions: ::std::option::Option<Vec<crate::schemas::Interaction>>,
        #[doc = "Additional keywords or phrases that should match the item. Used internally for user generated content. The maximum number of elements is 100. The maximum length is 8192 characters."]
        #[serde(
            rename = "keywords",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keywords: ::std::option::Option<Vec<String>>,
        #[doc = "The original mime-type of ItemContent.content in the source repository. The maximum length is 256 characters."]
        #[serde(
            rename = "mimeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mime_type: ::std::option::Option<String>,
        #[doc = "The type of the item. This should correspond to the name of an object definition in the schema registered for the data source. For example, if the schema for the data source contains an object definition with name 'document', then item indexing requests for objects of that type should set objectType to 'document'. The maximum length is 256 characters."]
        #[serde(
            rename = "objectType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_type: ::std::option::Option<String>,
        #[doc = "Additional search quality metadata of the item"]
        #[serde(
            rename = "searchQualityMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_quality_metadata: ::std::option::Option<crate::schemas::SearchQualityMetadata>,
        #[doc = "Link to the source repository serving the data. Seach results apply this link to the title. Whitespace or special characters may cause Cloud Seach result links to trigger a redirect notice; to avoid this, encode the URL. The maximum length is 2048 characters."]
        #[serde(
            rename = "sourceRepositoryUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_repository_url: ::std::option::Option<String>,
        #[doc = "The title of the item. If given, this will be the displayed title of the Search result. The maximum length is 2048 characters."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "The time when the item was last modified in the source repository."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ItemMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ItemMetadata {
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
    pub struct ItemStatus {
        #[doc = "Status code."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<crate::schemas::ItemStatusCode>,
        #[doc = "Error details in case the item is in ERROR state."]
        #[serde(
            rename = "processingErrors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub processing_errors: ::std::option::Option<Vec<crate::schemas::ProcessingError>>,
        #[doc = "Repository error reported by connector."]
        #[serde(
            rename = "repositoryErrors",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub repository_errors: ::std::option::Option<Vec<crate::schemas::RepositoryError>>,
    }
    impl ::google_field_selector::FieldSelector for ItemStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ItemStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ItemStatusCode {
        #[doc = "API has accepted the up-to-date data of this item."]
        Accepted,
        #[doc = "Input-only value. Used with Items.list to list all items in the queue, regardless of status."]
        CodeUnspecified,
        #[doc = "Error encountered by Cloud Search while processing this item. Details of the error are in repositoryError."]
        Error,
        #[doc = "Item has been modified in the repository, and is out of date with the version previously accepted into Cloud Search."]
        Modified,
        #[doc = "Item is known to exist in the repository, but is not yet accepted by Cloud Search. An item can be in this state when Items.push has been called for an item of this name that did not exist previously."]
        NewItem,
    }
    impl ItemStatusCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ItemStatusCode::Accepted => "ACCEPTED",
                ItemStatusCode::CodeUnspecified => "CODE_UNSPECIFIED",
                ItemStatusCode::Error => "ERROR",
                ItemStatusCode::Modified => "MODIFIED",
                ItemStatusCode::NewItem => "NEW_ITEM",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ItemStatusCode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ItemStatusCode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ItemStatusCode, ()> {
            Ok(match s {
                "ACCEPTED" => ItemStatusCode::Accepted,
                "CODE_UNSPECIFIED" => ItemStatusCode::CodeUnspecified,
                "ERROR" => ItemStatusCode::Error,
                "MODIFIED" => ItemStatusCode::Modified,
                "NEW_ITEM" => ItemStatusCode::NewItem,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ItemStatusCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ItemStatusCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ItemStatusCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCEPTED" => ItemStatusCode::Accepted,
                "CODE_UNSPECIFIED" => ItemStatusCode::CodeUnspecified,
                "ERROR" => ItemStatusCode::Error,
                "MODIFIED" => ItemStatusCode::Modified,
                "NEW_ITEM" => ItemStatusCode::NewItem,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ItemStatusCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ItemStatusCode {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ItemStructuredData {
        #[doc = "Hashing value provided by the API caller. This can be used with the items.push method to calculate modified state. The maximum length is 2048 characters."]
        #[serde(
            rename = "hash",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hash: ::std::option::Option<String>,
        #[doc = "The structured data object that should conform to a registered object definition in the schema for the data source."]
        #[serde(
            rename = "object",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object: ::std::option::Option<crate::schemas::StructuredDataObject>,
    }
    impl ::google_field_selector::FieldSelector for ItemStructuredData {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ItemStructuredData {
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
    pub struct ListDataSourceResponse {
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[serde(
            rename = "sources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sources: ::std::option::Option<Vec<crate::schemas::DataSource>>,
    }
    impl ::google_field_selector::FieldSelector for ListDataSourceResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListDataSourceResponse {
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
    pub struct ListItemNamesForUnmappedIdentityResponse {
        #[serde(
            rename = "itemNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub item_names: ::std::option::Option<Vec<String>>,
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListItemNamesForUnmappedIdentityResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListItemNamesForUnmappedIdentityResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListItemsResponse {
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::Item>>,
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListItemsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListItemsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListOperationsResponse {
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
        pub operations: ::std::option::Option<Vec<crate::schemas::Operation>>,
    }
    impl ::google_field_selector::FieldSelector for ListOperationsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListOperationsResponse {
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
    pub struct ListQuerySourcesResponse {
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[serde(
            rename = "sources",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sources: ::std::option::Option<Vec<crate::schemas::QuerySource>>,
    }
    impl ::google_field_selector::FieldSelector for ListQuerySourcesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListQuerySourcesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListSearchApplicationsResponse {
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[serde(
            rename = "searchApplications",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_applications: ::std::option::Option<Vec<crate::schemas::SearchApplication>>,
    }
    impl ::google_field_selector::FieldSelector for ListSearchApplicationsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListSearchApplicationsResponse {
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
    pub struct ListUnmappedIdentitiesResponse {
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[serde(
            rename = "unmappedIdentities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unmapped_identities: ::std::option::Option<Vec<crate::schemas::UnmappedIdentity>>,
    }
    impl ::google_field_selector::FieldSelector for ListUnmappedIdentitiesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListUnmappedIdentitiesResponse {
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
    pub struct MatchRange {
        #[doc = "End of the match in the snippet."]
        #[serde(
            rename = "end",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end: ::std::option::Option<i32>,
        #[doc = "Starting position of the match in the snippet."]
        #[serde(
            rename = "start",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for MatchRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MatchRange {
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
    pub struct Media {
        #[doc = "Name of the media resource."]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Media {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Media {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Metadata {
        #[doc = "The creation time for this document or object in the search result."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Options that specify how to display a structured data search result."]
        #[serde(
            rename = "displayOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_options: ::std::option::Option<crate::schemas::ResultDisplayMetadata>,
        #[doc = "Indexed fields in structured data, returned as a generic named property."]
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<Vec<crate::schemas::NamedProperty>>,
        #[doc = "Mime type of the search result."]
        #[serde(
            rename = "mimeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mime_type: ::std::option::Option<String>,
        #[doc = "Object type of the search result."]
        #[serde(
            rename = "objectType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_type: ::std::option::Option<String>,
        #[doc = "Owner (usually creator) of the document or object of the search result."]
        #[serde(
            rename = "owner",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub owner: ::std::option::Option<crate::schemas::Person>,
        #[doc = "The named source for the result, such as Gmail."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<crate::schemas::Source>,
        #[doc = "The thumbnail URL of the result."]
        #[serde(
            rename = "thumbnailUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnail_url: ::std::option::Option<String>,
        #[doc = "The last modified date for the object in the search result. If not set in the item, the value returned here is empty. When `updateTime` is used for calculating freshness and is not set, this value defaults to 2 years from the current time."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Metadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Metadata {
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
    pub struct Metaline {
        #[doc = "The list of displayed properties for the metaline. The maximum number of properties is 5."]
        #[serde(
            rename = "properties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub properties: ::std::option::Option<Vec<crate::schemas::DisplayedProperty>>,
    }
    impl ::google_field_selector::FieldSelector for Metaline {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Metaline {
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
    pub struct Name {
        #[doc = "The read-only display name formatted according to the locale specified by the viewer's account or the Accept-Language HTTP header."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Name {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Name {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct NamedProperty {
        #[serde(
            rename = "booleanValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub boolean_value: ::std::option::Option<bool>,
        #[serde(
            rename = "dateValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_values: ::std::option::Option<crate::schemas::DateValues>,
        #[serde(
            rename = "doubleValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub double_values: ::std::option::Option<crate::schemas::DoubleValues>,
        #[serde(
            rename = "enumValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enum_values: ::std::option::Option<crate::schemas::EnumValues>,
        #[serde(
            rename = "htmlValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub html_values: ::std::option::Option<crate::schemas::HtmlValues>,
        #[serde(
            rename = "integerValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub integer_values: ::std::option::Option<crate::schemas::IntegerValues>,
        #[doc = "The name of the property. This name should correspond to the name of the property that was registered for object definition in the schema. The maximum allowable length for this property is 256 characters."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[serde(
            rename = "objectValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_values: ::std::option::Option<crate::schemas::ObjectValues>,
        #[serde(
            rename = "textValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_values: ::std::option::Option<crate::schemas::TextValues>,
        #[serde(
            rename = "timestampValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timestamp_values: ::std::option::Option<crate::schemas::TimestampValues>,
    }
    impl ::google_field_selector::FieldSelector for NamedProperty {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NamedProperty {
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
    pub struct ObjectDefinition {
        #[doc = "Name for the object, which then defines its type. Item indexing requests should set the objectType field equal to this value. For example, if *name* is *Document*, then indexing requests for items of type Document should set objectType equal to *Document*. Each object definition must be uniquely named within a schema. The name must start with a letter and can only contain letters (A-Z, a-z) or numbers (0-9). The maximum length is 256 characters."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The optional object-specific options."]
        #[serde(
            rename = "options",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub options: ::std::option::Option<crate::schemas::ObjectOptions>,
        #[doc = "The property definitions for the object. The maximum number of elements is 1000."]
        #[serde(
            rename = "propertyDefinitions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub property_definitions: ::std::option::Option<Vec<crate::schemas::PropertyDefinition>>,
    }
    impl ::google_field_selector::FieldSelector for ObjectDefinition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ObjectDefinition {
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
    pub struct ObjectDisplayOptions {
        #[doc = "Defines the properties that are displayed in the metalines of the search results. The property values are displayed in the order given here. If a property holds multiple values, all of the values are displayed before the next properties. For this reason, it is a good practice to specify singular properties before repeated properties in this list. All of the properties must set is_returnable to true. The maximum number of metalines is 3."]
        #[serde(
            rename = "metalines",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metalines: ::std::option::Option<Vec<crate::schemas::Metaline>>,
        #[doc = "The user friendly label to display in the search result to indicate the type of the item. This is OPTIONAL; if not provided, an object label isn't displayed on the context line of the search results. The maximum length is 64 characters."]
        #[serde(
            rename = "objectDisplayLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_display_label: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ObjectDisplayOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ObjectDisplayOptions {
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
    pub struct ObjectOptions {
        #[doc = "Options that determine how the object is displayed in the Cloud Search results page."]
        #[serde(
            rename = "displayOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_options: ::std::option::Option<crate::schemas::ObjectDisplayOptions>,
        #[doc = "The freshness options for an object."]
        #[serde(
            rename = "freshnessOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub freshness_options: ::std::option::Option<crate::schemas::FreshnessOptions>,
    }
    impl ::google_field_selector::FieldSelector for ObjectOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ObjectOptions {
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
    pub struct ObjectPropertyOptions {
        #[doc = "The properties of the sub-object. These properties represent a nested object. For example, if this property represents a postal address, the subobjectProperties might be named *street*, *city*, and *state*. The maximum number of elements is 1000."]
        #[serde(
            rename = "subobjectProperties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subobject_properties: ::std::option::Option<Vec<crate::schemas::PropertyDefinition>>,
    }
    impl ::google_field_selector::FieldSelector for ObjectPropertyOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ObjectPropertyOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ObjectValues {
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<crate::schemas::StructuredDataObject>>,
    }
    impl ::google_field_selector::FieldSelector for ObjectValues {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ObjectValues {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Operation {
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
        pub error: ::std::option::Option<crate::schemas::Status>,
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
    impl ::google_field_selector::FieldSelector for Operation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Operation {
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
    pub struct PeopleSuggestion {
        #[doc = "Suggested person. All fields of the person object might not be populated."]
        #[serde(
            rename = "person",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub person: ::std::option::Option<crate::schemas::Person>,
    }
    impl ::google_field_selector::FieldSelector for PeopleSuggestion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PeopleSuggestion {
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
    pub struct Person {
        #[doc = "The person's email addresses"]
        #[serde(
            rename = "emailAddresses",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email_addresses: ::std::option::Option<Vec<crate::schemas::EmailAddress>>,
        #[doc = "The resource name of the person to provide information about. See People.get from Google People API."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Obfuscated ID of a person."]
        #[serde(
            rename = "obfuscatedId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub obfuscated_id: ::std::option::Option<String>,
        #[doc = "The person's name"]
        #[serde(
            rename = "personNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub person_names: ::std::option::Option<Vec<crate::schemas::Name>>,
        #[doc = "A person's read-only photo. A picture shown next to the person's name to help others recognize the person in search results."]
        #[serde(
            rename = "photos",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub photos: ::std::option::Option<Vec<crate::schemas::Photo>>,
    }
    impl ::google_field_selector::FieldSelector for Person {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Person {
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
    pub struct Photo {
        #[doc = "The URL of the photo."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Photo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Photo {
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
    pub struct PollItemsRequest {
        #[doc = "Name of connector making this call. Format: datasources/{source_id}/connectors/{ID}"]
        #[serde(
            rename = "connectorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub connector_name: ::std::option::Option<String>,
        #[doc = "Common debug options."]
        #[serde(
            rename = "debugOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub debug_options: ::std::option::Option<crate::schemas::DebugOptions>,
        #[doc = "Maximum number of items to return. The maximum value is 100 and the default value is 20."]
        #[serde(
            rename = "limit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub limit: ::std::option::Option<i32>,
        #[doc = "Queue name to fetch items from. If unspecified, PollItems will fetch from 'default' queue. The maximum length is 100 characters."]
        #[serde(
            rename = "queue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub queue: ::std::option::Option<String>,
        #[doc = "Limit the items polled to the ones with these statuses."]
        #[serde(
            rename = "statusCodes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status_codes:
            ::std::option::Option<Vec<crate::schemas::PollItemsRequestStatusCodesItems>>,
    }
    impl ::google_field_selector::FieldSelector for PollItemsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PollItemsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PollItemsRequestStatusCodesItems {
        #[doc = "API has accepted the up-to-date data of this item."]
        Accepted,
        #[doc = "Input-only value. Used with Items.list to list all items in the queue, regardless of status."]
        CodeUnspecified,
        #[doc = "Error encountered by Cloud Search while processing this item. Details of the error are in repositoryError."]
        Error,
        #[doc = "Item has been modified in the repository, and is out of date with the version previously accepted into Cloud Search."]
        Modified,
        #[doc = "Item is known to exist in the repository, but is not yet accepted by Cloud Search. An item can be in this state when Items.push has been called for an item of this name that did not exist previously."]
        NewItem,
    }
    impl PollItemsRequestStatusCodesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                PollItemsRequestStatusCodesItems::Accepted => "ACCEPTED",
                PollItemsRequestStatusCodesItems::CodeUnspecified => "CODE_UNSPECIFIED",
                PollItemsRequestStatusCodesItems::Error => "ERROR",
                PollItemsRequestStatusCodesItems::Modified => "MODIFIED",
                PollItemsRequestStatusCodesItems::NewItem => "NEW_ITEM",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PollItemsRequestStatusCodesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PollItemsRequestStatusCodesItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PollItemsRequestStatusCodesItems, ()> {
            Ok(match s {
                "ACCEPTED" => PollItemsRequestStatusCodesItems::Accepted,
                "CODE_UNSPECIFIED" => PollItemsRequestStatusCodesItems::CodeUnspecified,
                "ERROR" => PollItemsRequestStatusCodesItems::Error,
                "MODIFIED" => PollItemsRequestStatusCodesItems::Modified,
                "NEW_ITEM" => PollItemsRequestStatusCodesItems::NewItem,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PollItemsRequestStatusCodesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PollItemsRequestStatusCodesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PollItemsRequestStatusCodesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCEPTED" => PollItemsRequestStatusCodesItems::Accepted,
                "CODE_UNSPECIFIED" => PollItemsRequestStatusCodesItems::CodeUnspecified,
                "ERROR" => PollItemsRequestStatusCodesItems::Error,
                "MODIFIED" => PollItemsRequestStatusCodesItems::Modified,
                "NEW_ITEM" => PollItemsRequestStatusCodesItems::NewItem,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PollItemsRequestStatusCodesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PollItemsRequestStatusCodesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PollItemsResponse {
        #[doc = "Set of items from the queue available for connector to process. These items have the following subset of fields populated: version metadata.hash structured_data.hash content.hash payload status queue"]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::Item>>,
    }
    impl ::google_field_selector::FieldSelector for PollItemsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PollItemsResponse {
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
    pub struct Principal {
        #[doc = "This principal is a group identified using an external identity. The name field must specify the group resource name with this format: identitysources/{source_id}/groups/{ID}"]
        #[serde(
            rename = "groupResourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group_resource_name: ::std::option::Option<String>,
        #[doc = "This principal is a GSuite user, group or domain."]
        #[serde(
            rename = "gsuitePrincipal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gsuite_principal: ::std::option::Option<crate::schemas::GsuitePrincipal>,
        #[doc = "This principal is a user identified using an external identity. The name field must specify the user resource name with this format: identitysources/{source_id}/users/{ID}"]
        #[serde(
            rename = "userResourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Principal {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Principal {
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
    pub struct ProcessingError {
        #[doc = "Error code indicating the nature of the error."]
        #[serde(
            rename = "code",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub code: ::std::option::Option<crate::schemas::ProcessingErrorCode>,
        #[doc = "Description of the error."]
        #[serde(
            rename = "errorMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_message: ::std::option::Option<String>,
        #[doc = "In case the item fields are invalid, this field contains the details about the validation errors."]
        #[serde(
            rename = "fieldViolations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field_violations: ::std::option::Option<Vec<crate::schemas::FieldViolation>>,
    }
    impl ::google_field_selector::FieldSelector for ProcessingError {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ProcessingError {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ProcessingErrorCode {
        #[doc = "ACL inheritance graph formed a cycle."]
        AclCycle,
        #[doc = "Items with incomplete ACL information due to inheriting other items with broken ACL or having groups with unmapped descendants."]
        IndirectBrokenAcl,
        #[doc = "Item's ACL, metadata, or content is malformed or in invalid state. FieldViolations contains more details on where the problem is."]
        MalformedRequest,
        #[doc = "Input only value. Use this value in Items."]
        ProcessingErrorCodeUnspecified,
        #[doc = "Countent format is unsupported."]
        UnsupportedContentFormat,
    }
    impl ProcessingErrorCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ProcessingErrorCode::AclCycle => "ACL_CYCLE",
                ProcessingErrorCode::IndirectBrokenAcl => "INDIRECT_BROKEN_ACL",
                ProcessingErrorCode::MalformedRequest => "MALFORMED_REQUEST",
                ProcessingErrorCode::ProcessingErrorCodeUnspecified => {
                    "PROCESSING_ERROR_CODE_UNSPECIFIED"
                }
                ProcessingErrorCode::UnsupportedContentFormat => "UNSUPPORTED_CONTENT_FORMAT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ProcessingErrorCode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ProcessingErrorCode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ProcessingErrorCode, ()> {
            Ok(match s {
                "ACL_CYCLE" => ProcessingErrorCode::AclCycle,
                "INDIRECT_BROKEN_ACL" => ProcessingErrorCode::IndirectBrokenAcl,
                "MALFORMED_REQUEST" => ProcessingErrorCode::MalformedRequest,
                "PROCESSING_ERROR_CODE_UNSPECIFIED" => {
                    ProcessingErrorCode::ProcessingErrorCodeUnspecified
                }
                "UNSUPPORTED_CONTENT_FORMAT" => ProcessingErrorCode::UnsupportedContentFormat,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ProcessingErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ProcessingErrorCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ProcessingErrorCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACL_CYCLE" => ProcessingErrorCode::AclCycle,
                "INDIRECT_BROKEN_ACL" => ProcessingErrorCode::IndirectBrokenAcl,
                "MALFORMED_REQUEST" => ProcessingErrorCode::MalformedRequest,
                "PROCESSING_ERROR_CODE_UNSPECIFIED" => {
                    ProcessingErrorCode::ProcessingErrorCodeUnspecified
                }
                "UNSUPPORTED_CONTENT_FORMAT" => ProcessingErrorCode::UnsupportedContentFormat,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ProcessingErrorCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ProcessingErrorCode {
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
    pub struct PropertyDefinition {
        #[serde(
            rename = "booleanPropertyOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub boolean_property_options: ::std::option::Option<crate::schemas::BooleanPropertyOptions>,
        #[serde(
            rename = "datePropertyOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_property_options: ::std::option::Option<crate::schemas::DatePropertyOptions>,
        #[doc = "Options that determine how the property is displayed in the Cloud Search results page if it is specified to be displayed in the object's display options ."]
        #[serde(
            rename = "displayOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_options: ::std::option::Option<crate::schemas::PropertyDisplayOptions>,
        #[serde(
            rename = "doublePropertyOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub double_property_options: ::std::option::Option<crate::schemas::DoublePropertyOptions>,
        #[serde(
            rename = "enumPropertyOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enum_property_options: ::std::option::Option<crate::schemas::EnumPropertyOptions>,
        #[serde(
            rename = "htmlPropertyOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub html_property_options: ::std::option::Option<crate::schemas::HtmlPropertyOptions>,
        #[serde(
            rename = "integerPropertyOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub integer_property_options: ::std::option::Option<crate::schemas::IntegerPropertyOptions>,
        #[doc = "Indicates that the property can be used for generating facets. Cannot be true for properties whose type is object. IsReturnable must be true to set this option. Only supported for Boolean, Enum, and Text properties."]
        #[serde(
            rename = "isFacetable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_facetable: ::std::option::Option<bool>,
        #[doc = "Indicates that multiple values are allowed for the property. For example, a document only has one description but can have multiple comments. Cannot be true for properties whose type is a boolean. If set to false, properties that contain more than one value cause the indexing request for that item to be rejected."]
        #[serde(
            rename = "isRepeatable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_repeatable: ::std::option::Option<bool>,
        #[doc = "Indicates that the property identifies data that should be returned in search results via the Query API. If set to *true*, indicates that Query API users can use matching property fields in results. However, storing fields requires more space allocation and uses more bandwidth for search queries, which impacts performance over large datasets. Set to *true* here only if the field is needed for search results. Cannot be true for properties whose type is an object."]
        #[serde(
            rename = "isReturnable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_returnable: ::std::option::Option<bool>,
        #[doc = "Indicates that the property can be used for sorting. Cannot be true for properties that are repeatable. Cannot be true for properties whose type is object. IsReturnable must be true to set this option. Only supported for Boolean, Date, Double, Integer, and Timestamp properties."]
        #[serde(
            rename = "isSortable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_sortable: ::std::option::Option<bool>,
        #[doc = "Indicates that the property can be used for generating query suggestions."]
        #[serde(
            rename = "isSuggestable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_suggestable: ::std::option::Option<bool>,
        #[doc = "Indicates that users can perform wildcard search for this property. Only supported for Text properties. IsReturnable must be true to set this option. In a given datasource maximum of 5 properties can be marked as is_wildcard_searchable."]
        #[serde(
            rename = "isWildcardSearchable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_wildcard_searchable: ::std::option::Option<bool>,
        #[doc = "The name of the property. Item indexing requests sent to the Indexing API should set the property name equal to this value. For example, if name is *subject_line*, then indexing requests for document items with subject fields should set the name for that field equal to *subject_line*. Use the name as the identifier for the object property. Once registered as a property for an object, you cannot re-use this name for another property within that object. The name must start with a letter and can only contain letters (A-Z, a-z) or numbers (0-9). The maximum length is 256 characters."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[serde(
            rename = "objectPropertyOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_property_options: ::std::option::Option<crate::schemas::ObjectPropertyOptions>,
        #[serde(
            rename = "textPropertyOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text_property_options: ::std::option::Option<crate::schemas::TextPropertyOptions>,
        #[serde(
            rename = "timestampPropertyOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timestamp_property_options:
            ::std::option::Option<crate::schemas::TimestampPropertyOptions>,
    }
    impl ::google_field_selector::FieldSelector for PropertyDefinition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PropertyDefinition {
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
    pub struct PropertyDisplayOptions {
        #[doc = "The user friendly label for the property that is used if the property is specified to be displayed in ObjectDisplayOptions. If provided, the display label is shown in front of the property values when the property is part of the object display options. For example, if the property value is '1', the value by itself may not be useful context for the user. If the display name given was 'priority', then the user sees 'priority : 1' in the search results which provides clear context to search users. This is OPTIONAL; if not given, only the property values are displayed. The maximum length is 64 characters."]
        #[serde(
            rename = "displayLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_label: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PropertyDisplayOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PropertyDisplayOptions {
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
    pub struct PushItem {
        #[doc = "Content hash of the item according to the repository. If specified, this is used to determine how to modify this item's status. Setting this field and the type field results in argument error. The maximum length is 2048 characters."]
        #[serde(
            rename = "contentHash",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_hash: ::std::option::Option<String>,
        #[doc = "Metadata hash of the item according to the repository. If specified, this is used to determine how to modify this item's status. Setting this field and the type field results in argument error. The maximum length is 2048 characters."]
        #[serde(
            rename = "metadataHash",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata_hash: ::std::option::Option<String>,
        #[doc = "Provides additional document state information for the connector, such as an alternate repository ID and other metadata. The maximum length is 8192 bytes."]
        #[serde(
            rename = "payload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub payload: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Queue to which this item belongs to. The default queue is chosen if this field is not specified. The maximum length is 512 characters."]
        #[serde(
            rename = "queue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub queue: ::std::option::Option<String>,
        #[doc = "The type of the push operation that defines the push behavior."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::PushItemType>,
        #[doc = "Populate this field to store Connector or repository error details. This information is displayed in the Admin Console. This field may only be populated when the Type is REPOSITORY_ERROR."]
        #[serde(
            rename = "repositoryError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub repository_error: ::std::option::Option<crate::schemas::RepositoryError>,
        #[doc = "Structured data hash of the item according to the repository. If specified, this is used to determine how to modify this item's status. Setting this field and the type field results in argument error. The maximum length is 2048 characters."]
        #[serde(
            rename = "structuredDataHash",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub structured_data_hash: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PushItem {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PushItem {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PushItemType {
        #[doc = "Indicates that the repository document has been modified or updated since the previous update call. This changes status to MODIFIED state for an existing item. If this is called on a non existing item, the status is changed to NEW_ITEM."]
        Modified,
        #[doc = "Item in the repository has not been modified since the last update call. This push operation will set status to ACCEPTED state."]
        NotModified,
        #[doc = "Connector is facing a repository error regarding this item. Change status to REPOSITORY_ERROR state. Item is unreserved and rescheduled at a future time determined by exponential backoff."]
        RepositoryError,
        #[doc = "Call push with REQUEUE only for items that have been reserved. This action unreserves the item and resets its available time to the wall clock time."]
        Requeue,
        #[doc = "Default UNSPECIFIED. Specifies that the push operation should not modify ItemStatus"]
        Unspecified,
    }
    impl PushItemType {
        pub fn as_str(self) -> &'static str {
            match self {
                PushItemType::Modified => "MODIFIED",
                PushItemType::NotModified => "NOT_MODIFIED",
                PushItemType::RepositoryError => "REPOSITORY_ERROR",
                PushItemType::Requeue => "REQUEUE",
                PushItemType::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PushItemType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PushItemType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PushItemType, ()> {
            Ok(match s {
                "MODIFIED" => PushItemType::Modified,
                "NOT_MODIFIED" => PushItemType::NotModified,
                "REPOSITORY_ERROR" => PushItemType::RepositoryError,
                "REQUEUE" => PushItemType::Requeue,
                "UNSPECIFIED" => PushItemType::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PushItemType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PushItemType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PushItemType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MODIFIED" => PushItemType::Modified,
                "NOT_MODIFIED" => PushItemType::NotModified,
                "REPOSITORY_ERROR" => PushItemType::RepositoryError,
                "REQUEUE" => PushItemType::Requeue,
                "UNSPECIFIED" => PushItemType::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PushItemType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PushItemType {
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
    pub struct PushItemRequest {
        #[doc = "Name of connector making this call. Format: datasources/{source_id}/connectors/{ID}"]
        #[serde(
            rename = "connectorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub connector_name: ::std::option::Option<String>,
        #[doc = "Common debug options."]
        #[serde(
            rename = "debugOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub debug_options: ::std::option::Option<crate::schemas::DebugOptions>,
        #[doc = "Item to push onto the queue."]
        #[serde(
            rename = "item",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub item: ::std::option::Option<crate::schemas::PushItem>,
    }
    impl ::google_field_selector::FieldSelector for PushItemRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PushItemRequest {
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
    pub struct QueryCountByStatus {
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub count: ::std::option::Option<i64>,
        #[doc = "This represents the http status code."]
        #[serde(
            rename = "statusCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status_code: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for QueryCountByStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryCountByStatus {
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
    pub struct QueryInterpretation {
        #[serde(
            rename = "interpretationType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub interpretation_type:
            ::std::option::Option<crate::schemas::QueryInterpretationInterpretationType>,
        #[doc = "The interpretation of the query used in search. For example, queries with natural language intent like \"email from john\" will be interpreted as \"from:john source:mail\". This field will not be filled when the reason is NOT_ENOUGH_RESULTS_FOUND_FOR_USER_QUERY."]
        #[serde(
            rename = "interpretedQuery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub interpreted_query: ::std::option::Option<String>,
        #[doc = "The reason for interpretation of the query. This field will not be UNSPECIFIED if the interpretation type is not NONE."]
        #[serde(
            rename = "reason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reason: ::std::option::Option<crate::schemas::QueryInterpretationReason>,
    }
    impl ::google_field_selector::FieldSelector for QueryInterpretation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryInterpretation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum QueryInterpretationInterpretationType {
        #[doc = "The results from original query are blended with other results. The reason for blending these other results with the results from original query is populated in the 'Reason' field below."]
        Blend,
        #[doc = "Neither the natural language interpretation, nor a broader version of the query is used to fetch the search results."]
        None,
        #[doc = "The results from original query are replaced. The reason for replacing the results from original query is populated in the 'Reason' field below."]
        Replace,
    }
    impl QueryInterpretationInterpretationType {
        pub fn as_str(self) -> &'static str {
            match self {
                QueryInterpretationInterpretationType::Blend => "BLEND",
                QueryInterpretationInterpretationType::None => "NONE",
                QueryInterpretationInterpretationType::Replace => "REPLACE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for QueryInterpretationInterpretationType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for QueryInterpretationInterpretationType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<QueryInterpretationInterpretationType, ()> {
            Ok(match s {
                "BLEND" => QueryInterpretationInterpretationType::Blend,
                "NONE" => QueryInterpretationInterpretationType::None,
                "REPLACE" => QueryInterpretationInterpretationType::Replace,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for QueryInterpretationInterpretationType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for QueryInterpretationInterpretationType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for QueryInterpretationInterpretationType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BLEND" => QueryInterpretationInterpretationType::Blend,
                "NONE" => QueryInterpretationInterpretationType::None,
                "REPLACE" => QueryInterpretationInterpretationType::Replace,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for QueryInterpretationInterpretationType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryInterpretationInterpretationType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum QueryInterpretationReason {
        #[doc = "Query and document terms similarity is used to selectively broaden the query to retrieve additional search results since enough results were not found for the user query. Interpreted query will be empty for this case."]
        NotEnoughResultsFoundForUserQuery,
        #[doc = "Natural language interpretation of the query is used to fetch the search results."]
        QueryHasNaturalLanguageIntent,
        Unspecified,
    }
    impl QueryInterpretationReason {
        pub fn as_str(self) -> &'static str {
            match self {
                QueryInterpretationReason::NotEnoughResultsFoundForUserQuery => {
                    "NOT_ENOUGH_RESULTS_FOUND_FOR_USER_QUERY"
                }
                QueryInterpretationReason::QueryHasNaturalLanguageIntent => {
                    "QUERY_HAS_NATURAL_LANGUAGE_INTENT"
                }
                QueryInterpretationReason::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for QueryInterpretationReason {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for QueryInterpretationReason {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<QueryInterpretationReason, ()> {
            Ok(match s {
                "NOT_ENOUGH_RESULTS_FOUND_FOR_USER_QUERY" => {
                    QueryInterpretationReason::NotEnoughResultsFoundForUserQuery
                }
                "QUERY_HAS_NATURAL_LANGUAGE_INTENT" => {
                    QueryInterpretationReason::QueryHasNaturalLanguageIntent
                }
                "UNSPECIFIED" => QueryInterpretationReason::Unspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for QueryInterpretationReason {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for QueryInterpretationReason {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for QueryInterpretationReason {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NOT_ENOUGH_RESULTS_FOUND_FOR_USER_QUERY" => {
                    QueryInterpretationReason::NotEnoughResultsFoundForUserQuery
                }
                "QUERY_HAS_NATURAL_LANGUAGE_INTENT" => {
                    QueryInterpretationReason::QueryHasNaturalLanguageIntent
                }
                "UNSPECIFIED" => QueryInterpretationReason::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for QueryInterpretationReason {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryInterpretationReason {
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
    pub struct QueryInterpretationConfig {
        #[doc = "Set this flag to disable supplemental results retrieval, setting a flag here will not retrieve supplemental results for queries associated with a given search application. If this flag is set to True, it will take precedence over the option set at Query level. For the default value of False, query level flag will set the correct interpretation for supplemental results."]
        #[serde(
            rename = "forceDisableSupplementalResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub force_disable_supplemental_results: ::std::option::Option<bool>,
        #[doc = "Enable this flag to turn off all internal optimizations like natural language (NL) interpretation of queries, supplemental results retrieval, and usage of synonyms including custom ones. If this flag is set to True, it will take precedence over the option set at Query level. For the default value of False, query level flag will set the correct interpretation for verbatim mode."]
        #[serde(
            rename = "forceVerbatimMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub force_verbatim_mode: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for QueryInterpretationConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryInterpretationConfig {
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
    pub struct QueryInterpretationOptions {
        #[doc = "Flag to disable natural language (NL) interpretation of queries. Default is false, Set to true to disable natural language interpretation. NL interpretation only applies to predefined datasources."]
        #[serde(
            rename = "disableNlInterpretation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_nl_interpretation: ::std::option::Option<bool>,
        #[doc = "Use this flag to disable supplemental results for a query. Supplemental results setting chosen at SearchApplication level will take precedence if set to True."]
        #[serde(
            rename = "disableSupplementalResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_supplemental_results: ::std::option::Option<bool>,
        #[doc = "Enable this flag to turn off all internal optimizations like natural language (NL) interpretation of queries, supplemental result retrieval, and usage of synonyms including custom ones. Nl interpretation will be disabled if either one of the two flags is true."]
        #[serde(
            rename = "enableVerbatimMode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_verbatim_mode: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for QueryInterpretationOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryInterpretationOptions {
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
    pub struct QueryItem {
        #[doc = "True if the text was generated by means other than a previous user search."]
        #[serde(
            rename = "isSynthetic",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_synthetic: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for QueryItem {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryItem {
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
    pub struct QueryOperator {
        #[doc = "Display name of the operator"]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Potential list of values for the opeatror field. This field is only filled when we can safely enumerate all the possible values of this operator."]
        #[serde(
            rename = "enumValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enum_values: ::std::option::Option<Vec<String>>,
        #[doc = "Indicates the operator name that can be used to isolate the property using the greater-than operator."]
        #[serde(
            rename = "greaterThanOperatorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub greater_than_operator_name: ::std::option::Option<String>,
        #[doc = "Can this operator be used to get facets."]
        #[serde(
            rename = "isFacetable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_facetable: ::std::option::Option<bool>,
        #[doc = "Indicates if multiple values can be set for this property."]
        #[serde(
            rename = "isRepeatable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_repeatable: ::std::option::Option<bool>,
        #[doc = "Will the property associated with this facet be returned as part of search results."]
        #[serde(
            rename = "isReturnable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_returnable: ::std::option::Option<bool>,
        #[doc = "Can this operator be used to sort results."]
        #[serde(
            rename = "isSortable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_sortable: ::std::option::Option<bool>,
        #[doc = "Can get suggestions for this field."]
        #[serde(
            rename = "isSuggestable",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_suggestable: ::std::option::Option<bool>,
        #[doc = "Indicates the operator name that can be used to isolate the property using the less-than operator."]
        #[serde(
            rename = "lessThanOperatorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub less_than_operator_name: ::std::option::Option<String>,
        #[doc = "Name of the object corresponding to the operator. This field is only filled for schema-specific operators, and is unset for common operators."]
        #[serde(
            rename = "objectType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_type: ::std::option::Option<String>,
        #[doc = "The name of the operator."]
        #[serde(
            rename = "operatorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator_name: ::std::option::Option<String>,
        #[doc = "Type of the operator."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::QueryOperatorType>,
    }
    impl ::google_field_selector::FieldSelector for QueryOperator {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryOperator {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum QueryOperatorType {
        Boolean,
        Date,
        Double,
        Enum,
        Html,
        Integer,
        Text,
        Timestamp,
        #[doc = "Invalid value."]
        Unknown,
    }
    impl QueryOperatorType {
        pub fn as_str(self) -> &'static str {
            match self {
                QueryOperatorType::Boolean => "BOOLEAN",
                QueryOperatorType::Date => "DATE",
                QueryOperatorType::Double => "DOUBLE",
                QueryOperatorType::Enum => "ENUM",
                QueryOperatorType::Html => "HTML",
                QueryOperatorType::Integer => "INTEGER",
                QueryOperatorType::Text => "TEXT",
                QueryOperatorType::Timestamp => "TIMESTAMP",
                QueryOperatorType::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for QueryOperatorType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for QueryOperatorType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<QueryOperatorType, ()> {
            Ok(match s {
                "BOOLEAN" => QueryOperatorType::Boolean,
                "DATE" => QueryOperatorType::Date,
                "DOUBLE" => QueryOperatorType::Double,
                "ENUM" => QueryOperatorType::Enum,
                "HTML" => QueryOperatorType::Html,
                "INTEGER" => QueryOperatorType::Integer,
                "TEXT" => QueryOperatorType::Text,
                "TIMESTAMP" => QueryOperatorType::Timestamp,
                "UNKNOWN" => QueryOperatorType::Unknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for QueryOperatorType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for QueryOperatorType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for QueryOperatorType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOOLEAN" => QueryOperatorType::Boolean,
                "DATE" => QueryOperatorType::Date,
                "DOUBLE" => QueryOperatorType::Double,
                "ENUM" => QueryOperatorType::Enum,
                "HTML" => QueryOperatorType::Html,
                "INTEGER" => QueryOperatorType::Integer,
                "TEXT" => QueryOperatorType::Text,
                "TIMESTAMP" => QueryOperatorType::Timestamp,
                "UNKNOWN" => QueryOperatorType::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for QueryOperatorType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryOperatorType {
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
    pub struct QuerySource {
        #[doc = "Display name of the data source."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "List of all operators applicable for this source."]
        #[serde(
            rename = "operators",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operators: ::std::option::Option<Vec<crate::schemas::QueryOperator>>,
        #[doc = "A short name or alias for the source. This value can be used with the 'source' operator."]
        #[serde(
            rename = "shortName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub short_name: ::std::option::Option<String>,
        #[doc = "Name of the source"]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<crate::schemas::Source>,
    }
    impl ::google_field_selector::FieldSelector for QuerySource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QuerySource {
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
    pub struct QuerySuggestion {}
    impl ::google_field_selector::FieldSelector for QuerySuggestion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QuerySuggestion {
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
    pub struct RepositoryError {
        #[doc = "Message that describes the error. The maximum allowable length of the message is 8192 characters."]
        #[serde(
            rename = "errorMessage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_message: ::std::option::Option<String>,
        #[doc = "Error codes. Matches the definition of HTTP status codes."]
        #[serde(
            rename = "httpStatusCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub http_status_code: ::std::option::Option<i32>,
        #[doc = "Type of error."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::RepositoryErrorType>,
    }
    impl ::google_field_selector::FieldSelector for RepositoryError {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RepositoryError {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RepositoryErrorType {
        #[doc = "Failed authentication due to incorrect credentials."]
        AuthenticationError,
        #[doc = "Service account is not authorized for the repository."]
        AuthorizationError,
        #[doc = "Client-related error, such as an invalid request from the connector to the repository server."]
        ClientError,
        #[doc = "Cannot connect to the repository server."]
        ConnectionError,
        #[doc = "DNS problem, such as the DNS server is not responding."]
        DnsError,
        #[doc = "Unknown or unreachable host."]
        NetworkError,
        #[doc = "Quota exceeded."]
        QuotaExceeded,
        #[doc = "Repository server error."]
        ServerError,
        #[doc = "Server temporarily unavailable."]
        ServiceUnavailable,
        #[doc = "Unknown error."]
        Unknown,
    }
    impl RepositoryErrorType {
        pub fn as_str(self) -> &'static str {
            match self {
                RepositoryErrorType::AuthenticationError => "AUTHENTICATION_ERROR",
                RepositoryErrorType::AuthorizationError => "AUTHORIZATION_ERROR",
                RepositoryErrorType::ClientError => "CLIENT_ERROR",
                RepositoryErrorType::ConnectionError => "CONNECTION_ERROR",
                RepositoryErrorType::DnsError => "DNS_ERROR",
                RepositoryErrorType::NetworkError => "NETWORK_ERROR",
                RepositoryErrorType::QuotaExceeded => "QUOTA_EXCEEDED",
                RepositoryErrorType::ServerError => "SERVER_ERROR",
                RepositoryErrorType::ServiceUnavailable => "SERVICE_UNAVAILABLE",
                RepositoryErrorType::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for RepositoryErrorType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RepositoryErrorType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<RepositoryErrorType, ()> {
            Ok(match s {
                "AUTHENTICATION_ERROR" => RepositoryErrorType::AuthenticationError,
                "AUTHORIZATION_ERROR" => RepositoryErrorType::AuthorizationError,
                "CLIENT_ERROR" => RepositoryErrorType::ClientError,
                "CONNECTION_ERROR" => RepositoryErrorType::ConnectionError,
                "DNS_ERROR" => RepositoryErrorType::DnsError,
                "NETWORK_ERROR" => RepositoryErrorType::NetworkError,
                "QUOTA_EXCEEDED" => RepositoryErrorType::QuotaExceeded,
                "SERVER_ERROR" => RepositoryErrorType::ServerError,
                "SERVICE_UNAVAILABLE" => RepositoryErrorType::ServiceUnavailable,
                "UNKNOWN" => RepositoryErrorType::Unknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for RepositoryErrorType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RepositoryErrorType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RepositoryErrorType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AUTHENTICATION_ERROR" => RepositoryErrorType::AuthenticationError,
                "AUTHORIZATION_ERROR" => RepositoryErrorType::AuthorizationError,
                "CLIENT_ERROR" => RepositoryErrorType::ClientError,
                "CONNECTION_ERROR" => RepositoryErrorType::ConnectionError,
                "DNS_ERROR" => RepositoryErrorType::DnsError,
                "NETWORK_ERROR" => RepositoryErrorType::NetworkError,
                "QUOTA_EXCEEDED" => RepositoryErrorType::QuotaExceeded,
                "SERVER_ERROR" => RepositoryErrorType::ServerError,
                "SERVICE_UNAVAILABLE" => RepositoryErrorType::ServiceUnavailable,
                "UNKNOWN" => RepositoryErrorType::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RepositoryErrorType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RepositoryErrorType {
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
    pub struct RequestOptions {
        #[doc = "Debug options of the request"]
        #[serde(
            rename = "debugOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub debug_options: ::std::option::Option<crate::schemas::DebugOptions>,
        #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier. For translations. Set this field using the language set in browser or for the page. In the event that the user's language preference is known, set this field to the known user language. When specified, the documents in search results are biased towards the specified language. The suggest API does not use this parameter. Instead, suggest autocompletes only based on characters in the query."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
        #[doc = "The ID generated when you create a search application using the [admin console](https://support.google.com/a/answer/9043922)."]
        #[serde(
            rename = "searchApplicationId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_application_id: ::std::option::Option<String>,
        #[doc = "Current user's time zone id, such as \"America/Los_Angeles\" or \"Australia/Sydney\". These IDs are defined by [Unicode Common Locale Data Repository (CLDR)](http://cldr.unicode.org/) project, and currently available in the file [timezone.xml](http://unicode.org/repos/cldr/trunk/common/bcp47/timezone.xml). This field is used to correctly interpret date and time queries. If this field is not specified, the default time zone (UTC) is used."]
        #[serde(
            rename = "timeZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_zone: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RequestOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RequestOptions {
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
    pub struct ResetSearchApplicationRequest {
        #[doc = "Common debug options."]
        #[serde(
            rename = "debugOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub debug_options: ::std::option::Option<crate::schemas::DebugOptions>,
    }
    impl ::google_field_selector::FieldSelector for ResetSearchApplicationRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResetSearchApplicationRequest {
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
    pub struct ResponseDebugInfo {
        #[doc = "General debug info formatted for display."]
        #[serde(
            rename = "formattedDebugInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_debug_info: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ResponseDebugInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResponseDebugInfo {
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
    pub struct RestrictItem {
        #[serde(
            rename = "driveFollowUpRestrict",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub drive_follow_up_restrict: ::std::option::Option<crate::schemas::DriveFollowUpRestrict>,
        #[serde(
            rename = "driveLocationRestrict",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub drive_location_restrict: ::std::option::Option<crate::schemas::DriveLocationRestrict>,
        #[doc = "Drive Types."]
        #[serde(
            rename = "driveMimeTypeRestrict",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub drive_mime_type_restrict: ::std::option::Option<crate::schemas::DriveMimeTypeRestrict>,
        #[serde(
            rename = "driveTimeSpanRestrict",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub drive_time_span_restrict: ::std::option::Option<crate::schemas::DriveTimeSpanRestrict>,
        #[doc = "The search restrict (e.g. \"after:2017-09-11 before:2017-09-12\")."]
        #[serde(
            rename = "searchOperator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_operator: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RestrictItem {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RestrictItem {
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
    pub struct ResultCounts {
        #[doc = "Result count information for each source with results."]
        #[serde(
            rename = "sourceResultCounts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_result_counts: ::std::option::Option<Vec<crate::schemas::SourceResultCount>>,
    }
    impl ::google_field_selector::FieldSelector for ResultCounts {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResultCounts {
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
    pub struct ResultDebugInfo {
        #[doc = "General debug info formatted for display."]
        #[serde(
            rename = "formattedDebugInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_debug_info: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ResultDebugInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResultDebugInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ResultDisplayField {
        #[doc = "The display label for the property."]
        #[serde(
            rename = "label",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label: ::std::option::Option<String>,
        #[doc = "The operator name of the property."]
        #[serde(
            rename = "operatorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator_name: ::std::option::Option<String>,
        #[doc = "The name value pair for the property."]
        #[serde(
            rename = "property",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub property: ::std::option::Option<crate::schemas::NamedProperty>,
    }
    impl ::google_field_selector::FieldSelector for ResultDisplayField {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResultDisplayField {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ResultDisplayLine {
        #[serde(
            rename = "fields",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fields: ::std::option::Option<Vec<crate::schemas::ResultDisplayField>>,
    }
    impl ::google_field_selector::FieldSelector for ResultDisplayLine {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResultDisplayLine {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ResultDisplayMetadata {
        #[doc = "The metalines content to be displayed with the result."]
        #[serde(
            rename = "metalines",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metalines: ::std::option::Option<Vec<crate::schemas::ResultDisplayLine>>,
        #[doc = "The display label for the object."]
        #[serde(
            rename = "objectTypeLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_type_label: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ResultDisplayMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResultDisplayMetadata {
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
    pub struct RetrievalImportance {
        #[doc = "Indicates the ranking importance given to property when it is matched during retrieval. Once set, the token importance of a property cannot be changed."]
        #[serde(
            rename = "importance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub importance: ::std::option::Option<crate::schemas::RetrievalImportanceImportance>,
    }
    impl ::google_field_selector::FieldSelector for RetrievalImportance {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RetrievalImportance {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RetrievalImportanceImportance {
        #[doc = "Treat the match like a body text match."]
        Default,
        #[doc = "Treat the match with higher importance than body text."]
        High,
        #[doc = "Treat the match like a match against title of the item."]
        Highest,
        #[doc = "Treat the match with lower importance than body text."]
        Low,
        #[doc = "Do not match against this field during retrieval. The property can still be used for operator matching, faceting, and suggest if desired."]
        None,
    }
    impl RetrievalImportanceImportance {
        pub fn as_str(self) -> &'static str {
            match self {
                RetrievalImportanceImportance::Default => "DEFAULT",
                RetrievalImportanceImportance::High => "HIGH",
                RetrievalImportanceImportance::Highest => "HIGHEST",
                RetrievalImportanceImportance::Low => "LOW",
                RetrievalImportanceImportance::None => "NONE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for RetrievalImportanceImportance {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RetrievalImportanceImportance {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<RetrievalImportanceImportance, ()> {
            Ok(match s {
                "DEFAULT" => RetrievalImportanceImportance::Default,
                "HIGH" => RetrievalImportanceImportance::High,
                "HIGHEST" => RetrievalImportanceImportance::Highest,
                "LOW" => RetrievalImportanceImportance::Low,
                "NONE" => RetrievalImportanceImportance::None,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for RetrievalImportanceImportance {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RetrievalImportanceImportance {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RetrievalImportanceImportance {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEFAULT" => RetrievalImportanceImportance::Default,
                "HIGH" => RetrievalImportanceImportance::High,
                "HIGHEST" => RetrievalImportanceImportance::Highest,
                "LOW" => RetrievalImportanceImportance::Low,
                "NONE" => RetrievalImportanceImportance::None,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RetrievalImportanceImportance {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RetrievalImportanceImportance {
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
    pub struct Schema {
        #[doc = "The list of top-level objects for the data source. The maximum number of elements is 10."]
        #[serde(
            rename = "objectDefinitions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub object_definitions: ::std::option::Option<Vec<crate::schemas::ObjectDefinition>>,
        #[doc = "IDs of the Long Running Operations (LROs) currently running for this schema. After modifying the schema, wait for operations to complete before indexing additional content."]
        #[serde(
            rename = "operationIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation_ids: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for Schema {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Schema {
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
    pub struct ScoringConfig {
        #[doc = "Whether to use freshness as a ranking signal. By default, freshness is used as a ranking signal. Note that this setting is not available in the Admin UI."]
        #[serde(
            rename = "disableFreshness",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_freshness: ::std::option::Option<bool>,
        #[doc = "Whether to personalize the results. By default, personal signals will be used to boost results."]
        #[serde(
            rename = "disablePersonalization",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_personalization: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for ScoringConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScoringConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SearchApplication {
        #[doc = "Retrictions applied to the configurations. The maximum number of elements is 10."]
        #[serde(
            rename = "dataSourceRestrictions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_source_restrictions:
            ::std::option::Option<Vec<crate::schemas::DataSourceRestriction>>,
        #[doc = "The default fields for returning facet results. The sources specified here also have been included in data_source_restrictions above."]
        #[serde(
            rename = "defaultFacetOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_facet_options: ::std::option::Option<Vec<crate::schemas::FacetOptions>>,
        #[doc = "The default options for sorting the search results"]
        #[serde(
            rename = "defaultSortOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub default_sort_options: ::std::option::Option<crate::schemas::SortOptions>,
        #[doc = "Display name of the Search Application. The maximum length is 300 characters."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Indicates whether audit logging is on/off for requests made for the search application in query APIs."]
        #[serde(
            rename = "enableAuditLog",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_audit_log: ::std::option::Option<bool>,
        #[doc = "Name of the Search Application. Format: searchapplications/{application_id}."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. IDs of the Long Running Operations (LROs) currently running for this schema. Output only field."]
        #[serde(
            rename = "operationIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation_ids: ::std::option::Option<Vec<String>>,
        #[doc = "The default options for query interpretation"]
        #[serde(
            rename = "queryInterpretationConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query_interpretation_config:
            ::std::option::Option<crate::schemas::QueryInterpretationConfig>,
        #[doc = "With each result we should return the URI for its thumbnail (when applicable)"]
        #[serde(
            rename = "returnResultThumbnailUrls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub return_result_thumbnail_urls: ::std::option::Option<bool>,
        #[doc = "Configuration for ranking results."]
        #[serde(
            rename = "scoringConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scoring_config: ::std::option::Option<crate::schemas::ScoringConfig>,
        #[doc = "Configuration for a sources specified in data_source_restrictions."]
        #[serde(
            rename = "sourceConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_config: ::std::option::Option<Vec<crate::schemas::SourceConfig>>,
    }
    impl ::google_field_selector::FieldSelector for SearchApplication {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchApplication {
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
    pub struct SearchApplicationQueryStats {
        #[doc = "Date for which query stats were calculated. Stats calculated on the next day close to midnight are returned."]
        #[serde(
            rename = "date",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date: ::std::option::Option<crate::schemas::Date>,
        #[serde(
            rename = "queryCountByStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query_count_by_status: ::std::option::Option<Vec<crate::schemas::QueryCountByStatus>>,
    }
    impl ::google_field_selector::FieldSelector for SearchApplicationQueryStats {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchApplicationQueryStats {
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
    pub struct SearchApplicationSessionStats {
        #[doc = "Date for which session stats were calculated. Stats are calculated on the following day, close to midnight PST, and then returned."]
        #[serde(
            rename = "date",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "The count of search sessions on the day"]
        #[serde(
            rename = "searchSessionsCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub search_sessions_count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for SearchApplicationSessionStats {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchApplicationSessionStats {
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
    pub struct SearchApplicationUserStats {
        #[doc = "Date for which session stats were calculated. Stats calculated on the next day close to midnight are returned."]
        #[serde(
            rename = "date",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "The count of unique active users in the past one day"]
        #[serde(
            rename = "oneDayActiveUsersCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub one_day_active_users_count: ::std::option::Option<i64>,
        #[doc = "The count of unique active users in the past seven days"]
        #[serde(
            rename = "sevenDaysActiveUsersCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub seven_days_active_users_count: ::std::option::Option<i64>,
        #[doc = "The count of unique active users in the past thirty days"]
        #[serde(
            rename = "thirtyDaysActiveUsersCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub thirty_days_active_users_count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for SearchApplicationUserStats {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchApplicationUserStats {
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
    pub struct SearchItemsByViewUrlRequest {
        #[doc = "Common debug options."]
        #[serde(
            rename = "debugOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub debug_options: ::std::option::Option<crate::schemas::DebugOptions>,
        #[doc = "The next_page_token value returned from a previous request, if any."]
        #[serde(
            rename = "pageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_token: ::std::option::Option<String>,
        #[doc = "Specify the full view URL to find the corresponding item. The maximum length is 2048 characters."]
        #[serde(
            rename = "viewUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub view_url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SearchItemsByViewUrlRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchItemsByViewUrlRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SearchItemsByViewUrlResponse {
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::Item>>,
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SearchItemsByViewUrlResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchItemsByViewUrlResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SearchQualityMetadata {
        #[doc = "An indication of the quality of the item, used to influence search quality. Value should be between 0.0 (lowest quality) and 1.0 (highest quality). The default value is 0.0."]
        #[serde(
            rename = "quality",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub quality: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for SearchQualityMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchQualityMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SearchRequest {
        #[doc = "Context attributes for the request which will be used to adjust ranking of search results. The maximum number of elements is 10."]
        #[serde(
            rename = "contextAttributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub context_attributes: ::std::option::Option<Vec<crate::schemas::ContextAttribute>>,
        #[doc = "The sources to use for querying. If not specified, all data sources from the current search application are used."]
        #[serde(
            rename = "dataSourceRestrictions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_source_restrictions:
            ::std::option::Option<Vec<crate::schemas::DataSourceRestriction>>,
        #[serde(
            rename = "facetOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub facet_options: ::std::option::Option<Vec<crate::schemas::FacetOptions>>,
        #[doc = "Maximum number of search results to return in one page. Valid values are between 1 and 100, inclusive. Default value is 10. Minimum value is 50 when results beyond 2000 are requested."]
        #[serde(
            rename = "pageSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_size: ::std::option::Option<i32>,
        #[doc = "The raw query string. See supported search operators in the [Narrow your search with operators](https://support.google.com/cloudsearch/answer/6172299)"]
        #[serde(
            rename = "query",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query: ::std::option::Option<String>,
        #[doc = "Options to interpret the user query."]
        #[serde(
            rename = "queryInterpretationOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query_interpretation_options:
            ::std::option::Option<crate::schemas::QueryInterpretationOptions>,
        #[doc = "Request options, such as the search application and user timezone."]
        #[serde(
            rename = "requestOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_options: ::std::option::Option<crate::schemas::RequestOptions>,
        #[doc = "The options for sorting the search results"]
        #[serde(
            rename = "sortOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sort_options: ::std::option::Option<crate::schemas::SortOptions>,
        #[doc = "Starting index of the results."]
        #[serde(
            rename = "start",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for SearchRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SearchResponse {
        #[doc = "Debugging information about the response."]
        #[serde(
            rename = "debugInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub debug_info: ::std::option::Option<crate::schemas::ResponseDebugInfo>,
        #[doc = "Error information about the response."]
        #[serde(
            rename = "errorInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_info: ::std::option::Option<crate::schemas::ErrorInfo>,
        #[doc = "Repeated facet results."]
        #[serde(
            rename = "facetResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub facet_results: ::std::option::Option<Vec<crate::schemas::FacetResult>>,
        #[doc = "Whether there are more search results matching the query."]
        #[serde(
            rename = "hasMoreResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub has_more_results: ::std::option::Option<bool>,
        #[doc = "Query interpretation result for user query. Empty if query interpretation is disabled."]
        #[serde(
            rename = "queryInterpretation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query_interpretation: ::std::option::Option<crate::schemas::QueryInterpretation>,
        #[doc = "The estimated result count for this query."]
        #[serde(
            rename = "resultCountEstimate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub result_count_estimate: ::std::option::Option<i64>,
        #[doc = "The exact result count for this query."]
        #[serde(
            rename = "resultCountExact",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub result_count_exact: ::std::option::Option<i64>,
        #[doc = "Expanded result count information."]
        #[serde(
            rename = "resultCounts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub result_counts: ::std::option::Option<crate::schemas::ResultCounts>,
        #[doc = "Results from a search query."]
        #[serde(
            rename = "results",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub results: ::std::option::Option<Vec<crate::schemas::SearchResult>>,
        #[doc = "Suggested spelling for the query."]
        #[serde(
            rename = "spellResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spell_results: ::std::option::Option<Vec<crate::schemas::SpellResult>>,
        #[doc = "Structured results for the user query. These results are not counted against the page_size."]
        #[serde(
            rename = "structuredResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub structured_results: ::std::option::Option<Vec<crate::schemas::StructuredResult>>,
    }
    impl ::google_field_selector::FieldSelector for SearchResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SearchResult {
        #[doc = "If source is clustered, provide list of clustered results. There will only be one level of clustered results. If current source is not enabled for clustering, this field will be empty."]
        #[serde(
            rename = "clusteredResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub clustered_results: ::std::option::Option<Vec<crate::schemas::SearchResult>>,
        #[doc = "Debugging information about this search result."]
        #[serde(
            rename = "debugInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub debug_info: ::std::option::Option<crate::schemas::ResultDebugInfo>,
        #[doc = "Metadata of the search result."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::Metadata>,
        #[doc = "The concatenation of all snippets (summaries) available for this result."]
        #[serde(
            rename = "snippet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub snippet: ::std::option::Option<crate::schemas::Snippet>,
        #[doc = "Title of the search result."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "The URL of the search result. The URL contains a Google redirect to the actual item. This URL is signed and shouldn't be changed."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SearchResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchResult {
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
    pub struct ShareScope {
        #[doc = "If scope is DOMAIN, this field contains the dasher domain, for example \"google.com\"."]
        #[serde(
            rename = "domain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain: ::std::option::Option<String>,
        #[doc = "The scope to which the content was shared."]
        #[serde(
            rename = "scope",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scope: ::std::option::Option<crate::schemas::ShareScopeScope>,
    }
    impl ::google_field_selector::FieldSelector for ShareScope {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ShareScope {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ShareScopeScope {
        DasherDomain,
        #[doc = "Viewable by extended circles."]
        Extended,
        #[doc = "Viewable only by a set of people."]
        Limited,
        #[doc = "Only the author can view the post."]
        Private,
        Public,
        Unknown,
    }
    impl ShareScopeScope {
        pub fn as_str(self) -> &'static str {
            match self {
                ShareScopeScope::DasherDomain => "DASHER_DOMAIN",
                ShareScopeScope::Extended => "EXTENDED",
                ShareScopeScope::Limited => "LIMITED",
                ShareScopeScope::Private => "PRIVATE",
                ShareScopeScope::Public => "PUBLIC",
                ShareScopeScope::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ShareScopeScope {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ShareScopeScope {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ShareScopeScope, ()> {
            Ok(match s {
                "DASHER_DOMAIN" => ShareScopeScope::DasherDomain,
                "EXTENDED" => ShareScopeScope::Extended,
                "LIMITED" => ShareScopeScope::Limited,
                "PRIVATE" => ShareScopeScope::Private,
                "PUBLIC" => ShareScopeScope::Public,
                "UNKNOWN" => ShareScopeScope::Unknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ShareScopeScope {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ShareScopeScope {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ShareScopeScope {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DASHER_DOMAIN" => ShareScopeScope::DasherDomain,
                "EXTENDED" => ShareScopeScope::Extended,
                "LIMITED" => ShareScopeScope::Limited,
                "PRIVATE" => ShareScopeScope::Private,
                "PUBLIC" => ShareScopeScope::Public,
                "UNKNOWN" => ShareScopeScope::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ShareScopeScope {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ShareScopeScope {
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
    pub struct Snippet {
        #[doc = "The matched ranges in the snippet."]
        #[serde(
            rename = "matchRanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub match_ranges: ::std::option::Option<Vec<crate::schemas::MatchRange>>,
        #[doc = "The snippet of the document. The snippet of the document. May contain escaped HTML character that should be unescaped prior to rendering."]
        #[serde(
            rename = "snippet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub snippet: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Snippet {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Snippet {
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
    pub struct SortOptions {
        #[doc = "Name of the operator corresponding to the field to sort on. The corresponding property must be marked as sortable."]
        #[serde(
            rename = "operatorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator_name: ::std::option::Option<String>,
        #[doc = "Ascending is the default sort order"]
        #[serde(
            rename = "sortOrder",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sort_order: ::std::option::Option<crate::schemas::SortOptionsSortOrder>,
    }
    impl ::google_field_selector::FieldSelector for SortOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SortOptions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SortOptionsSortOrder {
        Ascending,
        Descending,
    }
    impl SortOptionsSortOrder {
        pub fn as_str(self) -> &'static str {
            match self {
                SortOptionsSortOrder::Ascending => "ASCENDING",
                SortOptionsSortOrder::Descending => "DESCENDING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SortOptionsSortOrder {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SortOptionsSortOrder {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SortOptionsSortOrder, ()> {
            Ok(match s {
                "ASCENDING" => SortOptionsSortOrder::Ascending,
                "DESCENDING" => SortOptionsSortOrder::Descending,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SortOptionsSortOrder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SortOptionsSortOrder {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SortOptionsSortOrder {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ASCENDING" => SortOptionsSortOrder::Ascending,
                "DESCENDING" => SortOptionsSortOrder::Descending,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SortOptionsSortOrder {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SortOptionsSortOrder {
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
    pub struct Source {
        #[doc = "Source name for content indexed by the Indexing API."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Predefined content source for Google Apps."]
        #[serde(
            rename = "predefinedSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub predefined_source: ::std::option::Option<crate::schemas::SourcePredefinedSource>,
    }
    impl ::google_field_selector::FieldSelector for Source {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Source {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SourcePredefinedSource {
        GoogleCalendar,
        GoogleDrive,
        GoogleGmail,
        GoogleGroups,
        GoogleKeep,
        GoogleSites,
        None,
        #[doc = "Suggests people in the organization. Only valid when used with the suggest API. Results in an error when used in the query API."]
        Person,
        #[doc = "Suggests queries issued by the user in the past. Only valid when used with the suggest API. Ignored when used in the query API."]
        QueryHistory,
    }
    impl SourcePredefinedSource {
        pub fn as_str(self) -> &'static str {
            match self {
                SourcePredefinedSource::GoogleCalendar => "GOOGLE_CALENDAR",
                SourcePredefinedSource::GoogleDrive => "GOOGLE_DRIVE",
                SourcePredefinedSource::GoogleGmail => "GOOGLE_GMAIL",
                SourcePredefinedSource::GoogleGroups => "GOOGLE_GROUPS",
                SourcePredefinedSource::GoogleKeep => "GOOGLE_KEEP",
                SourcePredefinedSource::GoogleSites => "GOOGLE_SITES",
                SourcePredefinedSource::None => "NONE",
                SourcePredefinedSource::Person => "PERSON",
                SourcePredefinedSource::QueryHistory => "QUERY_HISTORY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SourcePredefinedSource {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SourcePredefinedSource {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SourcePredefinedSource, ()> {
            Ok(match s {
                "GOOGLE_CALENDAR" => SourcePredefinedSource::GoogleCalendar,
                "GOOGLE_DRIVE" => SourcePredefinedSource::GoogleDrive,
                "GOOGLE_GMAIL" => SourcePredefinedSource::GoogleGmail,
                "GOOGLE_GROUPS" => SourcePredefinedSource::GoogleGroups,
                "GOOGLE_KEEP" => SourcePredefinedSource::GoogleKeep,
                "GOOGLE_SITES" => SourcePredefinedSource::GoogleSites,
                "NONE" => SourcePredefinedSource::None,
                "PERSON" => SourcePredefinedSource::Person,
                "QUERY_HISTORY" => SourcePredefinedSource::QueryHistory,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SourcePredefinedSource {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SourcePredefinedSource {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SourcePredefinedSource {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "GOOGLE_CALENDAR" => SourcePredefinedSource::GoogleCalendar,
                "GOOGLE_DRIVE" => SourcePredefinedSource::GoogleDrive,
                "GOOGLE_GMAIL" => SourcePredefinedSource::GoogleGmail,
                "GOOGLE_GROUPS" => SourcePredefinedSource::GoogleGroups,
                "GOOGLE_KEEP" => SourcePredefinedSource::GoogleKeep,
                "GOOGLE_SITES" => SourcePredefinedSource::GoogleSites,
                "NONE" => SourcePredefinedSource::None,
                "PERSON" => SourcePredefinedSource::Person,
                "QUERY_HISTORY" => SourcePredefinedSource::QueryHistory,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SourcePredefinedSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SourcePredefinedSource {
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
    pub struct SourceConfig {
        #[doc = "The crowding configuration for the source."]
        #[serde(
            rename = "crowdingConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub crowding_config: ::std::option::Option<crate::schemas::SourceCrowdingConfig>,
        #[doc = "The scoring configuration for the source."]
        #[serde(
            rename = "scoringConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scoring_config: ::std::option::Option<crate::schemas::SourceScoringConfig>,
        #[doc = "The source for which this configuration is to be used."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<crate::schemas::Source>,
    }
    impl ::google_field_selector::FieldSelector for SourceConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SourceConfig {
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
    pub struct SourceCrowdingConfig {
        #[doc = "Maximum number of results allowed from a datasource in a result page as long as results from other sources are not exhausted. Value specified must not be negative. A default value is used if this value is equal to 0. To disable crowding, set the value greater than 100."]
        #[serde(
            rename = "numResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub num_results: ::std::option::Option<i32>,
        #[doc = "Maximum number of suggestions allowed from a source. No limits will be set on results if this value is less than or equal to 0."]
        #[serde(
            rename = "numSuggestions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub num_suggestions: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for SourceCrowdingConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SourceCrowdingConfig {
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
    pub struct SourceResultCount {
        #[doc = "Whether there are more search results for this source."]
        #[serde(
            rename = "hasMoreResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub has_more_results: ::std::option::Option<bool>,
        #[doc = "The estimated result count for this source."]
        #[serde(
            rename = "resultCountEstimate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub result_count_estimate: ::std::option::Option<i64>,
        #[doc = "The exact result count for this source."]
        #[serde(
            rename = "resultCountExact",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub result_count_exact: ::std::option::Option<i64>,
        #[doc = "The source the result count information is associated with."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<crate::schemas::Source>,
    }
    impl ::google_field_selector::FieldSelector for SourceResultCount {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SourceResultCount {
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
    pub struct SourceScoringConfig {
        #[doc = "Importance of the source."]
        #[serde(
            rename = "sourceImportance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_importance:
            ::std::option::Option<crate::schemas::SourceScoringConfigSourceImportance>,
    }
    impl ::google_field_selector::FieldSelector for SourceScoringConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SourceScoringConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SourceScoringConfigSourceImportance {
        Default,
        High,
        Low,
    }
    impl SourceScoringConfigSourceImportance {
        pub fn as_str(self) -> &'static str {
            match self {
                SourceScoringConfigSourceImportance::Default => "DEFAULT",
                SourceScoringConfigSourceImportance::High => "HIGH",
                SourceScoringConfigSourceImportance::Low => "LOW",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SourceScoringConfigSourceImportance {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SourceScoringConfigSourceImportance {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SourceScoringConfigSourceImportance, ()> {
            Ok(match s {
                "DEFAULT" => SourceScoringConfigSourceImportance::Default,
                "HIGH" => SourceScoringConfigSourceImportance::High,
                "LOW" => SourceScoringConfigSourceImportance::Low,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SourceScoringConfigSourceImportance {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SourceScoringConfigSourceImportance {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SourceScoringConfigSourceImportance {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEFAULT" => SourceScoringConfigSourceImportance::Default,
                "HIGH" => SourceScoringConfigSourceImportance::High,
                "LOW" => SourceScoringConfigSourceImportance::Low,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SourceScoringConfigSourceImportance {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SourceScoringConfigSourceImportance {
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
    pub struct SpaceId {
        #[doc = "Unique, immutable ID of the Space"]
        #[serde(
            rename = "spaceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub space_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SpaceId {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SpaceId {
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
    pub struct SpaceInfo {
        #[serde(
            rename = "avatarInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub avatar_info: ::std::option::Option<crate::schemas::AvatarInfo>,
        #[serde(
            rename = "avatarUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub avatar_url: ::std::option::Option<String>,
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[serde(
            rename = "groupId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group_id: ::std::option::Option<crate::schemas::GroupId>,
        #[doc = "The email address of the user that invited the calling user to the room, if available. This field will only be populated for direct invites, it will be empty if the user was indirectly invited to the group."]
        #[serde(
            rename = "inviterEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub inviter_email: ::std::option::Option<String>,
        #[doc = "Whether this is a space that enables guest access"]
        #[serde(
            rename = "isExternal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_external: ::std::option::Option<bool>,
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[serde(
            rename = "numMembers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub num_members: ::std::option::Option<i32>,
        #[doc = "searching user's membership state in this space"]
        #[serde(
            rename = "userMembershipState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_membership_state:
            ::std::option::Option<crate::schemas::SpaceInfoUserMembershipState>,
    }
    impl ::google_field_selector::FieldSelector for SpaceInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SpaceInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SpaceInfoUserMembershipState {
        #[doc = "This state should never be stored in Spanner. It is a state for responses to the clients to indicate that membership mutations have failed and the member is in its previous state."]
        MemberFailed,
        #[doc = "An invitation to the space has been sent"]
        MemberInvited,
        #[doc = "User has joined the space"]
        MemberJoined,
        #[doc = "User is not a member"]
        MemberNotAMember,
        #[doc = "Default state, do not use"]
        MemberUnknown,
    }
    impl SpaceInfoUserMembershipState {
        pub fn as_str(self) -> &'static str {
            match self {
                SpaceInfoUserMembershipState::MemberFailed => "MEMBER_FAILED",
                SpaceInfoUserMembershipState::MemberInvited => "MEMBER_INVITED",
                SpaceInfoUserMembershipState::MemberJoined => "MEMBER_JOINED",
                SpaceInfoUserMembershipState::MemberNotAMember => "MEMBER_NOT_A_MEMBER",
                SpaceInfoUserMembershipState::MemberUnknown => "MEMBER_UNKNOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SpaceInfoUserMembershipState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SpaceInfoUserMembershipState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SpaceInfoUserMembershipState, ()> {
            Ok(match s {
                "MEMBER_FAILED" => SpaceInfoUserMembershipState::MemberFailed,
                "MEMBER_INVITED" => SpaceInfoUserMembershipState::MemberInvited,
                "MEMBER_JOINED" => SpaceInfoUserMembershipState::MemberJoined,
                "MEMBER_NOT_A_MEMBER" => SpaceInfoUserMembershipState::MemberNotAMember,
                "MEMBER_UNKNOWN" => SpaceInfoUserMembershipState::MemberUnknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SpaceInfoUserMembershipState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SpaceInfoUserMembershipState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SpaceInfoUserMembershipState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "MEMBER_FAILED" => SpaceInfoUserMembershipState::MemberFailed,
                "MEMBER_INVITED" => SpaceInfoUserMembershipState::MemberInvited,
                "MEMBER_JOINED" => SpaceInfoUserMembershipState::MemberJoined,
                "MEMBER_NOT_A_MEMBER" => SpaceInfoUserMembershipState::MemberNotAMember,
                "MEMBER_UNKNOWN" => SpaceInfoUserMembershipState::MemberUnknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SpaceInfoUserMembershipState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SpaceInfoUserMembershipState {
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
    pub struct SpellResult {
        #[doc = "The suggested spelling of the query."]
        #[serde(
            rename = "suggestedQuery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_query: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SpellResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SpellResult {
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
    pub struct StartUploadItemRequest {
        #[doc = "Name of connector making this call. Format: datasources/{source_id}/connectors/{ID}"]
        #[serde(
            rename = "connectorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub connector_name: ::std::option::Option<String>,
        #[doc = "Common debug options."]
        #[serde(
            rename = "debugOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub debug_options: ::std::option::Option<crate::schemas::DebugOptions>,
    }
    impl ::google_field_selector::FieldSelector for StartUploadItemRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StartUploadItemRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Status {
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
    impl ::google_field_selector::FieldSelector for Status {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Status {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct StructuredDataObject {
        #[doc = "The properties for the object. The maximum number of elements is 1000."]
        #[serde(
            rename = "properties",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub properties: ::std::option::Option<Vec<crate::schemas::NamedProperty>>,
    }
    impl ::google_field_selector::FieldSelector for StructuredDataObject {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StructuredDataObject {
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
    pub struct StructuredResult {
        #[doc = "Representation of a person"]
        #[serde(
            rename = "person",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub person: ::std::option::Option<crate::schemas::Person>,
    }
    impl ::google_field_selector::FieldSelector for StructuredResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StructuredResult {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SuggestRequest {
        #[doc = "The sources to use for suggestions. If not specified, the data sources are taken from the current search application. NOTE: Suggestions are only supported for the following sources: * Third-party data sources * PredefinedSource.PERSON * PredefinedSource.GOOGLE_DRIVE"]
        #[serde(
            rename = "dataSourceRestrictions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_source_restrictions:
            ::std::option::Option<Vec<crate::schemas::DataSourceRestriction>>,
        #[doc = "Partial query for which autocomplete suggestions will be shown. For example, if the query is \"sea\", then the server might return \"season\", \"search\", \"seagull\" and so on."]
        #[serde(
            rename = "query",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query: ::std::option::Option<String>,
        #[doc = "Request options, such as the search application and user timezone."]
        #[serde(
            rename = "requestOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_options: ::std::option::Option<crate::schemas::RequestOptions>,
    }
    impl ::google_field_selector::FieldSelector for SuggestRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuggestRequest {
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
    pub struct SuggestResponse {
        #[doc = "List of suggestions."]
        #[serde(
            rename = "suggestResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggest_results: ::std::option::Option<Vec<crate::schemas::SuggestResult>>,
    }
    impl ::google_field_selector::FieldSelector for SuggestResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuggestResponse {
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
    pub struct SuggestResult {
        #[doc = "This is present when the suggestion indicates a person. It contains more information about the person - like their email ID, name etc."]
        #[serde(
            rename = "peopleSuggestion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub people_suggestion: ::std::option::Option<crate::schemas::PeopleSuggestion>,
        #[doc = "This field will be present if the suggested query is a word/phrase completion."]
        #[serde(
            rename = "querySuggestion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query_suggestion: ::std::option::Option<crate::schemas::QuerySuggestion>,
        #[doc = "The source of the suggestion."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<crate::schemas::Source>,
        #[doc = "The suggested query that will be used for search, when the user clicks on the suggestion"]
        #[serde(
            rename = "suggestedQuery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_query: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SuggestResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuggestResult {
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
    pub struct TextOperatorOptions {
        #[doc = "If true, the text value is tokenized as one atomic value in operator searches and facet matches. For example, if the operator name is \"genre\" and the value is \"science-fiction\" the query restrictions \"genre:science\" and \"genre:fiction\" doesn't match the item; \"genre:science-fiction\" does. Value matching is case-sensitive and does not remove special characters. If false, the text is tokenized. For example, if the value is \"science-fiction\" the queries \"genre:science\" and \"genre:fiction\" matches the item."]
        #[serde(
            rename = "exactMatchWithOperator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exact_match_with_operator: ::std::option::Option<bool>,
        #[doc = "Indicates the operator name required in the query in order to isolate the text property. For example, if operatorName is *subject* and the property's name is *subjectLine*, then queries like *subject:<value>* show results only where the value of the property named *subjectLine* matches *<value>*. By contrast, a search that uses the same *<value>* without an operator returns all items where *<value>* matches the value of any text properties or text within the content field for the item. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
        #[serde(
            rename = "operatorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TextOperatorOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextOperatorOptions {
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
    pub struct TextPropertyOptions {
        #[doc = "If set, describes how the property should be used as a search operator."]
        #[serde(
            rename = "operatorOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator_options: ::std::option::Option<crate::schemas::TextOperatorOptions>,
        #[doc = "Indicates the search quality importance of the tokens within the field when used for retrieval."]
        #[serde(
            rename = "retrievalImportance",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub retrieval_importance: ::std::option::Option<crate::schemas::RetrievalImportance>,
    }
    impl ::google_field_selector::FieldSelector for TextPropertyOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextPropertyOptions {
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
    pub struct TextValues {
        #[doc = "The maximum allowable length for text values is 2048 characters."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for TextValues {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TextValues {
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
    pub struct TimestampOperatorOptions {
        #[doc = "Indicates the operator name required in the query in order to isolate the timestamp property using the greater-than operator. For example, if greaterThanOperatorName is *closedafter* and the property's name is *closeDate*, then queries like *closedafter:<value>* show results only where the value of the property named *closeDate* is later than *<value>*. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
        #[serde(
            rename = "greaterThanOperatorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub greater_than_operator_name: ::std::option::Option<String>,
        #[doc = "Indicates the operator name required in the query in order to isolate the timestamp property using the less-than operator. For example, if lessThanOperatorName is *closedbefore* and the property's name is *closeDate*, then queries like *closedbefore:<value>* show results only where the value of the property named *closeDate* is earlier than *<value>*. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
        #[serde(
            rename = "lessThanOperatorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub less_than_operator_name: ::std::option::Option<String>,
        #[doc = "Indicates the operator name required in the query in order to isolate the timestamp property. For example, if operatorName is *closedon* and the property's name is *closeDate*, then queries like *closedon:<value>* show results only where the value of the property named *closeDate* matches *<value>*. By contrast, a search that uses the same *<value>* without an operator returns all items where *<value>* matches the value of any String properties or text within the content field for the item. The operator name can only contain lowercase letters (a-z). The maximum length is 32 characters."]
        #[serde(
            rename = "operatorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TimestampOperatorOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TimestampOperatorOptions {
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
    pub struct TimestampPropertyOptions {
        #[doc = "If set, describes how the timestamp should be used as a search operator."]
        #[serde(
            rename = "operatorOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator_options: ::std::option::Option<crate::schemas::TimestampOperatorOptions>,
    }
    impl ::google_field_selector::FieldSelector for TimestampPropertyOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TimestampPropertyOptions {
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
    pub struct TimestampValues {
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for TimestampValues {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TimestampValues {
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
    pub struct TypeInfo {
        #[doc = "Contains additional video information only if document_type is VIDEO."]
        #[serde(
            rename = "videoInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub video_info: ::std::option::Option<crate::schemas::VideoInfo>,
    }
    impl ::google_field_selector::FieldSelector for TypeInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TypeInfo {
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
    pub struct UnmappedIdentity {
        #[doc = "The resource name for an external user."]
        #[serde(
            rename = "externalIdentity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub external_identity: ::std::option::Option<crate::schemas::Principal>,
        #[doc = "The resolution status for the external identity."]
        #[serde(
            rename = "resolutionStatusCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resolution_status_code:
            ::std::option::Option<crate::schemas::UnmappedIdentityResolutionStatusCode>,
    }
    impl ::google_field_selector::FieldSelector for UnmappedIdentity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UnmappedIdentity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum UnmappedIdentityResolutionStatusCode {
        #[doc = "Input-only value. Used to list all unmapped identities regardless of status."]
        CodeUnspecified,
        #[doc = "IDaaS does not understand the identity source, probably because the schema was modified in a non compatible way."]
        IdentitySourceMisconfigured,
        #[doc = "The identity source associated with the identity was either not found or deleted."]
        IdentitySourceNotFound,
        #[doc = "Internal error."]
        InternalError,
        #[doc = "The unmapped identity was not found in IDaaS, and needs to be provided by the user."]
        NotFound,
        #[doc = "The number of users associated with the external identity is too large."]
        TooManyMappingsFound,
    }
    impl UnmappedIdentityResolutionStatusCode {
        pub fn as_str(self) -> &'static str {
            match self {
                UnmappedIdentityResolutionStatusCode::CodeUnspecified => "CODE_UNSPECIFIED",
                UnmappedIdentityResolutionStatusCode::IdentitySourceMisconfigured => {
                    "IDENTITY_SOURCE_MISCONFIGURED"
                }
                UnmappedIdentityResolutionStatusCode::IdentitySourceNotFound => {
                    "IDENTITY_SOURCE_NOT_FOUND"
                }
                UnmappedIdentityResolutionStatusCode::InternalError => "INTERNAL_ERROR",
                UnmappedIdentityResolutionStatusCode::NotFound => "NOT_FOUND",
                UnmappedIdentityResolutionStatusCode::TooManyMappingsFound => {
                    "TOO_MANY_MAPPINGS_FOUND"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for UnmappedIdentityResolutionStatusCode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for UnmappedIdentityResolutionStatusCode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<UnmappedIdentityResolutionStatusCode, ()> {
            Ok(match s {
                "CODE_UNSPECIFIED" => UnmappedIdentityResolutionStatusCode::CodeUnspecified,
                "IDENTITY_SOURCE_MISCONFIGURED" => {
                    UnmappedIdentityResolutionStatusCode::IdentitySourceMisconfigured
                }
                "IDENTITY_SOURCE_NOT_FOUND" => {
                    UnmappedIdentityResolutionStatusCode::IdentitySourceNotFound
                }
                "INTERNAL_ERROR" => UnmappedIdentityResolutionStatusCode::InternalError,
                "NOT_FOUND" => UnmappedIdentityResolutionStatusCode::NotFound,
                "TOO_MANY_MAPPINGS_FOUND" => {
                    UnmappedIdentityResolutionStatusCode::TooManyMappingsFound
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for UnmappedIdentityResolutionStatusCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for UnmappedIdentityResolutionStatusCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UnmappedIdentityResolutionStatusCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CODE_UNSPECIFIED" => UnmappedIdentityResolutionStatusCode::CodeUnspecified,
                "IDENTITY_SOURCE_MISCONFIGURED" => {
                    UnmappedIdentityResolutionStatusCode::IdentitySourceMisconfigured
                }
                "IDENTITY_SOURCE_NOT_FOUND" => {
                    UnmappedIdentityResolutionStatusCode::IdentitySourceNotFound
                }
                "INTERNAL_ERROR" => UnmappedIdentityResolutionStatusCode::InternalError,
                "NOT_FOUND" => UnmappedIdentityResolutionStatusCode::NotFound,
                "TOO_MANY_MAPPINGS_FOUND" => {
                    UnmappedIdentityResolutionStatusCode::TooManyMappingsFound
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for UnmappedIdentityResolutionStatusCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UnmappedIdentityResolutionStatusCode {
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
    pub struct UnreserveItemsRequest {
        #[doc = "Name of connector making this call. Format: datasources/{source_id}/connectors/{ID}"]
        #[serde(
            rename = "connectorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub connector_name: ::std::option::Option<String>,
        #[doc = "Common debug options."]
        #[serde(
            rename = "debugOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub debug_options: ::std::option::Option<crate::schemas::DebugOptions>,
        #[doc = "Name of a queue to unreserve items from."]
        #[serde(
            rename = "queue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub queue: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UnreserveItemsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UnreserveItemsRequest {
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
    pub struct UpdateDataSourceRequest {
        #[doc = "Common debug options."]
        #[serde(
            rename = "debugOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub debug_options: ::std::option::Option<crate::schemas::DebugOptions>,
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<crate::schemas::DataSource>,
    }
    impl ::google_field_selector::FieldSelector for UpdateDataSourceRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateDataSourceRequest {
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
    pub struct UpdateSchemaRequest {
        #[doc = "Common debug options."]
        #[serde(
            rename = "debugOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub debug_options: ::std::option::Option<crate::schemas::DebugOptions>,
        #[doc = "The new schema for the source."]
        #[serde(
            rename = "schema",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schema: ::std::option::Option<crate::schemas::Schema>,
        #[doc = "If true, the schema will be checked for validity, but will not be registered with the data source, even if valid."]
        #[serde(
            rename = "validateOnly",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub validate_only: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for UpdateSchemaRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UpdateSchemaRequest {
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
    pub struct UploadItemRef {
        #[doc = "Name of the content reference. The maximum length is 2048 characters."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UploadItemRef {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UploadItemRef {
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
    pub struct UserId {
        #[doc = "Optional. Opaque, server-assigned ID of the user profile associated with App/user acting on behalf of the human user. This is currently only set when a 3P application is acting on the user's behalf."]
        #[serde(
            rename = "actingUserId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub acting_user_id: ::std::option::Option<String>,
        #[doc = "Opaque, server-assigned ID of the User."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Optional. Identifier of the App involved (directly or on behalf of a human creator) in creating this message. This is not set if the user posted a message directly, but is used in the case of, for example, a message being generated by a 1P integration based on a user action (creating an event, creating a task etc). This should only be used on the BE. For clients, please use the field in the FE message proto instead (google3/apps/dynamite/v1/frontend/api/message.proto?q=origin_app_id)."]
        #[serde(
            rename = "originAppId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub origin_app_id: ::std::option::Option<crate::schemas::AppId>,
        #[doc = "Clients do not need to send UserType to Backend, but Backend will always send this field to clients per the following rule: 1. For HUMAN Ids, the field is empty but by default .getType() will return HUMAN. 2. For BOT Ids, the field is ALWAYS set to BOT."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::UserIdType>,
    }
    impl ::google_field_selector::FieldSelector for UserId {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UserId {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum UserIdType {
        Bot,
        #[doc = "Notes on HUMAN type: 1) Leaving UserId.UserType field empty will return HUMAN as default value. This is expected because all the existing UserIds are without explicitly setting UserType, most of which are HUMAN Ids. For Bot Ids we will always set BOT in UserType field. 2) DO NOT explicitly set HUMAN as type. This is a proto2 issue, that a UserId with explicitly set default value HUMAN as type is NOT equal to an id without setting the field. aka. UserId id1 = UserId.newBuilder() .setId(\"dummy\").setType(UserType.HUMAN).build(); UserId id2 = UserId.newBuilder().setId(\"dummy\").build(); AssertThat(id1).isNotEqual(id2); AssertThat(id2.getType()).isEqualTo(UserType.HUMAN);"]
        Human,
    }
    impl UserIdType {
        pub fn as_str(self) -> &'static str {
            match self {
                UserIdType::Bot => "BOT",
                UserIdType::Human => "HUMAN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for UserIdType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for UserIdType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<UserIdType, ()> {
            Ok(match s {
                "BOT" => UserIdType::Bot,
                "HUMAN" => UserIdType::Human,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for UserIdType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for UserIdType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UserIdType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOT" => UserIdType::Bot,
                "HUMAN" => UserIdType::Human,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for UserIdType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UserIdType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Value {
        #[serde(
            rename = "booleanValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub boolean_value: ::std::option::Option<bool>,
        #[serde(
            rename = "dateValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_value: ::std::option::Option<crate::schemas::Date>,
        #[serde(
            rename = "doubleValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub double_value: ::std::option::Option<f64>,
        #[serde(
            rename = "integerValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub integer_value: ::std::option::Option<i64>,
        #[serde(
            rename = "stringValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub string_value: ::std::option::Option<String>,
        #[serde(
            rename = "timestampValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timestamp_value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Value {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Value {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ValueFilter {
        #[doc = "The `operator_name` applied to the query, such as *price_greater_than*. The filter can work against both types of filters defined in the schema for your data source: 1. `operator_name`, where the query filters results by the property that matches the value. 2. `greater_than_operator_name` or `less_than_operator_name` in your schema. The query filters the results for the property values that are greater than or less than the supplied value in the query."]
        #[serde(
            rename = "operatorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operator_name: ::std::option::Option<String>,
        #[doc = "The value to be compared with."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<crate::schemas::Value>,
    }
    impl ::google_field_selector::FieldSelector for ValueFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ValueFilter {
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
    pub struct VideoInfo {
        #[doc = "Duration of the video in milliseconds. This field can be absent for recently uploaded video or inaccurate sometimes."]
        #[serde(
            rename = "duration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub duration: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for VideoInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VideoInfo {
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
    pub struct Vpcsettings {
        #[doc = "The resource name of the GCP Project to be used for VPC SC policy check. VPC security settings on this project will be honored for Cloud Search APIs after project_name has been updated through CustomerService. Format: projects/{project_id}"]
        #[serde(
            rename = "project",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub project: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Vpcsettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Vpcsettings {
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
    #[doc = "Actions that can be performed on the debug resource"]
    pub fn debug(&self) -> crate::resources::debug::DebugActions {
        crate::resources::debug::DebugActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the indexing resource"]
    pub fn indexing(&self) -> crate::resources::indexing::IndexingActions {
        crate::resources::indexing::IndexingActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the media resource"]
    pub fn media(&self) -> crate::resources::media::MediaActions {
        crate::resources::media::MediaActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the operations resource"]
    pub fn operations(&self) -> crate::resources::operations::OperationsActions {
        crate::resources::operations::OperationsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the query resource"]
    pub fn query(&self) -> crate::resources::query::QueryActions {
        crate::resources::query::QueryActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the settings resource"]
    pub fn settings(&self) -> crate::resources::settings::SettingsActions {
        crate::resources::settings::SettingsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the stats resource"]
    pub fn stats(&self) -> crate::resources::stats::StatsActions {
        crate::resources::stats::StatsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the v_1 resource"]
    pub fn v_1(&self) -> crate::resources::v_1::V1Actions {
        crate::resources::v_1::V1Actions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod debug {
        pub mod params {}
        pub struct DebugActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> DebugActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the datasources resource"]
            pub fn datasources(&self) -> crate::resources::debug::datasources::DatasourcesActions {
                crate::resources::debug::datasources::DatasourcesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the identitysources resource"]
            pub fn identitysources(
                &self,
            ) -> crate::resources::debug::identitysources::IdentitysourcesActions {
                crate::resources::debug::identitysources::IdentitysourcesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod datasources {
            pub mod params {}
            pub struct DatasourcesActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> DatasourcesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Actions that can be performed on the items resource"]
                pub fn items(&self) -> crate::resources::debug::datasources::items::ItemsActions {
                    crate::resources::debug::datasources::items::ItemsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            pub mod items {
                pub mod params {}
                pub struct ItemsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> ItemsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Checks whether an item is accessible by specified principal. Principal must be a user; groups and domain values aren't supported. **Note:** This API requires an admin account to execute."]
                    pub fn check_access(
                        &self,
                        request: crate::schemas::Principal,
                        name: impl Into<String>,
                    ) -> CheckAccessRequestBuilder {
                        CheckAccessRequestBuilder {
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
                            debug_options_enable_debugging: None,
                        }
                    }
                    #[doc = "Fetches the item whose viewUrl exactly matches that of the URL provided in the request. **Note:** This API requires an admin account to execute."]
                    pub fn search_by_view_url(
                        &self,
                        request: crate::schemas::SearchItemsByViewUrlRequest,
                        name: impl Into<String>,
                    ) -> SearchByViewUrlRequestBuilder {
                        SearchByViewUrlRequestBuilder {
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
                    #[doc = "Actions that can be performed on the unmappedids resource"]
                    pub fn unmappedids(
                        &self,
                    ) -> crate::resources::debug::datasources::items::unmappedids::UnmappedidsActions
                    {
                        crate :: resources :: debug :: datasources :: items :: unmappedids :: UnmappedidsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                    }
                }
                #[doc = "Created via [ItemsActions::check_access()](struct.ItemsActions.html#method.check_access)"]
                #[derive(Debug, Clone)]
                pub struct CheckAccessRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::Principal,
                    name: String,
                    debug_options_enable_debugging: Option<bool>,
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
                impl<'a> CheckAccessRequestBuilder<'a> {
                    #[doc = "If you are asked by Google to help with debugging, set this field. Otherwise, ignore this field."]
                    pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                        self.debug_options_enable_debugging = Some(value);
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
                    ) -> Result<crate::schemas::CheckAccessResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::CheckAccessResponse, crate::Error>
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/debug/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":checkAccess");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                        req = req.query(&[(
                            "debugOptions.enableDebugging",
                            &self.debug_options_enable_debugging,
                        )]);
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
                #[doc = "Created via [ItemsActions::search_by_view_url()](struct.ItemsActions.html#method.search_by_view_url)"]
                #[derive(Debug, Clone)]
                pub struct SearchByViewUrlRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::SearchItemsByViewUrlRequest,
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
                impl<'a> SearchByViewUrlRequestBuilder<'a> {
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
                    ) -> Result<crate::schemas::SearchItemsByViewUrlResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::SearchItemsByViewUrlResponse, crate::Error>
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/debug/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/items:searchByViewUrl");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                pub mod unmappedids {
                    pub mod params {}
                    pub struct UnmappedidsActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> UnmappedidsActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "List all unmapped identities for a specific item. **Note:** This API requires an admin account to execute."]
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
                                debug_options_enable_debugging: None,
                                page_size: None,
                                page_token: None,
                            }
                        }
                    }
                    #[doc = "Created via [UnmappedidsActions::list()](struct.UnmappedidsActions.html#method.list)"]
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        parent: String,
                        debug_options_enable_debugging: Option<bool>,
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
                        #[doc = "If you are asked by Google to help with debugging, set this field. Otherwise, ignore this field."]
                        pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                            self.debug_options_enable_debugging = Some(value);
                            self
                        }
                        #[doc = "Maximum number of items to fetch in a request. Defaults to 100."]
                        pub fn page_size(mut self, value: i32) -> Self {
                            self.page_size = Some(value);
                            self
                        }
                        #[doc = "The next_page_token value returned from a previous List request, if any."]
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
                        ) -> Result<crate::schemas::ListUnmappedIdentitiesResponse, crate::Error>
                        {
                            self.execute_with_fields(None::<&str>).await
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub async fn execute_with_all_fields(
                            self,
                        ) -> Result<crate::schemas::ListUnmappedIdentitiesResponse, crate::Error>
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
                            let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                            output.push_str("v1/debug/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/unmappedids");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                            req = req.query(&[(
                                "debugOptions.enableDebugging",
                                &self.debug_options_enable_debugging,
                            )]);
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
        pub mod identitysources {
            pub mod params {}
            pub struct IdentitysourcesActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> IdentitysourcesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Actions that can be performed on the items resource"]
                pub fn items(
                    &self,
                ) -> crate::resources::debug::identitysources::items::ItemsActions {
                    crate::resources::debug::identitysources::items::ItemsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
                #[doc = "Actions that can be performed on the unmappedids resource"]
                pub fn unmappedids(
                    &self,
                ) -> crate::resources::debug::identitysources::unmappedids::UnmappedidsActions
                {
                    crate::resources::debug::identitysources::unmappedids::UnmappedidsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            pub mod items {
                pub mod params {}
                pub struct ItemsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> ItemsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Lists names of items associated with an unmapped identity. **Note:** This API requires an admin account to execute."]
                    pub fn list_forunmappedidentity(
                        &self,
                        parent: impl Into<String>,
                    ) -> ListForunmappedidentityRequestBuilder {
                        ListForunmappedidentityRequestBuilder {
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
                            debug_options_enable_debugging: None,
                            group_resource_name: None,
                            page_size: None,
                            page_token: None,
                            user_resource_name: None,
                        }
                    }
                }
                #[doc = "Created via [ItemsActions::list_forunmappedidentity()](struct.ItemsActions.html#method.list_forunmappedidentity)"]
                #[derive(Debug, Clone)]
                pub struct ListForunmappedidentityRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    parent: String,
                    debug_options_enable_debugging: Option<bool>,
                    group_resource_name: Option<String>,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    user_resource_name: Option<String>,
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
                impl<'a> ListForunmappedidentityRequestBuilder<'a> {
                    #[doc = "If you are asked by Google to help with debugging, set this field. Otherwise, ignore this field."]
                    pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                        self.debug_options_enable_debugging = Some(value);
                        self
                    }
                    #[doc = ""]
                    pub fn group_resource_name(mut self, value: impl Into<String>) -> Self {
                        self.group_resource_name = Some(value.into());
                        self
                    }
                    #[doc = "Maximum number of items to fetch in a request. Defaults to 100."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "The next_page_token value returned from a previous List request, if any."]
                    pub fn page_token(mut self, value: impl Into<String>) -> Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = ""]
                    pub fn user_resource_name(mut self, value: impl Into<String>) -> Self {
                        self.user_resource_name = Some(value.into());
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
                        crate::schemas::ListItemNamesForUnmappedIdentityResponse,
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
                        crate::schemas::ListItemNamesForUnmappedIdentityResponse,
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/debug/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/items:forunmappedidentity");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[(
                            "debugOptions.enableDebugging",
                            &self.debug_options_enable_debugging,
                        )]);
                        req = req.query(&[("groupResourceName", &self.group_resource_name)]);
                        req = req.query(&[("pageSize", &self.page_size)]);
                        req = req.query(&[("pageToken", &self.page_token)]);
                        req = req.query(&[("userResourceName", &self.user_resource_name)]);
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
            pub mod unmappedids {
                pub mod params {
                    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                    pub enum ListResolutionStatusCode {
                        #[doc = "Input-only value. Used to list all unmapped identities regardless of status."]
                        CodeUnspecified,
                        #[doc = "IDaaS does not understand the identity source, probably because the schema was modified in a non compatible way."]
                        IdentitySourceMisconfigured,
                        #[doc = "The identity source associated with the identity was either not found or deleted."]
                        IdentitySourceNotFound,
                        #[doc = "Internal error."]
                        InternalError,
                        #[doc = "The unmapped identity was not found in IDaaS, and needs to be provided by the user."]
                        NotFound,
                        #[doc = "The number of users associated with the external identity is too large."]
                        TooManyMappingsFound,
                    }
                    impl ListResolutionStatusCode {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                ListResolutionStatusCode::CodeUnspecified => "CODE_UNSPECIFIED",
                                ListResolutionStatusCode::IdentitySourceMisconfigured => {
                                    "IDENTITY_SOURCE_MISCONFIGURED"
                                }
                                ListResolutionStatusCode::IdentitySourceNotFound => {
                                    "IDENTITY_SOURCE_NOT_FOUND"
                                }
                                ListResolutionStatusCode::InternalError => "INTERNAL_ERROR",
                                ListResolutionStatusCode::NotFound => "NOT_FOUND",
                                ListResolutionStatusCode::TooManyMappingsFound => {
                                    "TOO_MANY_MAPPINGS_FOUND"
                                }
                            }
                        }
                    }
                    impl ::std::convert::AsRef<str> for ListResolutionStatusCode {
                        fn as_ref(&self) -> &str {
                            self.as_str()
                        }
                    }
                    impl ::std::str::FromStr for ListResolutionStatusCode {
                        type Err = ();
                        fn from_str(
                            s: &str,
                        ) -> ::std::result::Result<ListResolutionStatusCode, ()>
                        {
                            Ok(match s {
                                "CODE_UNSPECIFIED" => ListResolutionStatusCode::CodeUnspecified,
                                "IDENTITY_SOURCE_MISCONFIGURED" => {
                                    ListResolutionStatusCode::IdentitySourceMisconfigured
                                }
                                "IDENTITY_SOURCE_NOT_FOUND" => {
                                    ListResolutionStatusCode::IdentitySourceNotFound
                                }
                                "INTERNAL_ERROR" => ListResolutionStatusCode::InternalError,
                                "NOT_FOUND" => ListResolutionStatusCode::NotFound,
                                "TOO_MANY_MAPPINGS_FOUND" => {
                                    ListResolutionStatusCode::TooManyMappingsFound
                                }
                                _ => return Err(()),
                            })
                        }
                    }
                    impl ::std::fmt::Display for ListResolutionStatusCode {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for ListResolutionStatusCode {
                        fn serialize<S>(
                            &self,
                            serializer: S,
                        ) -> ::std::result::Result<S::Ok, S::Error>
                        where
                            S: ::serde::ser::Serializer,
                        {
                            serializer.serialize_str(self.as_str())
                        }
                    }
                    impl<'de> ::serde::Deserialize<'de> for ListResolutionStatusCode {
                        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            let value: &'de str = <&str>::deserialize(deserializer)?;
                            Ok(match value {
                                "CODE_UNSPECIFIED" => ListResolutionStatusCode::CodeUnspecified,
                                "IDENTITY_SOURCE_MISCONFIGURED" => {
                                    ListResolutionStatusCode::IdentitySourceMisconfigured
                                }
                                "IDENTITY_SOURCE_NOT_FOUND" => {
                                    ListResolutionStatusCode::IdentitySourceNotFound
                                }
                                "INTERNAL_ERROR" => ListResolutionStatusCode::InternalError,
                                "NOT_FOUND" => ListResolutionStatusCode::NotFound,
                                "TOO_MANY_MAPPINGS_FOUND" => {
                                    ListResolutionStatusCode::TooManyMappingsFound
                                }
                                _ => {
                                    return Err(::serde::de::Error::custom(format!(
                                        "invalid enum for #name: {}",
                                        value
                                    )))
                                }
                            })
                        }
                    }
                    impl ::google_field_selector::FieldSelector for ListResolutionStatusCode {
                        fn fields() -> Vec<::google_field_selector::Field> {
                            Vec::new()
                        }
                    }
                    impl ::google_field_selector::ToFieldType for ListResolutionStatusCode {
                        fn field_type() -> ::google_field_selector::FieldType {
                            ::google_field_selector::FieldType::Leaf
                        }
                    }
                }
                pub struct UnmappedidsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> UnmappedidsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Lists unmapped user identities for an identity source. **Note:** This API requires an admin account to execute."]
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
                            debug_options_enable_debugging: None,
                            page_size: None,
                            page_token: None,
                            resolution_status_code: None,
                        }
                    }
                }
                #[doc = "Created via [UnmappedidsActions::list()](struct.UnmappedidsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , parent : String , debug_options_enable_debugging : Option < bool > , page_size : Option < i32 > , page_token : Option < String > , resolution_status_code : Option < crate :: resources :: debug :: identitysources :: unmappedids :: params :: ListResolutionStatusCode > , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
                impl<'a> ListRequestBuilder<'a> {
                    #[doc = "If you are asked by Google to help with debugging, set this field. Otherwise, ignore this field."]
                    pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                        self.debug_options_enable_debugging = Some(value);
                        self
                    }
                    #[doc = "Maximum number of items to fetch in a request. Defaults to 100."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "The next_page_token value returned from a previous List request, if any."]
                    pub fn page_token(mut self, value: impl Into<String>) -> Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "Limit users selection to this status."]
                    pub fn resolution_status_code(
                        mut self,
                        value : crate :: resources :: debug :: identitysources :: unmappedids :: params :: ListResolutionStatusCode,
                    ) -> Self {
                        self.resolution_status_code = Some(value);
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
                    ) -> Result<crate::schemas::ListUnmappedIdentitiesResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListUnmappedIdentitiesResponse, crate::Error>
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/debug/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/unmappedids");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[(
                            "debugOptions.enableDebugging",
                            &self.debug_options_enable_debugging,
                        )]);
                        req = req.query(&[("pageSize", &self.page_size)]);
                        req = req.query(&[("pageToken", &self.page_token)]);
                        req = req.query(&[("resolutionStatusCode", &self.resolution_status_code)]);
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
    pub mod indexing {
        pub mod params {}
        pub struct IndexingActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> IndexingActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the datasources resource"]
            pub fn datasources(
                &self,
            ) -> crate::resources::indexing::datasources::DatasourcesActions {
                crate::resources::indexing::datasources::DatasourcesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod datasources {
            pub mod params {}
            pub struct DatasourcesActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> DatasourcesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Deletes the schema of a data source. **Note:** This API requires an admin or service account to execute."]
                pub fn delete_schema(&self, name: impl Into<String>) -> DeleteSchemaRequestBuilder {
                    DeleteSchemaRequestBuilder {
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
                        debug_options_enable_debugging: None,
                    }
                }
                #[doc = "Gets the schema of a data source. **Note:** This API requires an admin or service account to execute."]
                pub fn get_schema(&self, name: impl Into<String>) -> GetSchemaRequestBuilder {
                    GetSchemaRequestBuilder {
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
                        debug_options_enable_debugging: None,
                    }
                }
                #[doc = "Updates the schema of a data source. This method does not perform incremental updates to the schema. Instead, this method updates the schema by overwriting the entire schema. **Note:** This API requires an admin or service account to execute."]
                pub fn update_schema(
                    &self,
                    request: crate::schemas::UpdateSchemaRequest,
                    name: impl Into<String>,
                ) -> UpdateSchemaRequestBuilder {
                    UpdateSchemaRequestBuilder {
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
                #[doc = "Actions that can be performed on the items resource"]
                pub fn items(
                    &self,
                ) -> crate::resources::indexing::datasources::items::ItemsActions {
                    crate::resources::indexing::datasources::items::ItemsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            #[doc = "Created via [DatasourcesActions::delete_schema()](struct.DatasourcesActions.html#method.delete_schema)"]
            #[derive(Debug, Clone)]
            pub struct DeleteSchemaRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                debug_options_enable_debugging: Option<bool>,
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
            impl<'a> DeleteSchemaRequestBuilder<'a> {
                #[doc = "If you are asked by Google to help with debugging, set this field. Otherwise, ignore this field."]
                pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                    self.debug_options_enable_debugging = Some(value);
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
                ) -> Result<crate::schemas::Operation, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Operation, crate::Error> {
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/indexing/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/schema");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::DELETE, path);
                    req = req.query(&[(
                        "debugOptions.enableDebugging",
                        &self.debug_options_enable_debugging,
                    )]);
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
            #[doc = "Created via [DatasourcesActions::get_schema()](struct.DatasourcesActions.html#method.get_schema)"]
            #[derive(Debug, Clone)]
            pub struct GetSchemaRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                debug_options_enable_debugging: Option<bool>,
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
            impl<'a> GetSchemaRequestBuilder<'a> {
                #[doc = "If you are asked by Google to help with debugging, set this field. Otherwise, ignore this field."]
                pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                    self.debug_options_enable_debugging = Some(value);
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
                ) -> Result<crate::schemas::Schema, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Schema, crate::Error> {
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/indexing/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/schema");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[(
                        "debugOptions.enableDebugging",
                        &self.debug_options_enable_debugging,
                    )]);
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
            #[doc = "Created via [DatasourcesActions::update_schema()](struct.DatasourcesActions.html#method.update_schema)"]
            #[derive(Debug, Clone)]
            pub struct UpdateSchemaRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::UpdateSchemaRequest,
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
            impl<'a> UpdateSchemaRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::Operation, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Operation, crate::Error> {
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/indexing/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/schema");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::PUT, path);
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
            pub mod items {
                pub mod params {
                    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                    pub enum DeleteMode {
                        #[doc = "For changes that are executed after the response is sent back to the caller."]
                        Asynchronous,
                        #[doc = "For real-time updates."]
                        Synchronous,
                        #[doc = "Priority is not specified in the update request. Leaving priority unspecified results in an update failure."]
                        Unspecified,
                    }
                    impl DeleteMode {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                DeleteMode::Asynchronous => "ASYNCHRONOUS",
                                DeleteMode::Synchronous => "SYNCHRONOUS",
                                DeleteMode::Unspecified => "UNSPECIFIED",
                            }
                        }
                    }
                    impl ::std::convert::AsRef<str> for DeleteMode {
                        fn as_ref(&self) -> &str {
                            self.as_str()
                        }
                    }
                    impl ::std::str::FromStr for DeleteMode {
                        type Err = ();
                        fn from_str(s: &str) -> ::std::result::Result<DeleteMode, ()> {
                            Ok(match s {
                                "ASYNCHRONOUS" => DeleteMode::Asynchronous,
                                "SYNCHRONOUS" => DeleteMode::Synchronous,
                                "UNSPECIFIED" => DeleteMode::Unspecified,
                                _ => return Err(()),
                            })
                        }
                    }
                    impl ::std::fmt::Display for DeleteMode {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for DeleteMode {
                        fn serialize<S>(
                            &self,
                            serializer: S,
                        ) -> ::std::result::Result<S::Ok, S::Error>
                        where
                            S: ::serde::ser::Serializer,
                        {
                            serializer.serialize_str(self.as_str())
                        }
                    }
                    impl<'de> ::serde::Deserialize<'de> for DeleteMode {
                        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            let value: &'de str = <&str>::deserialize(deserializer)?;
                            Ok(match value {
                                "ASYNCHRONOUS" => DeleteMode::Asynchronous,
                                "SYNCHRONOUS" => DeleteMode::Synchronous,
                                "UNSPECIFIED" => DeleteMode::Unspecified,
                                _ => {
                                    return Err(::serde::de::Error::custom(format!(
                                        "invalid enum for #name: {}",
                                        value
                                    )))
                                }
                            })
                        }
                    }
                    impl ::google_field_selector::FieldSelector for DeleteMode {
                        fn fields() -> Vec<::google_field_selector::Field> {
                            Vec::new()
                        }
                    }
                    impl ::google_field_selector::ToFieldType for DeleteMode {
                        fn field_type() -> ::google_field_selector::FieldType {
                            ::google_field_selector::FieldType::Leaf
                        }
                    }
                }
                pub struct ItemsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> ItemsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Deletes Item resource for the specified resource name. This API requires an admin or service account to execute. The service account used is the one whitelisted in the corresponding data source."]
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
                            connector_name: None,
                            debug_options_enable_debugging: None,
                            mode: None,
                            version: None,
                        }
                    }
                    #[doc = "Deletes all items in a queue. This method is useful for deleting stale items. This API requires an admin or service account to execute. The service account used is the one whitelisted in the corresponding data source."]
                    pub fn delete_queue_items(
                        &self,
                        request: crate::schemas::DeleteQueueItemsRequest,
                        name: impl Into<String>,
                    ) -> DeleteQueueItemsRequestBuilder {
                        DeleteQueueItemsRequestBuilder {
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
                    #[doc = "Gets Item resource by item name. This API requires an admin or service account to execute. The service account used is the one whitelisted in the corresponding data source."]
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
                            connector_name: None,
                            debug_options_enable_debugging: None,
                        }
                    }
                    #[doc = "Updates Item ACL, metadata, and content. It will insert the Item if it does not exist. This method does not support partial updates. Fields with no provided values are cleared out in the Cloud Search index. This API requires an admin or service account to execute. The service account used is the one whitelisted in the corresponding data source."]
                    pub fn index(
                        &self,
                        request: crate::schemas::IndexItemRequest,
                        name: impl Into<String>,
                    ) -> IndexRequestBuilder {
                        IndexRequestBuilder {
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
                    #[doc = "Lists all or a subset of Item resources. This API requires an admin or service account to execute. The service account used is the one whitelisted in the corresponding data source."]
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
                            brief: None,
                            connector_name: None,
                            debug_options_enable_debugging: None,
                            page_size: None,
                            page_token: None,
                        }
                    }
                    #[doc = "Polls for unreserved items from the indexing queue and marks a set as reserved, starting with items that have the oldest timestamp from the highest priority ItemStatus. The priority order is as follows: ERROR MODIFIED NEW_ITEM ACCEPTED Reserving items ensures that polling from other threads cannot create overlapping sets. After handling the reserved items, the client should put items back into the unreserved state, either by calling index, or by calling push with the type REQUEUE. Items automatically become available (unreserved) after 4 hours even if no update or push method is called. This API requires an admin or service account to execute. The service account used is the one whitelisted in the corresponding data source."]
                    pub fn poll(
                        &self,
                        request: crate::schemas::PollItemsRequest,
                        name: impl Into<String>,
                    ) -> PollRequestBuilder {
                        PollRequestBuilder {
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
                    #[doc = "Pushes an item onto a queue for later polling and updating. This API requires an admin or service account to execute. The service account used is the one whitelisted in the corresponding data source."]
                    pub fn push(
                        &self,
                        request: crate::schemas::PushItemRequest,
                        name: impl Into<String>,
                    ) -> PushRequestBuilder {
                        PushRequestBuilder {
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
                    #[doc = "Unreserves all items from a queue, making them all eligible to be polled. This method is useful for resetting the indexing queue after a connector has been restarted. This API requires an admin or service account to execute. The service account used is the one whitelisted in the corresponding data source."]
                    pub fn unreserve(
                        &self,
                        request: crate::schemas::UnreserveItemsRequest,
                        name: impl Into<String>,
                    ) -> UnreserveRequestBuilder {
                        UnreserveRequestBuilder {
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
                    #[doc = "Creates an upload session for uploading item content. For items smaller than 100 KB, it's easier to embed the content inline within an index request. This API requires an admin or service account to execute. The service account used is the one whitelisted in the corresponding data source."]
                    pub fn upload(
                        &self,
                        request: crate::schemas::StartUploadItemRequest,
                        name: impl Into<String>,
                    ) -> UploadRequestBuilder {
                        UploadRequestBuilder {
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
                #[doc = "Created via [ItemsActions::delete()](struct.ItemsActions.html#method.delete)"]
                #[derive(Debug, Clone)]
                pub struct DeleteRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    connector_name: Option<String>,
                    debug_options_enable_debugging: Option<bool>,
                    mode:
                        Option<crate::resources::indexing::datasources::items::params::DeleteMode>,
                    version: Option<::google_api_bytes::Bytes>,
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
                    #[doc = "Name of connector making this call. Format: datasources/{source_id}/connectors/{ID}"]
                    pub fn connector_name(mut self, value: impl Into<String>) -> Self {
                        self.connector_name = Some(value.into());
                        self
                    }
                    #[doc = "If you are asked by Google to help with debugging, set this field. Otherwise, ignore this field."]
                    pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                        self.debug_options_enable_debugging = Some(value);
                        self
                    }
                    #[doc = "Required. The RequestMode for this request."]
                    pub fn mode(
                        mut self,
                        value: crate::resources::indexing::datasources::items::params::DeleteMode,
                    ) -> Self {
                        self.mode = Some(value);
                        self
                    }
                    #[doc = "Required. The incremented version of the item to delete from the index. The indexing system stores the version from the datasource as a byte string and compares the Item version in the index to the version of the queued Item using lexical ordering. Cloud Search Indexing won't delete any queued item with a version value that is less than or equal to the version of the currently indexed item. The maximum length for this field is 1024 bytes. For information on how item version affects the deletion process, refer to [Handle revisions after manual deletes](https://developers.google.com/cloud-search/docs/guides/operations)."]
                    pub fn version(mut self, value: impl Into<Vec<u8>>) -> Self {
                        let v: Vec<u8> = value.into();
                        self.version = Some(v.into());
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
                    ) -> Result<crate::schemas::Operation, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Operation, crate::Error> {
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/indexing/");
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
                        let mut req = self.reqwest.request(::reqwest::Method::DELETE, path);
                        req = req.query(&[("connectorName", &self.connector_name)]);
                        req = req.query(&[(
                            "debugOptions.enableDebugging",
                            &self.debug_options_enable_debugging,
                        )]);
                        req = req.query(&[("mode", &self.mode)]);
                        req = req.query(&[("version", &self.version)]);
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
                #[doc = "Created via [ItemsActions::delete_queue_items()](struct.ItemsActions.html#method.delete_queue_items)"]
                #[derive(Debug, Clone)]
                pub struct DeleteQueueItemsRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::DeleteQueueItemsRequest,
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
                impl<'a> DeleteQueueItemsRequestBuilder<'a> {
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
                    ) -> Result<crate::schemas::Operation, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Operation, crate::Error> {
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/indexing/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/items:deleteQueueItems");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                #[doc = "Created via [ItemsActions::get()](struct.ItemsActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    connector_name: Option<String>,
                    debug_options_enable_debugging: Option<bool>,
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
                    #[doc = "Name of connector making this call. Format: datasources/{source_id}/connectors/{ID}"]
                    pub fn connector_name(mut self, value: impl Into<String>) -> Self {
                        self.connector_name = Some(value.into());
                        self
                    }
                    #[doc = "If you are asked by Google to help with debugging, set this field. Otherwise, ignore this field."]
                    pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                        self.debug_options_enable_debugging = Some(value);
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
                    ) -> Result<crate::schemas::Item, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Item, crate::Error> {
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/indexing/");
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
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[("connectorName", &self.connector_name)]);
                        req = req.query(&[(
                            "debugOptions.enableDebugging",
                            &self.debug_options_enable_debugging,
                        )]);
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
                #[doc = "Created via [ItemsActions::index()](struct.ItemsActions.html#method.index)"]
                #[derive(Debug, Clone)]
                pub struct IndexRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::IndexItemRequest,
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
                impl<'a> IndexRequestBuilder<'a> {
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
                    ) -> Result<crate::schemas::Operation, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Operation, crate::Error> {
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/indexing/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":index");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                #[doc = "Created via [ItemsActions::list()](struct.ItemsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    brief: Option<bool>,
                    connector_name: Option<String>,
                    debug_options_enable_debugging: Option<bool>,
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
                    #[doc = "When set to true, the indexing system only populates the following fields: name, version, queue. metadata.hash, metadata.title, metadata.sourceRepositoryURL, metadata.objectType, metadata.createTime, metadata.updateTime, metadata.contentLanguage, metadata.mimeType, structured_data.hash, content.hash, itemType, itemStatus.code, itemStatus.processingError.code, itemStatus.repositoryError.type, If this value is false, then all the fields are populated in Item."]
                    pub fn brief(mut self, value: bool) -> Self {
                        self.brief = Some(value);
                        self
                    }
                    #[doc = "Name of connector making this call. Format: datasources/{source_id}/connectors/{ID}"]
                    pub fn connector_name(mut self, value: impl Into<String>) -> Self {
                        self.connector_name = Some(value.into());
                        self
                    }
                    #[doc = "If you are asked by Google to help with debugging, set this field. Otherwise, ignore this field."]
                    pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                        self.debug_options_enable_debugging = Some(value);
                        self
                    }
                    #[doc = "Maximum number of items to fetch in a request. The max value is 1000 when brief is true. The max value is 10 if brief is false. The default value is 10"]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "The next_page_token value returned from a previous List request, if any."]
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
                    ) -> Result<crate::schemas::ListItemsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListItemsResponse, crate::Error>
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/indexing/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/items");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[("brief", &self.brief)]);
                        req = req.query(&[("connectorName", &self.connector_name)]);
                        req = req.query(&[(
                            "debugOptions.enableDebugging",
                            &self.debug_options_enable_debugging,
                        )]);
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
                #[doc = "Created via [ItemsActions::poll()](struct.ItemsActions.html#method.poll)"]
                #[derive(Debug, Clone)]
                pub struct PollRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::PollItemsRequest,
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
                impl<'a> PollRequestBuilder<'a> {
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
                    ) -> Result<crate::schemas::PollItemsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::PollItemsResponse, crate::Error>
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/indexing/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/items:poll");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                #[doc = "Created via [ItemsActions::push()](struct.ItemsActions.html#method.push)"]
                #[derive(Debug, Clone)]
                pub struct PushRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::PushItemRequest,
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
                impl<'a> PushRequestBuilder<'a> {
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
                    ) -> Result<crate::schemas::Item, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Item, crate::Error> {
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/indexing/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":push");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                #[doc = "Created via [ItemsActions::unreserve()](struct.ItemsActions.html#method.unreserve)"]
                #[derive(Debug, Clone)]
                pub struct UnreserveRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::UnreserveItemsRequest,
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
                impl<'a> UnreserveRequestBuilder<'a> {
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
                    ) -> Result<crate::schemas::Operation, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::Operation, crate::Error> {
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/indexing/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/items:unreserve");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
                #[doc = "Created via [ItemsActions::upload()](struct.ItemsActions.html#method.upload)"]
                #[derive(Debug, Clone)]
                pub struct UploadRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    request: crate::schemas::StartUploadItemRequest,
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
                impl<'a> UploadRequestBuilder<'a> {
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
                    ) -> Result<crate::schemas::UploadItemRef, crate::Error> {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::UploadItemRef, crate::Error> {
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/indexing/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":upload");
                        output
                    }
                    async fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
    }
    pub mod media {
        pub mod params {}
        pub struct MediaActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> MediaActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Uploads media for indexing. The upload endpoint supports direct and resumable upload protocols and is intended for large items that can not be [inlined during index requests](https://developers.google.com/cloud-search/docs/reference/rest/v1/indexing.datasources.items#itemcontent). To index large content: 1. Call indexing.datasources.items.upload with the item name to begin an upload session and retrieve the UploadItemRef. 1. Call media.upload to upload the content, as a streaming request, using the same resource name from the UploadItemRef from step 1. 1. Call indexing.datasources.items.index to index the item. Populate the [ItemContent](/cloud-search/docs/reference/rest/v1/indexing.datasources.items#ItemContent) with the UploadItemRef from step 1. For additional information, see [Create a content connector using the REST API](https://developers.google.com/cloud-search/docs/guides/content-connector#rest). **Note:** This API requires a service account to execute."]
            pub fn upload(
                &self,
                request: crate::schemas::Media,
                resource_name: impl Into<String>,
            ) -> UploadRequestBuilder {
                UploadRequestBuilder {
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
                    resource_name: resource_name.into(),
                }
            }
        }
        #[doc = "Created via [MediaActions::upload()](struct.MediaActions.html#method.upload)"]
        #[derive(Debug, Clone)]
        pub struct UploadRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Media,
            resource_name: String,
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
        impl<'a> UploadRequestBuilder<'a> {
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
            fn _simple_upload_path(&self) -> String {
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("upload/v1/media/");
                {
                    let var_as_str = &self.resource_name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            pub async fn upload<T, R>(
                mut self,
                content: R,
                mime_type: ::mime::Mime,
            ) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                R: futures::io::AsyncRead + std::marker::Unpin + Send + 'static,
            {
                use crate::multipart::{Part, RelatedMultiPart};
                use futures::io::AsyncReadExt;
                let fields = ::google_field_selector::to_string::<T>();
                self.fields = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                let req = self._request(&self._simple_upload_path()).await?;
                let req = req.query(&[("uploadType", "multipart")]);
                let mut multipart = RelatedMultiPart::new();
                let request_json = ::serde_json::to_vec(&self.request)?;
                multipart.new_part(Part::new(
                    ::mime::APPLICATION_JSON,
                    Box::new(futures::io::Cursor::new(request_json)),
                ));
                multipart.new_part(Part::new(mime_type, Box::new(content)));
                let req = req.header(
                    ::reqwest::header::CONTENT_TYPE,
                    format!("multipart/related; boundary={}", multipart.boundary()),
                );
                let mut body: Vec<u8> = vec![];
                let mut reader = multipart.into_reader();
                let _num_bytes = reader.read_to_end(&mut body).await?;
                let req = req.body(body);
                let response = req.send().await?.error_for_status()?;
                Ok(response.json().await?)
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
            ) -> Result<crate::schemas::Media, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Media, crate::Error> {
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/media/");
                {
                    let var_as_str = &self.resource_name;
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
            #[doc = "Actions that can be performed on the lro resource"]
            pub fn lro(&self) -> crate::resources::operations::lro::LroActions {
                crate::resources::operations::lro::LroActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
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
            ) -> Result<crate::schemas::Operation, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Operation, crate::Error> {
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/");
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
        pub mod lro {
            pub mod params {}
            pub struct LroActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> LroActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
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
            #[doc = "Created via [LroActions::list()](struct.LroActions.html#method.list)"]
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
                ) -> Result<crate::schemas::ListOperationsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListOperationsResponse, crate::Error> {
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/lro");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
    pub mod query {
        pub mod params {}
        pub struct QueryActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> QueryActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "The Cloud Search Query API provides the search method, which returns the most relevant results from a user query. The results can come from Google Workspace apps, such as Gmail or Google Drive, or they can come from data that you have indexed from a third party. **Note:** This API requires a standard end user account to execute. A service account can't perform Query API requests directly; to use a service account to perform queries, set up [Google Workspace domain-wide delegation of authority](https://developers.google.com/cloud-search/docs/guides/delegation/)."]
            pub fn search(&self, request: crate::schemas::SearchRequest) -> SearchRequestBuilder {
                SearchRequestBuilder {
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
                }
            }
            #[doc = "Provides suggestions for autocompleting the query. **Note:** This API requires a standard end user account to execute. A service account can't perform Query API requests directly; to use a service account to perform queries, set up [Google Workspace domain-wide delegation of authority](https://developers.google.com/cloud-search/docs/guides/delegation/)."]
            pub fn suggest(
                &self,
                request: crate::schemas::SuggestRequest,
            ) -> SuggestRequestBuilder {
                SuggestRequestBuilder {
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
                }
            }
            #[doc = "Actions that can be performed on the sources resource"]
            pub fn sources(&self) -> crate::resources::query::sources::SourcesActions {
                crate::resources::query::sources::SourcesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [QueryActions::search()](struct.QueryActions.html#method.search)"]
        #[derive(Debug, Clone)]
        pub struct SearchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::SearchRequest,
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
        impl<'a> SearchRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::SearchResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::SearchResponse, crate::Error> {
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/query/search");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
        #[doc = "Created via [QueryActions::suggest()](struct.QueryActions.html#method.suggest)"]
        #[derive(Debug, Clone)]
        pub struct SuggestRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::SuggestRequest,
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
        impl<'a> SuggestRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::SuggestResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::SuggestResponse, crate::Error> {
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/query/suggest");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
        pub mod sources {
            pub mod params {}
            pub struct SourcesActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> SourcesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Returns list of sources that user can use for Search and Suggest APIs. **Note:** This API requires a standard end user account to execute. A service account can't perform Query API requests directly; to use a service account to perform queries, set up [Google Workspace domain-wide delegation of authority](https://developers.google.com/cloud-search/docs/guides/delegation/)."]
                pub fn list(&self) -> ListRequestBuilder {
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
                        page_token: None,
                        request_options_debug_options_enable_debugging: None,
                        request_options_language_code: None,
                        request_options_search_application_id: None,
                        request_options_time_zone: None,
                    }
                }
            }
            #[doc = "Created via [SourcesActions::list()](struct.SourcesActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                page_token: Option<String>,
                request_options_debug_options_enable_debugging: Option<bool>,
                request_options_language_code: Option<String>,
                request_options_search_application_id: Option<String>,
                request_options_time_zone: Option<String>,
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
                #[doc = "Number of sources to return in the response."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "If you are asked by Google to help with debugging, set this field. Otherwise, ignore this field."]
                pub fn request_options_debug_options_enable_debugging(
                    mut self,
                    value: bool,
                ) -> Self {
                    self.request_options_debug_options_enable_debugging = Some(value);
                    self
                }
                #[doc = "The BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier. For translations. Set this field using the language set in browser or for the page. In the event that the user's language preference is known, set this field to the known user language. When specified, the documents in search results are biased towards the specified language. The suggest API does not use this parameter. Instead, suggest autocompletes only based on characters in the query."]
                pub fn request_options_language_code(mut self, value: impl Into<String>) -> Self {
                    self.request_options_language_code = Some(value.into());
                    self
                }
                #[doc = "The ID generated when you create a search application using the [admin console](https://support.google.com/a/answer/9043922)."]
                pub fn request_options_search_application_id(
                    mut self,
                    value: impl Into<String>,
                ) -> Self {
                    self.request_options_search_application_id = Some(value.into());
                    self
                }
                #[doc = "Current user's time zone id, such as \"America/Los_Angeles\" or \"Australia/Sydney\". These IDs are defined by [Unicode Common Locale Data Repository (CLDR)](http://cldr.unicode.org/) project, and currently available in the file [timezone.xml](http://unicode.org/repos/cldr/trunk/common/bcp47/timezone.xml). This field is used to correctly interpret date and time queries. If this field is not specified, the default time zone (UTC) is used."]
                pub fn request_options_time_zone(mut self, value: impl Into<String>) -> Self {
                    self.request_options_time_zone = Some(value.into());
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
                ) -> Result<crate::schemas::ListQuerySourcesResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListQuerySourcesResponse, crate::Error>
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/query/sources");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("pageToken", &self.page_token)]);
                    req = req.query(&[(
                        "requestOptions.debugOptions.enableDebugging",
                        &self.request_options_debug_options_enable_debugging,
                    )]);
                    req = req.query(&[(
                        "requestOptions.languageCode",
                        &self.request_options_language_code,
                    )]);
                    req = req.query(&[(
                        "requestOptions.searchApplicationId",
                        &self.request_options_search_application_id,
                    )]);
                    req =
                        req.query(&[("requestOptions.timeZone", &self.request_options_time_zone)]);
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
    pub mod settings {
        pub mod params {}
        pub struct SettingsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> SettingsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Get customer settings. **Note:** This API requires an admin account to execute."]
            pub fn get_customer(&self) -> GetCustomerRequestBuilder {
                GetCustomerRequestBuilder {
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
                }
            }
            #[doc = "Update customer settings. **Note:** This API requires an admin account to execute."]
            pub fn update_customer(
                &self,
                request: crate::schemas::CustomerSettings,
            ) -> UpdateCustomerRequestBuilder {
                UpdateCustomerRequestBuilder {
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
                    update_mask: None,
                }
            }
            #[doc = "Actions that can be performed on the datasources resource"]
            pub fn datasources(
                &self,
            ) -> crate::resources::settings::datasources::DatasourcesActions {
                crate::resources::settings::datasources::DatasourcesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the searchapplications resource"]
            pub fn searchapplications(
                &self,
            ) -> crate::resources::settings::searchapplications::SearchapplicationsActions
            {
                crate::resources::settings::searchapplications::SearchapplicationsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [SettingsActions::get_customer()](struct.SettingsActions.html#method.get_customer)"]
        #[derive(Debug, Clone)]
        pub struct GetCustomerRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> GetCustomerRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::CustomerSettings, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::CustomerSettings, crate::Error> {
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/settings/customer");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
        #[doc = "Created via [SettingsActions::update_customer()](struct.SettingsActions.html#method.update_customer)"]
        #[derive(Debug, Clone)]
        pub struct UpdateCustomerRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::CustomerSettings,
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
        impl<'a> UpdateCustomerRequestBuilder<'a> {
            #[doc = "Update mask to control which fields get updated. If you specify a field in the update_mask but don't specify its value here, that field will be cleared. If the mask is not present or empty, all fields will be updated. Currently supported field paths: vpc_settings and audit_logging_settings"]
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
            ) -> Result<crate::schemas::Operation, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Operation, crate::Error> {
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/settings/customer");
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
        pub mod datasources {
            pub mod params {}
            pub struct DatasourcesActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> DatasourcesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a datasource. **Note:** This API requires an admin account to execute."]
                pub fn create(&self, request: crate::schemas::DataSource) -> CreateRequestBuilder {
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
                    }
                }
                #[doc = "Deletes a datasource. **Note:** This API requires an admin account to execute."]
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
                        debug_options_enable_debugging: None,
                    }
                }
                #[doc = "Gets a datasource. **Note:** This API requires an admin account to execute."]
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
                        debug_options_enable_debugging: None,
                    }
                }
                #[doc = "Lists datasources. **Note:** This API requires an admin account to execute."]
                pub fn list(&self) -> ListRequestBuilder {
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
                        debug_options_enable_debugging: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Updates a datasource. **Note:** This API requires an admin account to execute."]
                pub fn update(
                    &self,
                    request: crate::schemas::UpdateDataSourceRequest,
                    name: impl Into<String>,
                ) -> UpdateRequestBuilder {
                    UpdateRequestBuilder {
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
            #[doc = "Created via [DatasourcesActions::create()](struct.DatasourcesActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::DataSource,
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
                ) -> Result<crate::schemas::Operation, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Operation, crate::Error> {
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/settings/datasources");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
            #[doc = "Created via [DatasourcesActions::delete()](struct.DatasourcesActions.html#method.delete)"]
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                debug_options_enable_debugging: Option<bool>,
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
                #[doc = "If you are asked by Google to help with debugging, set this field. Otherwise, ignore this field."]
                pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                    self.debug_options_enable_debugging = Some(value);
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
                ) -> Result<crate::schemas::Operation, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Operation, crate::Error> {
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/settings/");
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
                    let mut req = self.reqwest.request(::reqwest::Method::DELETE, path);
                    req = req.query(&[(
                        "debugOptions.enableDebugging",
                        &self.debug_options_enable_debugging,
                    )]);
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
            #[doc = "Created via [DatasourcesActions::get()](struct.DatasourcesActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                debug_options_enable_debugging: Option<bool>,
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
                #[doc = "If you are asked by Google to help with debugging, set this field. Otherwise, ignore this field."]
                pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                    self.debug_options_enable_debugging = Some(value);
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
                ) -> Result<crate::schemas::DataSource, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::DataSource, crate::Error> {
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/settings/");
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
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[(
                        "debugOptions.enableDebugging",
                        &self.debug_options_enable_debugging,
                    )]);
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
            #[doc = "Created via [DatasourcesActions::list()](struct.DatasourcesActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                debug_options_enable_debugging: Option<bool>,
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
                #[doc = "If you are asked by Google to help with debugging, set this field. Otherwise, ignore this field."]
                pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                    self.debug_options_enable_debugging = Some(value);
                    self
                }
                #[doc = "Maximum number of datasources to fetch in a request. The max value is 1000. The default value is 1000."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Starting index of the results."]
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
                ) -> Result<crate::schemas::ListDataSourceResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListDataSourceResponse, crate::Error> {
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/settings/datasources");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[(
                        "debugOptions.enableDebugging",
                        &self.debug_options_enable_debugging,
                    )]);
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
            #[doc = "Created via [DatasourcesActions::update()](struct.DatasourcesActions.html#method.update)"]
            #[derive(Debug, Clone)]
            pub struct UpdateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::UpdateDataSourceRequest,
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
            impl<'a> UpdateRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::Operation, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Operation, crate::Error> {
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/settings/");
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
                    let mut req = self.reqwest.request(::reqwest::Method::PUT, path);
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
        pub mod searchapplications {
            pub mod params {}
            pub struct SearchapplicationsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> SearchapplicationsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates a search application. **Note:** This API requires an admin account to execute."]
                pub fn create(
                    &self,
                    request: crate::schemas::SearchApplication,
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
                    }
                }
                #[doc = "Deletes a search application. **Note:** This API requires an admin account to execute."]
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
                        debug_options_enable_debugging: None,
                    }
                }
                #[doc = "Gets the specified search application. **Note:** This API requires an admin account to execute."]
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
                        debug_options_enable_debugging: None,
                    }
                }
                #[doc = "Lists all search applications. **Note:** This API requires an admin account to execute."]
                pub fn list(&self) -> ListRequestBuilder {
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
                        debug_options_enable_debugging: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Resets a search application to default settings. This will return an empty response. **Note:** This API requires an admin account to execute."]
                pub fn reset(
                    &self,
                    request: crate::schemas::ResetSearchApplicationRequest,
                    name: impl Into<String>,
                ) -> ResetRequestBuilder {
                    ResetRequestBuilder {
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
                #[doc = "Updates a search application. **Note:** This API requires an admin account to execute."]
                pub fn update(
                    &self,
                    request: crate::schemas::SearchApplication,
                    name: impl Into<String>,
                ) -> UpdateRequestBuilder {
                    UpdateRequestBuilder {
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
            #[doc = "Created via [SearchapplicationsActions::create()](struct.SearchapplicationsActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::SearchApplication,
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
                ) -> Result<crate::schemas::Operation, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Operation, crate::Error> {
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/settings/searchapplications");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
            #[doc = "Created via [SearchapplicationsActions::delete()](struct.SearchapplicationsActions.html#method.delete)"]
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                debug_options_enable_debugging: Option<bool>,
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
                #[doc = "If you are asked by Google to help with debugging, set this field. Otherwise, ignore this field."]
                pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                    self.debug_options_enable_debugging = Some(value);
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
                ) -> Result<crate::schemas::Operation, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Operation, crate::Error> {
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/settings/");
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
                    let mut req = self.reqwest.request(::reqwest::Method::DELETE, path);
                    req = req.query(&[(
                        "debugOptions.enableDebugging",
                        &self.debug_options_enable_debugging,
                    )]);
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
            #[doc = "Created via [SearchapplicationsActions::get()](struct.SearchapplicationsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                name: String,
                debug_options_enable_debugging: Option<bool>,
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
                #[doc = "If you are asked by Google to help with debugging, set this field. Otherwise, ignore this field."]
                pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                    self.debug_options_enable_debugging = Some(value);
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
                ) -> Result<crate::schemas::SearchApplication, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::SearchApplication, crate::Error> {
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/settings/");
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
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[(
                        "debugOptions.enableDebugging",
                        &self.debug_options_enable_debugging,
                    )]);
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
            #[doc = "Created via [SearchapplicationsActions::list()](struct.SearchapplicationsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                debug_options_enable_debugging: Option<bool>,
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
                #[doc = "If you are asked by Google to help with debugging, set this field. Otherwise, ignore this field."]
                pub fn debug_options_enable_debugging(mut self, value: bool) -> Self {
                    self.debug_options_enable_debugging = Some(value);
                    self
                }
                #[doc = "The maximum number of items to return."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "The next_page_token value returned from a previous List request, if any. The default value is 10"]
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
                ) -> Result<crate::schemas::ListSearchApplicationsResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListSearchApplicationsResponse, crate::Error>
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/settings/searchapplications");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[(
                        "debugOptions.enableDebugging",
                        &self.debug_options_enable_debugging,
                    )]);
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
            #[doc = "Created via [SearchapplicationsActions::reset()](struct.SearchapplicationsActions.html#method.reset)"]
            #[derive(Debug, Clone)]
            pub struct ResetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::ResetSearchApplicationRequest,
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
            impl<'a> ResetRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::Operation, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Operation, crate::Error> {
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/settings/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":reset");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
            #[doc = "Created via [SearchapplicationsActions::update()](struct.SearchapplicationsActions.html#method.update)"]
            #[derive(Debug, Clone)]
            pub struct UpdateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::SearchApplication,
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
            impl<'a> UpdateRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::Operation, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Operation, crate::Error> {
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
                    let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                    output.push_str("v1/settings/");
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
                    let mut req = self.reqwest.request(::reqwest::Method::PUT, path);
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
    pub mod stats {
        pub mod params {}
        pub struct StatsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> StatsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Gets indexed item statistics aggreggated across all data sources. This API only returns statistics for previous dates; it doesn't return statistics for the current day. **Note:** This API requires a standard end user account to execute."]
            pub fn get_index(&self) -> GetIndexRequestBuilder {
                GetIndexRequestBuilder {
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
                    from_date_day: None,
                    from_date_month: None,
                    from_date_year: None,
                    to_date_day: None,
                    to_date_month: None,
                    to_date_year: None,
                }
            }
            #[doc = "Get the query statistics for customer. **Note:** This API requires a standard end user account to execute."]
            pub fn get_query(&self) -> GetQueryRequestBuilder {
                GetQueryRequestBuilder {
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
                    from_date_day: None,
                    from_date_month: None,
                    from_date_year: None,
                    to_date_day: None,
                    to_date_month: None,
                    to_date_year: None,
                }
            }
            #[doc = "Get search application stats for customer. **Note:** This API requires a standard end user account to execute."]
            pub fn get_searchapplication(&self) -> GetSearchapplicationRequestBuilder {
                GetSearchapplicationRequestBuilder {
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
                    end_date_day: None,
                    end_date_month: None,
                    end_date_year: None,
                    start_date_day: None,
                    start_date_month: None,
                    start_date_year: None,
                }
            }
            #[doc = "Get the # of search sessions, % of successful sessions with a click query statistics for customer. **Note:** This API requires a standard end user account to execute."]
            pub fn get_session(&self) -> GetSessionRequestBuilder {
                GetSessionRequestBuilder {
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
                    from_date_day: None,
                    from_date_month: None,
                    from_date_year: None,
                    to_date_day: None,
                    to_date_month: None,
                    to_date_year: None,
                }
            }
            #[doc = "Get the users statistics for customer. **Note:** This API requires a standard end user account to execute."]
            pub fn get_user(&self) -> GetUserRequestBuilder {
                GetUserRequestBuilder {
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
                    from_date_day: None,
                    from_date_month: None,
                    from_date_year: None,
                    to_date_day: None,
                    to_date_month: None,
                    to_date_year: None,
                }
            }
            #[doc = "Actions that can be performed on the index resource"]
            pub fn index(&self) -> crate::resources::stats::index::IndexActions {
                crate::resources::stats::index::IndexActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the query resource"]
            pub fn query(&self) -> crate::resources::stats::query::QueryActions {
                crate::resources::stats::query::QueryActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the session resource"]
            pub fn session(&self) -> crate::resources::stats::session::SessionActions {
                crate::resources::stats::session::SessionActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the user resource"]
            pub fn user(&self) -> crate::resources::stats::user::UserActions {
                crate::resources::stats::user::UserActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [StatsActions::get_index()](struct.StatsActions.html#method.get_index)"]
        #[derive(Debug, Clone)]
        pub struct GetIndexRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            from_date_day: Option<i32>,
            from_date_month: Option<i32>,
            from_date_year: Option<i32>,
            to_date_day: Option<i32>,
            to_date_month: Option<i32>,
            to_date_year: Option<i32>,
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
        impl<'a> GetIndexRequestBuilder<'a> {
            #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
            pub fn from_date_day(mut self, value: i32) -> Self {
                self.from_date_day = Some(value);
                self
            }
            #[doc = "Month of date. Must be from 1 to 12."]
            pub fn from_date_month(mut self, value: i32) -> Self {
                self.from_date_month = Some(value);
                self
            }
            #[doc = "Year of date. Must be from 1 to 9999."]
            pub fn from_date_year(mut self, value: i32) -> Self {
                self.from_date_year = Some(value);
                self
            }
            #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
            pub fn to_date_day(mut self, value: i32) -> Self {
                self.to_date_day = Some(value);
                self
            }
            #[doc = "Month of date. Must be from 1 to 12."]
            pub fn to_date_month(mut self, value: i32) -> Self {
                self.to_date_month = Some(value);
                self
            }
            #[doc = "Year of date. Must be from 1 to 9999."]
            pub fn to_date_year(mut self, value: i32) -> Self {
                self.to_date_year = Some(value);
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
            ) -> Result<crate::schemas::GetCustomerIndexStatsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GetCustomerIndexStatsResponse, crate::Error> {
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/stats/index");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("fromDate.day", &self.from_date_day)]);
                req = req.query(&[("fromDate.month", &self.from_date_month)]);
                req = req.query(&[("fromDate.year", &self.from_date_year)]);
                req = req.query(&[("toDate.day", &self.to_date_day)]);
                req = req.query(&[("toDate.month", &self.to_date_month)]);
                req = req.query(&[("toDate.year", &self.to_date_year)]);
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
        #[doc = "Created via [StatsActions::get_query()](struct.StatsActions.html#method.get_query)"]
        #[derive(Debug, Clone)]
        pub struct GetQueryRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            from_date_day: Option<i32>,
            from_date_month: Option<i32>,
            from_date_year: Option<i32>,
            to_date_day: Option<i32>,
            to_date_month: Option<i32>,
            to_date_year: Option<i32>,
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
        impl<'a> GetQueryRequestBuilder<'a> {
            #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
            pub fn from_date_day(mut self, value: i32) -> Self {
                self.from_date_day = Some(value);
                self
            }
            #[doc = "Month of date. Must be from 1 to 12."]
            pub fn from_date_month(mut self, value: i32) -> Self {
                self.from_date_month = Some(value);
                self
            }
            #[doc = "Year of date. Must be from 1 to 9999."]
            pub fn from_date_year(mut self, value: i32) -> Self {
                self.from_date_year = Some(value);
                self
            }
            #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
            pub fn to_date_day(mut self, value: i32) -> Self {
                self.to_date_day = Some(value);
                self
            }
            #[doc = "Month of date. Must be from 1 to 12."]
            pub fn to_date_month(mut self, value: i32) -> Self {
                self.to_date_month = Some(value);
                self
            }
            #[doc = "Year of date. Must be from 1 to 9999."]
            pub fn to_date_year(mut self, value: i32) -> Self {
                self.to_date_year = Some(value);
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
            ) -> Result<crate::schemas::GetCustomerQueryStatsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GetCustomerQueryStatsResponse, crate::Error> {
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/stats/query");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("fromDate.day", &self.from_date_day)]);
                req = req.query(&[("fromDate.month", &self.from_date_month)]);
                req = req.query(&[("fromDate.year", &self.from_date_year)]);
                req = req.query(&[("toDate.day", &self.to_date_day)]);
                req = req.query(&[("toDate.month", &self.to_date_month)]);
                req = req.query(&[("toDate.year", &self.to_date_year)]);
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
        #[doc = "Created via [StatsActions::get_searchapplication()](struct.StatsActions.html#method.get_searchapplication)"]
        #[derive(Debug, Clone)]
        pub struct GetSearchapplicationRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            end_date_day: Option<i32>,
            end_date_month: Option<i32>,
            end_date_year: Option<i32>,
            start_date_day: Option<i32>,
            start_date_month: Option<i32>,
            start_date_year: Option<i32>,
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
        impl<'a> GetSearchapplicationRequestBuilder<'a> {
            #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
            pub fn end_date_day(mut self, value: i32) -> Self {
                self.end_date_day = Some(value);
                self
            }
            #[doc = "Month of date. Must be from 1 to 12."]
            pub fn end_date_month(mut self, value: i32) -> Self {
                self.end_date_month = Some(value);
                self
            }
            #[doc = "Year of date. Must be from 1 to 9999."]
            pub fn end_date_year(mut self, value: i32) -> Self {
                self.end_date_year = Some(value);
                self
            }
            #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
            pub fn start_date_day(mut self, value: i32) -> Self {
                self.start_date_day = Some(value);
                self
            }
            #[doc = "Month of date. Must be from 1 to 12."]
            pub fn start_date_month(mut self, value: i32) -> Self {
                self.start_date_month = Some(value);
                self
            }
            #[doc = "Year of date. Must be from 1 to 9999."]
            pub fn start_date_year(mut self, value: i32) -> Self {
                self.start_date_year = Some(value);
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
            ) -> Result<crate::schemas::GetCustomerSearchApplicationStatsResponse, crate::Error>
            {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GetCustomerSearchApplicationStatsResponse, crate::Error>
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/stats/searchapplication");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("endDate.day", &self.end_date_day)]);
                req = req.query(&[("endDate.month", &self.end_date_month)]);
                req = req.query(&[("endDate.year", &self.end_date_year)]);
                req = req.query(&[("startDate.day", &self.start_date_day)]);
                req = req.query(&[("startDate.month", &self.start_date_month)]);
                req = req.query(&[("startDate.year", &self.start_date_year)]);
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
        #[doc = "Created via [StatsActions::get_session()](struct.StatsActions.html#method.get_session)"]
        #[derive(Debug, Clone)]
        pub struct GetSessionRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            from_date_day: Option<i32>,
            from_date_month: Option<i32>,
            from_date_year: Option<i32>,
            to_date_day: Option<i32>,
            to_date_month: Option<i32>,
            to_date_year: Option<i32>,
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
        impl<'a> GetSessionRequestBuilder<'a> {
            #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
            pub fn from_date_day(mut self, value: i32) -> Self {
                self.from_date_day = Some(value);
                self
            }
            #[doc = "Month of date. Must be from 1 to 12."]
            pub fn from_date_month(mut self, value: i32) -> Self {
                self.from_date_month = Some(value);
                self
            }
            #[doc = "Year of date. Must be from 1 to 9999."]
            pub fn from_date_year(mut self, value: i32) -> Self {
                self.from_date_year = Some(value);
                self
            }
            #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
            pub fn to_date_day(mut self, value: i32) -> Self {
                self.to_date_day = Some(value);
                self
            }
            #[doc = "Month of date. Must be from 1 to 12."]
            pub fn to_date_month(mut self, value: i32) -> Self {
                self.to_date_month = Some(value);
                self
            }
            #[doc = "Year of date. Must be from 1 to 9999."]
            pub fn to_date_year(mut self, value: i32) -> Self {
                self.to_date_year = Some(value);
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
            ) -> Result<crate::schemas::GetCustomerSessionStatsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GetCustomerSessionStatsResponse, crate::Error> {
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/stats/session");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("fromDate.day", &self.from_date_day)]);
                req = req.query(&[("fromDate.month", &self.from_date_month)]);
                req = req.query(&[("fromDate.year", &self.from_date_year)]);
                req = req.query(&[("toDate.day", &self.to_date_day)]);
                req = req.query(&[("toDate.month", &self.to_date_month)]);
                req = req.query(&[("toDate.year", &self.to_date_year)]);
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
        #[doc = "Created via [StatsActions::get_user()](struct.StatsActions.html#method.get_user)"]
        #[derive(Debug, Clone)]
        pub struct GetUserRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            from_date_day: Option<i32>,
            from_date_month: Option<i32>,
            from_date_year: Option<i32>,
            to_date_day: Option<i32>,
            to_date_month: Option<i32>,
            to_date_year: Option<i32>,
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
        impl<'a> GetUserRequestBuilder<'a> {
            #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
            pub fn from_date_day(mut self, value: i32) -> Self {
                self.from_date_day = Some(value);
                self
            }
            #[doc = "Month of date. Must be from 1 to 12."]
            pub fn from_date_month(mut self, value: i32) -> Self {
                self.from_date_month = Some(value);
                self
            }
            #[doc = "Year of date. Must be from 1 to 9999."]
            pub fn from_date_year(mut self, value: i32) -> Self {
                self.from_date_year = Some(value);
                self
            }
            #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
            pub fn to_date_day(mut self, value: i32) -> Self {
                self.to_date_day = Some(value);
                self
            }
            #[doc = "Month of date. Must be from 1 to 12."]
            pub fn to_date_month(mut self, value: i32) -> Self {
                self.to_date_month = Some(value);
                self
            }
            #[doc = "Year of date. Must be from 1 to 9999."]
            pub fn to_date_year(mut self, value: i32) -> Self {
                self.to_date_year = Some(value);
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
            ) -> Result<crate::schemas::GetCustomerUserStatsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GetCustomerUserStatsResponse, crate::Error> {
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1/stats/user");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("fromDate.day", &self.from_date_day)]);
                req = req.query(&[("fromDate.month", &self.from_date_month)]);
                req = req.query(&[("fromDate.year", &self.from_date_year)]);
                req = req.query(&[("toDate.day", &self.to_date_day)]);
                req = req.query(&[("toDate.month", &self.to_date_month)]);
                req = req.query(&[("toDate.year", &self.to_date_year)]);
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
        pub mod index {
            pub mod params {}
            pub struct IndexActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> IndexActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Actions that can be performed on the datasources resource"]
                pub fn datasources(
                    &self,
                ) -> crate::resources::stats::index::datasources::DatasourcesActions
                {
                    crate::resources::stats::index::datasources::DatasourcesActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            pub mod datasources {
                pub mod params {}
                pub struct DatasourcesActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> DatasourcesActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Gets indexed item statistics for a single data source. **Note:** This API requires a standard end user account to execute."]
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
                            from_date_day: None,
                            from_date_month: None,
                            from_date_year: None,
                            to_date_day: None,
                            to_date_month: None,
                            to_date_year: None,
                        }
                    }
                }
                #[doc = "Created via [DatasourcesActions::get()](struct.DatasourcesActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    from_date_day: Option<i32>,
                    from_date_month: Option<i32>,
                    from_date_year: Option<i32>,
                    to_date_day: Option<i32>,
                    to_date_month: Option<i32>,
                    to_date_year: Option<i32>,
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
                    #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
                    pub fn from_date_day(mut self, value: i32) -> Self {
                        self.from_date_day = Some(value);
                        self
                    }
                    #[doc = "Month of date. Must be from 1 to 12."]
                    pub fn from_date_month(mut self, value: i32) -> Self {
                        self.from_date_month = Some(value);
                        self
                    }
                    #[doc = "Year of date. Must be from 1 to 9999."]
                    pub fn from_date_year(mut self, value: i32) -> Self {
                        self.from_date_year = Some(value);
                        self
                    }
                    #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
                    pub fn to_date_day(mut self, value: i32) -> Self {
                        self.to_date_day = Some(value);
                        self
                    }
                    #[doc = "Month of date. Must be from 1 to 12."]
                    pub fn to_date_month(mut self, value: i32) -> Self {
                        self.to_date_month = Some(value);
                        self
                    }
                    #[doc = "Year of date. Must be from 1 to 9999."]
                    pub fn to_date_year(mut self, value: i32) -> Self {
                        self.to_date_year = Some(value);
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
                    ) -> Result<crate::schemas::GetDataSourceIndexStatsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GetDataSourceIndexStatsResponse, crate::Error>
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/stats/index/");
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
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[("fromDate.day", &self.from_date_day)]);
                        req = req.query(&[("fromDate.month", &self.from_date_month)]);
                        req = req.query(&[("fromDate.year", &self.from_date_year)]);
                        req = req.query(&[("toDate.day", &self.to_date_day)]);
                        req = req.query(&[("toDate.month", &self.to_date_month)]);
                        req = req.query(&[("toDate.year", &self.to_date_year)]);
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
        pub mod query {
            pub mod params {}
            pub struct QueryActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> QueryActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Actions that can be performed on the searchapplications resource"]
                pub fn searchapplications(
                    &self,
                ) -> crate::resources::stats::query::searchapplications::SearchapplicationsActions
                {
                    crate::resources::stats::query::searchapplications::SearchapplicationsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            pub mod searchapplications {
                pub mod params {}
                pub struct SearchapplicationsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> SearchapplicationsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Get the query statistics for search application. **Note:** This API requires a standard end user account to execute."]
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
                            from_date_day: None,
                            from_date_month: None,
                            from_date_year: None,
                            to_date_day: None,
                            to_date_month: None,
                            to_date_year: None,
                        }
                    }
                }
                #[doc = "Created via [SearchapplicationsActions::get()](struct.SearchapplicationsActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    from_date_day: Option<i32>,
                    from_date_month: Option<i32>,
                    from_date_year: Option<i32>,
                    to_date_day: Option<i32>,
                    to_date_month: Option<i32>,
                    to_date_year: Option<i32>,
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
                    #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
                    pub fn from_date_day(mut self, value: i32) -> Self {
                        self.from_date_day = Some(value);
                        self
                    }
                    #[doc = "Month of date. Must be from 1 to 12."]
                    pub fn from_date_month(mut self, value: i32) -> Self {
                        self.from_date_month = Some(value);
                        self
                    }
                    #[doc = "Year of date. Must be from 1 to 9999."]
                    pub fn from_date_year(mut self, value: i32) -> Self {
                        self.from_date_year = Some(value);
                        self
                    }
                    #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
                    pub fn to_date_day(mut self, value: i32) -> Self {
                        self.to_date_day = Some(value);
                        self
                    }
                    #[doc = "Month of date. Must be from 1 to 12."]
                    pub fn to_date_month(mut self, value: i32) -> Self {
                        self.to_date_month = Some(value);
                        self
                    }
                    #[doc = "Year of date. Must be from 1 to 9999."]
                    pub fn to_date_year(mut self, value: i32) -> Self {
                        self.to_date_year = Some(value);
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
                    ) -> Result<crate::schemas::GetSearchApplicationQueryStatsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GetSearchApplicationQueryStatsResponse, crate::Error>
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/stats/query/");
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
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[("fromDate.day", &self.from_date_day)]);
                        req = req.query(&[("fromDate.month", &self.from_date_month)]);
                        req = req.query(&[("fromDate.year", &self.from_date_year)]);
                        req = req.query(&[("toDate.day", &self.to_date_day)]);
                        req = req.query(&[("toDate.month", &self.to_date_month)]);
                        req = req.query(&[("toDate.year", &self.to_date_year)]);
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
        pub mod session {
            pub mod params {}
            pub struct SessionActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> SessionActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Actions that can be performed on the searchapplications resource"]
                pub fn searchapplications(
                    &self,
                ) -> crate::resources::stats::session::searchapplications::SearchapplicationsActions
                {
                    crate :: resources :: stats :: session :: searchapplications :: SearchapplicationsActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                }
            }
            pub mod searchapplications {
                pub mod params {}
                pub struct SearchapplicationsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> SearchapplicationsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Get the # of search sessions, % of successful sessions with a click query statistics for search application. **Note:** This API requires a standard end user account to execute."]
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
                            from_date_day: None,
                            from_date_month: None,
                            from_date_year: None,
                            to_date_day: None,
                            to_date_month: None,
                            to_date_year: None,
                        }
                    }
                }
                #[doc = "Created via [SearchapplicationsActions::get()](struct.SearchapplicationsActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    from_date_day: Option<i32>,
                    from_date_month: Option<i32>,
                    from_date_year: Option<i32>,
                    to_date_day: Option<i32>,
                    to_date_month: Option<i32>,
                    to_date_year: Option<i32>,
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
                    #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
                    pub fn from_date_day(mut self, value: i32) -> Self {
                        self.from_date_day = Some(value);
                        self
                    }
                    #[doc = "Month of date. Must be from 1 to 12."]
                    pub fn from_date_month(mut self, value: i32) -> Self {
                        self.from_date_month = Some(value);
                        self
                    }
                    #[doc = "Year of date. Must be from 1 to 9999."]
                    pub fn from_date_year(mut self, value: i32) -> Self {
                        self.from_date_year = Some(value);
                        self
                    }
                    #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
                    pub fn to_date_day(mut self, value: i32) -> Self {
                        self.to_date_day = Some(value);
                        self
                    }
                    #[doc = "Month of date. Must be from 1 to 12."]
                    pub fn to_date_month(mut self, value: i32) -> Self {
                        self.to_date_month = Some(value);
                        self
                    }
                    #[doc = "Year of date. Must be from 1 to 9999."]
                    pub fn to_date_year(mut self, value: i32) -> Self {
                        self.to_date_year = Some(value);
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
                        crate::schemas::GetSearchApplicationSessionStatsResponse,
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
                        crate::schemas::GetSearchApplicationSessionStatsResponse,
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/stats/session/");
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
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[("fromDate.day", &self.from_date_day)]);
                        req = req.query(&[("fromDate.month", &self.from_date_month)]);
                        req = req.query(&[("fromDate.year", &self.from_date_year)]);
                        req = req.query(&[("toDate.day", &self.to_date_day)]);
                        req = req.query(&[("toDate.month", &self.to_date_month)]);
                        req = req.query(&[("toDate.year", &self.to_date_year)]);
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
        pub mod user {
            pub mod params {}
            pub struct UserActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> UserActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Actions that can be performed on the searchapplications resource"]
                pub fn searchapplications(
                    &self,
                ) -> crate::resources::stats::user::searchapplications::SearchapplicationsActions
                {
                    crate::resources::stats::user::searchapplications::SearchapplicationsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            pub mod searchapplications {
                pub mod params {}
                pub struct SearchapplicationsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> SearchapplicationsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Get the users statistics for search application. **Note:** This API requires a standard end user account to execute."]
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
                            from_date_day: None,
                            from_date_month: None,
                            from_date_year: None,
                            to_date_day: None,
                            to_date_month: None,
                            to_date_year: None,
                        }
                    }
                }
                #[doc = "Created via [SearchapplicationsActions::get()](struct.SearchapplicationsActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    name: String,
                    from_date_day: Option<i32>,
                    from_date_month: Option<i32>,
                    from_date_year: Option<i32>,
                    to_date_day: Option<i32>,
                    to_date_month: Option<i32>,
                    to_date_year: Option<i32>,
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
                    #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
                    pub fn from_date_day(mut self, value: i32) -> Self {
                        self.from_date_day = Some(value);
                        self
                    }
                    #[doc = "Month of date. Must be from 1 to 12."]
                    pub fn from_date_month(mut self, value: i32) -> Self {
                        self.from_date_month = Some(value);
                        self
                    }
                    #[doc = "Year of date. Must be from 1 to 9999."]
                    pub fn from_date_year(mut self, value: i32) -> Self {
                        self.from_date_year = Some(value);
                        self
                    }
                    #[doc = "Day of month. Must be from 1 to 31 and valid for the year and month."]
                    pub fn to_date_day(mut self, value: i32) -> Self {
                        self.to_date_day = Some(value);
                        self
                    }
                    #[doc = "Month of date. Must be from 1 to 12."]
                    pub fn to_date_month(mut self, value: i32) -> Self {
                        self.to_date_month = Some(value);
                        self
                    }
                    #[doc = "Year of date. Must be from 1 to 9999."]
                    pub fn to_date_year(mut self, value: i32) -> Self {
                        self.to_date_year = Some(value);
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
                    ) -> Result<crate::schemas::GetSearchApplicationUserStatsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GetSearchApplicationUserStatsResponse, crate::Error>
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
                        let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                        output.push_str("v1/stats/user/");
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
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[("fromDate.day", &self.from_date_day)]);
                        req = req.query(&[("fromDate.month", &self.from_date_month)]);
                        req = req.query(&[("fromDate.year", &self.from_date_year)]);
                        req = req.query(&[("toDate.day", &self.to_date_day)]);
                        req = req.query(&[("toDate.month", &self.to_date_month)]);
                        req = req.query(&[("toDate.year", &self.to_date_year)]);
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
    pub mod v_1 {
        pub mod params {}
        pub struct V1Actions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> V1Actions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Enables `third party` support in Google Cloud Search. **Note:** This API requires an admin account to execute."]
            pub fn initialize_customer(
                &self,
                request: crate::schemas::InitializeCustomerRequest,
            ) -> InitializeCustomerRequestBuilder {
                InitializeCustomerRequestBuilder {
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
                }
            }
        }
        #[doc = "Created via [V1Actions::initialize_customer()](struct.V1Actions.html#method.initialize_customer)"]
        #[derive(Debug, Clone)]
        pub struct InitializeCustomerRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::InitializeCustomerRequest,
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
        impl<'a> InitializeCustomerRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::Operation, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Operation, crate::Error> {
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
                let mut output = "https://cloudsearch.googleapis.com/".to_owned();
                output.push_str("v1:initializeCustomer");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
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
