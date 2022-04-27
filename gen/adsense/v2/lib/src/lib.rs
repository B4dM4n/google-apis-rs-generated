#![doc = "# Resources and Methods\n    * [accounts](resources/accounts/struct.AccountsActions.html)\n      * [*get*](resources/accounts/struct.GetRequestBuilder.html), [*list*](resources/accounts/struct.ListRequestBuilder.html), [*listChildAccounts*](resources/accounts/struct.ListChildAccountsRequestBuilder.html)\n      * [adclients](resources/accounts/adclients/struct.AdclientsActions.html)\n        * [*getAdcode*](resources/accounts/adclients/struct.GetAdcodeRequestBuilder.html), [*list*](resources/accounts/adclients/struct.ListRequestBuilder.html)\n        * [adunits](resources/accounts/adclients/adunits/struct.AdunitsActions.html)\n          * [*get*](resources/accounts/adclients/adunits/struct.GetRequestBuilder.html), [*getAdcode*](resources/accounts/adclients/adunits/struct.GetAdcodeRequestBuilder.html), [*list*](resources/accounts/adclients/adunits/struct.ListRequestBuilder.html), [*listLinkedCustomChannels*](resources/accounts/adclients/adunits/struct.ListLinkedCustomChannelsRequestBuilder.html)\n        * [customchannels](resources/accounts/adclients/customchannels/struct.CustomchannelsActions.html)\n          * [*get*](resources/accounts/adclients/customchannels/struct.GetRequestBuilder.html), [*list*](resources/accounts/adclients/customchannels/struct.ListRequestBuilder.html), [*listLinkedAdUnits*](resources/accounts/adclients/customchannels/struct.ListLinkedAdUnitsRequestBuilder.html)\n        * [urlchannels](resources/accounts/adclients/urlchannels/struct.UrlchannelsActions.html)\n          * [*list*](resources/accounts/adclients/urlchannels/struct.ListRequestBuilder.html)\n      * [alerts](resources/accounts/alerts/struct.AlertsActions.html)\n        * [*list*](resources/accounts/alerts/struct.ListRequestBuilder.html)\n      * [payments](resources/accounts/payments/struct.PaymentsActions.html)\n        * [*list*](resources/accounts/payments/struct.ListRequestBuilder.html)\n      * [reports](resources/accounts/reports/struct.ReportsActions.html)\n        * [*generate*](resources/accounts/reports/struct.GenerateRequestBuilder.html), [*generateCsv*](resources/accounts/reports/struct.GenerateCsvRequestBuilder.html)\n        * [saved](resources/accounts/reports/saved/struct.SavedActions.html)\n          * [*generate*](resources/accounts/reports/saved/struct.GenerateRequestBuilder.html), [*generateCsv*](resources/accounts/reports/saved/struct.GenerateCsvRequestBuilder.html), [*list*](resources/accounts/reports/saved/struct.ListRequestBuilder.html)\n      * [sites](resources/accounts/sites/struct.SitesActions.html)\n        * [*get*](resources/accounts/sites/struct.GetRequestBuilder.html), [*list*](resources/accounts/sites/struct.ListRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View and manage your AdSense data\n\n`https://www.googleapis.com/auth/adsense`"]
    pub const ADSENSE: &str = "https://www.googleapis.com/auth/adsense";
    #[doc = "View your AdSense data\n\n`https://www.googleapis.com/auth/adsense.readonly`"]
    pub const ADSENSE_READONLY: &str = "https://www.googleapis.com/auth/adsense.readonly";
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
    pub struct Account {
        #[doc = "Output only. Creation time of the account."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Output only. Display name of this account."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Resource name of the account. Format: accounts/pub-[0-9]+"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. Outstanding tasks that need to be completed as part of the sign-up process for a new account. e.g. \"billing-profile-creation\", \"phone-pin-verification\"."]
        #[serde(
            rename = "pendingTasks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pending_tasks: ::std::option::Option<Vec<String>>,
        #[doc = "Output only. Whether this account is premium."]
        #[serde(
            rename = "premium",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub premium: ::std::option::Option<bool>,
        #[doc = "Output only. State of the account."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::AccountState>,
        #[doc = "The account time zone, as used by reporting. For more information, see [changing the time zone of your reports](https://support.google.com/adsense/answer/9830725)."]
        #[serde(
            rename = "timeZone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub time_zone: ::std::option::Option<crate::schemas::TimeZone>,
    }
    impl ::google_field_selector::FieldSelector for Account {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Account {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AccountState {
        #[doc = "The account is closed and can't serve ads."]
        Closed,
        #[doc = "There are some issues with this account. Publishers should visit AdSense in order to fix the account."]
        NeedsAttention,
        #[doc = "The account is open and ready to serve ads."]
        Ready,
        #[doc = "State unspecified."]
        StateUnspecified,
    }
    impl AccountState {
        pub fn as_str(self) -> &'static str {
            match self {
                AccountState::Closed => "CLOSED",
                AccountState::NeedsAttention => "NEEDS_ATTENTION",
                AccountState::Ready => "READY",
                AccountState::StateUnspecified => "STATE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AccountState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AccountState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AccountState, ()> {
            Ok(match s {
                "CLOSED" => AccountState::Closed,
                "NEEDS_ATTENTION" => AccountState::NeedsAttention,
                "READY" => AccountState::Ready,
                "STATE_UNSPECIFIED" => AccountState::StateUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AccountState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AccountState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AccountState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CLOSED" => AccountState::Closed,
                "NEEDS_ATTENTION" => AccountState::NeedsAttention,
                "READY" => AccountState::Ready,
                "STATE_UNSPECIFIED" => AccountState::StateUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AccountState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AccountState {
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
    pub struct AdClient {
        #[doc = "Resource name of the ad client. Format: accounts/{account}/adclients/{adclient}"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. Product code of the ad client. For example, \"AFC\" for AdSense for Content."]
        #[serde(
            rename = "productCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub product_code: ::std::option::Option<String>,
        #[doc = "Output only. Unique ID of the ad client as used in the `AD_CLIENT_ID` reporting dimension. Present only if the ad client supports reporting."]
        #[serde(
            rename = "reportingDimensionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reporting_dimension_id: ::std::option::Option<String>,
        #[doc = "Output only. State of the ad client."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::AdClientState>,
    }
    impl ::google_field_selector::FieldSelector for AdClient {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AdClient {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AdClientState {
        #[doc = "Running some checks on the ad client before it is ready to serve ads."]
        GettingReady,
        #[doc = "The ad client is ready to show ads."]
        Ready,
        #[doc = "The ad client hasn't been checked yet. There are tasks pending before AdSense will start the review."]
        RequiresReview,
        #[doc = "State unspecified."]
        StateUnspecified,
    }
    impl AdClientState {
        pub fn as_str(self) -> &'static str {
            match self {
                AdClientState::GettingReady => "GETTING_READY",
                AdClientState::Ready => "READY",
                AdClientState::RequiresReview => "REQUIRES_REVIEW",
                AdClientState::StateUnspecified => "STATE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AdClientState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AdClientState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AdClientState, ()> {
            Ok(match s {
                "GETTING_READY" => AdClientState::GettingReady,
                "READY" => AdClientState::Ready,
                "REQUIRES_REVIEW" => AdClientState::RequiresReview,
                "STATE_UNSPECIFIED" => AdClientState::StateUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AdClientState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AdClientState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AdClientState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "GETTING_READY" => AdClientState::GettingReady,
                "READY" => AdClientState::Ready,
                "REQUIRES_REVIEW" => AdClientState::RequiresReview,
                "STATE_UNSPECIFIED" => AdClientState::StateUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AdClientState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AdClientState {
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
    pub struct AdClientAdCode {
        #[doc = "Output only. The AdSense code snippet to add to the head of an HTML page."]
        #[serde(
            rename = "adCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ad_code: ::std::option::Option<String>,
        #[doc = "Output only. The AdSense code snippet to add to the body of an AMP page."]
        #[serde(
            rename = "ampBody",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub amp_body: ::std::option::Option<String>,
        #[doc = "Output only. The AdSense code snippet to add to the head of an AMP page."]
        #[serde(
            rename = "ampHead",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub amp_head: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AdClientAdCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AdClientAdCode {
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
    pub struct AdUnit {
        #[doc = "Required. Settings specific to content ads (AFC)."]
        #[serde(
            rename = "contentAdsSettings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_ads_settings: ::std::option::Option<crate::schemas::ContentAdsSettings>,
        #[doc = "Required. Display name of the ad unit, as provided when the ad unit was created."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Resource name of the ad unit. Format: accounts/{account}/adclients/{adclient}/adunits/{adunit}"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. Unique ID of the ad unit as used in the `AD_UNIT_ID` reporting dimension."]
        #[serde(
            rename = "reportingDimensionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reporting_dimension_id: ::std::option::Option<String>,
        #[doc = "State of the ad unit."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::AdUnitState>,
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
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AdUnitState {
        #[doc = "Ad unit has been activated by the user."]
        Active,
        #[doc = "Ad unit has been archived by the user. Note that archived ad units are only removed from the default view in the UI. Archived ad units can still serve ads."]
        Archived,
        #[doc = "State unspecified."]
        StateUnspecified,
    }
    impl AdUnitState {
        pub fn as_str(self) -> &'static str {
            match self {
                AdUnitState::Active => "ACTIVE",
                AdUnitState::Archived => "ARCHIVED",
                AdUnitState::StateUnspecified => "STATE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AdUnitState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AdUnitState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AdUnitState, ()> {
            Ok(match s {
                "ACTIVE" => AdUnitState::Active,
                "ARCHIVED" => AdUnitState::Archived,
                "STATE_UNSPECIFIED" => AdUnitState::StateUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AdUnitState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AdUnitState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AdUnitState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVE" => AdUnitState::Active,
                "ARCHIVED" => AdUnitState::Archived,
                "STATE_UNSPECIFIED" => AdUnitState::StateUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AdUnitState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AdUnitState {
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
    pub struct AdUnitAdCode {
        #[doc = "Output only. The AdSense code snippet to add to the body of an HTML page."]
        #[serde(
            rename = "adCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ad_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AdUnitAdCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AdUnitAdCode {
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
    pub struct Alert {
        #[doc = "Output only. The localized alert message. This may contain HTML markup, such as phrase elements or links."]
        #[serde(
            rename = "message",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message: ::std::option::Option<String>,
        #[doc = "Resource name of the alert. Format: accounts/{account}/alerts/{alert}"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. Type of alert. This identifies the broad type of this alert, and provides a stable machine-readable identifier that will not be translated. For example, \"payment-hold\"."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "Output only. Severity of this alert."]
        #[serde(
            rename = "severity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub severity: ::std::option::Option<crate::schemas::AlertSeverity>,
    }
    impl ::google_field_selector::FieldSelector for Alert {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Alert {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AlertSeverity {
        #[doc = "Info."]
        Info,
        #[doc = "Severe."]
        Severe,
        #[doc = "Unspecified severity."]
        SeverityUnspecified,
        #[doc = "Warning."]
        Warning,
    }
    impl AlertSeverity {
        pub fn as_str(self) -> &'static str {
            match self {
                AlertSeverity::Info => "INFO",
                AlertSeverity::Severe => "SEVERE",
                AlertSeverity::SeverityUnspecified => "SEVERITY_UNSPECIFIED",
                AlertSeverity::Warning => "WARNING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AlertSeverity {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AlertSeverity {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AlertSeverity, ()> {
            Ok(match s {
                "INFO" => AlertSeverity::Info,
                "SEVERE" => AlertSeverity::Severe,
                "SEVERITY_UNSPECIFIED" => AlertSeverity::SeverityUnspecified,
                "WARNING" => AlertSeverity::Warning,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AlertSeverity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AlertSeverity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AlertSeverity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INFO" => AlertSeverity::Info,
                "SEVERE" => AlertSeverity::Severe,
                "SEVERITY_UNSPECIFIED" => AlertSeverity::SeverityUnspecified,
                "WARNING" => AlertSeverity::Warning,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AlertSeverity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AlertSeverity {
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
    pub struct Cell {
        #[doc = "Value in the cell. The dimension cells contain strings, and the metric cells contain numbers."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Cell {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Cell {
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
    pub struct ContentAdsSettings {
        #[doc = "Required. Type of the ad unit."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::ContentAdsSettingsType>,
        #[doc = "Required. Size of the ad unit. e.g. \"728x90\", \"1x3\" (for responsive ad units)."]
        #[serde(
            rename = "size",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub size: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ContentAdsSettings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContentAdsSettings {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ContentAdsSettingsType {
        #[doc = "In-article ad unit."]
        Article,
        #[doc = "Display ad unit."]
        Display,
        #[doc = "In-feed ad unit."]
        Feed,
        #[doc = "Link ad unit. Note that link ad units have now been retired, see https://support.google.com/adsense/answer/9987221."]
        Link,
        #[doc = "Matched content unit."]
        MatchedContent,
        #[doc = "Unspecified ad unit type."]
        TypeUnspecified,
    }
    impl ContentAdsSettingsType {
        pub fn as_str(self) -> &'static str {
            match self {
                ContentAdsSettingsType::Article => "ARTICLE",
                ContentAdsSettingsType::Display => "DISPLAY",
                ContentAdsSettingsType::Feed => "FEED",
                ContentAdsSettingsType::Link => "LINK",
                ContentAdsSettingsType::MatchedContent => "MATCHED_CONTENT",
                ContentAdsSettingsType::TypeUnspecified => "TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ContentAdsSettingsType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ContentAdsSettingsType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ContentAdsSettingsType, ()> {
            Ok(match s {
                "ARTICLE" => ContentAdsSettingsType::Article,
                "DISPLAY" => ContentAdsSettingsType::Display,
                "FEED" => ContentAdsSettingsType::Feed,
                "LINK" => ContentAdsSettingsType::Link,
                "MATCHED_CONTENT" => ContentAdsSettingsType::MatchedContent,
                "TYPE_UNSPECIFIED" => ContentAdsSettingsType::TypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ContentAdsSettingsType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ContentAdsSettingsType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ContentAdsSettingsType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ARTICLE" => ContentAdsSettingsType::Article,
                "DISPLAY" => ContentAdsSettingsType::Display,
                "FEED" => ContentAdsSettingsType::Feed,
                "LINK" => ContentAdsSettingsType::Link,
                "MATCHED_CONTENT" => ContentAdsSettingsType::MatchedContent,
                "TYPE_UNSPECIFIED" => ContentAdsSettingsType::TypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ContentAdsSettingsType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ContentAdsSettingsType {
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
    pub struct CustomChannel {
        #[doc = "Required. Display name of the custom channel."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Resource name of the custom channel. Format: accounts/{account}/adclients/{adclient}/customchannels/{customchannel}"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. Unique ID of the custom channel as used in the `CUSTOM_CHANNEL_ID` reporting dimension."]
        #[serde(
            rename = "reportingDimensionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reporting_dimension_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CustomChannel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomChannel {
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
    pub struct Header {
        #[doc = "The [ISO-4217 currency code](https://en.wikipedia.org/wiki/ISO_4217) of this column. Only present if the header type is METRIC_CURRENCY."]
        #[serde(
            rename = "currencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency_code: ::std::option::Option<String>,
        #[doc = "Required. Name of the header."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Required. Type of the header."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::HeaderType>,
    }
    impl ::google_field_selector::FieldSelector for Header {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Header {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum HeaderType {
        #[doc = "Dimension header type."]
        Dimension,
        #[doc = "Unspecified header."]
        HeaderTypeUnspecified,
        #[doc = "Currency header type."]
        MetricCurrency,
        #[doc = "Decimal header type."]
        MetricDecimal,
        #[doc = "Milliseconds header type."]
        MetricMilliseconds,
        #[doc = "Ratio header type."]
        MetricRatio,
        #[doc = "Tally header type."]
        MetricTally,
    }
    impl HeaderType {
        pub fn as_str(self) -> &'static str {
            match self {
                HeaderType::Dimension => "DIMENSION",
                HeaderType::HeaderTypeUnspecified => "HEADER_TYPE_UNSPECIFIED",
                HeaderType::MetricCurrency => "METRIC_CURRENCY",
                HeaderType::MetricDecimal => "METRIC_DECIMAL",
                HeaderType::MetricMilliseconds => "METRIC_MILLISECONDS",
                HeaderType::MetricRatio => "METRIC_RATIO",
                HeaderType::MetricTally => "METRIC_TALLY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for HeaderType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for HeaderType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<HeaderType, ()> {
            Ok(match s {
                "DIMENSION" => HeaderType::Dimension,
                "HEADER_TYPE_UNSPECIFIED" => HeaderType::HeaderTypeUnspecified,
                "METRIC_CURRENCY" => HeaderType::MetricCurrency,
                "METRIC_DECIMAL" => HeaderType::MetricDecimal,
                "METRIC_MILLISECONDS" => HeaderType::MetricMilliseconds,
                "METRIC_RATIO" => HeaderType::MetricRatio,
                "METRIC_TALLY" => HeaderType::MetricTally,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for HeaderType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for HeaderType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for HeaderType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DIMENSION" => HeaderType::Dimension,
                "HEADER_TYPE_UNSPECIFIED" => HeaderType::HeaderTypeUnspecified,
                "METRIC_CURRENCY" => HeaderType::MetricCurrency,
                "METRIC_DECIMAL" => HeaderType::MetricDecimal,
                "METRIC_MILLISECONDS" => HeaderType::MetricMilliseconds,
                "METRIC_RATIO" => HeaderType::MetricRatio,
                "METRIC_TALLY" => HeaderType::MetricTally,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for HeaderType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HeaderType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct HttpBody {
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
    impl ::google_field_selector::FieldSelector for HttpBody {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for HttpBody {
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
    pub struct ListAccountsResponse {
        #[doc = "The accounts returned in this list response."]
        #[serde(
            rename = "accounts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub accounts: ::std::option::Option<Vec<crate::schemas::Account>>,
        #[doc = "Continuation token used to page through accounts. To retrieve the next page of the results, set the next request's \"page_token\" value to this."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListAccountsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListAccountsResponse {
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
    pub struct ListAdClientsResponse {
        #[doc = "The ad clients returned in this list response."]
        #[serde(
            rename = "adClients",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ad_clients: ::std::option::Option<Vec<crate::schemas::AdClient>>,
        #[doc = "Continuation token used to page through ad clients. To retrieve the next page of the results, set the next request's \"page_token\" value to this."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListAdClientsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListAdClientsResponse {
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
        #[doc = "The ad units returned in the list response."]
        #[serde(
            rename = "adUnits",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ad_units: ::std::option::Option<Vec<crate::schemas::AdUnit>>,
        #[doc = "Continuation token used to page through ad units. To retrieve the next page of the results, set the next request's \"page_token\" value to this."]
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
    pub struct ListAlertsResponse {
        #[doc = "The alerts returned in this list response."]
        #[serde(
            rename = "alerts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alerts: ::std::option::Option<Vec<crate::schemas::Alert>>,
    }
    impl ::google_field_selector::FieldSelector for ListAlertsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListAlertsResponse {
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
    pub struct ListChildAccountsResponse {
        #[doc = "The accounts returned in this list response."]
        #[serde(
            rename = "accounts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub accounts: ::std::option::Option<Vec<crate::schemas::Account>>,
        #[doc = "Continuation token used to page through accounts. To retrieve the next page of the results, set the next request's \"page_token\" value to this."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListChildAccountsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListChildAccountsResponse {
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
    pub struct ListCustomChannelsResponse {
        #[doc = "The custom channels returned in this list response."]
        #[serde(
            rename = "customChannels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_channels: ::std::option::Option<Vec<crate::schemas::CustomChannel>>,
        #[doc = "Continuation token used to page through alerts. To retrieve the next page of the results, set the next request's \"page_token\" value to this."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListCustomChannelsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListCustomChannelsResponse {
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
    pub struct ListLinkedAdUnitsResponse {
        #[doc = "The ad units returned in the list response."]
        #[serde(
            rename = "adUnits",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ad_units: ::std::option::Option<Vec<crate::schemas::AdUnit>>,
        #[doc = "Continuation token used to page through ad units. To retrieve the next page of the results, set the next request's \"page_token\" value to this."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListLinkedAdUnitsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListLinkedAdUnitsResponse {
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
    pub struct ListLinkedCustomChannelsResponse {
        #[doc = "The custom channels returned in this list response."]
        #[serde(
            rename = "customChannels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_channels: ::std::option::Option<Vec<crate::schemas::CustomChannel>>,
        #[doc = "Continuation token used to page through alerts. To retrieve the next page of the results, set the next request's \"page_token\" value to this."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ListLinkedCustomChannelsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListLinkedCustomChannelsResponse {
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
    pub struct ListPaymentsResponse {
        #[doc = "The payments returned in this list response."]
        #[serde(
            rename = "payments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub payments: ::std::option::Option<Vec<crate::schemas::Payment>>,
    }
    impl ::google_field_selector::FieldSelector for ListPaymentsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListPaymentsResponse {
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
    pub struct ListSavedReportsResponse {
        #[doc = "Continuation token used to page through reports. To retrieve the next page of the results, set the next request's \"page_token\" value to this."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The reports returned in this list response."]
        #[serde(
            rename = "savedReports",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub saved_reports: ::std::option::Option<Vec<crate::schemas::SavedReport>>,
    }
    impl ::google_field_selector::FieldSelector for ListSavedReportsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListSavedReportsResponse {
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
    pub struct ListSitesResponse {
        #[doc = "Continuation token used to page through sites. To retrieve the next page of the results, set the next request's \"page_token\" value to this."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The sites returned in this list response."]
        #[serde(
            rename = "sites",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sites: ::std::option::Option<Vec<crate::schemas::Site>>,
    }
    impl ::google_field_selector::FieldSelector for ListSitesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListSitesResponse {
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
    pub struct ListUrlChannelsResponse {
        #[doc = "Continuation token used to page through url channels. To retrieve the next page of the results, set the next request's \"page_token\" value to this."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The url channels returned in this list response."]
        #[serde(
            rename = "urlChannels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url_channels: ::std::option::Option<Vec<crate::schemas::UrlChannel>>,
    }
    impl ::google_field_selector::FieldSelector for ListUrlChannelsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListUrlChannelsResponse {
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
    pub struct Payment {
        #[doc = "Output only. The amount of unpaid or paid earnings, as a formatted string, including the currency. E.g. \"¥1,235 JPY\", \"$1,234.57\", \"£87.65\"."]
        #[serde(
            rename = "amount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub amount: ::std::option::Option<String>,
        #[doc = "Output only. For paid earnings, the date that the payment was credited. For unpaid earnings, this field is empty. Payment dates are always returned in the billing timezone (America/Los_Angeles)."]
        #[serde(
            rename = "date",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "Resource name of the payment. Format: - accounts/{account}/payments/unpaid for unpaid (current) AdSense earnings. - accounts/{account}/payments/youtube-unpaid for unpaid (current) YouTube earnings. - accounts/{account}/payments/yyyy-MM-dd for paid AdSense earnings. - accounts/{account}/payments/youtube-yyyy-MM-dd for paid YouTube earnings. "]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Payment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Payment {
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
    pub struct ReportResult {
        #[doc = "The averages of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty."]
        #[serde(
            rename = "averages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub averages: ::std::option::Option<crate::schemas::Row>,
        #[doc = "Required. End date of the range (inclusive)."]
        #[serde(
            rename = "endDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "The header information; one for each dimension in the request, followed by one for each metric in the request."]
        #[serde(
            rename = "headers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub headers: ::std::option::Option<Vec<crate::schemas::Header>>,
        #[doc = "The output rows of the report. Each row is a list of cells; one for each dimension in the request, followed by one for each metric in the request."]
        #[serde(
            rename = "rows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rows: ::std::option::Option<Vec<crate::schemas::Row>>,
        #[doc = "Required. Start date of the range (inclusive)."]
        #[serde(
            rename = "startDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_date: ::std::option::Option<crate::schemas::Date>,
        #[doc = "The total number of rows matched by the report request."]
        #[serde(
            rename = "totalMatchedRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_matched_rows: ::std::option::Option<i64>,
        #[doc = "The totals of the report. This is the same length as any other row in the report; cells corresponding to dimension columns are empty."]
        #[serde(
            rename = "totals",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub totals: ::std::option::Option<crate::schemas::Row>,
        #[doc = "Any warnings associated with generation of the report. These warnings are always returned in English."]
        #[serde(
            rename = "warnings",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub warnings: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for ReportResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportResult {
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
        #[doc = "Cells in the row."]
        #[serde(
            rename = "cells",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cells: ::std::option::Option<Vec<crate::schemas::Cell>>,
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
    pub struct SavedReport {
        #[doc = "Resource name of the report. Format: accounts/{account}/reports/{report}"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Report title as specified by publisher."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SavedReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SavedReport {
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
    pub struct Site {
        #[doc = "Whether auto ads is turned on for the site."]
        #[serde(
            rename = "autoAdsEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub auto_ads_enabled: ::std::option::Option<bool>,
        #[doc = "Domain (or subdomain) of the site, e.g. \"example.com\" or \"www.example.com\". This is used in the `OWNED_SITE_DOMAIN_NAME` reporting dimension."]
        #[serde(
            rename = "domain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain: ::std::option::Option<String>,
        #[doc = "Resource name of a site. Format: accounts/{account}/sites/{site}"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. Unique ID of the site as used in the `OWNED_SITE_ID` reporting dimension."]
        #[serde(
            rename = "reportingDimensionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reporting_dimension_id: ::std::option::Option<String>,
        #[doc = "Output only. State of a site."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::SiteState>,
    }
    impl ::google_field_selector::FieldSelector for Site {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Site {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SiteState {
        #[doc = "Running some checks on the site. This usually takes a few days, but in some cases can take up to 2 weeks."]
        GettingReady,
        #[doc = "Publisher needs to fix some issues before the site is ready to show ads."]
        NeedsAttention,
        #[doc = "The site is ready to show ads."]
        Ready,
        #[doc = "The site hasn't been checked yet."]
        RequiresReview,
        #[doc = "State unspecified."]
        StateUnspecified,
    }
    impl SiteState {
        pub fn as_str(self) -> &'static str {
            match self {
                SiteState::GettingReady => "GETTING_READY",
                SiteState::NeedsAttention => "NEEDS_ATTENTION",
                SiteState::Ready => "READY",
                SiteState::RequiresReview => "REQUIRES_REVIEW",
                SiteState::StateUnspecified => "STATE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for SiteState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for SiteState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<SiteState, ()> {
            Ok(match s {
                "GETTING_READY" => SiteState::GettingReady,
                "NEEDS_ATTENTION" => SiteState::NeedsAttention,
                "READY" => SiteState::Ready,
                "REQUIRES_REVIEW" => SiteState::RequiresReview,
                "STATE_UNSPECIFIED" => SiteState::StateUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for SiteState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SiteState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SiteState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "GETTING_READY" => SiteState::GettingReady,
                "NEEDS_ATTENTION" => SiteState::NeedsAttention,
                "READY" => SiteState::Ready,
                "REQUIRES_REVIEW" => SiteState::RequiresReview,
                "STATE_UNSPECIFIED" => SiteState::StateUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for SiteState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SiteState {
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
    pub struct TimeZone {
        #[doc = "IANA Time Zone Database time zone, e.g. \"America/New_York\"."]
        #[serde(
            rename = "id",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub id: ::std::option::Option<String>,
        #[doc = "Optional. IANA Time Zone Database version number, e.g. \"2019a\"."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for TimeZone {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for TimeZone {
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
    pub struct UrlChannel {
        #[doc = "Resource name of the URL channel. Format: accounts/{account}/adclients/{adclient}/urlchannels/{urlchannel}"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. Unique ID of the custom channel as used in the `URL_CHANNEL_ID` reporting dimension."]
        #[serde(
            rename = "reportingDimensionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reporting_dimension_id: ::std::option::Option<String>,
        #[doc = "URI pattern of the channel. Does not include \"http://\" or \"https://\". Example: www.example.com/home"]
        #[serde(
            rename = "uriPattern",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub uri_pattern: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UrlChannel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UrlChannel {
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
    reqwest: ::reqwest::blocking::Client,
    auth: Box<dyn ::google_api_auth::GetAccessToken>,
}
impl Client {
    pub fn new<A>(auth: A) -> Self
    where
        A: ::google_api_auth::GetAccessToken + 'static,
    {
        Client::with_reqwest_client(
            auth,
            ::reqwest::blocking::Client::builder()
                .timeout(None)
                .build()
                .unwrap(),
        )
    }
    pub fn with_reqwest_client<A>(auth: A, reqwest: ::reqwest::blocking::Client) -> Self
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
            pub(crate) reqwest: &'a reqwest::blocking::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> AccountsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Gets information about the selected AdSense account."]
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
            #[doc = "Lists all accounts available to this user."]
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
            #[doc = "Lists all accounts directly managed by the given AdSense account."]
            pub fn list_child_accounts(
                &self,
                parent: impl Into<String>,
            ) -> ListChildAccountsRequestBuilder {
                ListChildAccountsRequestBuilder {
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
            #[doc = "Actions that can be performed on the adclients resource"]
            pub fn adclients(&self) -> crate::resources::accounts::adclients::AdclientsActions {
                crate::resources::accounts::adclients::AdclientsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the alerts resource"]
            pub fn alerts(&self) -> crate::resources::accounts::alerts::AlertsActions {
                crate::resources::accounts::alerts::AlertsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the payments resource"]
            pub fn payments(&self) -> crate::resources::accounts::payments::PaymentsActions {
                crate::resources::accounts::payments::PaymentsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the reports resource"]
            pub fn reports(&self) -> crate::resources::accounts::reports::ReportsActions {
                crate::resources::accounts::reports::ReportsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the sites resource"]
            pub fn sites(&self) -> crate::resources::accounts::sites::SitesActions {
                crate::resources::accounts::sites::SitesActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [AccountsActions::get()](struct.AccountsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
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
            pub fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::Account, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(self) -> Result<crate::schemas::Account, crate::Error> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adsense.googleapis.com/".to_owned();
                output.push_str("v2/");
                {
                    let var_as_str = &self.name;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        #[doc = "Created via [AccountsActions::list()](struct.AccountsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
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
            #[doc = "The maximum number of accounts to include in the response, used for paging. If unspecified, at most 10000 accounts will be returned. The maximum value is 10000; values above 10000 will be coerced to 10000."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "A page token, received from a previous `ListAccounts` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListAccounts` must match the call that provided the page token."]
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
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_accounts<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_accounts_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_accounts_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Account> {
                self.iter_accounts_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_accounts_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Account> {
                self.iter_accounts_with_fields(Some("*"))
            }
            pub fn iter_accounts_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "accounts").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "accounts")
            }
            pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_with_fields(fields)
            }
            pub fn iter_with_default_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListAccountsResponse> {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListAccountsResponse> {
                self.iter_with_fields(Some("*"))
            }
            pub fn iter_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                if !fields.is_empty() {
                    match fields.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => fields.push_str(","),
                    }
                    fields.push_str("nextPageToken");
                    self.fields = Some(fields);
                }
                crate::iter::PageIter::new(self)
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::ListAccountsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListAccountsResponse, crate::Error> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adsense.googleapis.com/".to_owned();
                output.push_str("v2/accounts");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                self._execute()
            }
        }
        #[doc = "Created via [AccountsActions::list_child_accounts()](struct.AccountsActions.html#method.list_child_accounts)"]
        #[derive(Debug, Clone)]
        pub struct ListChildAccountsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::blocking::Client,
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
        impl<'a> ListChildAccountsRequestBuilder<'a> {
            #[doc = "The maximum number of accounts to include in the response, used for paging. If unspecified, at most 10000 accounts will be returned. The maximum value is 10000; values above 10000 will be coerced to 10000."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "A page token, received from a previous `ListAccounts` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListAccounts` must match the call that provided the page token."]
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
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_accounts<T>(self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_accounts_with_fields(fields)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_accounts_with_default_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Account> {
                self.iter_accounts_with_fields(None::<String>)
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_accounts_with_all_fields(
                self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Account> {
                self.iter_accounts_with_fields(Some("*"))
            }
            pub fn iter_accounts_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                self.fields = Some({
                    let mut selector = concat!("nextPageToken,", "accounts").to_owned();
                    let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                    if !items_fields.is_empty() {
                        selector.push_str("(");
                        selector.push_str(items_fields);
                        selector.push_str(")");
                    }
                    selector
                });
                crate::iter::PageItemIter::new(self, "accounts")
            }
            pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.iter_with_fields(fields)
            }
            pub fn iter_with_default_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListChildAccountsResponse>
            {
                self.iter_with_fields(None::<&str>)
            }
            pub fn iter_with_all_fields(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListChildAccountsResponse>
            {
                self.iter_with_fields(Some("*"))
            }
            pub fn iter_with_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> crate::iter::PageIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned,
                F: AsRef<str>,
            {
                let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                if !fields.is_empty() {
                    match fields.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => fields.push_str(","),
                    }
                    fields.push_str("nextPageToken");
                    self.fields = Some(fields);
                }
                crate::iter::PageIter::new(self)
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
            {
                let fields = ::google_field_selector::to_string::<T>();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_with_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_with_default_fields(
                self,
            ) -> Result<crate::schemas::ListChildAccountsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListChildAccountsResponse, crate::Error> {
                self.execute_with_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_with_fields<T, F>(mut self, fields: Option<F>) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path())?;
                Ok(crate::error_from_response(req.send()?)?.json()?)
            }
            fn _path(&self) -> String {
                let mut output = "https://adsense.googleapis.com/".to_owned();
                output.push_str("v2/");
                {
                    let var_as_str = &self.parent;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":listChildAccounts");
                output
            }
            fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                req = req.bearer_auth(
                    self.auth
                        .access_token()
                        .map_err(|err| crate::Error::OAuth2(err))?,
                );
                Ok(req)
            }
        }
        impl<'a> crate::iter::IterableMethod for ListChildAccountsRequestBuilder<'a> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: ::serde::de::DeserializeOwned,
            {
                self._execute()
            }
        }
        pub mod adclients {
            pub mod params {}
            pub struct AdclientsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> AdclientsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Gets the AdSense code for a given ad client. This returns what was previously known as the 'auto ad code'. This is only supported for ad clients with a product_code of AFC. For more information, see [About the AdSense code](https://support.google.com/adsense/answer/9274634)."]
                pub fn get_adcode(&self, name: impl Into<String>) -> GetAdcodeRequestBuilder {
                    GetAdcodeRequestBuilder {
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
                #[doc = "Lists all the ad clients available in an account."]
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
                #[doc = "Actions that can be performed on the adunits resource"]
                pub fn adunits(
                    &self,
                ) -> crate::resources::accounts::adclients::adunits::AdunitsActions
                {
                    crate::resources::accounts::adclients::adunits::AdunitsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
                #[doc = "Actions that can be performed on the customchannels resource"]
                pub fn customchannels(
                    &self,
                ) -> crate::resources::accounts::adclients::customchannels::CustomchannelsActions
                {
                    crate::resources::accounts::adclients::customchannels::CustomchannelsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
                #[doc = "Actions that can be performed on the urlchannels resource"]
                pub fn urlchannels(
                    &self,
                ) -> crate::resources::accounts::adclients::urlchannels::UrlchannelsActions
                {
                    crate::resources::accounts::adclients::urlchannels::UrlchannelsActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            #[doc = "Created via [AdclientsActions::get_adcode()](struct.AdclientsActions.html#method.get_adcode)"]
            #[derive(Debug, Clone)]
            pub struct GetAdcodeRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
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
            impl<'a> GetAdcodeRequestBuilder<'a> {
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::AdClientAdCode, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::AdClientAdCode, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adsense.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/adcode");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [AdclientsActions::list()](struct.AdclientsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
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
                #[doc = "The maximum number of ad clients to include in the response, used for paging. If unspecified, at most 10000 ad clients will be returned. The maximum value is 10000; values above 10000 will be coerced to 10000."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A page token, received from a previous `ListAdClients` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListAdClients` must match the call that provided the page token."]
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
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                #[doc = r" populated fields in the yielded items will be determined by the"]
                #[doc = r" `FieldSelector` implementation."]
                pub fn iter_ad_clients<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_ad_clients_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_ad_clients_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::AdClient> {
                    self.iter_ad_clients_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_ad_clients_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::AdClient> {
                    self.iter_ad_clients_with_fields(Some("*"))
                }
                pub fn iter_ad_clients_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "adClients").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "adClients")
                }
                pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_with_fields(fields)
                }
                pub fn iter_with_default_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListAdClientsResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListAdClientsResponse>
                {
                    self.iter_with_fields(Some("*"))
                }
                pub fn iter_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                    if !fields.is_empty() {
                        match fields.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => fields.push_str(","),
                        }
                        fields.push_str("nextPageToken");
                        self.fields = Some(fields);
                    }
                    crate::iter::PageIter::new(self)
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::ListAdClientsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListAdClientsResponse, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adsense.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/adclients");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    self._execute()
                }
            }
            pub mod adunits {
                pub mod params {}
                pub struct AdunitsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> AdunitsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Gets an ad unit from a specified account and ad client."]
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
                    #[doc = "Gets the AdSense code for a given ad unit."]
                    pub fn get_adcode(&self, name: impl Into<String>) -> GetAdcodeRequestBuilder {
                        GetAdcodeRequestBuilder {
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
                    #[doc = "Lists all ad units under a specified account and ad client."]
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
                    #[doc = "Lists all the custom channels available for an ad unit."]
                    pub fn list_linked_custom_channels(
                        &self,
                        parent: impl Into<String>,
                    ) -> ListLinkedCustomChannelsRequestBuilder {
                        ListLinkedCustomChannelsRequestBuilder {
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
                #[doc = "Created via [AdunitsActions::get()](struct.AdunitsActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
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
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::AdUnit, crate::Error> {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::AdUnit, crate::Error> {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://adsense.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
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
                        req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[doc = "Created via [AdunitsActions::get_adcode()](struct.AdunitsActions.html#method.get_adcode)"]
                #[derive(Debug, Clone)]
                pub struct GetAdcodeRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
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
                impl<'a> GetAdcodeRequestBuilder<'a> {
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
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::AdUnitAdCode, crate::Error> {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::AdUnitAdCode, crate::Error> {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://adsense.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/adcode");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
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
                        req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[doc = "Created via [AdunitsActions::list()](struct.AdunitsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
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
                    #[doc = "The maximum number of ad units to include in the response, used for paging. If unspecified, at most 10000 ad units will be returned. The maximum value is 10000; values above 10000 will be coerced to 10000."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A page token, received from a previous `ListAdUnits` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListAdUnits` must match the call that provided the page token."]
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
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                    #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                    #[doc = r" populated fields in the yielded items will be determined by the"]
                    #[doc = r" `FieldSelector` implementation."]
                    pub fn iter_ad_units<T>(self) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_ad_units_with_fields(fields)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_ad_units_with_default_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::AdUnit>
                    {
                        self.iter_ad_units_with_fields(None::<String>)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_ad_units_with_all_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::AdUnit>
                    {
                        self.iter_ad_units_with_fields(Some("*"))
                    }
                    pub fn iter_ad_units_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        self.fields = Some({
                            let mut selector = concat!("nextPageToken,", "adUnits").to_owned();
                            let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                            if !items_fields.is_empty() {
                                selector.push_str("(");
                                selector.push_str(items_fields);
                                selector.push_str(")");
                            }
                            selector
                        });
                        crate::iter::PageItemIter::new(self, "adUnits")
                    }
                    pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_with_fields(fields)
                    }
                    pub fn iter_with_default_fields(
                        self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListAdUnitsResponse>
                    {
                        self.iter_with_fields(None::<&str>)
                    }
                    pub fn iter_with_all_fields(
                        self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListAdUnitsResponse>
                    {
                        self.iter_with_fields(Some("*"))
                    }
                    pub fn iter_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        let mut fields =
                            fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                        if !fields.is_empty() {
                            match fields.chars().rev().nth(0) {
                                Some(',') | None => {}
                                _ => fields.push_str(","),
                            }
                            fields.push_str("nextPageToken");
                            self.fields = Some(fields);
                        }
                        crate::iter::PageIter::new(self)
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::ListAdUnitsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListAdUnitsResponse, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://adsense.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/adunits");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
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
                        req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        self._execute()
                    }
                }
                #[doc = "Created via [AdunitsActions::list_linked_custom_channels()](struct.AdunitsActions.html#method.list_linked_custom_channels)"]
                #[derive(Debug, Clone)]
                pub struct ListLinkedCustomChannelsRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
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
                impl<'a> ListLinkedCustomChannelsRequestBuilder<'a> {
                    #[doc = "The maximum number of custom channels to include in the response, used for paging. If unspecified, at most 10000 custom channels will be returned. The maximum value is 10000; values above 10000 will be coerced to 10000."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A page token, received from a previous `ListLinkedCustomChannels` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListLinkedCustomChannels` must match the call that provided the page token."]
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
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                    #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                    #[doc = r" populated fields in the yielded items will be determined by the"]
                    #[doc = r" `FieldSelector` implementation."]
                    pub fn iter_custom_channels<T>(self) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_custom_channels_with_fields(fields)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_custom_channels_with_default_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::CustomChannel>
                    {
                        self.iter_custom_channels_with_fields(None::<String>)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_custom_channels_with_all_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::CustomChannel>
                    {
                        self.iter_custom_channels_with_fields(Some("*"))
                    }
                    pub fn iter_custom_channels_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        self.fields = Some({
                            let mut selector =
                                concat!("nextPageToken,", "customChannels").to_owned();
                            let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                            if !items_fields.is_empty() {
                                selector.push_str("(");
                                selector.push_str(items_fields);
                                selector.push_str(")");
                            }
                            selector
                        });
                        crate::iter::PageItemIter::new(self, "customChannels")
                    }
                    pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_with_fields(fields)
                    }
                    pub fn iter_with_default_fields(
                        self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListLinkedCustomChannelsResponse>
                    {
                        self.iter_with_fields(None::<&str>)
                    }
                    pub fn iter_with_all_fields(
                        self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListLinkedCustomChannelsResponse>
                    {
                        self.iter_with_fields(Some("*"))
                    }
                    pub fn iter_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        let mut fields =
                            fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                        if !fields.is_empty() {
                            match fields.chars().rev().nth(0) {
                                Some(',') | None => {}
                                _ => fields.push_str(","),
                            }
                            fields.push_str("nextPageToken");
                            self.fields = Some(fields);
                        }
                        crate::iter::PageIter::new(self)
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::ListLinkedCustomChannelsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListLinkedCustomChannelsResponse, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://adsense.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":listLinkedCustomChannels");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
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
                        req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                impl<'a> crate::iter::IterableMethod for ListLinkedCustomChannelsRequestBuilder<'a> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        self._execute()
                    }
                }
            }
            pub mod customchannels {
                pub mod params {}
                pub struct CustomchannelsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> CustomchannelsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Gets information about the selected custom channel."]
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
                    #[doc = "Lists all the custom channels available in an ad client."]
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
                    #[doc = "Lists all the ad units available for a custom channel."]
                    pub fn list_linked_ad_units(
                        &self,
                        parent: impl Into<String>,
                    ) -> ListLinkedAdUnitsRequestBuilder {
                        ListLinkedAdUnitsRequestBuilder {
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
                #[doc = "Created via [CustomchannelsActions::get()](struct.CustomchannelsActions.html#method.get)"]
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
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
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::CustomChannel, crate::Error> {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::CustomChannel, crate::Error> {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://adsense.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
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
                        req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[doc = "Created via [CustomchannelsActions::list()](struct.CustomchannelsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
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
                    #[doc = "The maximum number of custom channels to include in the response, used for paging. If unspecified, at most 10000 custom channels will be returned. The maximum value is 10000; values above 10000 will be coerced to 10000."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A page token, received from a previous `ListCustomChannels` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListCustomChannels` must match the call that provided the page token."]
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
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                    #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                    #[doc = r" populated fields in the yielded items will be determined by the"]
                    #[doc = r" `FieldSelector` implementation."]
                    pub fn iter_custom_channels<T>(self) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_custom_channels_with_fields(fields)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_custom_channels_with_default_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::CustomChannel>
                    {
                        self.iter_custom_channels_with_fields(None::<String>)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_custom_channels_with_all_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::CustomChannel>
                    {
                        self.iter_custom_channels_with_fields(Some("*"))
                    }
                    pub fn iter_custom_channels_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        self.fields = Some({
                            let mut selector =
                                concat!("nextPageToken,", "customChannels").to_owned();
                            let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                            if !items_fields.is_empty() {
                                selector.push_str("(");
                                selector.push_str(items_fields);
                                selector.push_str(")");
                            }
                            selector
                        });
                        crate::iter::PageItemIter::new(self, "customChannels")
                    }
                    pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_with_fields(fields)
                    }
                    pub fn iter_with_default_fields(
                        self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListCustomChannelsResponse>
                    {
                        self.iter_with_fields(None::<&str>)
                    }
                    pub fn iter_with_all_fields(
                        self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListCustomChannelsResponse>
                    {
                        self.iter_with_fields(Some("*"))
                    }
                    pub fn iter_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        let mut fields =
                            fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                        if !fields.is_empty() {
                            match fields.chars().rev().nth(0) {
                                Some(',') | None => {}
                                _ => fields.push_str(","),
                            }
                            fields.push_str("nextPageToken");
                            self.fields = Some(fields);
                        }
                        crate::iter::PageIter::new(self)
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::ListCustomChannelsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListCustomChannelsResponse, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://adsense.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/customchannels");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
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
                        req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        self._execute()
                    }
                }
                #[doc = "Created via [CustomchannelsActions::list_linked_ad_units()](struct.CustomchannelsActions.html#method.list_linked_ad_units)"]
                #[derive(Debug, Clone)]
                pub struct ListLinkedAdUnitsRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
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
                impl<'a> ListLinkedAdUnitsRequestBuilder<'a> {
                    #[doc = "The maximum number of ad units to include in the response, used for paging. If unspecified, at most 10000 ad units will be returned. The maximum value is 10000; values above 10000 will be coerced to 10000."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A page token, received from a previous `ListLinkedAdUnits` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListLinkedAdUnits` must match the call that provided the page token."]
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
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                    #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                    #[doc = r" populated fields in the yielded items will be determined by the"]
                    #[doc = r" `FieldSelector` implementation."]
                    pub fn iter_ad_units<T>(self) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_ad_units_with_fields(fields)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_ad_units_with_default_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::AdUnit>
                    {
                        self.iter_ad_units_with_fields(None::<String>)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_ad_units_with_all_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::AdUnit>
                    {
                        self.iter_ad_units_with_fields(Some("*"))
                    }
                    pub fn iter_ad_units_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        self.fields = Some({
                            let mut selector = concat!("nextPageToken,", "adUnits").to_owned();
                            let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                            if !items_fields.is_empty() {
                                selector.push_str("(");
                                selector.push_str(items_fields);
                                selector.push_str(")");
                            }
                            selector
                        });
                        crate::iter::PageItemIter::new(self, "adUnits")
                    }
                    pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_with_fields(fields)
                    }
                    pub fn iter_with_default_fields(
                        self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListLinkedAdUnitsResponse>
                    {
                        self.iter_with_fields(None::<&str>)
                    }
                    pub fn iter_with_all_fields(
                        self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListLinkedAdUnitsResponse>
                    {
                        self.iter_with_fields(Some("*"))
                    }
                    pub fn iter_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        let mut fields =
                            fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                        if !fields.is_empty() {
                            match fields.chars().rev().nth(0) {
                                Some(',') | None => {}
                                _ => fields.push_str(","),
                            }
                            fields.push_str("nextPageToken");
                            self.fields = Some(fields);
                        }
                        crate::iter::PageIter::new(self)
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::ListLinkedAdUnitsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListLinkedAdUnitsResponse, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://adsense.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str(":listLinkedAdUnits");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
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
                        req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                impl<'a> crate::iter::IterableMethod for ListLinkedAdUnitsRequestBuilder<'a> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        self._execute()
                    }
                }
            }
            pub mod urlchannels {
                pub mod params {}
                pub struct UrlchannelsActions<'a> {
                    pub(crate) reqwest: &'a reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> UrlchannelsActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Lists active url channels."]
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
                #[doc = "Created via [UrlchannelsActions::list()](struct.UrlchannelsActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
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
                    #[doc = "The maximum number of url channels to include in the response, used for paging. If unspecified, at most 10000 url channels will be returned. The maximum value is 10000; values above 10000 will be coerced to 10000."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A page token, received from a previous `ListUrlChannels` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListUrlChannels` must match the call that provided the page token."]
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
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                    #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                    #[doc = r" populated fields in the yielded items will be determined by the"]
                    #[doc = r" `FieldSelector` implementation."]
                    pub fn iter_url_channels<T>(self) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_url_channels_with_fields(fields)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_url_channels_with_default_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::UrlChannel>
                    {
                        self.iter_url_channels_with_fields(None::<String>)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_url_channels_with_all_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::UrlChannel>
                    {
                        self.iter_url_channels_with_fields(Some("*"))
                    }
                    pub fn iter_url_channels_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        self.fields = Some({
                            let mut selector = concat!("nextPageToken,", "urlChannels").to_owned();
                            let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                            if !items_fields.is_empty() {
                                selector.push_str("(");
                                selector.push_str(items_fields);
                                selector.push_str(")");
                            }
                            selector
                        });
                        crate::iter::PageItemIter::new(self, "urlChannels")
                    }
                    pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_with_fields(fields)
                    }
                    pub fn iter_with_default_fields(
                        self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListUrlChannelsResponse>
                    {
                        self.iter_with_fields(None::<&str>)
                    }
                    pub fn iter_with_all_fields(
                        self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListUrlChannelsResponse>
                    {
                        self.iter_with_fields(Some("*"))
                    }
                    pub fn iter_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        let mut fields =
                            fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                        if !fields.is_empty() {
                            match fields.chars().rev().nth(0) {
                                Some(',') | None => {}
                                _ => fields.push_str(","),
                            }
                            fields.push_str("nextPageToken");
                            self.fields = Some(fields);
                        }
                        crate::iter::PageIter::new(self)
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::ListUrlChannelsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListUrlChannelsResponse, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://adsense.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/urlchannels");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
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
                        req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        self._execute()
                    }
                }
            }
        }
        pub mod alerts {
            pub mod params {}
            pub struct AlertsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> AlertsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Lists all the alerts available in an account."]
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
                        language_code: None,
                    }
                }
            }
            #[doc = "Created via [AlertsActions::list()](struct.AlertsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                language_code: Option<String>,
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
                #[doc = "The language to use for translating alert messages. If unspecified, this defaults to the user's display language. If the given language is not supported, alerts will be returned in English. The language is specified as an [IETF BCP-47 language code](https://en.wikipedia.org/wiki/IETF_language_tag)."]
                pub fn language_code(mut self, value: impl Into<String>) -> Self {
                    self.language_code = Some(value.into());
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::ListAlertsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListAlertsResponse, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adsense.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/alerts");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("languageCode", &self.language_code)]);
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
        }
        pub mod payments {
            pub mod params {}
            pub struct PaymentsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> PaymentsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Lists all the payments available for an account."]
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
                    }
                }
            }
            #[doc = "Created via [PaymentsActions::list()](struct.PaymentsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
            impl<'a> ListRequestBuilder<'a> {
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::ListPaymentsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListPaymentsResponse, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adsense.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/payments");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
        }
        pub mod reports {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum GenerateDateRange {
                    #[doc = "A custom date range specified using the `start_date` and `end_date` fields. This is the default if no ReportingDateRange is provided."]
                    Custom,
                    #[doc = "Last 30 days, excluding current day."]
                    Last30Days,
                    #[doc = "Last 7 days, excluding current day."]
                    Last7Days,
                    #[doc = "From the start of the current month to the current day. e.g. if the current date is 2020-03-12 then the range will be [2020-03-01, 2020-03-12]."]
                    MonthToDate,
                    #[doc = "Unspecified date range."]
                    ReportingDateRangeUnspecified,
                    #[doc = "Current day."]
                    Today,
                    #[doc = "From the start of the current year to the current day. e.g. if the current date is 2020-03-12 then the range will be [2020-01-01, 2020-03-12]."]
                    YearToDate,
                    #[doc = "Yesterday."]
                    Yesterday,
                }
                impl GenerateDateRange {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            GenerateDateRange::Custom => "CUSTOM",
                            GenerateDateRange::Last30Days => "LAST_30_DAYS",
                            GenerateDateRange::Last7Days => "LAST_7_DAYS",
                            GenerateDateRange::MonthToDate => "MONTH_TO_DATE",
                            GenerateDateRange::ReportingDateRangeUnspecified => {
                                "REPORTING_DATE_RANGE_UNSPECIFIED"
                            }
                            GenerateDateRange::Today => "TODAY",
                            GenerateDateRange::YearToDate => "YEAR_TO_DATE",
                            GenerateDateRange::Yesterday => "YESTERDAY",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for GenerateDateRange {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for GenerateDateRange {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<GenerateDateRange, ()> {
                        Ok(match s {
                            "CUSTOM" => GenerateDateRange::Custom,
                            "LAST_30_DAYS" => GenerateDateRange::Last30Days,
                            "LAST_7_DAYS" => GenerateDateRange::Last7Days,
                            "MONTH_TO_DATE" => GenerateDateRange::MonthToDate,
                            "REPORTING_DATE_RANGE_UNSPECIFIED" => {
                                GenerateDateRange::ReportingDateRangeUnspecified
                            }
                            "TODAY" => GenerateDateRange::Today,
                            "YEAR_TO_DATE" => GenerateDateRange::YearToDate,
                            "YESTERDAY" => GenerateDateRange::Yesterday,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for GenerateDateRange {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for GenerateDateRange {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for GenerateDateRange {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "CUSTOM" => GenerateDateRange::Custom,
                            "LAST_30_DAYS" => GenerateDateRange::Last30Days,
                            "LAST_7_DAYS" => GenerateDateRange::Last7Days,
                            "MONTH_TO_DATE" => GenerateDateRange::MonthToDate,
                            "REPORTING_DATE_RANGE_UNSPECIFIED" => {
                                GenerateDateRange::ReportingDateRangeUnspecified
                            }
                            "TODAY" => GenerateDateRange::Today,
                            "YEAR_TO_DATE" => GenerateDateRange::YearToDate,
                            "YESTERDAY" => GenerateDateRange::Yesterday,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for GenerateDateRange {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for GenerateDateRange {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum GenerateDimensionsItems {
                    #[doc = "Account name. The members of this dimension match the values from Account.display_name."]
                    AccountName,
                    #[doc = "Unique ID of an ad client. The members of this dimension match the values from AdClient.reporting_dimension_id."]
                    AdClientId,
                    #[doc = "Ad format code indicating the way an ad is shown to the users on your site (e.g. \"ON_PAGE\", \"ANCHOR\", \"INTERSTITIAL\")."]
                    AdFormatCode,
                    #[doc = "Localized ad format name indicating the way an ad is shown to the users on your site (e.g. \"In-page\", \"Anchor\", \"Vignette\")."]
                    AdFormatName,
                    #[doc = "Ad placement code (e.g. \"AD_UNIT\", \"ca-pub-123456:78910\", \"OTHER\")."]
                    AdPlacementCode,
                    #[doc = "Localized ad placement name (e.g. \"Ad unit\", \"Global settings\", \"Manual\")."]
                    AdPlacementName,
                    #[doc = "Unique ID of an ad unit (within which an ad was served). The members of this dimension match the values from AdUnit.reporting_dimension_id."]
                    AdUnitId,
                    #[doc = "Ad unit name (within which an ad was served). The members of this dimension match the values from AdUnit.display_name."]
                    AdUnitName,
                    #[doc = "The size code of an ad unit (e.g. \"728x90\", \"responsive\")."]
                    AdUnitSizeCode,
                    #[doc = "Localized size of an ad unit (e.g. \"728x90\", \"Responsive\")."]
                    AdUnitSizeName,
                    #[doc = "Type of a bid (e.g. \"cpc\", \"cpm\") for a served ad."]
                    BidTypeCode,
                    #[doc = "Localized bid type name (e.g. \"CPC bids\", \"CPM bids\") for a served ad."]
                    BidTypeName,
                    #[doc = "Unique (opaque) ID of an ad network that returned the winning ads for an ad request."]
                    BuyerNetworkId,
                    #[doc = "Name of an ad network that returned the winning ads for an ad request (e.g. \"Google AdWords\"). Note that unlike other \"NAME\" dimensions, the members of this dimensions are not localized."]
                    BuyerNetworkName,
                    #[doc = "Content platform code an ad request was made from (e.g. \"AMP\", \"HTML\")."]
                    ContentPlatformCode,
                    #[doc = "Localized content platform name an ad request was made from (e.g. \"AMP\", \"Web\")."]
                    ContentPlatformName,
                    #[doc = "CLDR region code of a user viewing an ad (e.g. \"US\", \"FR\")."]
                    CountryCode,
                    #[doc = "Localized region name of a user viewing an ad (e.g. \"United States\", \"France\")."]
                    CountryName,
                    #[doc = "Creative size code (e.g. \"728x90\", \"dynamic\") of a served ad."]
                    CreativeSizeCode,
                    #[doc = "Localized creative size name (e.g. \"728x90\", \"Dynamic\") of a served ad."]
                    CreativeSizeName,
                    #[doc = "Unique ID of a custom channel. The members of this dimension match the values from CustomChannel.reporting_dimension_id."]
                    CustomChannelId,
                    #[doc = "Custom channel name. The members of this dimension match the values from CustomChannel.display_name."]
                    CustomChannelName,
                    #[doc = "Custom search style id."]
                    CustomSearchStyleId,
                    #[doc = "Custom search style name."]
                    CustomSearchStyleName,
                    #[doc = "Date dimension in YYYY-MM-DD format (e.g. \"2010-02-10\")."]
                    Date,
                    #[doc = "Unspecified dimension."]
                    DimensionUnspecified,
                    #[doc = "Name of a host on which an ad was served (e.g. \"www.google.com\", \"webcaches\", \"xn--bcher-kva.example\")."]
                    DomainCode,
                    #[doc = "Localized name of a host on which an ad was served, after IDNA decoding (e.g. \"www.google.com\", \"Web caches and other\", \"bücher.example\")."]
                    DomainName,
                    #[doc = "Domain registrants."]
                    DomainRegistrant,
                    #[doc = "Month dimension in YYYY-MM format (e.g. \"2010-02\")."]
                    Month,
                    #[doc = "Domain name of a verified site (e.g. \"example.com\"). The members of this dimension match the values from Site.domain."]
                    OwnedSiteDomainName,
                    #[doc = "Unique ID of a verified site. The members of this dimension match the values from Site.reporting_dimension_id."]
                    OwnedSiteId,
                    #[doc = "Platform type code (e.g. \"HighEndMobile\", \"Desktop\")."]
                    PlatformTypeCode,
                    #[doc = "Localized platform type name (e.g. \"High-end mobile devices\", \"Desktop\")."]
                    PlatformTypeName,
                    #[doc = "Product code (e.g. \"AFC\", \"AFS\"). The members of this dimension match the values from AdClient.product_code."]
                    ProductCode,
                    #[doc = "Localized product name (e.g. \"AdSense for Content\", \"AdSense for Search\")."]
                    ProductName,
                    #[doc = "Requested ad type code (e.g. \"IMAGE\", \"RADLINK\", \"OTHER\")."]
                    RequestedAdTypeCode,
                    #[doc = "Localized requested ad type name (e.g. \"Display\", \"Link unit\", \"Other\")."]
                    RequestedAdTypeName,
                    #[doc = "Served ad type code (e.g. \"IMAGE\", \"RADLINK\", \"OTHER\")."]
                    ServedAdTypeCode,
                    #[doc = "Localized served ad type name (e.g. \"Display\", \"Link unit\", \"Other\")."]
                    ServedAdTypeName,
                    #[doc = "Targeting type code (e.g. \"Keyword\", \"UserInterest\", \"RunOfNetwork\")."]
                    TargetingTypeCode,
                    #[doc = "Localized targeting type name (e.g. \"Contextual\", \"Personalized\", \"Run of Network\")."]
                    TargetingTypeName,
                    #[doc = "Unique ID of a URL channel. The members of this dimension match the values from UrlChannel.reporting_dimension_id."]
                    UrlChannelId,
                    #[doc = "Name of a URL channel. The members of this dimension match the values from UrlChannel.uri_pattern."]
                    UrlChannelName,
                    #[doc = "Query strings for web searches."]
                    WebsearchQueryString,
                    #[doc = "Week dimension in YYYY-MM-DD format, representing the first day of each week (e.g. \"2010-02-08\"). The first day of the week is determined by the language_code specified in a report generation request (so e.g. this would be a Monday for \"en-GB\" or \"es\", but a Sunday for \"en\" or \"fr-CA\")."]
                    Week,
                }
                impl GenerateDimensionsItems {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            GenerateDimensionsItems::AccountName => "ACCOUNT_NAME",
                            GenerateDimensionsItems::AdClientId => "AD_CLIENT_ID",
                            GenerateDimensionsItems::AdFormatCode => "AD_FORMAT_CODE",
                            GenerateDimensionsItems::AdFormatName => "AD_FORMAT_NAME",
                            GenerateDimensionsItems::AdPlacementCode => "AD_PLACEMENT_CODE",
                            GenerateDimensionsItems::AdPlacementName => "AD_PLACEMENT_NAME",
                            GenerateDimensionsItems::AdUnitId => "AD_UNIT_ID",
                            GenerateDimensionsItems::AdUnitName => "AD_UNIT_NAME",
                            GenerateDimensionsItems::AdUnitSizeCode => "AD_UNIT_SIZE_CODE",
                            GenerateDimensionsItems::AdUnitSizeName => "AD_UNIT_SIZE_NAME",
                            GenerateDimensionsItems::BidTypeCode => "BID_TYPE_CODE",
                            GenerateDimensionsItems::BidTypeName => "BID_TYPE_NAME",
                            GenerateDimensionsItems::BuyerNetworkId => "BUYER_NETWORK_ID",
                            GenerateDimensionsItems::BuyerNetworkName => "BUYER_NETWORK_NAME",
                            GenerateDimensionsItems::ContentPlatformCode => "CONTENT_PLATFORM_CODE",
                            GenerateDimensionsItems::ContentPlatformName => "CONTENT_PLATFORM_NAME",
                            GenerateDimensionsItems::CountryCode => "COUNTRY_CODE",
                            GenerateDimensionsItems::CountryName => "COUNTRY_NAME",
                            GenerateDimensionsItems::CreativeSizeCode => "CREATIVE_SIZE_CODE",
                            GenerateDimensionsItems::CreativeSizeName => "CREATIVE_SIZE_NAME",
                            GenerateDimensionsItems::CustomChannelId => "CUSTOM_CHANNEL_ID",
                            GenerateDimensionsItems::CustomChannelName => "CUSTOM_CHANNEL_NAME",
                            GenerateDimensionsItems::CustomSearchStyleId => {
                                "CUSTOM_SEARCH_STYLE_ID"
                            }
                            GenerateDimensionsItems::CustomSearchStyleName => {
                                "CUSTOM_SEARCH_STYLE_NAME"
                            }
                            GenerateDimensionsItems::Date => "DATE",
                            GenerateDimensionsItems::DimensionUnspecified => {
                                "DIMENSION_UNSPECIFIED"
                            }
                            GenerateDimensionsItems::DomainCode => "DOMAIN_CODE",
                            GenerateDimensionsItems::DomainName => "DOMAIN_NAME",
                            GenerateDimensionsItems::DomainRegistrant => "DOMAIN_REGISTRANT",
                            GenerateDimensionsItems::Month => "MONTH",
                            GenerateDimensionsItems::OwnedSiteDomainName => {
                                "OWNED_SITE_DOMAIN_NAME"
                            }
                            GenerateDimensionsItems::OwnedSiteId => "OWNED_SITE_ID",
                            GenerateDimensionsItems::PlatformTypeCode => "PLATFORM_TYPE_CODE",
                            GenerateDimensionsItems::PlatformTypeName => "PLATFORM_TYPE_NAME",
                            GenerateDimensionsItems::ProductCode => "PRODUCT_CODE",
                            GenerateDimensionsItems::ProductName => "PRODUCT_NAME",
                            GenerateDimensionsItems::RequestedAdTypeCode => {
                                "REQUESTED_AD_TYPE_CODE"
                            }
                            GenerateDimensionsItems::RequestedAdTypeName => {
                                "REQUESTED_AD_TYPE_NAME"
                            }
                            GenerateDimensionsItems::ServedAdTypeCode => "SERVED_AD_TYPE_CODE",
                            GenerateDimensionsItems::ServedAdTypeName => "SERVED_AD_TYPE_NAME",
                            GenerateDimensionsItems::TargetingTypeCode => "TARGETING_TYPE_CODE",
                            GenerateDimensionsItems::TargetingTypeName => "TARGETING_TYPE_NAME",
                            GenerateDimensionsItems::UrlChannelId => "URL_CHANNEL_ID",
                            GenerateDimensionsItems::UrlChannelName => "URL_CHANNEL_NAME",
                            GenerateDimensionsItems::WebsearchQueryString => {
                                "WEBSEARCH_QUERY_STRING"
                            }
                            GenerateDimensionsItems::Week => "WEEK",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for GenerateDimensionsItems {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for GenerateDimensionsItems {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<GenerateDimensionsItems, ()> {
                        Ok(match s {
                            "ACCOUNT_NAME" => GenerateDimensionsItems::AccountName,
                            "AD_CLIENT_ID" => GenerateDimensionsItems::AdClientId,
                            "AD_FORMAT_CODE" => GenerateDimensionsItems::AdFormatCode,
                            "AD_FORMAT_NAME" => GenerateDimensionsItems::AdFormatName,
                            "AD_PLACEMENT_CODE" => GenerateDimensionsItems::AdPlacementCode,
                            "AD_PLACEMENT_NAME" => GenerateDimensionsItems::AdPlacementName,
                            "AD_UNIT_ID" => GenerateDimensionsItems::AdUnitId,
                            "AD_UNIT_NAME" => GenerateDimensionsItems::AdUnitName,
                            "AD_UNIT_SIZE_CODE" => GenerateDimensionsItems::AdUnitSizeCode,
                            "AD_UNIT_SIZE_NAME" => GenerateDimensionsItems::AdUnitSizeName,
                            "BID_TYPE_CODE" => GenerateDimensionsItems::BidTypeCode,
                            "BID_TYPE_NAME" => GenerateDimensionsItems::BidTypeName,
                            "BUYER_NETWORK_ID" => GenerateDimensionsItems::BuyerNetworkId,
                            "BUYER_NETWORK_NAME" => GenerateDimensionsItems::BuyerNetworkName,
                            "CONTENT_PLATFORM_CODE" => GenerateDimensionsItems::ContentPlatformCode,
                            "CONTENT_PLATFORM_NAME" => GenerateDimensionsItems::ContentPlatformName,
                            "COUNTRY_CODE" => GenerateDimensionsItems::CountryCode,
                            "COUNTRY_NAME" => GenerateDimensionsItems::CountryName,
                            "CREATIVE_SIZE_CODE" => GenerateDimensionsItems::CreativeSizeCode,
                            "CREATIVE_SIZE_NAME" => GenerateDimensionsItems::CreativeSizeName,
                            "CUSTOM_CHANNEL_ID" => GenerateDimensionsItems::CustomChannelId,
                            "CUSTOM_CHANNEL_NAME" => GenerateDimensionsItems::CustomChannelName,
                            "CUSTOM_SEARCH_STYLE_ID" => {
                                GenerateDimensionsItems::CustomSearchStyleId
                            }
                            "CUSTOM_SEARCH_STYLE_NAME" => {
                                GenerateDimensionsItems::CustomSearchStyleName
                            }
                            "DATE" => GenerateDimensionsItems::Date,
                            "DIMENSION_UNSPECIFIED" => {
                                GenerateDimensionsItems::DimensionUnspecified
                            }
                            "DOMAIN_CODE" => GenerateDimensionsItems::DomainCode,
                            "DOMAIN_NAME" => GenerateDimensionsItems::DomainName,
                            "DOMAIN_REGISTRANT" => GenerateDimensionsItems::DomainRegistrant,
                            "MONTH" => GenerateDimensionsItems::Month,
                            "OWNED_SITE_DOMAIN_NAME" => {
                                GenerateDimensionsItems::OwnedSiteDomainName
                            }
                            "OWNED_SITE_ID" => GenerateDimensionsItems::OwnedSiteId,
                            "PLATFORM_TYPE_CODE" => GenerateDimensionsItems::PlatformTypeCode,
                            "PLATFORM_TYPE_NAME" => GenerateDimensionsItems::PlatformTypeName,
                            "PRODUCT_CODE" => GenerateDimensionsItems::ProductCode,
                            "PRODUCT_NAME" => GenerateDimensionsItems::ProductName,
                            "REQUESTED_AD_TYPE_CODE" => {
                                GenerateDimensionsItems::RequestedAdTypeCode
                            }
                            "REQUESTED_AD_TYPE_NAME" => {
                                GenerateDimensionsItems::RequestedAdTypeName
                            }
                            "SERVED_AD_TYPE_CODE" => GenerateDimensionsItems::ServedAdTypeCode,
                            "SERVED_AD_TYPE_NAME" => GenerateDimensionsItems::ServedAdTypeName,
                            "TARGETING_TYPE_CODE" => GenerateDimensionsItems::TargetingTypeCode,
                            "TARGETING_TYPE_NAME" => GenerateDimensionsItems::TargetingTypeName,
                            "URL_CHANNEL_ID" => GenerateDimensionsItems::UrlChannelId,
                            "URL_CHANNEL_NAME" => GenerateDimensionsItems::UrlChannelName,
                            "WEBSEARCH_QUERY_STRING" => {
                                GenerateDimensionsItems::WebsearchQueryString
                            }
                            "WEEK" => GenerateDimensionsItems::Week,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for GenerateDimensionsItems {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for GenerateDimensionsItems {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for GenerateDimensionsItems {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "ACCOUNT_NAME" => GenerateDimensionsItems::AccountName,
                            "AD_CLIENT_ID" => GenerateDimensionsItems::AdClientId,
                            "AD_FORMAT_CODE" => GenerateDimensionsItems::AdFormatCode,
                            "AD_FORMAT_NAME" => GenerateDimensionsItems::AdFormatName,
                            "AD_PLACEMENT_CODE" => GenerateDimensionsItems::AdPlacementCode,
                            "AD_PLACEMENT_NAME" => GenerateDimensionsItems::AdPlacementName,
                            "AD_UNIT_ID" => GenerateDimensionsItems::AdUnitId,
                            "AD_UNIT_NAME" => GenerateDimensionsItems::AdUnitName,
                            "AD_UNIT_SIZE_CODE" => GenerateDimensionsItems::AdUnitSizeCode,
                            "AD_UNIT_SIZE_NAME" => GenerateDimensionsItems::AdUnitSizeName,
                            "BID_TYPE_CODE" => GenerateDimensionsItems::BidTypeCode,
                            "BID_TYPE_NAME" => GenerateDimensionsItems::BidTypeName,
                            "BUYER_NETWORK_ID" => GenerateDimensionsItems::BuyerNetworkId,
                            "BUYER_NETWORK_NAME" => GenerateDimensionsItems::BuyerNetworkName,
                            "CONTENT_PLATFORM_CODE" => GenerateDimensionsItems::ContentPlatformCode,
                            "CONTENT_PLATFORM_NAME" => GenerateDimensionsItems::ContentPlatformName,
                            "COUNTRY_CODE" => GenerateDimensionsItems::CountryCode,
                            "COUNTRY_NAME" => GenerateDimensionsItems::CountryName,
                            "CREATIVE_SIZE_CODE" => GenerateDimensionsItems::CreativeSizeCode,
                            "CREATIVE_SIZE_NAME" => GenerateDimensionsItems::CreativeSizeName,
                            "CUSTOM_CHANNEL_ID" => GenerateDimensionsItems::CustomChannelId,
                            "CUSTOM_CHANNEL_NAME" => GenerateDimensionsItems::CustomChannelName,
                            "CUSTOM_SEARCH_STYLE_ID" => {
                                GenerateDimensionsItems::CustomSearchStyleId
                            }
                            "CUSTOM_SEARCH_STYLE_NAME" => {
                                GenerateDimensionsItems::CustomSearchStyleName
                            }
                            "DATE" => GenerateDimensionsItems::Date,
                            "DIMENSION_UNSPECIFIED" => {
                                GenerateDimensionsItems::DimensionUnspecified
                            }
                            "DOMAIN_CODE" => GenerateDimensionsItems::DomainCode,
                            "DOMAIN_NAME" => GenerateDimensionsItems::DomainName,
                            "DOMAIN_REGISTRANT" => GenerateDimensionsItems::DomainRegistrant,
                            "MONTH" => GenerateDimensionsItems::Month,
                            "OWNED_SITE_DOMAIN_NAME" => {
                                GenerateDimensionsItems::OwnedSiteDomainName
                            }
                            "OWNED_SITE_ID" => GenerateDimensionsItems::OwnedSiteId,
                            "PLATFORM_TYPE_CODE" => GenerateDimensionsItems::PlatformTypeCode,
                            "PLATFORM_TYPE_NAME" => GenerateDimensionsItems::PlatformTypeName,
                            "PRODUCT_CODE" => GenerateDimensionsItems::ProductCode,
                            "PRODUCT_NAME" => GenerateDimensionsItems::ProductName,
                            "REQUESTED_AD_TYPE_CODE" => {
                                GenerateDimensionsItems::RequestedAdTypeCode
                            }
                            "REQUESTED_AD_TYPE_NAME" => {
                                GenerateDimensionsItems::RequestedAdTypeName
                            }
                            "SERVED_AD_TYPE_CODE" => GenerateDimensionsItems::ServedAdTypeCode,
                            "SERVED_AD_TYPE_NAME" => GenerateDimensionsItems::ServedAdTypeName,
                            "TARGETING_TYPE_CODE" => GenerateDimensionsItems::TargetingTypeCode,
                            "TARGETING_TYPE_NAME" => GenerateDimensionsItems::TargetingTypeName,
                            "URL_CHANNEL_ID" => GenerateDimensionsItems::UrlChannelId,
                            "URL_CHANNEL_NAME" => GenerateDimensionsItems::UrlChannelName,
                            "WEBSEARCH_QUERY_STRING" => {
                                GenerateDimensionsItems::WebsearchQueryString
                            }
                            "WEEK" => GenerateDimensionsItems::Week,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for GenerateDimensionsItems {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for GenerateDimensionsItems {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum GenerateMetricsItems {
                    #[doc = "Ratio of requests that were measurable for viewability."]
                    ActiveViewMeasurability,
                    #[doc = "Mean time an ad was displayed on screen."]
                    ActiveViewTime,
                    #[doc = "Ratio of requests that were viewable."]
                    ActiveViewViewability,
                    #[doc = "Number of ad units that requested ads (for content ads) or search queries (for search ads). An ad request may result in zero, one, or multiple individual ad impressions depending on the size of the ad unit and whether any ads were available."]
                    AdRequests,
                    #[doc = "Ratio of requested ad units or queries to the number returned to the site."]
                    AdRequestsCoverage,
                    #[doc = "Ratio of ad requests that resulted in a click."]
                    AdRequestsCtr,
                    #[doc = "Revenue per thousand ad requests. This is calculated by dividing estimated revenue by the number of ad requests multiplied by 1000."]
                    AdRequestsRpm,
                    #[doc = "Fraction of ad requests considered to be spam. Only available to premium accounts."]
                    AdRequestsSpamRatio,
                    #[doc = "Number of ad views per impression."]
                    AdsPerImpression,
                    #[doc = "Number of times a user clicked on a standard content ad."]
                    Clicks,
                    #[doc = "Fraction of clicks considered to be spam. Only available to premium accounts."]
                    ClicksSpamRatio,
                    #[doc = "Amount the publisher earns each time a user clicks on an ad. CPC is calculated by dividing the estimated revenue by the number of clicks received."]
                    CostPerClick,
                    #[doc = "Estimated earnings of the publisher. Note that earnings up to yesterday are accurate, more recent earnings are estimated due to the possibility of spam, or exchange rate fluctuations."]
                    EstimatedEarnings,
                    #[doc = "Impressions. An impression is counted for each ad request where at least one ad has been downloaded to the user’s device and has begun to load. It is the number of ad units (for content ads) or search queries (for search ads) that showed ads."]
                    Impressions,
                    #[doc = "Ratio of IMPRESSIONS that resulted in a click."]
                    ImpressionsCtr,
                    #[doc = "Revenue per thousand ad impressions. This is calculated by dividing estimated revenue by the number of ad impressions multiplied by 1000."]
                    ImpressionsRpm,
                    #[doc = "Fraction of impressions considered to be spam. Only available to premium accounts."]
                    ImpressionsSpamRatio,
                    #[doc = "Ads shown. Different ad formats will display varying numbers of ads. For example, a vertical banner may consist of 2 or more ads. Also, the number of ads in an ad unit may vary depending on whether the ad unit is displaying standard text ads, expanded text ads or image ads."]
                    IndividualAdImpressions,
                    #[doc = "Ratio of individual ad impressions that resulted in a click."]
                    IndividualAdImpressionsCtr,
                    #[doc = "Revenue per thousand individual ad impressions. This is calculated by dividing estimated revenue by the number of individual ad impressions multiplied by 1000."]
                    IndividualAdImpressionsRpm,
                    #[doc = "Fraction of ad impressions considered to be spam. Only available to premium accounts."]
                    IndividualAdImpressionsSpamRatio,
                    #[doc = "Requests that returned at least one ad."]
                    MatchedAdRequests,
                    #[doc = "Ratio of clicks to matched requests."]
                    MatchedAdRequestsCtr,
                    #[doc = "Revenue per thousand matched ad requests. This is calculated by dividing estimated revenue by the number of matched ad requests multiplied by 1000."]
                    MatchedAdRequestsRpm,
                    #[doc = "Fraction of ad requests that returned ads considered to be spam. Only available to premium accounts."]
                    MatchedAdRequestsSpamRatio,
                    #[doc = "Unspecified metric."]
                    MetricUnspecified,
                    #[doc = "Number of page views."]
                    PageViews,
                    #[doc = "Ratio of individual page views that resulted in a click."]
                    PageViewsCtr,
                    #[doc = "Revenue per thousand page views. This is calculated by dividing the estimated revenue by the number of page views multiplied by 1000."]
                    PageViewsRpm,
                    #[doc = "Fraction of page views considered to be spam. Only available to premium accounts."]
                    PageViewsSpamRatio,
                    #[doc = "Total earnings are the gross estimated earnings from revenue shared traffic before any parent and child account revenue share is applied."]
                    TotalEarnings,
                    #[doc = "Impressions. An impression is counted for each ad request where at least one ad has been downloaded to the user’s device and has begun to load. It is the number of ad units (for content ads) or search queries (for search ads) that showed ads."]
                    TotalImpressions,
                    #[doc = "Number of results pages."]
                    WebsearchResultPages,
                }
                impl GenerateMetricsItems {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            GenerateMetricsItems::ActiveViewMeasurability => {
                                "ACTIVE_VIEW_MEASURABILITY"
                            }
                            GenerateMetricsItems::ActiveViewTime => "ACTIVE_VIEW_TIME",
                            GenerateMetricsItems::ActiveViewViewability => {
                                "ACTIVE_VIEW_VIEWABILITY"
                            }
                            GenerateMetricsItems::AdRequests => "AD_REQUESTS",
                            GenerateMetricsItems::AdRequestsCoverage => "AD_REQUESTS_COVERAGE",
                            GenerateMetricsItems::AdRequestsCtr => "AD_REQUESTS_CTR",
                            GenerateMetricsItems::AdRequestsRpm => "AD_REQUESTS_RPM",
                            GenerateMetricsItems::AdRequestsSpamRatio => "AD_REQUESTS_SPAM_RATIO",
                            GenerateMetricsItems::AdsPerImpression => "ADS_PER_IMPRESSION",
                            GenerateMetricsItems::Clicks => "CLICKS",
                            GenerateMetricsItems::ClicksSpamRatio => "CLICKS_SPAM_RATIO",
                            GenerateMetricsItems::CostPerClick => "COST_PER_CLICK",
                            GenerateMetricsItems::EstimatedEarnings => "ESTIMATED_EARNINGS",
                            GenerateMetricsItems::Impressions => "IMPRESSIONS",
                            GenerateMetricsItems::ImpressionsCtr => "IMPRESSIONS_CTR",
                            GenerateMetricsItems::ImpressionsRpm => "IMPRESSIONS_RPM",
                            GenerateMetricsItems::ImpressionsSpamRatio => "IMPRESSIONS_SPAM_RATIO",
                            GenerateMetricsItems::IndividualAdImpressions => {
                                "INDIVIDUAL_AD_IMPRESSIONS"
                            }
                            GenerateMetricsItems::IndividualAdImpressionsCtr => {
                                "INDIVIDUAL_AD_IMPRESSIONS_CTR"
                            }
                            GenerateMetricsItems::IndividualAdImpressionsRpm => {
                                "INDIVIDUAL_AD_IMPRESSIONS_RPM"
                            }
                            GenerateMetricsItems::IndividualAdImpressionsSpamRatio => {
                                "INDIVIDUAL_AD_IMPRESSIONS_SPAM_RATIO"
                            }
                            GenerateMetricsItems::MatchedAdRequests => "MATCHED_AD_REQUESTS",
                            GenerateMetricsItems::MatchedAdRequestsCtr => "MATCHED_AD_REQUESTS_CTR",
                            GenerateMetricsItems::MatchedAdRequestsRpm => "MATCHED_AD_REQUESTS_RPM",
                            GenerateMetricsItems::MatchedAdRequestsSpamRatio => {
                                "MATCHED_AD_REQUESTS_SPAM_RATIO"
                            }
                            GenerateMetricsItems::MetricUnspecified => "METRIC_UNSPECIFIED",
                            GenerateMetricsItems::PageViews => "PAGE_VIEWS",
                            GenerateMetricsItems::PageViewsCtr => "PAGE_VIEWS_CTR",
                            GenerateMetricsItems::PageViewsRpm => "PAGE_VIEWS_RPM",
                            GenerateMetricsItems::PageViewsSpamRatio => "PAGE_VIEWS_SPAM_RATIO",
                            GenerateMetricsItems::TotalEarnings => "TOTAL_EARNINGS",
                            GenerateMetricsItems::TotalImpressions => "TOTAL_IMPRESSIONS",
                            GenerateMetricsItems::WebsearchResultPages => "WEBSEARCH_RESULT_PAGES",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for GenerateMetricsItems {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for GenerateMetricsItems {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<GenerateMetricsItems, ()> {
                        Ok(match s {
                            "ACTIVE_VIEW_MEASURABILITY" => {
                                GenerateMetricsItems::ActiveViewMeasurability
                            }
                            "ACTIVE_VIEW_TIME" => GenerateMetricsItems::ActiveViewTime,
                            "ACTIVE_VIEW_VIEWABILITY" => {
                                GenerateMetricsItems::ActiveViewViewability
                            }
                            "AD_REQUESTS" => GenerateMetricsItems::AdRequests,
                            "AD_REQUESTS_COVERAGE" => GenerateMetricsItems::AdRequestsCoverage,
                            "AD_REQUESTS_CTR" => GenerateMetricsItems::AdRequestsCtr,
                            "AD_REQUESTS_RPM" => GenerateMetricsItems::AdRequestsRpm,
                            "AD_REQUESTS_SPAM_RATIO" => GenerateMetricsItems::AdRequestsSpamRatio,
                            "ADS_PER_IMPRESSION" => GenerateMetricsItems::AdsPerImpression,
                            "CLICKS" => GenerateMetricsItems::Clicks,
                            "CLICKS_SPAM_RATIO" => GenerateMetricsItems::ClicksSpamRatio,
                            "COST_PER_CLICK" => GenerateMetricsItems::CostPerClick,
                            "ESTIMATED_EARNINGS" => GenerateMetricsItems::EstimatedEarnings,
                            "IMPRESSIONS" => GenerateMetricsItems::Impressions,
                            "IMPRESSIONS_CTR" => GenerateMetricsItems::ImpressionsCtr,
                            "IMPRESSIONS_RPM" => GenerateMetricsItems::ImpressionsRpm,
                            "IMPRESSIONS_SPAM_RATIO" => GenerateMetricsItems::ImpressionsSpamRatio,
                            "INDIVIDUAL_AD_IMPRESSIONS" => {
                                GenerateMetricsItems::IndividualAdImpressions
                            }
                            "INDIVIDUAL_AD_IMPRESSIONS_CTR" => {
                                GenerateMetricsItems::IndividualAdImpressionsCtr
                            }
                            "INDIVIDUAL_AD_IMPRESSIONS_RPM" => {
                                GenerateMetricsItems::IndividualAdImpressionsRpm
                            }
                            "INDIVIDUAL_AD_IMPRESSIONS_SPAM_RATIO" => {
                                GenerateMetricsItems::IndividualAdImpressionsSpamRatio
                            }
                            "MATCHED_AD_REQUESTS" => GenerateMetricsItems::MatchedAdRequests,
                            "MATCHED_AD_REQUESTS_CTR" => GenerateMetricsItems::MatchedAdRequestsCtr,
                            "MATCHED_AD_REQUESTS_RPM" => GenerateMetricsItems::MatchedAdRequestsRpm,
                            "MATCHED_AD_REQUESTS_SPAM_RATIO" => {
                                GenerateMetricsItems::MatchedAdRequestsSpamRatio
                            }
                            "METRIC_UNSPECIFIED" => GenerateMetricsItems::MetricUnspecified,
                            "PAGE_VIEWS" => GenerateMetricsItems::PageViews,
                            "PAGE_VIEWS_CTR" => GenerateMetricsItems::PageViewsCtr,
                            "PAGE_VIEWS_RPM" => GenerateMetricsItems::PageViewsRpm,
                            "PAGE_VIEWS_SPAM_RATIO" => GenerateMetricsItems::PageViewsSpamRatio,
                            "TOTAL_EARNINGS" => GenerateMetricsItems::TotalEarnings,
                            "TOTAL_IMPRESSIONS" => GenerateMetricsItems::TotalImpressions,
                            "WEBSEARCH_RESULT_PAGES" => GenerateMetricsItems::WebsearchResultPages,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for GenerateMetricsItems {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for GenerateMetricsItems {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for GenerateMetricsItems {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "ACTIVE_VIEW_MEASURABILITY" => {
                                GenerateMetricsItems::ActiveViewMeasurability
                            }
                            "ACTIVE_VIEW_TIME" => GenerateMetricsItems::ActiveViewTime,
                            "ACTIVE_VIEW_VIEWABILITY" => {
                                GenerateMetricsItems::ActiveViewViewability
                            }
                            "AD_REQUESTS" => GenerateMetricsItems::AdRequests,
                            "AD_REQUESTS_COVERAGE" => GenerateMetricsItems::AdRequestsCoverage,
                            "AD_REQUESTS_CTR" => GenerateMetricsItems::AdRequestsCtr,
                            "AD_REQUESTS_RPM" => GenerateMetricsItems::AdRequestsRpm,
                            "AD_REQUESTS_SPAM_RATIO" => GenerateMetricsItems::AdRequestsSpamRatio,
                            "ADS_PER_IMPRESSION" => GenerateMetricsItems::AdsPerImpression,
                            "CLICKS" => GenerateMetricsItems::Clicks,
                            "CLICKS_SPAM_RATIO" => GenerateMetricsItems::ClicksSpamRatio,
                            "COST_PER_CLICK" => GenerateMetricsItems::CostPerClick,
                            "ESTIMATED_EARNINGS" => GenerateMetricsItems::EstimatedEarnings,
                            "IMPRESSIONS" => GenerateMetricsItems::Impressions,
                            "IMPRESSIONS_CTR" => GenerateMetricsItems::ImpressionsCtr,
                            "IMPRESSIONS_RPM" => GenerateMetricsItems::ImpressionsRpm,
                            "IMPRESSIONS_SPAM_RATIO" => GenerateMetricsItems::ImpressionsSpamRatio,
                            "INDIVIDUAL_AD_IMPRESSIONS" => {
                                GenerateMetricsItems::IndividualAdImpressions
                            }
                            "INDIVIDUAL_AD_IMPRESSIONS_CTR" => {
                                GenerateMetricsItems::IndividualAdImpressionsCtr
                            }
                            "INDIVIDUAL_AD_IMPRESSIONS_RPM" => {
                                GenerateMetricsItems::IndividualAdImpressionsRpm
                            }
                            "INDIVIDUAL_AD_IMPRESSIONS_SPAM_RATIO" => {
                                GenerateMetricsItems::IndividualAdImpressionsSpamRatio
                            }
                            "MATCHED_AD_REQUESTS" => GenerateMetricsItems::MatchedAdRequests,
                            "MATCHED_AD_REQUESTS_CTR" => GenerateMetricsItems::MatchedAdRequestsCtr,
                            "MATCHED_AD_REQUESTS_RPM" => GenerateMetricsItems::MatchedAdRequestsRpm,
                            "MATCHED_AD_REQUESTS_SPAM_RATIO" => {
                                GenerateMetricsItems::MatchedAdRequestsSpamRatio
                            }
                            "METRIC_UNSPECIFIED" => GenerateMetricsItems::MetricUnspecified,
                            "PAGE_VIEWS" => GenerateMetricsItems::PageViews,
                            "PAGE_VIEWS_CTR" => GenerateMetricsItems::PageViewsCtr,
                            "PAGE_VIEWS_RPM" => GenerateMetricsItems::PageViewsRpm,
                            "PAGE_VIEWS_SPAM_RATIO" => GenerateMetricsItems::PageViewsSpamRatio,
                            "TOTAL_EARNINGS" => GenerateMetricsItems::TotalEarnings,
                            "TOTAL_IMPRESSIONS" => GenerateMetricsItems::TotalImpressions,
                            "WEBSEARCH_RESULT_PAGES" => GenerateMetricsItems::WebsearchResultPages,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for GenerateMetricsItems {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for GenerateMetricsItems {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum GenerateReportingTimeZone {
                    #[doc = "Use the account timezone in the report."]
                    AccountTimeZone,
                    #[doc = "Use the Google timezone in the report (America/Los_Angeles)."]
                    GoogleTimeZone,
                    #[doc = "Unspecified timezone."]
                    ReportingTimeZoneUnspecified,
                }
                impl GenerateReportingTimeZone {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            GenerateReportingTimeZone::AccountTimeZone => "ACCOUNT_TIME_ZONE",
                            GenerateReportingTimeZone::GoogleTimeZone => "GOOGLE_TIME_ZONE",
                            GenerateReportingTimeZone::ReportingTimeZoneUnspecified => {
                                "REPORTING_TIME_ZONE_UNSPECIFIED"
                            }
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for GenerateReportingTimeZone {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for GenerateReportingTimeZone {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<GenerateReportingTimeZone, ()> {
                        Ok(match s {
                            "ACCOUNT_TIME_ZONE" => GenerateReportingTimeZone::AccountTimeZone,
                            "GOOGLE_TIME_ZONE" => GenerateReportingTimeZone::GoogleTimeZone,
                            "REPORTING_TIME_ZONE_UNSPECIFIED" => {
                                GenerateReportingTimeZone::ReportingTimeZoneUnspecified
                            }
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for GenerateReportingTimeZone {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for GenerateReportingTimeZone {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for GenerateReportingTimeZone {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "ACCOUNT_TIME_ZONE" => GenerateReportingTimeZone::AccountTimeZone,
                            "GOOGLE_TIME_ZONE" => GenerateReportingTimeZone::GoogleTimeZone,
                            "REPORTING_TIME_ZONE_UNSPECIFIED" => {
                                GenerateReportingTimeZone::ReportingTimeZoneUnspecified
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
                impl ::google_field_selector::FieldSelector for GenerateReportingTimeZone {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for GenerateReportingTimeZone {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum GenerateCsvDateRange {
                    #[doc = "A custom date range specified using the `start_date` and `end_date` fields. This is the default if no ReportingDateRange is provided."]
                    Custom,
                    #[doc = "Last 30 days, excluding current day."]
                    Last30Days,
                    #[doc = "Last 7 days, excluding current day."]
                    Last7Days,
                    #[doc = "From the start of the current month to the current day. e.g. if the current date is 2020-03-12 then the range will be [2020-03-01, 2020-03-12]."]
                    MonthToDate,
                    #[doc = "Unspecified date range."]
                    ReportingDateRangeUnspecified,
                    #[doc = "Current day."]
                    Today,
                    #[doc = "From the start of the current year to the current day. e.g. if the current date is 2020-03-12 then the range will be [2020-01-01, 2020-03-12]."]
                    YearToDate,
                    #[doc = "Yesterday."]
                    Yesterday,
                }
                impl GenerateCsvDateRange {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            GenerateCsvDateRange::Custom => "CUSTOM",
                            GenerateCsvDateRange::Last30Days => "LAST_30_DAYS",
                            GenerateCsvDateRange::Last7Days => "LAST_7_DAYS",
                            GenerateCsvDateRange::MonthToDate => "MONTH_TO_DATE",
                            GenerateCsvDateRange::ReportingDateRangeUnspecified => {
                                "REPORTING_DATE_RANGE_UNSPECIFIED"
                            }
                            GenerateCsvDateRange::Today => "TODAY",
                            GenerateCsvDateRange::YearToDate => "YEAR_TO_DATE",
                            GenerateCsvDateRange::Yesterday => "YESTERDAY",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for GenerateCsvDateRange {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for GenerateCsvDateRange {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<GenerateCsvDateRange, ()> {
                        Ok(match s {
                            "CUSTOM" => GenerateCsvDateRange::Custom,
                            "LAST_30_DAYS" => GenerateCsvDateRange::Last30Days,
                            "LAST_7_DAYS" => GenerateCsvDateRange::Last7Days,
                            "MONTH_TO_DATE" => GenerateCsvDateRange::MonthToDate,
                            "REPORTING_DATE_RANGE_UNSPECIFIED" => {
                                GenerateCsvDateRange::ReportingDateRangeUnspecified
                            }
                            "TODAY" => GenerateCsvDateRange::Today,
                            "YEAR_TO_DATE" => GenerateCsvDateRange::YearToDate,
                            "YESTERDAY" => GenerateCsvDateRange::Yesterday,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for GenerateCsvDateRange {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for GenerateCsvDateRange {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for GenerateCsvDateRange {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "CUSTOM" => GenerateCsvDateRange::Custom,
                            "LAST_30_DAYS" => GenerateCsvDateRange::Last30Days,
                            "LAST_7_DAYS" => GenerateCsvDateRange::Last7Days,
                            "MONTH_TO_DATE" => GenerateCsvDateRange::MonthToDate,
                            "REPORTING_DATE_RANGE_UNSPECIFIED" => {
                                GenerateCsvDateRange::ReportingDateRangeUnspecified
                            }
                            "TODAY" => GenerateCsvDateRange::Today,
                            "YEAR_TO_DATE" => GenerateCsvDateRange::YearToDate,
                            "YESTERDAY" => GenerateCsvDateRange::Yesterday,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for GenerateCsvDateRange {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for GenerateCsvDateRange {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum GenerateCsvDimensionsItems {
                    #[doc = "Account name. The members of this dimension match the values from Account.display_name."]
                    AccountName,
                    #[doc = "Unique ID of an ad client. The members of this dimension match the values from AdClient.reporting_dimension_id."]
                    AdClientId,
                    #[doc = "Ad format code indicating the way an ad is shown to the users on your site (e.g. \"ON_PAGE\", \"ANCHOR\", \"INTERSTITIAL\")."]
                    AdFormatCode,
                    #[doc = "Localized ad format name indicating the way an ad is shown to the users on your site (e.g. \"In-page\", \"Anchor\", \"Vignette\")."]
                    AdFormatName,
                    #[doc = "Ad placement code (e.g. \"AD_UNIT\", \"ca-pub-123456:78910\", \"OTHER\")."]
                    AdPlacementCode,
                    #[doc = "Localized ad placement name (e.g. \"Ad unit\", \"Global settings\", \"Manual\")."]
                    AdPlacementName,
                    #[doc = "Unique ID of an ad unit (within which an ad was served). The members of this dimension match the values from AdUnit.reporting_dimension_id."]
                    AdUnitId,
                    #[doc = "Ad unit name (within which an ad was served). The members of this dimension match the values from AdUnit.display_name."]
                    AdUnitName,
                    #[doc = "The size code of an ad unit (e.g. \"728x90\", \"responsive\")."]
                    AdUnitSizeCode,
                    #[doc = "Localized size of an ad unit (e.g. \"728x90\", \"Responsive\")."]
                    AdUnitSizeName,
                    #[doc = "Type of a bid (e.g. \"cpc\", \"cpm\") for a served ad."]
                    BidTypeCode,
                    #[doc = "Localized bid type name (e.g. \"CPC bids\", \"CPM bids\") for a served ad."]
                    BidTypeName,
                    #[doc = "Unique (opaque) ID of an ad network that returned the winning ads for an ad request."]
                    BuyerNetworkId,
                    #[doc = "Name of an ad network that returned the winning ads for an ad request (e.g. \"Google AdWords\"). Note that unlike other \"NAME\" dimensions, the members of this dimensions are not localized."]
                    BuyerNetworkName,
                    #[doc = "Content platform code an ad request was made from (e.g. \"AMP\", \"HTML\")."]
                    ContentPlatformCode,
                    #[doc = "Localized content platform name an ad request was made from (e.g. \"AMP\", \"Web\")."]
                    ContentPlatformName,
                    #[doc = "CLDR region code of a user viewing an ad (e.g. \"US\", \"FR\")."]
                    CountryCode,
                    #[doc = "Localized region name of a user viewing an ad (e.g. \"United States\", \"France\")."]
                    CountryName,
                    #[doc = "Creative size code (e.g. \"728x90\", \"dynamic\") of a served ad."]
                    CreativeSizeCode,
                    #[doc = "Localized creative size name (e.g. \"728x90\", \"Dynamic\") of a served ad."]
                    CreativeSizeName,
                    #[doc = "Unique ID of a custom channel. The members of this dimension match the values from CustomChannel.reporting_dimension_id."]
                    CustomChannelId,
                    #[doc = "Custom channel name. The members of this dimension match the values from CustomChannel.display_name."]
                    CustomChannelName,
                    #[doc = "Custom search style id."]
                    CustomSearchStyleId,
                    #[doc = "Custom search style name."]
                    CustomSearchStyleName,
                    #[doc = "Date dimension in YYYY-MM-DD format (e.g. \"2010-02-10\")."]
                    Date,
                    #[doc = "Unspecified dimension."]
                    DimensionUnspecified,
                    #[doc = "Name of a host on which an ad was served (e.g. \"www.google.com\", \"webcaches\", \"xn--bcher-kva.example\")."]
                    DomainCode,
                    #[doc = "Localized name of a host on which an ad was served, after IDNA decoding (e.g. \"www.google.com\", \"Web caches and other\", \"bücher.example\")."]
                    DomainName,
                    #[doc = "Domain registrants."]
                    DomainRegistrant,
                    #[doc = "Month dimension in YYYY-MM format (e.g. \"2010-02\")."]
                    Month,
                    #[doc = "Domain name of a verified site (e.g. \"example.com\"). The members of this dimension match the values from Site.domain."]
                    OwnedSiteDomainName,
                    #[doc = "Unique ID of a verified site. The members of this dimension match the values from Site.reporting_dimension_id."]
                    OwnedSiteId,
                    #[doc = "Platform type code (e.g. \"HighEndMobile\", \"Desktop\")."]
                    PlatformTypeCode,
                    #[doc = "Localized platform type name (e.g. \"High-end mobile devices\", \"Desktop\")."]
                    PlatformTypeName,
                    #[doc = "Product code (e.g. \"AFC\", \"AFS\"). The members of this dimension match the values from AdClient.product_code."]
                    ProductCode,
                    #[doc = "Localized product name (e.g. \"AdSense for Content\", \"AdSense for Search\")."]
                    ProductName,
                    #[doc = "Requested ad type code (e.g. \"IMAGE\", \"RADLINK\", \"OTHER\")."]
                    RequestedAdTypeCode,
                    #[doc = "Localized requested ad type name (e.g. \"Display\", \"Link unit\", \"Other\")."]
                    RequestedAdTypeName,
                    #[doc = "Served ad type code (e.g. \"IMAGE\", \"RADLINK\", \"OTHER\")."]
                    ServedAdTypeCode,
                    #[doc = "Localized served ad type name (e.g. \"Display\", \"Link unit\", \"Other\")."]
                    ServedAdTypeName,
                    #[doc = "Targeting type code (e.g. \"Keyword\", \"UserInterest\", \"RunOfNetwork\")."]
                    TargetingTypeCode,
                    #[doc = "Localized targeting type name (e.g. \"Contextual\", \"Personalized\", \"Run of Network\")."]
                    TargetingTypeName,
                    #[doc = "Unique ID of a URL channel. The members of this dimension match the values from UrlChannel.reporting_dimension_id."]
                    UrlChannelId,
                    #[doc = "Name of a URL channel. The members of this dimension match the values from UrlChannel.uri_pattern."]
                    UrlChannelName,
                    #[doc = "Query strings for web searches."]
                    WebsearchQueryString,
                    #[doc = "Week dimension in YYYY-MM-DD format, representing the first day of each week (e.g. \"2010-02-08\"). The first day of the week is determined by the language_code specified in a report generation request (so e.g. this would be a Monday for \"en-GB\" or \"es\", but a Sunday for \"en\" or \"fr-CA\")."]
                    Week,
                }
                impl GenerateCsvDimensionsItems {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            GenerateCsvDimensionsItems::AccountName => "ACCOUNT_NAME",
                            GenerateCsvDimensionsItems::AdClientId => "AD_CLIENT_ID",
                            GenerateCsvDimensionsItems::AdFormatCode => "AD_FORMAT_CODE",
                            GenerateCsvDimensionsItems::AdFormatName => "AD_FORMAT_NAME",
                            GenerateCsvDimensionsItems::AdPlacementCode => "AD_PLACEMENT_CODE",
                            GenerateCsvDimensionsItems::AdPlacementName => "AD_PLACEMENT_NAME",
                            GenerateCsvDimensionsItems::AdUnitId => "AD_UNIT_ID",
                            GenerateCsvDimensionsItems::AdUnitName => "AD_UNIT_NAME",
                            GenerateCsvDimensionsItems::AdUnitSizeCode => "AD_UNIT_SIZE_CODE",
                            GenerateCsvDimensionsItems::AdUnitSizeName => "AD_UNIT_SIZE_NAME",
                            GenerateCsvDimensionsItems::BidTypeCode => "BID_TYPE_CODE",
                            GenerateCsvDimensionsItems::BidTypeName => "BID_TYPE_NAME",
                            GenerateCsvDimensionsItems::BuyerNetworkId => "BUYER_NETWORK_ID",
                            GenerateCsvDimensionsItems::BuyerNetworkName => "BUYER_NETWORK_NAME",
                            GenerateCsvDimensionsItems::ContentPlatformCode => {
                                "CONTENT_PLATFORM_CODE"
                            }
                            GenerateCsvDimensionsItems::ContentPlatformName => {
                                "CONTENT_PLATFORM_NAME"
                            }
                            GenerateCsvDimensionsItems::CountryCode => "COUNTRY_CODE",
                            GenerateCsvDimensionsItems::CountryName => "COUNTRY_NAME",
                            GenerateCsvDimensionsItems::CreativeSizeCode => "CREATIVE_SIZE_CODE",
                            GenerateCsvDimensionsItems::CreativeSizeName => "CREATIVE_SIZE_NAME",
                            GenerateCsvDimensionsItems::CustomChannelId => "CUSTOM_CHANNEL_ID",
                            GenerateCsvDimensionsItems::CustomChannelName => "CUSTOM_CHANNEL_NAME",
                            GenerateCsvDimensionsItems::CustomSearchStyleId => {
                                "CUSTOM_SEARCH_STYLE_ID"
                            }
                            GenerateCsvDimensionsItems::CustomSearchStyleName => {
                                "CUSTOM_SEARCH_STYLE_NAME"
                            }
                            GenerateCsvDimensionsItems::Date => "DATE",
                            GenerateCsvDimensionsItems::DimensionUnspecified => {
                                "DIMENSION_UNSPECIFIED"
                            }
                            GenerateCsvDimensionsItems::DomainCode => "DOMAIN_CODE",
                            GenerateCsvDimensionsItems::DomainName => "DOMAIN_NAME",
                            GenerateCsvDimensionsItems::DomainRegistrant => "DOMAIN_REGISTRANT",
                            GenerateCsvDimensionsItems::Month => "MONTH",
                            GenerateCsvDimensionsItems::OwnedSiteDomainName => {
                                "OWNED_SITE_DOMAIN_NAME"
                            }
                            GenerateCsvDimensionsItems::OwnedSiteId => "OWNED_SITE_ID",
                            GenerateCsvDimensionsItems::PlatformTypeCode => "PLATFORM_TYPE_CODE",
                            GenerateCsvDimensionsItems::PlatformTypeName => "PLATFORM_TYPE_NAME",
                            GenerateCsvDimensionsItems::ProductCode => "PRODUCT_CODE",
                            GenerateCsvDimensionsItems::ProductName => "PRODUCT_NAME",
                            GenerateCsvDimensionsItems::RequestedAdTypeCode => {
                                "REQUESTED_AD_TYPE_CODE"
                            }
                            GenerateCsvDimensionsItems::RequestedAdTypeName => {
                                "REQUESTED_AD_TYPE_NAME"
                            }
                            GenerateCsvDimensionsItems::ServedAdTypeCode => "SERVED_AD_TYPE_CODE",
                            GenerateCsvDimensionsItems::ServedAdTypeName => "SERVED_AD_TYPE_NAME",
                            GenerateCsvDimensionsItems::TargetingTypeCode => "TARGETING_TYPE_CODE",
                            GenerateCsvDimensionsItems::TargetingTypeName => "TARGETING_TYPE_NAME",
                            GenerateCsvDimensionsItems::UrlChannelId => "URL_CHANNEL_ID",
                            GenerateCsvDimensionsItems::UrlChannelName => "URL_CHANNEL_NAME",
                            GenerateCsvDimensionsItems::WebsearchQueryString => {
                                "WEBSEARCH_QUERY_STRING"
                            }
                            GenerateCsvDimensionsItems::Week => "WEEK",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for GenerateCsvDimensionsItems {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for GenerateCsvDimensionsItems {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<GenerateCsvDimensionsItems, ()> {
                        Ok(match s {
                            "ACCOUNT_NAME" => GenerateCsvDimensionsItems::AccountName,
                            "AD_CLIENT_ID" => GenerateCsvDimensionsItems::AdClientId,
                            "AD_FORMAT_CODE" => GenerateCsvDimensionsItems::AdFormatCode,
                            "AD_FORMAT_NAME" => GenerateCsvDimensionsItems::AdFormatName,
                            "AD_PLACEMENT_CODE" => GenerateCsvDimensionsItems::AdPlacementCode,
                            "AD_PLACEMENT_NAME" => GenerateCsvDimensionsItems::AdPlacementName,
                            "AD_UNIT_ID" => GenerateCsvDimensionsItems::AdUnitId,
                            "AD_UNIT_NAME" => GenerateCsvDimensionsItems::AdUnitName,
                            "AD_UNIT_SIZE_CODE" => GenerateCsvDimensionsItems::AdUnitSizeCode,
                            "AD_UNIT_SIZE_NAME" => GenerateCsvDimensionsItems::AdUnitSizeName,
                            "BID_TYPE_CODE" => GenerateCsvDimensionsItems::BidTypeCode,
                            "BID_TYPE_NAME" => GenerateCsvDimensionsItems::BidTypeName,
                            "BUYER_NETWORK_ID" => GenerateCsvDimensionsItems::BuyerNetworkId,
                            "BUYER_NETWORK_NAME" => GenerateCsvDimensionsItems::BuyerNetworkName,
                            "CONTENT_PLATFORM_CODE" => {
                                GenerateCsvDimensionsItems::ContentPlatformCode
                            }
                            "CONTENT_PLATFORM_NAME" => {
                                GenerateCsvDimensionsItems::ContentPlatformName
                            }
                            "COUNTRY_CODE" => GenerateCsvDimensionsItems::CountryCode,
                            "COUNTRY_NAME" => GenerateCsvDimensionsItems::CountryName,
                            "CREATIVE_SIZE_CODE" => GenerateCsvDimensionsItems::CreativeSizeCode,
                            "CREATIVE_SIZE_NAME" => GenerateCsvDimensionsItems::CreativeSizeName,
                            "CUSTOM_CHANNEL_ID" => GenerateCsvDimensionsItems::CustomChannelId,
                            "CUSTOM_CHANNEL_NAME" => GenerateCsvDimensionsItems::CustomChannelName,
                            "CUSTOM_SEARCH_STYLE_ID" => {
                                GenerateCsvDimensionsItems::CustomSearchStyleId
                            }
                            "CUSTOM_SEARCH_STYLE_NAME" => {
                                GenerateCsvDimensionsItems::CustomSearchStyleName
                            }
                            "DATE" => GenerateCsvDimensionsItems::Date,
                            "DIMENSION_UNSPECIFIED" => {
                                GenerateCsvDimensionsItems::DimensionUnspecified
                            }
                            "DOMAIN_CODE" => GenerateCsvDimensionsItems::DomainCode,
                            "DOMAIN_NAME" => GenerateCsvDimensionsItems::DomainName,
                            "DOMAIN_REGISTRANT" => GenerateCsvDimensionsItems::DomainRegistrant,
                            "MONTH" => GenerateCsvDimensionsItems::Month,
                            "OWNED_SITE_DOMAIN_NAME" => {
                                GenerateCsvDimensionsItems::OwnedSiteDomainName
                            }
                            "OWNED_SITE_ID" => GenerateCsvDimensionsItems::OwnedSiteId,
                            "PLATFORM_TYPE_CODE" => GenerateCsvDimensionsItems::PlatformTypeCode,
                            "PLATFORM_TYPE_NAME" => GenerateCsvDimensionsItems::PlatformTypeName,
                            "PRODUCT_CODE" => GenerateCsvDimensionsItems::ProductCode,
                            "PRODUCT_NAME" => GenerateCsvDimensionsItems::ProductName,
                            "REQUESTED_AD_TYPE_CODE" => {
                                GenerateCsvDimensionsItems::RequestedAdTypeCode
                            }
                            "REQUESTED_AD_TYPE_NAME" => {
                                GenerateCsvDimensionsItems::RequestedAdTypeName
                            }
                            "SERVED_AD_TYPE_CODE" => GenerateCsvDimensionsItems::ServedAdTypeCode,
                            "SERVED_AD_TYPE_NAME" => GenerateCsvDimensionsItems::ServedAdTypeName,
                            "TARGETING_TYPE_CODE" => GenerateCsvDimensionsItems::TargetingTypeCode,
                            "TARGETING_TYPE_NAME" => GenerateCsvDimensionsItems::TargetingTypeName,
                            "URL_CHANNEL_ID" => GenerateCsvDimensionsItems::UrlChannelId,
                            "URL_CHANNEL_NAME" => GenerateCsvDimensionsItems::UrlChannelName,
                            "WEBSEARCH_QUERY_STRING" => {
                                GenerateCsvDimensionsItems::WebsearchQueryString
                            }
                            "WEEK" => GenerateCsvDimensionsItems::Week,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for GenerateCsvDimensionsItems {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for GenerateCsvDimensionsItems {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for GenerateCsvDimensionsItems {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "ACCOUNT_NAME" => GenerateCsvDimensionsItems::AccountName,
                            "AD_CLIENT_ID" => GenerateCsvDimensionsItems::AdClientId,
                            "AD_FORMAT_CODE" => GenerateCsvDimensionsItems::AdFormatCode,
                            "AD_FORMAT_NAME" => GenerateCsvDimensionsItems::AdFormatName,
                            "AD_PLACEMENT_CODE" => GenerateCsvDimensionsItems::AdPlacementCode,
                            "AD_PLACEMENT_NAME" => GenerateCsvDimensionsItems::AdPlacementName,
                            "AD_UNIT_ID" => GenerateCsvDimensionsItems::AdUnitId,
                            "AD_UNIT_NAME" => GenerateCsvDimensionsItems::AdUnitName,
                            "AD_UNIT_SIZE_CODE" => GenerateCsvDimensionsItems::AdUnitSizeCode,
                            "AD_UNIT_SIZE_NAME" => GenerateCsvDimensionsItems::AdUnitSizeName,
                            "BID_TYPE_CODE" => GenerateCsvDimensionsItems::BidTypeCode,
                            "BID_TYPE_NAME" => GenerateCsvDimensionsItems::BidTypeName,
                            "BUYER_NETWORK_ID" => GenerateCsvDimensionsItems::BuyerNetworkId,
                            "BUYER_NETWORK_NAME" => GenerateCsvDimensionsItems::BuyerNetworkName,
                            "CONTENT_PLATFORM_CODE" => {
                                GenerateCsvDimensionsItems::ContentPlatformCode
                            }
                            "CONTENT_PLATFORM_NAME" => {
                                GenerateCsvDimensionsItems::ContentPlatformName
                            }
                            "COUNTRY_CODE" => GenerateCsvDimensionsItems::CountryCode,
                            "COUNTRY_NAME" => GenerateCsvDimensionsItems::CountryName,
                            "CREATIVE_SIZE_CODE" => GenerateCsvDimensionsItems::CreativeSizeCode,
                            "CREATIVE_SIZE_NAME" => GenerateCsvDimensionsItems::CreativeSizeName,
                            "CUSTOM_CHANNEL_ID" => GenerateCsvDimensionsItems::CustomChannelId,
                            "CUSTOM_CHANNEL_NAME" => GenerateCsvDimensionsItems::CustomChannelName,
                            "CUSTOM_SEARCH_STYLE_ID" => {
                                GenerateCsvDimensionsItems::CustomSearchStyleId
                            }
                            "CUSTOM_SEARCH_STYLE_NAME" => {
                                GenerateCsvDimensionsItems::CustomSearchStyleName
                            }
                            "DATE" => GenerateCsvDimensionsItems::Date,
                            "DIMENSION_UNSPECIFIED" => {
                                GenerateCsvDimensionsItems::DimensionUnspecified
                            }
                            "DOMAIN_CODE" => GenerateCsvDimensionsItems::DomainCode,
                            "DOMAIN_NAME" => GenerateCsvDimensionsItems::DomainName,
                            "DOMAIN_REGISTRANT" => GenerateCsvDimensionsItems::DomainRegistrant,
                            "MONTH" => GenerateCsvDimensionsItems::Month,
                            "OWNED_SITE_DOMAIN_NAME" => {
                                GenerateCsvDimensionsItems::OwnedSiteDomainName
                            }
                            "OWNED_SITE_ID" => GenerateCsvDimensionsItems::OwnedSiteId,
                            "PLATFORM_TYPE_CODE" => GenerateCsvDimensionsItems::PlatformTypeCode,
                            "PLATFORM_TYPE_NAME" => GenerateCsvDimensionsItems::PlatformTypeName,
                            "PRODUCT_CODE" => GenerateCsvDimensionsItems::ProductCode,
                            "PRODUCT_NAME" => GenerateCsvDimensionsItems::ProductName,
                            "REQUESTED_AD_TYPE_CODE" => {
                                GenerateCsvDimensionsItems::RequestedAdTypeCode
                            }
                            "REQUESTED_AD_TYPE_NAME" => {
                                GenerateCsvDimensionsItems::RequestedAdTypeName
                            }
                            "SERVED_AD_TYPE_CODE" => GenerateCsvDimensionsItems::ServedAdTypeCode,
                            "SERVED_AD_TYPE_NAME" => GenerateCsvDimensionsItems::ServedAdTypeName,
                            "TARGETING_TYPE_CODE" => GenerateCsvDimensionsItems::TargetingTypeCode,
                            "TARGETING_TYPE_NAME" => GenerateCsvDimensionsItems::TargetingTypeName,
                            "URL_CHANNEL_ID" => GenerateCsvDimensionsItems::UrlChannelId,
                            "URL_CHANNEL_NAME" => GenerateCsvDimensionsItems::UrlChannelName,
                            "WEBSEARCH_QUERY_STRING" => {
                                GenerateCsvDimensionsItems::WebsearchQueryString
                            }
                            "WEEK" => GenerateCsvDimensionsItems::Week,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for GenerateCsvDimensionsItems {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for GenerateCsvDimensionsItems {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum GenerateCsvMetricsItems {
                    #[doc = "Ratio of requests that were measurable for viewability."]
                    ActiveViewMeasurability,
                    #[doc = "Mean time an ad was displayed on screen."]
                    ActiveViewTime,
                    #[doc = "Ratio of requests that were viewable."]
                    ActiveViewViewability,
                    #[doc = "Number of ad units that requested ads (for content ads) or search queries (for search ads). An ad request may result in zero, one, or multiple individual ad impressions depending on the size of the ad unit and whether any ads were available."]
                    AdRequests,
                    #[doc = "Ratio of requested ad units or queries to the number returned to the site."]
                    AdRequestsCoverage,
                    #[doc = "Ratio of ad requests that resulted in a click."]
                    AdRequestsCtr,
                    #[doc = "Revenue per thousand ad requests. This is calculated by dividing estimated revenue by the number of ad requests multiplied by 1000."]
                    AdRequestsRpm,
                    #[doc = "Fraction of ad requests considered to be spam. Only available to premium accounts."]
                    AdRequestsSpamRatio,
                    #[doc = "Number of ad views per impression."]
                    AdsPerImpression,
                    #[doc = "Number of times a user clicked on a standard content ad."]
                    Clicks,
                    #[doc = "Fraction of clicks considered to be spam. Only available to premium accounts."]
                    ClicksSpamRatio,
                    #[doc = "Amount the publisher earns each time a user clicks on an ad. CPC is calculated by dividing the estimated revenue by the number of clicks received."]
                    CostPerClick,
                    #[doc = "Estimated earnings of the publisher. Note that earnings up to yesterday are accurate, more recent earnings are estimated due to the possibility of spam, or exchange rate fluctuations."]
                    EstimatedEarnings,
                    #[doc = "Impressions. An impression is counted for each ad request where at least one ad has been downloaded to the user’s device and has begun to load. It is the number of ad units (for content ads) or search queries (for search ads) that showed ads."]
                    Impressions,
                    #[doc = "Ratio of IMPRESSIONS that resulted in a click."]
                    ImpressionsCtr,
                    #[doc = "Revenue per thousand ad impressions. This is calculated by dividing estimated revenue by the number of ad impressions multiplied by 1000."]
                    ImpressionsRpm,
                    #[doc = "Fraction of impressions considered to be spam. Only available to premium accounts."]
                    ImpressionsSpamRatio,
                    #[doc = "Ads shown. Different ad formats will display varying numbers of ads. For example, a vertical banner may consist of 2 or more ads. Also, the number of ads in an ad unit may vary depending on whether the ad unit is displaying standard text ads, expanded text ads or image ads."]
                    IndividualAdImpressions,
                    #[doc = "Ratio of individual ad impressions that resulted in a click."]
                    IndividualAdImpressionsCtr,
                    #[doc = "Revenue per thousand individual ad impressions. This is calculated by dividing estimated revenue by the number of individual ad impressions multiplied by 1000."]
                    IndividualAdImpressionsRpm,
                    #[doc = "Fraction of ad impressions considered to be spam. Only available to premium accounts."]
                    IndividualAdImpressionsSpamRatio,
                    #[doc = "Requests that returned at least one ad."]
                    MatchedAdRequests,
                    #[doc = "Ratio of clicks to matched requests."]
                    MatchedAdRequestsCtr,
                    #[doc = "Revenue per thousand matched ad requests. This is calculated by dividing estimated revenue by the number of matched ad requests multiplied by 1000."]
                    MatchedAdRequestsRpm,
                    #[doc = "Fraction of ad requests that returned ads considered to be spam. Only available to premium accounts."]
                    MatchedAdRequestsSpamRatio,
                    #[doc = "Unspecified metric."]
                    MetricUnspecified,
                    #[doc = "Number of page views."]
                    PageViews,
                    #[doc = "Ratio of individual page views that resulted in a click."]
                    PageViewsCtr,
                    #[doc = "Revenue per thousand page views. This is calculated by dividing the estimated revenue by the number of page views multiplied by 1000."]
                    PageViewsRpm,
                    #[doc = "Fraction of page views considered to be spam. Only available to premium accounts."]
                    PageViewsSpamRatio,
                    #[doc = "Total earnings are the gross estimated earnings from revenue shared traffic before any parent and child account revenue share is applied."]
                    TotalEarnings,
                    #[doc = "Impressions. An impression is counted for each ad request where at least one ad has been downloaded to the user’s device and has begun to load. It is the number of ad units (for content ads) or search queries (for search ads) that showed ads."]
                    TotalImpressions,
                    #[doc = "Number of results pages."]
                    WebsearchResultPages,
                }
                impl GenerateCsvMetricsItems {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            GenerateCsvMetricsItems::ActiveViewMeasurability => {
                                "ACTIVE_VIEW_MEASURABILITY"
                            }
                            GenerateCsvMetricsItems::ActiveViewTime => "ACTIVE_VIEW_TIME",
                            GenerateCsvMetricsItems::ActiveViewViewability => {
                                "ACTIVE_VIEW_VIEWABILITY"
                            }
                            GenerateCsvMetricsItems::AdRequests => "AD_REQUESTS",
                            GenerateCsvMetricsItems::AdRequestsCoverage => "AD_REQUESTS_COVERAGE",
                            GenerateCsvMetricsItems::AdRequestsCtr => "AD_REQUESTS_CTR",
                            GenerateCsvMetricsItems::AdRequestsRpm => "AD_REQUESTS_RPM",
                            GenerateCsvMetricsItems::AdRequestsSpamRatio => {
                                "AD_REQUESTS_SPAM_RATIO"
                            }
                            GenerateCsvMetricsItems::AdsPerImpression => "ADS_PER_IMPRESSION",
                            GenerateCsvMetricsItems::Clicks => "CLICKS",
                            GenerateCsvMetricsItems::ClicksSpamRatio => "CLICKS_SPAM_RATIO",
                            GenerateCsvMetricsItems::CostPerClick => "COST_PER_CLICK",
                            GenerateCsvMetricsItems::EstimatedEarnings => "ESTIMATED_EARNINGS",
                            GenerateCsvMetricsItems::Impressions => "IMPRESSIONS",
                            GenerateCsvMetricsItems::ImpressionsCtr => "IMPRESSIONS_CTR",
                            GenerateCsvMetricsItems::ImpressionsRpm => "IMPRESSIONS_RPM",
                            GenerateCsvMetricsItems::ImpressionsSpamRatio => {
                                "IMPRESSIONS_SPAM_RATIO"
                            }
                            GenerateCsvMetricsItems::IndividualAdImpressions => {
                                "INDIVIDUAL_AD_IMPRESSIONS"
                            }
                            GenerateCsvMetricsItems::IndividualAdImpressionsCtr => {
                                "INDIVIDUAL_AD_IMPRESSIONS_CTR"
                            }
                            GenerateCsvMetricsItems::IndividualAdImpressionsRpm => {
                                "INDIVIDUAL_AD_IMPRESSIONS_RPM"
                            }
                            GenerateCsvMetricsItems::IndividualAdImpressionsSpamRatio => {
                                "INDIVIDUAL_AD_IMPRESSIONS_SPAM_RATIO"
                            }
                            GenerateCsvMetricsItems::MatchedAdRequests => "MATCHED_AD_REQUESTS",
                            GenerateCsvMetricsItems::MatchedAdRequestsCtr => {
                                "MATCHED_AD_REQUESTS_CTR"
                            }
                            GenerateCsvMetricsItems::MatchedAdRequestsRpm => {
                                "MATCHED_AD_REQUESTS_RPM"
                            }
                            GenerateCsvMetricsItems::MatchedAdRequestsSpamRatio => {
                                "MATCHED_AD_REQUESTS_SPAM_RATIO"
                            }
                            GenerateCsvMetricsItems::MetricUnspecified => "METRIC_UNSPECIFIED",
                            GenerateCsvMetricsItems::PageViews => "PAGE_VIEWS",
                            GenerateCsvMetricsItems::PageViewsCtr => "PAGE_VIEWS_CTR",
                            GenerateCsvMetricsItems::PageViewsRpm => "PAGE_VIEWS_RPM",
                            GenerateCsvMetricsItems::PageViewsSpamRatio => "PAGE_VIEWS_SPAM_RATIO",
                            GenerateCsvMetricsItems::TotalEarnings => "TOTAL_EARNINGS",
                            GenerateCsvMetricsItems::TotalImpressions => "TOTAL_IMPRESSIONS",
                            GenerateCsvMetricsItems::WebsearchResultPages => {
                                "WEBSEARCH_RESULT_PAGES"
                            }
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for GenerateCsvMetricsItems {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for GenerateCsvMetricsItems {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<GenerateCsvMetricsItems, ()> {
                        Ok(match s {
                            "ACTIVE_VIEW_MEASURABILITY" => {
                                GenerateCsvMetricsItems::ActiveViewMeasurability
                            }
                            "ACTIVE_VIEW_TIME" => GenerateCsvMetricsItems::ActiveViewTime,
                            "ACTIVE_VIEW_VIEWABILITY" => {
                                GenerateCsvMetricsItems::ActiveViewViewability
                            }
                            "AD_REQUESTS" => GenerateCsvMetricsItems::AdRequests,
                            "AD_REQUESTS_COVERAGE" => GenerateCsvMetricsItems::AdRequestsCoverage,
                            "AD_REQUESTS_CTR" => GenerateCsvMetricsItems::AdRequestsCtr,
                            "AD_REQUESTS_RPM" => GenerateCsvMetricsItems::AdRequestsRpm,
                            "AD_REQUESTS_SPAM_RATIO" => {
                                GenerateCsvMetricsItems::AdRequestsSpamRatio
                            }
                            "ADS_PER_IMPRESSION" => GenerateCsvMetricsItems::AdsPerImpression,
                            "CLICKS" => GenerateCsvMetricsItems::Clicks,
                            "CLICKS_SPAM_RATIO" => GenerateCsvMetricsItems::ClicksSpamRatio,
                            "COST_PER_CLICK" => GenerateCsvMetricsItems::CostPerClick,
                            "ESTIMATED_EARNINGS" => GenerateCsvMetricsItems::EstimatedEarnings,
                            "IMPRESSIONS" => GenerateCsvMetricsItems::Impressions,
                            "IMPRESSIONS_CTR" => GenerateCsvMetricsItems::ImpressionsCtr,
                            "IMPRESSIONS_RPM" => GenerateCsvMetricsItems::ImpressionsRpm,
                            "IMPRESSIONS_SPAM_RATIO" => {
                                GenerateCsvMetricsItems::ImpressionsSpamRatio
                            }
                            "INDIVIDUAL_AD_IMPRESSIONS" => {
                                GenerateCsvMetricsItems::IndividualAdImpressions
                            }
                            "INDIVIDUAL_AD_IMPRESSIONS_CTR" => {
                                GenerateCsvMetricsItems::IndividualAdImpressionsCtr
                            }
                            "INDIVIDUAL_AD_IMPRESSIONS_RPM" => {
                                GenerateCsvMetricsItems::IndividualAdImpressionsRpm
                            }
                            "INDIVIDUAL_AD_IMPRESSIONS_SPAM_RATIO" => {
                                GenerateCsvMetricsItems::IndividualAdImpressionsSpamRatio
                            }
                            "MATCHED_AD_REQUESTS" => GenerateCsvMetricsItems::MatchedAdRequests,
                            "MATCHED_AD_REQUESTS_CTR" => {
                                GenerateCsvMetricsItems::MatchedAdRequestsCtr
                            }
                            "MATCHED_AD_REQUESTS_RPM" => {
                                GenerateCsvMetricsItems::MatchedAdRequestsRpm
                            }
                            "MATCHED_AD_REQUESTS_SPAM_RATIO" => {
                                GenerateCsvMetricsItems::MatchedAdRequestsSpamRatio
                            }
                            "METRIC_UNSPECIFIED" => GenerateCsvMetricsItems::MetricUnspecified,
                            "PAGE_VIEWS" => GenerateCsvMetricsItems::PageViews,
                            "PAGE_VIEWS_CTR" => GenerateCsvMetricsItems::PageViewsCtr,
                            "PAGE_VIEWS_RPM" => GenerateCsvMetricsItems::PageViewsRpm,
                            "PAGE_VIEWS_SPAM_RATIO" => GenerateCsvMetricsItems::PageViewsSpamRatio,
                            "TOTAL_EARNINGS" => GenerateCsvMetricsItems::TotalEarnings,
                            "TOTAL_IMPRESSIONS" => GenerateCsvMetricsItems::TotalImpressions,
                            "WEBSEARCH_RESULT_PAGES" => {
                                GenerateCsvMetricsItems::WebsearchResultPages
                            }
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for GenerateCsvMetricsItems {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for GenerateCsvMetricsItems {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for GenerateCsvMetricsItems {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "ACTIVE_VIEW_MEASURABILITY" => {
                                GenerateCsvMetricsItems::ActiveViewMeasurability
                            }
                            "ACTIVE_VIEW_TIME" => GenerateCsvMetricsItems::ActiveViewTime,
                            "ACTIVE_VIEW_VIEWABILITY" => {
                                GenerateCsvMetricsItems::ActiveViewViewability
                            }
                            "AD_REQUESTS" => GenerateCsvMetricsItems::AdRequests,
                            "AD_REQUESTS_COVERAGE" => GenerateCsvMetricsItems::AdRequestsCoverage,
                            "AD_REQUESTS_CTR" => GenerateCsvMetricsItems::AdRequestsCtr,
                            "AD_REQUESTS_RPM" => GenerateCsvMetricsItems::AdRequestsRpm,
                            "AD_REQUESTS_SPAM_RATIO" => {
                                GenerateCsvMetricsItems::AdRequestsSpamRatio
                            }
                            "ADS_PER_IMPRESSION" => GenerateCsvMetricsItems::AdsPerImpression,
                            "CLICKS" => GenerateCsvMetricsItems::Clicks,
                            "CLICKS_SPAM_RATIO" => GenerateCsvMetricsItems::ClicksSpamRatio,
                            "COST_PER_CLICK" => GenerateCsvMetricsItems::CostPerClick,
                            "ESTIMATED_EARNINGS" => GenerateCsvMetricsItems::EstimatedEarnings,
                            "IMPRESSIONS" => GenerateCsvMetricsItems::Impressions,
                            "IMPRESSIONS_CTR" => GenerateCsvMetricsItems::ImpressionsCtr,
                            "IMPRESSIONS_RPM" => GenerateCsvMetricsItems::ImpressionsRpm,
                            "IMPRESSIONS_SPAM_RATIO" => {
                                GenerateCsvMetricsItems::ImpressionsSpamRatio
                            }
                            "INDIVIDUAL_AD_IMPRESSIONS" => {
                                GenerateCsvMetricsItems::IndividualAdImpressions
                            }
                            "INDIVIDUAL_AD_IMPRESSIONS_CTR" => {
                                GenerateCsvMetricsItems::IndividualAdImpressionsCtr
                            }
                            "INDIVIDUAL_AD_IMPRESSIONS_RPM" => {
                                GenerateCsvMetricsItems::IndividualAdImpressionsRpm
                            }
                            "INDIVIDUAL_AD_IMPRESSIONS_SPAM_RATIO" => {
                                GenerateCsvMetricsItems::IndividualAdImpressionsSpamRatio
                            }
                            "MATCHED_AD_REQUESTS" => GenerateCsvMetricsItems::MatchedAdRequests,
                            "MATCHED_AD_REQUESTS_CTR" => {
                                GenerateCsvMetricsItems::MatchedAdRequestsCtr
                            }
                            "MATCHED_AD_REQUESTS_RPM" => {
                                GenerateCsvMetricsItems::MatchedAdRequestsRpm
                            }
                            "MATCHED_AD_REQUESTS_SPAM_RATIO" => {
                                GenerateCsvMetricsItems::MatchedAdRequestsSpamRatio
                            }
                            "METRIC_UNSPECIFIED" => GenerateCsvMetricsItems::MetricUnspecified,
                            "PAGE_VIEWS" => GenerateCsvMetricsItems::PageViews,
                            "PAGE_VIEWS_CTR" => GenerateCsvMetricsItems::PageViewsCtr,
                            "PAGE_VIEWS_RPM" => GenerateCsvMetricsItems::PageViewsRpm,
                            "PAGE_VIEWS_SPAM_RATIO" => GenerateCsvMetricsItems::PageViewsSpamRatio,
                            "TOTAL_EARNINGS" => GenerateCsvMetricsItems::TotalEarnings,
                            "TOTAL_IMPRESSIONS" => GenerateCsvMetricsItems::TotalImpressions,
                            "WEBSEARCH_RESULT_PAGES" => {
                                GenerateCsvMetricsItems::WebsearchResultPages
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
                impl ::google_field_selector::FieldSelector for GenerateCsvMetricsItems {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for GenerateCsvMetricsItems {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum GenerateCsvReportingTimeZone {
                    #[doc = "Use the account timezone in the report."]
                    AccountTimeZone,
                    #[doc = "Use the Google timezone in the report (America/Los_Angeles)."]
                    GoogleTimeZone,
                    #[doc = "Unspecified timezone."]
                    ReportingTimeZoneUnspecified,
                }
                impl GenerateCsvReportingTimeZone {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            GenerateCsvReportingTimeZone::AccountTimeZone => "ACCOUNT_TIME_ZONE",
                            GenerateCsvReportingTimeZone::GoogleTimeZone => "GOOGLE_TIME_ZONE",
                            GenerateCsvReportingTimeZone::ReportingTimeZoneUnspecified => {
                                "REPORTING_TIME_ZONE_UNSPECIFIED"
                            }
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for GenerateCsvReportingTimeZone {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for GenerateCsvReportingTimeZone {
                    type Err = ();
                    fn from_str(
                        s: &str,
                    ) -> ::std::result::Result<GenerateCsvReportingTimeZone, ()>
                    {
                        Ok(match s {
                            "ACCOUNT_TIME_ZONE" => GenerateCsvReportingTimeZone::AccountTimeZone,
                            "GOOGLE_TIME_ZONE" => GenerateCsvReportingTimeZone::GoogleTimeZone,
                            "REPORTING_TIME_ZONE_UNSPECIFIED" => {
                                GenerateCsvReportingTimeZone::ReportingTimeZoneUnspecified
                            }
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for GenerateCsvReportingTimeZone {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for GenerateCsvReportingTimeZone {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for GenerateCsvReportingTimeZone {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "ACCOUNT_TIME_ZONE" => GenerateCsvReportingTimeZone::AccountTimeZone,
                            "GOOGLE_TIME_ZONE" => GenerateCsvReportingTimeZone::GoogleTimeZone,
                            "REPORTING_TIME_ZONE_UNSPECIFIED" => {
                                GenerateCsvReportingTimeZone::ReportingTimeZoneUnspecified
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
                impl ::google_field_selector::FieldSelector for GenerateCsvReportingTimeZone {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for GenerateCsvReportingTimeZone {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct ReportsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> ReportsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Generates an ad hoc report."]
                pub fn generate(&self, account: impl Into<String>) -> GenerateRequestBuilder {
                    GenerateRequestBuilder {
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
                        account: account.into(),
                        currency_code: None,
                        date_range: None,
                        dimensions: None,
                        end_date_day: None,
                        end_date_month: None,
                        end_date_year: None,
                        filters: None,
                        language_code: None,
                        limit: None,
                        metrics: None,
                        order_by: None,
                        reporting_time_zone: None,
                        start_date_day: None,
                        start_date_month: None,
                        start_date_year: None,
                    }
                }
                #[doc = "Generates a csv formatted ad hoc report."]
                pub fn generate_csv(
                    &self,
                    account: impl Into<String>,
                ) -> GenerateCsvRequestBuilder {
                    GenerateCsvRequestBuilder {
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
                        account: account.into(),
                        currency_code: None,
                        date_range: None,
                        dimensions: None,
                        end_date_day: None,
                        end_date_month: None,
                        end_date_year: None,
                        filters: None,
                        language_code: None,
                        limit: None,
                        metrics: None,
                        order_by: None,
                        reporting_time_zone: None,
                        start_date_day: None,
                        start_date_month: None,
                        start_date_year: None,
                    }
                }
                #[doc = "Actions that can be performed on the saved resource"]
                pub fn saved(&self) -> crate::resources::accounts::reports::saved::SavedActions {
                    crate::resources::accounts::reports::saved::SavedActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            #[doc = "Created via [ReportsActions::generate()](struct.ReportsActions.html#method.generate)"]
            #[derive(Debug, Clone)]
            pub struct GenerateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                account: String,
                currency_code: Option<String>,
                date_range: Option<crate::resources::accounts::reports::params::GenerateDateRange>,
                dimensions: Option<
                    Vec<crate::resources::accounts::reports::params::GenerateDimensionsItems>,
                >,
                end_date_day: Option<i32>,
                end_date_month: Option<i32>,
                end_date_year: Option<i32>,
                filters: Option<Vec<String>>,
                language_code: Option<String>,
                limit: Option<i32>,
                metrics:
                    Option<Vec<crate::resources::accounts::reports::params::GenerateMetricsItems>>,
                order_by: Option<Vec<String>>,
                reporting_time_zone:
                    Option<crate::resources::accounts::reports::params::GenerateReportingTimeZone>,
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
            impl<'a> GenerateRequestBuilder<'a> {
                #[doc = "The [ISO-4217 currency code](https://en.wikipedia.org/wiki/ISO_4217) to use when reporting on monetary metrics. Defaults to the account's currency if not set."]
                pub fn currency_code(mut self, value: impl Into<String>) -> Self {
                    self.currency_code = Some(value.into());
                    self
                }
                #[doc = "Date range of the report, if unset the range will be considered CUSTOM."]
                pub fn date_range(
                    mut self,
                    value: crate::resources::accounts::reports::params::GenerateDateRange,
                ) -> Self {
                    self.date_range = Some(value);
                    self
                }
                #[doc = "Dimensions to base the report on."]
                pub fn dimensions(
                    mut self,
                    value: impl Into<
                        Vec<crate::resources::accounts::reports::params::GenerateDimensionsItems>,
                    >,
                ) -> Self {
                    self.dimensions = Some(value.into());
                    self
                }
                #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
                pub fn end_date_day(mut self, value: i32) -> Self {
                    self.end_date_day = Some(value);
                    self
                }
                #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
                pub fn end_date_month(mut self, value: i32) -> Self {
                    self.end_date_month = Some(value);
                    self
                }
                #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
                pub fn end_date_year(mut self, value: i32) -> Self {
                    self.end_date_year = Some(value);
                    self
                }
                #[doc = "Filters to be run on the report."]
                pub fn filters(mut self, value: impl Into<Vec<String>>) -> Self {
                    self.filters = Some(value.into());
                    self
                }
                #[doc = "The language to use for translating report output. If unspecified, this defaults to English (\"en\"). If the given language is not supported, report output will be returned in English. The language is specified as an [IETF BCP-47 language code](https://en.wikipedia.org/wiki/IETF_language_tag)."]
                pub fn language_code(mut self, value: impl Into<String>) -> Self {
                    self.language_code = Some(value.into());
                    self
                }
                #[doc = "The maximum number of rows of report data to return. Reports producing more rows than the requested limit will be truncated. If unset, this defaults to 100,000 rows for `Reports.GenerateReport` and 1,000,000 rows for `Reports.GenerateCsvReport`, which are also the maximum values permitted here. Report truncation can be identified (for `Reports.GenerateReport` only) by comparing the number of rows returned to the value returned in `total_matched_rows`."]
                pub fn limit(mut self, value: i32) -> Self {
                    self.limit = Some(value);
                    self
                }
                #[doc = "Required. Reporting metrics."]
                pub fn metrics(
                    mut self,
                    value: impl Into<
                        Vec<crate::resources::accounts::reports::params::GenerateMetricsItems>,
                    >,
                ) -> Self {
                    self.metrics = Some(value.into());
                    self
                }
                #[doc = "The name of a dimension or metric to sort the resulting report on, can be prefixed with \"+\" to sort ascending or \"-\" to sort descending. If no prefix is specified, the column is sorted ascending."]
                pub fn order_by(mut self, value: impl Into<Vec<String>>) -> Self {
                    self.order_by = Some(value.into());
                    self
                }
                #[doc = "Timezone in which to generate the report. If unspecified, this defaults to the account timezone. For more information, see [changing the time zone of your reports](https://support.google.com/adsense/answer/9830725)."]
                pub fn reporting_time_zone(
                    mut self,
                    value: crate::resources::accounts::reports::params::GenerateReportingTimeZone,
                ) -> Self {
                    self.reporting_time_zone = Some(value);
                    self
                }
                #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
                pub fn start_date_day(mut self, value: i32) -> Self {
                    self.start_date_day = Some(value);
                    self
                }
                #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
                pub fn start_date_month(mut self, value: i32) -> Self {
                    self.start_date_month = Some(value);
                    self
                }
                #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::ReportResult, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ReportResult, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adsense.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.account;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/reports:generate");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("currencyCode", &self.currency_code)]);
                    req = req.query(&[("dateRange", &self.date_range)]);
                    for value in self.dimensions.iter().flatten() {
                        req = req.query(&[("dimensions", value)]);
                    }
                    req = req.query(&[("endDate.day", &self.end_date_day)]);
                    req = req.query(&[("endDate.month", &self.end_date_month)]);
                    req = req.query(&[("endDate.year", &self.end_date_year)]);
                    for value in self.filters.iter().flatten() {
                        req = req.query(&[("filters", value)]);
                    }
                    req = req.query(&[("languageCode", &self.language_code)]);
                    req = req.query(&[("limit", &self.limit)]);
                    for value in self.metrics.iter().flatten() {
                        req = req.query(&[("metrics", value)]);
                    }
                    for value in self.order_by.iter().flatten() {
                        req = req.query(&[("orderBy", value)]);
                    }
                    req = req.query(&[("reportingTimeZone", &self.reporting_time_zone)]);
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [ReportsActions::generate_csv()](struct.ReportsActions.html#method.generate_csv)"]
            #[derive(Debug, Clone)]
            pub struct GenerateCsvRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                account: String,
                currency_code: Option<String>,
                date_range:
                    Option<crate::resources::accounts::reports::params::GenerateCsvDateRange>,
                dimensions: Option<
                    Vec<crate::resources::accounts::reports::params::GenerateCsvDimensionsItems>,
                >,
                end_date_day: Option<i32>,
                end_date_month: Option<i32>,
                end_date_year: Option<i32>,
                filters: Option<Vec<String>>,
                language_code: Option<String>,
                limit: Option<i32>,
                metrics: Option<
                    Vec<crate::resources::accounts::reports::params::GenerateCsvMetricsItems>,
                >,
                order_by: Option<Vec<String>>,
                reporting_time_zone: Option<
                    crate::resources::accounts::reports::params::GenerateCsvReportingTimeZone,
                >,
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
            impl<'a> GenerateCsvRequestBuilder<'a> {
                #[doc = "The [ISO-4217 currency code](https://en.wikipedia.org/wiki/ISO_4217) to use when reporting on monetary metrics. Defaults to the account's currency if not set."]
                pub fn currency_code(mut self, value: impl Into<String>) -> Self {
                    self.currency_code = Some(value.into());
                    self
                }
                #[doc = "Date range of the report, if unset the range will be considered CUSTOM."]
                pub fn date_range(
                    mut self,
                    value: crate::resources::accounts::reports::params::GenerateCsvDateRange,
                ) -> Self {
                    self.date_range = Some(value);
                    self
                }
                #[doc = "Dimensions to base the report on."]
                pub fn dimensions(
                    mut self,
                    value: impl Into<
                        Vec<
                            crate::resources::accounts::reports::params::GenerateCsvDimensionsItems,
                        >,
                    >,
                ) -> Self {
                    self.dimensions = Some(value.into());
                    self
                }
                #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
                pub fn end_date_day(mut self, value: i32) -> Self {
                    self.end_date_day = Some(value);
                    self
                }
                #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
                pub fn end_date_month(mut self, value: i32) -> Self {
                    self.end_date_month = Some(value);
                    self
                }
                #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
                pub fn end_date_year(mut self, value: i32) -> Self {
                    self.end_date_year = Some(value);
                    self
                }
                #[doc = "Filters to be run on the report."]
                pub fn filters(mut self, value: impl Into<Vec<String>>) -> Self {
                    self.filters = Some(value.into());
                    self
                }
                #[doc = "The language to use for translating report output. If unspecified, this defaults to English (\"en\"). If the given language is not supported, report output will be returned in English. The language is specified as an [IETF BCP-47 language code](https://en.wikipedia.org/wiki/IETF_language_tag)."]
                pub fn language_code(mut self, value: impl Into<String>) -> Self {
                    self.language_code = Some(value.into());
                    self
                }
                #[doc = "The maximum number of rows of report data to return. Reports producing more rows than the requested limit will be truncated. If unset, this defaults to 100,000 rows for `Reports.GenerateReport` and 1,000,000 rows for `Reports.GenerateCsvReport`, which are also the maximum values permitted here. Report truncation can be identified (for `Reports.GenerateReport` only) by comparing the number of rows returned to the value returned in `total_matched_rows`."]
                pub fn limit(mut self, value: i32) -> Self {
                    self.limit = Some(value);
                    self
                }
                #[doc = "Required. Reporting metrics."]
                pub fn metrics(
                    mut self,
                    value: impl Into<
                        Vec<crate::resources::accounts::reports::params::GenerateCsvMetricsItems>,
                    >,
                ) -> Self {
                    self.metrics = Some(value.into());
                    self
                }
                #[doc = "The name of a dimension or metric to sort the resulting report on, can be prefixed with \"+\" to sort ascending or \"-\" to sort descending. If no prefix is specified, the column is sorted ascending."]
                pub fn order_by(mut self, value: impl Into<Vec<String>>) -> Self {
                    self.order_by = Some(value.into());
                    self
                }
                #[doc = "Timezone in which to generate the report. If unspecified, this defaults to the account timezone. For more information, see [changing the time zone of your reports](https://support.google.com/adsense/answer/9830725)."]
                pub fn reporting_time_zone(
                    mut self,
                    value : crate :: resources :: accounts :: reports :: params :: GenerateCsvReportingTimeZone,
                ) -> Self {
                    self.reporting_time_zone = Some(value);
                    self
                }
                #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
                pub fn start_date_day(mut self, value: i32) -> Self {
                    self.start_date_day = Some(value);
                    self
                }
                #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
                pub fn start_date_month(mut self, value: i32) -> Self {
                    self.start_date_month = Some(value);
                    self
                }
                #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::HttpBody, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::HttpBody, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adsense.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.account;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/reports:generateCsv");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("currencyCode", &self.currency_code)]);
                    req = req.query(&[("dateRange", &self.date_range)]);
                    for value in self.dimensions.iter().flatten() {
                        req = req.query(&[("dimensions", value)]);
                    }
                    req = req.query(&[("endDate.day", &self.end_date_day)]);
                    req = req.query(&[("endDate.month", &self.end_date_month)]);
                    req = req.query(&[("endDate.year", &self.end_date_year)]);
                    for value in self.filters.iter().flatten() {
                        req = req.query(&[("filters", value)]);
                    }
                    req = req.query(&[("languageCode", &self.language_code)]);
                    req = req.query(&[("limit", &self.limit)]);
                    for value in self.metrics.iter().flatten() {
                        req = req.query(&[("metrics", value)]);
                    }
                    for value in self.order_by.iter().flatten() {
                        req = req.query(&[("orderBy", value)]);
                    }
                    req = req.query(&[("reportingTimeZone", &self.reporting_time_zone)]);
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            pub mod saved {
                pub mod params {
                    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                    pub enum GenerateDateRange {
                        #[doc = "A custom date range specified using the `start_date` and `end_date` fields. This is the default if no ReportingDateRange is provided."]
                        Custom,
                        #[doc = "Last 30 days, excluding current day."]
                        Last30Days,
                        #[doc = "Last 7 days, excluding current day."]
                        Last7Days,
                        #[doc = "From the start of the current month to the current day. e.g. if the current date is 2020-03-12 then the range will be [2020-03-01, 2020-03-12]."]
                        MonthToDate,
                        #[doc = "Unspecified date range."]
                        ReportingDateRangeUnspecified,
                        #[doc = "Current day."]
                        Today,
                        #[doc = "From the start of the current year to the current day. e.g. if the current date is 2020-03-12 then the range will be [2020-01-01, 2020-03-12]."]
                        YearToDate,
                        #[doc = "Yesterday."]
                        Yesterday,
                    }
                    impl GenerateDateRange {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                GenerateDateRange::Custom => "CUSTOM",
                                GenerateDateRange::Last30Days => "LAST_30_DAYS",
                                GenerateDateRange::Last7Days => "LAST_7_DAYS",
                                GenerateDateRange::MonthToDate => "MONTH_TO_DATE",
                                GenerateDateRange::ReportingDateRangeUnspecified => {
                                    "REPORTING_DATE_RANGE_UNSPECIFIED"
                                }
                                GenerateDateRange::Today => "TODAY",
                                GenerateDateRange::YearToDate => "YEAR_TO_DATE",
                                GenerateDateRange::Yesterday => "YESTERDAY",
                            }
                        }
                    }
                    impl ::std::convert::AsRef<str> for GenerateDateRange {
                        fn as_ref(&self) -> &str {
                            self.as_str()
                        }
                    }
                    impl ::std::str::FromStr for GenerateDateRange {
                        type Err = ();
                        fn from_str(s: &str) -> ::std::result::Result<GenerateDateRange, ()> {
                            Ok(match s {
                                "CUSTOM" => GenerateDateRange::Custom,
                                "LAST_30_DAYS" => GenerateDateRange::Last30Days,
                                "LAST_7_DAYS" => GenerateDateRange::Last7Days,
                                "MONTH_TO_DATE" => GenerateDateRange::MonthToDate,
                                "REPORTING_DATE_RANGE_UNSPECIFIED" => {
                                    GenerateDateRange::ReportingDateRangeUnspecified
                                }
                                "TODAY" => GenerateDateRange::Today,
                                "YEAR_TO_DATE" => GenerateDateRange::YearToDate,
                                "YESTERDAY" => GenerateDateRange::Yesterday,
                                _ => return Err(()),
                            })
                        }
                    }
                    impl ::std::fmt::Display for GenerateDateRange {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for GenerateDateRange {
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
                    impl<'de> ::serde::Deserialize<'de> for GenerateDateRange {
                        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            let value: &'de str = <&str>::deserialize(deserializer)?;
                            Ok(match value {
                                "CUSTOM" => GenerateDateRange::Custom,
                                "LAST_30_DAYS" => GenerateDateRange::Last30Days,
                                "LAST_7_DAYS" => GenerateDateRange::Last7Days,
                                "MONTH_TO_DATE" => GenerateDateRange::MonthToDate,
                                "REPORTING_DATE_RANGE_UNSPECIFIED" => {
                                    GenerateDateRange::ReportingDateRangeUnspecified
                                }
                                "TODAY" => GenerateDateRange::Today,
                                "YEAR_TO_DATE" => GenerateDateRange::YearToDate,
                                "YESTERDAY" => GenerateDateRange::Yesterday,
                                _ => {
                                    return Err(::serde::de::Error::custom(format!(
                                        "invalid enum for #name: {}",
                                        value
                                    )))
                                }
                            })
                        }
                    }
                    impl ::google_field_selector::FieldSelector for GenerateDateRange {
                        fn fields() -> Vec<::google_field_selector::Field> {
                            Vec::new()
                        }
                    }
                    impl ::google_field_selector::ToFieldType for GenerateDateRange {
                        fn field_type() -> ::google_field_selector::FieldType {
                            ::google_field_selector::FieldType::Leaf
                        }
                    }
                    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                    pub enum GenerateReportingTimeZone {
                        #[doc = "Use the account timezone in the report."]
                        AccountTimeZone,
                        #[doc = "Use the Google timezone in the report (America/Los_Angeles)."]
                        GoogleTimeZone,
                        #[doc = "Unspecified timezone."]
                        ReportingTimeZoneUnspecified,
                    }
                    impl GenerateReportingTimeZone {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                GenerateReportingTimeZone::AccountTimeZone => "ACCOUNT_TIME_ZONE",
                                GenerateReportingTimeZone::GoogleTimeZone => "GOOGLE_TIME_ZONE",
                                GenerateReportingTimeZone::ReportingTimeZoneUnspecified => {
                                    "REPORTING_TIME_ZONE_UNSPECIFIED"
                                }
                            }
                        }
                    }
                    impl ::std::convert::AsRef<str> for GenerateReportingTimeZone {
                        fn as_ref(&self) -> &str {
                            self.as_str()
                        }
                    }
                    impl ::std::str::FromStr for GenerateReportingTimeZone {
                        type Err = ();
                        fn from_str(
                            s: &str,
                        ) -> ::std::result::Result<GenerateReportingTimeZone, ()>
                        {
                            Ok(match s {
                                "ACCOUNT_TIME_ZONE" => GenerateReportingTimeZone::AccountTimeZone,
                                "GOOGLE_TIME_ZONE" => GenerateReportingTimeZone::GoogleTimeZone,
                                "REPORTING_TIME_ZONE_UNSPECIFIED" => {
                                    GenerateReportingTimeZone::ReportingTimeZoneUnspecified
                                }
                                _ => return Err(()),
                            })
                        }
                    }
                    impl ::std::fmt::Display for GenerateReportingTimeZone {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for GenerateReportingTimeZone {
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
                    impl<'de> ::serde::Deserialize<'de> for GenerateReportingTimeZone {
                        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            let value: &'de str = <&str>::deserialize(deserializer)?;
                            Ok(match value {
                                "ACCOUNT_TIME_ZONE" => GenerateReportingTimeZone::AccountTimeZone,
                                "GOOGLE_TIME_ZONE" => GenerateReportingTimeZone::GoogleTimeZone,
                                "REPORTING_TIME_ZONE_UNSPECIFIED" => {
                                    GenerateReportingTimeZone::ReportingTimeZoneUnspecified
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
                    impl ::google_field_selector::FieldSelector for GenerateReportingTimeZone {
                        fn fields() -> Vec<::google_field_selector::Field> {
                            Vec::new()
                        }
                    }
                    impl ::google_field_selector::ToFieldType for GenerateReportingTimeZone {
                        fn field_type() -> ::google_field_selector::FieldType {
                            ::google_field_selector::FieldType::Leaf
                        }
                    }
                    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                    pub enum GenerateCsvDateRange {
                        #[doc = "A custom date range specified using the `start_date` and `end_date` fields. This is the default if no ReportingDateRange is provided."]
                        Custom,
                        #[doc = "Last 30 days, excluding current day."]
                        Last30Days,
                        #[doc = "Last 7 days, excluding current day."]
                        Last7Days,
                        #[doc = "From the start of the current month to the current day. e.g. if the current date is 2020-03-12 then the range will be [2020-03-01, 2020-03-12]."]
                        MonthToDate,
                        #[doc = "Unspecified date range."]
                        ReportingDateRangeUnspecified,
                        #[doc = "Current day."]
                        Today,
                        #[doc = "From the start of the current year to the current day. e.g. if the current date is 2020-03-12 then the range will be [2020-01-01, 2020-03-12]."]
                        YearToDate,
                        #[doc = "Yesterday."]
                        Yesterday,
                    }
                    impl GenerateCsvDateRange {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                GenerateCsvDateRange::Custom => "CUSTOM",
                                GenerateCsvDateRange::Last30Days => "LAST_30_DAYS",
                                GenerateCsvDateRange::Last7Days => "LAST_7_DAYS",
                                GenerateCsvDateRange::MonthToDate => "MONTH_TO_DATE",
                                GenerateCsvDateRange::ReportingDateRangeUnspecified => {
                                    "REPORTING_DATE_RANGE_UNSPECIFIED"
                                }
                                GenerateCsvDateRange::Today => "TODAY",
                                GenerateCsvDateRange::YearToDate => "YEAR_TO_DATE",
                                GenerateCsvDateRange::Yesterday => "YESTERDAY",
                            }
                        }
                    }
                    impl ::std::convert::AsRef<str> for GenerateCsvDateRange {
                        fn as_ref(&self) -> &str {
                            self.as_str()
                        }
                    }
                    impl ::std::str::FromStr for GenerateCsvDateRange {
                        type Err = ();
                        fn from_str(s: &str) -> ::std::result::Result<GenerateCsvDateRange, ()> {
                            Ok(match s {
                                "CUSTOM" => GenerateCsvDateRange::Custom,
                                "LAST_30_DAYS" => GenerateCsvDateRange::Last30Days,
                                "LAST_7_DAYS" => GenerateCsvDateRange::Last7Days,
                                "MONTH_TO_DATE" => GenerateCsvDateRange::MonthToDate,
                                "REPORTING_DATE_RANGE_UNSPECIFIED" => {
                                    GenerateCsvDateRange::ReportingDateRangeUnspecified
                                }
                                "TODAY" => GenerateCsvDateRange::Today,
                                "YEAR_TO_DATE" => GenerateCsvDateRange::YearToDate,
                                "YESTERDAY" => GenerateCsvDateRange::Yesterday,
                                _ => return Err(()),
                            })
                        }
                    }
                    impl ::std::fmt::Display for GenerateCsvDateRange {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for GenerateCsvDateRange {
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
                    impl<'de> ::serde::Deserialize<'de> for GenerateCsvDateRange {
                        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            let value: &'de str = <&str>::deserialize(deserializer)?;
                            Ok(match value {
                                "CUSTOM" => GenerateCsvDateRange::Custom,
                                "LAST_30_DAYS" => GenerateCsvDateRange::Last30Days,
                                "LAST_7_DAYS" => GenerateCsvDateRange::Last7Days,
                                "MONTH_TO_DATE" => GenerateCsvDateRange::MonthToDate,
                                "REPORTING_DATE_RANGE_UNSPECIFIED" => {
                                    GenerateCsvDateRange::ReportingDateRangeUnspecified
                                }
                                "TODAY" => GenerateCsvDateRange::Today,
                                "YEAR_TO_DATE" => GenerateCsvDateRange::YearToDate,
                                "YESTERDAY" => GenerateCsvDateRange::Yesterday,
                                _ => {
                                    return Err(::serde::de::Error::custom(format!(
                                        "invalid enum for #name: {}",
                                        value
                                    )))
                                }
                            })
                        }
                    }
                    impl ::google_field_selector::FieldSelector for GenerateCsvDateRange {
                        fn fields() -> Vec<::google_field_selector::Field> {
                            Vec::new()
                        }
                    }
                    impl ::google_field_selector::ToFieldType for GenerateCsvDateRange {
                        fn field_type() -> ::google_field_selector::FieldType {
                            ::google_field_selector::FieldType::Leaf
                        }
                    }
                    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                    pub enum GenerateCsvReportingTimeZone {
                        #[doc = "Use the account timezone in the report."]
                        AccountTimeZone,
                        #[doc = "Use the Google timezone in the report (America/Los_Angeles)."]
                        GoogleTimeZone,
                        #[doc = "Unspecified timezone."]
                        ReportingTimeZoneUnspecified,
                    }
                    impl GenerateCsvReportingTimeZone {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                GenerateCsvReportingTimeZone::AccountTimeZone => {
                                    "ACCOUNT_TIME_ZONE"
                                }
                                GenerateCsvReportingTimeZone::GoogleTimeZone => "GOOGLE_TIME_ZONE",
                                GenerateCsvReportingTimeZone::ReportingTimeZoneUnspecified => {
                                    "REPORTING_TIME_ZONE_UNSPECIFIED"
                                }
                            }
                        }
                    }
                    impl ::std::convert::AsRef<str> for GenerateCsvReportingTimeZone {
                        fn as_ref(&self) -> &str {
                            self.as_str()
                        }
                    }
                    impl ::std::str::FromStr for GenerateCsvReportingTimeZone {
                        type Err = ();
                        fn from_str(
                            s: &str,
                        ) -> ::std::result::Result<GenerateCsvReportingTimeZone, ()>
                        {
                            Ok(match s {
                                "ACCOUNT_TIME_ZONE" => {
                                    GenerateCsvReportingTimeZone::AccountTimeZone
                                }
                                "GOOGLE_TIME_ZONE" => GenerateCsvReportingTimeZone::GoogleTimeZone,
                                "REPORTING_TIME_ZONE_UNSPECIFIED" => {
                                    GenerateCsvReportingTimeZone::ReportingTimeZoneUnspecified
                                }
                                _ => return Err(()),
                            })
                        }
                    }
                    impl ::std::fmt::Display for GenerateCsvReportingTimeZone {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for GenerateCsvReportingTimeZone {
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
                    impl<'de> ::serde::Deserialize<'de> for GenerateCsvReportingTimeZone {
                        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            let value: &'de str = <&str>::deserialize(deserializer)?;
                            Ok(match value {
                                "ACCOUNT_TIME_ZONE" => {
                                    GenerateCsvReportingTimeZone::AccountTimeZone
                                }
                                "GOOGLE_TIME_ZONE" => GenerateCsvReportingTimeZone::GoogleTimeZone,
                                "REPORTING_TIME_ZONE_UNSPECIFIED" => {
                                    GenerateCsvReportingTimeZone::ReportingTimeZoneUnspecified
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
                    impl ::google_field_selector::FieldSelector for GenerateCsvReportingTimeZone {
                        fn fields() -> Vec<::google_field_selector::Field> {
                            Vec::new()
                        }
                    }
                    impl ::google_field_selector::ToFieldType for GenerateCsvReportingTimeZone {
                        fn field_type() -> ::google_field_selector::FieldType {
                            ::google_field_selector::FieldType::Leaf
                        }
                    }
                }
                pub struct SavedActions<'a> {
                    pub(crate) reqwest: &'a reqwest::blocking::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> SavedActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Generates a saved report."]
                    pub fn generate(&self, name: impl Into<String>) -> GenerateRequestBuilder {
                        GenerateRequestBuilder {
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
                            currency_code: None,
                            date_range: None,
                            end_date_day: None,
                            end_date_month: None,
                            end_date_year: None,
                            language_code: None,
                            reporting_time_zone: None,
                            start_date_day: None,
                            start_date_month: None,
                            start_date_year: None,
                        }
                    }
                    #[doc = "Generates a csv formatted saved report."]
                    pub fn generate_csv(
                        &self,
                        name: impl Into<String>,
                    ) -> GenerateCsvRequestBuilder {
                        GenerateCsvRequestBuilder {
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
                            currency_code: None,
                            date_range: None,
                            end_date_day: None,
                            end_date_month: None,
                            end_date_year: None,
                            language_code: None,
                            reporting_time_zone: None,
                            start_date_day: None,
                            start_date_month: None,
                            start_date_year: None,
                        }
                    }
                    #[doc = "Lists saved reports."]
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
                #[doc = "Created via [SavedActions::generate()](struct.SavedActions.html#method.generate)"]
                #[derive(Debug, Clone)]
                pub struct GenerateRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: blocking :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , name : String , currency_code : Option < String > , date_range : Option < crate :: resources :: accounts :: reports :: saved :: params :: GenerateDateRange > , end_date_day : Option < i32 > , end_date_month : Option < i32 > , end_date_year : Option < i32 > , language_code : Option < String > , reporting_time_zone : Option < crate :: resources :: accounts :: reports :: saved :: params :: GenerateReportingTimeZone > , start_date_day : Option < i32 > , start_date_month : Option < i32 > , start_date_year : Option < i32 > , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
                impl<'a> GenerateRequestBuilder<'a> {
                    #[doc = "The [ISO-4217 currency code](https://en.wikipedia.org/wiki/ISO_4217) to use when reporting on monetary metrics. Defaults to the account's currency if not set."]
                    pub fn currency_code(mut self, value: impl Into<String>) -> Self {
                        self.currency_code = Some(value.into());
                        self
                    }
                    #[doc = "Date range of the report, if unset the range will be considered CUSTOM."]
                    pub fn date_range(
                        mut self,
                        value : crate :: resources :: accounts :: reports :: saved :: params :: GenerateDateRange,
                    ) -> Self {
                        self.date_range = Some(value);
                        self
                    }
                    #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
                    pub fn end_date_day(mut self, value: i32) -> Self {
                        self.end_date_day = Some(value);
                        self
                    }
                    #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
                    pub fn end_date_month(mut self, value: i32) -> Self {
                        self.end_date_month = Some(value);
                        self
                    }
                    #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
                    pub fn end_date_year(mut self, value: i32) -> Self {
                        self.end_date_year = Some(value);
                        self
                    }
                    #[doc = "The language to use for translating report output. If unspecified, this defaults to English (\"en\"). If the given language is not supported, report output will be returned in English. The language is specified as an [IETF BCP-47 language code](https://en.wikipedia.org/wiki/IETF_language_tag)."]
                    pub fn language_code(mut self, value: impl Into<String>) -> Self {
                        self.language_code = Some(value.into());
                        self
                    }
                    #[doc = "Timezone in which to generate the report. If unspecified, this defaults to the account timezone. For more information, see [changing the time zone of your reports](https://support.google.com/adsense/answer/9830725)."]
                    pub fn reporting_time_zone(
                        mut self,
                        value : crate :: resources :: accounts :: reports :: saved :: params :: GenerateReportingTimeZone,
                    ) -> Self {
                        self.reporting_time_zone = Some(value);
                        self
                    }
                    #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
                    pub fn start_date_day(mut self, value: i32) -> Self {
                        self.start_date_day = Some(value);
                        self
                    }
                    #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
                    pub fn start_date_month(mut self, value: i32) -> Self {
                        self.start_date_month = Some(value);
                        self
                    }
                    #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
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
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::ReportResult, crate::Error> {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ReportResult, crate::Error> {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://adsense.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/saved:generate");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[("currencyCode", &self.currency_code)]);
                        req = req.query(&[("dateRange", &self.date_range)]);
                        req = req.query(&[("endDate.day", &self.end_date_day)]);
                        req = req.query(&[("endDate.month", &self.end_date_month)]);
                        req = req.query(&[("endDate.year", &self.end_date_year)]);
                        req = req.query(&[("languageCode", &self.language_code)]);
                        req = req.query(&[("reportingTimeZone", &self.reporting_time_zone)]);
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
                        req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[doc = "Created via [SavedActions::generate_csv()](struct.SavedActions.html#method.generate_csv)"]
                #[derive(Debug, Clone)]
                pub struct GenerateCsvRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: blocking :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , name : String , currency_code : Option < String > , date_range : Option < crate :: resources :: accounts :: reports :: saved :: params :: GenerateCsvDateRange > , end_date_day : Option < i32 > , end_date_month : Option < i32 > , end_date_year : Option < i32 > , language_code : Option < String > , reporting_time_zone : Option < crate :: resources :: accounts :: reports :: saved :: params :: GenerateCsvReportingTimeZone > , start_date_day : Option < i32 > , start_date_month : Option < i32 > , start_date_year : Option < i32 > , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
                impl<'a> GenerateCsvRequestBuilder<'a> {
                    #[doc = "The [ISO-4217 currency code](https://en.wikipedia.org/wiki/ISO_4217) to use when reporting on monetary metrics. Defaults to the account's currency if not set."]
                    pub fn currency_code(mut self, value: impl Into<String>) -> Self {
                        self.currency_code = Some(value.into());
                        self
                    }
                    #[doc = "Date range of the report, if unset the range will be considered CUSTOM."]
                    pub fn date_range(
                        mut self,
                        value : crate :: resources :: accounts :: reports :: saved :: params :: GenerateCsvDateRange,
                    ) -> Self {
                        self.date_range = Some(value);
                        self
                    }
                    #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
                    pub fn end_date_day(mut self, value: i32) -> Self {
                        self.end_date_day = Some(value);
                        self
                    }
                    #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
                    pub fn end_date_month(mut self, value: i32) -> Self {
                        self.end_date_month = Some(value);
                        self
                    }
                    #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
                    pub fn end_date_year(mut self, value: i32) -> Self {
                        self.end_date_year = Some(value);
                        self
                    }
                    #[doc = "The language to use for translating report output. If unspecified, this defaults to English (\"en\"). If the given language is not supported, report output will be returned in English. The language is specified as an [IETF BCP-47 language code](https://en.wikipedia.org/wiki/IETF_language_tag)."]
                    pub fn language_code(mut self, value: impl Into<String>) -> Self {
                        self.language_code = Some(value.into());
                        self
                    }
                    #[doc = "Timezone in which to generate the report. If unspecified, this defaults to the account timezone. For more information, see [changing the time zone of your reports](https://support.google.com/adsense/answer/9830725)."]
                    pub fn reporting_time_zone(
                        mut self,
                        value : crate :: resources :: accounts :: reports :: saved :: params :: GenerateCsvReportingTimeZone,
                    ) -> Self {
                        self.reporting_time_zone = Some(value);
                        self
                    }
                    #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
                    pub fn start_date_day(mut self, value: i32) -> Self {
                        self.start_date_day = Some(value);
                        self
                    }
                    #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
                    pub fn start_date_month(mut self, value: i32) -> Self {
                        self.start_date_month = Some(value);
                        self
                    }
                    #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
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
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::HttpBody, crate::Error> {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::HttpBody, crate::Error> {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://adsense.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/saved:generateCsv");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
                    {
                        let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                        req = req.query(&[("currencyCode", &self.currency_code)]);
                        req = req.query(&[("dateRange", &self.date_range)]);
                        req = req.query(&[("endDate.day", &self.end_date_day)]);
                        req = req.query(&[("endDate.month", &self.end_date_month)]);
                        req = req.query(&[("endDate.year", &self.end_date_year)]);
                        req = req.query(&[("languageCode", &self.language_code)]);
                        req = req.query(&[("reportingTimeZone", &self.reporting_time_zone)]);
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
                        req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                #[doc = "Created via [SavedActions::list()](struct.SavedActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::blocking::Client,
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
                    #[doc = "The maximum number of reports to include in the response, used for paging. If unspecified, at most 10000 reports will be returned. The maximum value is 10000; values above 10000 will be coerced to 10000."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "A page token, received from a previous `ListPayments` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListPayments` must match the call that provided the page token."]
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
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                    #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                    #[doc = r" populated fields in the yielded items will be determined by the"]
                    #[doc = r" `FieldSelector` implementation."]
                    pub fn iter_saved_reports<T>(self) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_saved_reports_with_fields(fields)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_saved_reports_with_default_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::SavedReport>
                    {
                        self.iter_saved_reports_with_fields(None::<String>)
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_saved_reports_with_all_fields(
                        self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::SavedReport>
                    {
                        self.iter_saved_reports_with_fields(Some("*"))
                    }
                    pub fn iter_saved_reports_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        self.fields = Some({
                            let mut selector = concat!("nextPageToken,", "savedReports").to_owned();
                            let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                            if !items_fields.is_empty() {
                                selector.push_str("(");
                                selector.push_str(items_fields);
                                selector.push_str(")");
                            }
                            selector
                        });
                        crate::iter::PageItemIter::new(self, "savedReports")
                    }
                    pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.iter_with_fields(fields)
                    }
                    pub fn iter_with_default_fields(
                        self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListSavedReportsResponse>
                    {
                        self.iter_with_fields(None::<&str>)
                    }
                    pub fn iter_with_all_fields(
                        self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListSavedReportsResponse>
                    {
                        self.iter_with_fields(Some("*"))
                    }
                    pub fn iter_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: AsRef<str>,
                    {
                        let mut fields =
                            fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                        if !fields.is_empty() {
                            match fields.chars().rev().nth(0) {
                                Some(',') | None => {}
                                _ => fields.push_str(","),
                            }
                            fields.push_str("nextPageToken");
                            self.fields = Some(fields);
                        }
                        crate::iter::PageIter::new(self)
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                    {
                        let fields = ::google_field_selector::to_string::<T>();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_with_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_with_default_fields(
                        self,
                    ) -> Result<crate::schemas::ListSavedReportsResponse, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::ListSavedReportsResponse, crate::Error>
                    {
                        self.execute_with_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_with_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path())?;
                        Ok(crate::error_from_response(req.send()?)?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://adsense.googleapis.com/".to_owned();
                        output.push_str("v2/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/reports/saved");
                        output
                    }
                    fn _request(
                        &self,
                        path: &str,
                    ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error>
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
                        req = req.bearer_auth(
                            self.auth
                                .access_token()
                                .map_err(|err| crate::Error::OAuth2(err))?,
                        );
                        Ok(req)
                    }
                }
                impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, crate::Error>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        self._execute()
                    }
                }
            }
        }
        pub mod sites {
            pub mod params {}
            pub struct SitesActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> SitesActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Gets information about the selected site."]
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
                #[doc = "Lists all the sites available in an account."]
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
            #[doc = "Created via [SitesActions::get()](struct.SitesActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
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
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::Site, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(self) -> Result<crate::schemas::Site, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adsense.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [SitesActions::list()](struct.SitesActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
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
                #[doc = "The maximum number of sites to include in the response, used for paging. If unspecified, at most 10000 sites will be returned. The maximum value is 10000; values above 10000 will be coerced to 10000."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "A page token, received from a previous `ListSites` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListSites` must match the call that provided the page token."]
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
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                #[doc = r" populated fields in the yielded items will be determined by the"]
                #[doc = r" `FieldSelector` implementation."]
                pub fn iter_sites<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_sites_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_sites_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Site> {
                    self.iter_sites_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_sites_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Site> {
                    self.iter_sites_with_fields(Some("*"))
                }
                pub fn iter_sites_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "sites").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::iter::PageItemIter::new(self, "sites")
                }
                pub fn iter<T>(self) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_with_fields(fields)
                }
                pub fn iter_with_default_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListSitesResponse>
                {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListSitesResponse>
                {
                    self.iter_with_fields(Some("*"))
                }
                pub fn iter_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
                    let mut fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("").to_owned();
                    if !fields.is_empty() {
                        match fields.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => fields.push_str(","),
                        }
                        fields.push_str("nextPageToken");
                        self.fields = Some(fields);
                    }
                    crate::iter::PageIter::new(self)
                }
                #[doc = r" Execute the given operation. The fields requested are"]
                #[doc = r" determined by the FieldSelector attribute of the return type."]
                #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                #[doc = r" are not generic over the return type and deserialize the"]
                #[doc = r" response into an auto-generated struct will all possible"]
                #[doc = r" fields."]
                pub fn execute<T>(self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.execute_with_fields(fields)
                }
                #[doc = r" Execute the given operation. This will not provide any"]
                #[doc = r" `fields` selector indicating that the server will determine"]
                #[doc = r" the fields returned. This typically includes the most common"]
                #[doc = r" fields, but it will not include every possible attribute of"]
                #[doc = r" the response resource."]
                pub fn execute_with_default_fields(
                    self,
                ) -> Result<crate::schemas::ListSitesResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListSitesResponse, crate::Error> {
                    self.execute_with_fields(Some("*"))
                }
                #[doc = r" Execute the given operation. This will use the `fields`"]
                #[doc = r" selector provided and will deserialize the response into"]
                #[doc = r" whatever return value is provided."]
                pub fn execute_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: Into<String>,
                {
                    self.fields = fields.map(Into::into);
                    self._execute()
                }
                fn _execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    let req = self._request(&self._path())?;
                    Ok(crate::error_from_response(req.send()?)?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output = "https://adsense.googleapis.com/".to_owned();
                    output.push_str("v2/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/sites");
                    output
                }
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            impl<'a> crate::iter::IterableMethod for ListRequestBuilder<'a> {
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: ::serde::de::DeserializeOwned,
                {
                    self._execute()
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
    Other(Box<dyn ::std::error::Error + Send + Sync>),
}

impl Error {
    pub fn json_error(&self) -> Option<&::serde_json::Error> {
        match self {
            Error::OAuth2(_) => None,
            Error::JSON(err) => Some(err),
            Error::Reqwest { .. } => None,
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

/// Check the response to see if the status code represents an error. If so
/// convert it into the Reqwest variant of Error.
fn error_from_response(
    response: ::reqwest::blocking::Response,
) -> Result<::reqwest::blocking::Response, Error> {
    match response.error_for_status_ref() {
        Err(reqwest_err) => {
            let body = response.text().ok();
            Err(Error::Reqwest { reqwest_err, body })
        }
        Ok(_) => Ok(response),
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
        body: Box<dyn ::std::io::Read + Send>,
    }

    impl Part {
        pub(crate) fn new(
            content_type: ::mime::Mime,
            body: Box<dyn ::std::io::Read + Send>,
        ) -> Part {
            Part { content_type, body }
        }
    }

    pub(crate) struct RelatedMultiPartReader {
        state: RelatedMultiPartReaderState,
        boundary: String,
        next_body: Option<Box<dyn ::std::io::Read + Send>>,
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
            body: Box<dyn ::std::io::Read + Send>,
        },
    }

    impl ::std::io::Read for RelatedMultiPartReader {
        fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
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
                        let written = body.read(rem_buf)?;
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
            Ok(bytes_written)
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
pub mod iter {
    pub trait IterableMethod {
        fn set_page_token(&mut self, value: String);
        fn execute<T>(&mut self) -> Result<T, crate::Error>
        where
            T: ::serde::de::DeserializeOwned;
    }

    pub struct PageIter<M, T> {
        pub method: M,
        pub finished: bool,
        pub _phantom: ::std::marker::PhantomData<T>,
    }

    impl<M, T> PageIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        pub(crate) fn new(method: M) -> Self {
            PageIter {
                method,
                finished: false,
                _phantom: ::std::marker::PhantomData,
            }
        }
    }

    impl<M, T> Iterator for PageIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        type Item = Result<T, crate::Error>;

        fn next(&mut self) -> Option<Result<T, crate::Error>> {
            if self.finished {
                return None;
            }
            let paginated_result: ::serde_json::Map<String, ::serde_json::Value> =
                match self.method.execute() {
                    Ok(r) => r,
                    Err(err) => return Some(Err(err)),
                };
            if let Some(next_page_token) = paginated_result
                .get("nextPageToken")
                .and_then(|t| t.as_str())
            {
                self.method.set_page_token(next_page_token.to_owned());
            } else {
                self.finished = true;
            }

            Some(
                match ::serde_json::from_value(::serde_json::Value::Object(paginated_result)) {
                    Ok(resp) => Ok(resp),
                    Err(err) => Err(err.into()),
                },
            )
        }
    }

    pub struct PageItemIter<M, T> {
        items_field: &'static str,
        page_iter: PageIter<M, ::serde_json::Map<String, ::serde_json::Value>>,
        items: ::std::vec::IntoIter<T>,
    }

    impl<M, T> PageItemIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        pub(crate) fn new(method: M, items_field: &'static str) -> Self {
            PageItemIter {
                items_field,
                page_iter: PageIter::new(method),
                items: Vec::new().into_iter(),
            }
        }
    }

    impl<M, T> Iterator for PageItemIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        type Item = Result<T, crate::Error>;

        fn next(&mut self) -> Option<Result<T, crate::Error>> {
            loop {
                if let Some(v) = self.items.next() {
                    return Some(Ok(v));
                }

                let next_page = self.page_iter.next();
                match next_page {
                    None => return None,
                    Some(Err(err)) => return Some(Err(err)),
                    Some(Ok(next_page)) => {
                        let mut next_page: ::serde_json::Map<String, ::serde_json::Value> =
                            next_page;
                        let items_array = match next_page.remove(self.items_field) {
                            Some(items) => items,
                            None => {
                                return Some(Err(crate::Error::Other(
                                    format!("no {} field found in iter response", self.items_field)
                                        .into(),
                                )))
                            }
                        };
                        let items_vec: Result<Vec<T>, _> = ::serde_json::from_value(items_array);
                        match items_vec {
                            Ok(items) => self.items = items.into_iter(),
                            Err(err) => return Some(Err(err.into())),
                        }
                    }
                }
            }
        }
    }
}
