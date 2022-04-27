#![doc = "# Resources and Methods\n    * [partners](resources/partners/struct.PartnersActions.html)\n      * [products](resources/partners/products/struct.ProductsActions.html)\n        * [*list*](resources/partners/products/struct.ListRequestBuilder.html)\n      * [promotions](resources/partners/promotions/struct.PromotionsActions.html)\n        * [*list*](resources/partners/promotions/struct.ListRequestBuilder.html)\n      * [subscriptions](resources/partners/subscriptions/struct.SubscriptionsActions.html)\n        * [*cancel*](resources/partners/subscriptions/struct.CancelRequestBuilder.html), [*create*](resources/partners/subscriptions/struct.CreateRequestBuilder.html), [*entitle*](resources/partners/subscriptions/struct.EntitleRequestBuilder.html), [*extend*](resources/partners/subscriptions/struct.ExtendRequestBuilder.html), [*get*](resources/partners/subscriptions/struct.GetRequestBuilder.html), [*provision*](resources/partners/subscriptions/struct.ProvisionRequestBuilder.html), [*undoCancel*](resources/partners/subscriptions/struct.UndoCancelRequestBuilder.html)\n"]
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequest { # [doc = "Optional. If true, the subscription will be cancelled immediately. Otherwise, the subscription will be cancelled at renewal_time, and therefore no prorated refund will be issued for the rest of the cycle."] # [serde (rename = "cancelImmediately" , default , skip_serializing_if = "std::option::Option::is_none")] pub cancel_immediately : :: std :: option :: Option < bool > , # [doc = "Specifies the reason for the cancellation."] # [serde (rename = "cancellationReason" , default , skip_serializing_if = "std::option::Option::is_none")] pub cancellation_reason : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequestCancellationReason > , }
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
        #[doc = "Buyer's remorse."]
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
        #[doc = "The time at which the subscription is expected to be extended, in ISO 8061 format. UTC timezone. Example, \"cycleEndTime\":\"2019-08-31T17:28:54.564Z\""]
        #[serde(
            rename = "cycleEndTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cycle_end_time: ::std::option::Option<String>,
        #[doc = "End of the free trial period, in ISO 8061 format. UTC timezone. Example, \"freeTrialEndTime\":\"2019-08-31T17:28:54.564Z\" This time will be set the same as initial subscription creation time if no free trial period is offered to the partner."]
        #[serde(
            rename = "freeTrialEndTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub free_trial_end_time: ::std::option::Option<String>,
        #[doc = "Output only. The time at which the subscription is expected to be renewed by Google - a new charge will be incurred and the service entitlement will be renewed. A non-immediate cancellation will take place at this time too, before which, the service entitlement for the end user will remain valid. UTC timezone in ISO 8061 format. For example: \"2019-08-31T17:28:54.564Z\""]
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
    #[derive(
        Debug,
        Clone,
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
    #[derive(
        Debug,
        Clone,
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
        #[doc = "The postal code this location refers to. Ex. \"94043\""]
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
        #[doc = "Output only. Response only. Resource name of the subscription. It will have the format of \"partners/{partner_id}/products/{product_id}\""]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. 2-letter ISO region code where the product is available in. Ex. \"US\" Please refers to: https://en.wikipedia.org/wiki/ISO_3166-1"]
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1Promotion { # [doc = "Output only. The product ids this promotion can be applied to."] # [serde (rename = "applicableProducts" , default , skip_serializing_if = "std::option::Option::is_none")] pub applicable_products : :: std :: option :: Option < Vec < String > > , # [doc = "Optional. Specifies the end time (exclusive) of the period that the promotion is available in. If unset, the promotion is available indefinitely."] # [serde (rename = "endTime" , default , skip_serializing_if = "std::option::Option::is_none")] pub end_time : :: std :: option :: Option < String > , # [doc = "Optional. Specifies the duration of the free trial of the subscription when promotion_type is PROMOTION_TYPE_FREE_TRIAL"] # [serde (rename = "freeTrialDuration" , default , skip_serializing_if = "std::option::Option::is_none")] pub free_trial_duration : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1Duration > , # [doc = "Optional. Specifies the introductory pricing details when the promotion_type is PROMOTION_TYPE_INTRODUCTORY_PRICING."] # [serde (rename = "introductoryPricingDetails" , default , skip_serializing_if = "std::option::Option::is_none")] pub introductory_pricing_details : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1PromotionIntroductoryPricingDetails > , # [doc = "Output only. Response only. Resource name of the subscription promotion. It will have the format of \"partners/{partner_id}/promotion/{promotion_id}\""] # [serde (rename = "name" , default , skip_serializing_if = "std::option::Option::is_none")] pub name : :: std :: option :: Option < String > , # [doc = "Output only. Output Only. Specifies the type of the promotion."] # [serde (rename = "promotionType" , default , skip_serializing_if = "std::option::Option::is_none")] pub promotion_type : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1PromotionPromotionType > , # [doc = "Output only. 2-letter ISO region code where the promotion is available in. Ex. \"US\" Please refers to: https://en.wikipedia.org/wiki/ISO_3166-1"] # [serde (rename = "regionCodes" , default , skip_serializing_if = "std::option::Option::is_none")] pub region_codes : :: std :: option :: Option < Vec < String > > , # [doc = "Optional. Specifies the start time (inclusive) of the period that the promotion is available in."] # [serde (rename = "startTime" , default , skip_serializing_if = "std::option::Option::is_none")] pub start_time : :: std :: option :: Option < String > , # [doc = "Output only. Localized human readable name of the promotion."] # [serde (rename = "titles" , default , skip_serializing_if = "std::option::Option::is_none")] pub titles : :: std :: option :: Option < Vec < crate :: schemas :: GoogleTypeLocalizedText > > , }
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
        #[doc = "Output only. Output Only. The duration of an introductory offer in billing cycles."]
        #[serde(
            rename = "recurrenceCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recurrence_count: ::std::option::Option<i32>,
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
    pub struct GoogleCloudPaymentsResellerSubscriptionV1Subscription { # [doc = "Output only. Describes the details of a cancelled subscription. Only applicable to subscription of state `STATE_CANCELLED`."] # [serde (rename = "cancellationDetails" , default , skip_serializing_if = "std::option::Option::is_none")] pub cancellation_details : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1SubscriptionCancellationDetails > , # [doc = "Output only. System generated timestamp when the subscription is created. UTC timezone."] # [serde (rename = "createTime" , default , skip_serializing_if = "std::option::Option::is_none")] pub create_time : :: std :: option :: Option < String > , # [doc = "Output only. The time at which the subscription is expected to be extended, in ISO 8061 format. UTC timezone. For example: \"2019-08-31T17:28:54.564Z\""] # [serde (rename = "cycleEndTime" , default , skip_serializing_if = "std::option::Option::is_none")] pub cycle_end_time : :: std :: option :: Option < String > , # [doc = "Output only. Indicates if the subscription is entitled to the end user."] # [serde (rename = "endUserEntitled" , default , skip_serializing_if = "std::option::Option::is_none")] pub end_user_entitled : :: std :: option :: Option < bool > , # [doc = "Output only. End of the free trial period, in ISO 8061 format. For example, \"2019-08-31T17:28:54.564Z\". It will be set the same as createTime if no free trial promotion is specified."] # [serde (rename = "freeTrialEndTime" , default , skip_serializing_if = "std::option::Option::is_none")] pub free_trial_end_time : :: std :: option :: Option < String > , # [doc = "Output only. Response only. Resource name of the subscription. It will have the format of \"partners/{partner_id}/subscriptions/{subscription_id}\""] # [serde (rename = "name" , default , skip_serializing_if = "std::option::Option::is_none")] pub name : :: std :: option :: Option < String > , # [doc = "Required. Identifier of the end-user in partner’s system. The value is restricted to 63 ASCII characters at the maximum."] # [serde (rename = "partnerUserToken" , default , skip_serializing_if = "std::option::Option::is_none")] pub partner_user_token : :: std :: option :: Option < String > , # [doc = "Output only. Describes the processing state of the subscription. See more details at [the lifecycle of a subscription](/payments/reseller/subscription/reference/index/Receive.Notifications#payments-subscription-lifecycle)."] # [serde (rename = "processingState" , default , skip_serializing_if = "std::option::Option::is_none")] pub processing_state : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1SubscriptionProcessingState > , # [doc = "Required. Required. Resource name that identifies the purchased products. The format will be 'partners/{partner_id}/products/{product_id}'."] # [serde (rename = "products" , default , skip_serializing_if = "std::option::Option::is_none")] pub products : :: std :: option :: Option < Vec < String > > , # [doc = "Optional. Optional. Resource name that identifies one or more promotions that can be applied on the product. A typical promotion for a subscription is Free trial. The format will be 'partners/{partner_id}/promotions/{promotion_id}'."] # [serde (rename = "promotions" , default , skip_serializing_if = "std::option::Option::is_none")] pub promotions : :: std :: option :: Option < Vec < String > > , # [doc = "Output only. The place where partners should redirect the end-user to after creation. This field might also be populated when creation failed. However, Partners should always prepare a default URL to redirect the user in case this field is empty."] # [serde (rename = "redirectUri" , default , skip_serializing_if = "std::option::Option::is_none")] pub redirect_uri : :: std :: option :: Option < String > , # [doc = "Output only. The time at which the subscription is expected to be renewed by Google - a new charge will be incurred and the service entitlement will be renewed. A non-immediate cancellation will take place at this time too, before which, the service entitlement for the end user will remain valid. UTC timezone in ISO 8061 format. For example: \"2019-08-31T17:28:54.564Z\""] # [serde (rename = "renewalTime" , default , skip_serializing_if = "std::option::Option::is_none")] pub renewal_time : :: std :: option :: Option < String > , # [doc = "Required. The location that the service is provided as indicated by the partner."] # [serde (rename = "serviceLocation" , default , skip_serializing_if = "std::option::Option::is_none")] pub service_location : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1Location > , # [doc = "Output only. Describes the state of the subscription. See more details at [the lifecycle of a subscription](/payments/reseller/subscription/reference/index/Receive.Notifications#payments-subscription-lifecycle)."] # [serde (rename = "state" , default , skip_serializing_if = "std::option::Option::is_none")] pub state : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState > , # [doc = "Output only. System generated timestamp when the subscription is most recently updated. UTC timezone."] # [serde (rename = "updateTime" , default , skip_serializing_if = "std::option::Option::is_none")] pub update_time : :: std :: option :: Option < String > , # [doc = "Optional. Details about the previous subscription that this new subscription upgrades/downgrades from. Only populated if this subscription is an upgrade/downgrade from another subscription."] # [serde (rename = "upgradeDowngradeDetails" , default , skip_serializing_if = "std::option::Option::is_none")] pub upgrade_downgrade_details : :: std :: option :: Option < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1SubscriptionUpgradeDowngradeDetails > , }
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
        #[doc = "The state is unspecified."]
        StateUnspecified,
    }
    impl GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState {
        pub fn as_str(self) -> &'static str {
            match self { GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateActive => "STATE_ACTIVE" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateCancelAtEndOfCycle => "STATE_CANCEL_AT_END_OF_CYCLE" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateCancelled => "STATE_CANCELLED" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateCreated => "STATE_CREATED" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateInGracePeriod => "STATE_IN_GRACE_PERIOD" , GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateUnspecified => "STATE_UNSPECIFIED" , }
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
            Ok (match s { "STATE_ACTIVE" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateActive , "STATE_CANCEL_AT_END_OF_CYCLE" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateCancelAtEndOfCycle , "STATE_CANCELLED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateCancelled , "STATE_CREATED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateCreated , "STATE_IN_GRACE_PERIOD" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateInGracePeriod , "STATE_UNSPECIFIED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateUnspecified , _ => return Err (()) , })
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
            Ok (match value { "STATE_ACTIVE" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateActive , "STATE_CANCEL_AT_END_OF_CYCLE" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateCancelAtEndOfCycle , "STATE_CANCELLED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateCancelled , "STATE_CREATED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateCreated , "STATE_IN_GRACE_PERIOD" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateInGracePeriod , "STATE_UNSPECIFIED" => GoogleCloudPaymentsResellerSubscriptionV1SubscriptionState :: StateUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
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
        #[doc = "Buyer's remorse."]
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
    pub struct GoogleTypeLocalizedText {
        #[doc = "The text's BCP-47 language code, such as \"en-US\" or \"sr-Latn\". For more information, see http://www.unicode.org/reports/tr35/#Unicode_locale_identifier."]
        #[serde(
            rename = "languageCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language_code: ::std::option::Option<String>,
        #[doc = "Localized string in the language corresponding to `language_code' below."]
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
            pub(crate) reqwest: &'a reqwest::blocking::Client,
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
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> ProductsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Used by partners to list products that can be resold to their customers. It should be called directly by the partner using service accounts."]
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
            #[doc = "Created via [ProductsActions::list()](struct.ProductsActions.html#method.list)"]
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
                pub fn iter_products<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_products_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_products_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<
                    Self,
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Product,
                > {
                    self.iter_products_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_products_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<
                    Self,
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Product,
                > {
                    self.iter_products_with_fields(Some("*"))
                }
                pub fn iter_products_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
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
                    crate::iter::PageItemIter::new(self, "products")
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
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1ListProductsResponse,
                > {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1ListProductsResponse,
                > {
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
                ) -> Result<
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1ListProductsResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1ListProductsResponse,
                    crate::Error,
                > {
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
        pub mod promotions {
            pub mod params {}
            pub struct PromotionsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> PromotionsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Used by partners to list promotions, such as free trial, that can be applied on subscriptions. It should be called directly by the partner using service accounts."]
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
            #[doc = "Created via [PromotionsActions::list()](struct.PromotionsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
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
                #[doc = "Optional. Specifies the filters for the promotion results. The syntax defined in the EBNF grammar: https://google.aip.dev/assets/misc/ebnf-filtering.txt. Examples: - applicable_products: \"sku1\" - region_codes: \"US\" - applicable_products: \"sku1\" AND region_codes: \"US\""]
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
                pub fn iter_promotions<T>(self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::google_field_selector::FieldSelector,
                {
                    let fields = ::google_field_selector::to_string::<T>();
                    let fields: Option<String> = if fields.is_empty() {
                        None
                    } else {
                        Some(fields)
                    };
                    self.iter_promotions_with_fields(fields)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_promotions_with_default_fields(
                    self,
                ) -> crate::iter::PageItemIter<
                    Self,
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Promotion,
                > {
                    self.iter_promotions_with_fields(None::<String>)
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_promotions_with_all_fields(
                    self,
                ) -> crate::iter::PageItemIter<
                    Self,
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Promotion,
                > {
                    self.iter_promotions_with_fields(Some("*"))
                }
                pub fn iter_promotions_with_fields<T, F>(
                    mut self,
                    fields: Option<F>,
                ) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned,
                    F: AsRef<str>,
                {
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
                    crate::iter::PageItemIter::new(self, "promotions")
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
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1ListPromotionsResponse,
                > {
                    self.iter_with_fields(None::<&str>)
                }
                pub fn iter_with_all_fields(
                    self,
                ) -> crate::iter::PageIter<
                    Self,
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1ListPromotionsResponse,
                > {
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
                ) -> Result<
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1ListPromotionsResponse,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1ListPromotionsResponse,
                    crate::Error,
                > {
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
        pub mod subscriptions {
            pub mod params {}
            pub struct SubscriptionsActions<'a> {
                pub(crate) reqwest: &'a reqwest::blocking::Client,
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
                #[doc = "Used by partners to extend a subscription service for their customers on an ongoing basis for the subscription to remain active and renewable. It should be called directly by the partner using service accounts."]
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
            pub struct CancelRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: blocking :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , request : crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionRequest , name : String , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
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
                #[doc = r" the response resource."]                pub fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionResponse , crate :: Error >{
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]                pub fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1CancelSubscriptionResponse , crate :: Error >{
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
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [SubscriptionsActions::create()](struct.SubscriptionsActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Subscription,
                parent: String,
                subscription_id: Option<String>,
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
                ) -> Result<
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Subscription,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Subscription,
                    crate::Error,
                > {
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
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [SubscriptionsActions::entitle()](struct.SubscriptionsActions.html#method.entitle)"]
            #[derive(Debug, Clone)]
            pub struct EntitleRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: blocking :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , request : crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1EntitleSubscriptionRequest , name : String , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
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
                #[doc = r" the response resource."]                pub fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1EntitleSubscriptionResponse , crate :: Error >{
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]                pub fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1EntitleSubscriptionResponse , crate :: Error >{
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
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [SubscriptionsActions::extend()](struct.SubscriptionsActions.html#method.extend)"]
            #[derive(Debug, Clone)]
            pub struct ExtendRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: blocking :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , request : crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionRequest , name : String , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
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
                #[doc = r" the response resource."]                pub fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionResponse , crate :: Error >{
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]                pub fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1ExtendSubscriptionResponse , crate :: Error >{
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
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [SubscriptionsActions::get()](struct.SubscriptionsActions.html#method.get)"]
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
                ) -> Result<
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Subscription,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Subscription,
                    crate::Error,
                > {
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
            #[doc = "Created via [SubscriptionsActions::provision()](struct.SubscriptionsActions.html#method.provision)"]
            #[derive(Debug, Clone)]
            pub struct ProvisionRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::blocking::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Subscription,
                parent: String,
                subscription_id: Option<String>,
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
                ) -> Result<
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Subscription,
                    crate::Error,
                > {
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_with_all_fields(
                    self,
                ) -> Result<
                    crate::schemas::GoogleCloudPaymentsResellerSubscriptionV1Subscription,
                    crate::Error,
                > {
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
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
                    Ok(req)
                }
            }
            #[doc = "Created via [SubscriptionsActions::undo_cancel()](struct.SubscriptionsActions.html#method.undo_cancel)"]
            #[derive(Debug, Clone)]
            pub struct UndoCancelRequestBuilder < 'a > { pub (crate) reqwest : & 'a :: reqwest :: blocking :: Client , pub (crate) auth : & 'a dyn :: google_api_auth :: GetAccessToken , request : crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionRequest , name : String , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
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
                #[doc = r" the response resource."]                pub fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionResponse , crate :: Error >{
                    self.execute_with_fields(None::<&str>)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]                pub fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleCloudPaymentsResellerSubscriptionV1UndoCancelSubscriptionResponse , crate :: Error >{
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
                    let req = req.json(&self.request);
                    Ok(crate::error_from_response(req.send()?)?.json()?)
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
                fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::blocking::RequestBuilder, crate::Error> {
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
                    req = req.bearer_auth(
                        self.auth
                            .access_token()
                            .map_err(|err| crate::Error::OAuth2(err))?,
                    );
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
