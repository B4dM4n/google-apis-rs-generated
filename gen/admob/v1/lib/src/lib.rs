#![doc = "# Resources and Methods\n    * [accounts](resources/accounts/struct.AccountsActions.html)\n      * [*get*](resources/accounts/struct.GetRequestBuilder.html), [*list*](resources/accounts/struct.ListRequestBuilder.html)\n      * [ad_units](resources/accounts/ad_units/struct.AdUnitsActions.html)\n        * [*list*](resources/accounts/ad_units/struct.ListRequestBuilder.html)\n      * [apps](resources/accounts/apps/struct.AppsActions.html)\n        * [*list*](resources/accounts/apps/struct.ListRequestBuilder.html)\n      * [mediation_report](resources/accounts/mediation_report/struct.MediationReportActions.html)\n        * [*generate*](resources/accounts/mediation_report/struct.GenerateRequestBuilder.html)\n      * [network_report](resources/accounts/network_report/struct.NetworkReportActions.html)\n        * [*generate*](resources/accounts/network_report/struct.GenerateRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "See your AdMob data\n\n`https://www.googleapis.com/auth/admob.readonly`"]
    pub const ADMOB_READONLY: &str = "https://www.googleapis.com/auth/admob.readonly";
    #[doc = "See your AdMob data\n\n`https://www.googleapis.com/auth/admob.report`"]
    pub const ADMOB_REPORT: &str = "https://www.googleapis.com/auth/admob.report";
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
    pub struct AdUnit {
        #[doc = "AdFormat of the ad unit. Possible values are as follows: \"BANNER\" - Banner ad format. \"BANNER_INTERSTITIAL\" - Legacy format that can be used as either banner or interstitial. This format can no longer be created but can be targeted by mediation groups. \"INTERSTITIAL\" - A full screen ad. Supported ad types are \"RICH_MEDIA\" and \"VIDEO\". \"NATIVE\" - Native ad format. \"REWARDED\" - An ad that, once viewed, gets a callback verifying the view so that a reward can be given to the user. Supported ad types are \"RICH_MEDIA\" (interactive) and video where video can not be excluded."]
        #[serde(
            rename = "adFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ad_format: ::std::option::Option<String>,
        #[doc = "Ad media type supported by this ad unit. Possible values as follows: \"RICH_MEDIA\" - Text, image, and other non-video media. \"VIDEO\" - Video media."]
        #[serde(
            rename = "adTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ad_types: ::std::option::Option<Vec<String>>,
        #[doc = "The externally visible ID of the ad unit which can be used to integrate with the AdMob SDK. This is a read only property. Example: ca-app-pub-9876543210987654/0123456789"]
        #[serde(
            rename = "adUnitId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ad_unit_id: ::std::option::Option<String>,
        #[doc = "The externally visible ID of the app this ad unit is associated with. Example: ca-app-pub-9876543210987654~0123456789"]
        #[serde(
            rename = "appId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_id: ::std::option::Option<String>,
        #[doc = "The display name of the ad unit as shown in the AdMob UI, which is provided by the user. The maximum length allowed is 80 characters."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Resource name for this ad unit. Format is accounts/{publisher_id}/adUnits/{ad_unit_id_fragment} Example: accounts/pub-9876543210987654/adUnits/0123456789"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AdUnit {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AdUnit {
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
    pub struct App {
        #[doc = "The externally visible ID of the app which can be used to integrate with the AdMob SDK. This is a read only property. Example: ca-app-pub-9876543210987654~0123456789"]
        #[serde(
            rename = "appId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_id: ::std::option::Option<String>,
        #[doc = "Immutable. The information for an app that is linked to an app store. This field is present if and only if the app is linked to an app store."]
        #[serde(
            rename = "linkedAppInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub linked_app_info: ::std::option::Option<crate::schemas::AppLinkedAppInfo>,
        #[doc = "The information for an app that is not linked to any app store. After an app is linked, this information is still retrivable. If no name is provided for the app upon creation, a placeholder name will be used."]
        #[serde(
            rename = "manualAppInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub manual_app_info: ::std::option::Option<crate::schemas::AppManualAppInfo>,
        #[doc = "Resource name for this app. Format is accounts/{publisher_id}/apps/{app_id_fragment} Example: accounts/pub-9876543210987654/apps/0123456789"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Describes the platform of the app. Limited to \"IOS\" and \"ANDROID\"."]
        #[serde(
            rename = "platform",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub platform: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for App {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for App {
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
    pub struct AppLinkedAppInfo {
        #[doc = "The app store ID of the app; present if and only if the app is linked to an app store. If the app is added to the Google Play store, it will be the application ID of the app. For example: \"com.example.myapp\". See https://developer.android.com/studio/build/application-id. If the app is added to the Apple App Store, it will be app store ID. For example \"105169111\". Note that setting the app store id is considered an irreversible action. Once an app is linked, it cannot be unlinked."]
        #[serde(
            rename = "appStoreId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_store_id: ::std::option::Option<String>,
        #[doc = "Output only. Display name of the app as it appears in the app store. This is an output-only field, and may be empty if the app cannot be found in the store."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AppLinkedAppInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AppLinkedAppInfo {
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
    pub struct AppManualAppInfo {
        #[doc = "The display name of the app as shown in the AdMob UI, which is provided by the user. The maximum length allowed is 80 characters."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AppManualAppInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AppManualAppInfo {
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
        #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
        #[serde(
            rename = "day",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub day: ::std::option::Option<i32>,
        #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
        #[serde(
            rename = "month",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub month: ::std::option::Option<i32>,
        #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
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
    pub struct DateRange {
        #[doc = "End date of the date range, inclusive. Must be greater than or equal to the start date."]
        #[serde(
            rename = "endDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "Start date of the date range, inclusive. Must be less than or equal to the end date."]
        #[serde(
            rename = "startDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_date: ::std::option::Option<crate::schemas::Date>,
    }
    impl ::google_field_selector::FieldSelector for DateRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DateRange {
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
    pub struct GenerateMediationReportRequest {
        #[doc = "Network report specification."]
        #[serde(
            rename = "reportSpec",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_spec: ::std::option::Option<crate::schemas::MediationReportSpec>,
    }
    impl ::google_field_selector::FieldSelector for GenerateMediationReportRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GenerateMediationReportRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GenerateMediationReportResponse {
        #[doc = "Additional information about the generated report, such as warnings about the data."]
        #[serde(
            rename = "footer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub footer: ::std::option::Option<crate::schemas::ReportFooter>,
        #[doc = "Report generation settings that describes the report contents, such as the report date range and localization settings."]
        #[serde(
            rename = "header",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header: ::std::option::Option<crate::schemas::ReportHeader>,
        #[doc = "Actual report data."]
        #[serde(
            rename = "row",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row: ::std::option::Option<crate::schemas::ReportRow>,
    }
    impl ::google_field_selector::FieldSelector for GenerateMediationReportResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GenerateMediationReportResponse {
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
    pub struct GenerateNetworkReportRequest {
        #[doc = "Network report specification."]
        #[serde(
            rename = "reportSpec",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_spec: ::std::option::Option<crate::schemas::NetworkReportSpec>,
    }
    impl ::google_field_selector::FieldSelector for GenerateNetworkReportRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GenerateNetworkReportRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GenerateNetworkReportResponse {
        #[doc = "Additional information about the generated report, such as warnings about the data."]
        #[serde(
            rename = "footer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub footer: ::std::option::Option<crate::schemas::ReportFooter>,
        #[doc = "Report generation settings that describes the report contents, such as the report date range and localization settings."]
        #[serde(
            rename = "header",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header: ::std::option::Option<crate::schemas::ReportHeader>,
        #[doc = "Actual report data."]
        #[serde(
            rename = "row",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row: ::std::option::Option<crate::schemas::ReportRow>,
    }
    impl ::google_field_selector::FieldSelector for GenerateNetworkReportResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GenerateNetworkReportResponse {
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
    pub struct ListAdUnitsResponse {
        #[doc = "The resulting ad units for the requested account."]
        #[serde(
            rename = "adUnits",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ad_units: ::std::option::Option<Vec<crate::schemas::AdUnit>>,
        #[doc = "If not empty, indicates that there may be more ad units for the request; this value should be passed in a new `ListAdUnitsRequest`."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListAdUnitsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListAdUnitsResponse {
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
    pub struct ListAppsResponse {
        #[doc = "The resulting apps for the requested account."]
        #[serde(
            rename = "apps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub apps: ::std::option::Option<Vec<crate::schemas::App>>,
        #[doc = "If not empty, indicates that there may be more apps for the request; this value should be passed in a new `ListAppsRequest`."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListAppsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListAppsResponse {
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
    pub struct ListPublisherAccountsResponse {
        #[doc = "Publisher that the client credentials can access."]
        #[serde(
            rename = "account",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub account: ::std::option::Option<Vec<crate::schemas::PublisherAccount>>,
        #[doc = "If not empty, indicates that there might be more accounts for the request; you must pass this value in a new `ListPublisherAccountsRequest`."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListPublisherAccountsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListPublisherAccountsResponse {
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
    pub struct LocalizationSettings {
        #[doc = "Currency code of the earning related metrics, which is the 3-letter code defined in ISO 4217. The daily average rate is used for the currency conversion. Defaults to the account currency code if unspecified."]
        #[serde(
            rename = "currencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency_code: ::std::option::Option<String>,
        #[doc = "Language used for any localized text, such as some dimension value display labels. The language tag defined in the IETF BCP47. Defaults to 'en-US' if unspecified."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for LocalizationSettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LocalizationSettings {
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
    pub struct MediationReportSpec {
        #[doc = "The date range for which the report is generated."]
        #[serde(
            rename = "dateRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_range: ::std::option::Option<crate::schemas::DateRange>,
        #[doc = "Describes which report rows to match based on their dimension values."]
        #[serde(
            rename = "dimensionFilters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_filters:
            ::std::option::Option<Vec<crate::schemas::MediationReportSpecDimensionFilter>>,
        #[doc = "List of dimensions of the report. The value combination of these dimensions determines the row of the report. If no dimensions are specified, the report returns a single row of requested metrics for the entire account."]
        #[serde(
            rename = "dimensions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimensions:
            ::std::option::Option<Vec<crate::schemas::MediationReportSpecDimensionsItems>>,
        #[doc = "Localization settings of the report."]
        #[serde(
            rename = "localizationSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub localization_settings: ::std::option::Option<crate::schemas::LocalizationSettings>,
        #[doc = "Maximum number of report data rows to return. If the value is not set, the API returns as many rows as possible, up to 100000. Acceptable values are 1-100000, inclusive. Values larger than 100000 return an error."]
        #[serde(
            rename = "maxReportRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_report_rows: ::std::option::Option<i32>,
        #[doc = "List of metrics of the report. A report must specify at least one metric."]
        #[serde(
            rename = "metrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metrics: ::std::option::Option<Vec<crate::schemas::MediationReportSpecMetricsItems>>,
        #[doc = "Describes the sorting of report rows. The order of the condition in the list defines its precedence; the earlier the condition, the higher its precedence. If no sort conditions are specified, the row ordering is undefined."]
        #[serde(
            rename = "sortConditions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sort_conditions:
            ::std::option::Option<Vec<crate::schemas::MediationReportSpecSortCondition>>,
        #[doc = "A report time zone. Accepts an IANA TZ name values, such as \"America/Los_Angeles.\" If no time zone is defined, the account default takes effect. Check default value by the get account action. **Warning:** The \"America/Los_Angeles\" is the only supported value at the moment."]
        #[serde(
            rename = "timeZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_zone: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for MediationReportSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MediationReportSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MediationReportSpecDimensionsItems {
        #[doc = "The [unique ID of the ad source](/admob/api/v1/ad_sources) (for example, \"5450213213286189855\" and \"AdMob Network\" as label value)."]
        AdSource,
        #[doc = "The unique ID of the ad source instance (for example, \"ca-app-pub-1234:asi:5678\" and \"AdMob (default)\" as label value)."]
        AdSourceInstance,
        #[doc = "The unique ID of the ad unit (for example, \"ca-app-pub-1234/8790\"). If AD_UNIT dimension is specified, then APP is included automatically."]
        AdUnit,
        #[doc = "The unique ID of the mobile application (for example, \"ca-app-pub-1234~1234\")."]
        App,
        #[doc = "For Android, the app version name can be found in versionName in PackageInfo. For iOS, the app version name can be found in CFBundleShortVersionString. **Warning:** The dimension is incompatible with [ESTIMATED_EARNINGS](#Metric.ENUM_VALUES.ESTIMATED_EARNINGS), [OBSERVED_ECPM](#Metric.ENUM_VALUES.OBSERVED_ECPM) metrics."]
        AppVersionName,
        #[doc = "CLDR country code of the place where the ad views/clicks occur (for example, \"US\" or \"FR\"). This is a geography dimension."]
        Country,
        #[doc = "A date in the YYYYMMDD format (for example, \"20210701\"). Requests can specify at most one time dimension."]
        Date,
        #[doc = "Default value for an unset field. Do not use."]
        DimensionUnspecified,
        #[doc = "Format of the ad unit (for example, \"banner\", \"native\"), an ad delivery dimension."]
        Format,
        #[doc = "GMA SDK version, e.g. \"iOS 7.62.0\". **Warning:** The dimension is incompatible with [ESTIMATED_EARNINGS](#Metric.ENUM_VALUES.ESTIMATED_EARNINGS), [OBSERVED_ECPM](#Metric.ENUM_VALUES.OBSERVED_ECPM) metrics."]
        GmaSdkVersion,
        #[doc = "The unique ID of the mediation group (for example, \"ca-app-pub-1234:mg:1234\" and \"AdMob (default)\" as label value)."]
        MediationGroup,
        #[doc = "Mobile operating system version, e.g. \"iOS 13.5.1\". **Warning:** The dimension is incompatible with [ESTIMATED_EARNINGS](#Metric.ENUM_VALUES.ESTIMATED_EARNINGS), [OBSERVED_ECPM](#Metric.ENUM_VALUES.OBSERVED_ECPM) metrics."]
        MobileOsVersion,
        #[doc = "A month in the YYYYMM format (for example, \"202107\"). Requests can specify at most one time dimension."]
        Month,
        #[doc = "Mobile OS platform of the app (for example, \"Android\" or \"iOS\")."]
        Platform,
        #[doc = "Restriction mode for ads serving (e.g. \"Non-personalized ads\"). **Warning:** The dimension is incompatible with [ESTIMATED_EARNINGS](#Metric.ENUM_VALUES.ESTIMATED_EARNINGS) metric."]
        ServingRestriction,
        #[doc = "The date of the first day of a week in the YYYYMMDD format (for example, \"20210701\"). Requests can specify at most one time dimension."]
        Week,
    }
    impl MediationReportSpecDimensionsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                MediationReportSpecDimensionsItems::AdSource => "AD_SOURCE",
                MediationReportSpecDimensionsItems::AdSourceInstance => "AD_SOURCE_INSTANCE",
                MediationReportSpecDimensionsItems::AdUnit => "AD_UNIT",
                MediationReportSpecDimensionsItems::App => "APP",
                MediationReportSpecDimensionsItems::AppVersionName => "APP_VERSION_NAME",
                MediationReportSpecDimensionsItems::Country => "COUNTRY",
                MediationReportSpecDimensionsItems::Date => "DATE",
                MediationReportSpecDimensionsItems::DimensionUnspecified => "DIMENSION_UNSPECIFIED",
                MediationReportSpecDimensionsItems::Format => "FORMAT",
                MediationReportSpecDimensionsItems::GmaSdkVersion => "GMA_SDK_VERSION",
                MediationReportSpecDimensionsItems::MediationGroup => "MEDIATION_GROUP",
                MediationReportSpecDimensionsItems::MobileOsVersion => "MOBILE_OS_VERSION",
                MediationReportSpecDimensionsItems::Month => "MONTH",
                MediationReportSpecDimensionsItems::Platform => "PLATFORM",
                MediationReportSpecDimensionsItems::ServingRestriction => "SERVING_RESTRICTION",
                MediationReportSpecDimensionsItems::Week => "WEEK",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MediationReportSpecDimensionsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MediationReportSpecDimensionsItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MediationReportSpecDimensionsItems, ()> {
            Ok(match s {
                "AD_SOURCE" => MediationReportSpecDimensionsItems::AdSource,
                "AD_SOURCE_INSTANCE" => MediationReportSpecDimensionsItems::AdSourceInstance,
                "AD_UNIT" => MediationReportSpecDimensionsItems::AdUnit,
                "APP" => MediationReportSpecDimensionsItems::App,
                "APP_VERSION_NAME" => MediationReportSpecDimensionsItems::AppVersionName,
                "COUNTRY" => MediationReportSpecDimensionsItems::Country,
                "DATE" => MediationReportSpecDimensionsItems::Date,
                "DIMENSION_UNSPECIFIED" => MediationReportSpecDimensionsItems::DimensionUnspecified,
                "FORMAT" => MediationReportSpecDimensionsItems::Format,
                "GMA_SDK_VERSION" => MediationReportSpecDimensionsItems::GmaSdkVersion,
                "MEDIATION_GROUP" => MediationReportSpecDimensionsItems::MediationGroup,
                "MOBILE_OS_VERSION" => MediationReportSpecDimensionsItems::MobileOsVersion,
                "MONTH" => MediationReportSpecDimensionsItems::Month,
                "PLATFORM" => MediationReportSpecDimensionsItems::Platform,
                "SERVING_RESTRICTION" => MediationReportSpecDimensionsItems::ServingRestriction,
                "WEEK" => MediationReportSpecDimensionsItems::Week,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MediationReportSpecDimensionsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MediationReportSpecDimensionsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MediationReportSpecDimensionsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AD_SOURCE" => MediationReportSpecDimensionsItems::AdSource,
                "AD_SOURCE_INSTANCE" => MediationReportSpecDimensionsItems::AdSourceInstance,
                "AD_UNIT" => MediationReportSpecDimensionsItems::AdUnit,
                "APP" => MediationReportSpecDimensionsItems::App,
                "APP_VERSION_NAME" => MediationReportSpecDimensionsItems::AppVersionName,
                "COUNTRY" => MediationReportSpecDimensionsItems::Country,
                "DATE" => MediationReportSpecDimensionsItems::Date,
                "DIMENSION_UNSPECIFIED" => MediationReportSpecDimensionsItems::DimensionUnspecified,
                "FORMAT" => MediationReportSpecDimensionsItems::Format,
                "GMA_SDK_VERSION" => MediationReportSpecDimensionsItems::GmaSdkVersion,
                "MEDIATION_GROUP" => MediationReportSpecDimensionsItems::MediationGroup,
                "MOBILE_OS_VERSION" => MediationReportSpecDimensionsItems::MobileOsVersion,
                "MONTH" => MediationReportSpecDimensionsItems::Month,
                "PLATFORM" => MediationReportSpecDimensionsItems::Platform,
                "SERVING_RESTRICTION" => MediationReportSpecDimensionsItems::ServingRestriction,
                "WEEK" => MediationReportSpecDimensionsItems::Week,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MediationReportSpecDimensionsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MediationReportSpecDimensionsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MediationReportSpecMetricsItems {
        #[doc = "The number of requests. The value is an integer."]
        AdRequests,
        #[doc = "The number of times a user clicks an ad. The value is an integer."]
        Clicks,
        #[doc = "The estimated earnings of the AdMob publisher. The currency unit (USD, EUR, or other) of the earning metrics are determined by the localization setting for currency. The amount is in micros. For example, $6.50 would be represented as 6500000. Estimated earnings per mediation group and per ad source instance level is supported dating back to October 20, 2019. Third-party estimated earnings will show 0 for dates prior to October 20, 2019."]
        EstimatedEarnings,
        #[doc = "The ratio of clicks over impressions. The value is a double precision (approximate) decimal value."]
        ImpressionCtr,
        #[doc = "The total number of ads shown to users. The value is an integer."]
        Impressions,
        #[doc = "The ratio of matched ad requests over the total ad requests. The value is a double precision (approximate) decimal value."]
        MatchRate,
        #[doc = "The number of times ads are returned in response to a request. The value is an integer."]
        MatchedRequests,
        #[doc = "Default value for an unset field. Do not use."]
        MetricUnspecified,
        #[doc = "The third-party ad network's estimated average eCPM. The currency unit (USD, EUR, or other) of the earning metrics are determined by the localization setting for currency. The amount is in micros. For example, $2.30 would be represented as 2300000. The estimated average eCPM per mediation group and per ad source instance level is supported dating back to October 20, 2019. Third-party estimated average eCPM will show 0 for dates prior to October 20, 2019."]
        ObservedEcpm,
    }
    impl MediationReportSpecMetricsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                MediationReportSpecMetricsItems::AdRequests => "AD_REQUESTS",
                MediationReportSpecMetricsItems::Clicks => "CLICKS",
                MediationReportSpecMetricsItems::EstimatedEarnings => "ESTIMATED_EARNINGS",
                MediationReportSpecMetricsItems::ImpressionCtr => "IMPRESSION_CTR",
                MediationReportSpecMetricsItems::Impressions => "IMPRESSIONS",
                MediationReportSpecMetricsItems::MatchRate => "MATCH_RATE",
                MediationReportSpecMetricsItems::MatchedRequests => "MATCHED_REQUESTS",
                MediationReportSpecMetricsItems::MetricUnspecified => "METRIC_UNSPECIFIED",
                MediationReportSpecMetricsItems::ObservedEcpm => "OBSERVED_ECPM",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MediationReportSpecMetricsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MediationReportSpecMetricsItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MediationReportSpecMetricsItems, ()> {
            Ok(match s {
                "AD_REQUESTS" => MediationReportSpecMetricsItems::AdRequests,
                "CLICKS" => MediationReportSpecMetricsItems::Clicks,
                "ESTIMATED_EARNINGS" => MediationReportSpecMetricsItems::EstimatedEarnings,
                "IMPRESSION_CTR" => MediationReportSpecMetricsItems::ImpressionCtr,
                "IMPRESSIONS" => MediationReportSpecMetricsItems::Impressions,
                "MATCH_RATE" => MediationReportSpecMetricsItems::MatchRate,
                "MATCHED_REQUESTS" => MediationReportSpecMetricsItems::MatchedRequests,
                "METRIC_UNSPECIFIED" => MediationReportSpecMetricsItems::MetricUnspecified,
                "OBSERVED_ECPM" => MediationReportSpecMetricsItems::ObservedEcpm,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MediationReportSpecMetricsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MediationReportSpecMetricsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MediationReportSpecMetricsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AD_REQUESTS" => MediationReportSpecMetricsItems::AdRequests,
                "CLICKS" => MediationReportSpecMetricsItems::Clicks,
                "ESTIMATED_EARNINGS" => MediationReportSpecMetricsItems::EstimatedEarnings,
                "IMPRESSION_CTR" => MediationReportSpecMetricsItems::ImpressionCtr,
                "IMPRESSIONS" => MediationReportSpecMetricsItems::Impressions,
                "MATCH_RATE" => MediationReportSpecMetricsItems::MatchRate,
                "MATCHED_REQUESTS" => MediationReportSpecMetricsItems::MatchedRequests,
                "METRIC_UNSPECIFIED" => MediationReportSpecMetricsItems::MetricUnspecified,
                "OBSERVED_ECPM" => MediationReportSpecMetricsItems::ObservedEcpm,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MediationReportSpecMetricsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MediationReportSpecMetricsItems {
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
    pub struct MediationReportSpecDimensionFilter {
        #[doc = "Applies the filter criterion to the specified dimension."]
        #[serde(
            rename = "dimension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension:
            ::std::option::Option<crate::schemas::MediationReportSpecDimensionFilterDimension>,
        #[doc = "Matches a row if its value for the specified dimension is in one of the values specified in this condition."]
        #[serde(
            rename = "matchesAny",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub matches_any: ::std::option::Option<crate::schemas::StringList>,
    }
    impl ::google_field_selector::FieldSelector for MediationReportSpecDimensionFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MediationReportSpecDimensionFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MediationReportSpecDimensionFilterDimension {
        #[doc = "The [unique ID of the ad source](/admob/api/v1/ad_sources) (for example, \"5450213213286189855\" and \"AdMob Network\" as label value)."]
        AdSource,
        #[doc = "The unique ID of the ad source instance (for example, \"ca-app-pub-1234:asi:5678\" and \"AdMob (default)\" as label value)."]
        AdSourceInstance,
        #[doc = "The unique ID of the ad unit (for example, \"ca-app-pub-1234/8790\"). If AD_UNIT dimension is specified, then APP is included automatically."]
        AdUnit,
        #[doc = "The unique ID of the mobile application (for example, \"ca-app-pub-1234~1234\")."]
        App,
        #[doc = "For Android, the app version name can be found in versionName in PackageInfo. For iOS, the app version name can be found in CFBundleShortVersionString. **Warning:** The dimension is incompatible with [ESTIMATED_EARNINGS](#Metric.ENUM_VALUES.ESTIMATED_EARNINGS), [OBSERVED_ECPM](#Metric.ENUM_VALUES.OBSERVED_ECPM) metrics."]
        AppVersionName,
        #[doc = "CLDR country code of the place where the ad views/clicks occur (for example, \"US\" or \"FR\"). This is a geography dimension."]
        Country,
        #[doc = "A date in the YYYYMMDD format (for example, \"20210701\"). Requests can specify at most one time dimension."]
        Date,
        #[doc = "Default value for an unset field. Do not use."]
        DimensionUnspecified,
        #[doc = "Format of the ad unit (for example, \"banner\", \"native\"), an ad delivery dimension."]
        Format,
        #[doc = "GMA SDK version, e.g. \"iOS 7.62.0\". **Warning:** The dimension is incompatible with [ESTIMATED_EARNINGS](#Metric.ENUM_VALUES.ESTIMATED_EARNINGS), [OBSERVED_ECPM](#Metric.ENUM_VALUES.OBSERVED_ECPM) metrics."]
        GmaSdkVersion,
        #[doc = "The unique ID of the mediation group (for example, \"ca-app-pub-1234:mg:1234\" and \"AdMob (default)\" as label value)."]
        MediationGroup,
        #[doc = "Mobile operating system version, e.g. \"iOS 13.5.1\". **Warning:** The dimension is incompatible with [ESTIMATED_EARNINGS](#Metric.ENUM_VALUES.ESTIMATED_EARNINGS), [OBSERVED_ECPM](#Metric.ENUM_VALUES.OBSERVED_ECPM) metrics."]
        MobileOsVersion,
        #[doc = "A month in the YYYYMM format (for example, \"202107\"). Requests can specify at most one time dimension."]
        Month,
        #[doc = "Mobile OS platform of the app (for example, \"Android\" or \"iOS\")."]
        Platform,
        #[doc = "Restriction mode for ads serving (e.g. \"Non-personalized ads\"). **Warning:** The dimension is incompatible with [ESTIMATED_EARNINGS](#Metric.ENUM_VALUES.ESTIMATED_EARNINGS) metric."]
        ServingRestriction,
        #[doc = "The date of the first day of a week in the YYYYMMDD format (for example, \"20210701\"). Requests can specify at most one time dimension."]
        Week,
    }
    impl MediationReportSpecDimensionFilterDimension {
        pub fn as_str(self) -> &'static str {
            match self {
                MediationReportSpecDimensionFilterDimension::AdSource => "AD_SOURCE",
                MediationReportSpecDimensionFilterDimension::AdSourceInstance => {
                    "AD_SOURCE_INSTANCE"
                }
                MediationReportSpecDimensionFilterDimension::AdUnit => "AD_UNIT",
                MediationReportSpecDimensionFilterDimension::App => "APP",
                MediationReportSpecDimensionFilterDimension::AppVersionName => "APP_VERSION_NAME",
                MediationReportSpecDimensionFilterDimension::Country => "COUNTRY",
                MediationReportSpecDimensionFilterDimension::Date => "DATE",
                MediationReportSpecDimensionFilterDimension::DimensionUnspecified => {
                    "DIMENSION_UNSPECIFIED"
                }
                MediationReportSpecDimensionFilterDimension::Format => "FORMAT",
                MediationReportSpecDimensionFilterDimension::GmaSdkVersion => "GMA_SDK_VERSION",
                MediationReportSpecDimensionFilterDimension::MediationGroup => "MEDIATION_GROUP",
                MediationReportSpecDimensionFilterDimension::MobileOsVersion => "MOBILE_OS_VERSION",
                MediationReportSpecDimensionFilterDimension::Month => "MONTH",
                MediationReportSpecDimensionFilterDimension::Platform => "PLATFORM",
                MediationReportSpecDimensionFilterDimension::ServingRestriction => {
                    "SERVING_RESTRICTION"
                }
                MediationReportSpecDimensionFilterDimension::Week => "WEEK",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MediationReportSpecDimensionFilterDimension {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MediationReportSpecDimensionFilterDimension {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<MediationReportSpecDimensionFilterDimension, ()> {
            Ok(match s {
                "AD_SOURCE" => MediationReportSpecDimensionFilterDimension::AdSource,
                "AD_SOURCE_INSTANCE" => {
                    MediationReportSpecDimensionFilterDimension::AdSourceInstance
                }
                "AD_UNIT" => MediationReportSpecDimensionFilterDimension::AdUnit,
                "APP" => MediationReportSpecDimensionFilterDimension::App,
                "APP_VERSION_NAME" => MediationReportSpecDimensionFilterDimension::AppVersionName,
                "COUNTRY" => MediationReportSpecDimensionFilterDimension::Country,
                "DATE" => MediationReportSpecDimensionFilterDimension::Date,
                "DIMENSION_UNSPECIFIED" => {
                    MediationReportSpecDimensionFilterDimension::DimensionUnspecified
                }
                "FORMAT" => MediationReportSpecDimensionFilterDimension::Format,
                "GMA_SDK_VERSION" => MediationReportSpecDimensionFilterDimension::GmaSdkVersion,
                "MEDIATION_GROUP" => MediationReportSpecDimensionFilterDimension::MediationGroup,
                "MOBILE_OS_VERSION" => MediationReportSpecDimensionFilterDimension::MobileOsVersion,
                "MONTH" => MediationReportSpecDimensionFilterDimension::Month,
                "PLATFORM" => MediationReportSpecDimensionFilterDimension::Platform,
                "SERVING_RESTRICTION" => {
                    MediationReportSpecDimensionFilterDimension::ServingRestriction
                }
                "WEEK" => MediationReportSpecDimensionFilterDimension::Week,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MediationReportSpecDimensionFilterDimension {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MediationReportSpecDimensionFilterDimension {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MediationReportSpecDimensionFilterDimension {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AD_SOURCE" => MediationReportSpecDimensionFilterDimension::AdSource,
                "AD_SOURCE_INSTANCE" => {
                    MediationReportSpecDimensionFilterDimension::AdSourceInstance
                }
                "AD_UNIT" => MediationReportSpecDimensionFilterDimension::AdUnit,
                "APP" => MediationReportSpecDimensionFilterDimension::App,
                "APP_VERSION_NAME" => MediationReportSpecDimensionFilterDimension::AppVersionName,
                "COUNTRY" => MediationReportSpecDimensionFilterDimension::Country,
                "DATE" => MediationReportSpecDimensionFilterDimension::Date,
                "DIMENSION_UNSPECIFIED" => {
                    MediationReportSpecDimensionFilterDimension::DimensionUnspecified
                }
                "FORMAT" => MediationReportSpecDimensionFilterDimension::Format,
                "GMA_SDK_VERSION" => MediationReportSpecDimensionFilterDimension::GmaSdkVersion,
                "MEDIATION_GROUP" => MediationReportSpecDimensionFilterDimension::MediationGroup,
                "MOBILE_OS_VERSION" => MediationReportSpecDimensionFilterDimension::MobileOsVersion,
                "MONTH" => MediationReportSpecDimensionFilterDimension::Month,
                "PLATFORM" => MediationReportSpecDimensionFilterDimension::Platform,
                "SERVING_RESTRICTION" => {
                    MediationReportSpecDimensionFilterDimension::ServingRestriction
                }
                "WEEK" => MediationReportSpecDimensionFilterDimension::Week,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MediationReportSpecDimensionFilterDimension {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MediationReportSpecDimensionFilterDimension {
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
    pub struct MediationReportSpecSortCondition {
        #[doc = "Sort by the specified dimension."]
        #[serde(
            rename = "dimension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension:
            ::std::option::Option<crate::schemas::MediationReportSpecSortConditionDimension>,
        #[doc = "Sort by the specified metric."]
        #[serde(
            rename = "metric",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric: ::std::option::Option<crate::schemas::MediationReportSpecSortConditionMetric>,
        #[doc = "Sorting order of the dimension or metric."]
        #[serde(
            rename = "order",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub order: ::std::option::Option<crate::schemas::MediationReportSpecSortConditionOrder>,
    }
    impl ::google_field_selector::FieldSelector for MediationReportSpecSortCondition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MediationReportSpecSortCondition {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MediationReportSpecSortConditionDimension {
        #[doc = "The [unique ID of the ad source](/admob/api/v1/ad_sources) (for example, \"5450213213286189855\" and \"AdMob Network\" as label value)."]
        AdSource,
        #[doc = "The unique ID of the ad source instance (for example, \"ca-app-pub-1234:asi:5678\" and \"AdMob (default)\" as label value)."]
        AdSourceInstance,
        #[doc = "The unique ID of the ad unit (for example, \"ca-app-pub-1234/8790\"). If AD_UNIT dimension is specified, then APP is included automatically."]
        AdUnit,
        #[doc = "The unique ID of the mobile application (for example, \"ca-app-pub-1234~1234\")."]
        App,
        #[doc = "For Android, the app version name can be found in versionName in PackageInfo. For iOS, the app version name can be found in CFBundleShortVersionString. **Warning:** The dimension is incompatible with [ESTIMATED_EARNINGS](#Metric.ENUM_VALUES.ESTIMATED_EARNINGS), [OBSERVED_ECPM](#Metric.ENUM_VALUES.OBSERVED_ECPM) metrics."]
        AppVersionName,
        #[doc = "CLDR country code of the place where the ad views/clicks occur (for example, \"US\" or \"FR\"). This is a geography dimension."]
        Country,
        #[doc = "A date in the YYYYMMDD format (for example, \"20210701\"). Requests can specify at most one time dimension."]
        Date,
        #[doc = "Default value for an unset field. Do not use."]
        DimensionUnspecified,
        #[doc = "Format of the ad unit (for example, \"banner\", \"native\"), an ad delivery dimension."]
        Format,
        #[doc = "GMA SDK version, e.g. \"iOS 7.62.0\". **Warning:** The dimension is incompatible with [ESTIMATED_EARNINGS](#Metric.ENUM_VALUES.ESTIMATED_EARNINGS), [OBSERVED_ECPM](#Metric.ENUM_VALUES.OBSERVED_ECPM) metrics."]
        GmaSdkVersion,
        #[doc = "The unique ID of the mediation group (for example, \"ca-app-pub-1234:mg:1234\" and \"AdMob (default)\" as label value)."]
        MediationGroup,
        #[doc = "Mobile operating system version, e.g. \"iOS 13.5.1\". **Warning:** The dimension is incompatible with [ESTIMATED_EARNINGS](#Metric.ENUM_VALUES.ESTIMATED_EARNINGS), [OBSERVED_ECPM](#Metric.ENUM_VALUES.OBSERVED_ECPM) metrics."]
        MobileOsVersion,
        #[doc = "A month in the YYYYMM format (for example, \"202107\"). Requests can specify at most one time dimension."]
        Month,
        #[doc = "Mobile OS platform of the app (for example, \"Android\" or \"iOS\")."]
        Platform,
        #[doc = "Restriction mode for ads serving (e.g. \"Non-personalized ads\"). **Warning:** The dimension is incompatible with [ESTIMATED_EARNINGS](#Metric.ENUM_VALUES.ESTIMATED_EARNINGS) metric."]
        ServingRestriction,
        #[doc = "The date of the first day of a week in the YYYYMMDD format (for example, \"20210701\"). Requests can specify at most one time dimension."]
        Week,
    }
    impl MediationReportSpecSortConditionDimension {
        pub fn as_str(self) -> &'static str {
            match self {
                MediationReportSpecSortConditionDimension::AdSource => "AD_SOURCE",
                MediationReportSpecSortConditionDimension::AdSourceInstance => "AD_SOURCE_INSTANCE",
                MediationReportSpecSortConditionDimension::AdUnit => "AD_UNIT",
                MediationReportSpecSortConditionDimension::App => "APP",
                MediationReportSpecSortConditionDimension::AppVersionName => "APP_VERSION_NAME",
                MediationReportSpecSortConditionDimension::Country => "COUNTRY",
                MediationReportSpecSortConditionDimension::Date => "DATE",
                MediationReportSpecSortConditionDimension::DimensionUnspecified => {
                    "DIMENSION_UNSPECIFIED"
                }
                MediationReportSpecSortConditionDimension::Format => "FORMAT",
                MediationReportSpecSortConditionDimension::GmaSdkVersion => "GMA_SDK_VERSION",
                MediationReportSpecSortConditionDimension::MediationGroup => "MEDIATION_GROUP",
                MediationReportSpecSortConditionDimension::MobileOsVersion => "MOBILE_OS_VERSION",
                MediationReportSpecSortConditionDimension::Month => "MONTH",
                MediationReportSpecSortConditionDimension::Platform => "PLATFORM",
                MediationReportSpecSortConditionDimension::ServingRestriction => {
                    "SERVING_RESTRICTION"
                }
                MediationReportSpecSortConditionDimension::Week => "WEEK",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MediationReportSpecSortConditionDimension {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MediationReportSpecSortConditionDimension {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<MediationReportSpecSortConditionDimension, ()> {
            Ok(match s {
                "AD_SOURCE" => MediationReportSpecSortConditionDimension::AdSource,
                "AD_SOURCE_INSTANCE" => MediationReportSpecSortConditionDimension::AdSourceInstance,
                "AD_UNIT" => MediationReportSpecSortConditionDimension::AdUnit,
                "APP" => MediationReportSpecSortConditionDimension::App,
                "APP_VERSION_NAME" => MediationReportSpecSortConditionDimension::AppVersionName,
                "COUNTRY" => MediationReportSpecSortConditionDimension::Country,
                "DATE" => MediationReportSpecSortConditionDimension::Date,
                "DIMENSION_UNSPECIFIED" => {
                    MediationReportSpecSortConditionDimension::DimensionUnspecified
                }
                "FORMAT" => MediationReportSpecSortConditionDimension::Format,
                "GMA_SDK_VERSION" => MediationReportSpecSortConditionDimension::GmaSdkVersion,
                "MEDIATION_GROUP" => MediationReportSpecSortConditionDimension::MediationGroup,
                "MOBILE_OS_VERSION" => MediationReportSpecSortConditionDimension::MobileOsVersion,
                "MONTH" => MediationReportSpecSortConditionDimension::Month,
                "PLATFORM" => MediationReportSpecSortConditionDimension::Platform,
                "SERVING_RESTRICTION" => {
                    MediationReportSpecSortConditionDimension::ServingRestriction
                }
                "WEEK" => MediationReportSpecSortConditionDimension::Week,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MediationReportSpecSortConditionDimension {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MediationReportSpecSortConditionDimension {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MediationReportSpecSortConditionDimension {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AD_SOURCE" => MediationReportSpecSortConditionDimension::AdSource,
                "AD_SOURCE_INSTANCE" => MediationReportSpecSortConditionDimension::AdSourceInstance,
                "AD_UNIT" => MediationReportSpecSortConditionDimension::AdUnit,
                "APP" => MediationReportSpecSortConditionDimension::App,
                "APP_VERSION_NAME" => MediationReportSpecSortConditionDimension::AppVersionName,
                "COUNTRY" => MediationReportSpecSortConditionDimension::Country,
                "DATE" => MediationReportSpecSortConditionDimension::Date,
                "DIMENSION_UNSPECIFIED" => {
                    MediationReportSpecSortConditionDimension::DimensionUnspecified
                }
                "FORMAT" => MediationReportSpecSortConditionDimension::Format,
                "GMA_SDK_VERSION" => MediationReportSpecSortConditionDimension::GmaSdkVersion,
                "MEDIATION_GROUP" => MediationReportSpecSortConditionDimension::MediationGroup,
                "MOBILE_OS_VERSION" => MediationReportSpecSortConditionDimension::MobileOsVersion,
                "MONTH" => MediationReportSpecSortConditionDimension::Month,
                "PLATFORM" => MediationReportSpecSortConditionDimension::Platform,
                "SERVING_RESTRICTION" => {
                    MediationReportSpecSortConditionDimension::ServingRestriction
                }
                "WEEK" => MediationReportSpecSortConditionDimension::Week,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MediationReportSpecSortConditionDimension {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MediationReportSpecSortConditionDimension {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MediationReportSpecSortConditionMetric {
        #[doc = "The number of requests. The value is an integer."]
        AdRequests,
        #[doc = "The number of times a user clicks an ad. The value is an integer."]
        Clicks,
        #[doc = "The estimated earnings of the AdMob publisher. The currency unit (USD, EUR, or other) of the earning metrics are determined by the localization setting for currency. The amount is in micros. For example, $6.50 would be represented as 6500000. Estimated earnings per mediation group and per ad source instance level is supported dating back to October 20, 2019. Third-party estimated earnings will show 0 for dates prior to October 20, 2019."]
        EstimatedEarnings,
        #[doc = "The ratio of clicks over impressions. The value is a double precision (approximate) decimal value."]
        ImpressionCtr,
        #[doc = "The total number of ads shown to users. The value is an integer."]
        Impressions,
        #[doc = "The ratio of matched ad requests over the total ad requests. The value is a double precision (approximate) decimal value."]
        MatchRate,
        #[doc = "The number of times ads are returned in response to a request. The value is an integer."]
        MatchedRequests,
        #[doc = "Default value for an unset field. Do not use."]
        MetricUnspecified,
        #[doc = "The third-party ad network's estimated average eCPM. The currency unit (USD, EUR, or other) of the earning metrics are determined by the localization setting for currency. The amount is in micros. For example, $2.30 would be represented as 2300000. The estimated average eCPM per mediation group and per ad source instance level is supported dating back to October 20, 2019. Third-party estimated average eCPM will show 0 for dates prior to October 20, 2019."]
        ObservedEcpm,
    }
    impl MediationReportSpecSortConditionMetric {
        pub fn as_str(self) -> &'static str {
            match self {
                MediationReportSpecSortConditionMetric::AdRequests => "AD_REQUESTS",
                MediationReportSpecSortConditionMetric::Clicks => "CLICKS",
                MediationReportSpecSortConditionMetric::EstimatedEarnings => "ESTIMATED_EARNINGS",
                MediationReportSpecSortConditionMetric::ImpressionCtr => "IMPRESSION_CTR",
                MediationReportSpecSortConditionMetric::Impressions => "IMPRESSIONS",
                MediationReportSpecSortConditionMetric::MatchRate => "MATCH_RATE",
                MediationReportSpecSortConditionMetric::MatchedRequests => "MATCHED_REQUESTS",
                MediationReportSpecSortConditionMetric::MetricUnspecified => "METRIC_UNSPECIFIED",
                MediationReportSpecSortConditionMetric::ObservedEcpm => "OBSERVED_ECPM",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MediationReportSpecSortConditionMetric {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MediationReportSpecSortConditionMetric {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MediationReportSpecSortConditionMetric, ()> {
            Ok(match s {
                "AD_REQUESTS" => MediationReportSpecSortConditionMetric::AdRequests,
                "CLICKS" => MediationReportSpecSortConditionMetric::Clicks,
                "ESTIMATED_EARNINGS" => MediationReportSpecSortConditionMetric::EstimatedEarnings,
                "IMPRESSION_CTR" => MediationReportSpecSortConditionMetric::ImpressionCtr,
                "IMPRESSIONS" => MediationReportSpecSortConditionMetric::Impressions,
                "MATCH_RATE" => MediationReportSpecSortConditionMetric::MatchRate,
                "MATCHED_REQUESTS" => MediationReportSpecSortConditionMetric::MatchedRequests,
                "METRIC_UNSPECIFIED" => MediationReportSpecSortConditionMetric::MetricUnspecified,
                "OBSERVED_ECPM" => MediationReportSpecSortConditionMetric::ObservedEcpm,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MediationReportSpecSortConditionMetric {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MediationReportSpecSortConditionMetric {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MediationReportSpecSortConditionMetric {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AD_REQUESTS" => MediationReportSpecSortConditionMetric::AdRequests,
                "CLICKS" => MediationReportSpecSortConditionMetric::Clicks,
                "ESTIMATED_EARNINGS" => MediationReportSpecSortConditionMetric::EstimatedEarnings,
                "IMPRESSION_CTR" => MediationReportSpecSortConditionMetric::ImpressionCtr,
                "IMPRESSIONS" => MediationReportSpecSortConditionMetric::Impressions,
                "MATCH_RATE" => MediationReportSpecSortConditionMetric::MatchRate,
                "MATCHED_REQUESTS" => MediationReportSpecSortConditionMetric::MatchedRequests,
                "METRIC_UNSPECIFIED" => MediationReportSpecSortConditionMetric::MetricUnspecified,
                "OBSERVED_ECPM" => MediationReportSpecSortConditionMetric::ObservedEcpm,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MediationReportSpecSortConditionMetric {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MediationReportSpecSortConditionMetric {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MediationReportSpecSortConditionOrder {
        #[doc = "Sort dimension value or metric value in ascending order."]
        Ascending,
        #[doc = "Sort dimension value or metric value in descending order."]
        Descending,
        #[doc = "Default value for an unset field. Do not use."]
        SortOrderUnspecified,
    }
    impl MediationReportSpecSortConditionOrder {
        pub fn as_str(self) -> &'static str {
            match self {
                MediationReportSpecSortConditionOrder::Ascending => "ASCENDING",
                MediationReportSpecSortConditionOrder::Descending => "DESCENDING",
                MediationReportSpecSortConditionOrder::SortOrderUnspecified => {
                    "SORT_ORDER_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for MediationReportSpecSortConditionOrder {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MediationReportSpecSortConditionOrder {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MediationReportSpecSortConditionOrder, ()> {
            Ok(match s {
                "ASCENDING" => MediationReportSpecSortConditionOrder::Ascending,
                "DESCENDING" => MediationReportSpecSortConditionOrder::Descending,
                "SORT_ORDER_UNSPECIFIED" => {
                    MediationReportSpecSortConditionOrder::SortOrderUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MediationReportSpecSortConditionOrder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MediationReportSpecSortConditionOrder {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MediationReportSpecSortConditionOrder {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ASCENDING" => MediationReportSpecSortConditionOrder::Ascending,
                "DESCENDING" => MediationReportSpecSortConditionOrder::Descending,
                "SORT_ORDER_UNSPECIFIED" => {
                    MediationReportSpecSortConditionOrder::SortOrderUnspecified
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
    impl ::google_field_selector::FieldSelector for MediationReportSpecSortConditionOrder {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MediationReportSpecSortConditionOrder {
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
    pub struct NetworkReportSpec {
        #[doc = "The date range for which the report is generated."]
        #[serde(
            rename = "dateRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_range: ::std::option::Option<crate::schemas::DateRange>,
        #[doc = "Describes which report rows to match based on their dimension values."]
        #[serde(
            rename = "dimensionFilters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_filters:
            ::std::option::Option<Vec<crate::schemas::NetworkReportSpecDimensionFilter>>,
        #[doc = "List of dimensions of the report. The value combination of these dimensions determines the row of the report. If no dimensions are specified, the report returns a single row of requested metrics for the entire account."]
        #[serde(
            rename = "dimensions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimensions:
            ::std::option::Option<Vec<crate::schemas::NetworkReportSpecDimensionsItems>>,
        #[doc = "Localization settings of the report."]
        #[serde(
            rename = "localizationSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub localization_settings: ::std::option::Option<crate::schemas::LocalizationSettings>,
        #[doc = "Maximum number of report data rows to return. If the value is not set, the API returns as many rows as possible, up to 100000. Acceptable values are 1-100000, inclusive. Values larger than 100000 return an error."]
        #[serde(
            rename = "maxReportRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_report_rows: ::std::option::Option<i32>,
        #[doc = "List of metrics of the report. A report must specify at least one metric."]
        #[serde(
            rename = "metrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metrics: ::std::option::Option<Vec<crate::schemas::NetworkReportSpecMetricsItems>>,
        #[doc = "Describes the sorting of report rows. The order of the condition in the list defines its precedence; the earlier the condition, the higher its precedence. If no sort conditions are specified, the row ordering is undefined."]
        #[serde(
            rename = "sortConditions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sort_conditions:
            ::std::option::Option<Vec<crate::schemas::NetworkReportSpecSortCondition>>,
        #[doc = "A report time zone. Accepts an IANA TZ name values, such as \"America/Los_Angeles.\" If no time zone is defined, the account default takes effect. Check default value by the get account action. **Warning:** The \"America/Los_Angeles\" is the only supported value at the moment."]
        #[serde(
            rename = "timeZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_zone: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for NetworkReportSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NetworkReportSpec {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NetworkReportSpecDimensionsItems {
        #[doc = "Type of the ad (for example, \"text\" or \"image\"), an ad delivery dimension. **Warning:** The dimension is incompatible with [AD_REQUESTS](#Metric.ENUM_VALUES.AD_REQUESTS), [MATCH_RATE](#Metric.ENUM_VALUES.MATCH_RATE) and [IMPRESSION_RPM](#Metric.ENUM_VALUES.IMPRESSION_RPM) metrics."]
        AdType,
        #[doc = "The unique ID of the ad unit (for example, \"ca-app-pub-1234/1234\"). If AD_UNIT dimension is specified, then APP is included automatically."]
        AdUnit,
        #[doc = "The unique ID of the mobile application (for example, \"ca-app-pub-1234~1234\")."]
        App,
        #[doc = "For Android, the app version name can be found in versionName in PackageInfo. For iOS, the app version name can be found in CFBundleShortVersionString."]
        AppVersionName,
        #[doc = "CLDR country code of the place where the ad views/clicks occur (for example, \"US\" or \"FR\"). This is a geography dimension."]
        Country,
        #[doc = "A date in the YYYYMMDD format (for example, \"20210701\"). Requests can specify at most one time dimension."]
        Date,
        #[doc = "Default value for an unset field. Do not use."]
        DimensionUnspecified,
        #[doc = "Format of the ad unit (for example, \"banner\", \"native\"), an ad delivery dimension."]
        Format,
        #[doc = "GMA SDK version, e.g. \"iOS 7.62.0\"."]
        GmaSdkVersion,
        #[doc = "Mobile operating system version, e.g. \"iOS 13.5.1\"."]
        MobileOsVersion,
        #[doc = "A month in the YYYYMM format (for example, \"202107\"). Requests can specify at most one time dimension."]
        Month,
        #[doc = "Mobile OS platform of the app (for example, \"Android\" or \"iOS\")."]
        Platform,
        #[doc = "Restriction mode for ads serving (e.g. \"Non-personalized ads\")."]
        ServingRestriction,
        #[doc = "The date of the first day of a week in the YYYYMMDD format (for example, \"20210701\"). Requests can specify at most one time dimension."]
        Week,
    }
    impl NetworkReportSpecDimensionsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                NetworkReportSpecDimensionsItems::AdType => "AD_TYPE",
                NetworkReportSpecDimensionsItems::AdUnit => "AD_UNIT",
                NetworkReportSpecDimensionsItems::App => "APP",
                NetworkReportSpecDimensionsItems::AppVersionName => "APP_VERSION_NAME",
                NetworkReportSpecDimensionsItems::Country => "COUNTRY",
                NetworkReportSpecDimensionsItems::Date => "DATE",
                NetworkReportSpecDimensionsItems::DimensionUnspecified => "DIMENSION_UNSPECIFIED",
                NetworkReportSpecDimensionsItems::Format => "FORMAT",
                NetworkReportSpecDimensionsItems::GmaSdkVersion => "GMA_SDK_VERSION",
                NetworkReportSpecDimensionsItems::MobileOsVersion => "MOBILE_OS_VERSION",
                NetworkReportSpecDimensionsItems::Month => "MONTH",
                NetworkReportSpecDimensionsItems::Platform => "PLATFORM",
                NetworkReportSpecDimensionsItems::ServingRestriction => "SERVING_RESTRICTION",
                NetworkReportSpecDimensionsItems::Week => "WEEK",
            }
        }
    }
    impl ::std::convert::AsRef<str> for NetworkReportSpecDimensionsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for NetworkReportSpecDimensionsItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<NetworkReportSpecDimensionsItems, ()> {
            Ok(match s {
                "AD_TYPE" => NetworkReportSpecDimensionsItems::AdType,
                "AD_UNIT" => NetworkReportSpecDimensionsItems::AdUnit,
                "APP" => NetworkReportSpecDimensionsItems::App,
                "APP_VERSION_NAME" => NetworkReportSpecDimensionsItems::AppVersionName,
                "COUNTRY" => NetworkReportSpecDimensionsItems::Country,
                "DATE" => NetworkReportSpecDimensionsItems::Date,
                "DIMENSION_UNSPECIFIED" => NetworkReportSpecDimensionsItems::DimensionUnspecified,
                "FORMAT" => NetworkReportSpecDimensionsItems::Format,
                "GMA_SDK_VERSION" => NetworkReportSpecDimensionsItems::GmaSdkVersion,
                "MOBILE_OS_VERSION" => NetworkReportSpecDimensionsItems::MobileOsVersion,
                "MONTH" => NetworkReportSpecDimensionsItems::Month,
                "PLATFORM" => NetworkReportSpecDimensionsItems::Platform,
                "SERVING_RESTRICTION" => NetworkReportSpecDimensionsItems::ServingRestriction,
                "WEEK" => NetworkReportSpecDimensionsItems::Week,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for NetworkReportSpecDimensionsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NetworkReportSpecDimensionsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NetworkReportSpecDimensionsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AD_TYPE" => NetworkReportSpecDimensionsItems::AdType,
                "AD_UNIT" => NetworkReportSpecDimensionsItems::AdUnit,
                "APP" => NetworkReportSpecDimensionsItems::App,
                "APP_VERSION_NAME" => NetworkReportSpecDimensionsItems::AppVersionName,
                "COUNTRY" => NetworkReportSpecDimensionsItems::Country,
                "DATE" => NetworkReportSpecDimensionsItems::Date,
                "DIMENSION_UNSPECIFIED" => NetworkReportSpecDimensionsItems::DimensionUnspecified,
                "FORMAT" => NetworkReportSpecDimensionsItems::Format,
                "GMA_SDK_VERSION" => NetworkReportSpecDimensionsItems::GmaSdkVersion,
                "MOBILE_OS_VERSION" => NetworkReportSpecDimensionsItems::MobileOsVersion,
                "MONTH" => NetworkReportSpecDimensionsItems::Month,
                "PLATFORM" => NetworkReportSpecDimensionsItems::Platform,
                "SERVING_RESTRICTION" => NetworkReportSpecDimensionsItems::ServingRestriction,
                "WEEK" => NetworkReportSpecDimensionsItems::Week,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for NetworkReportSpecDimensionsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NetworkReportSpecDimensionsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NetworkReportSpecMetricsItems {
        #[doc = "The number of ad requests. The value is an integer. **Warning:** The metric is incompatible with [AD_TYPE](#Dimension.ENUM_VALUES.AD_TYPE) dimension."]
        AdRequests,
        #[doc = "The number of times a user clicks an ad. The value is an integer."]
        Clicks,
        #[doc = "The estimated earnings of the AdMob publisher. The currency unit (USD, EUR, or other) of the earning metrics are determined by the localization setting for currency. The amount is in micros. For example, $6.50 would be represented as 6500000."]
        EstimatedEarnings,
        #[doc = "The ratio of clicks over impressions. The value is a double precision (approximate) decimal value."]
        ImpressionCtr,
        #[doc = "The estimated earnings per thousand ad impressions. The value is in micros. For example, $1.03 would be represented as 1030000. Equivalent to eCPM in the AdMob UI. **Warning:** The metric is incompatible with [AD_TYPE](#Dimension.ENUM_VALUES.AD_TYPE) dimension."]
        ImpressionRpm,
        #[doc = "The total number of ads shown to users. The value is an integer."]
        Impressions,
        #[doc = "The ratio of matched ad requests over the total ad requests. The value is a double precision (approximate) decimal value. **Warning:** The metric is incompatible with [AD_TYPE](#Dimension.ENUM_VALUES.AD_TYPE) dimension."]
        MatchRate,
        #[doc = "The number of times ads are returned in response to a request. The value is an integer."]
        MatchedRequests,
        #[doc = "Default value for an unset field. Do not use."]
        MetricUnspecified,
        #[doc = "The ratio of ads that are displayed over ads that are returned, defined as impressions / matched requests. The value is a double precision (approximate) decimal value."]
        ShowRate,
    }
    impl NetworkReportSpecMetricsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                NetworkReportSpecMetricsItems::AdRequests => "AD_REQUESTS",
                NetworkReportSpecMetricsItems::Clicks => "CLICKS",
                NetworkReportSpecMetricsItems::EstimatedEarnings => "ESTIMATED_EARNINGS",
                NetworkReportSpecMetricsItems::ImpressionCtr => "IMPRESSION_CTR",
                NetworkReportSpecMetricsItems::ImpressionRpm => "IMPRESSION_RPM",
                NetworkReportSpecMetricsItems::Impressions => "IMPRESSIONS",
                NetworkReportSpecMetricsItems::MatchRate => "MATCH_RATE",
                NetworkReportSpecMetricsItems::MatchedRequests => "MATCHED_REQUESTS",
                NetworkReportSpecMetricsItems::MetricUnspecified => "METRIC_UNSPECIFIED",
                NetworkReportSpecMetricsItems::ShowRate => "SHOW_RATE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for NetworkReportSpecMetricsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for NetworkReportSpecMetricsItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<NetworkReportSpecMetricsItems, ()> {
            Ok(match s {
                "AD_REQUESTS" => NetworkReportSpecMetricsItems::AdRequests,
                "CLICKS" => NetworkReportSpecMetricsItems::Clicks,
                "ESTIMATED_EARNINGS" => NetworkReportSpecMetricsItems::EstimatedEarnings,
                "IMPRESSION_CTR" => NetworkReportSpecMetricsItems::ImpressionCtr,
                "IMPRESSION_RPM" => NetworkReportSpecMetricsItems::ImpressionRpm,
                "IMPRESSIONS" => NetworkReportSpecMetricsItems::Impressions,
                "MATCH_RATE" => NetworkReportSpecMetricsItems::MatchRate,
                "MATCHED_REQUESTS" => NetworkReportSpecMetricsItems::MatchedRequests,
                "METRIC_UNSPECIFIED" => NetworkReportSpecMetricsItems::MetricUnspecified,
                "SHOW_RATE" => NetworkReportSpecMetricsItems::ShowRate,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for NetworkReportSpecMetricsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NetworkReportSpecMetricsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NetworkReportSpecMetricsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AD_REQUESTS" => NetworkReportSpecMetricsItems::AdRequests,
                "CLICKS" => NetworkReportSpecMetricsItems::Clicks,
                "ESTIMATED_EARNINGS" => NetworkReportSpecMetricsItems::EstimatedEarnings,
                "IMPRESSION_CTR" => NetworkReportSpecMetricsItems::ImpressionCtr,
                "IMPRESSION_RPM" => NetworkReportSpecMetricsItems::ImpressionRpm,
                "IMPRESSIONS" => NetworkReportSpecMetricsItems::Impressions,
                "MATCH_RATE" => NetworkReportSpecMetricsItems::MatchRate,
                "MATCHED_REQUESTS" => NetworkReportSpecMetricsItems::MatchedRequests,
                "METRIC_UNSPECIFIED" => NetworkReportSpecMetricsItems::MetricUnspecified,
                "SHOW_RATE" => NetworkReportSpecMetricsItems::ShowRate,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for NetworkReportSpecMetricsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NetworkReportSpecMetricsItems {
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
    pub struct NetworkReportSpecDimensionFilter {
        #[doc = "Applies the filter criterion to the specified dimension."]
        #[serde(
            rename = "dimension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension:
            ::std::option::Option<crate::schemas::NetworkReportSpecDimensionFilterDimension>,
        #[doc = "Matches a row if its value for the specified dimension is in one of the values specified in this condition."]
        #[serde(
            rename = "matchesAny",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub matches_any: ::std::option::Option<crate::schemas::StringList>,
    }
    impl ::google_field_selector::FieldSelector for NetworkReportSpecDimensionFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NetworkReportSpecDimensionFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NetworkReportSpecDimensionFilterDimension {
        #[doc = "Type of the ad (for example, \"text\" or \"image\"), an ad delivery dimension. **Warning:** The dimension is incompatible with [AD_REQUESTS](#Metric.ENUM_VALUES.AD_REQUESTS), [MATCH_RATE](#Metric.ENUM_VALUES.MATCH_RATE) and [IMPRESSION_RPM](#Metric.ENUM_VALUES.IMPRESSION_RPM) metrics."]
        AdType,
        #[doc = "The unique ID of the ad unit (for example, \"ca-app-pub-1234/1234\"). If AD_UNIT dimension is specified, then APP is included automatically."]
        AdUnit,
        #[doc = "The unique ID of the mobile application (for example, \"ca-app-pub-1234~1234\")."]
        App,
        #[doc = "For Android, the app version name can be found in versionName in PackageInfo. For iOS, the app version name can be found in CFBundleShortVersionString."]
        AppVersionName,
        #[doc = "CLDR country code of the place where the ad views/clicks occur (for example, \"US\" or \"FR\"). This is a geography dimension."]
        Country,
        #[doc = "A date in the YYYYMMDD format (for example, \"20210701\"). Requests can specify at most one time dimension."]
        Date,
        #[doc = "Default value for an unset field. Do not use."]
        DimensionUnspecified,
        #[doc = "Format of the ad unit (for example, \"banner\", \"native\"), an ad delivery dimension."]
        Format,
        #[doc = "GMA SDK version, e.g. \"iOS 7.62.0\"."]
        GmaSdkVersion,
        #[doc = "Mobile operating system version, e.g. \"iOS 13.5.1\"."]
        MobileOsVersion,
        #[doc = "A month in the YYYYMM format (for example, \"202107\"). Requests can specify at most one time dimension."]
        Month,
        #[doc = "Mobile OS platform of the app (for example, \"Android\" or \"iOS\")."]
        Platform,
        #[doc = "Restriction mode for ads serving (e.g. \"Non-personalized ads\")."]
        ServingRestriction,
        #[doc = "The date of the first day of a week in the YYYYMMDD format (for example, \"20210701\"). Requests can specify at most one time dimension."]
        Week,
    }
    impl NetworkReportSpecDimensionFilterDimension {
        pub fn as_str(self) -> &'static str {
            match self {
                NetworkReportSpecDimensionFilterDimension::AdType => "AD_TYPE",
                NetworkReportSpecDimensionFilterDimension::AdUnit => "AD_UNIT",
                NetworkReportSpecDimensionFilterDimension::App => "APP",
                NetworkReportSpecDimensionFilterDimension::AppVersionName => "APP_VERSION_NAME",
                NetworkReportSpecDimensionFilterDimension::Country => "COUNTRY",
                NetworkReportSpecDimensionFilterDimension::Date => "DATE",
                NetworkReportSpecDimensionFilterDimension::DimensionUnspecified => {
                    "DIMENSION_UNSPECIFIED"
                }
                NetworkReportSpecDimensionFilterDimension::Format => "FORMAT",
                NetworkReportSpecDimensionFilterDimension::GmaSdkVersion => "GMA_SDK_VERSION",
                NetworkReportSpecDimensionFilterDimension::MobileOsVersion => "MOBILE_OS_VERSION",
                NetworkReportSpecDimensionFilterDimension::Month => "MONTH",
                NetworkReportSpecDimensionFilterDimension::Platform => "PLATFORM",
                NetworkReportSpecDimensionFilterDimension::ServingRestriction => {
                    "SERVING_RESTRICTION"
                }
                NetworkReportSpecDimensionFilterDimension::Week => "WEEK",
            }
        }
    }
    impl ::std::convert::AsRef<str> for NetworkReportSpecDimensionFilterDimension {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for NetworkReportSpecDimensionFilterDimension {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<NetworkReportSpecDimensionFilterDimension, ()> {
            Ok(match s {
                "AD_TYPE" => NetworkReportSpecDimensionFilterDimension::AdType,
                "AD_UNIT" => NetworkReportSpecDimensionFilterDimension::AdUnit,
                "APP" => NetworkReportSpecDimensionFilterDimension::App,
                "APP_VERSION_NAME" => NetworkReportSpecDimensionFilterDimension::AppVersionName,
                "COUNTRY" => NetworkReportSpecDimensionFilterDimension::Country,
                "DATE" => NetworkReportSpecDimensionFilterDimension::Date,
                "DIMENSION_UNSPECIFIED" => {
                    NetworkReportSpecDimensionFilterDimension::DimensionUnspecified
                }
                "FORMAT" => NetworkReportSpecDimensionFilterDimension::Format,
                "GMA_SDK_VERSION" => NetworkReportSpecDimensionFilterDimension::GmaSdkVersion,
                "MOBILE_OS_VERSION" => NetworkReportSpecDimensionFilterDimension::MobileOsVersion,
                "MONTH" => NetworkReportSpecDimensionFilterDimension::Month,
                "PLATFORM" => NetworkReportSpecDimensionFilterDimension::Platform,
                "SERVING_RESTRICTION" => {
                    NetworkReportSpecDimensionFilterDimension::ServingRestriction
                }
                "WEEK" => NetworkReportSpecDimensionFilterDimension::Week,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for NetworkReportSpecDimensionFilterDimension {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NetworkReportSpecDimensionFilterDimension {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NetworkReportSpecDimensionFilterDimension {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AD_TYPE" => NetworkReportSpecDimensionFilterDimension::AdType,
                "AD_UNIT" => NetworkReportSpecDimensionFilterDimension::AdUnit,
                "APP" => NetworkReportSpecDimensionFilterDimension::App,
                "APP_VERSION_NAME" => NetworkReportSpecDimensionFilterDimension::AppVersionName,
                "COUNTRY" => NetworkReportSpecDimensionFilterDimension::Country,
                "DATE" => NetworkReportSpecDimensionFilterDimension::Date,
                "DIMENSION_UNSPECIFIED" => {
                    NetworkReportSpecDimensionFilterDimension::DimensionUnspecified
                }
                "FORMAT" => NetworkReportSpecDimensionFilterDimension::Format,
                "GMA_SDK_VERSION" => NetworkReportSpecDimensionFilterDimension::GmaSdkVersion,
                "MOBILE_OS_VERSION" => NetworkReportSpecDimensionFilterDimension::MobileOsVersion,
                "MONTH" => NetworkReportSpecDimensionFilterDimension::Month,
                "PLATFORM" => NetworkReportSpecDimensionFilterDimension::Platform,
                "SERVING_RESTRICTION" => {
                    NetworkReportSpecDimensionFilterDimension::ServingRestriction
                }
                "WEEK" => NetworkReportSpecDimensionFilterDimension::Week,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for NetworkReportSpecDimensionFilterDimension {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NetworkReportSpecDimensionFilterDimension {
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
    pub struct NetworkReportSpecSortCondition {
        #[doc = "Sort by the specified dimension."]
        #[serde(
            rename = "dimension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension:
            ::std::option::Option<crate::schemas::NetworkReportSpecSortConditionDimension>,
        #[doc = "Sort by the specified metric."]
        #[serde(
            rename = "metric",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric: ::std::option::Option<crate::schemas::NetworkReportSpecSortConditionMetric>,
        #[doc = "Sorting order of the dimension or metric."]
        #[serde(
            rename = "order",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub order: ::std::option::Option<crate::schemas::NetworkReportSpecSortConditionOrder>,
    }
    impl ::google_field_selector::FieldSelector for NetworkReportSpecSortCondition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NetworkReportSpecSortCondition {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NetworkReportSpecSortConditionDimension {
        #[doc = "Type of the ad (for example, \"text\" or \"image\"), an ad delivery dimension. **Warning:** The dimension is incompatible with [AD_REQUESTS](#Metric.ENUM_VALUES.AD_REQUESTS), [MATCH_RATE](#Metric.ENUM_VALUES.MATCH_RATE) and [IMPRESSION_RPM](#Metric.ENUM_VALUES.IMPRESSION_RPM) metrics."]
        AdType,
        #[doc = "The unique ID of the ad unit (for example, \"ca-app-pub-1234/1234\"). If AD_UNIT dimension is specified, then APP is included automatically."]
        AdUnit,
        #[doc = "The unique ID of the mobile application (for example, \"ca-app-pub-1234~1234\")."]
        App,
        #[doc = "For Android, the app version name can be found in versionName in PackageInfo. For iOS, the app version name can be found in CFBundleShortVersionString."]
        AppVersionName,
        #[doc = "CLDR country code of the place where the ad views/clicks occur (for example, \"US\" or \"FR\"). This is a geography dimension."]
        Country,
        #[doc = "A date in the YYYYMMDD format (for example, \"20210701\"). Requests can specify at most one time dimension."]
        Date,
        #[doc = "Default value for an unset field. Do not use."]
        DimensionUnspecified,
        #[doc = "Format of the ad unit (for example, \"banner\", \"native\"), an ad delivery dimension."]
        Format,
        #[doc = "GMA SDK version, e.g. \"iOS 7.62.0\"."]
        GmaSdkVersion,
        #[doc = "Mobile operating system version, e.g. \"iOS 13.5.1\"."]
        MobileOsVersion,
        #[doc = "A month in the YYYYMM format (for example, \"202107\"). Requests can specify at most one time dimension."]
        Month,
        #[doc = "Mobile OS platform of the app (for example, \"Android\" or \"iOS\")."]
        Platform,
        #[doc = "Restriction mode for ads serving (e.g. \"Non-personalized ads\")."]
        ServingRestriction,
        #[doc = "The date of the first day of a week in the YYYYMMDD format (for example, \"20210701\"). Requests can specify at most one time dimension."]
        Week,
    }
    impl NetworkReportSpecSortConditionDimension {
        pub fn as_str(self) -> &'static str {
            match self {
                NetworkReportSpecSortConditionDimension::AdType => "AD_TYPE",
                NetworkReportSpecSortConditionDimension::AdUnit => "AD_UNIT",
                NetworkReportSpecSortConditionDimension::App => "APP",
                NetworkReportSpecSortConditionDimension::AppVersionName => "APP_VERSION_NAME",
                NetworkReportSpecSortConditionDimension::Country => "COUNTRY",
                NetworkReportSpecSortConditionDimension::Date => "DATE",
                NetworkReportSpecSortConditionDimension::DimensionUnspecified => {
                    "DIMENSION_UNSPECIFIED"
                }
                NetworkReportSpecSortConditionDimension::Format => "FORMAT",
                NetworkReportSpecSortConditionDimension::GmaSdkVersion => "GMA_SDK_VERSION",
                NetworkReportSpecSortConditionDimension::MobileOsVersion => "MOBILE_OS_VERSION",
                NetworkReportSpecSortConditionDimension::Month => "MONTH",
                NetworkReportSpecSortConditionDimension::Platform => "PLATFORM",
                NetworkReportSpecSortConditionDimension::ServingRestriction => {
                    "SERVING_RESTRICTION"
                }
                NetworkReportSpecSortConditionDimension::Week => "WEEK",
            }
        }
    }
    impl ::std::convert::AsRef<str> for NetworkReportSpecSortConditionDimension {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for NetworkReportSpecSortConditionDimension {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<NetworkReportSpecSortConditionDimension, ()> {
            Ok(match s {
                "AD_TYPE" => NetworkReportSpecSortConditionDimension::AdType,
                "AD_UNIT" => NetworkReportSpecSortConditionDimension::AdUnit,
                "APP" => NetworkReportSpecSortConditionDimension::App,
                "APP_VERSION_NAME" => NetworkReportSpecSortConditionDimension::AppVersionName,
                "COUNTRY" => NetworkReportSpecSortConditionDimension::Country,
                "DATE" => NetworkReportSpecSortConditionDimension::Date,
                "DIMENSION_UNSPECIFIED" => {
                    NetworkReportSpecSortConditionDimension::DimensionUnspecified
                }
                "FORMAT" => NetworkReportSpecSortConditionDimension::Format,
                "GMA_SDK_VERSION" => NetworkReportSpecSortConditionDimension::GmaSdkVersion,
                "MOBILE_OS_VERSION" => NetworkReportSpecSortConditionDimension::MobileOsVersion,
                "MONTH" => NetworkReportSpecSortConditionDimension::Month,
                "PLATFORM" => NetworkReportSpecSortConditionDimension::Platform,
                "SERVING_RESTRICTION" => {
                    NetworkReportSpecSortConditionDimension::ServingRestriction
                }
                "WEEK" => NetworkReportSpecSortConditionDimension::Week,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for NetworkReportSpecSortConditionDimension {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NetworkReportSpecSortConditionDimension {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NetworkReportSpecSortConditionDimension {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AD_TYPE" => NetworkReportSpecSortConditionDimension::AdType,
                "AD_UNIT" => NetworkReportSpecSortConditionDimension::AdUnit,
                "APP" => NetworkReportSpecSortConditionDimension::App,
                "APP_VERSION_NAME" => NetworkReportSpecSortConditionDimension::AppVersionName,
                "COUNTRY" => NetworkReportSpecSortConditionDimension::Country,
                "DATE" => NetworkReportSpecSortConditionDimension::Date,
                "DIMENSION_UNSPECIFIED" => {
                    NetworkReportSpecSortConditionDimension::DimensionUnspecified
                }
                "FORMAT" => NetworkReportSpecSortConditionDimension::Format,
                "GMA_SDK_VERSION" => NetworkReportSpecSortConditionDimension::GmaSdkVersion,
                "MOBILE_OS_VERSION" => NetworkReportSpecSortConditionDimension::MobileOsVersion,
                "MONTH" => NetworkReportSpecSortConditionDimension::Month,
                "PLATFORM" => NetworkReportSpecSortConditionDimension::Platform,
                "SERVING_RESTRICTION" => {
                    NetworkReportSpecSortConditionDimension::ServingRestriction
                }
                "WEEK" => NetworkReportSpecSortConditionDimension::Week,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for NetworkReportSpecSortConditionDimension {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NetworkReportSpecSortConditionDimension {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NetworkReportSpecSortConditionMetric {
        #[doc = "The number of ad requests. The value is an integer. **Warning:** The metric is incompatible with [AD_TYPE](#Dimension.ENUM_VALUES.AD_TYPE) dimension."]
        AdRequests,
        #[doc = "The number of times a user clicks an ad. The value is an integer."]
        Clicks,
        #[doc = "The estimated earnings of the AdMob publisher. The currency unit (USD, EUR, or other) of the earning metrics are determined by the localization setting for currency. The amount is in micros. For example, $6.50 would be represented as 6500000."]
        EstimatedEarnings,
        #[doc = "The ratio of clicks over impressions. The value is a double precision (approximate) decimal value."]
        ImpressionCtr,
        #[doc = "The estimated earnings per thousand ad impressions. The value is in micros. For example, $1.03 would be represented as 1030000. Equivalent to eCPM in the AdMob UI. **Warning:** The metric is incompatible with [AD_TYPE](#Dimension.ENUM_VALUES.AD_TYPE) dimension."]
        ImpressionRpm,
        #[doc = "The total number of ads shown to users. The value is an integer."]
        Impressions,
        #[doc = "The ratio of matched ad requests over the total ad requests. The value is a double precision (approximate) decimal value. **Warning:** The metric is incompatible with [AD_TYPE](#Dimension.ENUM_VALUES.AD_TYPE) dimension."]
        MatchRate,
        #[doc = "The number of times ads are returned in response to a request. The value is an integer."]
        MatchedRequests,
        #[doc = "Default value for an unset field. Do not use."]
        MetricUnspecified,
        #[doc = "The ratio of ads that are displayed over ads that are returned, defined as impressions / matched requests. The value is a double precision (approximate) decimal value."]
        ShowRate,
    }
    impl NetworkReportSpecSortConditionMetric {
        pub fn as_str(self) -> &'static str {
            match self {
                NetworkReportSpecSortConditionMetric::AdRequests => "AD_REQUESTS",
                NetworkReportSpecSortConditionMetric::Clicks => "CLICKS",
                NetworkReportSpecSortConditionMetric::EstimatedEarnings => "ESTIMATED_EARNINGS",
                NetworkReportSpecSortConditionMetric::ImpressionCtr => "IMPRESSION_CTR",
                NetworkReportSpecSortConditionMetric::ImpressionRpm => "IMPRESSION_RPM",
                NetworkReportSpecSortConditionMetric::Impressions => "IMPRESSIONS",
                NetworkReportSpecSortConditionMetric::MatchRate => "MATCH_RATE",
                NetworkReportSpecSortConditionMetric::MatchedRequests => "MATCHED_REQUESTS",
                NetworkReportSpecSortConditionMetric::MetricUnspecified => "METRIC_UNSPECIFIED",
                NetworkReportSpecSortConditionMetric::ShowRate => "SHOW_RATE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for NetworkReportSpecSortConditionMetric {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for NetworkReportSpecSortConditionMetric {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<NetworkReportSpecSortConditionMetric, ()> {
            Ok(match s {
                "AD_REQUESTS" => NetworkReportSpecSortConditionMetric::AdRequests,
                "CLICKS" => NetworkReportSpecSortConditionMetric::Clicks,
                "ESTIMATED_EARNINGS" => NetworkReportSpecSortConditionMetric::EstimatedEarnings,
                "IMPRESSION_CTR" => NetworkReportSpecSortConditionMetric::ImpressionCtr,
                "IMPRESSION_RPM" => NetworkReportSpecSortConditionMetric::ImpressionRpm,
                "IMPRESSIONS" => NetworkReportSpecSortConditionMetric::Impressions,
                "MATCH_RATE" => NetworkReportSpecSortConditionMetric::MatchRate,
                "MATCHED_REQUESTS" => NetworkReportSpecSortConditionMetric::MatchedRequests,
                "METRIC_UNSPECIFIED" => NetworkReportSpecSortConditionMetric::MetricUnspecified,
                "SHOW_RATE" => NetworkReportSpecSortConditionMetric::ShowRate,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for NetworkReportSpecSortConditionMetric {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NetworkReportSpecSortConditionMetric {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NetworkReportSpecSortConditionMetric {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AD_REQUESTS" => NetworkReportSpecSortConditionMetric::AdRequests,
                "CLICKS" => NetworkReportSpecSortConditionMetric::Clicks,
                "ESTIMATED_EARNINGS" => NetworkReportSpecSortConditionMetric::EstimatedEarnings,
                "IMPRESSION_CTR" => NetworkReportSpecSortConditionMetric::ImpressionCtr,
                "IMPRESSION_RPM" => NetworkReportSpecSortConditionMetric::ImpressionRpm,
                "IMPRESSIONS" => NetworkReportSpecSortConditionMetric::Impressions,
                "MATCH_RATE" => NetworkReportSpecSortConditionMetric::MatchRate,
                "MATCHED_REQUESTS" => NetworkReportSpecSortConditionMetric::MatchedRequests,
                "METRIC_UNSPECIFIED" => NetworkReportSpecSortConditionMetric::MetricUnspecified,
                "SHOW_RATE" => NetworkReportSpecSortConditionMetric::ShowRate,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for NetworkReportSpecSortConditionMetric {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NetworkReportSpecSortConditionMetric {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NetworkReportSpecSortConditionOrder {
        #[doc = "Sort dimension value or metric value in ascending order."]
        Ascending,
        #[doc = "Sort dimension value or metric value in descending order."]
        Descending,
        #[doc = "Default value for an unset field. Do not use."]
        SortOrderUnspecified,
    }
    impl NetworkReportSpecSortConditionOrder {
        pub fn as_str(self) -> &'static str {
            match self {
                NetworkReportSpecSortConditionOrder::Ascending => "ASCENDING",
                NetworkReportSpecSortConditionOrder::Descending => "DESCENDING",
                NetworkReportSpecSortConditionOrder::SortOrderUnspecified => {
                    "SORT_ORDER_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for NetworkReportSpecSortConditionOrder {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for NetworkReportSpecSortConditionOrder {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<NetworkReportSpecSortConditionOrder, ()> {
            Ok(match s {
                "ASCENDING" => NetworkReportSpecSortConditionOrder::Ascending,
                "DESCENDING" => NetworkReportSpecSortConditionOrder::Descending,
                "SORT_ORDER_UNSPECIFIED" => {
                    NetworkReportSpecSortConditionOrder::SortOrderUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for NetworkReportSpecSortConditionOrder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NetworkReportSpecSortConditionOrder {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NetworkReportSpecSortConditionOrder {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ASCENDING" => NetworkReportSpecSortConditionOrder::Ascending,
                "DESCENDING" => NetworkReportSpecSortConditionOrder::Descending,
                "SORT_ORDER_UNSPECIFIED" => {
                    NetworkReportSpecSortConditionOrder::SortOrderUnspecified
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
    impl ::google_field_selector::FieldSelector for NetworkReportSpecSortConditionOrder {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NetworkReportSpecSortConditionOrder {
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
    pub struct PublisherAccount {
        #[doc = "Currency code of the earning-related metrics, which is the 3-letter code defined in ISO 4217. The daily average rate is used for the currency conversion."]
        #[serde(
            rename = "currencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency_code: ::std::option::Option<String>,
        #[doc = "Resource name of this account. Format is accounts/{publisher_id}."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The unique ID by which this publisher account can be identified in the API requests (for example, pub-1234567890)."]
        #[serde(
            rename = "publisherId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub publisher_id: ::std::option::Option<String>,
        #[doc = "The time zone that is used in reports that are generated for this account. The value is a time-zone ID as specified by the CLDR project, for example, \"America/Los_Angeles\"."]
        #[serde(
            rename = "reportingTimeZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reporting_time_zone: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PublisherAccount {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PublisherAccount {
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
    pub struct ReportFooter {
        #[doc = "Total number of rows that matched the request. Warning: This count does NOT always match the number of rows in the response. Do not make that assumption when processing the response."]
        #[serde(
            rename = "matchingRowCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub matching_row_count: ::std::option::Option<i64>,
        #[doc = "Warnings associated with generation of the report."]
        #[serde(
            rename = "warnings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub warnings: ::std::option::Option<Vec<crate::schemas::ReportWarning>>,
    }
    impl ::google_field_selector::FieldSelector for ReportFooter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportFooter {
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
    pub struct ReportHeader {
        #[doc = "The date range for which the report is generated. This is identical to the range specified in the report request."]
        #[serde(
            rename = "dateRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_range: ::std::option::Option<crate::schemas::DateRange>,
        #[doc = "Localization settings of the report. This is identical to the settings in the report request."]
        #[serde(
            rename = "localizationSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub localization_settings: ::std::option::Option<crate::schemas::LocalizationSettings>,
        #[doc = "The report time zone. The value is a time-zone ID as specified by the CLDR project, for example, \"America/Los_Angeles\"."]
        #[serde(
            rename = "reportingTimeZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reporting_time_zone: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ReportHeader {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportHeader {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ReportRow {
        #[doc = "Map of dimension values in a row, with keys as enum name of the dimensions."]
        #[serde(
            rename = "dimensionValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_values: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::ReportRowDimensionValue>,
        >,
        #[doc = "Map of metric values in a row, with keys as enum name of the metrics. If a metric being requested has no value returned, the map will not include it."]
        #[serde(
            rename = "metricValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric_values: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::ReportRowMetricValue>,
        >,
    }
    impl ::google_field_selector::FieldSelector for ReportRow {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportRow {
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
    pub struct ReportRowDimensionValue {
        #[doc = "The localized string representation of the value. If unspecified, the display label should be derived from the value."]
        #[serde(
            rename = "displayLabel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_label: ::std::option::Option<String>,
        #[doc = "Dimension value in the format specified in the report's spec Dimension enum."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ReportRowDimensionValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportRowDimensionValue {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ReportRowMetricValue {
        #[doc = "Double precision (approximate) decimal values. Rates are from 0 to 1."]
        #[serde(
            rename = "doubleValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub double_value: ::std::option::Option<f64>,
        #[doc = "Metric integer value."]
        #[serde(
            rename = "integerValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub integer_value: ::std::option::Option<i64>,
        #[doc = "Amount in micros. One million is equivalent to one unit. Currency value is in the unit (USD, EUR or other) specified by the request. For example, $6.50 whould be represented as 6500000 micros."]
        #[serde(
            rename = "microsValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub micros_value: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for ReportRowMetricValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportRowMetricValue {
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
    pub struct ReportWarning {
        #[doc = "Describes the details of the warning message, in English."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Type of the warning."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::ReportWarningType>,
    }
    impl ::google_field_selector::FieldSelector for ReportWarning {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportWarning {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ReportWarningType {
        #[doc = "Some data in this report is aggregated based on a time zone different from the requested time zone. This could happen if a local time-zone report has the start time before the last time this time zone changed. The description field will contain the date of the last time zone change."]
        DataBeforeAccountTimezoneChange,
        #[doc = "There is an unusual delay in processing the source data for the requested date range. The report results might be less up to date than usual. AdMob is aware of the issue and is actively working to resolve it."]
        DataDelayed,
        #[doc = "Warnings that are exposed without a specific type. Useful when new warning types are added but the API is not changed yet."]
        Other,
        #[doc = "The currency being requested is not the account currency. The earning metrics will be based on the requested currency, and thus not a good estimation of the final payment anymore, due to the currency rate fluctuation."]
        ReportCurrencyNotAccountCurrency,
        #[doc = "Default value for an unset field. Do not use."]
        TypeUnspecified,
    }
    impl ReportWarningType {
        pub fn as_str(self) -> &'static str {
            match self {
                ReportWarningType::DataBeforeAccountTimezoneChange => {
                    "DATA_BEFORE_ACCOUNT_TIMEZONE_CHANGE"
                }
                ReportWarningType::DataDelayed => "DATA_DELAYED",
                ReportWarningType::Other => "OTHER",
                ReportWarningType::ReportCurrencyNotAccountCurrency => {
                    "REPORT_CURRENCY_NOT_ACCOUNT_CURRENCY"
                }
                ReportWarningType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ReportWarningType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ReportWarningType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ReportWarningType, ()> {
            Ok(match s {
                "DATA_BEFORE_ACCOUNT_TIMEZONE_CHANGE" => {
                    ReportWarningType::DataBeforeAccountTimezoneChange
                }
                "DATA_DELAYED" => ReportWarningType::DataDelayed,
                "OTHER" => ReportWarningType::Other,
                "REPORT_CURRENCY_NOT_ACCOUNT_CURRENCY" => {
                    ReportWarningType::ReportCurrencyNotAccountCurrency
                }
                "TYPE_UNSPECIFIED" => ReportWarningType::TypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ReportWarningType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ReportWarningType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReportWarningType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DATA_BEFORE_ACCOUNT_TIMEZONE_CHANGE" => {
                    ReportWarningType::DataBeforeAccountTimezoneChange
                }
                "DATA_DELAYED" => ReportWarningType::DataDelayed,
                "OTHER" => ReportWarningType::Other,
                "REPORT_CURRENCY_NOT_ACCOUNT_CURRENCY" => {
                    ReportWarningType::ReportCurrencyNotAccountCurrency
                }
                "TYPE_UNSPECIFIED" => ReportWarningType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ReportWarningType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportWarningType {
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
    pub struct StringList {
        #[doc = "The string values."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for StringList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StringList {
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
    #[doc = "Actions that can be performed on the accounts resource"]
    pub fn accounts(&self) -> crate::resources::accounts::AccountsActions {
        crate::resources::accounts::AccountsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod accounts {
        pub mod params {}
        pub struct AccountsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> AccountsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Gets information about the specified AdMob publisher account."]
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
            #[doc = "Lists the AdMob publisher account that was most recently signed in to from the AdMob UI. For more information, see https://support.google.com/admob/answer/10243672."]
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
                    page_size: None,
                    page_token: None,
                }
            }
            #[doc = "Actions that can be performed on the ad_units resource"]
            pub fn ad_units(&self) -> crate::resources::accounts::ad_units::AdUnitsActions {
                crate::resources::accounts::ad_units::AdUnitsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the apps resource"]
            pub fn apps(&self) -> crate::resources::accounts::apps::AppsActions {
                crate::resources::accounts::apps::AppsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the mediation_report resource"]
            pub fn mediation_report(
                &self,
            ) -> crate::resources::accounts::mediation_report::MediationReportActions {
                crate::resources::accounts::mediation_report::MediationReportActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the network_report resource"]
            pub fn network_report(
                &self,
            ) -> crate::resources::accounts::network_report::NetworkReportActions {
                crate::resources::accounts::network_report::NetworkReportActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [AccountsActions::get()](struct.AccountsActions.html#method.get)"]
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
            ) -> Result<crate::schemas::PublisherAccount, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::PublisherAccount, crate::Error> {
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
                let mut output = "https://admob.googleapis.com/".to_owned();
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
        #[doc = "Created via [AccountsActions::list()](struct.AccountsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
            #[doc = "Maximum number of accounts to return."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "The value returned by the last `ListPublisherAccountsResponse`; indicates that this is a continuation of a prior `ListPublisherAccounts` call, and that the system should return the next page of data."]
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
            ) -> Result<crate::schemas::ListPublisherAccountsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListPublisherAccountsResponse, crate::Error> {
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
                let mut output = "https://admob.googleapis.com/".to_owned();
                output.push_str("v1/accounts");
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
        pub mod ad_units {
            pub mod params {}
            pub struct AdUnitsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> AdUnitsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "List the ad units under the specified AdMob account."]
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
            #[doc = "Created via [AdUnitsActions::list()](struct.AdUnitsActions.html#method.list)"]
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
                #[doc = "The maximum number of ad units to return. If unspecified or 0, at most 1000 ad units will be returned. The maximum value is 10,000; values above 10,000 will be coerced to 10,000."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "The value returned by the last `ListAdUnitsResponse`; indicates that this is a continuation of a prior `ListAdUnits` call, and that the system should return the next page of data."]
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
                ) -> Result<crate::schemas::ListAdUnitsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListAdUnitsResponse, crate::Error> {
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
                    let mut output = "https://admob.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/adUnits");
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
        }
        pub mod apps {
            pub mod params {}
            pub struct AppsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> AppsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "List the apps under the specified AdMob account."]
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
            #[doc = "Created via [AppsActions::list()](struct.AppsActions.html#method.list)"]
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
                #[doc = "The maximum number of apps to return. If unspecified or 0, at most 1000 apps will be returned. The maximum value is 10,000; values above 10,000 will be coerced to 10,000."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "The value returned by the last `ListAppsResponse`; indicates that this is a continuation of a prior `ListApps` call, and that the system should return the next page of data."]
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
                ) -> Result<crate::schemas::ListAppsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListAppsResponse, crate::Error> {
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
                    let mut output = "https://admob.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/apps");
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
        }
        pub mod mediation_report {
            pub mod params {}
            pub struct MediationReportActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> MediationReportActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Generates an AdMob Mediation report based on the provided report specification. Returns result of a server-side streaming RPC. The result is returned in a sequence of responses."]
                pub fn generate(
                    &self,
                    request: crate::schemas::GenerateMediationReportRequest,
                    parent: impl Into<String>,
                ) -> GenerateRequestBuilder {
                    GenerateRequestBuilder {
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
            #[doc = "Created via [MediationReportActions::generate()](struct.MediationReportActions.html#method.generate)"]
            #[derive(Debug, Clone)]
            pub struct GenerateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GenerateMediationReportRequest,
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
            impl<'a> GenerateRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::GenerateMediationReportResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GenerateMediationReportResponse, crate::Error>
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
                    let mut output = "https://admob.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/mediationReport:generate");
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
        pub mod network_report {
            pub mod params {}
            pub struct NetworkReportActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> NetworkReportActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Generates an AdMob Network report based on the provided report specification. Returns result of a server-side streaming RPC. The result is returned in a sequence of responses."]
                pub fn generate(
                    &self,
                    request: crate::schemas::GenerateNetworkReportRequest,
                    parent: impl Into<String>,
                ) -> GenerateRequestBuilder {
                    GenerateRequestBuilder {
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
            #[doc = "Created via [NetworkReportActions::generate()](struct.NetworkReportActions.html#method.generate)"]
            #[derive(Debug, Clone)]
            pub struct GenerateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GenerateNetworkReportRequest,
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
            impl<'a> GenerateRequestBuilder<'a> {
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
                ) -> Result<crate::schemas::GenerateNetworkReportResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::GenerateNetworkReportResponse, crate::Error>
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
                    let mut output = "https://admob.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/networkReport:generate");
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
