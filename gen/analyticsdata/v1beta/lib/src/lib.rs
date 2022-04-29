#![doc = "# Resources and Methods\n* [properties](resources/properties/struct.PropertiesActions.html)\n  * [*batchRunPivotReports*](resources/properties/struct.BatchRunPivotReportsRequestBuilder.html), [*batchRunReports*](resources/properties/struct.BatchRunReportsRequestBuilder.html), [*checkCompatibility*](resources/properties/struct.CheckCompatibilityRequestBuilder.html), [*getMetadata*](resources/properties/struct.GetMetadataRequestBuilder.html), [*runPivotReport*](resources/properties/struct.RunPivotReportRequestBuilder.html), [*runRealtimeReport*](resources/properties/struct.RunRealtimeReportRequestBuilder.html), [*runReport*](resources/properties/struct.RunReportRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View and manage your Google Analytics data\n\n`https://www.googleapis.com/auth/analytics`"]
    pub const ANALYTICS: &str = "https://www.googleapis.com/auth/analytics";
    #[doc = "See and download your Google Analytics data\n\n`https://www.googleapis.com/auth/analytics.readonly`"]
    pub const ANALYTICS_READONLY: &str = "https://www.googleapis.com/auth/analytics.readonly";
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
    pub struct ActiveMetricRestriction {
        #[doc = "The name of the restricted metric."]
        #[serde(
            rename = "metricName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric_name: ::std::option::Option<String>,
        #[doc = "The reason for this metric's restriction."]
        #[serde(
            rename = "restrictedMetricTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub restricted_metric_types: ::std::option::Option<
            Vec<crate::schemas::ActiveMetricRestrictionRestrictedMetricTypesItems>,
        >,
    }
    impl ::google_field_selector::FieldSelector for ActiveMetricRestriction {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ActiveMetricRestriction {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ActiveMetricRestrictionRestrictedMetricTypesItems {
        #[doc = "Cost metrics such as `adCost`."]
        CostData,
        #[doc = "Unspecified type."]
        RestrictedMetricTypeUnspecified,
        #[doc = "Revenue metrics such as `purchaseRevenue`."]
        RevenueData,
    }
    impl ActiveMetricRestrictionRestrictedMetricTypesItems {
        pub fn as_str(self) -> &'static str {
            match self { ActiveMetricRestrictionRestrictedMetricTypesItems :: CostData => "COST_DATA" , ActiveMetricRestrictionRestrictedMetricTypesItems :: RestrictedMetricTypeUnspecified => "RESTRICTED_METRIC_TYPE_UNSPECIFIED" , ActiveMetricRestrictionRestrictedMetricTypesItems :: RevenueData => "REVENUE_DATA" , }
        }
    }
    impl ::std::convert::AsRef<str> for ActiveMetricRestrictionRestrictedMetricTypesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ActiveMetricRestrictionRestrictedMetricTypesItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<ActiveMetricRestrictionRestrictedMetricTypesItems, ()> {
            Ok (match s { "COST_DATA" => ActiveMetricRestrictionRestrictedMetricTypesItems :: CostData , "RESTRICTED_METRIC_TYPE_UNSPECIFIED" => ActiveMetricRestrictionRestrictedMetricTypesItems :: RestrictedMetricTypeUnspecified , "REVENUE_DATA" => ActiveMetricRestrictionRestrictedMetricTypesItems :: RevenueData , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for ActiveMetricRestrictionRestrictedMetricTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ActiveMetricRestrictionRestrictedMetricTypesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ActiveMetricRestrictionRestrictedMetricTypesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "COST_DATA" => ActiveMetricRestrictionRestrictedMetricTypesItems :: CostData , "RESTRICTED_METRIC_TYPE_UNSPECIFIED" => ActiveMetricRestrictionRestrictedMetricTypesItems :: RestrictedMetricTypeUnspecified , "REVENUE_DATA" => ActiveMetricRestrictionRestrictedMetricTypesItems :: RevenueData , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector for ActiveMetricRestrictionRestrictedMetricTypesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ActiveMetricRestrictionRestrictedMetricTypesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BatchRunPivotReportsRequest {
        #[doc = "Individual requests. Each request has a separate pivot report response. Each batch request is allowed up to 5 requests."]
        #[serde(
            rename = "requests",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requests: ::std::option::Option<Vec<crate::schemas::RunPivotReportRequest>>,
    }
    impl ::google_field_selector::FieldSelector for BatchRunPivotReportsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchRunPivotReportsRequest {
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
    pub struct BatchRunPivotReportsResponse {
        #[doc = "Identifies what kind of resource this message is. This `kind` is always the fixed string \"analyticsData#batchRunPivotReports\". Useful to distinguish between response types in JSON."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Individual responses. Each response has a separate pivot report request."]
        #[serde(
            rename = "pivotReports",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pivot_reports: ::std::option::Option<Vec<crate::schemas::RunPivotReportResponse>>,
    }
    impl ::google_field_selector::FieldSelector for BatchRunPivotReportsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchRunPivotReportsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BatchRunReportsRequest {
        #[doc = "Individual requests. Each request has a separate report response. Each batch request is allowed up to 5 requests."]
        #[serde(
            rename = "requests",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requests: ::std::option::Option<Vec<crate::schemas::RunReportRequest>>,
    }
    impl ::google_field_selector::FieldSelector for BatchRunReportsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchRunReportsRequest {
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
    pub struct BatchRunReportsResponse {
        #[doc = "Identifies what kind of resource this message is. This `kind` is always the fixed string \"analyticsData#batchRunReports\". Useful to distinguish between response types in JSON."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Individual responses. Each response has a separate report request."]
        #[serde(
            rename = "reports",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reports: ::std::option::Option<Vec<crate::schemas::RunReportResponse>>,
    }
    impl ::google_field_selector::FieldSelector for BatchRunReportsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchRunReportsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct BetweenFilter {
        #[doc = "Begins with this number."]
        #[serde(
            rename = "fromValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub from_value: ::std::option::Option<crate::schemas::NumericValue>,
        #[doc = "Ends with this number."]
        #[serde(
            rename = "toValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub to_value: ::std::option::Option<crate::schemas::NumericValue>,
    }
    impl ::google_field_selector::FieldSelector for BetweenFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BetweenFilter {
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
    pub struct CaseExpression {
        #[doc = "Name of a dimension. The name must refer back to a name in dimensions field of the request."]
        #[serde(
            rename = "dimensionName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CaseExpression {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CaseExpression {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CheckCompatibilityRequest {
        #[doc = "Filters the dimensions and metrics in the response to just this compatibility. Commonly used as `”compatibilityFilter”: “COMPATIBLE”` to only return compatible dimensions & metrics."]
        #[serde(
            rename = "compatibilityFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compatibility_filter:
            ::std::option::Option<crate::schemas::CheckCompatibilityRequestCompatibilityFilter>,
        #[doc = "The filter clause of dimensions. `dimensionFilter` should be the same value as in your `runReport` request."]
        #[serde(
            rename = "dimensionFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_filter: ::std::option::Option<crate::schemas::FilterExpression>,
        #[doc = "The dimensions in this report. `dimensions` should be the same value as in your `runReport` request."]
        #[serde(
            rename = "dimensions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimensions: ::std::option::Option<Vec<crate::schemas::Dimension>>,
        #[doc = "The filter clause of metrics. `metricFilter` should be the same value as in your `runReport` request"]
        #[serde(
            rename = "metricFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric_filter: ::std::option::Option<crate::schemas::FilterExpression>,
        #[doc = "The metrics in this report. `metrics` should be the same value as in your `runReport` request."]
        #[serde(
            rename = "metrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metrics: ::std::option::Option<Vec<crate::schemas::Metric>>,
    }
    impl ::google_field_selector::FieldSelector for CheckCompatibilityRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CheckCompatibilityRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CheckCompatibilityRequestCompatibilityFilter {
        #[doc = "Unspecified compatibility."]
        CompatibilityUnspecified,
        #[doc = "The dimension or metric is compatible. This dimension or metric can be successfully added to a report."]
        Compatible,
        #[doc = "The dimension or metric is incompatible. This dimension or metric cannot be successfully added to a report."]
        Incompatible,
    }
    impl CheckCompatibilityRequestCompatibilityFilter {
        pub fn as_str(self) -> &'static str {
            match self {
                CheckCompatibilityRequestCompatibilityFilter::CompatibilityUnspecified => {
                    "COMPATIBILITY_UNSPECIFIED"
                }
                CheckCompatibilityRequestCompatibilityFilter::Compatible => "COMPATIBLE",
                CheckCompatibilityRequestCompatibilityFilter::Incompatible => "INCOMPATIBLE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CheckCompatibilityRequestCompatibilityFilter {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CheckCompatibilityRequestCompatibilityFilter {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<CheckCompatibilityRequestCompatibilityFilter, ()> {
            Ok(match s {
                "COMPATIBILITY_UNSPECIFIED" => {
                    CheckCompatibilityRequestCompatibilityFilter::CompatibilityUnspecified
                }
                "COMPATIBLE" => CheckCompatibilityRequestCompatibilityFilter::Compatible,
                "INCOMPATIBLE" => CheckCompatibilityRequestCompatibilityFilter::Incompatible,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CheckCompatibilityRequestCompatibilityFilter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CheckCompatibilityRequestCompatibilityFilter {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CheckCompatibilityRequestCompatibilityFilter {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMPATIBILITY_UNSPECIFIED" => {
                    CheckCompatibilityRequestCompatibilityFilter::CompatibilityUnspecified
                }
                "COMPATIBLE" => CheckCompatibilityRequestCompatibilityFilter::Compatible,
                "INCOMPATIBLE" => CheckCompatibilityRequestCompatibilityFilter::Incompatible,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CheckCompatibilityRequestCompatibilityFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CheckCompatibilityRequestCompatibilityFilter {
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
    pub struct CheckCompatibilityResponse {
        #[doc = "The compatibility of each dimension."]
        #[serde(
            rename = "dimensionCompatibilities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_compatibilities:
            ::std::option::Option<Vec<crate::schemas::DimensionCompatibility>>,
        #[doc = "The compatibility of each metric."]
        #[serde(
            rename = "metricCompatibilities",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric_compatibilities: ::std::option::Option<Vec<crate::schemas::MetricCompatibility>>,
    }
    impl ::google_field_selector::FieldSelector for CheckCompatibilityResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CheckCompatibilityResponse {
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
    pub struct Cohort {
        #[doc = "The cohort selects users whose first touch date is between start date and end date defined in the `dateRange`. This `dateRange` does not specify the full date range of event data that is present in a cohort report. In a cohort report, this `dateRange` is extended by the granularity and offset present in the `cohortsRange`; event data for the extended reporting date range is present in a cohort report. In a cohort request, this `dateRange` is required and the `dateRanges` in the `RunReportRequest` or `RunPivotReportRequest` must be unspecified. This `dateRange` should generally be aligned with the cohort's granularity. If `CohortsRange` uses daily granularity, this `dateRange` can be a single day. If `CohortsRange` uses weekly granularity, this `dateRange` can be aligned to a week boundary, starting at Sunday and ending Saturday. If `CohortsRange` uses monthly granularity, this `dateRange` can be aligned to a month, starting at the first and ending on the last day of the month."]
        #[serde(
            rename = "dateRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_range: ::std::option::Option<crate::schemas::DateRange>,
        #[doc = "Dimension used by the cohort. Required and only supports `firstSessionDate`."]
        #[serde(
            rename = "dimension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension: ::std::option::Option<String>,
        #[doc = "Assigns a name to this cohort. The dimension `cohort` is valued to this name in a report response. If set, cannot begin with `cohort_` or `RESERVED_`. If not set, cohorts are named by their zero based index `cohort_0`, `cohort_1`, etc."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Cohort {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Cohort {
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
    pub struct CohortReportSettings {
        #[doc = "If true, accumulates the result from first touch day to the end day. Not supported in `RunReportRequest`."]
        #[serde(
            rename = "accumulate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub accumulate: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for CohortReportSettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CohortReportSettings {
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
    pub struct CohortSpec {
        #[doc = "Optional settings for a cohort report."]
        #[serde(
            rename = "cohortReportSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cohort_report_settings: ::std::option::Option<crate::schemas::CohortReportSettings>,
        #[doc = "Defines the selection criteria to group users into cohorts. Most cohort reports define only a single cohort. If multiple cohorts are specified, each cohort can be recognized in the report by their name."]
        #[serde(
            rename = "cohorts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cohorts: ::std::option::Option<Vec<crate::schemas::Cohort>>,
        #[doc = "Cohort reports follow cohorts over an extended reporting date range. This range specifies an offset duration to follow the cohorts over."]
        #[serde(
            rename = "cohortsRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cohorts_range: ::std::option::Option<crate::schemas::CohortsRange>,
    }
    impl ::google_field_selector::FieldSelector for CohortSpec {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CohortSpec {
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
    pub struct CohortsRange {
        #[doc = "Required. `endOffset` specifies the end date of the extended reporting date range for a cohort report. `endOffset` can be any positive integer but is commonly set to 5 to 10 so that reports contain data on the cohort for the next several granularity time periods. If `granularity` is `DAILY`, the `endDate` of the extended reporting date range is `endDate` of the cohort plus `endOffset` days. If `granularity` is `WEEKLY`, the `endDate` of the extended reporting date range is `endDate` of the cohort plus `endOffset * 7` days. If `granularity` is `MONTHLY`, the `endDate` of the extended reporting date range is `endDate` of the cohort plus `endOffset * 30` days."]
        #[serde(
            rename = "endOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_offset: ::std::option::Option<i32>,
        #[doc = "Required. The granularity used to interpret the `startOffset` and `endOffset` for the extended reporting date range for a cohort report."]
        #[serde(
            rename = "granularity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub granularity: ::std::option::Option<crate::schemas::CohortsRangeGranularity>,
        #[doc = "`startOffset` specifies the start date of the extended reporting date range for a cohort report. `startOffset` is commonly set to 0 so that reports contain data from the acquisition of the cohort forward. If `granularity` is `DAILY`, the `startDate` of the extended reporting date range is `startDate` of the cohort plus `startOffset` days. If `granularity` is `WEEKLY`, the `startDate` of the extended reporting date range is `startDate` of the cohort plus `startOffset * 7` days. If `granularity` is `MONTHLY`, the `startDate` of the extended reporting date range is `startDate` of the cohort plus `startOffset * 30` days."]
        #[serde(
            rename = "startOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_offset: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for CohortsRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CohortsRange {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CohortsRangeGranularity {
        #[doc = "Daily granularity. Commonly used if the cohort's `dateRange` is a single day and the request contains `cohortNthDay`."]
        Daily,
        #[doc = "Should never be specified."]
        GranularityUnspecified,
        #[doc = "Monthly granularity. Commonly used if the cohort's `dateRange` is a month in duration and the request contains `cohortNthMonth`."]
        Monthly,
        #[doc = "Weekly granularity. Commonly used if the cohort's `dateRange` is a week in duration (starting on Sunday and ending on Saturday) and the request contains `cohortNthWeek`."]
        Weekly,
    }
    impl CohortsRangeGranularity {
        pub fn as_str(self) -> &'static str {
            match self {
                CohortsRangeGranularity::Daily => "DAILY",
                CohortsRangeGranularity::GranularityUnspecified => "GRANULARITY_UNSPECIFIED",
                CohortsRangeGranularity::Monthly => "MONTHLY",
                CohortsRangeGranularity::Weekly => "WEEKLY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CohortsRangeGranularity {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CohortsRangeGranularity {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CohortsRangeGranularity, ()> {
            Ok(match s {
                "DAILY" => CohortsRangeGranularity::Daily,
                "GRANULARITY_UNSPECIFIED" => CohortsRangeGranularity::GranularityUnspecified,
                "MONTHLY" => CohortsRangeGranularity::Monthly,
                "WEEKLY" => CohortsRangeGranularity::Weekly,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CohortsRangeGranularity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CohortsRangeGranularity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CohortsRangeGranularity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DAILY" => CohortsRangeGranularity::Daily,
                "GRANULARITY_UNSPECIFIED" => CohortsRangeGranularity::GranularityUnspecified,
                "MONTHLY" => CohortsRangeGranularity::Monthly,
                "WEEKLY" => CohortsRangeGranularity::Weekly,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CohortsRangeGranularity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CohortsRangeGranularity {
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
    pub struct ConcatenateExpression {
        #[doc = "The delimiter placed between dimension names. Delimiters are often single characters such as \"|\" or \",\" but can be longer strings. If a dimension value contains the delimiter, both will be present in response with no distinction. For example if dimension 1 value = \"US,FR\", dimension 2 value = \"JP\", and delimiter = \",\", then the response will contain \"US,FR,JP\"."]
        #[serde(
            rename = "delimiter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub delimiter: ::std::option::Option<String>,
        #[doc = "Names of dimensions. The names must refer back to names in the dimensions field of the request."]
        #[serde(
            rename = "dimensionNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_names: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for ConcatenateExpression {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ConcatenateExpression {
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
        #[doc = "The inclusive end date for the query in the format `YYYY-MM-DD`. Cannot be before `start_date`. The format `NdaysAgo`, `yesterday`, or `today` is also accepted, and in that case, the date is inferred based on the property's reporting time zone."]
        #[serde(
            rename = "endDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_date: ::std::option::Option<String>,
        #[doc = "Assigns a name to this date range. The dimension `dateRange` is valued to this name in a report response. If set, cannot begin with `date_range_` or `RESERVED_`. If not set, date ranges are named by their zero based index in the request: `date_range_0`, `date_range_1`, etc."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The inclusive start date for the query in the format `YYYY-MM-DD`. Cannot be after `end_date`. The format `NdaysAgo`, `yesterday`, or `today` is also accepted, and in that case, the date is inferred based on the property's reporting time zone."]
        #[serde(
            rename = "startDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_date: ::std::option::Option<String>,
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
    pub struct Dimension {
        #[doc = "One dimension can be the result of an expression of multiple dimensions. For example, dimension \"country, city\": concatenate(country, \", \", city)."]
        #[serde(
            rename = "dimensionExpression",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_expression: ::std::option::Option<crate::schemas::DimensionExpression>,
        #[doc = "The name of the dimension. See the [API Dimensions](https://developers.google.com/analytics/devguides/reporting/data/v1/api-schema#dimensions) for the list of dimension names. If `dimensionExpression` is specified, `name` can be any string that you would like within the allowed character set. For example if a `dimensionExpression` concatenates `country` and `city`, you could call that dimension `countryAndCity`. Dimension names that you choose must match the regular expression `^[a-zA-Z0-9_]$`. Dimensions are referenced by `name` in `dimensionFilter`, `orderBys`, `dimensionExpression`, and `pivots`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Dimension {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Dimension {
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
    pub struct DimensionCompatibility {
        #[doc = "The compatibility of this dimension. If the compatibility is COMPATIBLE, this dimension can be successfully added to the report."]
        #[serde(
            rename = "compatibility",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compatibility:
            ::std::option::Option<crate::schemas::DimensionCompatibilityCompatibility>,
        #[doc = "The dimension metadata contains the API name for this compatibility information. The dimension metadata also contains other helpful information like the UI name and description."]
        #[serde(
            rename = "dimensionMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_metadata: ::std::option::Option<crate::schemas::DimensionMetadata>,
    }
    impl ::google_field_selector::FieldSelector for DimensionCompatibility {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DimensionCompatibility {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DimensionCompatibilityCompatibility {
        #[doc = "Unspecified compatibility."]
        CompatibilityUnspecified,
        #[doc = "The dimension or metric is compatible. This dimension or metric can be successfully added to a report."]
        Compatible,
        #[doc = "The dimension or metric is incompatible. This dimension or metric cannot be successfully added to a report."]
        Incompatible,
    }
    impl DimensionCompatibilityCompatibility {
        pub fn as_str(self) -> &'static str {
            match self {
                DimensionCompatibilityCompatibility::CompatibilityUnspecified => {
                    "COMPATIBILITY_UNSPECIFIED"
                }
                DimensionCompatibilityCompatibility::Compatible => "COMPATIBLE",
                DimensionCompatibilityCompatibility::Incompatible => "INCOMPATIBLE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DimensionCompatibilityCompatibility {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DimensionCompatibilityCompatibility {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DimensionCompatibilityCompatibility, ()> {
            Ok(match s {
                "COMPATIBILITY_UNSPECIFIED" => {
                    DimensionCompatibilityCompatibility::CompatibilityUnspecified
                }
                "COMPATIBLE" => DimensionCompatibilityCompatibility::Compatible,
                "INCOMPATIBLE" => DimensionCompatibilityCompatibility::Incompatible,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DimensionCompatibilityCompatibility {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DimensionCompatibilityCompatibility {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DimensionCompatibilityCompatibility {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMPATIBILITY_UNSPECIFIED" => {
                    DimensionCompatibilityCompatibility::CompatibilityUnspecified
                }
                "COMPATIBLE" => DimensionCompatibilityCompatibility::Compatible,
                "INCOMPATIBLE" => DimensionCompatibilityCompatibility::Incompatible,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DimensionCompatibilityCompatibility {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DimensionCompatibilityCompatibility {
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
    pub struct DimensionExpression {
        #[doc = "Used to combine dimension values to a single dimension. For example, dimension \"country, city\": concatenate(country, \", \", city)."]
        #[serde(
            rename = "concatenate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub concatenate: ::std::option::Option<crate::schemas::ConcatenateExpression>,
        #[doc = "Used to convert a dimension value to lower case."]
        #[serde(
            rename = "lowerCase",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lower_case: ::std::option::Option<crate::schemas::CaseExpression>,
        #[doc = "Used to convert a dimension value to upper case."]
        #[serde(
            rename = "upperCase",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub upper_case: ::std::option::Option<crate::schemas::CaseExpression>,
    }
    impl ::google_field_selector::FieldSelector for DimensionExpression {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DimensionExpression {
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
    pub struct DimensionHeader {
        #[doc = "The dimension's name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DimensionHeader {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DimensionHeader {
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
    pub struct DimensionMetadata {
        #[doc = "This dimension's name. Useable in [Dimension](#Dimension)'s `name`. For example, `eventName`."]
        #[serde(
            rename = "apiName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub api_name: ::std::option::Option<String>,
        #[doc = "The display name of the category that this dimension belongs to. Similar dimensions and metrics are categorized together."]
        #[serde(
            rename = "category",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category: ::std::option::Option<String>,
        #[doc = "True if the dimension is a custom dimension for this property."]
        #[serde(
            rename = "customDefinition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_definition: ::std::option::Option<bool>,
        #[doc = "Still usable but deprecated names for this dimension. If populated, this dimension is available by either `apiName` or one of `deprecatedApiNames` for a period of time. After the deprecation period, the dimension will be available only by `apiName`."]
        #[serde(
            rename = "deprecatedApiNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deprecated_api_names: ::std::option::Option<Vec<String>>,
        #[doc = "Description of how this dimension is used and calculated."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "This dimension's name within the Google Analytics user interface. For example, `Event name`."]
        #[serde(
            rename = "uiName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ui_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DimensionMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DimensionMetadata {
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
    pub struct DimensionOrderBy {
        #[doc = "A dimension name in the request to order by."]
        #[serde(
            rename = "dimensionName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_name: ::std::option::Option<String>,
        #[doc = "Controls the rule for dimension value ordering."]
        #[serde(
            rename = "orderType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub order_type: ::std::option::Option<crate::schemas::DimensionOrderByOrderType>,
    }
    impl ::google_field_selector::FieldSelector for DimensionOrderBy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DimensionOrderBy {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DimensionOrderByOrderType {
        #[doc = "Alphanumeric sort by Unicode code point. For example, \"2\" < \"A\" < \"X\" < \"b\" < \"z\"."]
        Alphanumeric,
        #[doc = "Case insensitive alphanumeric sort by lower case Unicode code point. For example, \"2\" < \"A\" < \"b\" < \"X\" < \"z\"."]
        CaseInsensitiveAlphanumeric,
        #[doc = "Dimension values are converted to numbers before sorting. For example in NUMERIC sort, \"25\" < \"100\", and in `ALPHANUMERIC` sort, \"100\" < \"25\". Non-numeric dimension values all have equal ordering value below all numeric values."]
        Numeric,
        #[doc = "Unspecified."]
        OrderTypeUnspecified,
    }
    impl DimensionOrderByOrderType {
        pub fn as_str(self) -> &'static str {
            match self {
                DimensionOrderByOrderType::Alphanumeric => "ALPHANUMERIC",
                DimensionOrderByOrderType::CaseInsensitiveAlphanumeric => {
                    "CASE_INSENSITIVE_ALPHANUMERIC"
                }
                DimensionOrderByOrderType::Numeric => "NUMERIC",
                DimensionOrderByOrderType::OrderTypeUnspecified => "ORDER_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DimensionOrderByOrderType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DimensionOrderByOrderType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DimensionOrderByOrderType, ()> {
            Ok(match s {
                "ALPHANUMERIC" => DimensionOrderByOrderType::Alphanumeric,
                "CASE_INSENSITIVE_ALPHANUMERIC" => {
                    DimensionOrderByOrderType::CaseInsensitiveAlphanumeric
                }
                "NUMERIC" => DimensionOrderByOrderType::Numeric,
                "ORDER_TYPE_UNSPECIFIED" => DimensionOrderByOrderType::OrderTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DimensionOrderByOrderType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DimensionOrderByOrderType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DimensionOrderByOrderType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALPHANUMERIC" => DimensionOrderByOrderType::Alphanumeric,
                "CASE_INSENSITIVE_ALPHANUMERIC" => {
                    DimensionOrderByOrderType::CaseInsensitiveAlphanumeric
                }
                "NUMERIC" => DimensionOrderByOrderType::Numeric,
                "ORDER_TYPE_UNSPECIFIED" => DimensionOrderByOrderType::OrderTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DimensionOrderByOrderType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DimensionOrderByOrderType {
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
    pub struct DimensionValue {
        #[doc = "Value as a string if the dimension type is a string."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DimensionValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DimensionValue {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Filter {
        #[doc = "A filter for two values."]
        #[serde(
            rename = "betweenFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub between_filter: ::std::option::Option<crate::schemas::BetweenFilter>,
        #[doc = "The dimension name or metric name. Must be a name defined in dimensions or metrics."]
        #[serde(
            rename = "fieldName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field_name: ::std::option::Option<String>,
        #[doc = "A filter for in list values."]
        #[serde(
            rename = "inListFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub in_list_filter: ::std::option::Option<crate::schemas::InListFilter>,
        #[doc = "A filter for numeric or date values."]
        #[serde(
            rename = "numericFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub numeric_filter: ::std::option::Option<crate::schemas::NumericFilter>,
        #[doc = "Strings related filter."]
        #[serde(
            rename = "stringFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub string_filter: ::std::option::Option<crate::schemas::StringFilter>,
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
    pub struct FilterExpression {
        #[doc = "The FilterExpressions in and_group have an AND relationship."]
        #[serde(
            rename = "andGroup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub and_group: ::std::option::Option<crate::schemas::FilterExpressionList>,
        #[doc = "A primitive filter. In the same FilterExpression, all of the filter's field names need to be either all dimensions or all metrics."]
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter: ::std::option::Option<crate::schemas::Filter>,
        #[doc = "The FilterExpression is NOT of not_expression."]
        #[serde(
            rename = "notExpression",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub not_expression: ::std::option::Option<Box<crate::schemas::FilterExpression>>,
        #[doc = "The FilterExpressions in or_group have an OR relationship."]
        #[serde(
            rename = "orGroup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub or_group: ::std::option::Option<crate::schemas::FilterExpressionList>,
    }
    impl ::google_field_selector::FieldSelector for FilterExpression {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FilterExpression {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct FilterExpressionList {
        #[doc = "A list of filter expressions."]
        #[serde(
            rename = "expressions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expressions: ::std::option::Option<Vec<crate::schemas::FilterExpression>>,
    }
    impl ::google_field_selector::FieldSelector for FilterExpressionList {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FilterExpressionList {
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
    pub struct InListFilter {
        #[doc = "If true, the string value is case sensitive."]
        #[serde(
            rename = "caseSensitive",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub case_sensitive: ::std::option::Option<bool>,
        #[doc = "The list of string values. Must be non-empty."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for InListFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InListFilter {
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
    pub struct Metadata {
        #[doc = "The dimension descriptions."]
        #[serde(
            rename = "dimensions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimensions: ::std::option::Option<Vec<crate::schemas::DimensionMetadata>>,
        #[doc = "The metric descriptions."]
        #[serde(
            rename = "metrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metrics: ::std::option::Option<Vec<crate::schemas::MetricMetadata>>,
        #[doc = "Resource name of this metadata."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
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
    pub struct Metric {
        #[doc = "A mathematical expression for derived metrics. For example, the metric Event count per user is `eventCount/totalUsers`."]
        #[serde(
            rename = "expression",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expression: ::std::option::Option<String>,
        #[doc = "Indicates if a metric is invisible in the report response. If a metric is invisible, the metric will not produce a column in the response, but can be used in `metricFilter`, `orderBys`, or a metric `expression`."]
        #[serde(
            rename = "invisible",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub invisible: ::std::option::Option<bool>,
        #[doc = "The name of the metric. See the [API Metrics](https://developers.google.com/analytics/devguides/reporting/data/v1/api-schema#metrics) for the list of metric names. If `expression` is specified, `name` can be any string that you would like within the allowed character set. For example if `expression` is `screenPageViews/sessions`, you could call that metric's name = `viewsPerSession`. Metric names that you choose must match the regular expression `^[a-zA-Z0-9_]$`. Metrics are referenced by `name` in `metricFilter`, `orderBys`, and metric `expression`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Metric {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Metric {
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
    pub struct MetricCompatibility {
        #[doc = "The compatibility of this metric. If the compatibility is COMPATIBLE, this metric can be successfully added to the report."]
        #[serde(
            rename = "compatibility",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compatibility: ::std::option::Option<crate::schemas::MetricCompatibilityCompatibility>,
        #[doc = "The metric metadata contains the API name for this compatibility information. The metric metadata also contains other helpful information like the UI name and description."]
        #[serde(
            rename = "metricMetadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric_metadata: ::std::option::Option<crate::schemas::MetricMetadata>,
    }
    impl ::google_field_selector::FieldSelector for MetricCompatibility {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetricCompatibility {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MetricCompatibilityCompatibility {
        #[doc = "Unspecified compatibility."]
        CompatibilityUnspecified,
        #[doc = "The dimension or metric is compatible. This dimension or metric can be successfully added to a report."]
        Compatible,
        #[doc = "The dimension or metric is incompatible. This dimension or metric cannot be successfully added to a report."]
        Incompatible,
    }
    impl MetricCompatibilityCompatibility {
        pub fn as_str(self) -> &'static str {
            match self {
                MetricCompatibilityCompatibility::CompatibilityUnspecified => {
                    "COMPATIBILITY_UNSPECIFIED"
                }
                MetricCompatibilityCompatibility::Compatible => "COMPATIBLE",
                MetricCompatibilityCompatibility::Incompatible => "INCOMPATIBLE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MetricCompatibilityCompatibility {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MetricCompatibilityCompatibility {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MetricCompatibilityCompatibility, ()> {
            Ok(match s {
                "COMPATIBILITY_UNSPECIFIED" => {
                    MetricCompatibilityCompatibility::CompatibilityUnspecified
                }
                "COMPATIBLE" => MetricCompatibilityCompatibility::Compatible,
                "INCOMPATIBLE" => MetricCompatibilityCompatibility::Incompatible,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MetricCompatibilityCompatibility {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MetricCompatibilityCompatibility {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MetricCompatibilityCompatibility {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMPATIBILITY_UNSPECIFIED" => {
                    MetricCompatibilityCompatibility::CompatibilityUnspecified
                }
                "COMPATIBLE" => MetricCompatibilityCompatibility::Compatible,
                "INCOMPATIBLE" => MetricCompatibilityCompatibility::Incompatible,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MetricCompatibilityCompatibility {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetricCompatibilityCompatibility {
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
    pub struct MetricHeader {
        #[doc = "The metric's name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The metric's data type."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::MetricHeaderType>,
    }
    impl ::google_field_selector::FieldSelector for MetricHeader {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetricHeader {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MetricHeaderType {
        #[doc = "Unspecified type."]
        MetricTypeUnspecified,
        #[doc = "An amount of money; a special floating point type."]
        TypeCurrency,
        #[doc = "A length in feet; a special floating point type."]
        TypeFeet,
        #[doc = "Floating point type."]
        TypeFloat,
        #[doc = "A duration in hours; a special floating point type."]
        TypeHours,
        #[doc = "Integer type."]
        TypeInteger,
        #[doc = "A length in kilometers; a special floating point type."]
        TypeKilometers,
        #[doc = "A length in meters; a special floating point type."]
        TypeMeters,
        #[doc = "A length in miles; a special floating point type."]
        TypeMiles,
        #[doc = "A duration in milliseconds; a special floating point type."]
        TypeMilliseconds,
        #[doc = "A duration in minutes; a special floating point type."]
        TypeMinutes,
        #[doc = "A duration of seconds; a special floating point type."]
        TypeSeconds,
        #[doc = "A custom metric of standard type; a special floating point type."]
        TypeStandard,
    }
    impl MetricHeaderType {
        pub fn as_str(self) -> &'static str {
            match self {
                MetricHeaderType::MetricTypeUnspecified => "METRIC_TYPE_UNSPECIFIED",
                MetricHeaderType::TypeCurrency => "TYPE_CURRENCY",
                MetricHeaderType::TypeFeet => "TYPE_FEET",
                MetricHeaderType::TypeFloat => "TYPE_FLOAT",
                MetricHeaderType::TypeHours => "TYPE_HOURS",
                MetricHeaderType::TypeInteger => "TYPE_INTEGER",
                MetricHeaderType::TypeKilometers => "TYPE_KILOMETERS",
                MetricHeaderType::TypeMeters => "TYPE_METERS",
                MetricHeaderType::TypeMiles => "TYPE_MILES",
                MetricHeaderType::TypeMilliseconds => "TYPE_MILLISECONDS",
                MetricHeaderType::TypeMinutes => "TYPE_MINUTES",
                MetricHeaderType::TypeSeconds => "TYPE_SECONDS",
                MetricHeaderType::TypeStandard => "TYPE_STANDARD",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MetricHeaderType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MetricHeaderType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MetricHeaderType, ()> {
            Ok(match s {
                "METRIC_TYPE_UNSPECIFIED" => MetricHeaderType::MetricTypeUnspecified,
                "TYPE_CURRENCY" => MetricHeaderType::TypeCurrency,
                "TYPE_FEET" => MetricHeaderType::TypeFeet,
                "TYPE_FLOAT" => MetricHeaderType::TypeFloat,
                "TYPE_HOURS" => MetricHeaderType::TypeHours,
                "TYPE_INTEGER" => MetricHeaderType::TypeInteger,
                "TYPE_KILOMETERS" => MetricHeaderType::TypeKilometers,
                "TYPE_METERS" => MetricHeaderType::TypeMeters,
                "TYPE_MILES" => MetricHeaderType::TypeMiles,
                "TYPE_MILLISECONDS" => MetricHeaderType::TypeMilliseconds,
                "TYPE_MINUTES" => MetricHeaderType::TypeMinutes,
                "TYPE_SECONDS" => MetricHeaderType::TypeSeconds,
                "TYPE_STANDARD" => MetricHeaderType::TypeStandard,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MetricHeaderType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MetricHeaderType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MetricHeaderType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "METRIC_TYPE_UNSPECIFIED" => MetricHeaderType::MetricTypeUnspecified,
                "TYPE_CURRENCY" => MetricHeaderType::TypeCurrency,
                "TYPE_FEET" => MetricHeaderType::TypeFeet,
                "TYPE_FLOAT" => MetricHeaderType::TypeFloat,
                "TYPE_HOURS" => MetricHeaderType::TypeHours,
                "TYPE_INTEGER" => MetricHeaderType::TypeInteger,
                "TYPE_KILOMETERS" => MetricHeaderType::TypeKilometers,
                "TYPE_METERS" => MetricHeaderType::TypeMeters,
                "TYPE_MILES" => MetricHeaderType::TypeMiles,
                "TYPE_MILLISECONDS" => MetricHeaderType::TypeMilliseconds,
                "TYPE_MINUTES" => MetricHeaderType::TypeMinutes,
                "TYPE_SECONDS" => MetricHeaderType::TypeSeconds,
                "TYPE_STANDARD" => MetricHeaderType::TypeStandard,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MetricHeaderType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetricHeaderType {
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
    pub struct MetricMetadata {
        #[doc = "A metric name. Useable in [Metric](#Metric)'s `name`. For example, `eventCount`."]
        #[serde(
            rename = "apiName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub api_name: ::std::option::Option<String>,
        #[doc = "If reasons are specified, your access is blocked to this metric for this property. API requests from you to this property for this metric will succeed; however, the report will contain only zeros for this metric. API requests with metric filters on blocked metrics will fail. If reasons are empty, you have access to this metric. To learn more, see [Access and data-restriction management](https://support.google.com/analytics/answer/10851388)."]
        #[serde(
            rename = "blockedReasons",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub blocked_reasons:
            ::std::option::Option<Vec<crate::schemas::MetricMetadataBlockedReasonsItems>>,
        #[doc = "The display name of the category that this metrics belongs to. Similar dimensions and metrics are categorized together."]
        #[serde(
            rename = "category",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub category: ::std::option::Option<String>,
        #[doc = "True if the metric is a custom metric for this property."]
        #[serde(
            rename = "customDefinition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_definition: ::std::option::Option<bool>,
        #[doc = "Still usable but deprecated names for this metric. If populated, this metric is available by either `apiName` or one of `deprecatedApiNames` for a period of time. After the deprecation period, the metric will be available only by `apiName`."]
        #[serde(
            rename = "deprecatedApiNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deprecated_api_names: ::std::option::Option<Vec<String>>,
        #[doc = "Description of how this metric is used and calculated."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The mathematical expression for this derived metric. Can be used in [Metric](#Metric)'s `expression` field for equivalent reports. Most metrics are not expressions, and for non-expressions, this field is empty."]
        #[serde(
            rename = "expression",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub expression: ::std::option::Option<String>,
        #[doc = "The type of this metric."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::MetricMetadataType>,
        #[doc = "This metric's name within the Google Analytics user interface. For example, `Event count`."]
        #[serde(
            rename = "uiName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ui_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for MetricMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetricMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MetricMetadataBlockedReasonsItems {
        #[doc = "Will never be specified in API response."]
        BlockedReasonUnspecified,
        #[doc = "If present, your access is blocked to cost related metrics for this property, and this metric is cost related."]
        NoCostMetrics,
        #[doc = "If present, your access is blocked to revenue related metrics for this property, and this metric is revenue related."]
        NoRevenueMetrics,
    }
    impl MetricMetadataBlockedReasonsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                MetricMetadataBlockedReasonsItems::BlockedReasonUnspecified => {
                    "BLOCKED_REASON_UNSPECIFIED"
                }
                MetricMetadataBlockedReasonsItems::NoCostMetrics => "NO_COST_METRICS",
                MetricMetadataBlockedReasonsItems::NoRevenueMetrics => "NO_REVENUE_METRICS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MetricMetadataBlockedReasonsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MetricMetadataBlockedReasonsItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MetricMetadataBlockedReasonsItems, ()> {
            Ok(match s {
                "BLOCKED_REASON_UNSPECIFIED" => {
                    MetricMetadataBlockedReasonsItems::BlockedReasonUnspecified
                }
                "NO_COST_METRICS" => MetricMetadataBlockedReasonsItems::NoCostMetrics,
                "NO_REVENUE_METRICS" => MetricMetadataBlockedReasonsItems::NoRevenueMetrics,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MetricMetadataBlockedReasonsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MetricMetadataBlockedReasonsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MetricMetadataBlockedReasonsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BLOCKED_REASON_UNSPECIFIED" => {
                    MetricMetadataBlockedReasonsItems::BlockedReasonUnspecified
                }
                "NO_COST_METRICS" => MetricMetadataBlockedReasonsItems::NoCostMetrics,
                "NO_REVENUE_METRICS" => MetricMetadataBlockedReasonsItems::NoRevenueMetrics,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MetricMetadataBlockedReasonsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetricMetadataBlockedReasonsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MetricMetadataType {
        #[doc = "Unspecified type."]
        MetricTypeUnspecified,
        #[doc = "An amount of money; a special floating point type."]
        TypeCurrency,
        #[doc = "A length in feet; a special floating point type."]
        TypeFeet,
        #[doc = "Floating point type."]
        TypeFloat,
        #[doc = "A duration in hours; a special floating point type."]
        TypeHours,
        #[doc = "Integer type."]
        TypeInteger,
        #[doc = "A length in kilometers; a special floating point type."]
        TypeKilometers,
        #[doc = "A length in meters; a special floating point type."]
        TypeMeters,
        #[doc = "A length in miles; a special floating point type."]
        TypeMiles,
        #[doc = "A duration in milliseconds; a special floating point type."]
        TypeMilliseconds,
        #[doc = "A duration in minutes; a special floating point type."]
        TypeMinutes,
        #[doc = "A duration of seconds; a special floating point type."]
        TypeSeconds,
        #[doc = "A custom metric of standard type; a special floating point type."]
        TypeStandard,
    }
    impl MetricMetadataType {
        pub fn as_str(self) -> &'static str {
            match self {
                MetricMetadataType::MetricTypeUnspecified => "METRIC_TYPE_UNSPECIFIED",
                MetricMetadataType::TypeCurrency => "TYPE_CURRENCY",
                MetricMetadataType::TypeFeet => "TYPE_FEET",
                MetricMetadataType::TypeFloat => "TYPE_FLOAT",
                MetricMetadataType::TypeHours => "TYPE_HOURS",
                MetricMetadataType::TypeInteger => "TYPE_INTEGER",
                MetricMetadataType::TypeKilometers => "TYPE_KILOMETERS",
                MetricMetadataType::TypeMeters => "TYPE_METERS",
                MetricMetadataType::TypeMiles => "TYPE_MILES",
                MetricMetadataType::TypeMilliseconds => "TYPE_MILLISECONDS",
                MetricMetadataType::TypeMinutes => "TYPE_MINUTES",
                MetricMetadataType::TypeSeconds => "TYPE_SECONDS",
                MetricMetadataType::TypeStandard => "TYPE_STANDARD",
            }
        }
    }
    impl ::std::convert::AsRef<str> for MetricMetadataType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MetricMetadataType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MetricMetadataType, ()> {
            Ok(match s {
                "METRIC_TYPE_UNSPECIFIED" => MetricMetadataType::MetricTypeUnspecified,
                "TYPE_CURRENCY" => MetricMetadataType::TypeCurrency,
                "TYPE_FEET" => MetricMetadataType::TypeFeet,
                "TYPE_FLOAT" => MetricMetadataType::TypeFloat,
                "TYPE_HOURS" => MetricMetadataType::TypeHours,
                "TYPE_INTEGER" => MetricMetadataType::TypeInteger,
                "TYPE_KILOMETERS" => MetricMetadataType::TypeKilometers,
                "TYPE_METERS" => MetricMetadataType::TypeMeters,
                "TYPE_MILES" => MetricMetadataType::TypeMiles,
                "TYPE_MILLISECONDS" => MetricMetadataType::TypeMilliseconds,
                "TYPE_MINUTES" => MetricMetadataType::TypeMinutes,
                "TYPE_SECONDS" => MetricMetadataType::TypeSeconds,
                "TYPE_STANDARD" => MetricMetadataType::TypeStandard,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MetricMetadataType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MetricMetadataType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MetricMetadataType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "METRIC_TYPE_UNSPECIFIED" => MetricMetadataType::MetricTypeUnspecified,
                "TYPE_CURRENCY" => MetricMetadataType::TypeCurrency,
                "TYPE_FEET" => MetricMetadataType::TypeFeet,
                "TYPE_FLOAT" => MetricMetadataType::TypeFloat,
                "TYPE_HOURS" => MetricMetadataType::TypeHours,
                "TYPE_INTEGER" => MetricMetadataType::TypeInteger,
                "TYPE_KILOMETERS" => MetricMetadataType::TypeKilometers,
                "TYPE_METERS" => MetricMetadataType::TypeMeters,
                "TYPE_MILES" => MetricMetadataType::TypeMiles,
                "TYPE_MILLISECONDS" => MetricMetadataType::TypeMilliseconds,
                "TYPE_MINUTES" => MetricMetadataType::TypeMinutes,
                "TYPE_SECONDS" => MetricMetadataType::TypeSeconds,
                "TYPE_STANDARD" => MetricMetadataType::TypeStandard,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for MetricMetadataType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetricMetadataType {
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
    pub struct MetricOrderBy {
        #[doc = "A metric name in the request to order by."]
        #[serde(
            rename = "metricName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for MetricOrderBy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetricOrderBy {
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
    pub struct MetricValue {
        #[doc = "Measurement value. See MetricHeader for type."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for MetricValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MetricValue {
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
    pub struct MinuteRange {
        #[doc = "The inclusive end minute for the query as a number of minutes before now. Cannot be before `startMinutesAgo`. For example, `\"endMinutesAgo\": 15` specifies the report should include event data from prior to 15 minutes ago. If unspecified, `endMinutesAgo` is defaulted to 0. Standard Analytics properties can request any minute in the last 30 minutes of event data (`endMinutesAgo <= 29`), and 360 Analytics properties can request any minute in the last 60 minutes of event data (`endMinutesAgo <= 59`)."]
        #[serde(
            rename = "endMinutesAgo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_minutes_ago: ::std::option::Option<i32>,
        #[doc = "Assigns a name to this minute range. The dimension `dateRange` is valued to this name in a report response. If set, cannot begin with `date_range_` or `RESERVED_`. If not set, minute ranges are named by their zero based index in the request: `date_range_0`, `date_range_1`, etc."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "The inclusive start minute for the query as a number of minutes before now. For example, `\"startMinutesAgo\": 29` specifies the report should include event data from 29 minutes ago and after. Cannot be after `endMinutesAgo`. If unspecified, `startMinutesAgo` is defaulted to 29. Standard Analytics properties can request up to the last 30 minutes of event data (`startMinutesAgo <= 29`), and 360 Analytics properties can request up to the last 60 minutes of event data (`startMinutesAgo <= 59`)."]
        #[serde(
            rename = "startMinutesAgo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_minutes_ago: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for MinuteRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MinuteRange {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct NumericFilter {
        #[doc = "The operation type for this filter."]
        #[serde(
            rename = "operation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation: ::std::option::Option<crate::schemas::NumericFilterOperation>,
        #[doc = "A numeric value or a date value."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<crate::schemas::NumericValue>,
    }
    impl ::google_field_selector::FieldSelector for NumericFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NumericFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NumericFilterOperation {
        #[doc = "Equal"]
        Equal,
        #[doc = "Greater than"]
        GreaterThan,
        #[doc = "Greater than or equal"]
        GreaterThanOrEqual,
        #[doc = "Less than"]
        LessThan,
        #[doc = "Less than or equal"]
        LessThanOrEqual,
        #[doc = "Unspecified."]
        OperationUnspecified,
    }
    impl NumericFilterOperation {
        pub fn as_str(self) -> &'static str {
            match self {
                NumericFilterOperation::Equal => "EQUAL",
                NumericFilterOperation::GreaterThan => "GREATER_THAN",
                NumericFilterOperation::GreaterThanOrEqual => "GREATER_THAN_OR_EQUAL",
                NumericFilterOperation::LessThan => "LESS_THAN",
                NumericFilterOperation::LessThanOrEqual => "LESS_THAN_OR_EQUAL",
                NumericFilterOperation::OperationUnspecified => "OPERATION_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for NumericFilterOperation {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for NumericFilterOperation {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<NumericFilterOperation, ()> {
            Ok(match s {
                "EQUAL" => NumericFilterOperation::Equal,
                "GREATER_THAN" => NumericFilterOperation::GreaterThan,
                "GREATER_THAN_OR_EQUAL" => NumericFilterOperation::GreaterThanOrEqual,
                "LESS_THAN" => NumericFilterOperation::LessThan,
                "LESS_THAN_OR_EQUAL" => NumericFilterOperation::LessThanOrEqual,
                "OPERATION_UNSPECIFIED" => NumericFilterOperation::OperationUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for NumericFilterOperation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NumericFilterOperation {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NumericFilterOperation {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "EQUAL" => NumericFilterOperation::Equal,
                "GREATER_THAN" => NumericFilterOperation::GreaterThan,
                "GREATER_THAN_OR_EQUAL" => NumericFilterOperation::GreaterThanOrEqual,
                "LESS_THAN" => NumericFilterOperation::LessThan,
                "LESS_THAN_OR_EQUAL" => NumericFilterOperation::LessThanOrEqual,
                "OPERATION_UNSPECIFIED" => NumericFilterOperation::OperationUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for NumericFilterOperation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NumericFilterOperation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct NumericValue {
        #[doc = "Double value"]
        #[serde(
            rename = "doubleValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub double_value: ::std::option::Option<f64>,
        #[doc = "Integer value"]
        #[serde(
            rename = "int64Value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub int_64_value: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for NumericValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for NumericValue {
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
    pub struct OrderBy {
        #[doc = "If true, sorts by descending order."]
        #[serde(
            rename = "desc",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub desc: ::std::option::Option<bool>,
        #[doc = "Sorts results by a dimension's values."]
        #[serde(
            rename = "dimension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension: ::std::option::Option<crate::schemas::DimensionOrderBy>,
        #[doc = "Sorts results by a metric's values."]
        #[serde(
            rename = "metric",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric: ::std::option::Option<crate::schemas::MetricOrderBy>,
        #[doc = "Sorts results by a metric's values within a pivot column group."]
        #[serde(
            rename = "pivot",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pivot: ::std::option::Option<crate::schemas::PivotOrderBy>,
    }
    impl ::google_field_selector::FieldSelector for OrderBy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for OrderBy {
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
    pub struct Pivot {
        #[doc = "Dimension names for visible columns in the report response. Including \"dateRange\" produces a date range column; for each row in the response, dimension values in the date range column will indicate the corresponding date range from the request."]
        #[serde(
            rename = "fieldNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub field_names: ::std::option::Option<Vec<String>>,
        #[doc = "The number of unique combinations of dimension values to return in this pivot. The `limit` parameter is required. A `limit` of 10,000 is common for single pivot requests. The product of the `limit` for each `pivot` in a `RunPivotReportRequest` must not exceed 100,000. For example, a two pivot request with `limit: 1000` in each pivot will fail because the product is `1,000,000`."]
        #[serde(
            rename = "limit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub limit: ::std::option::Option<i64>,
        #[doc = "Aggregate the metrics by dimensions in this pivot using the specified metric_aggregations."]
        #[serde(
            rename = "metricAggregations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric_aggregations:
            ::std::option::Option<Vec<crate::schemas::PivotMetricAggregationsItems>>,
        #[doc = "The row count of the start row. The first row is counted as row 0."]
        #[serde(
            rename = "offset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub offset: ::std::option::Option<i64>,
        #[doc = "Specifies how dimensions are ordered in the pivot. In the first Pivot, the OrderBys determine Row and PivotDimensionHeader ordering; in subsequent Pivots, the OrderBys determine only PivotDimensionHeader ordering. Dimensions specified in these OrderBys must be a subset of Pivot.field_names."]
        #[serde(
            rename = "orderBys",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub order_bys: ::std::option::Option<Vec<crate::schemas::OrderBy>>,
    }
    impl ::google_field_selector::FieldSelector for Pivot {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Pivot {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PivotMetricAggregationsItems {
        #[doc = "Count operator."]
        Count,
        #[doc = "Maximum operator."]
        Maximum,
        #[doc = "Unspecified operator."]
        MetricAggregationUnspecified,
        #[doc = "Minimum operator."]
        Minimum,
        #[doc = "SUM operator."]
        Total,
    }
    impl PivotMetricAggregationsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                PivotMetricAggregationsItems::Count => "COUNT",
                PivotMetricAggregationsItems::Maximum => "MAXIMUM",
                PivotMetricAggregationsItems::MetricAggregationUnspecified => {
                    "METRIC_AGGREGATION_UNSPECIFIED"
                }
                PivotMetricAggregationsItems::Minimum => "MINIMUM",
                PivotMetricAggregationsItems::Total => "TOTAL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PivotMetricAggregationsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PivotMetricAggregationsItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PivotMetricAggregationsItems, ()> {
            Ok(match s {
                "COUNT" => PivotMetricAggregationsItems::Count,
                "MAXIMUM" => PivotMetricAggregationsItems::Maximum,
                "METRIC_AGGREGATION_UNSPECIFIED" => {
                    PivotMetricAggregationsItems::MetricAggregationUnspecified
                }
                "MINIMUM" => PivotMetricAggregationsItems::Minimum,
                "TOTAL" => PivotMetricAggregationsItems::Total,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PivotMetricAggregationsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PivotMetricAggregationsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PivotMetricAggregationsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COUNT" => PivotMetricAggregationsItems::Count,
                "MAXIMUM" => PivotMetricAggregationsItems::Maximum,
                "METRIC_AGGREGATION_UNSPECIFIED" => {
                    PivotMetricAggregationsItems::MetricAggregationUnspecified
                }
                "MINIMUM" => PivotMetricAggregationsItems::Minimum,
                "TOTAL" => PivotMetricAggregationsItems::Total,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PivotMetricAggregationsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PivotMetricAggregationsItems {
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
    pub struct PivotDimensionHeader {
        #[doc = "Values of multiple dimensions in a pivot."]
        #[serde(
            rename = "dimensionValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_values: ::std::option::Option<Vec<crate::schemas::DimensionValue>>,
    }
    impl ::google_field_selector::FieldSelector for PivotDimensionHeader {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PivotDimensionHeader {
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
    pub struct PivotHeader {
        #[doc = "The size is the same as the cardinality of the corresponding dimension combinations."]
        #[serde(
            rename = "pivotDimensionHeaders",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pivot_dimension_headers:
            ::std::option::Option<Vec<crate::schemas::PivotDimensionHeader>>,
        #[doc = "The cardinality of the pivot. The total number of rows for this pivot's fields regardless of how the parameters `offset` and `limit` are specified in the request."]
        #[serde(
            rename = "rowCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_count: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for PivotHeader {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PivotHeader {
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
    pub struct PivotOrderBy {
        #[doc = "In the response to order by, order rows by this column. Must be a metric name from the request."]
        #[serde(
            rename = "metricName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric_name: ::std::option::Option<String>,
        #[doc = "Used to select a dimension name and value pivot. If multiple pivot selections are given, the sort occurs on rows where all pivot selection dimension name and value pairs match the row's dimension name and value pair."]
        #[serde(
            rename = "pivotSelections",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pivot_selections: ::std::option::Option<Vec<crate::schemas::PivotSelection>>,
    }
    impl ::google_field_selector::FieldSelector for PivotOrderBy {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PivotOrderBy {
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
    pub struct PivotSelection {
        #[doc = "Must be a dimension name from the request."]
        #[serde(
            rename = "dimensionName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_name: ::std::option::Option<String>,
        #[doc = "Order by only when the named dimension is this value."]
        #[serde(
            rename = "dimensionValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PivotSelection {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PivotSelection {
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
    pub struct PropertyQuota {
        #[doc = "Standard Analytics Properties can send up to 10 concurrent requests; Analytics 360 Properties can use up to 50 concurrent requests."]
        #[serde(
            rename = "concurrentRequests",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub concurrent_requests: ::std::option::Option<crate::schemas::QuotaStatus>,
        #[doc = "Analytics Properties can send up to 120 requests with potentially thresholded dimensions per hour. In a batch request, each report request is individually counted for this quota if the request contains potentially thresholded dimensions."]
        #[serde(
            rename = "potentiallyThresholdedRequestsPerHour",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub potentially_thresholded_requests_per_hour:
            ::std::option::Option<crate::schemas::QuotaStatus>,
        #[doc = "Standard Analytics Properties and cloud project pairs can have up to 10 server errors per hour; Analytics 360 Properties and cloud project pairs can have up to 50 server errors per hour."]
        #[serde(
            rename = "serverErrorsPerProjectPerHour",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub server_errors_per_project_per_hour: ::std::option::Option<crate::schemas::QuotaStatus>,
        #[doc = "Standard Analytics Properties can use up to 25,000 tokens per day; Analytics 360 Properties can use 250,000 tokens per day. Most requests consume fewer than 10 tokens."]
        #[serde(
            rename = "tokensPerDay",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tokens_per_day: ::std::option::Option<crate::schemas::QuotaStatus>,
        #[doc = "Standard Analytics Properties can use up to 5,000 tokens per hour; Analytics 360 Properties can use 50,000 tokens per hour. An API request consumes a single number of tokens, and that number is deducted from both the hourly and daily quotas."]
        #[serde(
            rename = "tokensPerHour",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tokens_per_hour: ::std::option::Option<crate::schemas::QuotaStatus>,
    }
    impl ::google_field_selector::FieldSelector for PropertyQuota {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PropertyQuota {
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
    pub struct QuotaStatus {
        #[doc = "Quota consumed by this request."]
        #[serde(
            rename = "consumed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub consumed: ::std::option::Option<i32>,
        #[doc = "Quota remaining after this request."]
        #[serde(
            rename = "remaining",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub remaining: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for QuotaStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QuotaStatus {
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
    pub struct ResponseMetaData {
        #[doc = "The currency code used in this report. Intended to be used in formatting currency metrics like `purchaseRevenue` for visualization. If currency_code was specified in the request, this response parameter will echo the request parameter; otherwise, this response parameter is the property's current currency_code. Currency codes are string encodings of currency types from the ISO 4217 standard (https://en.wikipedia.org/wiki/ISO_4217); for example \"USD\", \"EUR\", \"JPY\". To learn more, see https://support.google.com/analytics/answer/9796179."]
        #[serde(
            rename = "currencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency_code: ::std::option::Option<String>,
        #[doc = "If true, indicates some buckets of dimension combinations are rolled into \"(other)\" row. This can happen for high cardinality reports."]
        #[serde(
            rename = "dataLossFromOtherRow",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_loss_from_other_row: ::std::option::Option<bool>,
        #[doc = "If empty reason is specified, the report is empty for this reason."]
        #[serde(
            rename = "emptyReason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub empty_reason: ::std::option::Option<String>,
        #[doc = "Describes the schema restrictions actively enforced in creating this report. To learn more, see [Access and data-restriction management](https://support.google.com/analytics/answer/10851388)."]
        #[serde(
            rename = "schemaRestrictionResponse",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schema_restriction_response:
            ::std::option::Option<crate::schemas::SchemaRestrictionResponse>,
        #[doc = "If `subjectToThresholding` is true, this report is subject to thresholding and only returns data that meets the minimum aggregation thresholds. It is possible for a request to be subject to thresholding thresholding and no data is absent from the report, and this happens when all data is above the thresholds. To learn more, see [Data thresholds](https://support.google.com/analytics/answer/9383630) and [About Demographics and Interests](https://support.google.com/analytics/answer/2799357)."]
        #[serde(
            rename = "subjectToThresholding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subject_to_thresholding: ::std::option::Option<bool>,
        #[doc = "The property's current timezone. Intended to be used to interpret time-based dimensions like `hour` and `minute`. Formatted as strings from the IANA Time Zone database (https://www.iana.org/time-zones); for example \"America/New_York\" or \"Asia/Tokyo\"."]
        #[serde(
            rename = "timeZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_zone: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ResponseMetaData {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResponseMetaData {
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
    pub struct Row {
        #[doc = "List of requested dimension values. In a PivotReport, dimension_values are only listed for dimensions included in a pivot."]
        #[serde(
            rename = "dimensionValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_values: ::std::option::Option<Vec<crate::schemas::DimensionValue>>,
        #[doc = "List of requested visible metric values."]
        #[serde(
            rename = "metricValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric_values: ::std::option::Option<Vec<crate::schemas::MetricValue>>,
    }
    impl ::google_field_selector::FieldSelector for Row {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Row {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct RunPivotReportRequest {
        #[doc = "Cohort group associated with this request. If there is a cohort group in the request the 'cohort' dimension must be present."]
        #[serde(
            rename = "cohortSpec",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cohort_spec: ::std::option::Option<crate::schemas::CohortSpec>,
        #[doc = "A currency code in ISO4217 format, such as \"AED\", \"USD\", \"JPY\". If the field is empty, the report uses the property's default currency."]
        #[serde(
            rename = "currencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency_code: ::std::option::Option<String>,
        #[doc = "The date range to retrieve event data for the report. If multiple date ranges are specified, event data from each date range is used in the report. A special dimension with field name \"dateRange\" can be included in a Pivot's field names; if included, the report compares between date ranges. In a cohort request, this `dateRanges` must be unspecified."]
        #[serde(
            rename = "dateRanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_ranges: ::std::option::Option<Vec<crate::schemas::DateRange>>,
        #[doc = "The filter clause of dimensions. Dimensions must be requested to be used in this filter. Metrics cannot be used in this filter."]
        #[serde(
            rename = "dimensionFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_filter: ::std::option::Option<crate::schemas::FilterExpression>,
        #[doc = "The dimensions requested. All defined dimensions must be used by one of the following: dimension_expression, dimension_filter, pivots, order_bys."]
        #[serde(
            rename = "dimensions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimensions: ::std::option::Option<Vec<crate::schemas::Dimension>>,
        #[doc = "If false or unspecified, each row with all metrics equal to 0 will not be returned. If true, these rows will be returned if they are not separately removed by a filter."]
        #[serde(
            rename = "keepEmptyRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keep_empty_rows: ::std::option::Option<bool>,
        #[doc = "The filter clause of metrics. Applied at post aggregation phase, similar to SQL having-clause. Metrics must be requested to be used in this filter. Dimensions cannot be used in this filter."]
        #[serde(
            rename = "metricFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric_filter: ::std::option::Option<crate::schemas::FilterExpression>,
        #[doc = "The metrics requested, at least one metric needs to be specified. All defined metrics must be used by one of the following: metric_expression, metric_filter, order_bys."]
        #[serde(
            rename = "metrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metrics: ::std::option::Option<Vec<crate::schemas::Metric>>,
        #[doc = "Describes the visual format of the report's dimensions in columns or rows. The union of the fieldNames (dimension names) in all pivots must be a subset of dimension names defined in Dimensions. No two pivots can share a dimension. A dimension is only visible if it appears in a pivot."]
        #[serde(
            rename = "pivots",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pivots: ::std::option::Option<Vec<crate::schemas::Pivot>>,
        #[doc = "A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). Within a batch request, this property should either be unspecified or consistent with the batch-level property. Example: properties/1234"]
        #[serde(
            rename = "property",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub property: ::std::option::Option<String>,
        #[doc = "Toggles whether to return the current state of this Analytics Property's quota. Quota is returned in [PropertyQuota](#PropertyQuota)."]
        #[serde(
            rename = "returnPropertyQuota",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub return_property_quota: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for RunPivotReportRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RunPivotReportRequest {
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
    pub struct RunPivotReportResponse {
        #[doc = "Aggregation of metric values. Can be totals, minimums, or maximums. The returned aggregations are controlled by the metric_aggregations in the pivot. The type of aggregation returned in each row is shown by the dimension_values which are set to \"RESERVED_\"."]
        #[serde(
            rename = "aggregates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub aggregates: ::std::option::Option<Vec<crate::schemas::Row>>,
        #[doc = "Describes dimension columns. The number of DimensionHeaders and ordering of DimensionHeaders matches the dimensions present in rows."]
        #[serde(
            rename = "dimensionHeaders",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_headers: ::std::option::Option<Vec<crate::schemas::DimensionHeader>>,
        #[doc = "Identifies what kind of resource this message is. This `kind` is always the fixed string \"analyticsData#runPivotReport\". Useful to distinguish between response types in JSON."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Metadata for the report."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::ResponseMetaData>,
        #[doc = "Describes metric columns. The number of MetricHeaders and ordering of MetricHeaders matches the metrics present in rows."]
        #[serde(
            rename = "metricHeaders",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric_headers: ::std::option::Option<Vec<crate::schemas::MetricHeader>>,
        #[doc = "Summarizes the columns and rows created by a pivot. Each pivot in the request produces one header in the response. If we have a request like this: \"pivots\": [{ \"fieldNames\": [\"country\", \"city\"] }, { \"fieldNames\": \"eventName\" }] We will have the following `pivotHeaders` in the response: \"pivotHeaders\" : [{ \"dimensionHeaders\": [{ \"dimensionValues\": [ { \"value\": \"United Kingdom\" }, { \"value\": \"London\" } ] }, { \"dimensionValues\": [ { \"value\": \"Japan\" }, { \"value\": \"Osaka\" } ] }] }, { \"dimensionHeaders\": [{ \"dimensionValues\": [{ \"value\": \"session_start\" }] }, { \"dimensionValues\": [{ \"value\": \"scroll\" }] }] }]"]
        #[serde(
            rename = "pivotHeaders",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pivot_headers: ::std::option::Option<Vec<crate::schemas::PivotHeader>>,
        #[doc = "This Analytics Property's quota state including this request."]
        #[serde(
            rename = "propertyQuota",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub property_quota: ::std::option::Option<crate::schemas::PropertyQuota>,
        #[doc = "Rows of dimension value combinations and metric values in the report."]
        #[serde(
            rename = "rows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows: ::std::option::Option<Vec<crate::schemas::Row>>,
    }
    impl ::google_field_selector::FieldSelector for RunPivotReportResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RunPivotReportResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct RunRealtimeReportRequest {
        #[doc = "The filter clause of dimensions. Dimensions must be requested to be used in this filter. Metrics cannot be used in this filter."]
        #[serde(
            rename = "dimensionFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_filter: ::std::option::Option<crate::schemas::FilterExpression>,
        #[doc = "The dimensions requested and displayed."]
        #[serde(
            rename = "dimensions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimensions: ::std::option::Option<Vec<crate::schemas::Dimension>>,
        #[doc = "The number of rows to return. If unspecified, 10,000 rows are returned. The API returns a maximum of 100,000 rows per request, no matter how many you ask for. `limit` must be positive. The API can also return fewer rows than the requested `limit`, if there aren't as many dimension values as the `limit`. For instance, there are fewer than 300 possible values for the dimension `country`, so when reporting on only `country`, you can't get more than 300 rows, even if you set `limit` to a higher value."]
        #[serde(
            rename = "limit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub limit: ::std::option::Option<i64>,
        #[doc = "Aggregation of metrics. Aggregated metric values will be shown in rows where the dimension_values are set to \"RESERVED_(MetricAggregation)\"."]
        #[serde(
            rename = "metricAggregations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric_aggregations: ::std::option::Option<
            Vec<crate::schemas::RunRealtimeReportRequestMetricAggregationsItems>,
        >,
        #[doc = "The filter clause of metrics. Applied at post aggregation phase, similar to SQL having-clause. Metrics must be requested to be used in this filter. Dimensions cannot be used in this filter."]
        #[serde(
            rename = "metricFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric_filter: ::std::option::Option<crate::schemas::FilterExpression>,
        #[doc = "The metrics requested and displayed."]
        #[serde(
            rename = "metrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metrics: ::std::option::Option<Vec<crate::schemas::Metric>>,
        #[doc = "The minute ranges of event data to read. If unspecified, one minute range for the last 30 minutes will be used. If multiple minute ranges are requested, each response row will contain a zero based minute range index. If two minute ranges overlap, the event data for the overlapping minutes is included in the response rows for both minute ranges."]
        #[serde(
            rename = "minuteRanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub minute_ranges: ::std::option::Option<Vec<crate::schemas::MinuteRange>>,
        #[doc = "Specifies how rows are ordered in the response."]
        #[serde(
            rename = "orderBys",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub order_bys: ::std::option::Option<Vec<crate::schemas::OrderBy>>,
        #[doc = "Toggles whether to return the current state of this Analytics Property's Realtime quota. Quota is returned in [PropertyQuota](#PropertyQuota)."]
        #[serde(
            rename = "returnPropertyQuota",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub return_property_quota: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for RunRealtimeReportRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RunRealtimeReportRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RunRealtimeReportRequestMetricAggregationsItems {
        #[doc = "Count operator."]
        Count,
        #[doc = "Maximum operator."]
        Maximum,
        #[doc = "Unspecified operator."]
        MetricAggregationUnspecified,
        #[doc = "Minimum operator."]
        Minimum,
        #[doc = "SUM operator."]
        Total,
    }
    impl RunRealtimeReportRequestMetricAggregationsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                RunRealtimeReportRequestMetricAggregationsItems::Count => "COUNT",
                RunRealtimeReportRequestMetricAggregationsItems::Maximum => "MAXIMUM",
                RunRealtimeReportRequestMetricAggregationsItems::MetricAggregationUnspecified => {
                    "METRIC_AGGREGATION_UNSPECIFIED"
                }
                RunRealtimeReportRequestMetricAggregationsItems::Minimum => "MINIMUM",
                RunRealtimeReportRequestMetricAggregationsItems::Total => "TOTAL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for RunRealtimeReportRequestMetricAggregationsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RunRealtimeReportRequestMetricAggregationsItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<RunRealtimeReportRequestMetricAggregationsItems, ()> {
            Ok(match s {
                "COUNT" => RunRealtimeReportRequestMetricAggregationsItems::Count,
                "MAXIMUM" => RunRealtimeReportRequestMetricAggregationsItems::Maximum,
                "METRIC_AGGREGATION_UNSPECIFIED" => {
                    RunRealtimeReportRequestMetricAggregationsItems::MetricAggregationUnspecified
                }
                "MINIMUM" => RunRealtimeReportRequestMetricAggregationsItems::Minimum,
                "TOTAL" => RunRealtimeReportRequestMetricAggregationsItems::Total,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for RunRealtimeReportRequestMetricAggregationsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RunRealtimeReportRequestMetricAggregationsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RunRealtimeReportRequestMetricAggregationsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COUNT" => RunRealtimeReportRequestMetricAggregationsItems::Count,
                "MAXIMUM" => RunRealtimeReportRequestMetricAggregationsItems::Maximum,
                "METRIC_AGGREGATION_UNSPECIFIED" => {
                    RunRealtimeReportRequestMetricAggregationsItems::MetricAggregationUnspecified
                }
                "MINIMUM" => RunRealtimeReportRequestMetricAggregationsItems::Minimum,
                "TOTAL" => RunRealtimeReportRequestMetricAggregationsItems::Total,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RunRealtimeReportRequestMetricAggregationsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RunRealtimeReportRequestMetricAggregationsItems {
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
    pub struct RunRealtimeReportResponse {
        #[doc = "Describes dimension columns. The number of DimensionHeaders and ordering of DimensionHeaders matches the dimensions present in rows."]
        #[serde(
            rename = "dimensionHeaders",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_headers: ::std::option::Option<Vec<crate::schemas::DimensionHeader>>,
        #[doc = "Identifies what kind of resource this message is. This `kind` is always the fixed string \"analyticsData#runRealtimeReport\". Useful to distinguish between response types in JSON."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "If requested, the maximum values of metrics."]
        #[serde(
            rename = "maximums",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub maximums: ::std::option::Option<Vec<crate::schemas::Row>>,
        #[doc = "Describes metric columns. The number of MetricHeaders and ordering of MetricHeaders matches the metrics present in rows."]
        #[serde(
            rename = "metricHeaders",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric_headers: ::std::option::Option<Vec<crate::schemas::MetricHeader>>,
        #[doc = "If requested, the minimum values of metrics."]
        #[serde(
            rename = "minimums",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub minimums: ::std::option::Option<Vec<crate::schemas::Row>>,
        #[doc = "This Analytics Property's Realtime quota state including this request."]
        #[serde(
            rename = "propertyQuota",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub property_quota: ::std::option::Option<crate::schemas::PropertyQuota>,
        #[doc = "The total number of rows in the query result. `rowCount` is independent of the number of rows returned in the response and the `limit` request parameter. For example if a query returns 175 rows and includes `limit` of 50 in the API request, the response will contain `rowCount` of 175 but only 50 rows."]
        #[serde(
            rename = "rowCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_count: ::std::option::Option<i32>,
        #[doc = "Rows of dimension value combinations and metric values in the report."]
        #[serde(
            rename = "rows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows: ::std::option::Option<Vec<crate::schemas::Row>>,
        #[doc = "If requested, the totaled values of metrics."]
        #[serde(
            rename = "totals",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub totals: ::std::option::Option<Vec<crate::schemas::Row>>,
    }
    impl ::google_field_selector::FieldSelector for RunRealtimeReportResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RunRealtimeReportResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct RunReportRequest {
        #[doc = "Cohort group associated with this request. If there is a cohort group in the request the 'cohort' dimension must be present."]
        #[serde(
            rename = "cohortSpec",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cohort_spec: ::std::option::Option<crate::schemas::CohortSpec>,
        #[doc = "A currency code in ISO4217 format, such as \"AED\", \"USD\", \"JPY\". If the field is empty, the report uses the property's default currency."]
        #[serde(
            rename = "currencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency_code: ::std::option::Option<String>,
        #[doc = "Date ranges of data to read. If multiple date ranges are requested, each response row will contain a zero based date range index. If two date ranges overlap, the event data for the overlapping days is included in the response rows for both date ranges. In a cohort request, this `dateRanges` must be unspecified."]
        #[serde(
            rename = "dateRanges",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_ranges: ::std::option::Option<Vec<crate::schemas::DateRange>>,
        #[doc = "Dimension filters allow you to ask for only specific dimension values in the report. To learn more, see [Fundamentals of Dimension Filters](https://developers.google.com/analytics/devguides/reporting/data/v1/basics#dimension_filters) for examples. Metrics cannot be used in this filter."]
        #[serde(
            rename = "dimensionFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_filter: ::std::option::Option<crate::schemas::FilterExpression>,
        #[doc = "The dimensions requested and displayed."]
        #[serde(
            rename = "dimensions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimensions: ::std::option::Option<Vec<crate::schemas::Dimension>>,
        #[doc = "If false or unspecified, each row with all metrics equal to 0 will not be returned. If true, these rows will be returned if they are not separately removed by a filter."]
        #[serde(
            rename = "keepEmptyRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub keep_empty_rows: ::std::option::Option<bool>,
        #[doc = "The number of rows to return. If unspecified, 10,000 rows are returned. The API returns a maximum of 100,000 rows per request, no matter how many you ask for. `limit` must be positive. The API can also return fewer rows than the requested `limit`, if there aren't as many dimension values as the `limit`. For instance, there are fewer than 300 possible values for the dimension `country`, so when reporting on only `country`, you can't get more than 300 rows, even if you set `limit` to a higher value. To learn more about this pagination parameter, see [Pagination](https://developers.google.com/analytics/devguides/reporting/data/v1/basics#pagination)."]
        #[serde(
            rename = "limit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub limit: ::std::option::Option<i64>,
        #[doc = "Aggregation of metrics. Aggregated metric values will be shown in rows where the dimension_values are set to \"RESERVED_(MetricAggregation)\"."]
        #[serde(
            rename = "metricAggregations",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric_aggregations:
            ::std::option::Option<Vec<crate::schemas::RunReportRequestMetricAggregationsItems>>,
        #[doc = "The filter clause of metrics. Applied after aggregating the report's rows, similar to SQL having-clause. Dimensions cannot be used in this filter."]
        #[serde(
            rename = "metricFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric_filter: ::std::option::Option<crate::schemas::FilterExpression>,
        #[doc = "The metrics requested and displayed."]
        #[serde(
            rename = "metrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metrics: ::std::option::Option<Vec<crate::schemas::Metric>>,
        #[doc = "The row count of the start row. The first row is counted as row 0. When paging, the first request does not specify offset; or equivalently, sets offset to 0; the first request returns the first `limit` of rows. The second request sets offset to the `limit` of the first request; the second request returns the second `limit` of rows. To learn more about this pagination parameter, see [Pagination](https://developers.google.com/analytics/devguides/reporting/data/v1/basics#pagination)."]
        #[serde(
            rename = "offset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub offset: ::std::option::Option<i64>,
        #[doc = "Specifies how rows are ordered in the response."]
        #[serde(
            rename = "orderBys",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub order_bys: ::std::option::Option<Vec<crate::schemas::OrderBy>>,
        #[doc = "A Google Analytics GA4 property identifier whose events are tracked. Specified in the URL path and not the body. To learn more, see [where to find your Property ID](https://developers.google.com/analytics/devguides/reporting/data/v1/property-id). Within a batch request, this property should either be unspecified or consistent with the batch-level property. Example: properties/1234"]
        #[serde(
            rename = "property",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub property: ::std::option::Option<String>,
        #[doc = "Toggles whether to return the current state of this Analytics Property's quota. Quota is returned in [PropertyQuota](#PropertyQuota)."]
        #[serde(
            rename = "returnPropertyQuota",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub return_property_quota: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for RunReportRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RunReportRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RunReportRequestMetricAggregationsItems {
        #[doc = "Count operator."]
        Count,
        #[doc = "Maximum operator."]
        Maximum,
        #[doc = "Unspecified operator."]
        MetricAggregationUnspecified,
        #[doc = "Minimum operator."]
        Minimum,
        #[doc = "SUM operator."]
        Total,
    }
    impl RunReportRequestMetricAggregationsItems {
        pub fn as_str(self) -> &'static str {
            match self {
                RunReportRequestMetricAggregationsItems::Count => "COUNT",
                RunReportRequestMetricAggregationsItems::Maximum => "MAXIMUM",
                RunReportRequestMetricAggregationsItems::MetricAggregationUnspecified => {
                    "METRIC_AGGREGATION_UNSPECIFIED"
                }
                RunReportRequestMetricAggregationsItems::Minimum => "MINIMUM",
                RunReportRequestMetricAggregationsItems::Total => "TOTAL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for RunReportRequestMetricAggregationsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RunReportRequestMetricAggregationsItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<RunReportRequestMetricAggregationsItems, ()> {
            Ok(match s {
                "COUNT" => RunReportRequestMetricAggregationsItems::Count,
                "MAXIMUM" => RunReportRequestMetricAggregationsItems::Maximum,
                "METRIC_AGGREGATION_UNSPECIFIED" => {
                    RunReportRequestMetricAggregationsItems::MetricAggregationUnspecified
                }
                "MINIMUM" => RunReportRequestMetricAggregationsItems::Minimum,
                "TOTAL" => RunReportRequestMetricAggregationsItems::Total,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for RunReportRequestMetricAggregationsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RunReportRequestMetricAggregationsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RunReportRequestMetricAggregationsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COUNT" => RunReportRequestMetricAggregationsItems::Count,
                "MAXIMUM" => RunReportRequestMetricAggregationsItems::Maximum,
                "METRIC_AGGREGATION_UNSPECIFIED" => {
                    RunReportRequestMetricAggregationsItems::MetricAggregationUnspecified
                }
                "MINIMUM" => RunReportRequestMetricAggregationsItems::Minimum,
                "TOTAL" => RunReportRequestMetricAggregationsItems::Total,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RunReportRequestMetricAggregationsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RunReportRequestMetricAggregationsItems {
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
    pub struct RunReportResponse {
        #[doc = "Describes dimension columns. The number of DimensionHeaders and ordering of DimensionHeaders matches the dimensions present in rows."]
        #[serde(
            rename = "dimensionHeaders",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_headers: ::std::option::Option<Vec<crate::schemas::DimensionHeader>>,
        #[doc = "Identifies what kind of resource this message is. This `kind` is always the fixed string \"analyticsData#runReport\". Useful to distinguish between response types in JSON."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "If requested, the maximum values of metrics."]
        #[serde(
            rename = "maximums",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub maximums: ::std::option::Option<Vec<crate::schemas::Row>>,
        #[doc = "Metadata for the report."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::ResponseMetaData>,
        #[doc = "Describes metric columns. The number of MetricHeaders and ordering of MetricHeaders matches the metrics present in rows."]
        #[serde(
            rename = "metricHeaders",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metric_headers: ::std::option::Option<Vec<crate::schemas::MetricHeader>>,
        #[doc = "If requested, the minimum values of metrics."]
        #[serde(
            rename = "minimums",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub minimums: ::std::option::Option<Vec<crate::schemas::Row>>,
        #[doc = "This Analytics Property's quota state including this request."]
        #[serde(
            rename = "propertyQuota",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub property_quota: ::std::option::Option<crate::schemas::PropertyQuota>,
        #[doc = "The total number of rows in the query result. `rowCount` is independent of the number of rows returned in the response, the `limit` request parameter, and the `offset` request parameter. For example if a query returns 175 rows and includes `limit` of 50 in the API request, the response will contain `rowCount` of 175 but only 50 rows. To learn more about this pagination parameter, see [Pagination](https://developers.google.com/analytics/devguides/reporting/data/v1/basics#pagination)."]
        #[serde(
            rename = "rowCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub row_count: ::std::option::Option<i32>,
        #[doc = "Rows of dimension value combinations and metric values in the report."]
        #[serde(
            rename = "rows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows: ::std::option::Option<Vec<crate::schemas::Row>>,
        #[doc = "If requested, the totaled values of metrics."]
        #[serde(
            rename = "totals",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub totals: ::std::option::Option<Vec<crate::schemas::Row>>,
    }
    impl ::google_field_selector::FieldSelector for RunReportResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RunReportResponse {
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
    pub struct SchemaRestrictionResponse {
        #[doc = "All restrictions actively enforced in creating the report. For example, `purchaseRevenue` always has the restriction type `REVENUE_DATA`. However, this active response restriction is only populated if the user's custom role disallows access to `REVENUE_DATA`."]
        #[serde(
            rename = "activeMetricRestrictions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub active_metric_restrictions:
            ::std::option::Option<Vec<crate::schemas::ActiveMetricRestriction>>,
    }
    impl ::google_field_selector::FieldSelector for SchemaRestrictionResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SchemaRestrictionResponse {
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
    pub struct StringFilter {
        #[doc = "If true, the string value is case sensitive."]
        #[serde(
            rename = "caseSensitive",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub case_sensitive: ::std::option::Option<bool>,
        #[doc = "The match type for this filter."]
        #[serde(
            rename = "matchType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub match_type: ::std::option::Option<crate::schemas::StringFilterMatchType>,
        #[doc = "The string value used for the matching."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for StringFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StringFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum StringFilterMatchType {
        #[doc = "Begins with the string value."]
        BeginsWith,
        #[doc = "Contains the string value."]
        Contains,
        #[doc = "Ends with the string value."]
        EndsWith,
        #[doc = "Exact match of the string value."]
        Exact,
        #[doc = "Full match for the regular expression with the string value."]
        FullRegexp,
        #[doc = "Unspecified"]
        MatchTypeUnspecified,
        #[doc = "Partial match for the regular expression with the string value."]
        PartialRegexp,
    }
    impl StringFilterMatchType {
        pub fn as_str(self) -> &'static str {
            match self {
                StringFilterMatchType::BeginsWith => "BEGINS_WITH",
                StringFilterMatchType::Contains => "CONTAINS",
                StringFilterMatchType::EndsWith => "ENDS_WITH",
                StringFilterMatchType::Exact => "EXACT",
                StringFilterMatchType::FullRegexp => "FULL_REGEXP",
                StringFilterMatchType::MatchTypeUnspecified => "MATCH_TYPE_UNSPECIFIED",
                StringFilterMatchType::PartialRegexp => "PARTIAL_REGEXP",
            }
        }
    }
    impl ::std::convert::AsRef<str> for StringFilterMatchType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for StringFilterMatchType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<StringFilterMatchType, ()> {
            Ok(match s {
                "BEGINS_WITH" => StringFilterMatchType::BeginsWith,
                "CONTAINS" => StringFilterMatchType::Contains,
                "ENDS_WITH" => StringFilterMatchType::EndsWith,
                "EXACT" => StringFilterMatchType::Exact,
                "FULL_REGEXP" => StringFilterMatchType::FullRegexp,
                "MATCH_TYPE_UNSPECIFIED" => StringFilterMatchType::MatchTypeUnspecified,
                "PARTIAL_REGEXP" => StringFilterMatchType::PartialRegexp,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for StringFilterMatchType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for StringFilterMatchType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for StringFilterMatchType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BEGINS_WITH" => StringFilterMatchType::BeginsWith,
                "CONTAINS" => StringFilterMatchType::Contains,
                "ENDS_WITH" => StringFilterMatchType::EndsWith,
                "EXACT" => StringFilterMatchType::Exact,
                "FULL_REGEXP" => StringFilterMatchType::FullRegexp,
                "MATCH_TYPE_UNSPECIFIED" => StringFilterMatchType::MatchTypeUnspecified,
                "PARTIAL_REGEXP" => StringFilterMatchType::PartialRegexp,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for StringFilterMatchType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StringFilterMatchType {
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
    #[doc = "Actions that can be performed on the properties resource"]
    pub fn properties(&self) -> crate::resources::properties::PropertiesActions {
        crate::resources::properties::PropertiesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod properties {
        pub mod params {}
        pub struct PropertiesActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> PropertiesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Returns multiple pivot reports in a batch. All reports must be for the same GA4 Property."]
            pub fn batch_run_pivot_reports(
                &self,
                request: crate::schemas::BatchRunPivotReportsRequest,
                property: impl Into<String>,
            ) -> BatchRunPivotReportsRequestBuilder {
                BatchRunPivotReportsRequestBuilder {
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
                    property: property.into(),
                }
            }
            #[doc = "Returns multiple reports in a batch. All reports must be for the same GA4 Property."]
            pub fn batch_run_reports(
                &self,
                request: crate::schemas::BatchRunReportsRequest,
                property: impl Into<String>,
            ) -> BatchRunReportsRequestBuilder {
                BatchRunReportsRequestBuilder {
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
                    property: property.into(),
                }
            }
            #[doc = "This compatibility method lists dimensions and metrics that can be added to a report request and maintain compatibility. This method fails if the request's dimensions and metrics are incompatible. In Google Analytics, reports fail if they request incompatible dimensions and/or metrics; in that case, you will need to remove dimensions and/or metrics from the incompatible report until the report is compatible. The Realtime and Core reports have different compatibility rules. This method checks compatibility for Core reports."]
            pub fn check_compatibility(
                &self,
                request: crate::schemas::CheckCompatibilityRequest,
                property: impl Into<String>,
            ) -> CheckCompatibilityRequestBuilder {
                CheckCompatibilityRequestBuilder {
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
                    property: property.into(),
                }
            }
            #[doc = "Returns metadata for dimensions and metrics available in reporting methods. Used to explore the dimensions and metrics. In this method, a Google Analytics GA4 Property Identifier is specified in the request, and the metadata response includes Custom dimensions and metrics as well as Universal metadata. For example if a custom metric with parameter name `levels_unlocked` is registered to a property, the Metadata response will contain `customEvent:levels_unlocked`. Universal metadata are dimensions and metrics applicable to any property such as `country` and `totalUsers`."]
            pub fn get_metadata(&self, name: impl Into<String>) -> GetMetadataRequestBuilder {
                GetMetadataRequestBuilder {
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
            #[doc = "Returns a customized pivot report of your Google Analytics event data. Pivot reports are more advanced and expressive formats than regular reports. In a pivot report, dimensions are only visible if they are included in a pivot. Multiple pivots can be specified to further dissect your data."]
            pub fn run_pivot_report(
                &self,
                request: crate::schemas::RunPivotReportRequest,
                property: impl Into<String>,
            ) -> RunPivotReportRequestBuilder {
                RunPivotReportRequestBuilder {
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
                    property: property.into(),
                }
            }
            #[doc = "The Google Analytics Realtime API returns a customized report of realtime event data for your property. These reports show events and usage from the last 30 minutes. For a guide to constructing realtime requests & understanding responses, see [Creating a Realtime Report](https://developers.google.com/analytics/devguides/reporting/data/v1/realtime-basics)."]
            pub fn run_realtime_report(
                &self,
                request: crate::schemas::RunRealtimeReportRequest,
                property: impl Into<String>,
            ) -> RunRealtimeReportRequestBuilder {
                RunRealtimeReportRequestBuilder {
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
                    property: property.into(),
                }
            }
            #[doc = "Returns a customized report of your Google Analytics event data. Reports contain statistics derived from data collected by the Google Analytics tracking code. The data returned from the API is as a table with columns for the requested dimensions and metrics. Metrics are individual measurements of user activity on your property, such as active users or event count. Dimensions break down metrics across some common criteria, such as country or event name. For a guide to constructing requests & understanding responses, see [Creating a Report](https://developers.google.com/analytics/devguides/reporting/data/v1/basics)."]
            pub fn run_report(
                &self,
                request: crate::schemas::RunReportRequest,
                property: impl Into<String>,
            ) -> RunReportRequestBuilder {
                RunReportRequestBuilder {
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
                    property: property.into(),
                }
            }
        }
        #[doc = "Created via [PropertiesActions::batch_run_pivot_reports()](struct.PropertiesActions.html#method.batch_run_pivot_reports)"]
        #[derive(Debug, Clone)]
        pub struct BatchRunPivotReportsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::BatchRunPivotReportsRequest,
            property: String,
            access_token: ::std::option::Option<String>,
            alt: ::std::option::Option<crate::params::Alt>,
            callback: ::std::option::Option<String>,
            fields: ::std::option::Option<String>,
            key: ::std::option::Option<String>,
            oauth_token: ::std::option::Option<String>,
            pretty_print: ::std::option::Option<bool>,
            quota_user: ::std::option::Option<String>,
            upload_protocol: ::std::option::Option<String>,
            upload_type: ::std::option::Option<String>,
            xgafv: ::std::option::Option<crate::params::Xgafv>,
        }
        impl<'a> BatchRunPivotReportsRequestBuilder<'a> {
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
                let fields: ::std::option::Option<String> = if fields.is_empty() {
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
            ) -> Result<crate::schemas::BatchRunPivotReportsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::BatchRunPivotReportsResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
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
                let mut output = "https://analyticsdata.googleapis.com/".to_owned();
                output.push_str("v1beta/");
                {
                    let var_as_str = &self.property;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":batchRunPivotReports");
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
        #[doc = "Created via [PropertiesActions::batch_run_reports()](struct.PropertiesActions.html#method.batch_run_reports)"]
        #[derive(Debug, Clone)]
        pub struct BatchRunReportsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::BatchRunReportsRequest,
            property: String,
            access_token: ::std::option::Option<String>,
            alt: ::std::option::Option<crate::params::Alt>,
            callback: ::std::option::Option<String>,
            fields: ::std::option::Option<String>,
            key: ::std::option::Option<String>,
            oauth_token: ::std::option::Option<String>,
            pretty_print: ::std::option::Option<bool>,
            quota_user: ::std::option::Option<String>,
            upload_protocol: ::std::option::Option<String>,
            upload_type: ::std::option::Option<String>,
            xgafv: ::std::option::Option<crate::params::Xgafv>,
        }
        impl<'a> BatchRunReportsRequestBuilder<'a> {
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
                let fields: ::std::option::Option<String> = if fields.is_empty() {
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
            ) -> Result<crate::schemas::BatchRunReportsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::BatchRunReportsResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
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
                let mut output = "https://analyticsdata.googleapis.com/".to_owned();
                output.push_str("v1beta/");
                {
                    let var_as_str = &self.property;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":batchRunReports");
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
        #[doc = "Created via [PropertiesActions::check_compatibility()](struct.PropertiesActions.html#method.check_compatibility)"]
        #[derive(Debug, Clone)]
        pub struct CheckCompatibilityRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::CheckCompatibilityRequest,
            property: String,
            access_token: ::std::option::Option<String>,
            alt: ::std::option::Option<crate::params::Alt>,
            callback: ::std::option::Option<String>,
            fields: ::std::option::Option<String>,
            key: ::std::option::Option<String>,
            oauth_token: ::std::option::Option<String>,
            pretty_print: ::std::option::Option<bool>,
            quota_user: ::std::option::Option<String>,
            upload_protocol: ::std::option::Option<String>,
            upload_type: ::std::option::Option<String>,
            xgafv: ::std::option::Option<crate::params::Xgafv>,
        }
        impl<'a> CheckCompatibilityRequestBuilder<'a> {
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
                let fields: ::std::option::Option<String> = if fields.is_empty() {
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
            ) -> Result<crate::schemas::CheckCompatibilityResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::CheckCompatibilityResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
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
                let mut output = "https://analyticsdata.googleapis.com/".to_owned();
                output.push_str("v1beta/");
                {
                    let var_as_str = &self.property;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":checkCompatibility");
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
        #[doc = "Created via [PropertiesActions::get_metadata()](struct.PropertiesActions.html#method.get_metadata)"]
        #[derive(Debug, Clone)]
        pub struct GetMetadataRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            name: String,
            access_token: ::std::option::Option<String>,
            alt: ::std::option::Option<crate::params::Alt>,
            callback: ::std::option::Option<String>,
            fields: ::std::option::Option<String>,
            key: ::std::option::Option<String>,
            oauth_token: ::std::option::Option<String>,
            pretty_print: ::std::option::Option<bool>,
            quota_user: ::std::option::Option<String>,
            upload_protocol: ::std::option::Option<String>,
            upload_type: ::std::option::Option<String>,
            xgafv: ::std::option::Option<crate::params::Xgafv>,
        }
        impl<'a> GetMetadataRequestBuilder<'a> {
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
                let fields: ::std::option::Option<String> = if fields.is_empty() {
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
            ) -> Result<crate::schemas::Metadata, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Metadata, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
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
                let mut output = "https://analyticsdata.googleapis.com/".to_owned();
                output.push_str("v1beta/");
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
        #[doc = "Created via [PropertiesActions::run_pivot_report()](struct.PropertiesActions.html#method.run_pivot_report)"]
        #[derive(Debug, Clone)]
        pub struct RunPivotReportRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::RunPivotReportRequest,
            property: String,
            access_token: ::std::option::Option<String>,
            alt: ::std::option::Option<crate::params::Alt>,
            callback: ::std::option::Option<String>,
            fields: ::std::option::Option<String>,
            key: ::std::option::Option<String>,
            oauth_token: ::std::option::Option<String>,
            pretty_print: ::std::option::Option<bool>,
            quota_user: ::std::option::Option<String>,
            upload_protocol: ::std::option::Option<String>,
            upload_type: ::std::option::Option<String>,
            xgafv: ::std::option::Option<crate::params::Xgafv>,
        }
        impl<'a> RunPivotReportRequestBuilder<'a> {
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
                let fields: ::std::option::Option<String> = if fields.is_empty() {
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
            ) -> Result<crate::schemas::RunPivotReportResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::RunPivotReportResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
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
                let mut output = "https://analyticsdata.googleapis.com/".to_owned();
                output.push_str("v1beta/");
                {
                    let var_as_str = &self.property;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":runPivotReport");
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
        #[doc = "Created via [PropertiesActions::run_realtime_report()](struct.PropertiesActions.html#method.run_realtime_report)"]
        #[derive(Debug, Clone)]
        pub struct RunRealtimeReportRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::RunRealtimeReportRequest,
            property: String,
            access_token: ::std::option::Option<String>,
            alt: ::std::option::Option<crate::params::Alt>,
            callback: ::std::option::Option<String>,
            fields: ::std::option::Option<String>,
            key: ::std::option::Option<String>,
            oauth_token: ::std::option::Option<String>,
            pretty_print: ::std::option::Option<bool>,
            quota_user: ::std::option::Option<String>,
            upload_protocol: ::std::option::Option<String>,
            upload_type: ::std::option::Option<String>,
            xgafv: ::std::option::Option<crate::params::Xgafv>,
        }
        impl<'a> RunRealtimeReportRequestBuilder<'a> {
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
                let fields: ::std::option::Option<String> = if fields.is_empty() {
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
            ) -> Result<crate::schemas::RunRealtimeReportResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::RunRealtimeReportResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
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
                let mut output = "https://analyticsdata.googleapis.com/".to_owned();
                output.push_str("v1beta/");
                {
                    let var_as_str = &self.property;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":runRealtimeReport");
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
        #[doc = "Created via [PropertiesActions::run_report()](struct.PropertiesActions.html#method.run_report)"]
        #[derive(Debug, Clone)]
        pub struct RunReportRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::RunReportRequest,
            property: String,
            access_token: ::std::option::Option<String>,
            alt: ::std::option::Option<crate::params::Alt>,
            callback: ::std::option::Option<String>,
            fields: ::std::option::Option<String>,
            key: ::std::option::Option<String>,
            oauth_token: ::std::option::Option<String>,
            pretty_print: ::std::option::Option<bool>,
            quota_user: ::std::option::Option<String>,
            upload_protocol: ::std::option::Option<String>,
            upload_type: ::std::option::Option<String>,
            xgafv: ::std::option::Option<crate::params::Xgafv>,
        }
        impl<'a> RunReportRequestBuilder<'a> {
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
                let fields: ::std::option::Option<String> = if fields.is_empty() {
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
            ) -> Result<crate::schemas::RunReportResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::RunReportResponse, crate::Error> {
                self.execute_with_fields(Some("*")).await
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub async fn execute_with_fields<T, F>(
                mut self,
                fields: ::std::option::Option<F>,
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
                let mut output = "https://analyticsdata.googleapis.com/".to_owned();
                output.push_str("v1beta/");
                {
                    let var_as_str = &self.property;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":runReport");
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
/// Represent the ability to extract the `nextPageToken` from a response.
pub trait GetNextPageToken {
    /// Get the `nextPageToken` from a response if present.
    fn next_page_token(&self) -> ::std::option::Option<String>;
}

impl GetNextPageToken for ::serde_json::Map<String, ::serde_json::Value> {
    fn next_page_token(&self) -> ::std::option::Option<String> {
        self.get("nextPageToken")
            .and_then(|t| t.as_str())
            .map(|s| s.to_owned())
    }
}
