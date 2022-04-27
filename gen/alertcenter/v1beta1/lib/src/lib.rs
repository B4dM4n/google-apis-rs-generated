#![doc = "# Resources and Methods\n    * [alerts](resources/alerts/struct.AlertsActions.html)\n      * [*batchDelete*](resources/alerts/struct.BatchDeleteRequestBuilder.html), [*batchUndelete*](resources/alerts/struct.BatchUndeleteRequestBuilder.html), [*delete*](resources/alerts/struct.DeleteRequestBuilder.html), [*get*](resources/alerts/struct.GetRequestBuilder.html), [*getMetadata*](resources/alerts/struct.GetMetadataRequestBuilder.html), [*list*](resources/alerts/struct.ListRequestBuilder.html), [*undelete*](resources/alerts/struct.UndeleteRequestBuilder.html)\n      * [feedback](resources/alerts/feedback/struct.FeedbackActions.html)\n        * [*create*](resources/alerts/feedback/struct.CreateRequestBuilder.html), [*list*](resources/alerts/feedback/struct.ListRequestBuilder.html)\n    * [v_1beta_1](resources/v_1beta_1/struct.V1Beta1Actions.html)\n      * [*getSettings*](resources/v_1beta_1/struct.GetSettingsRequestBuilder.html), [*updateSettings*](resources/v_1beta_1/struct.UpdateSettingsRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "See and delete your domain's G Suite alerts, and send alert feedback\n\n`https://www.googleapis.com/auth/apps.alerts`"]
    pub const APPS_ALERTS: &str = "https://www.googleapis.com/auth/apps.alerts";
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
    pub struct AccountSuspensionDetails {
        #[doc = "The reason why this account is receiving an account suspension warning."]
        #[serde(
            rename = "abuseReason",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub abuse_reason:
            ::std::option::Option<crate::schemas::AccountSuspensionDetailsAbuseReason>,
        #[doc = "The name of the product being abused. This is restricted to only the following values: \"Gmail\" \"Google Workspace\" \"Payments\" \"Voice\" \"YouTube\" \"Other\""]
        #[serde(
            rename = "productName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub product_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AccountSuspensionDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AccountSuspensionDetails {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AccountSuspensionDetailsAbuseReason {
        #[doc = "Abuse reason is unspecified."]
        AccountSuspensionAbuseReasonUnspecified,
        #[doc = "This account is being suspended for fraud."]
        Fraud,
        #[doc = "This account is being suspended for number harvesting."]
        NumberHarvesting,
        #[doc = "This account is being suspended for payments fraud."]
        PaymentsFraud,
        #[doc = "This account is being suspended for phishing."]
        Phishing,
        #[doc = "This account is being suspended for spam."]
        Spam,
        #[doc = "This account is being suspended for a Terms of Service violation."]
        TosViolation,
        #[doc = "This account is being suspended for artificially boosting traffic to a website."]
        TrafficPumping,
        #[doc = "This account is being suspended for unwanted content."]
        UnwantedContent,
    }
    impl AccountSuspensionDetailsAbuseReason {
        pub fn as_str(self) -> &'static str {
            match self {
                AccountSuspensionDetailsAbuseReason::AccountSuspensionAbuseReasonUnspecified => {
                    "ACCOUNT_SUSPENSION_ABUSE_REASON_UNSPECIFIED"
                }
                AccountSuspensionDetailsAbuseReason::Fraud => "FRAUD",
                AccountSuspensionDetailsAbuseReason::NumberHarvesting => "NUMBER_HARVESTING",
                AccountSuspensionDetailsAbuseReason::PaymentsFraud => "PAYMENTS_FRAUD",
                AccountSuspensionDetailsAbuseReason::Phishing => "PHISHING",
                AccountSuspensionDetailsAbuseReason::Spam => "SPAM",
                AccountSuspensionDetailsAbuseReason::TosViolation => "TOS_VIOLATION",
                AccountSuspensionDetailsAbuseReason::TrafficPumping => "TRAFFIC_PUMPING",
                AccountSuspensionDetailsAbuseReason::UnwantedContent => "UNWANTED_CONTENT",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AccountSuspensionDetailsAbuseReason {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AccountSuspensionDetailsAbuseReason {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AccountSuspensionDetailsAbuseReason, ()> {
            Ok(match s {
                "ACCOUNT_SUSPENSION_ABUSE_REASON_UNSPECIFIED" => {
                    AccountSuspensionDetailsAbuseReason::AccountSuspensionAbuseReasonUnspecified
                }
                "FRAUD" => AccountSuspensionDetailsAbuseReason::Fraud,
                "NUMBER_HARVESTING" => AccountSuspensionDetailsAbuseReason::NumberHarvesting,
                "PAYMENTS_FRAUD" => AccountSuspensionDetailsAbuseReason::PaymentsFraud,
                "PHISHING" => AccountSuspensionDetailsAbuseReason::Phishing,
                "SPAM" => AccountSuspensionDetailsAbuseReason::Spam,
                "TOS_VIOLATION" => AccountSuspensionDetailsAbuseReason::TosViolation,
                "TRAFFIC_PUMPING" => AccountSuspensionDetailsAbuseReason::TrafficPumping,
                "UNWANTED_CONTENT" => AccountSuspensionDetailsAbuseReason::UnwantedContent,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AccountSuspensionDetailsAbuseReason {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AccountSuspensionDetailsAbuseReason {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AccountSuspensionDetailsAbuseReason {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCOUNT_SUSPENSION_ABUSE_REASON_UNSPECIFIED" => {
                    AccountSuspensionDetailsAbuseReason::AccountSuspensionAbuseReasonUnspecified
                }
                "FRAUD" => AccountSuspensionDetailsAbuseReason::Fraud,
                "NUMBER_HARVESTING" => AccountSuspensionDetailsAbuseReason::NumberHarvesting,
                "PAYMENTS_FRAUD" => AccountSuspensionDetailsAbuseReason::PaymentsFraud,
                "PHISHING" => AccountSuspensionDetailsAbuseReason::Phishing,
                "SPAM" => AccountSuspensionDetailsAbuseReason::Spam,
                "TOS_VIOLATION" => AccountSuspensionDetailsAbuseReason::TosViolation,
                "TRAFFIC_PUMPING" => AccountSuspensionDetailsAbuseReason::TrafficPumping,
                "UNWANTED_CONTENT" => AccountSuspensionDetailsAbuseReason::UnwantedContent,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AccountSuspensionDetailsAbuseReason {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AccountSuspensionDetailsAbuseReason {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AccountSuspensionWarning {
        #[doc = "The amount of time remaining to appeal an imminent suspension. After this window has elapsed, the account will be suspended. Only populated if the account suspension is in WARNING state."]
        #[serde(
            rename = "appealWindow",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub appeal_window: ::std::option::Option<String>,
        #[doc = "Account suspension warning state."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::AccountSuspensionWarningState>,
        #[doc = "Details about why an account is being suspended."]
        #[serde(
            rename = "suspensionDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suspension_details:
            ::std::option::Option<Vec<crate::schemas::AccountSuspensionDetails>>,
    }
    impl ::google_field_selector::FieldSelector for AccountSuspensionWarning {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AccountSuspensionWarning {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AccountSuspensionWarningState {
        #[doc = "State is unspecified."]
        AccountSuspensionWarningStateUnspecified,
        #[doc = "Customer is being notified that their suspension appeal was approved."]
        AppealApproved,
        #[doc = "Customer has submitted their appeal, which is pending review."]
        AppealSubmitted,
        #[doc = "Customer is being notified that their account has been suspended."]
        Suspended,
        #[doc = "Customer is receiving a warning about imminent suspension."]
        Warning,
    }
    impl AccountSuspensionWarningState {
        pub fn as_str(self) -> &'static str {
            match self {
                AccountSuspensionWarningState::AccountSuspensionWarningStateUnspecified => {
                    "ACCOUNT_SUSPENSION_WARNING_STATE_UNSPECIFIED"
                }
                AccountSuspensionWarningState::AppealApproved => "APPEAL_APPROVED",
                AccountSuspensionWarningState::AppealSubmitted => "APPEAL_SUBMITTED",
                AccountSuspensionWarningState::Suspended => "SUSPENDED",
                AccountSuspensionWarningState::Warning => "WARNING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AccountSuspensionWarningState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AccountSuspensionWarningState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AccountSuspensionWarningState, ()> {
            Ok(match s {
                "ACCOUNT_SUSPENSION_WARNING_STATE_UNSPECIFIED" => {
                    AccountSuspensionWarningState::AccountSuspensionWarningStateUnspecified
                }
                "APPEAL_APPROVED" => AccountSuspensionWarningState::AppealApproved,
                "APPEAL_SUBMITTED" => AccountSuspensionWarningState::AppealSubmitted,
                "SUSPENDED" => AccountSuspensionWarningState::Suspended,
                "WARNING" => AccountSuspensionWarningState::Warning,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AccountSuspensionWarningState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AccountSuspensionWarningState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AccountSuspensionWarningState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACCOUNT_SUSPENSION_WARNING_STATE_UNSPECIFIED" => {
                    AccountSuspensionWarningState::AccountSuspensionWarningStateUnspecified
                }
                "APPEAL_APPROVED" => AccountSuspensionWarningState::AppealApproved,
                "APPEAL_SUBMITTED" => AccountSuspensionWarningState::AppealSubmitted,
                "SUSPENDED" => AccountSuspensionWarningState::Suspended,
                "WARNING" => AccountSuspensionWarningState::Warning,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AccountSuspensionWarningState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AccountSuspensionWarningState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AccountWarning {
        #[doc = "Required. The email of the user that this event belongs to."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "Optional. Details of the login action associated with the warning event. This is only available for: * Suspicious login * Suspicious login (less secure app) * Suspicious programmatic login * User suspended (suspicious activity)"]
        #[serde(
            rename = "loginDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub login_details: ::std::option::Option<crate::schemas::LoginDetails>,
    }
    impl ::google_field_selector::FieldSelector for AccountWarning {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AccountWarning {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct ActionInfo {}
    impl ::google_field_selector::FieldSelector for ActionInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ActionInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ActivityRule {
        #[doc = "List of action names associated with the rule threshold."]
        #[serde(
            rename = "actionNames",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub action_names: ::std::option::Option<Vec<String>>,
        #[doc = "Rule create timestamp."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Description of the rule."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Alert display name."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Rule name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Query that is used to get the data from the associated source."]
        #[serde(
            rename = "query",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query: ::std::option::Option<String>,
        #[doc = "List of alert IDs superseded by this alert. It is used to indicate that this alert is essentially extension of superseded alerts and we found the relationship after creating these alerts."]
        #[serde(
            rename = "supersededAlerts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub superseded_alerts: ::std::option::Option<Vec<String>>,
        #[doc = "Alert ID superseding this alert. It is used to indicate that superseding alert is essentially extension of this alert and we found the relationship after creating both alerts."]
        #[serde(
            rename = "supersedingAlert",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub superseding_alert: ::std::option::Option<String>,
        #[doc = "Alert threshold is for example “COUNT > 5”."]
        #[serde(
            rename = "threshold",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub threshold: ::std::option::Option<String>,
        #[doc = "The trigger sources for this rule. * GMAIL_EVENTS * DEVICE_EVENTS * USER_EVENTS"]
        #[serde(
            rename = "triggerSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub trigger_source: ::std::option::Option<String>,
        #[doc = "The timestamp of the last update to the rule."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
        #[doc = "Rule window size. Possible values are 1 hour or 24 hours."]
        #[serde(
            rename = "windowSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub window_size: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ActivityRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ActivityRule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Alert {
        #[doc = "Output only. The unique identifier for the alert."]
        #[serde(
            rename = "alertId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alert_id: ::std::option::Option<String>,
        #[doc = "Output only. The time this alert was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Output only. The unique identifier of the Google account of the customer."]
        #[serde(
            rename = "customerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub customer_id: ::std::option::Option<String>,
        #[doc = "Optional. The data associated with this alert, for example google.apps.alertcenter.type.DeviceCompromised."]
        #[serde(
            rename = "data",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Output only. `True` if this alert is marked for deletion."]
        #[serde(
            rename = "deleted",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub deleted: ::std::option::Option<bool>,
        #[doc = "Optional. The time the event that caused this alert ceased being active. If provided, the end time must not be earlier than the start time. If not provided, it indicates an ongoing alert."]
        #[serde(
            rename = "endTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub end_time: ::std::option::Option<String>,
        #[doc = "Optional. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of an alert from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform alert updates in order to avoid race conditions: An `etag` is returned in the response which contains alerts, and systems are expected to put that etag in the request to update alert to ensure that their change will be applied to the same version of the alert. If no `etag` is provided in the call to update alert, then the existing alert is overwritten blindly."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "Output only. The metadata associated with this alert."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::AlertMetadata>,
        #[doc = "Required. The type of the alert. This is output only after alert is created. For a list of available alert types see [Google Workspace Alert types](https://developers.google.com/admin-sdk/alertcenter/reference/alert-types)."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "Output only. An optional [Security Investigation Tool](https://support.google.com/a/answer/7575955) query for this alert."]
        #[serde(
            rename = "securityInvestigationToolLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub security_investigation_tool_link: ::std::option::Option<String>,
        #[doc = "Required. A unique identifier for the system that reported the alert. This is output only after alert is created. Supported sources are any of the following: * Google Operations * Mobile device management * Gmail phishing * Data Loss Prevention * Domain wide takeout * State sponsored attack * Google identity * Apps outage"]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<String>,
        #[doc = "Required. The time the event that caused this alert was started or detected."]
        #[serde(
            rename = "startTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_time: ::std::option::Option<String>,
        #[doc = "Output only. The time this alert was last updated."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
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
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AlertFeedback {
        #[doc = "Output only. The alert identifier."]
        #[serde(
            rename = "alertId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alert_id: ::std::option::Option<String>,
        #[doc = "Output only. The time this feedback was created."]
        #[serde(
            rename = "createTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub create_time: ::std::option::Option<String>,
        #[doc = "Output only. The unique identifier of the Google account of the customer."]
        #[serde(
            rename = "customerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub customer_id: ::std::option::Option<String>,
        #[doc = "Output only. The email of the user that provided the feedback."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "Output only. The unique identifier for the feedback."]
        #[serde(
            rename = "feedbackId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub feedback_id: ::std::option::Option<String>,
        #[doc = "Required. The type of the feedback."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::AlertFeedbackType>,
    }
    impl ::google_field_selector::FieldSelector for AlertFeedback {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AlertFeedback {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AlertFeedbackType {
        #[doc = "The feedback type is not specified."]
        AlertFeedbackTypeUnspecified,
        #[doc = "The alert report is not useful."]
        NotUseful,
        #[doc = "The alert report is somewhat useful."]
        SomewhatUseful,
        #[doc = "The alert report is very useful."]
        VeryUseful,
    }
    impl AlertFeedbackType {
        pub fn as_str(self) -> &'static str {
            match self {
                AlertFeedbackType::AlertFeedbackTypeUnspecified => {
                    "ALERT_FEEDBACK_TYPE_UNSPECIFIED"
                }
                AlertFeedbackType::NotUseful => "NOT_USEFUL",
                AlertFeedbackType::SomewhatUseful => "SOMEWHAT_USEFUL",
                AlertFeedbackType::VeryUseful => "VERY_USEFUL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AlertFeedbackType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AlertFeedbackType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AlertFeedbackType, ()> {
            Ok(match s {
                "ALERT_FEEDBACK_TYPE_UNSPECIFIED" => {
                    AlertFeedbackType::AlertFeedbackTypeUnspecified
                }
                "NOT_USEFUL" => AlertFeedbackType::NotUseful,
                "SOMEWHAT_USEFUL" => AlertFeedbackType::SomewhatUseful,
                "VERY_USEFUL" => AlertFeedbackType::VeryUseful,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AlertFeedbackType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AlertFeedbackType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AlertFeedbackType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALERT_FEEDBACK_TYPE_UNSPECIFIED" => {
                    AlertFeedbackType::AlertFeedbackTypeUnspecified
                }
                "NOT_USEFUL" => AlertFeedbackType::NotUseful,
                "SOMEWHAT_USEFUL" => AlertFeedbackType::SomewhatUseful,
                "VERY_USEFUL" => AlertFeedbackType::VeryUseful,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AlertFeedbackType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AlertFeedbackType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AlertMetadata {
        #[doc = "Output only. The alert identifier."]
        #[serde(
            rename = "alertId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alert_id: ::std::option::Option<String>,
        #[doc = "The email address of the user assigned to the alert."]
        #[serde(
            rename = "assignee",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub assignee: ::std::option::Option<String>,
        #[doc = "Output only. The unique identifier of the Google account of the customer."]
        #[serde(
            rename = "customerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub customer_id: ::std::option::Option<String>,
        #[doc = "Optional. `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of an alert metadata from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform metadata updates in order to avoid race conditions: An `etag` is returned in the response which contains alert metadata, and systems are expected to put that etag in the request to update alert metadata to ensure that their change will be applied to the same version of the alert metadata. If no `etag` is provided in the call to update alert metadata, then the existing alert metadata is overwritten blindly."]
        #[serde(
            rename = "etag",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub etag: ::std::option::Option<String>,
        #[doc = "The severity value of the alert. Alert Center will set this field at alert creation time, default's to an empty string when it could not be determined. The supported values for update actions on this field are the following: * HIGH * MEDIUM * LOW"]
        #[serde(
            rename = "severity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub severity: ::std::option::Option<String>,
        #[doc = "The current status of the alert. The supported values are the following: * NOT_STARTED * IN_PROGRESS * CLOSED"]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<String>,
        #[doc = "Output only. The time this metadata was last updated."]
        #[serde(
            rename = "updateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AlertMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AlertMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AppMakerSqlSetupNotification {
        #[doc = "List of applications with requests for default SQL set up."]
        #[serde(
            rename = "requestInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request_info: ::std::option::Option<Vec<crate::schemas::RequestInfo>>,
    }
    impl ::google_field_selector::FieldSelector for AppMakerSqlSetupNotification {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AppMakerSqlSetupNotification {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AppSettingsChanged {
        #[doc = "Any other associated alert details, for example, AlertConfiguration."]
        #[serde(
            rename = "alertDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alert_details: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Rule name"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for AppSettingsChanged {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AppSettingsChanged {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AppsOutage {
        #[doc = "Link to the outage event in Google Workspace Status Dashboard"]
        #[serde(
            rename = "dashboardUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dashboard_uri: ::std::option::Option<String>,
        #[doc = "Incident tracking ID."]
        #[serde(
            rename = "incidentTrackingId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub incident_tracking_id: ::std::option::Option<String>,
        #[doc = "Indicates new alert details under which the outage is communicated. Only populated when Status is MERGED."]
        #[serde(
            rename = "mergeInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub merge_info: ::std::option::Option<crate::schemas::MergeInfo>,
        #[doc = "Timestamp by which the next update is expected to arrive."]
        #[serde(
            rename = "nextUpdateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_update_time: ::std::option::Option<String>,
        #[doc = "List of products impacted by the outage."]
        #[serde(
            rename = "products",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub products: ::std::option::Option<Vec<String>>,
        #[doc = "Timestamp when the outage is expected to be resolved, or has confirmed resolution. Provided only when known."]
        #[serde(
            rename = "resolutionTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resolution_time: ::std::option::Option<String>,
        #[doc = "Current outage status."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::AppsOutageStatus>,
    }
    impl ::google_field_selector::FieldSelector for AppsOutage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AppsOutage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AppsOutageStatus {
        #[doc = "The incident has lower impact than initially anticipated."]
        Downgraded,
        #[doc = "Further assessment indicated no customer impact."]
        FalsePositive,
        #[doc = "The incident was merged into a parent."]
        Merged,
        #[doc = "The incident has just been reported."]
        New,
        #[doc = "The incident is ongoing."]
        Ongoing,
        #[doc = "The incident has been partially resolved."]
        PartiallyResolved,
        #[doc = "The incident has been resolved."]
        Resolved,
        #[doc = "Status is unspecified."]
        StatusUnspecified,
    }
    impl AppsOutageStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                AppsOutageStatus::Downgraded => "DOWNGRADED",
                AppsOutageStatus::FalsePositive => "FALSE_POSITIVE",
                AppsOutageStatus::Merged => "MERGED",
                AppsOutageStatus::New => "NEW",
                AppsOutageStatus::Ongoing => "ONGOING",
                AppsOutageStatus::PartiallyResolved => "PARTIALLY_RESOLVED",
                AppsOutageStatus::Resolved => "RESOLVED",
                AppsOutageStatus::StatusUnspecified => "STATUS_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for AppsOutageStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for AppsOutageStatus {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<AppsOutageStatus, ()> {
            Ok(match s {
                "DOWNGRADED" => AppsOutageStatus::Downgraded,
                "FALSE_POSITIVE" => AppsOutageStatus::FalsePositive,
                "MERGED" => AppsOutageStatus::Merged,
                "NEW" => AppsOutageStatus::New,
                "ONGOING" => AppsOutageStatus::Ongoing,
                "PARTIALLY_RESOLVED" => AppsOutageStatus::PartiallyResolved,
                "RESOLVED" => AppsOutageStatus::Resolved,
                "STATUS_UNSPECIFIED" => AppsOutageStatus::StatusUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for AppsOutageStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AppsOutageStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AppsOutageStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DOWNGRADED" => AppsOutageStatus::Downgraded,
                "FALSE_POSITIVE" => AppsOutageStatus::FalsePositive,
                "MERGED" => AppsOutageStatus::Merged,
                "NEW" => AppsOutageStatus::New,
                "ONGOING" => AppsOutageStatus::Ongoing,
                "PARTIALLY_RESOLVED" => AppsOutageStatus::PartiallyResolved,
                "RESOLVED" => AppsOutageStatus::Resolved,
                "STATUS_UNSPECIFIED" => AppsOutageStatus::StatusUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for AppsOutageStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for AppsOutageStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Attachment {
        #[doc = "A CSV file attachment."]
        #[serde(
            rename = "csv",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub csv: ::std::option::Option<crate::schemas::Csv>,
    }
    impl ::google_field_selector::FieldSelector for Attachment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Attachment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct BadWhitelist {
        #[doc = "The domain ID."]
        #[serde(
            rename = "domainId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain_id: ::std::option::Option<crate::schemas::DomainId>,
        #[doc = "The entity whose actions triggered a Gmail phishing alert."]
        #[serde(
            rename = "maliciousEntity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub malicious_entity: ::std::option::Option<crate::schemas::MaliciousEntity>,
        #[doc = "The list of messages contained by this alert."]
        #[serde(
            rename = "messages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub messages: ::std::option::Option<Vec<crate::schemas::GmailMessageInfo>>,
        #[doc = "The source IP address of the malicious email, for example, `127.0.0.1`."]
        #[serde(
            rename = "sourceIp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_ip: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BadWhitelist {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BadWhitelist {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct BatchDeleteAlertsRequest {
        #[doc = "Required. list of alert IDs."]
        #[serde(
            rename = "alertId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alert_id: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The unique identifier of the Google Workspace organization account of the customer the alerts are associated with."]
        #[serde(
            rename = "customerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub customer_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BatchDeleteAlertsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchDeleteAlertsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct BatchDeleteAlertsResponse {
        #[doc = "The status details for each failed alert_id."]
        #[serde(
            rename = "failedAlertStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub failed_alert_status:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::Status>>,
        #[doc = "The successful list of alert IDs."]
        #[serde(
            rename = "successAlertIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub success_alert_ids: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for BatchDeleteAlertsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchDeleteAlertsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct BatchUndeleteAlertsRequest {
        #[doc = "Required. list of alert IDs."]
        #[serde(
            rename = "alertId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alert_id: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. The unique identifier of the Google Workspace organization account of the customer the alerts are associated with."]
        #[serde(
            rename = "customerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub customer_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for BatchUndeleteAlertsRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchUndeleteAlertsRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct BatchUndeleteAlertsResponse {
        #[doc = "The status details for each failed alert_id."]
        #[serde(
            rename = "failedAlertStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub failed_alert_status:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::Status>>,
        #[doc = "The successful list of alert IDs."]
        #[serde(
            rename = "successAlertIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub success_alert_ids: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for BatchUndeleteAlertsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for BatchUndeleteAlertsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CloudPubsubTopic {
        #[doc = "Optional. The format of the payload that would be sent. If not specified the format will be JSON."]
        #[serde(
            rename = "payloadFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub payload_format: ::std::option::Option<crate::schemas::CloudPubsubTopicPayloadFormat>,
        #[doc = "The `name` field of a Cloud Pubsub [Topic] (https://cloud.google.com/pubsub/docs/reference/rest/v1/projects.topics#Topic)."]
        #[serde(
            rename = "topicName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub topic_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CloudPubsubTopic {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CloudPubsubTopic {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CloudPubsubTopicPayloadFormat {
        #[doc = "Use JSON."]
        Json,
        #[doc = "Payload format is not specified (will use JSON as default)."]
        PayloadFormatUnspecified,
    }
    impl CloudPubsubTopicPayloadFormat {
        pub fn as_str(self) -> &'static str {
            match self {
                CloudPubsubTopicPayloadFormat::Json => "JSON",
                CloudPubsubTopicPayloadFormat::PayloadFormatUnspecified => {
                    "PAYLOAD_FORMAT_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for CloudPubsubTopicPayloadFormat {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CloudPubsubTopicPayloadFormat {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CloudPubsubTopicPayloadFormat, ()> {
            Ok(match s {
                "JSON" => CloudPubsubTopicPayloadFormat::Json,
                "PAYLOAD_FORMAT_UNSPECIFIED" => {
                    CloudPubsubTopicPayloadFormat::PayloadFormatUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CloudPubsubTopicPayloadFormat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CloudPubsubTopicPayloadFormat {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CloudPubsubTopicPayloadFormat {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "JSON" => CloudPubsubTopicPayloadFormat::Json,
                "PAYLOAD_FORMAT_UNSPECIFIED" => {
                    CloudPubsubTopicPayloadFormat::PayloadFormatUnspecified
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
    impl ::google_field_selector::FieldSelector for CloudPubsubTopicPayloadFormat {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CloudPubsubTopicPayloadFormat {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Csv {
        #[doc = "The list of data rows in a CSV file, as string arrays rather than as a single comma-separated string."]
        #[serde(
            rename = "dataRows",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_rows: ::std::option::Option<Vec<crate::schemas::CsvRow>>,
        #[doc = "The list of headers for data columns in a CSV file."]
        #[serde(
            rename = "headers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub headers: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for Csv {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Csv {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CsvRow {
        #[doc = "The data entries in a CSV file row, as a string array rather than a single comma-separated string."]
        #[serde(
            rename = "entries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entries: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for CsvRow {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CsvRow {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeviceCompromised {
        #[doc = "The email of the user this alert was created for."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "Required. The list of security events."]
        #[serde(
            rename = "events",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub events: ::std::option::Option<Vec<crate::schemas::DeviceCompromisedSecurityDetail>>,
    }
    impl ::google_field_selector::FieldSelector for DeviceCompromised {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeviceCompromised {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeviceCompromisedSecurityDetail {
        #[doc = "The device compromised state. Possible values are \"`Compromised`\" or \"`Not Compromised`\"."]
        #[serde(
            rename = "deviceCompromisedState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_compromised_state: ::std::option::Option<String>,
        #[doc = "Required. The device ID."]
        #[serde(
            rename = "deviceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_id: ::std::option::Option<String>,
        #[doc = "The model of the device."]
        #[serde(
            rename = "deviceModel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_model: ::std::option::Option<String>,
        #[doc = "The type of the device."]
        #[serde(
            rename = "deviceType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_type: ::std::option::Option<String>,
        #[doc = "Required for iOS, empty for others."]
        #[serde(
            rename = "iosVendorId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_vendor_id: ::std::option::Option<String>,
        #[doc = "The device resource ID."]
        #[serde(
            rename = "resourceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_id: ::std::option::Option<String>,
        #[doc = "The serial number of the device."]
        #[serde(
            rename = "serialNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub serial_number: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DeviceCompromisedSecurityDetail {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DeviceCompromisedSecurityDetail {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DlpRuleViolation {
        #[doc = "Details about the violated DLP rule. Admins can use the predefined detectors provided by Google Cloud DLP https://cloud.google.com/dlp/ when setting up a DLP rule. Matched Cloud DLP detectors in this violation if any will be captured in the MatchInfo.predefined_detector."]
        #[serde(
            rename = "ruleViolationInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rule_violation_info: ::std::option::Option<crate::schemas::RuleViolationInfo>,
    }
    impl ::google_field_selector::FieldSelector for DlpRuleViolation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DlpRuleViolation {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DomainId {
        #[doc = "The primary domain for the customer."]
        #[serde(
            rename = "customerPrimaryDomain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub customer_primary_domain: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DomainId {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DomainId {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DomainWideTakeoutInitiated {
        #[doc = "The email of the admin who initiated the takeout."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "The takeout request ID."]
        #[serde(
            rename = "takeoutRequestId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub takeout_request_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DomainWideTakeoutInitiated {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DomainWideTakeoutInitiated {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
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
    pub struct Empty {}
    impl ::google_field_selector::FieldSelector for Empty {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Empty {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GmailMessageInfo {
        #[doc = "The `SHA256` hash of email's attachment and all MIME parts."]
        #[serde(
            rename = "attachmentsSha256Hash",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attachments_sha_256_hash: ::std::option::Option<Vec<String>>,
        #[doc = "The date of the event related to this email."]
        #[serde(
            rename = "date",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date: ::std::option::Option<String>,
        #[doc = "The hash of the message body text."]
        #[serde(
            rename = "md5HashMessageBody",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub md_5_hash_message_body: ::std::option::Option<String>,
        #[doc = "The MD5 Hash of email's subject (only available for reported emails)."]
        #[serde(
            rename = "md5HashSubject",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub md_5_hash_subject: ::std::option::Option<String>,
        #[doc = "The snippet of the message body text (only available for reported emails)."]
        #[serde(
            rename = "messageBodySnippet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message_body_snippet: ::std::option::Option<String>,
        #[doc = "The message ID."]
        #[serde(
            rename = "messageId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub message_id: ::std::option::Option<String>,
        #[doc = "The recipient of this email."]
        #[serde(
            rename = "recipient",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recipient: ::std::option::Option<String>,
        #[doc = "The email subject text (only available for reported emails)."]
        #[serde(
            rename = "subjectText",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub subject_text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GmailMessageInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GmailMessageInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleOperations {
        #[doc = "The list of emails which correspond to the users directly affected by the incident."]
        #[serde(
            rename = "affectedUserEmails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub affected_user_emails: ::std::option::Option<Vec<String>>,
        #[doc = "Optional. Application-specific data for an incident, provided when the Google Workspace application which reported the incident cannot be completely restored to a valid state."]
        #[serde(
            rename = "attachmentData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attachment_data: ::std::option::Option<crate::schemas::Attachment>,
        #[doc = "A detailed, freeform incident description."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Customer domain for email template personalization."]
        #[serde(
            rename = "domain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain: ::std::option::Option<String>,
        #[doc = "A header to display above the incident message. Typically used to attach a localized notice on the timeline for followup comms translations."]
        #[serde(
            rename = "header",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub header: ::std::option::Option<String>,
        #[doc = "A one-line incident description."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleOperations {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleOperations {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ListAlertFeedbackResponse {
        #[doc = "The list of alert feedback. Feedback entries for each alert are ordered by creation time descending."]
        #[serde(
            rename = "feedback",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub feedback: ::std::option::Option<Vec<crate::schemas::AlertFeedback>>,
    }
    impl ::google_field_selector::FieldSelector for ListAlertFeedbackResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListAlertFeedbackResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListAlertsResponse {
        #[doc = "The list of alerts."]
        #[serde(
            rename = "alerts",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alerts: ::std::option::Option<Vec<crate::schemas::Alert>>,
        #[doc = "The token for the next page. If not empty, indicates that there may be more alerts that match the listing request; this value can be used in a subsequent ListAlertsRequest to get alerts continuing from last result of the current list call."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
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
    pub struct LoginDetails {
        #[doc = "Optional. The human-readable IP address (for example, `11.22.33.44`) that is associated with the warning event."]
        #[serde(
            rename = "ipAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ip_address: ::std::option::Option<String>,
        #[doc = "Optional. The successful login time that is associated with the warning event. This isn't present for blocked login attempts."]
        #[serde(
            rename = "loginTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub login_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for LoginDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for LoginDetails {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MailPhishing {
        #[doc = "The domain ID."]
        #[serde(
            rename = "domainId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain_id: ::std::option::Option<crate::schemas::DomainId>,
        #[doc = "If `true`, the email originated from within the organization."]
        #[serde(
            rename = "isInternal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_internal: ::std::option::Option<bool>,
        #[doc = "The entity whose actions triggered a Gmail phishing alert."]
        #[serde(
            rename = "maliciousEntity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub malicious_entity: ::std::option::Option<crate::schemas::MaliciousEntity>,
        #[doc = "The list of messages contained by this alert."]
        #[serde(
            rename = "messages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub messages: ::std::option::Option<Vec<crate::schemas::GmailMessageInfo>>,
        #[doc = "System actions on the messages."]
        #[serde(
            rename = "systemActionType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub system_action_type: ::std::option::Option<crate::schemas::MailPhishingSystemActionType>,
    }
    impl ::google_field_selector::FieldSelector for MailPhishing {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MailPhishing {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum MailPhishingSystemActionType {
        #[doc = "No operation."]
        NoOperation,
        #[doc = "Messages were removed from the inbox."]
        RemovedFromInbox,
        #[doc = "System action is unspecified."]
        SystemActionTypeUnspecified,
    }
    impl MailPhishingSystemActionType {
        pub fn as_str(self) -> &'static str {
            match self {
                MailPhishingSystemActionType::NoOperation => "NO_OPERATION",
                MailPhishingSystemActionType::RemovedFromInbox => "REMOVED_FROM_INBOX",
                MailPhishingSystemActionType::SystemActionTypeUnspecified => {
                    "SYSTEM_ACTION_TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for MailPhishingSystemActionType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for MailPhishingSystemActionType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<MailPhishingSystemActionType, ()> {
            Ok(match s {
                "NO_OPERATION" => MailPhishingSystemActionType::NoOperation,
                "REMOVED_FROM_INBOX" => MailPhishingSystemActionType::RemovedFromInbox,
                "SYSTEM_ACTION_TYPE_UNSPECIFIED" => {
                    MailPhishingSystemActionType::SystemActionTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for MailPhishingSystemActionType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for MailPhishingSystemActionType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MailPhishingSystemActionType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "NO_OPERATION" => MailPhishingSystemActionType::NoOperation,
                "REMOVED_FROM_INBOX" => MailPhishingSystemActionType::RemovedFromInbox,
                "SYSTEM_ACTION_TYPE_UNSPECIFIED" => {
                    MailPhishingSystemActionType::SystemActionTypeUnspecified
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
    impl ::google_field_selector::FieldSelector for MailPhishingSystemActionType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MailPhishingSystemActionType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MaliciousEntity {
        #[doc = "The header from display name."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The actor who triggered a gmail phishing alert."]
        #[serde(
            rename = "entity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub entity: ::std::option::Option<crate::schemas::User>,
        #[doc = "The sender email address."]
        #[serde(
            rename = "fromHeader",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub from_header: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for MaliciousEntity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MaliciousEntity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MandatoryServiceAnnouncement {
        #[doc = "Detailed, freeform text describing the announcement"]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "One line summary of the announcement"]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for MandatoryServiceAnnouncement {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MandatoryServiceAnnouncement {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MatchInfo {
        #[doc = "For matched detector predefined by Google."]
        #[serde(
            rename = "predefinedDetector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub predefined_detector: ::std::option::Option<crate::schemas::PredefinedDetectorInfo>,
        #[doc = "For matched detector defined by administrators."]
        #[serde(
            rename = "userDefinedDetector",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub user_defined_detector: ::std::option::Option<crate::schemas::UserDefinedDetectorInfo>,
    }
    impl ::google_field_selector::FieldSelector for MatchInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MatchInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MergeInfo {
        #[doc = "Optional. New alert ID. Reference the [google.apps.alertcenter.Alert] with this ID for the current state."]
        #[serde(
            rename = "newAlertId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_alert_id: ::std::option::Option<String>,
        #[doc = "The new tracking ID from the parent incident."]
        #[serde(
            rename = "newIncidentTrackingId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_incident_tracking_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for MergeInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MergeInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Notification {
        #[doc = "A Google Cloud Pub/sub topic destination."]
        #[serde(
            rename = "cloudPubsubTopic",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cloud_pubsub_topic: ::std::option::Option<crate::schemas::CloudPubsubTopic>,
    }
    impl ::google_field_selector::FieldSelector for Notification {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Notification {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PhishingSpike {
        #[doc = "The domain ID."]
        #[serde(
            rename = "domainId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub domain_id: ::std::option::Option<crate::schemas::DomainId>,
        #[doc = "If `true`, the email originated from within the organization."]
        #[serde(
            rename = "isInternal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_internal: ::std::option::Option<bool>,
        #[doc = "The entity whose actions triggered a Gmail phishing alert."]
        #[serde(
            rename = "maliciousEntity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub malicious_entity: ::std::option::Option<crate::schemas::MaliciousEntity>,
        #[doc = "The list of messages contained by this alert."]
        #[serde(
            rename = "messages",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub messages: ::std::option::Option<Vec<crate::schemas::GmailMessageInfo>>,
    }
    impl ::google_field_selector::FieldSelector for PhishingSpike {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PhishingSpike {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PredefinedDetectorInfo {
        #[doc = "Name that uniquely identifies the detector."]
        #[serde(
            rename = "detectorName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detector_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PredefinedDetectorInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PredefinedDetectorInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ReportingRule {
        #[doc = "Any other associated alert details, for example, AlertConfiguration."]
        #[serde(
            rename = "alertDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alert_details: ::std::option::Option<::google_api_bytes::Bytes>,
        #[doc = "Rule name"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Alert Rule query Sample Query query { condition { filter { expected_application_id: 777491262838 expected_event_name: \"indexable_content_change\" filter_op: IN } } conjunction_operator: OR }"]
        #[serde(
            rename = "query",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub query: ::std::option::Option<::google_api_bytes::Bytes>,
    }
    impl ::google_field_selector::FieldSelector for ReportingRule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportingRule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RequestInfo {
        #[doc = "List of app developers who triggered notifications for above application."]
        #[serde(
            rename = "appDeveloperEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_developer_email: ::std::option::Option<Vec<String>>,
        #[doc = "Required. The application that requires the SQL setup."]
        #[serde(
            rename = "appKey",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_key: ::std::option::Option<String>,
        #[doc = "Required. Number of requests sent for this application to set up default SQL instance."]
        #[serde(
            rename = "numberOfRequests",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub number_of_requests: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for RequestInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RequestInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ResourceInfo {
        #[doc = "Drive file ID."]
        #[serde(
            rename = "documentId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub document_id: ::std::option::Option<String>,
        #[doc = "Title of the resource, for example email subject, or document title."]
        #[serde(
            rename = "resourceTitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ResourceInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResourceInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RuleInfo {
        #[doc = "User provided name of the rule."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Resource name that uniquely identifies the rule."]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RuleInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RuleInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RuleViolationInfo {
        #[doc = "Source of the data."]
        #[serde(
            rename = "dataSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_source: ::std::option::Option<crate::schemas::RuleViolationInfoDataSource>,
        #[doc = "List of matches that were found in the resource content."]
        #[serde(
            rename = "matchInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub match_info: ::std::option::Option<Vec<crate::schemas::MatchInfo>>,
        #[doc = "Resource recipients. For Drive, they are grantees that the Drive file was shared with at the time of rule triggering. Valid values include user emails, group emails, domains, or 'anyone' if the file was publicly accessible. If the file was private the recipients list will be empty. For Gmail, they are emails of the users or groups that the Gmail message was sent to."]
        #[serde(
            rename = "recipients",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub recipients: ::std::option::Option<Vec<String>>,
        #[doc = "Details of the resource which violated the rule."]
        #[serde(
            rename = "resourceInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_info: ::std::option::Option<crate::schemas::ResourceInfo>,
        #[doc = "Details of the violated rule."]
        #[serde(
            rename = "ruleInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rule_info: ::std::option::Option<crate::schemas::RuleInfo>,
        #[doc = "Actions suppressed due to other actions with higher priority."]
        #[serde(
            rename = "suppressedActionTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suppressed_action_types:
            ::std::option::Option<Vec<crate::schemas::RuleViolationInfoSuppressedActionTypesItems>>,
        #[doc = "Trigger of the rule."]
        #[serde(
            rename = "trigger",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub trigger: ::std::option::Option<crate::schemas::RuleViolationInfoTrigger>,
        #[doc = "Metadata related to the triggered actions."]
        #[serde(
            rename = "triggeredActionInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub triggered_action_info: ::std::option::Option<Vec<crate::schemas::ActionInfo>>,
        #[doc = "Actions applied as a consequence of the rule being triggered."]
        #[serde(
            rename = "triggeredActionTypes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub triggered_action_types:
            ::std::option::Option<Vec<crate::schemas::RuleViolationInfoTriggeredActionTypesItems>>,
        #[doc = "Email of the user who caused the violation. Value could be empty if not applicable, for example, a violation found by drive continuous scan."]
        #[serde(
            rename = "triggeringUserEmail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub triggering_user_email: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RuleViolationInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RuleViolationInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RuleViolationInfoDataSource {
        #[doc = "Data source is unspecified."]
        DataSourceUnspecified,
        #[doc = "Drive data source."]
        Drive,
    }
    impl RuleViolationInfoDataSource {
        pub fn as_str(self) -> &'static str {
            match self {
                RuleViolationInfoDataSource::DataSourceUnspecified => "DATA_SOURCE_UNSPECIFIED",
                RuleViolationInfoDataSource::Drive => "DRIVE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for RuleViolationInfoDataSource {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RuleViolationInfoDataSource {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<RuleViolationInfoDataSource, ()> {
            Ok(match s {
                "DATA_SOURCE_UNSPECIFIED" => RuleViolationInfoDataSource::DataSourceUnspecified,
                "DRIVE" => RuleViolationInfoDataSource::Drive,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for RuleViolationInfoDataSource {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RuleViolationInfoDataSource {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RuleViolationInfoDataSource {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DATA_SOURCE_UNSPECIFIED" => RuleViolationInfoDataSource::DataSourceUnspecified,
                "DRIVE" => RuleViolationInfoDataSource::Drive,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RuleViolationInfoDataSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RuleViolationInfoDataSource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RuleViolationInfoSuppressedActionTypesItems {
        #[doc = "Action type is unspecified."]
        ActionTypeUnspecified,
        #[doc = "Send alert."]
        Alert,
        #[doc = "Block sharing a file externally."]
        DriveBlockExternalSharing,
        #[doc = "Show a warning message when sharing a file externally."]
        DriveWarnOnExternalSharing,
        #[doc = "Activate Rule Action"]
        RuleActivate,
        #[doc = "Deactivate Rule Action"]
        RuleDeactivate,
    }
    impl RuleViolationInfoSuppressedActionTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                RuleViolationInfoSuppressedActionTypesItems::ActionTypeUnspecified => {
                    "ACTION_TYPE_UNSPECIFIED"
                }
                RuleViolationInfoSuppressedActionTypesItems::Alert => "ALERT",
                RuleViolationInfoSuppressedActionTypesItems::DriveBlockExternalSharing => {
                    "DRIVE_BLOCK_EXTERNAL_SHARING"
                }
                RuleViolationInfoSuppressedActionTypesItems::DriveWarnOnExternalSharing => {
                    "DRIVE_WARN_ON_EXTERNAL_SHARING"
                }
                RuleViolationInfoSuppressedActionTypesItems::RuleActivate => "RULE_ACTIVATE",
                RuleViolationInfoSuppressedActionTypesItems::RuleDeactivate => "RULE_DEACTIVATE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for RuleViolationInfoSuppressedActionTypesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RuleViolationInfoSuppressedActionTypesItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<RuleViolationInfoSuppressedActionTypesItems, ()> {
            Ok(match s {
                "ACTION_TYPE_UNSPECIFIED" => {
                    RuleViolationInfoSuppressedActionTypesItems::ActionTypeUnspecified
                }
                "ALERT" => RuleViolationInfoSuppressedActionTypesItems::Alert,
                "DRIVE_BLOCK_EXTERNAL_SHARING" => {
                    RuleViolationInfoSuppressedActionTypesItems::DriveBlockExternalSharing
                }
                "DRIVE_WARN_ON_EXTERNAL_SHARING" => {
                    RuleViolationInfoSuppressedActionTypesItems::DriveWarnOnExternalSharing
                }
                "RULE_ACTIVATE" => RuleViolationInfoSuppressedActionTypesItems::RuleActivate,
                "RULE_DEACTIVATE" => RuleViolationInfoSuppressedActionTypesItems::RuleDeactivate,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for RuleViolationInfoSuppressedActionTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RuleViolationInfoSuppressedActionTypesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RuleViolationInfoSuppressedActionTypesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTION_TYPE_UNSPECIFIED" => {
                    RuleViolationInfoSuppressedActionTypesItems::ActionTypeUnspecified
                }
                "ALERT" => RuleViolationInfoSuppressedActionTypesItems::Alert,
                "DRIVE_BLOCK_EXTERNAL_SHARING" => {
                    RuleViolationInfoSuppressedActionTypesItems::DriveBlockExternalSharing
                }
                "DRIVE_WARN_ON_EXTERNAL_SHARING" => {
                    RuleViolationInfoSuppressedActionTypesItems::DriveWarnOnExternalSharing
                }
                "RULE_ACTIVATE" => RuleViolationInfoSuppressedActionTypesItems::RuleActivate,
                "RULE_DEACTIVATE" => RuleViolationInfoSuppressedActionTypesItems::RuleDeactivate,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RuleViolationInfoSuppressedActionTypesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RuleViolationInfoSuppressedActionTypesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RuleViolationInfoTrigger {
        #[doc = "A Drive file is shared."]
        DriveShare,
        #[doc = "Trigger is unspecified."]
        TriggerUnspecified,
    }
    impl RuleViolationInfoTrigger {
        pub fn as_str(self) -> &'static str {
            match self {
                RuleViolationInfoTrigger::DriveShare => "DRIVE_SHARE",
                RuleViolationInfoTrigger::TriggerUnspecified => "TRIGGER_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for RuleViolationInfoTrigger {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RuleViolationInfoTrigger {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<RuleViolationInfoTrigger, ()> {
            Ok(match s {
                "DRIVE_SHARE" => RuleViolationInfoTrigger::DriveShare,
                "TRIGGER_UNSPECIFIED" => RuleViolationInfoTrigger::TriggerUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for RuleViolationInfoTrigger {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RuleViolationInfoTrigger {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RuleViolationInfoTrigger {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DRIVE_SHARE" => RuleViolationInfoTrigger::DriveShare,
                "TRIGGER_UNSPECIFIED" => RuleViolationInfoTrigger::TriggerUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RuleViolationInfoTrigger {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RuleViolationInfoTrigger {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RuleViolationInfoTriggeredActionTypesItems {
        #[doc = "Action type is unspecified."]
        ActionTypeUnspecified,
        #[doc = "Send alert."]
        Alert,
        #[doc = "Block sharing a file externally."]
        DriveBlockExternalSharing,
        #[doc = "Show a warning message when sharing a file externally."]
        DriveWarnOnExternalSharing,
        #[doc = "Activate Rule Action"]
        RuleActivate,
        #[doc = "Deactivate Rule Action"]
        RuleDeactivate,
    }
    impl RuleViolationInfoTriggeredActionTypesItems {
        pub fn as_str(self) -> &'static str {
            match self {
                RuleViolationInfoTriggeredActionTypesItems::ActionTypeUnspecified => {
                    "ACTION_TYPE_UNSPECIFIED"
                }
                RuleViolationInfoTriggeredActionTypesItems::Alert => "ALERT",
                RuleViolationInfoTriggeredActionTypesItems::DriveBlockExternalSharing => {
                    "DRIVE_BLOCK_EXTERNAL_SHARING"
                }
                RuleViolationInfoTriggeredActionTypesItems::DriveWarnOnExternalSharing => {
                    "DRIVE_WARN_ON_EXTERNAL_SHARING"
                }
                RuleViolationInfoTriggeredActionTypesItems::RuleActivate => "RULE_ACTIVATE",
                RuleViolationInfoTriggeredActionTypesItems::RuleDeactivate => "RULE_DEACTIVATE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for RuleViolationInfoTriggeredActionTypesItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RuleViolationInfoTriggeredActionTypesItems {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<RuleViolationInfoTriggeredActionTypesItems, ()> {
            Ok(match s {
                "ACTION_TYPE_UNSPECIFIED" => {
                    RuleViolationInfoTriggeredActionTypesItems::ActionTypeUnspecified
                }
                "ALERT" => RuleViolationInfoTriggeredActionTypesItems::Alert,
                "DRIVE_BLOCK_EXTERNAL_SHARING" => {
                    RuleViolationInfoTriggeredActionTypesItems::DriveBlockExternalSharing
                }
                "DRIVE_WARN_ON_EXTERNAL_SHARING" => {
                    RuleViolationInfoTriggeredActionTypesItems::DriveWarnOnExternalSharing
                }
                "RULE_ACTIVATE" => RuleViolationInfoTriggeredActionTypesItems::RuleActivate,
                "RULE_DEACTIVATE" => RuleViolationInfoTriggeredActionTypesItems::RuleDeactivate,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for RuleViolationInfoTriggeredActionTypesItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RuleViolationInfoTriggeredActionTypesItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RuleViolationInfoTriggeredActionTypesItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTION_TYPE_UNSPECIFIED" => {
                    RuleViolationInfoTriggeredActionTypesItems::ActionTypeUnspecified
                }
                "ALERT" => RuleViolationInfoTriggeredActionTypesItems::Alert,
                "DRIVE_BLOCK_EXTERNAL_SHARING" => {
                    RuleViolationInfoTriggeredActionTypesItems::DriveBlockExternalSharing
                }
                "DRIVE_WARN_ON_EXTERNAL_SHARING" => {
                    RuleViolationInfoTriggeredActionTypesItems::DriveWarnOnExternalSharing
                }
                "RULE_ACTIVATE" => RuleViolationInfoTriggeredActionTypesItems::RuleActivate,
                "RULE_DEACTIVATE" => RuleViolationInfoTriggeredActionTypesItems::RuleDeactivate,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RuleViolationInfoTriggeredActionTypesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RuleViolationInfoTriggeredActionTypesItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Settings {
        #[doc = "The list of notifications."]
        #[serde(
            rename = "notifications",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub notifications: ::std::option::Option<Vec<crate::schemas::Notification>>,
    }
    impl ::google_field_selector::FieldSelector for Settings {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Settings {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct StateSponsoredAttack {
        #[doc = "The email of the user this incident was created for."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for StateSponsoredAttack {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StateSponsoredAttack {
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
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SuspiciousActivity {
        #[doc = "The email of the user this alert was created for."]
        #[serde(
            rename = "email",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email: ::std::option::Option<String>,
        #[doc = "Required. The list of security events."]
        #[serde(
            rename = "events",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub events: ::std::option::Option<Vec<crate::schemas::SuspiciousActivitySecurityDetail>>,
    }
    impl ::google_field_selector::FieldSelector for SuspiciousActivity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuspiciousActivity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SuspiciousActivitySecurityDetail {
        #[doc = "Required. The device ID."]
        #[serde(
            rename = "deviceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_id: ::std::option::Option<String>,
        #[doc = "The model of the device."]
        #[serde(
            rename = "deviceModel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_model: ::std::option::Option<String>,
        #[doc = "The device property which was changed."]
        #[serde(
            rename = "deviceProperty",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_property: ::std::option::Option<String>,
        #[doc = "The type of the device."]
        #[serde(
            rename = "deviceType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_type: ::std::option::Option<String>,
        #[doc = "Required for iOS, empty for others."]
        #[serde(
            rename = "iosVendorId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ios_vendor_id: ::std::option::Option<String>,
        #[doc = "The new value of the device property after the change."]
        #[serde(
            rename = "newValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_value: ::std::option::Option<String>,
        #[doc = "The old value of the device property before the change."]
        #[serde(
            rename = "oldValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub old_value: ::std::option::Option<String>,
        #[doc = "The device resource ID."]
        #[serde(
            rename = "resourceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_id: ::std::option::Option<String>,
        #[doc = "The serial number of the device."]
        #[serde(
            rename = "serialNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub serial_number: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SuspiciousActivitySecurityDetail {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SuspiciousActivitySecurityDetail {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UndeleteAlertRequest {
        #[doc = "Optional. The unique identifier of the Google Workspace organization account of the customer the alert is associated with. Inferred from the caller identity if not provided."]
        #[serde(
            rename = "customerId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub customer_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UndeleteAlertRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UndeleteAlertRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct User {
        #[doc = "Display name of the user."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Email address of the user."]
        #[serde(
            rename = "emailAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub email_address: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for User {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for User {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UserChanges {
        #[doc = "Rule name"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UserChanges {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UserChanges {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UserDefinedDetectorInfo {
        #[doc = "Display name of the detector."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Resource name that uniquely identifies the detector."]
        #[serde(
            rename = "resourceName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resource_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for UserDefinedDetectorInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UserDefinedDetectorInfo {
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
    #[doc = "Actions that can be performed on the alerts resource"]
    pub fn alerts(&self) -> crate::resources::alerts::AlertsActions {
        crate::resources::alerts::AlertsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the v_1beta_1 resource"]
    pub fn v_1beta_1(&self) -> crate::resources::v_1beta_1::V1Beta1Actions {
        crate::resources::v_1beta_1::V1Beta1Actions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod alerts {
        pub mod params {}
        pub struct AlertsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> AlertsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Performs batch delete operation on alerts."]
            pub fn batch_delete(
                &self,
                request: crate::schemas::BatchDeleteAlertsRequest,
            ) -> BatchDeleteRequestBuilder {
                BatchDeleteRequestBuilder {
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
            #[doc = "Performs batch undelete operation on alerts."]
            pub fn batch_undelete(
                &self,
                request: crate::schemas::BatchUndeleteAlertsRequest,
            ) -> BatchUndeleteRequestBuilder {
                BatchUndeleteRequestBuilder {
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
            #[doc = "Marks the specified alert for deletion. An alert that has been marked for deletion is removed from Alert Center after 30 days. Marking an alert for deletion has no effect on an alert which has already been marked for deletion. Attempting to mark a nonexistent alert for deletion results in a `NOT_FOUND` error."]
            pub fn delete(&self, alert_id: impl Into<String>) -> DeleteRequestBuilder {
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
                    alert_id: alert_id.into(),
                    customer_id: None,
                }
            }
            #[doc = "Gets the specified alert. Attempting to get a nonexistent alert returns `NOT_FOUND` error."]
            pub fn get(&self, alert_id: impl Into<String>) -> GetRequestBuilder {
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
                    alert_id: alert_id.into(),
                    customer_id: None,
                }
            }
            #[doc = "Returns the metadata of an alert. Attempting to get metadata for a non-existent alert returns `NOT_FOUND` error."]
            pub fn get_metadata(&self, alert_id: impl Into<String>) -> GetMetadataRequestBuilder {
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
                    alert_id: alert_id.into(),
                    customer_id: None,
                }
            }
            #[doc = "Lists the alerts."]
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
                    customer_id: None,
                    filter: None,
                    order_by: None,
                    page_size: None,
                    page_token: None,
                }
            }
            #[doc = "Restores, or \"undeletes\", an alert that was marked for deletion within the past 30 days. Attempting to undelete an alert which was marked for deletion over 30 days ago (which has been removed from the Alert Center database) or a nonexistent alert returns a `NOT_FOUND` error. Attempting to undelete an alert which has not been marked for deletion has no effect."]
            pub fn undelete(
                &self,
                request: crate::schemas::UndeleteAlertRequest,
                alert_id: impl Into<String>,
            ) -> UndeleteRequestBuilder {
                UndeleteRequestBuilder {
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
                    alert_id: alert_id.into(),
                }
            }
            #[doc = "Actions that can be performed on the feedback resource"]
            pub fn feedback(&self) -> crate::resources::alerts::feedback::FeedbackActions {
                crate::resources::alerts::feedback::FeedbackActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [AlertsActions::batch_delete()](struct.AlertsActions.html#method.batch_delete)"]
        #[derive(Debug, Clone)]
        pub struct BatchDeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::BatchDeleteAlertsRequest,
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
        impl<'a> BatchDeleteRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::BatchDeleteAlertsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::BatchDeleteAlertsResponse, crate::Error> {
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
                let mut output = "https://alertcenter.googleapis.com/".to_owned();
                output.push_str("v1beta1/alerts:batchDelete");
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
        #[doc = "Created via [AlertsActions::batch_undelete()](struct.AlertsActions.html#method.batch_undelete)"]
        #[derive(Debug, Clone)]
        pub struct BatchUndeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::BatchUndeleteAlertsRequest,
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
        impl<'a> BatchUndeleteRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::BatchUndeleteAlertsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::BatchUndeleteAlertsResponse, crate::Error> {
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
                let mut output = "https://alertcenter.googleapis.com/".to_owned();
                output.push_str("v1beta1/alerts:batchUndelete");
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
        #[doc = "Created via [AlertsActions::delete()](struct.AlertsActions.html#method.delete)"]
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            alert_id: String,
            customer_id: Option<String>,
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
            #[doc = "Optional. The unique identifier of the Google Workspace organization account of the customer the alert is associated with. Inferred from the caller identity if not provided."]
            pub fn customer_id(mut self, value: impl Into<String>) -> Self {
                self.customer_id = Some(value.into());
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
            ) -> Result<crate::schemas::Empty, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Empty, crate::Error> {
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
                let mut output = "https://alertcenter.googleapis.com/".to_owned();
                output.push_str("v1beta1/alerts/");
                {
                    let var_as_str = &self.alert_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::DELETE, path);
                req = req.query(&[("customerId", &self.customer_id)]);
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
        #[doc = "Created via [AlertsActions::get()](struct.AlertsActions.html#method.get)"]
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            alert_id: String,
            customer_id: Option<String>,
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
            #[doc = "Optional. The unique identifier of the Google Workspace organization account of the customer the alert is associated with. Inferred from the caller identity if not provided."]
            pub fn customer_id(mut self, value: impl Into<String>) -> Self {
                self.customer_id = Some(value.into());
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
            ) -> Result<crate::schemas::Alert, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Alert, crate::Error> {
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
                let mut output = "https://alertcenter.googleapis.com/".to_owned();
                output.push_str("v1beta1/alerts/");
                {
                    let var_as_str = &self.alert_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("customerId", &self.customer_id)]);
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
        #[doc = "Created via [AlertsActions::get_metadata()](struct.AlertsActions.html#method.get_metadata)"]
        #[derive(Debug, Clone)]
        pub struct GetMetadataRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            alert_id: String,
            customer_id: Option<String>,
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
        impl<'a> GetMetadataRequestBuilder<'a> {
            #[doc = "Optional. The unique identifier of the Google Workspace organization account of the customer the alert metadata is associated with. Inferred from the caller identity if not provided."]
            pub fn customer_id(mut self, value: impl Into<String>) -> Self {
                self.customer_id = Some(value.into());
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
            ) -> Result<crate::schemas::AlertMetadata, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::AlertMetadata, crate::Error> {
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
                let mut output = "https://alertcenter.googleapis.com/".to_owned();
                output.push_str("v1beta1/alerts/");
                {
                    let var_as_str = &self.alert_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/metadata");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("customerId", &self.customer_id)]);
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
        #[doc = "Created via [AlertsActions::list()](struct.AlertsActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            customer_id: Option<String>,
            filter: Option<String>,
            order_by: Option<String>,
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
            #[doc = "Optional. The unique identifier of the Google Workspace organization account of the customer the alerts are associated with. Inferred from the caller identity if not provided."]
            pub fn customer_id(mut self, value: impl Into<String>) -> Self {
                self.customer_id = Some(value.into());
                self
            }
            #[doc = "Optional. A query string for filtering alert results. For more details, see [Query filters](https://developers.google.com/admin-sdk/alertcenter/guides/query-filters) and [Supported query filter fields](https://developers.google.com/admin-sdk/alertcenter/reference/filter-fields#alerts.list)."]
            pub fn filter(mut self, value: impl Into<String>) -> Self {
                self.filter = Some(value.into());
                self
            }
            #[doc = "Optional. The sort order of the list results. If not specified results may be returned in arbitrary order. You can sort the results in descending order based on the creation timestamp using `order_by=\"create_time desc\"`. Currently, supported sorting are `create_time asc`, `create_time desc`, `update_time desc`"]
            pub fn order_by(mut self, value: impl Into<String>) -> Self {
                self.order_by = Some(value.into());
                self
            }
            #[doc = "Optional. The requested page size. Server may return fewer items than requested. If unspecified, server picks an appropriate default."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "Optional. A token identifying a page of results the server should return. If empty, a new iteration is started. To continue an iteration, pass in the value from the previous ListAlertsResponse's next_page_token field."]
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
            ) -> Result<crate::schemas::ListAlertsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListAlertsResponse, crate::Error> {
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
                let mut output = "https://alertcenter.googleapis.com/".to_owned();
                output.push_str("v1beta1/alerts");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("customerId", &self.customer_id)]);
                req = req.query(&[("filter", &self.filter)]);
                req = req.query(&[("orderBy", &self.order_by)]);
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
        #[doc = "Created via [AlertsActions::undelete()](struct.AlertsActions.html#method.undelete)"]
        #[derive(Debug, Clone)]
        pub struct UndeleteRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::UndeleteAlertRequest,
            alert_id: String,
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
        impl<'a> UndeleteRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::Alert, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Alert, crate::Error> {
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
                let mut output = "https://alertcenter.googleapis.com/".to_owned();
                output.push_str("v1beta1/alerts/");
                {
                    let var_as_str = &self.alert_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str(":undelete");
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
        pub mod feedback {
            pub mod params {}
            pub struct FeedbackActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> FeedbackActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Creates new feedback for an alert. Attempting to create a feedback for a non-existent alert returns `NOT_FOUND` error. Attempting to create a feedback for an alert that is marked for deletion returns `FAILED_PRECONDITION' error."]
                pub fn create(
                    &self,
                    request: crate::schemas::AlertFeedback,
                    alert_id: impl Into<String>,
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
                        alert_id: alert_id.into(),
                        customer_id: None,
                    }
                }
                #[doc = "Lists all the feedback for an alert. Attempting to list feedbacks for a non-existent alert returns `NOT_FOUND` error."]
                pub fn list(&self, alert_id: impl Into<String>) -> ListRequestBuilder {
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
                        alert_id: alert_id.into(),
                        customer_id: None,
                        filter: None,
                    }
                }
            }
            #[doc = "Created via [FeedbackActions::create()](struct.FeedbackActions.html#method.create)"]
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::AlertFeedback,
                alert_id: String,
                customer_id: Option<String>,
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
                #[doc = "Optional. The unique identifier of the Google Workspace organization account of the customer the alert is associated with. Inferred from the caller identity if not provided."]
                pub fn customer_id(mut self, value: impl Into<String>) -> Self {
                    self.customer_id = Some(value.into());
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
                ) -> Result<crate::schemas::AlertFeedback, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::AlertFeedback, crate::Error> {
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
                    let mut output = "https://alertcenter.googleapis.com/".to_owned();
                    output.push_str("v1beta1/alerts/");
                    {
                        let var_as_str = &self.alert_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/feedback");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                    req = req.query(&[("customerId", &self.customer_id)]);
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
            #[doc = "Created via [FeedbackActions::list()](struct.FeedbackActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                alert_id: String,
                customer_id: Option<String>,
                filter: Option<String>,
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
                #[doc = "Optional. The unique identifier of the Google Workspace organization account of the customer the alert feedback are associated with. Inferred from the caller identity if not provided."]
                pub fn customer_id(mut self, value: impl Into<String>) -> Self {
                    self.customer_id = Some(value.into());
                    self
                }
                #[doc = "Optional. A query string for filtering alert feedback results. For more details, see [Query filters](https://developers.google.com/admin-sdk/alertcenter/guides/query-filters) and [Supported query filter fields](https://developers.google.com/admin-sdk/alertcenter/reference/filter-fields#alerts.feedback.list)."]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
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
                ) -> Result<crate::schemas::ListAlertFeedbackResponse, crate::Error>
                {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListAlertFeedbackResponse, crate::Error>
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
                    let mut output = "https://alertcenter.googleapis.com/".to_owned();
                    output.push_str("v1beta1/alerts/");
                    {
                        let var_as_str = &self.alert_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/feedback");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("customerId", &self.customer_id)]);
                    req = req.query(&[("filter", &self.filter)]);
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
    pub mod v_1beta_1 {
        pub mod params {}
        pub struct V1Beta1Actions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> V1Beta1Actions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Returns customer-level settings."]
            pub fn get_settings(&self) -> GetSettingsRequestBuilder {
                GetSettingsRequestBuilder {
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
                    customer_id: None,
                }
            }
            #[doc = "Updates the customer-level settings."]
            pub fn update_settings(
                &self,
                request: crate::schemas::Settings,
            ) -> UpdateSettingsRequestBuilder {
                UpdateSettingsRequestBuilder {
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
                    customer_id: None,
                }
            }
        }
        #[doc = "Created via [V1Beta1Actions::get_settings()](struct.V1Beta1Actions.html#method.get_settings)"]
        #[derive(Debug, Clone)]
        pub struct GetSettingsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            customer_id: Option<String>,
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
        impl<'a> GetSettingsRequestBuilder<'a> {
            #[doc = "Optional. The unique identifier of the Google Workspace organization account of the customer the alert settings are associated with. Inferred from the caller identity if not provided."]
            pub fn customer_id(mut self, value: impl Into<String>) -> Self {
                self.customer_id = Some(value.into());
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
            ) -> Result<crate::schemas::Settings, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Settings, crate::Error> {
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
                let mut output = "https://alertcenter.googleapis.com/".to_owned();
                output.push_str("v1beta1/settings");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("customerId", &self.customer_id)]);
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
        #[doc = "Created via [V1Beta1Actions::update_settings()](struct.V1Beta1Actions.html#method.update_settings)"]
        #[derive(Debug, Clone)]
        pub struct UpdateSettingsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Settings,
            customer_id: Option<String>,
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
        impl<'a> UpdateSettingsRequestBuilder<'a> {
            #[doc = "Optional. The unique identifier of the Google Workspace organization account of the customer the alert settings are associated with. Inferred from the caller identity if not provided."]
            pub fn customer_id(mut self, value: impl Into<String>) -> Self {
                self.customer_id = Some(value.into());
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
            ) -> Result<crate::schemas::Settings, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Settings, crate::Error> {
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
                let mut output = "https://alertcenter.googleapis.com/".to_owned();
                output.push_str("v1beta1/settings");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::PATCH, path);
                req = req.query(&[("customerId", &self.customer_id)]);
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
