#![doc = "# Resources and Methods\n    * [customers](resources/customers/struct.CustomersActions.html)\n      * [apps](resources/customers/apps/struct.AppsActions.html)\n        * [*countChromeAppRequests*](resources/customers/apps/struct.CountChromeAppRequestsRequestBuilder.html)\n        * [android](resources/customers/apps/android/struct.AndroidActions.html)\n          * [*get*](resources/customers/apps/android/struct.GetRequestBuilder.html)\n        * [chrome](resources/customers/apps/chrome/struct.ChromeActions.html)\n          * [*get*](resources/customers/apps/chrome/struct.GetRequestBuilder.html)\n        * [web](resources/customers/apps/web/struct.WebActions.html)\n          * [*get*](resources/customers/apps/web/struct.GetRequestBuilder.html)\n      * [reports](resources/customers/reports/struct.ReportsActions.html)\n        * [*countChromeVersions*](resources/customers/reports/struct.CountChromeVersionsRequestBuilder.html), [*countInstalledApps*](resources/customers/reports/struct.CountInstalledAppsRequestBuilder.html), [*findInstalledAppDevices*](resources/customers/reports/struct.FindInstalledAppDevicesRequestBuilder.html)\n      * [telemetry](resources/customers/telemetry/struct.TelemetryActions.html)\n        * [devices](resources/customers/telemetry/devices/struct.DevicesActions.html)\n          * [*list*](resources/customers/telemetry/devices/struct.ListRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "See detailed information about apps installed on Chrome browsers and devices managed by your organization\n\n`https://www.googleapis.com/auth/chrome.management.appdetails.readonly`"]
    pub const CHROME_MANAGEMENT_APPDETAILS_READONLY: &str =
        "https://www.googleapis.com/auth/chrome.management.appdetails.readonly";
    #[doc = "See reports about devices and Chrome browsers managed within your organization\n\n`https://www.googleapis.com/auth/chrome.management.reports.readonly`"]
    pub const CHROME_MANAGEMENT_REPORTS_READONLY: &str =
        "https://www.googleapis.com/auth/chrome.management.reports.readonly";
    #[doc = "See basic device and telemetry information collected from Chrome OS devices or users managed within your organization\n\n`https://www.googleapis.com/auth/chrome.management.telemetry.readonly`"]
    pub const CHROME_MANAGEMENT_TELEMETRY_READONLY: &str =
        "https://www.googleapis.com/auth/chrome.management.telemetry.readonly";
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
    pub struct GoogleChromeManagementV1AndroidAppInfo {
        #[doc = "Output only. Permissions requested by an Android app."]
        #[serde(
            rename = "permissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permissions: ::std::option::Option<
            Vec<crate::schemas::GoogleChromeManagementV1AndroidAppPermission>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1AndroidAppInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1AndroidAppInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1AndroidAppPermission {
        #[doc = "Output only. The type of the permission."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1AndroidAppPermission {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1AndroidAppPermission {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleChromeManagementV1AppDetails {
        #[doc = "Output only. Android app information."]
        #[serde(
            rename = "androidAppInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub android_app_info:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1AndroidAppInfo>,
        #[doc = "Output only. Unique store identifier for the item. Examples: \"gmbmikajjgmnabiglmofipeabaddhgne\" for the Save to Google Drive Chrome extension, \"com.google.android.apps.docs\" for the Google Drive Android app."]
        #[serde(
            rename = "appId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_id: ::std::option::Option<String>,
        #[doc = "Output only. Chrome Web Store app information."]
        #[serde(
            rename = "chromeAppInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub chrome_app_info:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1ChromeAppInfo>,
        #[doc = "Output only. App's description."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Output only. The uri for the detail page of the item."]
        #[serde(
            rename = "detailUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detail_uri: ::std::option::Option<String>,
        #[doc = "Output only. App's display name."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Output only. First published time."]
        #[serde(
            rename = "firstPublishTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub first_publish_time: ::std::option::Option<String>,
        #[doc = "Output only. Home page or Website uri."]
        #[serde(
            rename = "homepageUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub homepage_uri: ::std::option::Option<String>,
        #[doc = "Output only. A link to an image that can be used as an icon for the product."]
        #[serde(
            rename = "iconUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub icon_uri: ::std::option::Option<String>,
        #[doc = "Output only. Indicates if the app has to be paid for OR has paid content."]
        #[serde(
            rename = "isPaidApp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_paid_app: ::std::option::Option<bool>,
        #[doc = "Output only. Latest published time."]
        #[serde(
            rename = "latestPublishTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub latest_publish_time: ::std::option::Option<String>,
        #[doc = "Output only. Format: name=customers/{customer_id}/apps/{chrome|android|web}/{app_id}@{version}"]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. The URI pointing to the privacy policy of the app, if it was provided by the developer. Version-specific field that will only be set when the requested app version is found."]
        #[serde(
            rename = "privacyPolicyUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub privacy_policy_uri: ::std::option::Option<String>,
        #[doc = "Output only. The publisher of the item."]
        #[serde(
            rename = "publisher",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub publisher: ::std::option::Option<String>,
        #[doc = "Output only. App type."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::GoogleChromeManagementV1AppDetailsType>,
        #[doc = "Output only. Number of reviews received. Chrome Web Store review information will always be for the latest version of an app."]
        #[serde(
            rename = "reviewNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub review_number: ::std::option::Option<i64>,
        #[doc = "Output only. The rating of the app (on 5 stars). Chrome Web Store review information will always be for the latest version of an app."]
        #[serde(
            rename = "reviewRating",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub review_rating: ::std::option::Option<f32>,
        #[doc = "Output only. App version. A new revision is committed whenever a new version of the app is published."]
        #[serde(
            rename = "revisionId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub revision_id: ::std::option::Option<String>,
        #[doc = "Output only. Information about a partial service error if applicable."]
        #[serde(
            rename = "serviceError",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub service_error: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1AppDetails {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1AppDetails {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1AppDetailsType {
        #[doc = "ARC++ app."]
        Android,
        #[doc = "App type unspecified."]
        AppItemTypeUnspecified,
        #[doc = "Chrome app."]
        Chrome,
        #[doc = "Web app."]
        Web,
    }
    impl GoogleChromeManagementV1AppDetailsType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleChromeManagementV1AppDetailsType::Android => "ANDROID",
                GoogleChromeManagementV1AppDetailsType::AppItemTypeUnspecified => {
                    "APP_ITEM_TYPE_UNSPECIFIED"
                }
                GoogleChromeManagementV1AppDetailsType::Chrome => "CHROME",
                GoogleChromeManagementV1AppDetailsType::Web => "WEB",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1AppDetailsType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1AppDetailsType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<GoogleChromeManagementV1AppDetailsType, ()> {
            Ok(match s {
                "ANDROID" => GoogleChromeManagementV1AppDetailsType::Android,
                "APP_ITEM_TYPE_UNSPECIFIED" => {
                    GoogleChromeManagementV1AppDetailsType::AppItemTypeUnspecified
                }
                "CHROME" => GoogleChromeManagementV1AppDetailsType::Chrome,
                "WEB" => GoogleChromeManagementV1AppDetailsType::Web,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1AppDetailsType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1AppDetailsType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1AppDetailsType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANDROID" => GoogleChromeManagementV1AppDetailsType::Android,
                "APP_ITEM_TYPE_UNSPECIFIED" => {
                    GoogleChromeManagementV1AppDetailsType::AppItemTypeUnspecified
                }
                "CHROME" => GoogleChromeManagementV1AppDetailsType::Chrome,
                "WEB" => GoogleChromeManagementV1AppDetailsType::Web,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1AppDetailsType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1AppDetailsType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1AudioStatusReport {
        #[doc = "Output only. Active input device's name."]
        #[serde(
            rename = "inputDevice",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_device: ::std::option::Option<String>,
        #[doc = "Output only. Active input device's gain in [0, 100]."]
        #[serde(
            rename = "inputGain",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_gain: ::std::option::Option<i32>,
        #[doc = "Output only. Is active input device mute or not."]
        #[serde(
            rename = "inputMute",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_mute: ::std::option::Option<bool>,
        #[doc = "Output only. Active output device's name."]
        #[serde(
            rename = "outputDevice",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_device: ::std::option::Option<String>,
        #[doc = "Output only. Is active output device mute or not."]
        #[serde(
            rename = "outputMute",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_mute: ::std::option::Option<bool>,
        #[doc = "Output only. Active output device's volume in [0, 100]."]
        #[serde(
            rename = "outputVolume",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_volume: ::std::option::Option<i32>,
        #[doc = "Output only. Timestamp of when the sample was collected on device."]
        #[serde(
            rename = "reportTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1AudioStatusReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1AudioStatusReport {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1BatteryInfo {
        #[doc = "Output only. Design capacity (mAmpere-hours)."]
        #[serde(
            rename = "designCapacity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub design_capacity: ::std::option::Option<i64>,
        #[doc = "Output only. Designed minimum output voltage (mV)"]
        #[serde(
            rename = "designMinVoltage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub design_min_voltage: ::std::option::Option<i32>,
        #[doc = "Output only. The date the battery was manufactured."]
        #[serde(
            rename = "manufactureDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub manufacture_date: ::std::option::Option<crate::schemas::GoogleTypeDate>,
        #[doc = "Output only. Battery manufacturer."]
        #[serde(
            rename = "manufacturer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub manufacturer: ::std::option::Option<String>,
        #[doc = "Output only. Battery serial number."]
        #[serde(
            rename = "serialNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub serial_number: ::std::option::Option<String>,
        #[doc = "Output only. Technology of the battery. Example: Li-ion"]
        #[serde(
            rename = "technology",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub technology: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1BatteryInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1BatteryInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1BatterySampleReport {
        #[doc = "Output only. Battery charge percentage."]
        #[serde(
            rename = "chargeRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub charge_rate: ::std::option::Option<i32>,
        #[doc = "Output only. Battery current (mA)."]
        #[serde(
            rename = "current",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub current: ::std::option::Option<i64>,
        #[doc = "Output only. The battery discharge rate measured in mW. Positive if the battery is being discharged, negative if it's being charged."]
        #[serde(
            rename = "dischargeRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub discharge_rate: ::std::option::Option<i32>,
        #[doc = "Output only. Battery remaining capacity (mAmpere-hours)."]
        #[serde(
            rename = "remainingCapacity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub remaining_capacity: ::std::option::Option<i64>,
        #[doc = "Output only. Timestamp of when the sample was collected on device"]
        #[serde(
            rename = "reportTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_time: ::std::option::Option<String>,
        #[doc = "Output only. Battery status read from sysfs. Example: Discharging"]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<String>,
        #[doc = "Output only. Temperature in Celsius degrees."]
        #[serde(
            rename = "temperature",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub temperature: ::std::option::Option<i32>,
        #[doc = "Output only. Battery voltage (millivolt)."]
        #[serde(
            rename = "voltage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub voltage: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1BatterySampleReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1BatterySampleReport {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1BatteryStatusReport {
        #[doc = "Output only. Battery health."]
        #[serde(
            rename = "batteryHealth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub battery_health: ::std::option::Option<
            crate::schemas::GoogleChromeManagementV1BatteryStatusReportBatteryHealth,
        >,
        #[doc = "Output only. Cycle count."]
        #[serde(
            rename = "cycleCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cycle_count: ::std::option::Option<i32>,
        #[doc = "Output only. Full charge capacity (mAmpere-hours)."]
        #[serde(
            rename = "fullChargeCapacity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub full_charge_capacity: ::std::option::Option<i64>,
        #[doc = "Output only. Timestamp of when the sample was collected on device"]
        #[serde(
            rename = "reportTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_time: ::std::option::Option<String>,
        #[doc = "Output only. Sampling data for the battery sorted in a decreasing order of report_time."]
        #[serde(
            rename = "sample",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sample:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1BatterySampleReport>>,
        #[doc = "Output only. Battery serial number."]
        #[serde(
            rename = "serialNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub serial_number: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1BatteryStatusReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1BatteryStatusReport {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1BatteryStatusReportBatteryHealth {
        #[doc = "Battery is healthy."]
        BatteryHealthNormal,
        #[doc = "Health unknown."]
        BatteryHealthUnspecified,
        #[doc = "Battery is unhealthy and should be replaced."]
        BatteryReplaceNow,
        #[doc = "Battery is moderately unhealthy and should be replaced soon."]
        BatteryReplaceSoon,
    }
    impl GoogleChromeManagementV1BatteryStatusReportBatteryHealth {
        pub fn as_str(self) -> &'static str {
            match self { GoogleChromeManagementV1BatteryStatusReportBatteryHealth :: BatteryHealthNormal => "BATTERY_HEALTH_NORMAL" , GoogleChromeManagementV1BatteryStatusReportBatteryHealth :: BatteryHealthUnspecified => "BATTERY_HEALTH_UNSPECIFIED" , GoogleChromeManagementV1BatteryStatusReportBatteryHealth :: BatteryReplaceNow => "BATTERY_REPLACE_NOW" , GoogleChromeManagementV1BatteryStatusReportBatteryHealth :: BatteryReplaceSoon => "BATTERY_REPLACE_SOON" , }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1BatteryStatusReportBatteryHealth {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1BatteryStatusReportBatteryHealth {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1BatteryStatusReportBatteryHealth, ()>
        {
            Ok (match s { "BATTERY_HEALTH_NORMAL" => GoogleChromeManagementV1BatteryStatusReportBatteryHealth :: BatteryHealthNormal , "BATTERY_HEALTH_UNSPECIFIED" => GoogleChromeManagementV1BatteryStatusReportBatteryHealth :: BatteryHealthUnspecified , "BATTERY_REPLACE_NOW" => GoogleChromeManagementV1BatteryStatusReportBatteryHealth :: BatteryReplaceNow , "BATTERY_REPLACE_SOON" => GoogleChromeManagementV1BatteryStatusReportBatteryHealth :: BatteryReplaceSoon , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1BatteryStatusReportBatteryHealth {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1BatteryStatusReportBatteryHealth {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1BatteryStatusReportBatteryHealth {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "BATTERY_HEALTH_NORMAL" => GoogleChromeManagementV1BatteryStatusReportBatteryHealth :: BatteryHealthNormal , "BATTERY_HEALTH_UNSPECIFIED" => GoogleChromeManagementV1BatteryStatusReportBatteryHealth :: BatteryHealthUnspecified , "BATTERY_REPLACE_NOW" => GoogleChromeManagementV1BatteryStatusReportBatteryHealth :: BatteryReplaceNow , "BATTERY_REPLACE_SOON" => GoogleChromeManagementV1BatteryStatusReportBatteryHealth :: BatteryReplaceSoon , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1BatteryStatusReportBatteryHealth
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleChromeManagementV1BatteryStatusReportBatteryHealth
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
    pub struct GoogleChromeManagementV1BrowserVersion {
        #[doc = "Output only. The release channel of the installed browser."]
        #[serde(
            rename = "channel",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub channel:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1BrowserVersionChannel>,
        #[doc = "Output only. Count grouped by device_system and major version"]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub count: ::std::option::Option<i64>,
        #[doc = "Output only. Version of the system-specified operating system."]
        #[serde(
            rename = "deviceOsVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_os_version: ::std::option::Option<String>,
        #[doc = "Output only. The device operating system."]
        #[serde(
            rename = "system",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub system:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1BrowserVersionSystem>,
        #[doc = "Output only. The full version of the installed browser."]
        #[serde(
            rename = "version",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1BrowserVersion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1BrowserVersion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1BrowserVersionChannel {
        #[doc = "Beta release channel."]
        Beta,
        #[doc = "Canary release channel."]
        Canary,
        #[doc = "Dev release channel."]
        Dev,
        #[doc = "No release channel specified."]
        ReleaseChannelUnspecified,
        #[doc = "Stable release channel."]
        Stable,
    }
    impl GoogleChromeManagementV1BrowserVersionChannel {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleChromeManagementV1BrowserVersionChannel::Beta => "BETA",
                GoogleChromeManagementV1BrowserVersionChannel::Canary => "CANARY",
                GoogleChromeManagementV1BrowserVersionChannel::Dev => "DEV",
                GoogleChromeManagementV1BrowserVersionChannel::ReleaseChannelUnspecified => {
                    "RELEASE_CHANNEL_UNSPECIFIED"
                }
                GoogleChromeManagementV1BrowserVersionChannel::Stable => "STABLE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1BrowserVersionChannel {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1BrowserVersionChannel {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1BrowserVersionChannel, ()> {
            Ok(match s {
                "BETA" => GoogleChromeManagementV1BrowserVersionChannel::Beta,
                "CANARY" => GoogleChromeManagementV1BrowserVersionChannel::Canary,
                "DEV" => GoogleChromeManagementV1BrowserVersionChannel::Dev,
                "RELEASE_CHANNEL_UNSPECIFIED" => {
                    GoogleChromeManagementV1BrowserVersionChannel::ReleaseChannelUnspecified
                }
                "STABLE" => GoogleChromeManagementV1BrowserVersionChannel::Stable,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1BrowserVersionChannel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1BrowserVersionChannel {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1BrowserVersionChannel {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BETA" => GoogleChromeManagementV1BrowserVersionChannel::Beta,
                "CANARY" => GoogleChromeManagementV1BrowserVersionChannel::Canary,
                "DEV" => GoogleChromeManagementV1BrowserVersionChannel::Dev,
                "RELEASE_CHANNEL_UNSPECIFIED" => {
                    GoogleChromeManagementV1BrowserVersionChannel::ReleaseChannelUnspecified
                }
                "STABLE" => GoogleChromeManagementV1BrowserVersionChannel::Stable,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1BrowserVersionChannel {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1BrowserVersionChannel {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1BrowserVersionSystem {
        #[doc = "No operating system specified."]
        DeviceSystemUnspecified,
        #[doc = "Android operating system."]
        SystemAndroid,
        #[doc = "ChromeOS operating system."]
        SystemCros,
        #[doc = "Apple iOS operating system."]
        SystemIos,
        #[doc = "Linux operating system."]
        SystemLinux,
        #[doc = "Apple macOS operating system."]
        SystemMac,
        #[doc = "Other operating system."]
        SystemOther,
        #[doc = "Microsoft Windows operating system."]
        SystemWindows,
    }
    impl GoogleChromeManagementV1BrowserVersionSystem {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleChromeManagementV1BrowserVersionSystem::DeviceSystemUnspecified => {
                    "DEVICE_SYSTEM_UNSPECIFIED"
                }
                GoogleChromeManagementV1BrowserVersionSystem::SystemAndroid => "SYSTEM_ANDROID",
                GoogleChromeManagementV1BrowserVersionSystem::SystemCros => "SYSTEM_CROS",
                GoogleChromeManagementV1BrowserVersionSystem::SystemIos => "SYSTEM_IOS",
                GoogleChromeManagementV1BrowserVersionSystem::SystemLinux => "SYSTEM_LINUX",
                GoogleChromeManagementV1BrowserVersionSystem::SystemMac => "SYSTEM_MAC",
                GoogleChromeManagementV1BrowserVersionSystem::SystemOther => "SYSTEM_OTHER",
                GoogleChromeManagementV1BrowserVersionSystem::SystemWindows => "SYSTEM_WINDOWS",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1BrowserVersionSystem {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1BrowserVersionSystem {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1BrowserVersionSystem, ()> {
            Ok(match s {
                "DEVICE_SYSTEM_UNSPECIFIED" => {
                    GoogleChromeManagementV1BrowserVersionSystem::DeviceSystemUnspecified
                }
                "SYSTEM_ANDROID" => GoogleChromeManagementV1BrowserVersionSystem::SystemAndroid,
                "SYSTEM_CROS" => GoogleChromeManagementV1BrowserVersionSystem::SystemCros,
                "SYSTEM_IOS" => GoogleChromeManagementV1BrowserVersionSystem::SystemIos,
                "SYSTEM_LINUX" => GoogleChromeManagementV1BrowserVersionSystem::SystemLinux,
                "SYSTEM_MAC" => GoogleChromeManagementV1BrowserVersionSystem::SystemMac,
                "SYSTEM_OTHER" => GoogleChromeManagementV1BrowserVersionSystem::SystemOther,
                "SYSTEM_WINDOWS" => GoogleChromeManagementV1BrowserVersionSystem::SystemWindows,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1BrowserVersionSystem {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1BrowserVersionSystem {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1BrowserVersionSystem {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEVICE_SYSTEM_UNSPECIFIED" => {
                    GoogleChromeManagementV1BrowserVersionSystem::DeviceSystemUnspecified
                }
                "SYSTEM_ANDROID" => GoogleChromeManagementV1BrowserVersionSystem::SystemAndroid,
                "SYSTEM_CROS" => GoogleChromeManagementV1BrowserVersionSystem::SystemCros,
                "SYSTEM_IOS" => GoogleChromeManagementV1BrowserVersionSystem::SystemIos,
                "SYSTEM_LINUX" => GoogleChromeManagementV1BrowserVersionSystem::SystemLinux,
                "SYSTEM_MAC" => GoogleChromeManagementV1BrowserVersionSystem::SystemMac,
                "SYSTEM_OTHER" => GoogleChromeManagementV1BrowserVersionSystem::SystemOther,
                "SYSTEM_WINDOWS" => GoogleChromeManagementV1BrowserVersionSystem::SystemWindows,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1BrowserVersionSystem {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1BrowserVersionSystem {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1ChromeAppInfo {
        #[doc = "Output only. Whether the app or extension is built and maintained by Google. Version-specific field that will only be set when the requested app version is found."]
        #[serde(
            rename = "googleOwned",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_owned: ::std::option::Option<bool>,
        #[doc = "Output only. Whether the app or extension is in a published state in the Chrome Web Store."]
        #[serde(
            rename = "isCwsHosted",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_cws_hosted: ::std::option::Option<bool>,
        #[doc = "Output only. Whether the app is only for Kiosk mode on ChromeOS devices"]
        #[serde(
            rename = "isKioskOnly",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_kiosk_only: ::std::option::Option<bool>,
        #[doc = "Output only. Whether the app or extension is a theme."]
        #[serde(
            rename = "isTheme",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_theme: ::std::option::Option<bool>,
        #[doc = "Output only. Whether this app is enabled for Kiosk mode on ChromeOS devices"]
        #[serde(
            rename = "kioskEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kiosk_enabled: ::std::option::Option<bool>,
        #[doc = "Output only. The minimum number of users using this app."]
        #[serde(
            rename = "minUserCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub min_user_count: ::std::option::Option<i32>,
        #[doc = "Output only. Every custom permission requested by the app. Version-specific field that will only be set when the requested app version is found."]
        #[serde(
            rename = "permissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permissions:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1ChromeAppPermission>>,
        #[doc = "Output only. Every permission giving access to domains or broad host patterns. ( e.g. www.google.com). This includes the matches from content scripts as well as hosts in the permissions node of the manifest. Version-specific field that will only be set when the requested app version is found."]
        #[serde(
            rename = "siteAccess",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub site_access:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1ChromeAppSiteAccess>>,
        #[doc = "Output only. The app developer has enabled support for their app. Version-specific field that will only be set when the requested app version is found."]
        #[serde(
            rename = "supportEnabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub support_enabled: ::std::option::Option<bool>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1ChromeAppInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1ChromeAppInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1ChromeAppPermission {
        #[doc = "Output only. If available, whether this permissions grants the app/extension access to user data."]
        #[serde(
            rename = "accessUserData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub access_user_data: ::std::option::Option<bool>,
        #[doc = "Output only. If available, a URI to a page that has documentation for the current permission."]
        #[serde(
            rename = "documentationUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub documentation_uri: ::std::option::Option<String>,
        #[doc = "Output only. The type of the permission."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1ChromeAppPermission {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1ChromeAppPermission {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1ChromeAppRequest {
        #[doc = "Output only. Format: app_details=customers/{customer_id}/apps/chrome/{app_id}"]
        #[serde(
            rename = "appDetails",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_details: ::std::option::Option<String>,
        #[doc = "Output only. Unique store identifier for the app. Example: \"gmbmikajjgmnabiglmofipeabaddhgne\" for the Save to Google Drive Chrome extension."]
        #[serde(
            rename = "appId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_id: ::std::option::Option<String>,
        #[doc = "Output only. The uri for the detail page of the item."]
        #[serde(
            rename = "detailUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub detail_uri: ::std::option::Option<String>,
        #[doc = "Output only. App's display name."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Output only. A link to an image that can be used as an icon for the product."]
        #[serde(
            rename = "iconUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub icon_uri: ::std::option::Option<String>,
        #[doc = "Output only. The timestamp of the most recently made request for this app."]
        #[serde(
            rename = "latestRequestTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub latest_request_time: ::std::option::Option<String>,
        #[doc = "Output only. Total count of requests for this app."]
        #[serde(
            rename = "requestCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub request_count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1ChromeAppRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1ChromeAppRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1ChromeAppSiteAccess {
        #[doc = "Output only. This can contain very specific hosts, or patterns like \"*.com\" for instance."]
        #[serde(
            rename = "hostMatch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub host_match: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1ChromeAppSiteAccess {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1ChromeAppSiteAccess {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1CountChromeAppRequestsResponse {
        #[doc = "Token to specify the next page in the list."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Count of requested apps matching request."]
        #[serde(
            rename = "requestedApps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub requested_apps:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1ChromeAppRequest>>,
        #[doc = "Total number of matching app requests."]
        #[serde(
            rename = "totalSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_size: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1CountChromeAppRequestsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleChromeManagementV1CountChromeAppRequestsResponse
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
    pub struct GoogleChromeManagementV1CountChromeVersionsResponse {
        #[doc = "List of all browser versions and their install counts."]
        #[serde(
            rename = "browserVersions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub browser_versions:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1BrowserVersion>>,
        #[doc = "Token to specify the next page of the request."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Total number browser versions matching request."]
        #[serde(
            rename = "totalSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_size: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1CountChromeVersionsResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1CountChromeVersionsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1CountInstalledAppsResponse {
        #[doc = "List of installed apps matching request."]
        #[serde(
            rename = "installedApps",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub installed_apps:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1InstalledApp>>,
        #[doc = "Token to specify the next page of the request."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Total number of installed apps matching request."]
        #[serde(
            rename = "totalSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_size: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1CountInstalledAppsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1CountInstalledAppsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1CpuInfo {
        #[doc = "Output only. The CPU architecture."]
        #[serde(
            rename = "architecture",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub architecture:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1CpuInfoArchitecture>,
        #[doc = "Output only. The max CPU clock speed in kHz."]
        #[serde(
            rename = "maxClockSpeed",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub max_clock_speed: ::std::option::Option<i32>,
        #[doc = "Output only. The CPU model name. Example: Intel(R) Core(TM) i5-8250U CPU @ 1.60GHz"]
        #[serde(
            rename = "model",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1CpuInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1CpuInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1CpuInfoArchitecture {
        #[doc = "Architecture unknown."]
        ArchitectureUnspecified,
        #[doc = "x64 architecture"]
        X64,
    }
    impl GoogleChromeManagementV1CpuInfoArchitecture {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleChromeManagementV1CpuInfoArchitecture::ArchitectureUnspecified => {
                    "ARCHITECTURE_UNSPECIFIED"
                }
                GoogleChromeManagementV1CpuInfoArchitecture::X64 => "X64",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1CpuInfoArchitecture {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1CpuInfoArchitecture {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1CpuInfoArchitecture, ()> {
            Ok(match s {
                "ARCHITECTURE_UNSPECIFIED" => {
                    GoogleChromeManagementV1CpuInfoArchitecture::ArchitectureUnspecified
                }
                "X64" => GoogleChromeManagementV1CpuInfoArchitecture::X64,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1CpuInfoArchitecture {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1CpuInfoArchitecture {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1CpuInfoArchitecture {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ARCHITECTURE_UNSPECIFIED" => {
                    GoogleChromeManagementV1CpuInfoArchitecture::ArchitectureUnspecified
                }
                "X64" => GoogleChromeManagementV1CpuInfoArchitecture::X64,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1CpuInfoArchitecture {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1CpuInfoArchitecture {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1CpuStatusReport {
        #[doc = "Output only. CPU temperature sample info per CPU core in Celsius"]
        #[serde(
            rename = "cpuTemperatureInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpu_temperature_info:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1CpuTemperatureInfo>>,
        #[doc = "Output only. Sample of CPU utilization (0-100 percent)."]
        #[serde(
            rename = "cpuUtilizationPct",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpu_utilization_pct: ::std::option::Option<i32>,
        #[doc = "Output only. The timestamp in milliseconds representing time at which this report was sampled."]
        #[serde(
            rename = "reportTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_time: ::std::option::Option<String>,
        #[doc = "Output only. Frequency the report is sampled."]
        #[serde(
            rename = "sampleFrequency",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sample_frequency: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1CpuStatusReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1CpuStatusReport {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1CpuTemperatureInfo {
        #[doc = "Output only. CPU label. Example: Core 0"]
        #[serde(
            rename = "label",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label: ::std::option::Option<String>,
        #[doc = "Output only. CPU temperature in Celsius."]
        #[serde(
            rename = "temperatureCelsius",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub temperature_celsius: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1CpuTemperatureInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1CpuTemperatureInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1Device {
        #[doc = "Output only. The ID of the device that reported this Chrome browser information."]
        #[serde(
            rename = "deviceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_id: ::std::option::Option<String>,
        #[doc = "Output only. The name of the machine within its local network."]
        #[serde(
            rename = "machine",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub machine: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1Device {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1Device {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1DiskInfo {
        #[doc = "Output only. Number of bytes read since last boot."]
        #[serde(
            rename = "bytesReadThisSession",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub bytes_read_this_session: ::std::option::Option<i64>,
        #[doc = "Output only. Number of bytes written since last boot."]
        #[serde(
            rename = "bytesWrittenThisSession",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub bytes_written_this_session: ::std::option::Option<i64>,
        #[doc = "Output only. Time spent discarding since last boot. Discarding is writing to clear blocks which are no longer in use. Supported on kernels 4.18+."]
        #[serde(
            rename = "discardTimeThisSession",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub discard_time_this_session: ::std::option::Option<String>,
        #[doc = "Output only. Disk health."]
        #[serde(
            rename = "health",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub health: ::std::option::Option<String>,
        #[doc = "Output only. Counts the time the disk and queue were busy, so unlike the fields above, parallel requests are not counted multiple times."]
        #[serde(
            rename = "ioTimeThisSession",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub io_time_this_session: ::std::option::Option<String>,
        #[doc = "Output only. Disk manufacturer."]
        #[serde(
            rename = "manufacturer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub manufacturer: ::std::option::Option<String>,
        #[doc = "Output only. Disk model."]
        #[serde(
            rename = "model",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub model: ::std::option::Option<String>,
        #[doc = "Output only. Disk type: eMMC / NVMe / ATA / SCSI."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "Output only. Time spent reading from disk since last boot."]
        #[serde(
            rename = "readTimeThisSession",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub read_time_this_session: ::std::option::Option<String>,
        #[doc = "Output only. Disk serial number."]
        #[serde(
            rename = "serialNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub serial_number: ::std::option::Option<String>,
        #[doc = "Output only. Disk size."]
        #[serde(
            rename = "sizeBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub size_bytes: ::std::option::Option<i64>,
        #[doc = "Output only. Disk volumes."]
        #[serde(
            rename = "volumeIds",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub volume_ids: ::std::option::Option<Vec<String>>,
        #[doc = "Output only. Time spent writing to disk since last boot."]
        #[serde(
            rename = "writeTimeThisSession",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub write_time_this_session: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1DiskInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1DiskInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1DisplayInfo {
        #[doc = "Output only. Represents the graphics card device id."]
        #[serde(
            rename = "deviceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub device_id: ::std::option::Option<i64>,
        #[doc = "Output only. Indicates if display is internal or not."]
        #[serde(
            rename = "isInternal",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub is_internal: ::std::option::Option<bool>,
        #[doc = "Output only. Refresh rate in Hz."]
        #[serde(
            rename = "refreshRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub refresh_rate: ::std::option::Option<i32>,
        #[doc = "Output only. Resolution height in pixels."]
        #[serde(
            rename = "resolutionHeight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resolution_height: ::std::option::Option<i32>,
        #[doc = "Output only. Resolution width in pixels."]
        #[serde(
            rename = "resolutionWidth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resolution_width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1DisplayInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1DisplayInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1FindInstalledAppDevicesResponse {
        #[doc = "A list of devices which have the app installed. Sorted in ascending alphabetical order on the Device.machine field."]
        #[serde(
            rename = "devices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub devices: ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1Device>>,
        #[doc = "Token to specify the next page of the request."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Total number of devices matching request."]
        #[serde(
            rename = "totalSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_size: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1FindInstalledAppDevicesResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for GoogleChromeManagementV1FindInstalledAppDevicesResponse
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
    pub struct GoogleChromeManagementV1GraphicsAdapterInfo {
        #[doc = "Output only. Adapter name. Example: Mesa DRI Intel(R) UHD Graphics 620 (Kabylake GT2)."]
        #[serde(
            rename = "adapter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub adapter: ::std::option::Option<String>,
        #[doc = "Output only. Represents the graphics card device id."]
        #[serde(
            rename = "deviceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub device_id: ::std::option::Option<i64>,
        #[doc = "Output only. Version of the GPU driver."]
        #[serde(
            rename = "driverVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub driver_version: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1GraphicsAdapterInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1GraphicsAdapterInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1GraphicsInfo {
        #[doc = "Output only. Information about the graphics adapter (GPU)."]
        #[serde(
            rename = "adapterInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub adapter_info:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1GraphicsAdapterInfo>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1GraphicsInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1GraphicsInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1GraphicsStatusReport {
        #[doc = "Output only. Information about the displays for the device."]
        #[serde(
            rename = "displays",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub displays:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1DisplayInfo>>,
        #[doc = "Output only. Time at which the graphics data was reported."]
        #[serde(
            rename = "reportTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1GraphicsStatusReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1GraphicsStatusReport {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1InstalledApp {
        #[doc = "Output only. Unique identifier of the app. For Chrome apps and extensions, the 32-character id (e.g. ehoadneljpdggcbbknedodolkkjodefl). For Android apps, the package name (e.g. com.evernote)."]
        #[serde(
            rename = "appId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_id: ::std::option::Option<String>,
        #[doc = "Output only. How the app was installed."]
        #[serde(
            rename = "appInstallType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_install_type: ::std::option::Option<
            crate::schemas::GoogleChromeManagementV1InstalledAppAppInstallType,
        >,
        #[doc = "Output only. Source of the installed app."]
        #[serde(
            rename = "appSource",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_source:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1InstalledAppAppSource>,
        #[doc = "Output only. Type of the app."]
        #[serde(
            rename = "appType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub app_type:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1InstalledAppAppType>,
        #[doc = "Output only. Count of browser devices with this app installed."]
        #[serde(
            rename = "browserDeviceCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub browser_device_count: ::std::option::Option<i64>,
        #[doc = "Output only. Description of the installed app."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "Output only. Whether the app is disabled."]
        #[serde(
            rename = "disabled",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disabled: ::std::option::Option<bool>,
        #[doc = "Output only. Name of the installed app."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Output only. Homepage uri of the installed app."]
        #[serde(
            rename = "homepageUri",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub homepage_uri: ::std::option::Option<String>,
        #[doc = "Output only. Count of ChromeOS users with this app installed."]
        #[serde(
            rename = "osUserCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub os_user_count: ::std::option::Option<i64>,
        #[doc = "Output only. Permissions of the installed app."]
        #[serde(
            rename = "permissions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub permissions: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1InstalledApp {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1InstalledApp {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1InstalledAppAppInstallType {
        #[doc = "Administrator app install type."]
        Admin,
        #[doc = "Application install type not specified."]
        AppInstallTypeUnspecified,
        #[doc = "Development app install type."]
        Development,
        #[doc = "Multiple app install types."]
        Multiple,
        #[doc = "Normal app install type."]
        Normal,
        #[doc = "Other app install type."]
        Other,
        #[doc = "Sideloaded app install type."]
        Sideload,
    }
    impl GoogleChromeManagementV1InstalledAppAppInstallType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleChromeManagementV1InstalledAppAppInstallType::Admin => "ADMIN",
                GoogleChromeManagementV1InstalledAppAppInstallType::AppInstallTypeUnspecified => {
                    "APP_INSTALL_TYPE_UNSPECIFIED"
                }
                GoogleChromeManagementV1InstalledAppAppInstallType::Development => "DEVELOPMENT",
                GoogleChromeManagementV1InstalledAppAppInstallType::Multiple => "MULTIPLE",
                GoogleChromeManagementV1InstalledAppAppInstallType::Normal => "NORMAL",
                GoogleChromeManagementV1InstalledAppAppInstallType::Other => "OTHER",
                GoogleChromeManagementV1InstalledAppAppInstallType::Sideload => "SIDELOAD",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1InstalledAppAppInstallType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1InstalledAppAppInstallType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1InstalledAppAppInstallType, ()> {
            Ok(match s {
                "ADMIN" => GoogleChromeManagementV1InstalledAppAppInstallType::Admin,
                "APP_INSTALL_TYPE_UNSPECIFIED" => {
                    GoogleChromeManagementV1InstalledAppAppInstallType::AppInstallTypeUnspecified
                }
                "DEVELOPMENT" => GoogleChromeManagementV1InstalledAppAppInstallType::Development,
                "MULTIPLE" => GoogleChromeManagementV1InstalledAppAppInstallType::Multiple,
                "NORMAL" => GoogleChromeManagementV1InstalledAppAppInstallType::Normal,
                "OTHER" => GoogleChromeManagementV1InstalledAppAppInstallType::Other,
                "SIDELOAD" => GoogleChromeManagementV1InstalledAppAppInstallType::Sideload,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1InstalledAppAppInstallType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1InstalledAppAppInstallType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1InstalledAppAppInstallType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ADMIN" => GoogleChromeManagementV1InstalledAppAppInstallType::Admin,
                "APP_INSTALL_TYPE_UNSPECIFIED" => {
                    GoogleChromeManagementV1InstalledAppAppInstallType::AppInstallTypeUnspecified
                }
                "DEVELOPMENT" => GoogleChromeManagementV1InstalledAppAppInstallType::Development,
                "MULTIPLE" => GoogleChromeManagementV1InstalledAppAppInstallType::Multiple,
                "NORMAL" => GoogleChromeManagementV1InstalledAppAppInstallType::Normal,
                "OTHER" => GoogleChromeManagementV1InstalledAppAppInstallType::Other,
                "SIDELOAD" => GoogleChromeManagementV1InstalledAppAppInstallType::Sideload,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1InstalledAppAppInstallType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1InstalledAppAppInstallType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1InstalledAppAppSource {
        #[doc = "Application source not specified."]
        AppSourceUnspecified,
        #[doc = "Generally for extensions and Chrome apps."]
        ChromeWebstore,
        #[doc = "Play Store app."]
        PlayStore,
    }
    impl GoogleChromeManagementV1InstalledAppAppSource {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleChromeManagementV1InstalledAppAppSource::AppSourceUnspecified => {
                    "APP_SOURCE_UNSPECIFIED"
                }
                GoogleChromeManagementV1InstalledAppAppSource::ChromeWebstore => "CHROME_WEBSTORE",
                GoogleChromeManagementV1InstalledAppAppSource::PlayStore => "PLAY_STORE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1InstalledAppAppSource {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1InstalledAppAppSource {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1InstalledAppAppSource, ()> {
            Ok(match s {
                "APP_SOURCE_UNSPECIFIED" => {
                    GoogleChromeManagementV1InstalledAppAppSource::AppSourceUnspecified
                }
                "CHROME_WEBSTORE" => GoogleChromeManagementV1InstalledAppAppSource::ChromeWebstore,
                "PLAY_STORE" => GoogleChromeManagementV1InstalledAppAppSource::PlayStore,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1InstalledAppAppSource {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1InstalledAppAppSource {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1InstalledAppAppSource {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "APP_SOURCE_UNSPECIFIED" => {
                    GoogleChromeManagementV1InstalledAppAppSource::AppSourceUnspecified
                }
                "CHROME_WEBSTORE" => GoogleChromeManagementV1InstalledAppAppSource::ChromeWebstore,
                "PLAY_STORE" => GoogleChromeManagementV1InstalledAppAppSource::PlayStore,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1InstalledAppAppSource {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1InstalledAppAppSource {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1InstalledAppAppType {
        #[doc = "ARC++ app."]
        AndroidApp,
        #[doc = "Chrome app."]
        App,
        #[doc = "App type not specified."]
        AppTypeUnspecified,
        #[doc = "Chrome extension."]
        Extension,
        #[doc = "Chrome hosted app."]
        HostedApp,
        #[doc = "Chrome theme."]
        Theme,
    }
    impl GoogleChromeManagementV1InstalledAppAppType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleChromeManagementV1InstalledAppAppType::AndroidApp => "ANDROID_APP",
                GoogleChromeManagementV1InstalledAppAppType::App => "APP",
                GoogleChromeManagementV1InstalledAppAppType::AppTypeUnspecified => {
                    "APP_TYPE_UNSPECIFIED"
                }
                GoogleChromeManagementV1InstalledAppAppType::Extension => "EXTENSION",
                GoogleChromeManagementV1InstalledAppAppType::HostedApp => "HOSTED_APP",
                GoogleChromeManagementV1InstalledAppAppType::Theme => "THEME",
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1InstalledAppAppType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1InstalledAppAppType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1InstalledAppAppType, ()> {
            Ok(match s {
                "ANDROID_APP" => GoogleChromeManagementV1InstalledAppAppType::AndroidApp,
                "APP" => GoogleChromeManagementV1InstalledAppAppType::App,
                "APP_TYPE_UNSPECIFIED" => {
                    GoogleChromeManagementV1InstalledAppAppType::AppTypeUnspecified
                }
                "EXTENSION" => GoogleChromeManagementV1InstalledAppAppType::Extension,
                "HOSTED_APP" => GoogleChromeManagementV1InstalledAppAppType::HostedApp,
                "THEME" => GoogleChromeManagementV1InstalledAppAppType::Theme,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1InstalledAppAppType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1InstalledAppAppType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1InstalledAppAppType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANDROID_APP" => GoogleChromeManagementV1InstalledAppAppType::AndroidApp,
                "APP" => GoogleChromeManagementV1InstalledAppAppType::App,
                "APP_TYPE_UNSPECIFIED" => {
                    GoogleChromeManagementV1InstalledAppAppType::AppTypeUnspecified
                }
                "EXTENSION" => GoogleChromeManagementV1InstalledAppAppType::Extension,
                "HOSTED_APP" => GoogleChromeManagementV1InstalledAppAppType::HostedApp,
                "THEME" => GoogleChromeManagementV1InstalledAppAppType::Theme,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1InstalledAppAppType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1InstalledAppAppType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1ListTelemetryDevicesResponse {
        #[doc = "Telemetry devices returned in the response."]
        #[serde(
            rename = "devices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub devices:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1TelemetryDevice>>,
        #[doc = "Token to specify next page in the list."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector
        for GoogleChromeManagementV1ListTelemetryDevicesResponse
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1ListTelemetryDevicesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1MemoryInfo {
        #[doc = "Output only. Amount of available RAM in bytes."]
        #[serde(
            rename = "availableRamBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub available_ram_bytes: ::std::option::Option<i64>,
        #[doc = "Output only. Total RAM in bytes."]
        #[serde(
            rename = "totalRamBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_ram_bytes: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1MemoryInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1MemoryInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1MemoryStatusReport {
        #[doc = "Output only. Number of page faults during this collection"]
        #[serde(
            rename = "pageFaults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub page_faults: ::std::option::Option<i32>,
        #[doc = "Output only. The timestamp in milliseconds representing time at which this report was sampled."]
        #[serde(
            rename = "reportTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_time: ::std::option::Option<String>,
        #[doc = "Output only. Frequency the report is sampled."]
        #[serde(
            rename = "sampleFrequency",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sample_frequency: ::std::option::Option<String>,
        #[doc = "Output only. Amount of free RAM in bytes (unreliable due to Garbage Collection)."]
        #[serde(
            rename = "systemRamFreeBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub system_ram_free_bytes: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1MemoryStatusReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1MemoryStatusReport {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1NetworkStatusReport {
        #[doc = "Output only. Gateway IP address."]
        #[serde(
            rename = "gatewayIpAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gateway_ip_address: ::std::option::Option<String>,
        #[doc = "Output only. LAN IP address."]
        #[serde(
            rename = "lanIpAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub lan_ip_address: ::std::option::Option<String>,
        #[doc = "Output only. Time at which the network state was reported."]
        #[serde(
            rename = "reportTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_time: ::std::option::Option<String>,
        #[doc = "Output only. Frequency the report is sampled."]
        #[serde(
            rename = "sampleFrequency",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sample_frequency: ::std::option::Option<String>,
        #[doc = "Output only. Signal strength for wireless networks measured in decibels."]
        #[serde(
            rename = "signalStrengthDbm",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub signal_strength_dbm: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1NetworkStatusReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1NetworkStatusReport {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1OsUpdateStatus {
        #[doc = "Output only. Timestamp of the last reboot."]
        #[serde(
            rename = "lastRebootTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_reboot_time: ::std::option::Option<String>,
        #[doc = "Output only. Timestamp of the last update check."]
        #[serde(
            rename = "lastUpdateCheckTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_update_check_time: ::std::option::Option<String>,
        #[doc = "Output only. Timestamp of the last successful update."]
        #[serde(
            rename = "lastUpdateTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub last_update_time: ::std::option::Option<String>,
        #[doc = "Output only. New platform version of the os image being downloaded and applied. It is only set when update status is OS_IMAGE_DOWNLOAD_IN_PROGRESS or OS_UPDATE_NEED_REBOOT. Note this could be a dummy \"0.0.0.0\" for OS_UPDATE_NEED_REBOOT status for some edge cases, e.g. update engine is restarted without a reboot."]
        #[serde(
            rename = "newPlatformVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_platform_version: ::std::option::Option<String>,
        #[doc = "Output only. New requested platform version from the pending updated kiosk app."]
        #[serde(
            rename = "newRequestedPlatformVersion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub new_requested_platform_version: ::std::option::Option<String>,
        #[doc = "Output only. Current state of the os update."]
        #[serde(
            rename = "updateState",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub update_state: ::std::option::Option<
            crate::schemas::GoogleChromeManagementV1OsUpdateStatusUpdateState,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1OsUpdateStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1OsUpdateStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleChromeManagementV1OsUpdateStatusUpdateState {
        #[doc = "OS has started download on device."]
        OsImageDownloadInProgress,
        #[doc = "OS has not started downloading."]
        OsImageDownloadNotStarted,
        #[doc = "Device needs reboot to finish upload."]
        OsUpdateNeedReboot,
        #[doc = "State unspecified."]
        UpdateStateUnspecified,
    }
    impl GoogleChromeManagementV1OsUpdateStatusUpdateState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleChromeManagementV1OsUpdateStatusUpdateState::OsImageDownloadInProgress => {
                    "OS_IMAGE_DOWNLOAD_IN_PROGRESS"
                }
                GoogleChromeManagementV1OsUpdateStatusUpdateState::OsImageDownloadNotStarted => {
                    "OS_IMAGE_DOWNLOAD_NOT_STARTED"
                }
                GoogleChromeManagementV1OsUpdateStatusUpdateState::OsUpdateNeedReboot => {
                    "OS_UPDATE_NEED_REBOOT"
                }
                GoogleChromeManagementV1OsUpdateStatusUpdateState::UpdateStateUnspecified => {
                    "UPDATE_STATE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for GoogleChromeManagementV1OsUpdateStatusUpdateState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for GoogleChromeManagementV1OsUpdateStatusUpdateState {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<GoogleChromeManagementV1OsUpdateStatusUpdateState, ()> {
            Ok(match s {
                "OS_IMAGE_DOWNLOAD_IN_PROGRESS" => {
                    GoogleChromeManagementV1OsUpdateStatusUpdateState::OsImageDownloadInProgress
                }
                "OS_IMAGE_DOWNLOAD_NOT_STARTED" => {
                    GoogleChromeManagementV1OsUpdateStatusUpdateState::OsImageDownloadNotStarted
                }
                "OS_UPDATE_NEED_REBOOT" => {
                    GoogleChromeManagementV1OsUpdateStatusUpdateState::OsUpdateNeedReboot
                }
                "UPDATE_STATE_UNSPECIFIED" => {
                    GoogleChromeManagementV1OsUpdateStatusUpdateState::UpdateStateUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for GoogleChromeManagementV1OsUpdateStatusUpdateState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleChromeManagementV1OsUpdateStatusUpdateState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleChromeManagementV1OsUpdateStatusUpdateState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "OS_IMAGE_DOWNLOAD_IN_PROGRESS" => {
                    GoogleChromeManagementV1OsUpdateStatusUpdateState::OsImageDownloadInProgress
                }
                "OS_IMAGE_DOWNLOAD_NOT_STARTED" => {
                    GoogleChromeManagementV1OsUpdateStatusUpdateState::OsImageDownloadNotStarted
                }
                "OS_UPDATE_NEED_REBOOT" => {
                    GoogleChromeManagementV1OsUpdateStatusUpdateState::OsUpdateNeedReboot
                }
                "UPDATE_STATE_UNSPECIFIED" => {
                    GoogleChromeManagementV1OsUpdateStatusUpdateState::UpdateStateUnspecified
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
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1OsUpdateStatusUpdateState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1OsUpdateStatusUpdateState {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1StorageInfo {
        #[doc = "The available space for user data storage in the device in bytes."]
        #[serde(
            rename = "availableDiskBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub available_disk_bytes: ::std::option::Option<i64>,
        #[doc = "The total space for user data storage in the device in bytes."]
        #[serde(
            rename = "totalDiskBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_disk_bytes: ::std::option::Option<i64>,
        #[doc = "Information for disk volumes"]
        #[serde(
            rename = "volume",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub volume: ::std::option::Option<
            Vec<crate::schemas::GoogleChromeManagementV1StorageInfoDiskVolume>,
        >,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1StorageInfo {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1StorageInfo {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1StorageInfoDiskVolume {
        #[doc = "Free storage space in bytes."]
        #[serde(
            rename = "storageFreeBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub storage_free_bytes: ::std::option::Option<i64>,
        #[doc = "Total storage space in bytes."]
        #[serde(
            rename = "storageTotalBytes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub storage_total_bytes: ::std::option::Option<i64>,
        #[doc = "Disk volume id."]
        #[serde(
            rename = "volumeId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub volume_id: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1StorageInfoDiskVolume {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1StorageInfoDiskVolume {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1StorageStatusReport {
        #[doc = "Output only. Reports on disk"]
        #[serde(
            rename = "disk",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disk: ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1DiskInfo>>,
        #[doc = "Output only. Timestamp of when the sample was collected on device"]
        #[serde(
            rename = "reportTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_time: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1StorageStatusReport {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1StorageStatusReport {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleChromeManagementV1TelemetryDevice {
        #[doc = "Output only. Audio reports collected periodically sorted in a decreasing order of report_time."]
        #[serde(
            rename = "audioStatusReport",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub audio_status_report:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1AudioStatusReport>>,
        #[doc = "Output only. Information on battery specs for the device."]
        #[serde(
            rename = "batteryInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub battery_info:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1BatteryInfo>>,
        #[doc = "Output only. Battery reports collected periodically."]
        #[serde(
            rename = "batteryStatusReport",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub battery_status_report:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1BatteryStatusReport>>,
        #[doc = "Output only. Information regarding CPU specs for the device."]
        #[serde(
            rename = "cpuInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpu_info: ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1CpuInfo>>,
        #[doc = "Output only. CPU status reports collected periodically sorted in a decreasing order of report_time."]
        #[serde(
            rename = "cpuStatusReport",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cpu_status_report:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1CpuStatusReport>>,
        #[doc = "Output only. Google Workspace Customer whose enterprise enrolled the device."]
        #[serde(
            rename = "customer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub customer: ::std::option::Option<String>,
        #[doc = "Output only. The unique Directory API ID of the device. This value is the same as the Admin Console's Directory API ID in the ChromeOS Devices tab"]
        #[serde(
            rename = "deviceId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub device_id: ::std::option::Option<String>,
        #[doc = "Output only. Contains information regarding Graphic peripherals for the device."]
        #[serde(
            rename = "graphicsInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub graphics_info:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1GraphicsInfo>,
        #[doc = "Output only. Graphics reports collected periodically."]
        #[serde(
            rename = "graphicsStatusReport",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub graphics_status_report: ::std::option::Option<
            Vec<crate::schemas::GoogleChromeManagementV1GraphicsStatusReport>,
        >,
        #[doc = "Output only. Information regarding memory specs for the device."]
        #[serde(
            rename = "memoryInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub memory_info: ::std::option::Option<crate::schemas::GoogleChromeManagementV1MemoryInfo>,
        #[doc = "Output only. Memory status reports collected periodically sorted decreasing by report_time."]
        #[serde(
            rename = "memoryStatusReport",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub memory_status_report:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1MemoryStatusReport>>,
        #[doc = "Output only. Resource name of the device."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. Network specs collected periodically."]
        #[serde(
            rename = "networkStatusReport",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub network_status_report:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1NetworkStatusReport>>,
        #[doc = "Output only. Organization unit ID of the device."]
        #[serde(
            rename = "orgUnitId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub org_unit_id: ::std::option::Option<String>,
        #[doc = "Output only. Contains relevant information regarding ChromeOS update status."]
        #[serde(
            rename = "osUpdateStatus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub os_update_status:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1OsUpdateStatus>>,
        #[doc = "Output only. Device serial number. This value is the same as the Admin Console's Serial Number in the ChromeOS Devices tab."]
        #[serde(
            rename = "serialNumber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub serial_number: ::std::option::Option<String>,
        #[doc = "Output only. Information of storage specs for the device."]
        #[serde(
            rename = "storageInfo",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub storage_info:
            ::std::option::Option<crate::schemas::GoogleChromeManagementV1StorageInfo>,
        #[doc = "Output only. Storage reports collected periodically."]
        #[serde(
            rename = "storageStatusReport",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub storage_status_report:
            ::std::option::Option<Vec<crate::schemas::GoogleChromeManagementV1StorageStatusReport>>,
    }
    impl ::google_field_selector::FieldSelector for GoogleChromeManagementV1TelemetryDevice {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleChromeManagementV1TelemetryDevice {
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
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoogleTypeDate {
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
    impl ::google_field_selector::FieldSelector for GoogleTypeDate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GoogleTypeDate {
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
    #[doc = "Actions that can be performed on the customers resource"]
    pub fn customers(&self) -> crate::resources::customers::CustomersActions {
        crate::resources::customers::CustomersActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod customers {
        pub mod params {}
        pub struct CustomersActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> CustomersActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the apps resource"]
            pub fn apps(&self) -> crate::resources::customers::apps::AppsActions {
                crate::resources::customers::apps::AppsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the reports resource"]
            pub fn reports(&self) -> crate::resources::customers::reports::ReportsActions {
                crate::resources::customers::reports::ReportsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
            #[doc = "Actions that can be performed on the telemetry resource"]
            pub fn telemetry(&self) -> crate::resources::customers::telemetry::TelemetryActions {
                crate::resources::customers::telemetry::TelemetryActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
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
                #[doc = "Generate summary of app installation requests."]
                pub fn count_chrome_app_requests(
                    &self,
                    customer: impl Into<String>,
                ) -> CountChromeAppRequestsRequestBuilder {
                    CountChromeAppRequestsRequestBuilder {
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
                        customer: customer.into(),
                        order_by: None,
                        org_unit_id: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Actions that can be performed on the android resource"]
                pub fn android(
                    &self,
                ) -> crate::resources::customers::apps::android::AndroidActions {
                    crate::resources::customers::apps::android::AndroidActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
                #[doc = "Actions that can be performed on the chrome resource"]
                pub fn chrome(&self) -> crate::resources::customers::apps::chrome::ChromeActions {
                    crate::resources::customers::apps::chrome::ChromeActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
                #[doc = "Actions that can be performed on the web resource"]
                pub fn web(&self) -> crate::resources::customers::apps::web::WebActions {
                    crate::resources::customers::apps::web::WebActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            #[doc = "Created via [AppsActions::count_chrome_app_requests()](struct.AppsActions.html#method.count_chrome_app_requests)"]
            #[derive(Debug, Clone)]
            pub struct CountChromeAppRequestsRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                customer: String,
                order_by: Option<String>,
                org_unit_id: Option<String>,
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
            impl<'a> CountChromeAppRequestsRequestBuilder<'a> {
                #[doc = "Field used to order results. Supported fields: * request_count * latest_request_time"]
                pub fn order_by(mut self, value: impl Into<String>) -> Self {
                    self.order_by = Some(value.into());
                    self
                }
                #[doc = "The ID of the organizational unit."]
                pub fn org_unit_id(mut self, value: impl Into<String>) -> Self {
                    self.org_unit_id = Some(value.into());
                    self
                }
                #[doc = "Maximum number of results to return. Maximum and default are 50, anything above will be coerced to 50."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Token to specify the page of the request to be returned."]
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
                    crate::schemas::GoogleChromeManagementV1CountChromeAppRequestsResponse,
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
                    crate::schemas::GoogleChromeManagementV1CountChromeAppRequestsResponse,
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
                    let mut output = "https://chromemanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.customer;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/apps:countChromeAppRequests");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("orderBy", &self.order_by)]);
                    req = req.query(&[("orgUnitId", &self.org_unit_id)]);
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
            pub mod android {
                pub mod params {}
                pub struct AndroidActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> AndroidActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Get a specific app for a customer by its resource name."]
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
                }
                #[doc = "Created via [AndroidActions::get()](struct.AndroidActions.html#method.get)"]
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
                    ) -> Result<crate::schemas::GoogleChromeManagementV1AppDetails, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleChromeManagementV1AppDetails, crate::Error>
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
                        let mut output = "https://chromemanagement.googleapis.com/".to_owned();
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
            }
            pub mod chrome {
                pub mod params {}
                pub struct ChromeActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> ChromeActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Get a specific app for a customer by its resource name."]
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
                }
                #[doc = "Created via [ChromeActions::get()](struct.ChromeActions.html#method.get)"]
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
                    ) -> Result<crate::schemas::GoogleChromeManagementV1AppDetails, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleChromeManagementV1AppDetails, crate::Error>
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
                        let mut output = "https://chromemanagement.googleapis.com/".to_owned();
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
            }
            pub mod web {
                pub mod params {}
                pub struct WebActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> WebActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "Get a specific app for a customer by its resource name."]
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
                }
                #[doc = "Created via [WebActions::get()](struct.WebActions.html#method.get)"]
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
                    ) -> Result<crate::schemas::GoogleChromeManagementV1AppDetails, crate::Error>
                    {
                        self.execute_with_fields(None::<&str>).await
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub async fn execute_with_all_fields(
                        self,
                    ) -> Result<crate::schemas::GoogleChromeManagementV1AppDetails, crate::Error>
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
                        let mut output = "https://chromemanagement.googleapis.com/".to_owned();
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
            }
        }
        pub mod reports {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum FindInstalledAppDevicesAppType {
                    #[doc = "ARC++ app."]
                    AndroidApp,
                    #[doc = "Chrome app."]
                    App,
                    #[doc = "App type not specified."]
                    AppTypeUnspecified,
                    #[doc = "Chrome extension."]
                    Extension,
                    #[doc = "Chrome hosted app."]
                    HostedApp,
                    #[doc = "Chrome theme."]
                    Theme,
                }
                impl FindInstalledAppDevicesAppType {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            FindInstalledAppDevicesAppType::AndroidApp => "ANDROID_APP",
                            FindInstalledAppDevicesAppType::App => "APP",
                            FindInstalledAppDevicesAppType::AppTypeUnspecified => {
                                "APP_TYPE_UNSPECIFIED"
                            }
                            FindInstalledAppDevicesAppType::Extension => "EXTENSION",
                            FindInstalledAppDevicesAppType::HostedApp => "HOSTED_APP",
                            FindInstalledAppDevicesAppType::Theme => "THEME",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for FindInstalledAppDevicesAppType {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for FindInstalledAppDevicesAppType {
                    type Err = ();
                    fn from_str(
                        s: &str,
                    ) -> ::std::result::Result<FindInstalledAppDevicesAppType, ()>
                    {
                        Ok(match s {
                            "ANDROID_APP" => FindInstalledAppDevicesAppType::AndroidApp,
                            "APP" => FindInstalledAppDevicesAppType::App,
                            "APP_TYPE_UNSPECIFIED" => {
                                FindInstalledAppDevicesAppType::AppTypeUnspecified
                            }
                            "EXTENSION" => FindInstalledAppDevicesAppType::Extension,
                            "HOSTED_APP" => FindInstalledAppDevicesAppType::HostedApp,
                            "THEME" => FindInstalledAppDevicesAppType::Theme,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for FindInstalledAppDevicesAppType {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for FindInstalledAppDevicesAppType {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for FindInstalledAppDevicesAppType {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "ANDROID_APP" => FindInstalledAppDevicesAppType::AndroidApp,
                            "APP" => FindInstalledAppDevicesAppType::App,
                            "APP_TYPE_UNSPECIFIED" => {
                                FindInstalledAppDevicesAppType::AppTypeUnspecified
                            }
                            "EXTENSION" => FindInstalledAppDevicesAppType::Extension,
                            "HOSTED_APP" => FindInstalledAppDevicesAppType::HostedApp,
                            "THEME" => FindInstalledAppDevicesAppType::Theme,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for FindInstalledAppDevicesAppType {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for FindInstalledAppDevicesAppType {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct ReportsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> ReportsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Generate report of installed Chrome versions."]
                pub fn count_chrome_versions(
                    &self,
                    customer: impl Into<String>,
                ) -> CountChromeVersionsRequestBuilder {
                    CountChromeVersionsRequestBuilder {
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
                        customer: customer.into(),
                        filter: None,
                        org_unit_id: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Generate report of app installations."]
                pub fn count_installed_apps(
                    &self,
                    customer: impl Into<String>,
                ) -> CountInstalledAppsRequestBuilder {
                    CountInstalledAppsRequestBuilder {
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
                        customer: customer.into(),
                        filter: None,
                        order_by: None,
                        org_unit_id: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Generate report of devices that have a specified app installed."]
                pub fn find_installed_app_devices(
                    &self,
                    customer: impl Into<String>,
                ) -> FindInstalledAppDevicesRequestBuilder {
                    FindInstalledAppDevicesRequestBuilder {
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
                        customer: customer.into(),
                        app_id: None,
                        app_type: None,
                        filter: None,
                        order_by: None,
                        org_unit_id: None,
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[doc = "Created via [ReportsActions::count_chrome_versions()](struct.ReportsActions.html#method.count_chrome_versions)"]
            #[derive(Debug, Clone)]
            pub struct CountChromeVersionsRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                customer: String,
                filter: Option<String>,
                org_unit_id: Option<String>,
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
            impl<'a> CountChromeVersionsRequestBuilder<'a> {
                #[doc = "Query string to filter results, AND-separated fields in EBNF syntax. Note: OR operations are not supported in this filter. Supported filter fields: * last_active_date"]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "The ID of the organizational unit."]
                pub fn org_unit_id(mut self, value: impl Into<String>) -> Self {
                    self.org_unit_id = Some(value.into());
                    self
                }
                #[doc = "Maximum number of results to return. Maximum and default are 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Token to specify the page of the request to be returned."]
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
                    crate::schemas::GoogleChromeManagementV1CountChromeVersionsResponse,
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
                    crate::schemas::GoogleChromeManagementV1CountChromeVersionsResponse,
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
                    let mut output = "https://chromemanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.customer;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/reports:countChromeVersions");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("filter", &self.filter)]);
                    req = req.query(&[("orgUnitId", &self.org_unit_id)]);
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
            #[doc = "Created via [ReportsActions::count_installed_apps()](struct.ReportsActions.html#method.count_installed_apps)"]
            #[derive(Debug, Clone)]
            pub struct CountInstalledAppsRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                customer: String,
                filter: Option<String>,
                order_by: Option<String>,
                org_unit_id: Option<String>,
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
            impl<'a> CountInstalledAppsRequestBuilder<'a> {
                #[doc = "Query string to filter results, AND-separated fields in EBNF syntax. Note: OR operations are not supported in this filter. Supported filter fields: * app_name * app_type * install_type * number_of_permissions * total_install_count * latest_profile_active_date * permission_name"]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "Field used to order results. Supported order by fields: * app_name * app_type * install_type * number_of_permissions * total_install_count"]
                pub fn order_by(mut self, value: impl Into<String>) -> Self {
                    self.order_by = Some(value.into());
                    self
                }
                #[doc = "The ID of the organizational unit."]
                pub fn org_unit_id(mut self, value: impl Into<String>) -> Self {
                    self.org_unit_id = Some(value.into());
                    self
                }
                #[doc = "Maximum number of results to return. Maximum and default are 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Token to specify the page of the request to be returned."]
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
                    crate::schemas::GoogleChromeManagementV1CountInstalledAppsResponse,
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
                    crate::schemas::GoogleChromeManagementV1CountInstalledAppsResponse,
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
                    let mut output = "https://chromemanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.customer;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/reports:countInstalledApps");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("filter", &self.filter)]);
                    req = req.query(&[("orderBy", &self.order_by)]);
                    req = req.query(&[("orgUnitId", &self.org_unit_id)]);
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
            #[doc = "Created via [ReportsActions::find_installed_app_devices()](struct.ReportsActions.html#method.find_installed_app_devices)"]
            #[derive(Debug, Clone)]
            pub struct FindInstalledAppDevicesRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                customer: String,
                app_id: Option<String>,
                app_type: Option<
                    crate::resources::customers::reports::params::FindInstalledAppDevicesAppType,
                >,
                filter: Option<String>,
                order_by: Option<String>,
                org_unit_id: Option<String>,
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
            impl<'a> FindInstalledAppDevicesRequestBuilder<'a> {
                #[doc = "Unique identifier of the app. For Chrome apps and extensions, the 32-character id (e.g. ehoadneljpdggcbbknedodolkkjodefl). For Android apps, the package name (e.g. com.evernote)."]
                pub fn app_id(mut self, value: impl Into<String>) -> Self {
                    self.app_id = Some(value.into());
                    self
                }
                #[doc = "Type of the app."]
                pub fn app_type(
                    mut self,
                    value : crate :: resources :: customers :: reports :: params :: FindInstalledAppDevicesAppType,
                ) -> Self {
                    self.app_type = Some(value);
                    self
                }
                #[doc = "Query string to filter results, AND-separated fields in EBNF syntax. Note: OR operations are not supported in this filter. Supported filter fields: * last_active_date"]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "Field used to order results. Supported order by fields: * machine * device_id"]
                pub fn order_by(mut self, value: impl Into<String>) -> Self {
                    self.order_by = Some(value.into());
                    self
                }
                #[doc = "The ID of the organizational unit."]
                pub fn org_unit_id(mut self, value: impl Into<String>) -> Self {
                    self.org_unit_id = Some(value.into());
                    self
                }
                #[doc = "Maximum number of results to return. Maximum and default are 100."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Token to specify the page of the request to be returned."]
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
                    crate::schemas::GoogleChromeManagementV1FindInstalledAppDevicesResponse,
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
                    crate::schemas::GoogleChromeManagementV1FindInstalledAppDevicesResponse,
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
                    let mut output = "https://chromemanagement.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.customer;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/reports:findInstalledAppDevices");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("appId", &self.app_id)]);
                    req = req.query(&[("appType", &self.app_type)]);
                    req = req.query(&[("filter", &self.filter)]);
                    req = req.query(&[("orderBy", &self.order_by)]);
                    req = req.query(&[("orgUnitId", &self.org_unit_id)]);
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
        pub mod telemetry {
            pub mod params {}
            pub struct TelemetryActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> TelemetryActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Actions that can be performed on the devices resource"]
                pub fn devices(
                    &self,
                ) -> crate::resources::customers::telemetry::devices::DevicesActions
                {
                    crate::resources::customers::telemetry::devices::DevicesActions {
                        reqwest: &self.reqwest,
                        auth: self.auth_ref(),
                    }
                }
            }
            pub mod devices {
                pub mod params {}
                pub struct DevicesActions<'a> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                }
                impl<'a> DevicesActions<'a> {
                    fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                        self.auth
                    }
                    #[doc = "List all telemetry devices."]
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
                            read_mask: None,
                        }
                    }
                }
                #[doc = "Created via [DevicesActions::list()](struct.DevicesActions.html#method.list)"]
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                    parent: String,
                    filter: Option<String>,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    read_mask: Option<String>,
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
                    #[doc = "Optional. Only include resources that match the filter. Supported filter fields: - org_unit_id - serial_number - device_id "]
                    pub fn filter(mut self, value: impl Into<String>) -> Self {
                        self.filter = Some(value.into());
                        self
                    }
                    #[doc = "Maximum number of results to return. Default value is 100. Maximum value is 200."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "Token to specify next page in the list."]
                    pub fn page_token(mut self, value: impl Into<String>) -> Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "Required. Read mask to specify which fields to return."]
                    pub fn read_mask(mut self, value: impl Into<String>) -> Self {
                        self.read_mask = Some(value.into());
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
                        crate::schemas::GoogleChromeManagementV1ListTelemetryDevicesResponse,
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
                        crate::schemas::GoogleChromeManagementV1ListTelemetryDevicesResponse,
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
                        let mut output = "https://chromemanagement.googleapis.com/".to_owned();
                        output.push_str("v1/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/telemetry/devices");
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
                        req = req.query(&[("readMask", &self.read_mask)]);
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
