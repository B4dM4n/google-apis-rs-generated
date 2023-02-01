#![allow(rustdoc::bare_urls)]
#![doc = "# Resources and Methods\n* [locations](resources/locations/struct.LocationsActions.html)\n  * [*getDailyMetricsTimeSeries*](resources/locations/struct.GetDailyMetricsTimeSeriesRequestBuilder.html)\n  * [searchkeywords](resources/locations/searchkeywords/struct.SearchkeywordsActions.html)\n    * [impressions](resources/locations/searchkeywords/impressions/struct.ImpressionsActions.html)\n      * [monthly](resources/locations/searchkeywords/impressions/monthly/struct.MonthlyActions.html)\n        * [*list*](resources/locations/searchkeywords/impressions/monthly/struct.ListRequestBuilder.html)\n"]
pub mod scopes {}
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
    pub struct Date {
        #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn’t significant."]
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
    pub struct DatedValue {
        #[doc = "The date that the datapoint corresponds to. This represents a month value if the day field is not set."]
        #[serde(
            rename = "date",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "The value of the datapoint."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub value: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for DatedValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DatedValue {
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
    pub struct GetDailyMetricsTimeSeriesResponse {
        #[doc = "The daily time series."]
        #[serde(
            rename = "timeSeries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_series: ::std::option::Option<crate::schemas::TimeSeries>,
    }
    impl ::google_field_selector::FieldSelector for GetDailyMetricsTimeSeriesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GetDailyMetricsTimeSeriesResponse {
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
    pub struct InsightsValue {
        #[doc = "Represents the threshold below which the actual value falls."]
        #[serde(
            rename = "threshold",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub threshold: ::std::option::Option<i64>,
        #[doc = "Represents the actual value."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub value: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for InsightsValue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for InsightsValue {
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
    pub struct ListSearchKeywordImpressionsMonthlyResponse {
        #[doc = "A token indicating the last paginated result returned. This can be used by succeeding requests to get the next “page” of keywords. It will only be present when there are more results to be returned."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Search terms which have been used to find a business."]
        #[serde(
            rename = "searchKeywordsCounts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_keywords_counts: ::std::option::Option<Vec<crate::schemas::SearchKeywordCount>>,
    }
    impl ::google_field_selector::FieldSelector for ListSearchKeywordImpressionsMonthlyResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListSearchKeywordImpressionsMonthlyResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<::google_api_bytes::Bytes>
        for ListSearchKeywordImpressionsMonthlyResponse
    {
        fn next_page_token(&self) -> ::std::option::Option<::google_api_bytes::Bytes> {
            self.next_page_token.to_owned()
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
    pub struct SearchKeywordCount {
        #[doc = "One of either: 1) The sum of the number of unique users that used the keyword in a month, aggregated for each month requested. 2) A threshold that indicates that the actual value is below this threshold."]
        #[serde(
            rename = "insightsValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub insights_value: ::std::option::Option<crate::schemas::InsightsValue>,
        #[doc = "The lower-cased string that the user entered."]
        #[serde(
            rename = "searchKeyword",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_keyword: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SearchKeywordCount {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchKeywordCount {
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
    pub struct TimeSeries {
        #[doc = "List of datapoints in the timeseries, where each datapoint is a date-value pair."]
        #[serde(
            rename = "datedValues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dated_values: ::std::option::Option<Vec<crate::schemas::DatedValue>>,
    }
    impl ::google_field_selector::FieldSelector for TimeSeries {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TimeSeries {
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
    #[doc = "Actions that can be performed on the locations resource"]
    pub fn locations(&self) -> crate::resources::locations::LocationsActions {
        crate::resources::locations::LocationsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod locations {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum GetDailyMetricsTimeSeriesDailyMetric {
                #[doc = "The number of bookings received from the business profile."]
                BusinessBookings,
                #[doc = "The number of message conversations received on the business profile."]
                BusinessConversations,
                #[doc = "The number of times a direction request was requested to the business location."]
                BusinessDirectionRequests,
                #[doc = "The number of food orders received from the business profile."]
                BusinessFoodOrders,
                #[doc = "Business impressions on Google Maps on Desktop devices. Multiple impressions by a unique user within a single day are counted as a single impression."]
                BusinessImpressionsDesktopMaps,
                #[doc = "Business impressions on Google Search on Desktop devices. Multiple impressions by a unique user within a single day are counted as a single impression."]
                BusinessImpressionsDesktopSearch,
                #[doc = "Business impressions on Google Maps on Mobile devices. Multiple impressions by a unique user within a single day are counted as a single impression."]
                BusinessImpressionsMobileMaps,
                #[doc = "Business impressions on Google Search on Mobile devices. Multiple impressions by a unique user within a single day are counted as a single impression."]
                BusinessImpressionsMobileSearch,
                #[doc = "The number of times the business profile call button was clicked."]
                CallClicks,
                #[doc = "Represents the default unknown value."]
                DailyMetricUnknown,
                #[doc = "The number of times the business profile website was clicked."]
                WebsiteClicks,
            }
            impl GetDailyMetricsTimeSeriesDailyMetric {
                pub fn as_str(self) -> &'static str {
                    match self {
                        GetDailyMetricsTimeSeriesDailyMetric::BusinessBookings => {
                            "BUSINESS_BOOKINGS"
                        }
                        GetDailyMetricsTimeSeriesDailyMetric::BusinessConversations => {
                            "BUSINESS_CONVERSATIONS"
                        }
                        GetDailyMetricsTimeSeriesDailyMetric::BusinessDirectionRequests => {
                            "BUSINESS_DIRECTION_REQUESTS"
                        }
                        GetDailyMetricsTimeSeriesDailyMetric::BusinessFoodOrders => {
                            "BUSINESS_FOOD_ORDERS"
                        }
                        GetDailyMetricsTimeSeriesDailyMetric::BusinessImpressionsDesktopMaps => {
                            "BUSINESS_IMPRESSIONS_DESKTOP_MAPS"
                        }
                        GetDailyMetricsTimeSeriesDailyMetric::BusinessImpressionsDesktopSearch => {
                            "BUSINESS_IMPRESSIONS_DESKTOP_SEARCH"
                        }
                        GetDailyMetricsTimeSeriesDailyMetric::BusinessImpressionsMobileMaps => {
                            "BUSINESS_IMPRESSIONS_MOBILE_MAPS"
                        }
                        GetDailyMetricsTimeSeriesDailyMetric::BusinessImpressionsMobileSearch => {
                            "BUSINESS_IMPRESSIONS_MOBILE_SEARCH"
                        }
                        GetDailyMetricsTimeSeriesDailyMetric::CallClicks => "CALL_CLICKS",
                        GetDailyMetricsTimeSeriesDailyMetric::DailyMetricUnknown => {
                            "DAILY_METRIC_UNKNOWN"
                        }
                        GetDailyMetricsTimeSeriesDailyMetric::WebsiteClicks => "WEBSITE_CLICKS",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for GetDailyMetricsTimeSeriesDailyMetric {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for GetDailyMetricsTimeSeriesDailyMetric {
                type Err = ();
                fn from_str(
                    s: &str,
                ) -> ::std::result::Result<GetDailyMetricsTimeSeriesDailyMetric, ()>
                {
                    Ok(match s {
                        "BUSINESS_BOOKINGS" => {
                            GetDailyMetricsTimeSeriesDailyMetric::BusinessBookings
                        }
                        "BUSINESS_CONVERSATIONS" => {
                            GetDailyMetricsTimeSeriesDailyMetric::BusinessConversations
                        }
                        "BUSINESS_DIRECTION_REQUESTS" => {
                            GetDailyMetricsTimeSeriesDailyMetric::BusinessDirectionRequests
                        }
                        "BUSINESS_FOOD_ORDERS" => {
                            GetDailyMetricsTimeSeriesDailyMetric::BusinessFoodOrders
                        }
                        "BUSINESS_IMPRESSIONS_DESKTOP_MAPS" => {
                            GetDailyMetricsTimeSeriesDailyMetric::BusinessImpressionsDesktopMaps
                        }
                        "BUSINESS_IMPRESSIONS_DESKTOP_SEARCH" => {
                            GetDailyMetricsTimeSeriesDailyMetric::BusinessImpressionsDesktopSearch
                        }
                        "BUSINESS_IMPRESSIONS_MOBILE_MAPS" => {
                            GetDailyMetricsTimeSeriesDailyMetric::BusinessImpressionsMobileMaps
                        }
                        "BUSINESS_IMPRESSIONS_MOBILE_SEARCH" => {
                            GetDailyMetricsTimeSeriesDailyMetric::BusinessImpressionsMobileSearch
                        }
                        "CALL_CLICKS" => GetDailyMetricsTimeSeriesDailyMetric::CallClicks,
                        "DAILY_METRIC_UNKNOWN" => {
                            GetDailyMetricsTimeSeriesDailyMetric::DailyMetricUnknown
                        }
                        "WEBSITE_CLICKS" => GetDailyMetricsTimeSeriesDailyMetric::WebsiteClicks,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for GetDailyMetricsTimeSeriesDailyMetric {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for GetDailyMetricsTimeSeriesDailyMetric {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for GetDailyMetricsTimeSeriesDailyMetric {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "BUSINESS_BOOKINGS" => {
                            GetDailyMetricsTimeSeriesDailyMetric::BusinessBookings
                        }
                        "BUSINESS_CONVERSATIONS" => {
                            GetDailyMetricsTimeSeriesDailyMetric::BusinessConversations
                        }
                        "BUSINESS_DIRECTION_REQUESTS" => {
                            GetDailyMetricsTimeSeriesDailyMetric::BusinessDirectionRequests
                        }
                        "BUSINESS_FOOD_ORDERS" => {
                            GetDailyMetricsTimeSeriesDailyMetric::BusinessFoodOrders
                        }
                        "BUSINESS_IMPRESSIONS_DESKTOP_MAPS" => {
                            GetDailyMetricsTimeSeriesDailyMetric::BusinessImpressionsDesktopMaps
                        }
                        "BUSINESS_IMPRESSIONS_DESKTOP_SEARCH" => {
                            GetDailyMetricsTimeSeriesDailyMetric::BusinessImpressionsDesktopSearch
                        }
                        "BUSINESS_IMPRESSIONS_MOBILE_MAPS" => {
                            GetDailyMetricsTimeSeriesDailyMetric::BusinessImpressionsMobileMaps
                        }
                        "BUSINESS_IMPRESSIONS_MOBILE_SEARCH" => {
                            GetDailyMetricsTimeSeriesDailyMetric::BusinessImpressionsMobileSearch
                        }
                        "CALL_CLICKS" => GetDailyMetricsTimeSeriesDailyMetric::CallClicks,
                        "DAILY_METRIC_UNKNOWN" => {
                            GetDailyMetricsTimeSeriesDailyMetric::DailyMetricUnknown
                        }
                        "WEBSITE_CLICKS" => GetDailyMetricsTimeSeriesDailyMetric::WebsiteClicks,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for GetDailyMetricsTimeSeriesDailyMetric {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for GetDailyMetricsTimeSeriesDailyMetric {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek {
                #[doc = "The day of the week is unspecified."]
                DayOfWeekUnspecified,
                #[doc = "Friday"]
                Friday,
                #[doc = "Monday"]
                Monday,
                #[doc = "Saturday"]
                Saturday,
                #[doc = "Sunday"]
                Sunday,
                #[doc = "Thursday"]
                Thursday,
                #[doc = "Tuesday"]
                Tuesday,
                #[doc = "Wednesday"]
                Wednesday,
            }
            impl GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek {
                pub fn as_str(self) -> &'static str {
                    match self { GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek :: DayOfWeekUnspecified => "DAY_OF_WEEK_UNSPECIFIED" , GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek :: Friday => "FRIDAY" , GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek :: Monday => "MONDAY" , GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek :: Saturday => "SATURDAY" , GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek :: Sunday => "SUNDAY" , GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek :: Thursday => "THURSDAY" , GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek :: Tuesday => "TUESDAY" , GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek :: Wednesday => "WEDNESDAY" , }
                }
            }
            impl ::std::convert::AsRef<str> for GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek {
                type Err = ();
                fn from_str(
                    s: &str,
                ) -> ::std::result::Result<GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek, ()>
                {
                    Ok (match s { "DAY_OF_WEEK_UNSPECIFIED" => GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek :: DayOfWeekUnspecified , "FRIDAY" => GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek :: Friday , "MONDAY" => GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek :: Monday , "SATURDAY" => GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek :: Saturday , "SUNDAY" => GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek :: Sunday , "THURSDAY" => GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek :: Thursday , "TUESDAY" => GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek :: Tuesday , "WEDNESDAY" => GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek :: Wednesday , _ => return Err (()) , })
                }
            }
            impl ::std::fmt::Display for GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok (match value { "DAY_OF_WEEK_UNSPECIFIED" => GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek :: DayOfWeekUnspecified , "FRIDAY" => GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek :: Friday , "MONDAY" => GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek :: Monday , "SATURDAY" => GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek :: Saturday , "SUNDAY" => GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek :: Sunday , "THURSDAY" => GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek :: Thursday , "TUESDAY" => GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek :: Tuesday , "WEDNESDAY" => GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek :: Wednesday , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
                }
            }
            impl ::google_field_selector::FieldSelector
                for GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek
            {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct LocationsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> LocationsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Returns the values for each date from a given time range that are associated with the specific daily metric. Example request: `GET https://businessprofileperformance.googleapis.com/v1/locations/12345:getDailyMetricsTimeSeries?dailyMetric=WEBSITE_CLICKS&daily_range.start_date.year=2022&daily_range.start_date.month=1&daily_range.start_date.day=1&daily_range.end_date.year=2022&daily_range.end_date.month=3&daily_range.end_date.day=31`"]
            pub fn get_daily_metrics_time_series(
                &self,
                name: impl Into<String>,
            ) -> GetDailyMetricsTimeSeriesRequestBuilder {
                GetDailyMetricsTimeSeriesRequestBuilder {
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
                    daily_metric: None,
                    daily_range_end_date_day: None,
                    daily_range_end_date_month: None,
                    daily_range_end_date_year: None,
                    daily_range_start_date_day: None,
                    daily_range_start_date_month: None,
                    daily_range_start_date_year: None,
                    daily_sub_entity_type_day_of_week: None,
                    daily_sub_entity_type_time_of_day_hours: None,
                    daily_sub_entity_type_time_of_day_minutes: None,
                    daily_sub_entity_type_time_of_day_nanos: None,
                    daily_sub_entity_type_time_of_day_seconds: None,
                }
            }
            #[doc = "Actions that can be performed on the searchkeywords resource"]
            pub fn searchkeywords(
                &self,
            ) -> crate::resources::locations::searchkeywords::SearchkeywordsActions {
                crate::resources::locations::searchkeywords::SearchkeywordsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [LocationsActions::get_daily_metrics_time_series()](struct.LocationsActions.html#method.get_daily_metrics_time_series)"]
        #[derive(Debug, Clone)]
        pub struct GetDailyMetricsTimeSeriesRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , name : String , daily_metric : :: std :: option :: Option < crate :: resources :: locations :: params :: GetDailyMetricsTimeSeriesDailyMetric > , daily_range_end_date_day : :: std :: option :: Option < i32 > , daily_range_end_date_month : :: std :: option :: Option < i32 > , daily_range_end_date_year : :: std :: option :: Option < i32 > , daily_range_start_date_day : :: std :: option :: Option < i32 > , daily_range_start_date_month : :: std :: option :: Option < i32 > , daily_range_start_date_year : :: std :: option :: Option < i32 > , daily_sub_entity_type_day_of_week : :: std :: option :: Option < crate :: resources :: locations :: params :: GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek > , daily_sub_entity_type_time_of_day_hours : :: std :: option :: Option < i32 > , daily_sub_entity_type_time_of_day_minutes : :: std :: option :: Option < i32 > , daily_sub_entity_type_time_of_day_nanos : :: std :: option :: Option < i32 > , daily_sub_entity_type_time_of_day_seconds : :: std :: option :: Option < i32 > , access_token : :: std :: option :: Option < String > , alt : :: std :: option :: Option < crate :: params :: Alt > , callback : :: std :: option :: Option < String > , fields : :: std :: option :: Option < String > , key : :: std :: option :: Option < String > , oauth_token : :: std :: option :: Option < String > , pretty_print : :: std :: option :: Option < bool > , quota_user : :: std :: option :: Option < String > , upload_protocol : :: std :: option :: Option < String > , upload_type : :: std :: option :: Option < String > , xgafv : :: std :: option :: Option < crate :: params :: Xgafv > , }
        impl<'a> GetDailyMetricsTimeSeriesRequestBuilder<'a> {
            #[doc = "Required. The metric to retrieve time series."]
            pub fn daily_metric(
                mut self,
                value: crate::resources::locations::params::GetDailyMetricsTimeSeriesDailyMetric,
            ) -> Self {
                self.daily_metric = Some(value);
                self
            }
            #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn’t significant."]
            pub fn daily_range_end_date_day(mut self, value: i32) -> Self {
                self.daily_range_end_date_day = Some(value);
                self
            }
            #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
            pub fn daily_range_end_date_month(mut self, value: i32) -> Self {
                self.daily_range_end_date_month = Some(value);
                self
            }
            #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
            pub fn daily_range_end_date_year(mut self, value: i32) -> Self {
                self.daily_range_end_date_year = Some(value);
                self
            }
            #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn’t significant."]
            pub fn daily_range_start_date_day(mut self, value: i32) -> Self {
                self.daily_range_start_date_day = Some(value);
                self
            }
            #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
            pub fn daily_range_start_date_month(mut self, value: i32) -> Self {
                self.daily_range_start_date_month = Some(value);
                self
            }
            #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
            pub fn daily_range_start_date_year(mut self, value: i32) -> Self {
                self.daily_range_start_date_year = Some(value);
                self
            }
            #[doc = "Represents the day of the week. Eg: MONDAY."]
            pub fn daily_sub_entity_type_day_of_week(
                mut self,
                value : crate :: resources :: locations :: params :: GetDailyMetricsTimeSeriesDailySubEntityTypeDayOfWeek,
            ) -> Self {
                self.daily_sub_entity_type_day_of_week = Some(value);
                self
            }
            #[doc = "Hours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value “24:00:00” for scenarios like business closing time."]
            pub fn daily_sub_entity_type_time_of_day_hours(mut self, value: i32) -> Self {
                self.daily_sub_entity_type_time_of_day_hours = Some(value);
                self
            }
            #[doc = "Minutes of hour of day. Must be from 0 to 59."]
            pub fn daily_sub_entity_type_time_of_day_minutes(mut self, value: i32) -> Self {
                self.daily_sub_entity_type_time_of_day_minutes = Some(value);
                self
            }
            #[doc = "Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
            pub fn daily_sub_entity_type_time_of_day_nanos(mut self, value: i32) -> Self {
                self.daily_sub_entity_type_time_of_day_nanos = Some(value);
                self
            }
            #[doc = "Seconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds."]
            pub fn daily_sub_entity_type_time_of_day_seconds(mut self, value: i32) -> Self {
                self.daily_sub_entity_type_time_of_day_seconds = Some(value);
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
            #[doc = "Upload protocol for media (e.g. “raw”, “multipart”)."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. “media”, “multipart”)."]
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
            ) -> Result<crate::schemas::GetDailyMetricsTimeSeriesResponse, crate::Error>
            {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::GetDailyMetricsTimeSeriesResponse, crate::Error>
            {
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
                let mut output = "https://businessprofileperformance.googleapis.com/".to_owned();
                output.push_str("v1/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":getDailyMetricsTimeSeries");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("dailyMetric", &self.daily_metric)]);
                req = req.query(&[("dailyRange.endDate.day", &self.daily_range_end_date_day)]);
                req = req.query(&[("dailyRange.endDate.month", &self.daily_range_end_date_month)]);
                req = req.query(&[("dailyRange.endDate.year", &self.daily_range_end_date_year)]);
                req = req.query(&[("dailyRange.startDate.day", &self.daily_range_start_date_day)]);
                req = req.query(&[(
                    "dailyRange.startDate.month",
                    &self.daily_range_start_date_month,
                )]);
                req = req.query(&[(
                    "dailyRange.startDate.year",
                    &self.daily_range_start_date_year,
                )]);
                req = req.query(&[(
                    "dailySubEntityType.dayOfWeek",
                    &self.daily_sub_entity_type_day_of_week,
                )]);
                req = req.query(&[(
                    "dailySubEntityType.timeOfDay.hours",
                    &self.daily_sub_entity_type_time_of_day_hours,
                )]);
                req = req.query(&[(
                    "dailySubEntityType.timeOfDay.minutes",
                    &self.daily_sub_entity_type_time_of_day_minutes,
                )]);
                req = req.query(&[(
                    "dailySubEntityType.timeOfDay.nanos",
                    &self.daily_sub_entity_type_time_of_day_nanos,
                )]);
                req = req.query(&[(
                    "dailySubEntityType.timeOfDay.seconds",
                    &self.daily_sub_entity_type_time_of_day_seconds,
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
        pub mod searchkeywords {
            pub mod params {}
            pub struct SearchkeywordsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> SearchkeywordsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Actions that can be performed on the impressions resource"]
                pub fn impressions(
                    &self,
                ) -> crate::resources::locations::searchkeywords::impressions::ImpressionsActions
                {
                    crate::resources::locations::searchkeywords::impressions::ImpressionsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            pub mod impressions {
                pub mod params {}
                pub struct ImpressionsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> ImpressionsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Actions that can be performed on the monthly resource"]                    pub fn monthly (& self) -> crate :: resources :: locations :: searchkeywords :: impressions :: monthly :: MonthlyActions{
                        crate :: resources :: locations :: searchkeywords :: impressions :: monthly :: MonthlyActions { reqwest : & self . reqwest , auth : self . auth_ref () , }
                    }
                }
                pub mod monthly {
                    pub mod params {}
                    pub struct MonthlyActions<'a> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    }
                    impl<'a> MonthlyActions<'a> {
                        fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                            self.auth
                        }
                        #[doc = "Returns the search keywords used to find a business in search or maps. Each search keyword is accompanied by impressions which are aggregated on a monthly basis. Example request: `GET https://businessprofileperformance.googleapis.com/v1/locations/12345/searchkeywords/impressions/monthly?monthly_range.start_month.year=2022&monthly_range.start_month.month=1&monthly_range.end_month.year=2022&monthly_range.end_month.month=3`"]
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
                                monthly_range_end_month_day: None,
                                monthly_range_end_month_month: None,
                                monthly_range_end_month_year: None,
                                monthly_range_start_month_day: None,
                                monthly_range_start_month_month: None,
                                monthly_range_start_month_year: None,
                                page_size: None,
                                page_token: None,
                            }
                        }
                    }
                    #[doc = "Created via [MonthlyActions::list()](struct.MonthlyActions.html#method.list)"]
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                        parent: String,
                        monthly_range_end_month_day: ::std::option::Option<i32>,
                        monthly_range_end_month_month: ::std::option::Option<i32>,
                        monthly_range_end_month_year: ::std::option::Option<i32>,
                        monthly_range_start_month_day: ::std::option::Option<i32>,
                        monthly_range_start_month_month: ::std::option::Option<i32>,
                        monthly_range_start_month_year: ::std::option::Option<i32>,
                        page_size: ::std::option::Option<i32>,
                        page_token: ::std::option::Option<::google_api_bytes::Bytes>,
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
                    impl<'a> ListRequestBuilder<'a> {
                        #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn’t significant."]
                        pub fn monthly_range_end_month_day(mut self, value: i32) -> Self {
                            self.monthly_range_end_month_day = Some(value);
                            self
                        }
                        #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
                        pub fn monthly_range_end_month_month(mut self, value: i32) -> Self {
                            self.monthly_range_end_month_month = Some(value);
                            self
                        }
                        #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
                        pub fn monthly_range_end_month_year(mut self, value: i32) -> Self {
                            self.monthly_range_end_month_year = Some(value);
                            self
                        }
                        #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn’t significant."]
                        pub fn monthly_range_start_month_day(mut self, value: i32) -> Self {
                            self.monthly_range_start_month_day = Some(value);
                            self
                        }
                        #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
                        pub fn monthly_range_start_month_month(mut self, value: i32) -> Self {
                            self.monthly_range_start_month_month = Some(value);
                            self
                        }
                        #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
                        pub fn monthly_range_start_month_year(mut self, value: i32) -> Self {
                            self.monthly_range_start_month_year = Some(value);
                            self
                        }
                        #[doc = "Optional. The number of results requested. The default page size is 100. Page size can be set to a maximum of 100."]
                        pub fn page_size(mut self, value: i32) -> Self {
                            self.page_size = Some(value);
                            self
                        }
                        #[doc = "Optional. A token indicating the next paginated result to be returned."]
                        pub fn page_token(mut self, value: impl Into<Vec<u8>>) -> Self {
                            let v: Vec<u8> = value.into();
                            self.page_token = Some(v.into());
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
                        #[doc = "Upload protocol for media (e.g. “raw”, “multipart”)."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. “media”, “multipart”)."]
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
                        ) -> Result<
                            crate::schemas::ListSearchKeywordImpressionsMonthlyResponse,
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
                            crate::schemas::ListSearchKeywordImpressionsMonthlyResponse,
                            crate::Error,
                        > {
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
                            let mut output =
                                "https://businessprofileperformance.googleapis.com/".to_owned();
                            output.push_str("v1/");
                            {
                                let var_as_str = &self.parent;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::RESERVED,
                                ));
                            }
                            output.push_str("/searchkeywords/impressions/monthly");
                            output
                        }
                        async fn _request(
                            &self,
                            path: &str,
                        ) -> Result<::reqwest::RequestBuilder, crate::Error>
                        {
                            let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                            req = req.query(&[(
                                "monthlyRange.endMonth.day",
                                &self.monthly_range_end_month_day,
                            )]);
                            req = req.query(&[(
                                "monthlyRange.endMonth.month",
                                &self.monthly_range_end_month_month,
                            )]);
                            req = req.query(&[(
                                "monthlyRange.endMonth.year",
                                &self.monthly_range_end_month_year,
                            )]);
                            req = req.query(&[(
                                "monthlyRange.startMonth.day",
                                &self.monthly_range_start_month_day,
                            )]);
                            req = req.query(&[(
                                "monthlyRange.startMonth.month",
                                &self.monthly_range_start_month_month,
                            )]);
                            req = req.query(&[(
                                "monthlyRange.startMonth.year",
                                &self.monthly_range_start_month_year,
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
pub trait GetNextPageToken<T> {
    /// Get the `nextPageToken` from a response if present.
    fn next_page_token(&self) -> ::std::option::Option<T>;
}

impl<T: ::std::convert::From<::std::string::String>> GetNextPageToken<T>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn next_page_token(&self) -> ::std::option::Option<T> {
        self.get("nextPageToken")
            .and_then(|t| t.as_str())
            .map(|s| s.to_owned().into())
    }
}
