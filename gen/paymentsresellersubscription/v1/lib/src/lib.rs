#![allow(rustdoc::bare_urls)]
#![doc = "# Resources and Methods\n* [partners](resources/partners/struct.PartnersActions.html)\n  * [products](resources/partners/products/struct.ProductsActions.html)\n    * [*list*](resources/partners/products/struct.ListRequestBuilder.html)\n  * [promotions](resources/partners/promotions/struct.PromotionsActions.html)\n    * [*findEligible*](resources/partners/promotions/struct.FindEligibleRequestBuilder.html), [*list*](resources/partners/promotions/struct.ListRequestBuilder.html)\n  * [subscriptions](resources/partners/subscriptions/struct.SubscriptionsActions.html)\n    * [*cancel*](resources/partners/subscriptions/struct.CancelRequestBuilder.html), [*create*](resources/partners/subscriptions/struct.CreateRequestBuilder.html), [*entitle*](resources/partners/subscriptions/struct.EntitleRequestBuilder.html), [*extend*](resources/partners/subscriptions/struct.ExtendRequestBuilder.html), [*get*](resources/partners/subscriptions/struct.GetRequestBuilder.html), [*provision*](resources/partners/subscriptions/struct.ProvisionRequestBuilder.html), [*undoCancel*](resources/partners/subscriptions/struct.UndoCancelRequestBuilder.html)\n"]
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1Amount {
        #[doc = "Required. Amount in micros (1_000_000 micros = 1 currency unit)"]
        #[serde(
            rename = "amountMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub amount_micros: ::std::option::Option<i64>,
        #[doc = "Required. Currency codes in accordance with \\[ISO-4217 Currency Codes\\] (https://en.wikipedia.org/wiki/ISO_4217). For example, USD."]
        #[serde(
            rename = "currencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudPaymentsResellerSubscriptionV1Amount {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudPaymentsResellerSubscriptionV1Amount {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequest { # [doc = "Optional. If true, Google will cancel the subscription immediately, and may or may not (based on the contract) issue a prorated refund for the remainder of the billing cycle. Otherwise, Google defers the cancelation at renewal_time, and will not issue a refund."] # [serde (rename = "cancelImmediately" , default , skip_serializing_if = "std::option::Option::is_none")] pub cancel_immediately : :: std :: option :: Option < bool > , # [doc = "Specifies the reason for the cancellation."] # [serde (rename = "cancellationReason" , default , skip_serializing_if = "std::option::Option::is_none")] pub cancellation_reason : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequest
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason {
        #[doc = "Accidential purchase."]
        CancellationReasonAccidentalPurchase,
        #[doc = "User account closed."]
        CancellationReasonAccountClosed,
        #[doc = "Fraudualant transaction."]
        CancellationReasonFraud,
        #[doc = "Other reason."]
        CancellationReasonOther,
        #[doc = "Payment is past due."]
        CancellationReasonPastDue,
        #[doc = "Buyer’s remorse."]
        CancellationReasonRemorse,
        #[doc = "Reason is unspecified."]
        CancellationReasonUnspecified,
        #[doc = "Used for notification only, do not use in Cancel API. Cancellation due to upgrade or downgrade."]
        CancellationReasonUpgradeDowngrade,
        #[doc = "Cancellation due to user delinquency"]
        CancellationReasonUserDelinquency,
    }
    impl GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonAccidentalPurchase => "CANCELLATION_REASON_ACCIDENTAL_PURCHASE" , GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonAccountClosed => "CANCELLATION_REASON_ACCOUNT_CLOSED" , GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonFraud => "CANCELLATION_REASON_FRAUD" , GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonOther => "CANCELLATION_REASON_OTHER" , GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonPastDue => "CANCELLATION_REASON_PAST_DUE" , GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonRemorse => "CANCELLATION_REASON_REMORSE" , GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonUnspecified => "CANCELLATION_REASON_UNSPECIFIED" , GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonUpgradeDowngrade => "CANCELLATION_REASON_UPGRADE_DOWNGRADE" , GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonUserDelinquency => "CANCELLATION_REASON_USER_DELINQUENCY" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason,
            (),
        > {
            Ok (match s { "CANCELLATION_REASON_ACCIDENTAL_PURCHASE" => GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonAccidentalPurchase , "CANCELLATION_REASON_ACCOUNT_CLOSED" => GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonAccountClosed , "CANCELLATION_REASON_FRAUD" => GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonFraud , "CANCELLATION_REASON_OTHER" => GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonOther , "CANCELLATION_REASON_PAST_DUE" => GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonPastDue , "CANCELLATION_REASON_REMORSE" => GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonRemorse , "CANCELLATION_REASON_UNSPECIFIED" => GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonUnspecified , "CANCELLATION_REASON_UPGRADE_DOWNGRADE" => GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonUpgradeDowngrade , "CANCELLATION_REASON_USER_DELINQUENCY" => GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonUserDelinquency , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CANCELLATION_REASON_ACCIDENTAL_PURCHASE" => GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonAccidentalPurchase , "CANCELLATION_REASON_ACCOUNT_CLOSED" => GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonAccountClosed , "CANCELLATION_REASON_FRAUD" => GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonFraud , "CANCELLATION_REASON_OTHER" => GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonOther , "CANCELLATION_REASON_PAST_DUE" => GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonPastDue , "CANCELLATION_REASON_REMORSE" => GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonRemorse , "CANCELLATION_REASON_UNSPECIFIED" => GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonUnspecified , "CANCELLATION_REASON_UPGRADE_DOWNGRADE" => GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonUpgradeDowngrade , "CANCELLATION_REASON_USER_DELINQUENCY" => GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason :: CancellationReasonUserDelinquency , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionResponse {
        #[doc = "The cancelled subscription resource."]
        #[serde(
            rename = "subscription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subscription: ::std::option::Option<
            crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Subscription,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionResponse
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1Duration {
        #[doc = "number of duration units to be included."]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub count: ::std::option::Option<i32>,
        #[doc = "The unit used for the duration"]
        #[serde(
            rename = "unit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unit: ::std::option::Option<
            crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1DurationUnit,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudPaymentsResellerSubscriptionV1Duration {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudPaymentsResellerSubscriptionV1Duration {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPaymentsResellerSubscriptionV1DurationUnit {
        #[doc = "Unit of a day."]
        Day,
        #[doc = "Unit of a calendar month."]
        Month,
        #[doc = "Default value."]
        UnitUnspecified,
    }
    impl GoogleCloudPaymentsResellerSubscriptionV1DurationUnit {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleCloudPaymentsResellerSubscriptionV1DurationUnit::Day => "DAY",
                GoogleCloudPaymentsResellerSubscriptionV1DurationUnit::Month => "MONTH",
                GoogleCloudPaymentsResellerSubscriptionV1DurationUnit::UnitUnspecified => {
                    "UNIT_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudPaymentsResellerSubscriptionV1DurationUnit {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudPaymentsResellerSubscriptionV1DurationUnit {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudPaymentsResellerSubscriptionV1DurationUnit, ()>
        {
            Ok(match s {
                "DAY" => GoogleCloudPaymentsResellerSubscriptionV1DurationUnit::Day,
                "MONTH" => GoogleCloudPaymentsResellerSubscriptionV1DurationUnit::Month,
                "UNIT_UNSPECIFIED" => {
                    GoogleCloudPaymentsResellerSubscriptionV1DurationUnit::UnitUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleCloudPaymentsResellerSubscriptionV1DurationUnit {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudPaymentsResellerSubscriptionV1DurationUnit {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudPaymentsResellerSubscriptionV1DurationUnit {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DAY" => GoogleCloudPaymentsResellerSubscriptionV1DurationUnit::Day,
                "MONTH" => GoogleCloudPaymentsResellerSubscriptionV1DurationUnit::Month,
                "UNIT_UNSPECIFIED" => {
                    GoogleCloudPaymentsResellerSubscriptionV1DurationUnit::UnitUnspecified
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
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1DurationUnit
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1DurationUnit
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1EntitleSubscriptionRequest {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1EntitleSubscriptionRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1EntitleSubscriptionRequest
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1EntitleSubscriptionResponse {
        #[doc = "The subscription that has user linked to it."]
        #[serde(
            rename = "subscription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subscription: ::std::option::Option<
            crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Subscription,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1EntitleSubscriptionResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1EntitleSubscriptionResponse
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionRequest {
        #[doc = "Required. Specifies details of the extension. Currently, the duration of the extension must be exactly one billing cycle of the original subscription."]
        #[serde(
            rename = "extension",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub extension: ::std::option::Option<
            crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Extension,
        >,
        #[doc = "Required. Restricted to 36 ASCII characters. A random UUID is recommended. The idempotency key for the request. The ID generation logic is controlled by the partner. request_id should be the same as on retries of the same request. A different request_id must be used for a extension of a different cycle. A random UUID is recommended."]
        #[serde(
            rename = "requestId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionRequest
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionResponse {
        #[doc = "The time at which the subscription is expected to be extended, in ISO 8061 format. UTC timezone. Example, “cycleEndTime”:“2019-08-31T17:28:54.564Z”"]
        #[serde(
            rename = "cycleEndTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cycle_end_time: ::std::option::Option<String>,
        #[doc = "End of the free trial period, in ISO 8061 format. UTC timezone. Example, “freeTrialEndTime”:“2019-08-31T17:28:54.564Z” This time will be set the same as initial subscription creation time if no free trial period is offered to the partner."]
        #[serde(
            rename = "freeTrialEndTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub free_trial_end_time: ::std::option::Option<String>,
        #[doc = "Output only. The time at which the subscription is expected to be renewed by Google - a new charge will be incurred and the service entitlement will be renewed. A non-immediate cancellation will take place at this time too, before which, the service entitlement for the end user will remain valid. UTC timezone in ISO 8061 format. For example: “2019-08-31T17:28:54.564Z”"]
        #[serde(
            rename = "renewalTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub renewal_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionResponse
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1Extension {
        #[doc = "Specifies the period of access the subscription should grant."]
        #[serde(
            rename = "duration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub duration: ::std::option::Option<
            crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Duration,
        >,
        #[doc = "Required. Identifier of the end-user in partner’s system."]
        #[serde(
            rename = "partnerUserToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub partner_user_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudPaymentsResellerSubscriptionV1Extension {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudPaymentsResellerSubscriptionV1Extension {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudPaymentsResellerSubscriptionV1FindEligiblePromotionsRequest {
        #[doc = "Optional. Specifies the filters for the promotion results. The syntax is defined in https://google.aip.dev/160 with the following caveats: - Only the following features are supported: - Logical operator `AND` - Comparison operator `=` (no wildcards `*`) - Traversal operator `.` - Has operator `:` (no wildcards `*`) - Only the following fields are supported: - `applicableProducts` - `regionCodes` - `youtubePayload.partnerEligibilityId` - `youtubePayload.postalCode` - Unless explicitly mentioned above, other features are not supported. Example: `applicableProducts:partners/partner1/products/product1 AND regionCodes:US AND youtubePayload.postalCode=94043 AND youtubePayload.partnerEligibilityId=eligibility-id`"]
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter: ::std::option::Option<String>,
        #[doc = "Optional. The maximum number of promotions to return. The service may return fewer than this value. If unspecified, at most 50 products will be returned. The maximum value is 1000; values above 1000 will be coerced to 1000."]
        #[serde(
            rename = "pageSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_size: ::std::option::Option<i32>,
        #[doc = "Optional. A page token, received from a previous `ListPromotions` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListPromotions` must match the call that provided the page token."]
        #[serde(
            rename = "pageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1FindEligiblePromotionsRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1FindEligiblePromotionsRequest
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1FindEligiblePromotionsResponse {
        #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is empty, there are no subsequent pages."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The promotions for the current user."]
        #[serde(
            rename = "promotions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub promotions: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Promotion>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1FindEligiblePromotionsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1FindEligiblePromotionsResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String>
        for GoogleCloudPaymentsResellerSubscriptionV1FindEligiblePromotionsResponse
    {
        fn next_page_token(&self) -> ::std::option::Option<String> {
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayload {
        #[doc = "Campaign attributed to sales of this subscription."]
        #[serde(
            rename = "campaigns",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub campaigns: ::std::option::Option<Vec<String>>,
        #[doc = "The type of offering the subscription was sold by the partner. e.g. VAS."]
        #[serde(
            rename = "offering",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub offering: ::std::option::Option<
            crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering,
        >,
        #[doc = "The type of sales channel through which the subscription was sold."]
        #[serde(
            rename = "salesChannel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sales_channel: ::std::option::Option<
            crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel,
        >,
        #[doc = "The identifier for the partner store where the subscription was sold."]
        #[serde(
            rename = "storeId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub store_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayload
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayload
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering {
        #[doc = "Product purchased as part of a hard bundle where Google One was included with the bundle. Google One pricing is included in the bundle."]
        OfferingHardBundle,
        #[doc = "Purchased as part of a bundle where Google One was provided as an option. Google One pricing is included in the bundle."]
        OfferingSoftBundle,
        #[doc = "The type of partner offering is unspecified."]
        OfferingUnspecified,
        #[doc = "Google One product purchased as a Value added service in addition to existing partner’s products. Customer pays additional amount for Google One product."]
        OfferingVasBundle,
        #[doc = "Google One product purchased by itself by customer as a value add service. Customer pays additional amount for Google One product."]
        OfferingVasStandalone,
    }
    impl GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering :: OfferingHardBundle => "OFFERING_HARD_BUNDLE" , GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering :: OfferingSoftBundle => "OFFERING_SOFT_BUNDLE" , GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering :: OfferingUnspecified => "OFFERING_UNSPECIFIED" , GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering :: OfferingVasBundle => "OFFERING_VAS_BUNDLE" , GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering :: OfferingVasStandalone => "OFFERING_VAS_STANDALONE" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering,
            (),
        > {
            Ok (match s { "OFFERING_HARD_BUNDLE" => GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering :: OfferingHardBundle , "OFFERING_SOFT_BUNDLE" => GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering :: OfferingSoftBundle , "OFFERING_UNSPECIFIED" => GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering :: OfferingUnspecified , "OFFERING_VAS_BUNDLE" => GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering :: OfferingVasBundle , "OFFERING_VAS_STANDALONE" => GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering :: OfferingVasStandalone , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "OFFERING_HARD_BUNDLE" => GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering :: OfferingHardBundle , "OFFERING_SOFT_BUNDLE" => GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering :: OfferingSoftBundle , "OFFERING_UNSPECIFIED" => GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering :: OfferingUnspecified , "OFFERING_VAS_BUNDLE" => GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering :: OfferingVasBundle , "OFFERING_VAS_STANDALONE" => GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering :: OfferingVasStandalone , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadOffering
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel {
        #[doc = "Sold through partner android app."]
        ChannelOnlineAndroidApp,
        #[doc = "Sold through partner iOS app."]
        ChannelOnlineIosApp,
        #[doc = "Sold through partner website."]
        ChannelOnlineWeb,
        #[doc = "Sold at store."]
        ChannelRetail,
        #[doc = "The channel type is unspecified."]
        ChannelUnspecified,
    }
    impl GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel :: ChannelOnlineAndroidApp => "CHANNEL_ONLINE_ANDROID_APP" , GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel :: ChannelOnlineIosApp => "CHANNEL_ONLINE_IOS_APP" , GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel :: ChannelOnlineWeb => "CHANNEL_ONLINE_WEB" , GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel :: ChannelRetail => "CHANNEL_RETAIL" , GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel :: ChannelUnspecified => "CHANNEL_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel,
            (),
        > {
            Ok (match s { "CHANNEL_ONLINE_ANDROID_APP" => GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel :: ChannelOnlineAndroidApp , "CHANNEL_ONLINE_IOS_APP" => GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel :: ChannelOnlineIosApp , "CHANNEL_ONLINE_WEB" => GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel :: ChannelOnlineWeb , "CHANNEL_RETAIL" => GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel :: ChannelRetail , "CHANNEL_UNSPECIFIED" => GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel :: ChannelUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CHANNEL_ONLINE_ANDROID_APP" => GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel :: ChannelOnlineAndroidApp , "CHANNEL_ONLINE_IOS_APP" => GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel :: ChannelOnlineIosApp , "CHANNEL_ONLINE_WEB" => GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel :: ChannelOnlineWeb , "CHANNEL_RETAIL" => GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel :: ChannelRetail , "CHANNEL_UNSPECIFIED" => GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel :: ChannelUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayloadSalesChannel
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1ListProductsResponse {
        #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is empty, there are no subsequent pages."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The products for the specified partner."]
        #[serde(
            rename = "products",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub products: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Product>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1ListProductsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1ListProductsResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String>
        for GoogleCloudPaymentsResellerSubscriptionV1ListProductsResponse
    {
        fn next_page_token(&self) -> ::std::option::Option<String> {
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1ListPromotionsResponse {
        #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is empty, there are no subsequent pages."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The promotions for the specified partner."]
        #[serde(
            rename = "promotions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub promotions: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Promotion>,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1ListPromotionsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1ListPromotionsResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String>
        for GoogleCloudPaymentsResellerSubscriptionV1ListPromotionsResponse
    {
        fn next_page_token(&self) -> ::std::option::Option<String> {
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1Location {
        #[doc = "The postal code this location refers to. Ex. “94043”"]
        #[serde(
            rename = "postalCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub postal_code: ::std::option::Option<String>,
        #[doc = "2-letter ISO region code for current content region. Ex. “US” Please refers to: https://en.wikipedia.org/wiki/ISO_3166-1"]
        #[serde(
            rename = "regionCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudPaymentsResellerSubscriptionV1Location {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudPaymentsResellerSubscriptionV1Location {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudPaymentsResellerSubscriptionV1Product {
        #[doc = "Output only. Response only. Resource name of the product. It will have the format of “partners/{partner_id}/products/{product_id}”"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. Price configs for the product in the available regions."]
        #[serde(
            rename = "priceConfigs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub price_configs: ::std::option::Option<
            Vec<crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1ProductPriceConfig>,
        >,
        #[doc = "Output only. 2-letter ISO region code where the product is available in. Ex. “US” Please refers to: https://en.wikipedia.org/wiki/ISO_3166-1"]
        #[serde(
            rename = "regionCodes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region_codes: ::std::option::Option<Vec<String>>,
        #[doc = "Output only. Specifies the length of the billing cycle of the subscription."]
        #[serde(
            rename = "subscriptionBillingCycleDuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subscription_billing_cycle_duration: ::std::option::Option<
            crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Duration,
        >,
        #[doc = "Output only. Localized human readable name of the product."]
        #[serde(
            rename = "titles",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub titles: ::std::option::Option<Vec<crate::schemas::GoogleTypeLocalizedText>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleCloudPaymentsResellerSubscriptionV1Product {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudPaymentsResellerSubscriptionV1Product {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudPaymentsResellerSubscriptionV1ProductPayload {
        #[doc = "Payload specific to Google One products."]
        #[serde(
            rename = "googleOnePayload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_one_payload: ::std::option::Option<
            crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1GoogleOnePayload,
        >,
        #[doc = "Payload specific to Youtube products."]
        #[serde(
            rename = "youtubePayload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub youtube_payload: ::std::option::Option<
            crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1YoutubePayload,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1ProductPayload
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1ProductPayload
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1ProductPriceConfig {
        #[doc = "Output only. The price in the region."]
        #[serde(
            rename = "amount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub amount:
            ::std::option::Option<crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Amount>,
        #[doc = "Output only. 2-letter ISO region code where the product is available in. Ex. “US”."]
        #[serde(
            rename = "regionCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1ProductPriceConfig
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1ProductPriceConfig
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1Promotion { # [doc = "Output only. The product ids this promotion can be applied to."] # [serde (rename = "applicableProducts" , default , skip_serializing_if = "std::option::Option::is_none")] pub applicable_products : :: std :: option :: Option < Vec < String > > , # [doc = "Optional. Specifies the end time (exclusive) of the period that the promotion is available in. If unset, the promotion is available indefinitely."] # [serde (rename = "endTime" , default , skip_serializing_if = "std::option::Option::is_none")] pub end_time : :: std :: option :: Option < String > , # [doc = "Optional. Specifies the duration of the free trial of the subscription when promotion_type is PROMOTION_TYPE_FREE_TRIAL"] # [serde (rename = "freeTrialDuration" , default , skip_serializing_if = "std::option::Option::is_none")] pub free_trial_duration : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1Duration > , # [doc = "Optional. Specifies the introductory pricing details when the promotion_type is PROMOTION_TYPE_INTRODUCTORY_PRICING."] # [serde (rename = "introductoryPricingDetails" , default , skip_serializing_if = "std::option::Option::is_none")] pub introductory_pricing_details : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1PromotionIntroductoryPricingDetails > , # [doc = "Output only. Response only. Resource name of the subscription promotion. It will have the format of “partners/{partner_id}/promotion/{promotion_id}”"] # [serde (rename = "name" , default , skip_serializing_if = "std::option::Option::is_none")] pub name : :: std :: option :: Option < String > , # [doc = "Output only. Output Only. Specifies the type of the promotion."] # [serde (rename = "promotionType" , default , skip_serializing_if = "std::option::Option::is_none")] pub promotion_type : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionType > , # [doc = "Output only. 2-letter ISO region code where the promotion is available in. Ex. “US” Please refers to: https://en.wikipedia.org/wiki/ISO_3166-1"] # [serde (rename = "regionCodes" , default , skip_serializing_if = "std::option::Option::is_none")] pub region_codes : :: std :: option :: Option < Vec < String > > , # [doc = "Optional. Specifies the start time (inclusive) of the period that the promotion is available in."] # [serde (rename = "startTime" , default , skip_serializing_if = "std::option::Option::is_none")] pub start_time : :: std :: option :: Option < String > , # [doc = "Output only. Localized human readable name of the promotion."] # [serde (rename = "titles" , default , skip_serializing_if = "std::option::Option::is_none")] pub titles : :: std :: option :: Option < Vec < crate :: schemas :: GoogleTypeLocalizedText > > , }
    impl ::google_field_selector::FieldSelector for GoogleCloudPaymentsResellerSubscriptionV1Promotion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleCloudPaymentsResellerSubscriptionV1Promotion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionType {
        #[doc = "The promotion is a free trial."]
        PromotionTypeFreeTrial,
        #[doc = "The promotion is a reduced introductory pricing."]
        PromotionTypeIntroductoryPricing,
        #[doc = "The promotion type is unspecified."]
        PromotionTypeUnspecified,
    }
    impl GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionType :: PromotionTypeFreeTrial => "PROMOTION_TYPE_FREE_TRIAL" , GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionType :: PromotionTypeIntroductoryPricing => "PROMOTION_TYPE_INTRODUCTORY_PRICING" , GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionType :: PromotionTypeUnspecified => "PROMOTION_TYPE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionType,
            (),
        > {
            Ok (match s { "PROMOTION_TYPE_FREE_TRIAL" => GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionType :: PromotionTypeFreeTrial , "PROMOTION_TYPE_INTRODUCTORY_PRICING" => GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionType :: PromotionTypeIntroductoryPricing , "PROMOTION_TYPE_UNSPECIFIED" => GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionType :: PromotionTypeUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "PROMOTION_TYPE_FREE_TRIAL" => GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionType :: PromotionTypeFreeTrial , "PROMOTION_TYPE_INTRODUCTORY_PRICING" => GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionType :: PromotionTypeIntroductoryPricing , "PROMOTION_TYPE_UNSPECIFIED" => GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionType :: PromotionTypeUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionType
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1PromotionIntroductoryPricingDetails { # [doc = "Specifies the introductory pricing periods."] # [serde (rename = "introductoryPricingSpecs" , default , skip_serializing_if = "std::option::Option::is_none")] pub introductory_pricing_specs : :: std :: option :: Option < Vec < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1PromotionIntroductoryPricingDetailsIntroductoryPricingSpec > > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1PromotionIntroductoryPricingDetails
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1PromotionIntroductoryPricingDetails
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1PromotionIntroductoryPricingDetailsIntroductoryPricingSpec
    {
        #[doc = "Output only. The discount amount. The value is positive."]
        #[serde(
            rename = "discountAmount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub discount_amount:
            ::std::option::Option<crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Amount>,
        #[doc = "Output only. The discount percentage in micros. For example, 50,000 represents 5%."]
        #[serde(
            rename = "discountRatioMicros",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub discount_ratio_micros: ::std::option::Option<i64>,
        #[doc = "Output only. Output Only. The duration of an introductory offer in billing cycles."]
        #[serde(
            rename = "recurrenceCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recurrence_count: ::std::option::Option<i32>,
        #[doc = "Output only. 2-letter ISO region code where the product is available in. Ex. “US”."]
        #[serde(
            rename = "regionCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region_code: ::std::option::Option<String>,
    }
    impl :: google_field_selector :: FieldSelector for GoogleCloudPaymentsResellerSubscriptionV1PromotionIntroductoryPricingDetailsIntroductoryPricingSpec { fn fields () -> Vec < :: google_field_selector :: Field > { Vec :: new () } }
    impl :: google_field_selector :: ToFieldType for GoogleCloudPaymentsResellerSubscriptionV1PromotionIntroductoryPricingDetailsIntroductoryPricingSpec { fn field_type () -> :: google_field_selector :: FieldType { :: google_field_selector :: FieldType :: Leaf } }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleCloudPaymentsResellerSubscriptionV1ServicePeriod {
        #[doc = "Optional. The end time of the service period. Time is exclusive."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Required. The start time of the service period. Time is inclusive."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1ServicePeriod
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1ServicePeriod
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1Subscription { # [doc = "Output only. Describes the details of a cancelled subscription. Only applicable to subscription of state `STATE_CANCELLED`."] # [serde (rename = "cancellationDetails" , default , skip_serializing_if = "std::option::Option::is_none")] pub cancellation_details : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetails > , # [doc = "Output only. System generated timestamp when the subscription is created. UTC timezone."] # [serde (rename = "createTime" , default , skip_serializing_if = "std::option::Option::is_none")] pub create_time : :: std :: option :: Option < String > , # [doc = "Output only. The time at which the subscription is expected to be extended, in ISO 8061 format. UTC timezone. For example: “2019-08-31T17:28:54.564Z”"] # [serde (rename = "cycleEndTime" , default , skip_serializing_if = "std::option::Option::is_none")] pub cycle_end_time : :: std :: option :: Option < String > , # [doc = "Output only. Indicates if the subscription is entitled to the end user."] # [serde (rename = "endUserEntitled" , default , skip_serializing_if = "std::option::Option::is_none")] pub end_user_entitled : :: std :: option :: Option < bool > , # [doc = "Output only. End of the free trial period, in ISO 8061 format. For example, “2019-08-31T17:28:54.564Z”. It will be set the same as createTime if no free trial promotion is specified."] # [serde (rename = "freeTrialEndTime" , default , skip_serializing_if = "std::option::Option::is_none")] pub free_trial_end_time : :: std :: option :: Option < String > , # [doc = "Required. The line items of the subscription."] # [serde (rename = "lineItems" , default , skip_serializing_if = "std::option::Option::is_none")] pub line_items : :: std :: option :: Option < Vec < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItem > > , # [doc = "Optional. Resource name of the subscription. It will have the format of “partners/{partner_id}/subscriptions/{subscription_id}”. This is available for authorizeAddon, but otherwise is response only."] # [serde (rename = "name" , default , skip_serializing_if = "std::option::Option::is_none")] pub name : :: std :: option :: Option < String > , # [doc = "Required. Identifier of the end-user in partner’s system. The value is restricted to 63 ASCII characters at the maximum."] # [serde (rename = "partnerUserToken" , default , skip_serializing_if = "std::option::Option::is_none")] pub partner_user_token : :: std :: option :: Option < String > , # [doc = "Output only. Describes the processing state of the subscription. See more details at [the lifecycle of a subscription](/payments/reseller/subscription/reference/index/Receive.Notifications#payments-subscription-lifecycle)."] # [serde (rename = "processingState" , default , skip_serializing_if = "std::option::Option::is_none")] pub processing_state : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingState > , # [doc = "Required. Deprecated: consider using `line_items` as the input. Required. Resource name that identifies the purchased products. The format will be ‘partners/{partner_id}/products/{product_id}’."] # [serde (rename = "products" , default , skip_serializing_if = "std::option::Option::is_none")] pub products : :: std :: option :: Option < Vec < String > > , # [doc = "Optional. Subscription-level promotions. Only free trial is supported on this level. It determines the first renewal time of the subscription to be the end of the free trial period. Specify the promotion resource name only when used as input."] # [serde (rename = "promotionSpecs" , default , skip_serializing_if = "std::option::Option::is_none")] pub promotion_specs : :: std :: option :: Option < Vec < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpec > > , # [doc = "Optional. Deprecated: consider using the top-level `promotion_specs` as the input. Optional. Resource name that identifies one or more promotions that can be applied on the product. A typical promotion for a subscription is Free trial. The format will be ‘partners/{partner_id}/promotions/{promotion_id}’."] # [serde (rename = "promotions" , default , skip_serializing_if = "std::option::Option::is_none")] pub promotions : :: std :: option :: Option < Vec < String > > , # [doc = "Output only. The place where partners should redirect the end-user to after creation. This field might also be populated when creation failed. However, Partners should always prepare a default URL to redirect the user in case this field is empty."] # [serde (rename = "redirectUri" , default , skip_serializing_if = "std::option::Option::is_none")] pub redirect_uri : :: std :: option :: Option < String > , # [doc = "Output only. The time at which the subscription is expected to be renewed by Google - a new charge will be incurred and the service entitlement will be renewed. A non-immediate cancellation will take place at this time too, before which, the service entitlement for the end user will remain valid. UTC timezone in ISO 8061 format. For example: “2019-08-31T17:28:54.564Z”"] # [serde (rename = "renewalTime" , default , skip_serializing_if = "std::option::Option::is_none")] pub renewal_time : :: std :: option :: Option < String > , # [doc = "Required. The location that the service is provided as indicated by the partner."] # [serde (rename = "serviceLocation" , default , skip_serializing_if = "std::option::Option::is_none")] pub service_location : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1Location > , # [doc = "Output only. Describes the state of the subscription. See more details at [the lifecycle of a subscription](/payments/reseller/subscription/reference/index/Receive.Notifications#payments-subscription-lifecycle)."] # [serde (rename = "state" , default , skip_serializing_if = "std::option::Option::is_none")] pub state : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState > , # [doc = "Output only. System generated timestamp when the subscription is most recently updated. UTC timezone."] # [serde (rename = "updateTime" , default , skip_serializing_if = "std::option::Option::is_none")] pub update_time : :: std :: option :: Option < String > , # [doc = "Optional. Details about the previous subscription that this new subscription upgrades/downgrades from. Only populated if this subscription is an upgrade/downgrade from another subscription."] # [serde (rename = "upgradeDowngradeDetails" , default , skip_serializing_if = "std::option::Option::is_none")] pub upgrade_downgrade_details : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetails > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1Subscription
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1Subscription
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingState {
        #[doc = "The subscription is being cancelled."]
        ProcessingStateCancelling,
        #[doc = "The subscription is recurring."]
        ProcessingStateRecurring,
        #[doc = "The processing state is unspecified."]
        ProcessingStateUnspecified,
    }
    impl GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingState {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingState :: ProcessingStateCancelling => "PROCESSING_STATE_CANCELLING" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingState :: ProcessingStateRecurring => "PROCESSING_STATE_RECURRING" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingState :: ProcessingStateUnspecified => "PROCESSING_STATE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingState
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingState,
            (),
        > {
            Ok (match s { "PROCESSING_STATE_CANCELLING" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingState :: ProcessingStateCancelling , "PROCESSING_STATE_RECURRING" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingState :: ProcessingStateRecurring , "PROCESSING_STATE_UNSPECIFIED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingState :: ProcessingStateUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingState
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "PROCESSING_STATE_CANCELLING" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingState :: ProcessingStateCancelling , "PROCESSING_STATE_RECURRING" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingState :: ProcessingStateRecurring , "PROCESSING_STATE_UNSPECIFIED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingState :: ProcessingStateUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingState
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState {
        #[doc = "The subscription is active."]
        StateActive,
        #[doc = "The subscription is waiting to be cancelled by the next recurrence cycle."]
        StateCancelAtEndOfCycle,
        #[doc = "The subscription is cancelled."]
        StateCancelled,
        #[doc = "The subscription is created, a state before it is moved to STATE_ACTIVE."]
        StateCreated,
        #[doc = "The subscription has not been extended by the partner after the end of current cycle."]
        StateInGracePeriod,
        #[doc = "The subscription is suspended."]
        StateSuspended,
        #[doc = "The state is unspecified."]
        StateUnspecified,
    }
    impl GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateActive => "STATE_ACTIVE" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateCancelAtEndOfCycle => "STATE_CANCEL_AT_END_OF_CYCLE" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateCancelled => "STATE_CANCELLED" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateCreated => "STATE_CREATED" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateInGracePeriod => "STATE_IN_GRACE_PERIOD" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateSuspended => "STATE_SUSPENDED" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateUnspecified => "STATE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState, ()>
        {
            Ok (match s { "STATE_ACTIVE" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateActive , "STATE_CANCEL_AT_END_OF_CYCLE" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateCancelAtEndOfCycle , "STATE_CANCELLED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateCancelled , "STATE_CREATED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateCreated , "STATE_IN_GRACE_PERIOD" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateInGracePeriod , "STATE_SUSPENDED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateSuspended , "STATE_UNSPECIFIED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "STATE_ACTIVE" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateActive , "STATE_CANCEL_AT_END_OF_CYCLE" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateCancelAtEndOfCycle , "STATE_CANCELLED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateCancelled , "STATE_CREATED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateCreated , "STATE_IN_GRACE_PERIOD" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateInGracePeriod , "STATE_SUSPENDED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateSuspended , "STATE_UNSPECIFIED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetails { # [doc = "The reason of the cancellation."] # [serde (rename = "reason" , default , skip_serializing_if = "std::option::Option::is_none")] pub reason : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetails
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetails
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason {
        #[doc = "Accidential purchase."]
        CancellationReasonAccidentalPurchase,
        #[doc = "User account closed."]
        CancellationReasonAccountClosed,
        #[doc = "Fraudualant transaction."]
        CancellationReasonFraud,
        #[doc = "Other reason."]
        CancellationReasonOther,
        #[doc = "Payment is past due."]
        CancellationReasonPastDue,
        #[doc = "Buyer’s remorse."]
        CancellationReasonRemorse,
        #[doc = "Reason is unspecified."]
        CancellationReasonUnspecified,
        #[doc = "Used for notification only, do not use in Cancel API. Cancellation due to upgrade or downgrade."]
        CancellationReasonUpgradeDowngrade,
        #[doc = "Cancellation due to user delinquency"]
        CancellationReasonUserDelinquency,
    }
    impl GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonAccidentalPurchase => "CANCELLATION_REASON_ACCIDENTAL_PURCHASE" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonAccountClosed => "CANCELLATION_REASON_ACCOUNT_CLOSED" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonFraud => "CANCELLATION_REASON_FRAUD" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonOther => "CANCELLATION_REASON_OTHER" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonPastDue => "CANCELLATION_REASON_PAST_DUE" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonRemorse => "CANCELLATION_REASON_REMORSE" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonUnspecified => "CANCELLATION_REASON_UNSPECIFIED" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonUpgradeDowngrade => "CANCELLATION_REASON_UPGRADE_DOWNGRADE" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonUserDelinquency => "CANCELLATION_REASON_USER_DELINQUENCY" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason,
            (),
        > {
            Ok (match s { "CANCELLATION_REASON_ACCIDENTAL_PURCHASE" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonAccidentalPurchase , "CANCELLATION_REASON_ACCOUNT_CLOSED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonAccountClosed , "CANCELLATION_REASON_FRAUD" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonFraud , "CANCELLATION_REASON_OTHER" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonOther , "CANCELLATION_REASON_PAST_DUE" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonPastDue , "CANCELLATION_REASON_REMORSE" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonRemorse , "CANCELLATION_REASON_UNSPECIFIED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonUnspecified , "CANCELLATION_REASON_UPGRADE_DOWNGRADE" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonUpgradeDowngrade , "CANCELLATION_REASON_USER_DELINQUENCY" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonUserDelinquency , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CANCELLATION_REASON_ACCIDENTAL_PURCHASE" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonAccidentalPurchase , "CANCELLATION_REASON_ACCOUNT_CLOSED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonAccountClosed , "CANCELLATION_REASON_FRAUD" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonFraud , "CANCELLATION_REASON_OTHER" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonOther , "CANCELLATION_REASON_PAST_DUE" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonPastDue , "CANCELLATION_REASON_REMORSE" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonRemorse , "CANCELLATION_REASON_UNSPECIFIED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonUnspecified , "CANCELLATION_REASON_UPGRADE_DOWNGRADE" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonUpgradeDowngrade , "CANCELLATION_REASON_USER_DELINQUENCY" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason :: CancellationReasonUserDelinquency , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetailsReason
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItem { # [doc = "Output only. Description of this line item."] # [serde (rename = "description" , default , skip_serializing_if = "std::option::Option::is_none")] pub description : :: std :: option :: Option < String > , # [doc = "Output only. It is set only if the line item has its own free trial applied. End time of the line item free trial period, in ISO 8061 format. For example, “2019-08-31T17:28:54.564Z”. It will be set the same as createTime if no free trial promotion is specified."] # [serde (rename = "lineItemFreeTrialEndTime" , default , skip_serializing_if = "std::option::Option::is_none")] pub line_item_free_trial_end_time : :: std :: option :: Option < String > , # [doc = "Optional. The promotions applied on the line item. It can be: - a free trial promotion, which overrides the subscription-level free trial promotion. - an introductory pricing promotion. When used as input in Create or Provision API, specify its resource name only."] # [serde (rename = "lineItemPromotionSpecs" , default , skip_serializing_if = "std::option::Option::is_none")] pub line_item_promotion_specs : :: std :: option :: Option < Vec < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpec > > , # [doc = "Output only. Details only set for a ONE_TIME recurrence line item."] # [serde (rename = "oneTimeRecurrenceDetails" , default , skip_serializing_if = "std::option::Option::is_none")] pub one_time_recurrence_details : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemOneTimeRecurrenceDetails > , # [doc = "Required. Product resource name that identifies one the line item The format is ‘partners/{partner_id}/products/{product_id}’."] # [serde (rename = "product" , default , skip_serializing_if = "std::option::Option::is_none")] pub product : :: std :: option :: Option < String > , # [doc = "Optional. Product specific payload for this line item."] # [serde (rename = "productPayload" , default , skip_serializing_if = "std::option::Option::is_none")] pub product_payload : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1ProductPayload > , # [doc = "Output only. The recurrence type of the line item."] # [serde (rename = "recurrenceType" , default , skip_serializing_if = "std::option::Option::is_none")] pub recurrence_type : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceType > , # [doc = "Output only. The state of the line item."] # [serde (rename = "state" , default , skip_serializing_if = "std::option::Option::is_none")] pub state : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItem
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItem
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceType {
        #[doc = "The line item does not recur in the future."]
        LineItemRecurrenceTypeOneTime,
        #[doc = "The line item recurs periodically."]
        LineItemRecurrenceTypePeriodic,
        #[doc = "The line item recurrence type is unspecified."]
        LineItemRecurrenceTypeUnspecified,
    }
    impl GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceType :: LineItemRecurrenceTypeOneTime => "LINE_ITEM_RECURRENCE_TYPE_ONE_TIME" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceType :: LineItemRecurrenceTypePeriodic => "LINE_ITEM_RECURRENCE_TYPE_PERIODIC" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceType :: LineItemRecurrenceTypeUnspecified => "LINE_ITEM_RECURRENCE_TYPE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceType
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceType,
            (),
        > {
            Ok (match s { "LINE_ITEM_RECURRENCE_TYPE_ONE_TIME" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceType :: LineItemRecurrenceTypeOneTime , "LINE_ITEM_RECURRENCE_TYPE_PERIODIC" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceType :: LineItemRecurrenceTypePeriodic , "LINE_ITEM_RECURRENCE_TYPE_UNSPECIFIED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceType :: LineItemRecurrenceTypeUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceType
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceType
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "LINE_ITEM_RECURRENCE_TYPE_ONE_TIME" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceType :: LineItemRecurrenceTypeOneTime , "LINE_ITEM_RECURRENCE_TYPE_PERIODIC" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceType :: LineItemRecurrenceTypePeriodic , "LINE_ITEM_RECURRENCE_TYPE_UNSPECIFIED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceType :: LineItemRecurrenceTypeUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemRecurrenceType
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState {
        #[doc = "The line item is being activated in order to be charged. If a free trial applies to the line item, the line item is pending a prorated charge at the end of the free trial period, as indicated by `line_item_free_trial_end_time`."]
        LineItemStateActivating,
        #[doc = "The line item is in ACTIVE state."]
        LineItemStateActive,
        #[doc = "The line item is being deactivated, and a prorated refund in being processed."]
        LineItemStateDeactivating,
        #[doc = "The line item is in INACTIVE state."]
        LineItemStateInactive,
        #[doc = "The line item is new, and is not activated or charged yet."]
        LineItemStateNew,
        #[doc = "Unspecified state."]
        LineItemStateUnspecified,
        #[doc = "The line item is scheduled to be deactivated at the end of the current cycle."]
        LineItemStateWaitingToDeactivate,
    }
    impl GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState :: LineItemStateActivating => "LINE_ITEM_STATE_ACTIVATING" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState :: LineItemStateActive => "LINE_ITEM_STATE_ACTIVE" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState :: LineItemStateDeactivating => "LINE_ITEM_STATE_DEACTIVATING" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState :: LineItemStateInactive => "LINE_ITEM_STATE_INACTIVE" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState :: LineItemStateNew => "LINE_ITEM_STATE_NEW" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState :: LineItemStateUnspecified => "LINE_ITEM_STATE_UNSPECIFIED" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState :: LineItemStateWaitingToDeactivate => "LINE_ITEM_STATE_WAITING_TO_DEACTIVATE" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState,
            (),
        > {
            Ok (match s { "LINE_ITEM_STATE_ACTIVATING" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState :: LineItemStateActivating , "LINE_ITEM_STATE_ACTIVE" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState :: LineItemStateActive , "LINE_ITEM_STATE_DEACTIVATING" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState :: LineItemStateDeactivating , "LINE_ITEM_STATE_INACTIVE" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState :: LineItemStateInactive , "LINE_ITEM_STATE_NEW" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState :: LineItemStateNew , "LINE_ITEM_STATE_UNSPECIFIED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState :: LineItemStateUnspecified , "LINE_ITEM_STATE_WAITING_TO_DEACTIVATE" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState :: LineItemStateWaitingToDeactivate , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "LINE_ITEM_STATE_ACTIVATING" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState :: LineItemStateActivating , "LINE_ITEM_STATE_ACTIVE" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState :: LineItemStateActive , "LINE_ITEM_STATE_DEACTIVATING" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState :: LineItemStateDeactivating , "LINE_ITEM_STATE_INACTIVE" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState :: LineItemStateInactive , "LINE_ITEM_STATE_NEW" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState :: LineItemStateNew , "LINE_ITEM_STATE_UNSPECIFIED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState :: LineItemStateUnspecified , "LINE_ITEM_STATE_WAITING_TO_DEACTIVATE" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState :: LineItemStateWaitingToDeactivate , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemState
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemOneTimeRecurrenceDetails {
        #[doc = "The service period of the ONE_TIME line item."]
        #[serde(
            rename = "servicePeriod",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_period: ::std::option::Option<
            crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1ServicePeriod,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemOneTimeRecurrenceDetails
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionLineItemOneTimeRecurrenceDetails
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpec { # [doc = "Output only. The duration of the free trial if the promotion is of type FREE_TRIAL."] # [serde (rename = "freeTrialDuration" , default , skip_serializing_if = "std::option::Option::is_none")] pub free_trial_duration : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1Duration > , # [doc = "Output only. The details of the introductory pricing spec if the promotion is of type INTRODUCTORY_PRICING."] # [serde (rename = "introductoryPricingDetails" , default , skip_serializing_if = "std::option::Option::is_none")] pub introductory_pricing_details : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1PromotionIntroductoryPricingDetails > , # [doc = "Required. Promotion resource name that identifies a promotion. The format is ‘partners/{partner_id}/promotions/{promotion_id}’."] # [serde (rename = "promotion" , default , skip_serializing_if = "std::option::Option::is_none")] pub promotion : :: std :: option :: Option < String > , # [doc = "Output only. The type of the promotion for the spec."] # [serde (rename = "type" , default , skip_serializing_if = "std::option::Option::is_none")] pub r#type : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecType > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpec
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpec
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecType {
        #[doc = "The promotion is a free trial."]
        PromotionTypeFreeTrial,
        #[doc = "The promotion is a reduced introductory pricing."]
        PromotionTypeIntroductoryPricing,
        #[doc = "The promotion type is unspecified."]
        PromotionTypeUnspecified,
    }
    impl GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecType :: PromotionTypeFreeTrial => "PROMOTION_TYPE_FREE_TRIAL" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecType :: PromotionTypeIntroductoryPricing => "PROMOTION_TYPE_INTRODUCTORY_PRICING" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecType :: PromotionTypeUnspecified => "PROMOTION_TYPE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecType
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecType
    {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecType,
            (),
        > {
            Ok (match s { "PROMOTION_TYPE_FREE_TRIAL" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecType :: PromotionTypeFreeTrial , "PROMOTION_TYPE_INTRODUCTORY_PRICING" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecType :: PromotionTypeIntroductoryPricing , "PROMOTION_TYPE_UNSPECIFIED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecType :: PromotionTypeUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecType
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "PROMOTION_TYPE_FREE_TRIAL" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecType :: PromotionTypeFreeTrial , "PROMOTION_TYPE_INTRODUCTORY_PRICING" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecType :: PromotionTypeIntroductoryPricing , "PROMOTION_TYPE_UNSPECIFIED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecType :: PromotionTypeUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionPromotionSpecType
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetails { # [doc = "Required. Specifies the billing cycle spec for the new upgraded/downgraded subscription."] # [serde (rename = "billingCycleSpec" , default , skip_serializing_if = "std::option::Option::is_none")] pub billing_cycle_spec : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailsBillingCycleSpec > , # [doc = "Required. The previous subscription id to be replaced. This is not the full resource name, use the subscription_id segment only."] # [serde (rename = "previousSubscriptionId" , default , skip_serializing_if = "std::option::Option::is_none")] pub previous_subscription_id : :: std :: option :: Option < String > , }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetails
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetails
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailsBillingCycleSpec
    {
        #[doc = "The billing cycle of the new subscription aligns with the previous subscription it upgrades or downgrades from."]
        BillingCycleSpecAlignWithPreviousSubscription,
        #[doc = "The billing cycle of the new subscription starts immediately."]
        BillingCycleSpecStartImmediately,
        #[doc = "Billing cycle spec is not specified."]
        BillingCycleSpecUnspecified,
    }
    impl GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailsBillingCycleSpec {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailsBillingCycleSpec :: BillingCycleSpecAlignWithPreviousSubscription => "BILLING_CYCLE_SPEC_ALIGN_WITH_PREVIOUS_SUBSCRIPTION" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailsBillingCycleSpec :: BillingCycleSpecStartImmediately => "BILLING_CYCLE_SPEC_START_IMMEDIATELY" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailsBillingCycleSpec :: BillingCycleSpecUnspecified => "BILLING_CYCLE_SPEC_UNSPECIFIED" , }
        }
    }
    impl :: std :: convert :: AsRef < str > for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailsBillingCycleSpec { fn as_ref (& self) -> & str { self . as_str () } }
    impl :: std :: str :: FromStr for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailsBillingCycleSpec { type Err = () ; fn from_str (s : & str) -> :: std :: result :: Result < GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailsBillingCycleSpec , () > { Ok (match s { "BILLING_CYCLE_SPEC_ALIGN_WITH_PREVIOUS_SUBSCRIPTION" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailsBillingCycleSpec :: BillingCycleSpecAlignWithPreviousSubscription , "BILLING_CYCLE_SPEC_START_IMMEDIATELY" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailsBillingCycleSpec :: BillingCycleSpecStartImmediately , "BILLING_CYCLE_SPEC_UNSPECIFIED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailsBillingCycleSpec :: BillingCycleSpecUnspecified , _ => return Err (()) , }) } }
    impl :: std :: fmt :: Display for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailsBillingCycleSpec { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> :: std :: fmt :: Result { f . write_str (self . as_str ()) } }
    impl :: serde :: Serialize for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailsBillingCycleSpec { fn serialize < S > (& self , serializer : S) -> :: std :: result :: Result < S :: Ok , S :: Error > where S : :: serde :: ser :: Serializer { serializer . serialize_str (self . as_str ()) } }
    impl < 'de > :: serde :: Deserialize < 'de > for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailsBillingCycleSpec { fn deserialize < D > (deserializer : D) -> :: std :: result :: Result < Self , D :: Error > where D : :: serde :: de :: Deserializer < 'de > , { let value : & 'de str = < & str > :: deserialize (deserializer) ? ; Ok (match value { "BILLING_CYCLE_SPEC_ALIGN_WITH_PREVIOUS_SUBSCRIPTION" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailsBillingCycleSpec :: BillingCycleSpecAlignWithPreviousSubscription , "BILLING_CYCLE_SPEC_START_IMMEDIATELY" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailsBillingCycleSpec :: BillingCycleSpecStartImmediately , "BILLING_CYCLE_SPEC_UNSPECIFIED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailsBillingCycleSpec :: BillingCycleSpecUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , }) } }
    impl :: google_field_selector :: FieldSelector for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailsBillingCycleSpec { fn fields () -> Vec < :: google_field_selector :: Field > { Vec :: new () } }
    impl :: google_field_selector :: ToFieldType for GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetailsBillingCycleSpec { fn field_type () -> :: google_field_selector :: FieldType { :: google_field_selector :: FieldType :: Leaf } }
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionRequest {}
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionRequest
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionRequest
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionResponse {
        #[doc = "The updated subscription resource."]
        #[serde(
            rename = "subscription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subscription: ::std::option::Option<
            crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Subscription,
        >,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionResponse
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1YoutubePayload {
        #[doc = "The list of eligibility_ids which are applicable for the line item."]
        #[serde(
            rename = "partnerEligibilityIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub partner_eligibility_ids: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleCloudPaymentsResellerSubscriptionV1YoutubePayload
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleCloudPaymentsResellerSubscriptionV1YoutubePayload
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
    pub struct GoogleTypeLocalizedText {
        #[doc = "The text’s BCP-47 language code, such as “en-US” or “sr-Latn”. For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Localized string in the language corresponding to \\`language_code’ below."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleTypeLocalizedText {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleTypeLocalizedText {
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
    #[doc = "Actions that can be performed on the partners resource"]
    pub fn partners(&self) -> crate::resources::partners::PartnersActions {
        crate::resources::partners::PartnersActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod partners {
        pub mod params {}
        pub struct PartnersActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> PartnersActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the products resource"]
            pub fn products(&self) -> crate::resources::partners::products::ProductsActions {
                crate::resources::partners::products::ProductsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the promotions resource"]
            pub fn promotions(&self) -> crate::resources::partners::promotions::PromotionsActions {
                crate::resources::partners::promotions::PromotionsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the subscriptions resource"]
            pub fn subscriptions(
                &self,
            ) -> crate::resources::partners::subscriptions::SubscriptionsActions {
                crate::resources::partners::subscriptions::SubscriptionsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod products {
            pub mod params {}
            pub struct ProductsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> ProductsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "To retrieve the products that can be resold by the partner. It should be autenticated with a service account."]
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
            }
            #[doc = "Created via [ProductsActions::list()](struct.ProductsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                filter: ::std::option::Option<String>,
                page_size: ::std::option::Option<i32>,
                page_token: ::std::option::Option<String>,
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
                #[doc = "Optional. Specifies the filters for the product results. The syntax is defined in https://google.aip.dev/160 with the following caveats: - Only the following features are supported: - Logical operator `AND` - Comparison operator `=` (no wildcards `*`) - Traversal operator `.` - Has operator `:` (no wildcards `*`) - Only the following fields are supported: - `regionCodes` - `youtubePayload.partnerEligibilityId` - `youtubePayload.postalCode` - Unless explicitly mentioned above, other features are not supported. Example: `regionCodes:US AND youtubePayload.postalCode=94043 AND youtubePayload.partnerEligibilityId=eligibility-id`"]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "Optional. The maximum number of products to return. The service may return fewer than this value. If unspecified, at most 50 products will be returned. The maximum value is 1000; values above 1000 will be coerced to 1000."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Optional. A page token, received from a previous `ListProducts` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListProducts` must match the call that provided the page token."]
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
                #[doc = "\nExecute the request and yield each item in the `products` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                pub fn stream_products<T>(
                    self,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector + 'a,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.stream_products_with_fields(fields)
                }
                #[doc = "\nExecute the request and yield each item in the `products` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                pub fn stream_products_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<
                        crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Product,
                        crate::Error,
                    >,
                > + 'a {
                    self.stream_products_with_fields(None::<String>)
                }
                #[doc = "\nExecute the request and yield each item in the `products` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                pub fn stream_products_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<
                        crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Product,
                        crate::Error,
                    >,
                > + 'a {
                    self.stream_products_with_fields(Some("*"))
                }
                #[doc = "\nExecute the request and yield each item in the `products` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                pub fn stream_products_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + 'a,
                    F: AsRef<str>,
                {
                    #[derive(:: serde :: Deserialize, :: serde :: Serialize)]
                    struct Page<T> {
                        #[serde(rename = "nextPageToken")]
                        pub next_page_token: ::std::option::Option<String>,
                        #[serde(rename = "products")]
                        pub items: Vec<T>,
                    }
                    impl<T> crate::GetNextPageToken<String> for Page<T> {
                        fn next_page_token(&self) -> ::std::option::Option<String> {
                            self.next_page_token.to_owned()
                        }
                    }
                    impl<T> crate::stream::IntoPageItems for Page<T> {
                        type Items = Vec<T>;
                        fn into_page_items(self) -> Self::Items {
                            self.items
                        }
                    }
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "products").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::stream::page_item_stream::<_, Page<T>>(self)
                }
                #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                #[doc = r" token is returned."]
                #[doc = r""]
                #[doc = r" Requests the field given by the [`FieldSelector`] implementation from the server."]
                #[doc = r""]
                #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                #[doc = r" [`FieldSelector`]: ::google_field_selector::FieldSelector"]
                pub fn stream<T>(
                    self,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: crate::GetNextPageToken<String>
                        + ::serde::de::DeserializeOwned
                        + ::google_field_selector::FieldSelector
                        + 'a,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.stream_with_fields(fields)
                }
                #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                #[doc = r" repeated until no page token is returned."]
                #[doc = r""]
                #[doc = r" Requests the default set of fields from the server."]                pub fn stream_with_default_fields (self) -> impl :: futures :: Stream < Item = Result < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1ListProductsResponse , crate :: Error >> + 'a{
                    self.stream_with_fields(None::<&str>)
                }
                #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                #[doc = r" repeated until no page token is returned."]
                #[doc = r""]
                #[doc = r" Requests all fields from the server."]                pub fn stream_with_all_fields (self) -> impl :: futures :: Stream < Item = Result < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1ListProductsResponse , crate :: Error >> + 'a{
                    self.stream_with_fields(Some("*"))
                }
                #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                #[doc = r" token is returned."]
                #[doc = r""]
                #[doc = r" Only the given `fields` are requested from the server. If the list of fields is not"]
                #[doc = r" empty, the `nextPageToken` field will be added to the list."]
                #[doc = r""]
                #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                pub fn stream_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: crate::GetNextPageToken<String> + ::serde::de::DeserializeOwned + 'a,
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
                    crate::stream::page_stream(self)
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
                ) -> Result<
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1ListProductsResponse,
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
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1ListProductsResponse,
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
                        "https://paymentsresellersubscription.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/products");
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
            #[async_trait::async_trait]
            impl<'a> crate::stream::StreamableMethod for ListRequestBuilder<'a> {
                type PageToken = String;
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                async fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: crate::GetNextPageToken<String> + ::serde::de::DeserializeOwned,
                {
                    self._execute().await
                }
            }
        }
        pub mod promotions {
            pub mod params {}
            pub struct PromotionsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> PromotionsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "To find eligible promotions for the current user. The API requires user authorization via OAuth. The user is inferred from the authenticated OAuth credential."]
                pub fn find_eligible(
                    &self,
                    request : crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1FindEligiblePromotionsRequest,
                    parent: impl Into<String>,
                ) -> FindEligibleRequestBuilder {
                    FindEligibleRequestBuilder {
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
                #[doc = "To retrieve the promotions, such as free trial, that can be used by the partner. It should be autenticated with a service account."]
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
            }
            #[doc = "Created via [PromotionsActions::find_eligible()](struct.PromotionsActions.html#method.find_eligible)"]
            #[derive(Debug, Clone)]
            pub struct FindEligibleRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , request : crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1FindEligiblePromotionsRequest , parent : String , access_token : :: std :: option :: Option < String > , alt : :: std :: option :: Option < crate :: params :: Alt > , callback : :: std :: option :: Option < String > , fields : :: std :: option :: Option < String > , key : :: std :: option :: Option < String > , oauth_token : :: std :: option :: Option < String > , pretty_print : :: std :: option :: Option < bool > , quota_user : :: std :: option :: Option < String > , upload_protocol : :: std :: option :: Option < String > , upload_type : :: std :: option :: Option < String > , xgafv : :: std :: option :: Option < crate :: params :: Xgafv > , }
            impl<'a> FindEligibleRequestBuilder<'a> {
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
                #[doc = r" the response resource."]                pub async fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1FindEligiblePromotionsResponse , crate :: Error >{
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]                pub async fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1FindEligiblePromotionsResponse , crate :: Error >{
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
                    let mut output =
                        "https://paymentsresellersubscription.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/promotions:findEligible");
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
            #[doc = "Created via [PromotionsActions::list()](struct.PromotionsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                filter: ::std::option::Option<String>,
                page_size: ::std::option::Option<i32>,
                page_token: ::std::option::Option<String>,
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
                #[doc = "Optional. Specifies the filters for the promotion results. The syntax is defined in https://google.aip.dev/160 with the following caveats: - Only the following features are supported: - Logical operator `AND` - Comparison operator `=` (no wildcards `*`) - Traversal operator `.` - Has operator `:` (no wildcards `*`) - Only the following fields are supported: - `applicableProducts` - `regionCodes` - `youtubePayload.partnerEligibilityId` - `youtubePayload.postalCode` - Unless explicitly mentioned above, other features are not supported. Example: `applicableProducts:partners/partner1/products/product1 AND regionCodes:US AND youtubePayload.postalCode=94043 AND youtubePayload.partnerEligibilityId=eligibility-id`"]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "Optional. The maximum number of promotions to return. The service may return fewer than this value. If unspecified, at most 50 products will be returned. The maximum value is 1000; values above 1000 will be coerced to 1000."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Optional. A page token, received from a previous `ListPromotions` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListPromotions` must match the call that provided the page token."]
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
                #[doc = "\nExecute the request and yield each item in the `promotions` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
                pub fn stream_promotions<T>(
                    self,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector + 'a,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.stream_promotions_with_fields(fields)
                }
                #[doc = "\nExecute the request and yield each item in the `promotions` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
                pub fn stream_promotions_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<
                        crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Promotion,
                        crate::Error,
                    >,
                > + 'a {
                    self.stream_promotions_with_fields(None::<String>)
                }
                #[doc = "\nExecute the request and yield each item in the `promotions` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                pub fn stream_promotions_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<
                        crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Promotion,
                        crate::Error,
                    >,
                > + 'a {
                    self.stream_promotions_with_fields(Some("*"))
                }
                #[doc = "\nExecute the request and yield each item in the `promotions` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
                pub fn stream_promotions_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: ::serde::de::DeserializeOwned + 'a,
                    F: AsRef<str>,
                {
                    #[derive(:: serde :: Deserialize, :: serde :: Serialize)]
                    struct Page<T> {
                        #[serde(rename = "nextPageToken")]
                        pub next_page_token: ::std::option::Option<String>,
                        #[serde(rename = "promotions")]
                        pub items: Vec<T>,
                    }
                    impl<T> crate::GetNextPageToken<String> for Page<T> {
                        fn next_page_token(&self) -> ::std::option::Option<String> {
                            self.next_page_token.to_owned()
                        }
                    }
                    impl<T> crate::stream::IntoPageItems for Page<T> {
                        type Items = Vec<T>;
                        fn into_page_items(self) -> Self::Items {
                            self.items
                        }
                    }
                    self.fields = Some({
                        let mut selector = concat!("nextPageToken,", "promotions").to_owned();
                        let items_fields = fields.as_ref().map(|x| x.as_ref()).unwrap_or("");
                        if !items_fields.is_empty() {
                            selector.push_str("(");
                            selector.push_str(items_fields);
                            selector.push_str(")");
                        }
                        selector
                    });
                    crate::stream::page_item_stream::<_, Page<T>>(self)
                }
                #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                #[doc = r" token is returned."]
                #[doc = r""]
                #[doc = r" Requests the field given by the [`FieldSelector`] implementation from the server."]
                #[doc = r""]
                #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                #[doc = r" [`FieldSelector`]: ::google_field_selector::FieldSelector"]
                pub fn stream<T>(
                    self,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: crate::GetNextPageToken<String>
                        + ::serde::de::DeserializeOwned
                        + ::google_field_selector::FieldSelector
                        + 'a,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: ::std::option::Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.stream_with_fields(fields)
                }
                #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                #[doc = r" repeated until no page token is returned."]
                #[doc = r""]
                #[doc = r" Requests the default set of fields from the server."]                pub fn stream_with_default_fields (self) -> impl :: futures :: Stream < Item = Result < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1ListPromotionsResponse , crate :: Error >> + 'a{
                    self.stream_with_fields(None::<&str>)
                }
                #[doc = r" Execute the request and yield the returned value. If the response contains a"]
                #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
                #[doc = r" repeated until no page token is returned."]
                #[doc = r""]
                #[doc = r" Requests all fields from the server."]                pub fn stream_with_all_fields (self) -> impl :: futures :: Stream < Item = Result < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1ListPromotionsResponse , crate :: Error >> + 'a{
                    self.stream_with_fields(Some("*"))
                }
                #[doc = r" Execute the request and yield the returned value. If [`next_page_token`] returns a value,"]
                #[doc = r" the request is executed again with the new token. This process is repeated until no page"]
                #[doc = r" token is returned."]
                #[doc = r""]
                #[doc = r" Only the given `fields` are requested from the server. If the list of fields is not"]
                #[doc = r" empty, the `nextPageToken` field will be added to the list."]
                #[doc = r""]
                #[doc = r" [`next_page_token`]: crate::GetNextPageToken::next_page_token"]
                pub fn stream_with_fields<T, F>(
                    mut self,
                    fields: ::std::option::Option<F>,
                ) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
                where
                    T: crate::GetNextPageToken<String> + ::serde::de::DeserializeOwned + 'a,
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
                    crate::stream::page_stream(self)
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
                ) -> Result<
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1ListPromotionsResponse,
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
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1ListPromotionsResponse,
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
                        "https://paymentsresellersubscription.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/promotions");
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
            #[async_trait::async_trait]
            impl<'a> crate::stream::StreamableMethod for ListRequestBuilder<'a> {
                type PageToken = String;
                fn set_page_token(&mut self, value: String) {
                    self.page_token = value.into();
                }
                async fn execute<T>(&mut self) -> Result<T, crate::Error>
                where
                    T: crate::GetNextPageToken<String> + ::serde::de::DeserializeOwned,
                {
                    self._execute().await
                }
            }
        }
        pub mod subscriptions {
            pub mod params {}
            pub struct SubscriptionsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> SubscriptionsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Used by partners to cancel a subscription service either immediately or by the end of the current billing cycle for their customers. It should be called directly by the partner using service accounts."]
                pub fn cancel(
                    &self,
                    request : crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequest,
                    name: impl Into<String>,
                ) -> CancelRequestBuilder {
                    CancelRequestBuilder {
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
                #[doc = "Used by partners to create a subscription for their customers. The created subscription is associated with the end user inferred from the end user credentials. This API must be authorized by the end user using OAuth."]
                pub fn create(
                    &self,
                    request: crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Subscription,
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
                        subscription_id: None,
                    }
                }
                #[doc = "Used by partners to entitle a previously provisioned subscription to the current end user. The end user identity is inferred from the authorized credential of the request. This API must be authorized by the end user using OAuth."]
                pub fn entitle(
                    &self,
                    request : crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1EntitleSubscriptionRequest,
                    name: impl Into<String>,
                ) -> EntitleRequestBuilder {
                    EntitleRequestBuilder {
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
                #[doc = "\\[Deprecated\\] New partners should be on auto-extend by default. Used by partners to extend a subscription service for their customers on an ongoing basis for the subscription to remain active and renewable. It should be called directly by the partner using service accounts."]
                pub fn extend(
                    &self,
                    request : crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionRequest,
                    name: impl Into<String>,
                ) -> ExtendRequestBuilder {
                    ExtendRequestBuilder {
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
                #[doc = "Used by partners to get a subscription by id. It should be called directly by the partner using service accounts."]
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
                #[doc = "Used by partners to provision a subscription for their customers. This creates a subscription without associating it with the end user account. EntitleSubscription must be called separately using OAuth in order for the end user account to be associated with the subscription. It should be called directly by the partner using service accounts."]
                pub fn provision(
                    &self,
                    request: crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Subscription,
                    parent: impl Into<String>,
                ) -> ProvisionRequestBuilder {
                    ProvisionRequestBuilder {
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
                        subscription_id: None,
                    }
                }
                #[doc = "Used by partners to revoke the pending cancellation of a subscription, which is currently in `STATE_CANCEL_AT_END_OF_CYCLE` state. If the subscription is already cancelled, the request will fail. It should be called directly by the partner using service accounts."]
                pub fn undo_cancel(
                    &self,
                    request : crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionRequest,
                    name: impl Into<String>,
                ) -> UndoCancelRequestBuilder {
                    UndoCancelRequestBuilder {
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
            #[doc = "Created via [SubscriptionsActions::cancel()](struct.SubscriptionsActions.html#method.cancel)"]
            #[derive(Debug, Clone)]
            pub struct CancelRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , request : crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequest , name : String , access_token : :: std :: option :: Option < String > , alt : :: std :: option :: Option < crate :: params :: Alt > , callback : :: std :: option :: Option < String > , fields : :: std :: option :: Option < String > , key : :: std :: option :: Option < String > , oauth_token : :: std :: option :: Option < String > , pretty_print : :: std :: option :: Option < bool > , quota_user : :: std :: option :: Option < String > , upload_protocol : :: std :: option :: Option < String > , upload_type : :: std :: option :: Option < String > , xgafv : :: std :: option :: Option < crate :: params :: Xgafv > , }
            impl<'a> CancelRequestBuilder<'a> {
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
                #[doc = r" the response resource."]                pub async fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionResponse , crate :: Error >{
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]                pub async fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionResponse , crate :: Error >{
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
                    let mut output =
                        "https://paymentsresellersubscription.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":cancel");
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
            #[doc = "Created via [SubscriptionsActions::create()](struct.SubscriptionsActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Subscription,
                parent: String,
                subscription_id: ::std::option::Option<String>,
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
            impl<'a> CreateRequestBuilder<'a> {
                #[doc = "Required. Identifies the subscription resource on the Partner side. The value is restricted to 63 ASCII characters at the maximum. If a subscription was previously created with the same subscription_id, we will directly return that one."]
                pub fn subscription_id(mut self, value: impl Into<String>) -> Self {
                    self.subscription_id = Some(value.into());
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
                ) -> Result<
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Subscription,
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
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Subscription,
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
                    let req = req.json(&self.request);
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output =
                        "https://paymentsresellersubscription.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/subscriptions");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                    req = req.query(&[("subscriptionId", &self.subscription_id)]);
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
            #[doc = "Created via [SubscriptionsActions::entitle()](struct.SubscriptionsActions.html#method.entitle)"]
            #[derive(Debug, Clone)]
            pub struct EntitleRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , request : crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1EntitleSubscriptionRequest , name : String , access_token : :: std :: option :: Option < String > , alt : :: std :: option :: Option < crate :: params :: Alt > , callback : :: std :: option :: Option < String > , fields : :: std :: option :: Option < String > , key : :: std :: option :: Option < String > , oauth_token : :: std :: option :: Option < String > , pretty_print : :: std :: option :: Option < bool > , quota_user : :: std :: option :: Option < String > , upload_protocol : :: std :: option :: Option < String > , upload_type : :: std :: option :: Option < String > , xgafv : :: std :: option :: Option < crate :: params :: Xgafv > , }
            impl<'a> EntitleRequestBuilder<'a> {
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
                #[doc = r" the response resource."]                pub async fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1EntitleSubscriptionResponse , crate :: Error >{
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]                pub async fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1EntitleSubscriptionResponse , crate :: Error >{
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
                    let mut output =
                        "https://paymentsresellersubscription.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":entitle");
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
            #[doc = "Created via [SubscriptionsActions::extend()](struct.SubscriptionsActions.html#method.extend)"]
            #[derive(Debug, Clone)]
            pub struct ExtendRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , request : crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionRequest , name : String , access_token : :: std :: option :: Option < String > , alt : :: std :: option :: Option < crate :: params :: Alt > , callback : :: std :: option :: Option < String > , fields : :: std :: option :: Option < String > , key : :: std :: option :: Option < String > , oauth_token : :: std :: option :: Option < String > , pretty_print : :: std :: option :: Option < bool > , quota_user : :: std :: option :: Option < String > , upload_protocol : :: std :: option :: Option < String > , upload_type : :: std :: option :: Option < String > , xgafv : :: std :: option :: Option < crate :: params :: Xgafv > , }
            impl<'a> ExtendRequestBuilder<'a> {
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
                #[doc = r" the response resource."]                pub async fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionResponse , crate :: Error >{
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]                pub async fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionResponse , crate :: Error >{
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
                    let mut output =
                        "https://paymentsresellersubscription.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":extend");
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
            #[doc = "Created via [SubscriptionsActions::get()](struct.SubscriptionsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
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
                ) -> Result<
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Subscription,
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
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Subscription,
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
                        "https://paymentsresellersubscription.googleapis.com/".to_owned();
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
            #[doc = "Created via [SubscriptionsActions::provision()](struct.SubscriptionsActions.html#method.provision)"]
            #[derive(Debug, Clone)]
            pub struct ProvisionRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Subscription,
                parent: String,
                subscription_id: ::std::option::Option<String>,
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
            impl<'a> ProvisionRequestBuilder<'a> {
                #[doc = "Required. Identifies the subscription resource on the Partner side. The value is restricted to 63 ASCII characters at the maximum. If a subscription was previously created with the same subscription_id, we will directly return that one."]
                pub fn subscription_id(mut self, value: impl Into<String>) -> Self {
                    self.subscription_id = Some(value.into());
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
                ) -> Result<
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Subscription,
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
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Subscription,
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
                    let req = req.json(&self.request);
                    Ok(req.send().await?.error_for_status()?.json().await?)
                }
                fn _path(&self) -> String {
                    let mut output =
                        "https://paymentsresellersubscription.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/subscriptions:provision");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                    req = req.query(&[("subscriptionId", &self.subscription_id)]);
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
            #[doc = "Created via [SubscriptionsActions::undo_cancel()](struct.SubscriptionsActions.html#method.undo_cancel)"]
            #[derive(Debug, Clone)]
            pub struct UndoCancelRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , request : crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionRequest , name : String , access_token : :: std :: option :: Option < String > , alt : :: std :: option :: Option < crate :: params :: Alt > , callback : :: std :: option :: Option < String > , fields : :: std :: option :: Option < String > , key : :: std :: option :: Option < String > , oauth_token : :: std :: option :: Option < String > , pretty_print : :: std :: option :: Option < bool > , quota_user : :: std :: option :: Option < String > , upload_protocol : :: std :: option :: Option < String > , upload_type : :: std :: option :: Option < String > , xgafv : :: std :: option :: Option < crate :: params :: Xgafv > , }
            impl<'a> UndoCancelRequestBuilder<'a> {
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
                #[doc = r" the response resource."]                pub async fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionResponse , crate :: Error >{
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]                pub async fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionResponse , crate :: Error >{
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
                    let mut output =
                        "https://paymentsresellersubscription.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str(":undoCancel");
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
/// Traits and functions to improve streamable (multiple page) API method handling.
pub mod stream {
    use super::GetNextPageToken;

    /// Extract the items embedded in a page like response.
    pub trait IntoPageItems {
        /// Type of the items list in the page.
        type Items: IntoIterator;

        /// Consume the response and return the embedded items.
        fn into_page_items(self) -> Self::Items;
    }

    /// Represent a API method which can be invoked multiple times to retrieve
    /// multiple pages of items.
    #[async_trait::async_trait]
    pub trait StreamableMethod {
        /// Type of the `pageToken` and `nextPageToken` fields.
        type PageToken;

        /// Update the current page token of the request.
        fn set_page_token(&mut self, value: Self::PageToken);

        /// Execute the request.
        async fn execute<T>(&mut self) -> Result<T, crate::Error>
        where
            T: GetNextPageToken<Self::PageToken> + ::serde::de::DeserializeOwned;
    }

    /// Return a [`Stream`](::futures::Stream) over all pages of the given API
    /// method.
    pub fn page_stream<M, T>(method: M) -> impl ::futures::Stream<Item = Result<T, crate::Error>>
    where
        M: StreamableMethod,
        T: GetNextPageToken<M::PageToken> + ::serde::de::DeserializeOwned,
    {
        ::futures::stream::unfold((method, false), |(mut method, mut finished)| async move {
            if finished {
                return None;
            }
            let response = match method.execute::<T>().await {
                Ok(r) => r,
                Err(err) => return Some((Err(err), (method, false))),
            };
            if let Some(next_page_token) = response.next_page_token() {
                method.set_page_token(next_page_token);
            } else {
                finished = true;
            }

            Some((Ok(response), (method, finished)))
        })
    }

    /// Return a [`Stream`](::futures::Stream) over the items in all pages of
    /// the given API method.
    pub fn page_item_stream<M, T>(
        method: M,
    ) -> impl ::futures::Stream<Item = Result<<T::Items as IntoIterator>::Item, crate::Error>>
    where
        M: StreamableMethod,
        T: GetNextPageToken<M::PageToken> + ::serde::de::DeserializeOwned + IntoPageItems,
    {
        use ::futures::StreamExt;
        use ::futures::TryStreamExt;

        page_stream::<M, T>(method)
            .map_ok(|page| ::futures::stream::iter(page.into_page_items()).map(Ok))
            .try_flatten()
    }
}
