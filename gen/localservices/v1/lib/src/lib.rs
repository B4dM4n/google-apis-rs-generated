#![doc = "# Resources and Methods\n* [account_reports](resources/account_reports/struct.AccountReportsActions.html)\n  * [*search*](resources/account_reports/struct.SearchRequestBuilder.html)\n* [detailed_lead_reports](resources/detailed_lead_reports/struct.DetailedLeadReportsActions.html)\n  * [*search*](resources/detailed_lead_reports/struct.SearchRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "Manage your AdWords campaigns\n\n`https://www.googleapis.com/auth/adwords`"]
    pub const ADWORDS: &str = "https://www.googleapis.com/auth/adwords";
}
pub mod schemas {
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAdsHomeservicesLocalservicesV1AccountReport {
        #[doc = "Unique identifier of the GLS account."]
        #[serde(
            rename = "accountId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub account_id: ::std::option::Option<i64>,
        #[doc = "Aggregator specific information related to the account."]
        #[serde(
            rename = "aggregatorInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub aggregator_info: ::std::option::Option<
            crate::schemas::GoogleAdsHomeservicesLocalservicesV1AggregatorInfo,
        >,
        #[doc = "Average review rating score from 1-5 stars."]
        #[serde(
            rename = "averageFiveStarRating",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub average_five_star_rating: ::std::option::Option<f64>,
        #[doc = "Average weekly budget in the currency code of the account."]
        #[serde(
            rename = "averageWeeklyBudget",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub average_weekly_budget: ::std::option::Option<f64>,
        #[doc = "Business name of the account."]
        #[serde(
            rename = "businessName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub business_name: ::std::option::Option<String>,
        #[doc = "Currency code of the account."]
        #[serde(
            rename = "currencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency_code: ::std::option::Option<String>,
        #[doc = "Number of charged leads the account received in current specified period."]
        #[serde(
            rename = "currentPeriodChargedLeads",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub current_period_charged_leads: ::std::option::Option<i64>,
        #[doc = "Number of connected phone calls (duration over 30s) in current specified period."]
        #[serde(
            rename = "currentPeriodConnectedPhoneCalls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub current_period_connected_phone_calls: ::std::option::Option<i64>,
        #[doc = "Number of phone calls in current specified period, including both connected and unconnected calls."]
        #[serde(
            rename = "currentPeriodPhoneCalls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub current_period_phone_calls: ::std::option::Option<i64>,
        #[doc = "Total cost of the account in current specified period in the account's specified currency."]
        #[serde(
            rename = "currentPeriodTotalCost",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub current_period_total_cost: ::std::option::Option<f64>,
        #[doc = "Number of impressions that customers have had in the past 2 days."]
        #[serde(
            rename = "impressionsLastTwoDays",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub impressions_last_two_days: ::std::option::Option<i64>,
        #[doc = "Phone lead responsiveness of the account for the past 90 days from current date. This is computed by taking the total number of connected calls from charged phone leads and dividing by the total number of calls received."]
        #[serde(
            rename = "phoneLeadResponsiveness",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub phone_lead_responsiveness: ::std::option::Option<f64>,
        #[doc = "Number of charged leads the account received in previous specified period."]
        #[serde(
            rename = "previousPeriodChargedLeads",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub previous_period_charged_leads: ::std::option::Option<i64>,
        #[doc = "Number of connected phone calls (duration over 30s) in previous specified period."]
        #[serde(
            rename = "previousPeriodConnectedPhoneCalls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub previous_period_connected_phone_calls: ::std::option::Option<i64>,
        #[doc = "Number of phone calls in previous specified period, including both connected and unconnected calls."]
        #[serde(
            rename = "previousPeriodPhoneCalls",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub previous_period_phone_calls: ::std::option::Option<i64>,
        #[doc = "Total cost of the account in previous specified period in the account's specified currency."]
        #[serde(
            rename = "previousPeriodTotalCost",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub previous_period_total_cost: ::std::option::Option<f64>,
        #[doc = "Total number of reviews the account has up to current date."]
        #[serde(
            rename = "totalReview",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_review: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsHomeservicesLocalservicesV1AccountReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsHomeservicesLocalservicesV1AccountReport {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAdsHomeservicesLocalservicesV1AggregatorInfo {
        #[doc = "Provider id (listed in aggregator system) which maps to a account id in GLS system."]
        #[serde(
            rename = "aggregatorProviderId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub aggregator_provider_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsHomeservicesLocalservicesV1AggregatorInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsHomeservicesLocalservicesV1AggregatorInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAdsHomeservicesLocalservicesV1BookingLead {
        #[doc = "Timestamp of when service is provided by advertiser."]
        #[serde(
            rename = "bookingAppointmentTimestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub booking_appointment_timestamp: ::std::option::Option<String>,
        #[doc = "Consumer email associated with the booking lead."]
        #[serde(
            rename = "consumerEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub consumer_email: ::std::option::Option<String>,
        #[doc = "Consumer phone number associated with the booking lead."]
        #[serde(
            rename = "consumerPhoneNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub consumer_phone_number: ::std::option::Option<String>,
        #[doc = "Name of the customer who created the lead."]
        #[serde(
            rename = "customerName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub customer_name: ::std::option::Option<String>,
        #[doc = "The job type of the specified lead."]
        #[serde(
            rename = "jobType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsHomeservicesLocalservicesV1BookingLead {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsHomeservicesLocalservicesV1BookingLead {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAdsHomeservicesLocalservicesV1DetailedLeadReport {
        #[doc = "Identifies account that received the lead."]
        #[serde(
            rename = "accountId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub account_id: ::std::option::Option<i64>,
        #[doc = "Aggregator specific information related to the lead."]
        #[serde(
            rename = "aggregatorInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub aggregator_info: ::std::option::Option<
            crate::schemas::GoogleAdsHomeservicesLocalservicesV1AggregatorInfo,
        >,
        #[doc = "More information associated to only booking leads."]
        #[serde(
            rename = "bookingLead",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub booking_lead:
            ::std::option::Option<crate::schemas::GoogleAdsHomeservicesLocalservicesV1BookingLead>,
        #[doc = "Business name associated to the account."]
        #[serde(
            rename = "businessName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub business_name: ::std::option::Option<String>,
        #[doc = "Whether the lead has been charged."]
        #[serde(
            rename = "chargeStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub charge_status: ::std::option::Option<
            crate::schemas::GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatus,
        >,
        #[doc = "Currency code."]
        #[serde(
            rename = "currencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency_code: ::std::option::Option<String>,
        #[doc = "Dispute status related to the lead."]
        #[serde(
            rename = "disputeStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dispute_status: ::std::option::Option<String>,
        #[doc = "Location of the associated account's home city."]
        #[serde(
            rename = "geo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub geo: ::std::option::Option<String>,
        #[doc = "Lead category (e.g. hvac, plumber)"]
        #[serde(
            rename = "leadCategory",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lead_category: ::std::option::Option<String>,
        #[doc = "Timestamp of when the lead was created."]
        #[serde(
            rename = "leadCreationTimestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lead_creation_timestamp: ::std::option::Option<String>,
        #[doc = "Unique identifier of a Detailed Lead Report."]
        #[serde(
            rename = "leadId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub lead_id: ::std::option::Option<i64>,
        #[doc = "Price of the lead (available only after it has been charged)."]
        #[serde(
            rename = "leadPrice",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lead_price: ::std::option::Option<f64>,
        #[doc = "Lead type."]
        #[serde(
            rename = "leadType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lead_type: ::std::option::Option<
            crate::schemas::GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadType,
        >,
        #[doc = "More information associated to only message leads."]
        #[serde(
            rename = "messageLead",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message_lead:
            ::std::option::Option<crate::schemas::GoogleAdsHomeservicesLocalservicesV1MessageLead>,
        #[doc = "More information associated to only phone leads."]
        #[serde(
            rename = "phoneLead",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub phone_lead:
            ::std::option::Option<crate::schemas::GoogleAdsHomeservicesLocalservicesV1PhoneLead>,
        #[doc = "Timezone of the particular provider associated to a lead."]
        #[serde(
            rename = "timezone",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timezone: ::std::option::Option<crate::schemas::GoogleTypeTimeZone>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsHomeservicesLocalservicesV1DetailedLeadReport
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsHomeservicesLocalservicesV1DetailedLeadReport
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatus {
        #[doc = "Not specified."]
        ChargeStatusUnspecified,
        #[doc = "Charged."]
        Charged,
        #[doc = "Not charged."]
        NotCharged,
    }
    impl GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatus {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatus :: ChargeStatusUnspecified => "CHARGE_STATUS_UNSPECIFIED" , GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatus :: Charged => "CHARGED" , GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatus :: NotCharged => "NOT_CHARGED" , }
        }
    }
    impl ::std::convert::AsRef<str>
        for GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatus
    {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatus {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<
            GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatus,
            (),
        > {
            Ok (match s { "CHARGE_STATUS_UNSPECIFIED" => GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatus :: ChargeStatusUnspecified , "CHARGED" => GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatus :: Charged , "NOT_CHARGED" => GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatus :: NotCharged , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatus
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CHARGE_STATUS_UNSPECIFIED" => GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatus :: ChargeStatusUnspecified , "CHARGED" => GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatus :: Charged , "NOT_CHARGED" => GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatus :: NotCharged , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatus
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportChargeStatus
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadType {
        #[doc = "Booking lead."]
        Booking,
        #[doc = "Not specified."]
        LeadTypeUnspecified,
        #[doc = "Message lead."]
        Message,
        #[doc = "Phone call lead."]
        PhoneCall,
    }
    impl GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadType {
        pub fn as_str(self) -> &'static str {
            match self { GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadType :: Booking => "BOOKING" , GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadType :: LeadTypeUnspecified => "LEAD_TYPE_UNSPECIFIED" , GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadType :: Message => "MESSAGE" , GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadType :: PhoneCall => "PHONE_CALL" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadType, ()>
        {
            Ok (match s { "BOOKING" => GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadType :: Booking , "LEAD_TYPE_UNSPECIFIED" => GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadType :: LeadTypeUnspecified , "MESSAGE" => GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadType :: Message , "PHONE_CALL" => GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadType :: PhoneCall , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadType
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "BOOKING" => GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadType :: Booking , "LEAD_TYPE_UNSPECIFIED" => GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadType :: LeadTypeUnspecified , "MESSAGE" => GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadType :: Message , "PHONE_CALL" => GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadType :: PhoneCall , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadType
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsHomeservicesLocalservicesV1DetailedLeadReportLeadType
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
    pub struct GoogleAdsHomeservicesLocalservicesV1MessageLead {
        #[doc = "Consumer phone number associated with the message lead."]
        #[serde(
            rename = "consumerPhoneNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub consumer_phone_number: ::std::option::Option<String>,
        #[doc = "Name of the customer who created the lead."]
        #[serde(
            rename = "customerName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub customer_name: ::std::option::Option<String>,
        #[doc = "The job type of the specified lead."]
        #[serde(
            rename = "jobType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub job_type: ::std::option::Option<String>,
        #[doc = "The postal code of the customer who created the lead."]
        #[serde(
            rename = "postalCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub postal_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsHomeservicesLocalservicesV1MessageLead {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsHomeservicesLocalservicesV1MessageLead {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleAdsHomeservicesLocalservicesV1PhoneLead {
        #[doc = "Timestamp of the phone call which resulted in a charged phone lead."]
        #[serde(
            rename = "chargedCallTimestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub charged_call_timestamp: ::std::option::Option<String>,
        #[doc = "Duration of the charged phone call in seconds."]
        #[serde(
            rename = "chargedConnectedCallDurationSeconds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub charged_connected_call_duration_seconds: ::std::option::Option<String>,
        #[doc = "Consumer phone number associated with the phone lead."]
        #[serde(
            rename = "consumerPhoneNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub consumer_phone_number: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleAdsHomeservicesLocalservicesV1PhoneLead {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleAdsHomeservicesLocalservicesV1PhoneLead {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAdsHomeservicesLocalservicesV1SearchAccountReportsResponse {
        #[doc = "List of account reports which maps 1:1 to a particular linked GLS account."]
        #[serde(
            rename = "accountReports",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub account_reports: ::std::option::Option<
            Vec<crate::schemas::GoogleAdsHomeservicesLocalservicesV1AccountReport>,
        >,
        #[doc = "Pagination token to retrieve the next page of results. When `next_page_token` is not filled in, there is no next page and the list returned is the last page in the result set."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsHomeservicesLocalservicesV1SearchAccountReportsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsHomeservicesLocalservicesV1SearchAccountReportsResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken for GoogleAdsHomeservicesLocalservicesV1SearchAccountReportsResponse {
        fn next_page_token(&self) -> ::std::option::Option<String> {
            self.next_page_token.to_owned()
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct GoogleAdsHomeservicesLocalservicesV1SearchDetailedLeadReportsResponse {
        #[doc = "List of detailed lead reports uniquely identified by external lead id."]
        #[serde(
            rename = "detailedLeadReports",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detailed_lead_reports: ::std::option::Option<
            Vec<crate::schemas::GoogleAdsHomeservicesLocalservicesV1DetailedLeadReport>,
        >,
        #[doc = "Pagination token to retrieve the next page of results. When `next_page_token` is not filled in, there is no next page and the list returned is the last page in the result set."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleAdsHomeservicesLocalservicesV1SearchDetailedLeadReportsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleAdsHomeservicesLocalservicesV1SearchDetailedLeadReportsResponse
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken
        for GoogleAdsHomeservicesLocalservicesV1SearchDetailedLeadReportsResponse
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
    pub struct GoogleTypeTimeZone {
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
    impl ::google_field_selector::FieldSelector for GoogleTypeTimeZone {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleTypeTimeZone {
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
    #[doc = "Actions that can be performed on the account_reports resource"]
    pub fn account_reports(&self) -> crate::resources::account_reports::AccountReportsActions {
        crate::resources::account_reports::AccountReportsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the detailed_lead_reports resource"]
    pub fn detailed_lead_reports(
        &self,
    ) -> crate::resources::detailed_lead_reports::DetailedLeadReportsActions {
        crate::resources::detailed_lead_reports::DetailedLeadReportsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod account_reports {
        pub mod params {}
        pub struct AccountReportsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> AccountReportsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Get account reports containing aggregate account data of all linked GLS accounts. Caller needs to provide their manager customer id and the associated auth credential that allows them read permissions on their linked accounts."]
            pub fn search(&self) -> SearchRequestBuilder {
                SearchRequestBuilder {
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
                    page_size: None,
                    page_token: None,
                    query: None,
                    start_date_day: None,
                    start_date_month: None,
                    start_date_year: None,
                }
            }
        }
        #[doc = "Created via [AccountReportsActions::search()](struct.AccountReportsActions.html#method.search)"]
        #[derive(Debug, Clone)]
        pub struct SearchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            end_date_day: ::std::option::Option<i32>,
            end_date_month: ::std::option::Option<i32>,
            end_date_year: ::std::option::Option<i32>,
            page_size: ::std::option::Option<i32>,
            page_token: ::std::option::Option<String>,
            query: ::std::option::Option<String>,
            start_date_day: ::std::option::Option<i32>,
            start_date_month: ::std::option::Option<i32>,
            start_date_year: ::std::option::Option<i32>,
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
        impl<'a> SearchRequestBuilder<'a> {
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
            #[doc = "The maximum number of accounts to return. If the page size is unset, page size will default to 1000. Maximum page_size is 10000. Optional."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "The `next_page_token` value returned from a previous request to SearchAccountReports that indicates where listing should continue. Optional."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "A query string for searching for account reports. Caller must provide a customer id of their MCC account with an associated Gaia Mint that allows read permission on their linked accounts. Search expressions are case insensitive. Example query: | Query | Description | |-------------------------|-----------------------------------------------| | manager_customer_id:123 | Get Account Report for Manager with id 123. | Required."]
            pub fn query(mut self, value: impl Into<String>) -> Self {
                self.query = Some(value.into());
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
            #[doc = "\nExecute the request and yield each item in the `accountReports` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
            pub fn stream_account_reports<T>(
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
                self.stream_account_reports_with_fields(fields)
            }
            #[doc = "\nExecute the request and yield each item in the `accountReports` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
            pub fn stream_account_reports_with_default_fields(
                self,
            ) -> impl ::futures::Stream<
                Item = Result<
                    crate::schemas::GoogleAdsHomeservicesLocalservicesV1AccountReport,
                    crate::Error,
                >,
            > + 'a {
                self.stream_account_reports_with_fields(None::<String>)
            }
            #[doc = "\nExecute the request and yield each item in the `accountReports` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
            pub fn stream_account_reports_with_all_fields(
                self,
            ) -> impl ::futures::Stream<
                Item = Result<
                    crate::schemas::GoogleAdsHomeservicesLocalservicesV1AccountReport,
                    crate::Error,
                >,
            > + 'a {
                self.stream_account_reports_with_fields(Some("*"))
            }
            #[doc = "\nExecute the request and yield each item in the `accountReports` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
            pub fn stream_account_reports_with_fields<T, F>(
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
                    #[serde(rename = "accountReports")]
                    pub items: Vec<T>,
                }
                impl<T> crate::GetNextPageToken for Page<T> {
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
                    let mut selector = concat!("nextPageToken,", "accountReports").to_owned();
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
            pub fn stream<T>(self) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
            where
                T: crate::GetNextPageToken
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
            #[doc = r" Requests the default set of fields from the server."]            pub fn stream_with_default_fields (self) -> impl :: futures :: Stream < Item = Result < crate :: schemas :: GoogleAdsHomeservicesLocalservicesV1SearchAccountReportsResponse , crate :: Error >> + 'a{
                self.stream_with_fields(None::<&str>)
            }
            #[doc = r" Execute the request and yield the returned value. If the response contains a"]
            #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
            #[doc = r" repeated until no page token is returned."]
            #[doc = r""]
            #[doc = r" Requests all fields from the server."]            pub fn stream_with_all_fields (self) -> impl :: futures :: Stream < Item = Result < crate :: schemas :: GoogleAdsHomeservicesLocalservicesV1SearchAccountReportsResponse , crate :: Error >> + 'a{
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
                T: crate::GetNextPageToken + ::serde::de::DeserializeOwned + 'a,
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
                crate::schemas::GoogleAdsHomeservicesLocalservicesV1SearchAccountReportsResponse,
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
                crate::schemas::GoogleAdsHomeservicesLocalservicesV1SearchAccountReportsResponse,
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
                let mut output = "https://localservices.googleapis.com/".to_owned();
                output.push_str("v1/accountReports:search");
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
                req = req.query(&[("pageSize", &self.page_size)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("query", &self.query)]);
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
        #[async_trait::async_trait]
        impl<'a> crate::stream::StreamableMethod for SearchRequestBuilder<'a> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            async fn execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: crate::GetNextPageToken + ::serde::de::DeserializeOwned,
            {
                self._execute().await
            }
        }
    }
    pub mod detailed_lead_reports {
        pub mod params {}
        pub struct DetailedLeadReportsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> DetailedLeadReportsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Get detailed lead reports containing leads that have been received by all linked GLS accounts. Caller needs to provide their manager customer id and the associated auth credential that allows them read permissions on their linked accounts."]
            pub fn search(&self) -> SearchRequestBuilder {
                SearchRequestBuilder {
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
                    page_size: None,
                    page_token: None,
                    query: None,
                    start_date_day: None,
                    start_date_month: None,
                    start_date_year: None,
                }
            }
        }
        #[doc = "Created via [DetailedLeadReportsActions::search()](struct.DetailedLeadReportsActions.html#method.search)"]
        #[derive(Debug, Clone)]
        pub struct SearchRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            end_date_day: ::std::option::Option<i32>,
            end_date_month: ::std::option::Option<i32>,
            end_date_year: ::std::option::Option<i32>,
            page_size: ::std::option::Option<i32>,
            page_token: ::std::option::Option<String>,
            query: ::std::option::Option<String>,
            start_date_day: ::std::option::Option<i32>,
            start_date_month: ::std::option::Option<i32>,
            start_date_year: ::std::option::Option<i32>,
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
        impl<'a> SearchRequestBuilder<'a> {
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
            #[doc = "The maximum number of accounts to return. If the page size is unset, page size will default to 1000. Maximum page_size is 10000. Optional."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "The `next_page_token` value returned from a previous request to SearchDetailedLeadReports that indicates where listing should continue. Optional."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "A query string for searching for account reports. Caller must provide a customer id of their MCC account with an associated Gaia Mint that allows read permission on their linked accounts. Search expressions are case insensitive. Example query: | Query | Description | |-------------------------|-----------------------------------------------| | manager_customer_id:123 | Get Detailed Lead Report for Manager with id | | | 123. | Required."]
            pub fn query(mut self, value: impl Into<String>) -> Self {
                self.query = Some(value.into());
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
            #[doc = "\nExecute the request and yield each item in the `detailedLeadReports` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
            pub fn stream_detailed_lead_reports<T>(
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
                self.stream_detailed_lead_reports_with_fields(fields)
            }
            #[doc = "\nExecute the request and yield each item in the `detailedLeadReports` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
            pub fn stream_detailed_lead_reports_with_default_fields(
                self,
            ) -> impl ::futures::Stream<
                Item = Result<
                    crate::schemas::GoogleAdsHomeservicesLocalservicesV1DetailedLeadReport,
                    crate::Error,
                >,
            > + 'a {
                self.stream_detailed_lead_reports_with_fields(None::<String>)
            }
            #[doc = "\nExecute the request and yield each item in the `detailedLeadReports` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
            pub fn stream_detailed_lead_reports_with_all_fields(
                self,
            ) -> impl ::futures::Stream<
                Item = Result<
                    crate::schemas::GoogleAdsHomeservicesLocalservicesV1DetailedLeadReport,
                    crate::Error,
                >,
            > + 'a {
                self.stream_detailed_lead_reports_with_fields(Some("*"))
            }
            #[doc = "\nExecute the request and yield each item in the `detailedLeadReports` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
            pub fn stream_detailed_lead_reports_with_fields<T, F>(
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
                    #[serde(rename = "detailedLeadReports")]
                    pub items: Vec<T>,
                }
                impl<T> crate::GetNextPageToken for Page<T> {
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
                    let mut selector = concat!("nextPageToken,", "detailedLeadReports").to_owned();
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
            pub fn stream<T>(self) -> impl ::futures::Stream<Item = Result<T, crate::Error>> + 'a
            where
                T: crate::GetNextPageToken
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
            #[doc = r" Requests the default set of fields from the server."]            pub fn stream_with_default_fields (self) -> impl :: futures :: Stream < Item = Result < crate :: schemas :: GoogleAdsHomeservicesLocalservicesV1SearchDetailedLeadReportsResponse , crate :: Error >> + 'a{
                self.stream_with_fields(None::<&str>)
            }
            #[doc = r" Execute the request and yield the returned value. If the response contains a"]
            #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
            #[doc = r" repeated until no page token is returned."]
            #[doc = r""]
            #[doc = r" Requests all fields from the server."]            pub fn stream_with_all_fields (self) -> impl :: futures :: Stream < Item = Result < crate :: schemas :: GoogleAdsHomeservicesLocalservicesV1SearchDetailedLeadReportsResponse , crate :: Error >> + 'a{
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
                T: crate::GetNextPageToken + ::serde::de::DeserializeOwned + 'a,
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
            #[doc = r" the response resource."]            pub async fn execute_with_default_fields (self) -> Result < crate :: schemas :: GoogleAdsHomeservicesLocalservicesV1SearchDetailedLeadReportsResponse , crate :: Error >{
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]            pub async fn execute_with_all_fields (self) -> Result < crate :: schemas :: GoogleAdsHomeservicesLocalservicesV1SearchDetailedLeadReportsResponse , crate :: Error >{
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
                let mut output = "https://localservices.googleapis.com/".to_owned();
                output.push_str("v1/detailedLeadReports:search");
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
                req = req.query(&[("pageSize", &self.page_size)]);
                req = req.query(&[("pageToken", &self.page_token)]);
                req = req.query(&[("query", &self.query)]);
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
        #[async_trait::async_trait]
        impl<'a> crate::stream::StreamableMethod for SearchRequestBuilder<'a> {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            async fn execute<T>(&mut self) -> Result<T, crate::Error>
            where
                T: crate::GetNextPageToken + ::serde::de::DeserializeOwned,
            {
                self._execute().await
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
        /// Update the current page token of the request.
        fn set_page_token(&mut self, value: String);

        /// Execute the request.
        async fn execute<T>(&mut self) -> Result<T, crate::Error>
        where
            T: GetNextPageToken + ::serde::de::DeserializeOwned;
    }

    /// Return a [`Stream`](::futures::Stream) over all pages of the given API
    /// method.
    pub fn page_stream<M, T>(method: M) -> impl ::futures::Stream<Item = Result<T, crate::Error>>
    where
        M: StreamableMethod,
        T: GetNextPageToken + ::serde::de::DeserializeOwned,
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
        T: GetNextPageToken + ::serde::de::DeserializeOwned + IntoPageItems,
    {
        use ::futures::StreamExt;
        use ::futures::TryStreamExt;

        page_stream::<M, T>(method)
            .map_ok(|page| ::futures::stream::iter(page.into_page_items()).map(Ok))
            .try_flatten()
    }
}
