#![allow(rustdoc::bare_urls)]
#![doc = "# Resources and Methods\n* [billing_accounts](resources/billing_accounts/struct.BillingAccountsActions.html)\n  * [*estimateCostScenario*](resources/billing_accounts/struct.EstimateCostScenarioRequestBuilder.html)\n* [v_1beta](resources/v_1beta/struct.V1BetaActions.html)\n  * [*estimateCostScenario*](resources/v_1beta/struct.EstimateCostScenarioRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View and manage your Google Cloud Platform billing accounts\n\n`https://www.googleapis.com/auth/cloud-billing`"]
    pub const CLOUD_BILLING: &str = "https://www.googleapis.com/auth/cloud-billing";
    #[doc = "View your Google Cloud Platform billing accounts\n\n`https://www.googleapis.com/auth/cloud-billing.readonly`"]
    pub const CLOUD_BILLING_READONLY: &str =
        "https://www.googleapis.com/auth/cloud-billing.readonly";
    #[doc = "See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.\n\n`https://www.googleapis.com/auth/cloud-platform`"]
    pub const CLOUD_PLATFORM: &str = "https://www.googleapis.com/auth/cloud-platform";
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
    pub struct CacheFillRegions {
        #[doc = "The destination region for cache fill."]
        #[serde(
            rename = "destinationRegion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination_region:
            ::std::option::Option<crate::schemas::CacheFillRegionsDestinationRegion>,
        #[doc = "The source region for cache fill."]
        #[serde(
            rename = "sourceRegion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_region: ::std::option::Option<crate::schemas::CacheFillRegionsSourceRegion>,
    }
    impl ::google_field_selector::FieldSelector for CacheFillRegions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CacheFillRegions {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CacheFillRegionsDestinationRegion {
        #[doc = "Asia Pacific."]
        CacheFillDestinationRegionAsiaPacific,
        #[doc = "China."]
        CacheFillDestinationRegionChina,
        #[doc = "Europe"]
        CacheFillDestinationRegionEurope,
        #[doc = "North America."]
        CacheFillDestinationRegionNorthAmerica,
        #[doc = "Oceania."]
        CacheFillDestinationRegionOceania,
        #[doc = "Others."]
        CacheFillDestinationRegionOthers,
        #[doc = "South America."]
        CacheFillDestinationRegionSouthAmerica,
        #[doc = "Not specified."]
        CacheFillDestinationRegionUnspecified,
    }
    impl CacheFillRegionsDestinationRegion {
        pub fn as_str(self) -> &'static str {
            match self {
                CacheFillRegionsDestinationRegion::CacheFillDestinationRegionAsiaPacific => {
                    "CACHE_FILL_DESTINATION_REGION_ASIA_PACIFIC"
                }
                CacheFillRegionsDestinationRegion::CacheFillDestinationRegionChina => {
                    "CACHE_FILL_DESTINATION_REGION_CHINA"
                }
                CacheFillRegionsDestinationRegion::CacheFillDestinationRegionEurope => {
                    "CACHE_FILL_DESTINATION_REGION_EUROPE"
                }
                CacheFillRegionsDestinationRegion::CacheFillDestinationRegionNorthAmerica => {
                    "CACHE_FILL_DESTINATION_REGION_NORTH_AMERICA"
                }
                CacheFillRegionsDestinationRegion::CacheFillDestinationRegionOceania => {
                    "CACHE_FILL_DESTINATION_REGION_OCEANIA"
                }
                CacheFillRegionsDestinationRegion::CacheFillDestinationRegionOthers => {
                    "CACHE_FILL_DESTINATION_REGION_OTHERS"
                }
                CacheFillRegionsDestinationRegion::CacheFillDestinationRegionSouthAmerica => {
                    "CACHE_FILL_DESTINATION_REGION_SOUTH_AMERICA"
                }
                CacheFillRegionsDestinationRegion::CacheFillDestinationRegionUnspecified => {
                    "CACHE_FILL_DESTINATION_REGION_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for CacheFillRegionsDestinationRegion {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CacheFillRegionsDestinationRegion {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CacheFillRegionsDestinationRegion, ()> {
            Ok(match s {
                "CACHE_FILL_DESTINATION_REGION_ASIA_PACIFIC" => {
                    CacheFillRegionsDestinationRegion::CacheFillDestinationRegionAsiaPacific
                }
                "CACHE_FILL_DESTINATION_REGION_CHINA" => {
                    CacheFillRegionsDestinationRegion::CacheFillDestinationRegionChina
                }
                "CACHE_FILL_DESTINATION_REGION_EUROPE" => {
                    CacheFillRegionsDestinationRegion::CacheFillDestinationRegionEurope
                }
                "CACHE_FILL_DESTINATION_REGION_NORTH_AMERICA" => {
                    CacheFillRegionsDestinationRegion::CacheFillDestinationRegionNorthAmerica
                }
                "CACHE_FILL_DESTINATION_REGION_OCEANIA" => {
                    CacheFillRegionsDestinationRegion::CacheFillDestinationRegionOceania
                }
                "CACHE_FILL_DESTINATION_REGION_OTHERS" => {
                    CacheFillRegionsDestinationRegion::CacheFillDestinationRegionOthers
                }
                "CACHE_FILL_DESTINATION_REGION_SOUTH_AMERICA" => {
                    CacheFillRegionsDestinationRegion::CacheFillDestinationRegionSouthAmerica
                }
                "CACHE_FILL_DESTINATION_REGION_UNSPECIFIED" => {
                    CacheFillRegionsDestinationRegion::CacheFillDestinationRegionUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CacheFillRegionsDestinationRegion {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CacheFillRegionsDestinationRegion {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CacheFillRegionsDestinationRegion {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CACHE_FILL_DESTINATION_REGION_ASIA_PACIFIC" => {
                    CacheFillRegionsDestinationRegion::CacheFillDestinationRegionAsiaPacific
                }
                "CACHE_FILL_DESTINATION_REGION_CHINA" => {
                    CacheFillRegionsDestinationRegion::CacheFillDestinationRegionChina
                }
                "CACHE_FILL_DESTINATION_REGION_EUROPE" => {
                    CacheFillRegionsDestinationRegion::CacheFillDestinationRegionEurope
                }
                "CACHE_FILL_DESTINATION_REGION_NORTH_AMERICA" => {
                    CacheFillRegionsDestinationRegion::CacheFillDestinationRegionNorthAmerica
                }
                "CACHE_FILL_DESTINATION_REGION_OCEANIA" => {
                    CacheFillRegionsDestinationRegion::CacheFillDestinationRegionOceania
                }
                "CACHE_FILL_DESTINATION_REGION_OTHERS" => {
                    CacheFillRegionsDestinationRegion::CacheFillDestinationRegionOthers
                }
                "CACHE_FILL_DESTINATION_REGION_SOUTH_AMERICA" => {
                    CacheFillRegionsDestinationRegion::CacheFillDestinationRegionSouthAmerica
                }
                "CACHE_FILL_DESTINATION_REGION_UNSPECIFIED" => {
                    CacheFillRegionsDestinationRegion::CacheFillDestinationRegionUnspecified
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
    impl ::google_field_selector::FieldSelector for CacheFillRegionsDestinationRegion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CacheFillRegionsDestinationRegion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CacheFillRegionsSourceRegion {
        #[doc = "Asia Pacific."]
        CacheFillRegionAsiaPacific,
        #[doc = "Europe"]
        CacheFillSourceRegionEurope,
        #[doc = "North America."]
        CacheFillSourceRegionNorthAmerica,
        #[doc = "Oceania."]
        CacheFillSourceRegionOceania,
        #[doc = "South America."]
        CacheFillSourceRegionSouthAmerica,
        #[doc = "Not specified."]
        CacheFillSourceRegionUnspecified,
    }
    impl CacheFillRegionsSourceRegion {
        pub fn as_str(self) -> &'static str {
            match self {
                CacheFillRegionsSourceRegion::CacheFillRegionAsiaPacific => {
                    "CACHE_FILL_REGION_ASIA_PACIFIC"
                }
                CacheFillRegionsSourceRegion::CacheFillSourceRegionEurope => {
                    "CACHE_FILL_SOURCE_REGION_EUROPE"
                }
                CacheFillRegionsSourceRegion::CacheFillSourceRegionNorthAmerica => {
                    "CACHE_FILL_SOURCE_REGION_NORTH_AMERICA"
                }
                CacheFillRegionsSourceRegion::CacheFillSourceRegionOceania => {
                    "CACHE_FILL_SOURCE_REGION_OCEANIA"
                }
                CacheFillRegionsSourceRegion::CacheFillSourceRegionSouthAmerica => {
                    "CACHE_FILL_SOURCE_REGION_SOUTH_AMERICA"
                }
                CacheFillRegionsSourceRegion::CacheFillSourceRegionUnspecified => {
                    "CACHE_FILL_SOURCE_REGION_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for CacheFillRegionsSourceRegion {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CacheFillRegionsSourceRegion {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CacheFillRegionsSourceRegion, ()> {
            Ok(match s {
                "CACHE_FILL_REGION_ASIA_PACIFIC" => {
                    CacheFillRegionsSourceRegion::CacheFillRegionAsiaPacific
                }
                "CACHE_FILL_SOURCE_REGION_EUROPE" => {
                    CacheFillRegionsSourceRegion::CacheFillSourceRegionEurope
                }
                "CACHE_FILL_SOURCE_REGION_NORTH_AMERICA" => {
                    CacheFillRegionsSourceRegion::CacheFillSourceRegionNorthAmerica
                }
                "CACHE_FILL_SOURCE_REGION_OCEANIA" => {
                    CacheFillRegionsSourceRegion::CacheFillSourceRegionOceania
                }
                "CACHE_FILL_SOURCE_REGION_SOUTH_AMERICA" => {
                    CacheFillRegionsSourceRegion::CacheFillSourceRegionSouthAmerica
                }
                "CACHE_FILL_SOURCE_REGION_UNSPECIFIED" => {
                    CacheFillRegionsSourceRegion::CacheFillSourceRegionUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CacheFillRegionsSourceRegion {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CacheFillRegionsSourceRegion {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CacheFillRegionsSourceRegion {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CACHE_FILL_REGION_ASIA_PACIFIC" => {
                    CacheFillRegionsSourceRegion::CacheFillRegionAsiaPacific
                }
                "CACHE_FILL_SOURCE_REGION_EUROPE" => {
                    CacheFillRegionsSourceRegion::CacheFillSourceRegionEurope
                }
                "CACHE_FILL_SOURCE_REGION_NORTH_AMERICA" => {
                    CacheFillRegionsSourceRegion::CacheFillSourceRegionNorthAmerica
                }
                "CACHE_FILL_SOURCE_REGION_OCEANIA" => {
                    CacheFillRegionsSourceRegion::CacheFillSourceRegionOceania
                }
                "CACHE_FILL_SOURCE_REGION_SOUTH_AMERICA" => {
                    CacheFillRegionsSourceRegion::CacheFillSourceRegionSouthAmerica
                }
                "CACHE_FILL_SOURCE_REGION_UNSPECIFIED" => {
                    CacheFillRegionsSourceRegion::CacheFillSourceRegionUnspecified
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
    impl ::google_field_selector::FieldSelector for CacheFillRegionsSourceRegion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CacheFillRegionsSourceRegion {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CloudCdnEgressWorkload {
        #[doc = "The destination for the cache egress charges."]
        #[serde(
            rename = "cacheEgressDestination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cache_egress_destination:
            ::std::option::Option<crate::schemas::CloudCdnEgressWorkloadCacheEgressDestination>,
        #[doc = "Cache egress usage. The rate of data cache egressed in the destination. For example : units such as “GiBy/s” or “TBy/mo”."]
        #[serde(
            rename = "cacheEgressRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cache_egress_rate: ::std::option::Option<crate::schemas::Usage>,
    }
    impl ::google_field_selector::FieldSelector for CloudCdnEgressWorkload {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CloudCdnEgressWorkload {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CloudCdnEgressWorkloadCacheEgressDestination {
        #[doc = "Asia Pacific."]
        CacheEgressDestinationAsiaPacific,
        #[doc = "China."]
        CacheEgressDestinationChina,
        #[doc = "Europe."]
        CacheEgressDestinationEurope,
        #[doc = "Latin America (Including the Caribbean, South America and Central America.)"]
        CacheEgressDestinationLatinAmerica,
        #[doc = "North America."]
        CacheEgressDestinationNorthAmerica,
        #[doc = "Oceania including Australia, New Zealand, and surrounding Pacific Ocean islands such as Papua New Guinea and Fiji. This region excludes Hawaii."]
        CacheEgressDestinationOceania,
        #[doc = "All other destinations (including Africa and Antarctica)"]
        CacheEgressDestinationOtherDestinations,
        #[doc = "Unspecified."]
        CacheEgressDestinationUnspecified,
    }
    impl CloudCdnEgressWorkloadCacheEgressDestination {
        pub fn as_str(self) -> &'static str {
            match self { CloudCdnEgressWorkloadCacheEgressDestination :: CacheEgressDestinationAsiaPacific => "CACHE_EGRESS_DESTINATION_ASIA_PACIFIC" , CloudCdnEgressWorkloadCacheEgressDestination :: CacheEgressDestinationChina => "CACHE_EGRESS_DESTINATION_CHINA" , CloudCdnEgressWorkloadCacheEgressDestination :: CacheEgressDestinationEurope => "CACHE_EGRESS_DESTINATION_EUROPE" , CloudCdnEgressWorkloadCacheEgressDestination :: CacheEgressDestinationLatinAmerica => "CACHE_EGRESS_DESTINATION_LATIN_AMERICA" , CloudCdnEgressWorkloadCacheEgressDestination :: CacheEgressDestinationNorthAmerica => "CACHE_EGRESS_DESTINATION_NORTH_AMERICA" , CloudCdnEgressWorkloadCacheEgressDestination :: CacheEgressDestinationOceania => "CACHE_EGRESS_DESTINATION_OCEANIA" , CloudCdnEgressWorkloadCacheEgressDestination :: CacheEgressDestinationOtherDestinations => "CACHE_EGRESS_DESTINATION_OTHER_DESTINATIONS" , CloudCdnEgressWorkloadCacheEgressDestination :: CacheEgressDestinationUnspecified => "CACHE_EGRESS_DESTINATION_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for CloudCdnEgressWorkloadCacheEgressDestination {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CloudCdnEgressWorkloadCacheEgressDestination {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<CloudCdnEgressWorkloadCacheEgressDestination, ()> {
            Ok (match s { "CACHE_EGRESS_DESTINATION_ASIA_PACIFIC" => CloudCdnEgressWorkloadCacheEgressDestination :: CacheEgressDestinationAsiaPacific , "CACHE_EGRESS_DESTINATION_CHINA" => CloudCdnEgressWorkloadCacheEgressDestination :: CacheEgressDestinationChina , "CACHE_EGRESS_DESTINATION_EUROPE" => CloudCdnEgressWorkloadCacheEgressDestination :: CacheEgressDestinationEurope , "CACHE_EGRESS_DESTINATION_LATIN_AMERICA" => CloudCdnEgressWorkloadCacheEgressDestination :: CacheEgressDestinationLatinAmerica , "CACHE_EGRESS_DESTINATION_NORTH_AMERICA" => CloudCdnEgressWorkloadCacheEgressDestination :: CacheEgressDestinationNorthAmerica , "CACHE_EGRESS_DESTINATION_OCEANIA" => CloudCdnEgressWorkloadCacheEgressDestination :: CacheEgressDestinationOceania , "CACHE_EGRESS_DESTINATION_OTHER_DESTINATIONS" => CloudCdnEgressWorkloadCacheEgressDestination :: CacheEgressDestinationOtherDestinations , "CACHE_EGRESS_DESTINATION_UNSPECIFIED" => CloudCdnEgressWorkloadCacheEgressDestination :: CacheEgressDestinationUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for CloudCdnEgressWorkloadCacheEgressDestination {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CloudCdnEgressWorkloadCacheEgressDestination {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CloudCdnEgressWorkloadCacheEgressDestination {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CACHE_EGRESS_DESTINATION_ASIA_PACIFIC" => CloudCdnEgressWorkloadCacheEgressDestination :: CacheEgressDestinationAsiaPacific , "CACHE_EGRESS_DESTINATION_CHINA" => CloudCdnEgressWorkloadCacheEgressDestination :: CacheEgressDestinationChina , "CACHE_EGRESS_DESTINATION_EUROPE" => CloudCdnEgressWorkloadCacheEgressDestination :: CacheEgressDestinationEurope , "CACHE_EGRESS_DESTINATION_LATIN_AMERICA" => CloudCdnEgressWorkloadCacheEgressDestination :: CacheEgressDestinationLatinAmerica , "CACHE_EGRESS_DESTINATION_NORTH_AMERICA" => CloudCdnEgressWorkloadCacheEgressDestination :: CacheEgressDestinationNorthAmerica , "CACHE_EGRESS_DESTINATION_OCEANIA" => CloudCdnEgressWorkloadCacheEgressDestination :: CacheEgressDestinationOceania , "CACHE_EGRESS_DESTINATION_OTHER_DESTINATIONS" => CloudCdnEgressWorkloadCacheEgressDestination :: CacheEgressDestinationOtherDestinations , "CACHE_EGRESS_DESTINATION_UNSPECIFIED" => CloudCdnEgressWorkloadCacheEgressDestination :: CacheEgressDestinationUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector for CloudCdnEgressWorkloadCacheEgressDestination {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CloudCdnEgressWorkloadCacheEgressDestination {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CloudCdnWorkload {
        #[doc = "The source service for the cache fill."]
        #[serde(
            rename = "cacheFillOriginService",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cache_fill_origin_service:
            ::std::option::Option<crate::schemas::CloudCdnWorkloadCacheFillOriginService>,
        #[doc = "Cache fill usage. The rate of data transferred between cache fill regions. For example: units such as “GiBy/s” or “TBy/mo”."]
        #[serde(
            rename = "cacheFillRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cache_fill_rate: ::std::option::Option<crate::schemas::Usage>,
        #[doc = "The regions where data is transferred from Google data locations into Google global cache servers. The SKU prices for cache fill across services are the same."]
        #[serde(
            rename = "cacheFillRegions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cache_fill_regions: ::std::option::Option<crate::schemas::CacheFillRegions>,
        #[doc = "Cache look up requests. This is specified to indicate the number of requests. For example: units such as “1/s”."]
        #[serde(
            rename = "cacheLookUpRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cache_look_up_rate: ::std::option::Option<crate::schemas::Usage>,
    }
    impl ::google_field_selector::FieldSelector for CloudCdnWorkload {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CloudCdnWorkload {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CloudCdnWorkloadCacheFillOriginService {
        #[doc = "Origin service is backend service, such as Compute VMs, external backend, etc."]
        CacheFillOriginServiceBackendService,
        #[doc = "Origin service is Google Cloud Storage."]
        CacheFillOriginServiceGoogleCloudStorageBucket,
        #[doc = "Not specified."]
        CacheFillOriginServiceUnspecified,
    }
    impl CloudCdnWorkloadCacheFillOriginService {
        pub fn as_str(self) -> &'static str {
            match self { CloudCdnWorkloadCacheFillOriginService :: CacheFillOriginServiceBackendService => "CACHE_FILL_ORIGIN_SERVICE_BACKEND_SERVICE" , CloudCdnWorkloadCacheFillOriginService :: CacheFillOriginServiceGoogleCloudStorageBucket => "CACHE_FILL_ORIGIN_SERVICE_GOOGLE_CLOUD_STORAGE_BUCKET" , CloudCdnWorkloadCacheFillOriginService :: CacheFillOriginServiceUnspecified => "CACHE_FILL_ORIGIN_SERVICE_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for CloudCdnWorkloadCacheFillOriginService {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CloudCdnWorkloadCacheFillOriginService {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CloudCdnWorkloadCacheFillOriginService, ()> {
            Ok (match s { "CACHE_FILL_ORIGIN_SERVICE_BACKEND_SERVICE" => CloudCdnWorkloadCacheFillOriginService :: CacheFillOriginServiceBackendService , "CACHE_FILL_ORIGIN_SERVICE_GOOGLE_CLOUD_STORAGE_BUCKET" => CloudCdnWorkloadCacheFillOriginService :: CacheFillOriginServiceGoogleCloudStorageBucket , "CACHE_FILL_ORIGIN_SERVICE_UNSPECIFIED" => CloudCdnWorkloadCacheFillOriginService :: CacheFillOriginServiceUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for CloudCdnWorkloadCacheFillOriginService {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CloudCdnWorkloadCacheFillOriginService {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CloudCdnWorkloadCacheFillOriginService {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "CACHE_FILL_ORIGIN_SERVICE_BACKEND_SERVICE" => CloudCdnWorkloadCacheFillOriginService :: CacheFillOriginServiceBackendService , "CACHE_FILL_ORIGIN_SERVICE_GOOGLE_CLOUD_STORAGE_BUCKET" => CloudCdnWorkloadCacheFillOriginService :: CacheFillOriginServiceGoogleCloudStorageBucket , "CACHE_FILL_ORIGIN_SERVICE_UNSPECIFIED" => CloudCdnWorkloadCacheFillOriginService :: CacheFillOriginServiceUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector for CloudCdnWorkloadCacheFillOriginService {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CloudCdnWorkloadCacheFillOriginService {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CloudInterconnectEgressWorkload {
        #[doc = "Data egress usage. This usage applies when you move or copy data from one Google Cloud service to another service. Expected units such as “GiBy/s, By/s, etc.”"]
        #[serde(
            rename = "egressRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub egress_rate: ::std::option::Option<crate::schemas::Usage>,
        #[doc = "Locations in the [Interconnect connection location table](https://cloud.google.com/vpc/network-pricing#interconnect-pricing). This is the interconnect egress charges."]
        #[serde(
            rename = "interconnectConnectionLocation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub interconnect_connection_location: ::std::option::Option<
            crate::schemas::CloudInterconnectEgressWorkloadInterconnectConnectionLocation,
        >,
    }
    impl ::google_field_selector::FieldSelector for CloudInterconnectEgressWorkload {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CloudInterconnectEgressWorkload {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CloudInterconnectEgressWorkloadInterconnectConnectionLocation {
        #[doc = "Asia."]
        InterconnectConnectionLocationAsia,
        #[doc = "Australia."]
        InterconnectConnectionLocationAustralia,
        #[doc = "Europe."]
        InterconnectConnectionLocationEurope,
        #[doc = "North America."]
        InterconnectConnectionLocationNorthAmerica,
        #[doc = "South America."]
        InterconnectConnectionLocationSouthAmerica,
        #[doc = "Unspecified."]
        InterconnectConnectionLocationUnspecified,
    }
    impl CloudInterconnectEgressWorkloadInterconnectConnectionLocation {
        pub fn as_str(self) -> &'static str {
            match self { CloudInterconnectEgressWorkloadInterconnectConnectionLocation :: InterconnectConnectionLocationAsia => "INTERCONNECT_CONNECTION_LOCATION_ASIA" , CloudInterconnectEgressWorkloadInterconnectConnectionLocation :: InterconnectConnectionLocationAustralia => "INTERCONNECT_CONNECTION_LOCATION_AUSTRALIA" , CloudInterconnectEgressWorkloadInterconnectConnectionLocation :: InterconnectConnectionLocationEurope => "INTERCONNECT_CONNECTION_LOCATION_EUROPE" , CloudInterconnectEgressWorkloadInterconnectConnectionLocation :: InterconnectConnectionLocationNorthAmerica => "INTERCONNECT_CONNECTION_LOCATION_NORTH_AMERICA" , CloudInterconnectEgressWorkloadInterconnectConnectionLocation :: InterconnectConnectionLocationSouthAmerica => "INTERCONNECT_CONNECTION_LOCATION_SOUTH_AMERICA" , CloudInterconnectEgressWorkloadInterconnectConnectionLocation :: InterconnectConnectionLocationUnspecified => "INTERCONNECT_CONNECTION_LOCATION_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for CloudInterconnectEgressWorkloadInterconnectConnectionLocation {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CloudInterconnectEgressWorkloadInterconnectConnectionLocation {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<CloudInterconnectEgressWorkloadInterconnectConnectionLocation, ()>
        {
            Ok (match s { "INTERCONNECT_CONNECTION_LOCATION_ASIA" => CloudInterconnectEgressWorkloadInterconnectConnectionLocation :: InterconnectConnectionLocationAsia , "INTERCONNECT_CONNECTION_LOCATION_AUSTRALIA" => CloudInterconnectEgressWorkloadInterconnectConnectionLocation :: InterconnectConnectionLocationAustralia , "INTERCONNECT_CONNECTION_LOCATION_EUROPE" => CloudInterconnectEgressWorkloadInterconnectConnectionLocation :: InterconnectConnectionLocationEurope , "INTERCONNECT_CONNECTION_LOCATION_NORTH_AMERICA" => CloudInterconnectEgressWorkloadInterconnectConnectionLocation :: InterconnectConnectionLocationNorthAmerica , "INTERCONNECT_CONNECTION_LOCATION_SOUTH_AMERICA" => CloudInterconnectEgressWorkloadInterconnectConnectionLocation :: InterconnectConnectionLocationSouthAmerica , "INTERCONNECT_CONNECTION_LOCATION_UNSPECIFIED" => CloudInterconnectEgressWorkloadInterconnectConnectionLocation :: InterconnectConnectionLocationUnspecified , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for CloudInterconnectEgressWorkloadInterconnectConnectionLocation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CloudInterconnectEgressWorkloadInterconnectConnectionLocation {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for CloudInterconnectEgressWorkloadInterconnectConnectionLocation
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "INTERCONNECT_CONNECTION_LOCATION_ASIA" => CloudInterconnectEgressWorkloadInterconnectConnectionLocation :: InterconnectConnectionLocationAsia , "INTERCONNECT_CONNECTION_LOCATION_AUSTRALIA" => CloudInterconnectEgressWorkloadInterconnectConnectionLocation :: InterconnectConnectionLocationAustralia , "INTERCONNECT_CONNECTION_LOCATION_EUROPE" => CloudInterconnectEgressWorkloadInterconnectConnectionLocation :: InterconnectConnectionLocationEurope , "INTERCONNECT_CONNECTION_LOCATION_NORTH_AMERICA" => CloudInterconnectEgressWorkloadInterconnectConnectionLocation :: InterconnectConnectionLocationNorthAmerica , "INTERCONNECT_CONNECTION_LOCATION_SOUTH_AMERICA" => CloudInterconnectEgressWorkloadInterconnectConnectionLocation :: InterconnectConnectionLocationSouthAmerica , "INTERCONNECT_CONNECTION_LOCATION_UNSPECIFIED" => CloudInterconnectEgressWorkloadInterconnectConnectionLocation :: InterconnectConnectionLocationUnspecified , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector
        for CloudInterconnectEgressWorkloadInterconnectConnectionLocation
    {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType
        for CloudInterconnectEgressWorkloadInterconnectConnectionLocation
    {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CloudInterconnectWorkload {
        #[doc = "VLAN attachment used for interconnect."]
        #[serde(
            rename = "interconnectAttachments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub interconnect_attachments: ::std::option::Option<Vec<crate::schemas::VlanAttachment>>,
        #[doc = "Vlan attachment type."]
        #[serde(
            rename = "interconnectType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub interconnect_type:
            ::std::option::Option<crate::schemas::CloudInterconnectWorkloadInterconnectType>,
        #[doc = "Interconnect circuit link type."]
        #[serde(
            rename = "linkType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link_type: ::std::option::Option<crate::schemas::CloudInterconnectWorkloadLinkType>,
        #[doc = "Interconnect usage. This is specified as a unitless quantity which indicates the number of circuit provisioned in interconnect."]
        #[serde(
            rename = "provisionedLinkCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provisioned_link_count: ::std::option::Option<crate::schemas::Usage>,
    }
    impl ::google_field_selector::FieldSelector for CloudInterconnectWorkload {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CloudInterconnectWorkload {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CloudInterconnectWorkloadInterconnectType {
        #[doc = "Type is dedicated."]
        InterconnectTypeDedicated,
        #[doc = "Type is partner."]
        InterconnectTypePartner,
        #[doc = "Unspecified."]
        InterconnectTypeUnspecified,
    }
    impl CloudInterconnectWorkloadInterconnectType {
        pub fn as_str(self) -> &'static str {
            match self {
                CloudInterconnectWorkloadInterconnectType::InterconnectTypeDedicated => {
                    "INTERCONNECT_TYPE_DEDICATED"
                }
                CloudInterconnectWorkloadInterconnectType::InterconnectTypePartner => {
                    "INTERCONNECT_TYPE_PARTNER"
                }
                CloudInterconnectWorkloadInterconnectType::InterconnectTypeUnspecified => {
                    "INTERCONNECT_TYPE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for CloudInterconnectWorkloadInterconnectType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CloudInterconnectWorkloadInterconnectType {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<CloudInterconnectWorkloadInterconnectType, ()> {
            Ok(match s {
                "INTERCONNECT_TYPE_DEDICATED" => {
                    CloudInterconnectWorkloadInterconnectType::InterconnectTypeDedicated
                }
                "INTERCONNECT_TYPE_PARTNER" => {
                    CloudInterconnectWorkloadInterconnectType::InterconnectTypePartner
                }
                "INTERCONNECT_TYPE_UNSPECIFIED" => {
                    CloudInterconnectWorkloadInterconnectType::InterconnectTypeUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CloudInterconnectWorkloadInterconnectType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CloudInterconnectWorkloadInterconnectType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CloudInterconnectWorkloadInterconnectType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "INTERCONNECT_TYPE_DEDICATED" => {
                    CloudInterconnectWorkloadInterconnectType::InterconnectTypeDedicated
                }
                "INTERCONNECT_TYPE_PARTNER" => {
                    CloudInterconnectWorkloadInterconnectType::InterconnectTypePartner
                }
                "INTERCONNECT_TYPE_UNSPECIFIED" => {
                    CloudInterconnectWorkloadInterconnectType::InterconnectTypeUnspecified
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
    impl ::google_field_selector::FieldSelector for CloudInterconnectWorkloadInterconnectType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CloudInterconnectWorkloadInterconnectType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CloudInterconnectWorkloadLinkType {
        #[doc = "Link type is 100 gbps."]
        LinkTypeEthernet100GLr,
        #[doc = "Link type is 10 gbps."]
        LinkTypeEthernet10GLr,
        #[doc = "Unspecified."]
        LinkTypeUnspecified,
    }
    impl CloudInterconnectWorkloadLinkType {
        pub fn as_str(self) -> &'static str {
            match self {
                CloudInterconnectWorkloadLinkType::LinkTypeEthernet100GLr => {
                    "LINK_TYPE_ETHERNET_100G_LR"
                }
                CloudInterconnectWorkloadLinkType::LinkTypeEthernet10GLr => {
                    "LINK_TYPE_ETHERNET_10G_LR"
                }
                CloudInterconnectWorkloadLinkType::LinkTypeUnspecified => "LINK_TYPE_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for CloudInterconnectWorkloadLinkType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CloudInterconnectWorkloadLinkType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<CloudInterconnectWorkloadLinkType, ()> {
            Ok(match s {
                "LINK_TYPE_ETHERNET_100G_LR" => {
                    CloudInterconnectWorkloadLinkType::LinkTypeEthernet100GLr
                }
                "LINK_TYPE_ETHERNET_10G_LR" => {
                    CloudInterconnectWorkloadLinkType::LinkTypeEthernet10GLr
                }
                "LINK_TYPE_UNSPECIFIED" => CloudInterconnectWorkloadLinkType::LinkTypeUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CloudInterconnectWorkloadLinkType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CloudInterconnectWorkloadLinkType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CloudInterconnectWorkloadLinkType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "LINK_TYPE_ETHERNET_100G_LR" => {
                    CloudInterconnectWorkloadLinkType::LinkTypeEthernet100GLr
                }
                "LINK_TYPE_ETHERNET_10G_LR" => {
                    CloudInterconnectWorkloadLinkType::LinkTypeEthernet10GLr
                }
                "LINK_TYPE_UNSPECIFIED" => CloudInterconnectWorkloadLinkType::LinkTypeUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for CloudInterconnectWorkloadLinkType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CloudInterconnectWorkloadLinkType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CloudStorageEgressWorkload {
        #[doc = "Where the data is sent to."]
        #[serde(
            rename = "destinationContinent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination_continent:
            ::std::option::Option<crate::schemas::CloudStorageEgressWorkloadDestinationContinent>,
        #[doc = "Egress usage rate. This usage applies when you move or copy data from one Cloud Storage bucket to another or when another Google Cloud service accesses data in your Cloud Storage bucket. Expected units such as “GiBy/s, By/s, …”"]
        #[serde(
            rename = "egressRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub egress_rate: ::std::option::Option<crate::schemas::Usage>,
        #[doc = "Where the data comes from."]
        #[serde(
            rename = "sourceContinent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_continent:
            ::std::option::Option<crate::schemas::CloudStorageEgressWorkloadSourceContinent>,
    }
    impl ::google_field_selector::FieldSelector for CloudStorageEgressWorkload {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CloudStorageEgressWorkload {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CloudStorageEgressWorkloadDestinationContinent {
        #[doc = "Asia Pacific."]
        DestinationContinentAsiaPacific,
        #[doc = "Australia."]
        DestinationContinentAutralia,
        #[doc = "Europe."]
        DestinationContinentEurope,
        #[doc = "North America."]
        DestinationContinentNorthAmerica,
        #[doc = "South America"]
        DestinationContinentSouthAmerica,
        #[doc = "Not specified."]
        DestinationContinentUnspecified,
    }
    impl CloudStorageEgressWorkloadDestinationContinent {
        pub fn as_str(self) -> &'static str {
            match self { CloudStorageEgressWorkloadDestinationContinent :: DestinationContinentAsiaPacific => "DESTINATION_CONTINENT_ASIA_PACIFIC" , CloudStorageEgressWorkloadDestinationContinent :: DestinationContinentAutralia => "DESTINATION_CONTINENT_AUTRALIA" , CloudStorageEgressWorkloadDestinationContinent :: DestinationContinentEurope => "DESTINATION_CONTINENT_EUROPE" , CloudStorageEgressWorkloadDestinationContinent :: DestinationContinentNorthAmerica => "DESTINATION_CONTINENT_NORTH_AMERICA" , CloudStorageEgressWorkloadDestinationContinent :: DestinationContinentSouthAmerica => "DESTINATION_CONTINENT_SOUTH_AMERICA" , CloudStorageEgressWorkloadDestinationContinent :: DestinationContinentUnspecified => "DESTINATION_CONTINENT_UNSPECIFIED" , }
        }
    }
    impl ::std::convert::AsRef<str> for CloudStorageEgressWorkloadDestinationContinent {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CloudStorageEgressWorkloadDestinationContinent {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<CloudStorageEgressWorkloadDestinationContinent, ()> {
            Ok(match s {
                "DESTINATION_CONTINENT_ASIA_PACIFIC" => {
                    CloudStorageEgressWorkloadDestinationContinent::DestinationContinentAsiaPacific
                }
                "DESTINATION_CONTINENT_AUTRALIA" => {
                    CloudStorageEgressWorkloadDestinationContinent::DestinationContinentAutralia
                }
                "DESTINATION_CONTINENT_EUROPE" => {
                    CloudStorageEgressWorkloadDestinationContinent::DestinationContinentEurope
                }
                "DESTINATION_CONTINENT_NORTH_AMERICA" => {
                    CloudStorageEgressWorkloadDestinationContinent::DestinationContinentNorthAmerica
                }
                "DESTINATION_CONTINENT_SOUTH_AMERICA" => {
                    CloudStorageEgressWorkloadDestinationContinent::DestinationContinentSouthAmerica
                }
                "DESTINATION_CONTINENT_UNSPECIFIED" => {
                    CloudStorageEgressWorkloadDestinationContinent::DestinationContinentUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CloudStorageEgressWorkloadDestinationContinent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CloudStorageEgressWorkloadDestinationContinent {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CloudStorageEgressWorkloadDestinationContinent {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DESTINATION_CONTINENT_ASIA_PACIFIC" => {
                    CloudStorageEgressWorkloadDestinationContinent::DestinationContinentAsiaPacific
                }
                "DESTINATION_CONTINENT_AUTRALIA" => {
                    CloudStorageEgressWorkloadDestinationContinent::DestinationContinentAutralia
                }
                "DESTINATION_CONTINENT_EUROPE" => {
                    CloudStorageEgressWorkloadDestinationContinent::DestinationContinentEurope
                }
                "DESTINATION_CONTINENT_NORTH_AMERICA" => {
                    CloudStorageEgressWorkloadDestinationContinent::DestinationContinentNorthAmerica
                }
                "DESTINATION_CONTINENT_SOUTH_AMERICA" => {
                    CloudStorageEgressWorkloadDestinationContinent::DestinationContinentSouthAmerica
                }
                "DESTINATION_CONTINENT_UNSPECIFIED" => {
                    CloudStorageEgressWorkloadDestinationContinent::DestinationContinentUnspecified
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
    impl ::google_field_selector::FieldSelector for CloudStorageEgressWorkloadDestinationContinent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CloudStorageEgressWorkloadDestinationContinent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CloudStorageEgressWorkloadSourceContinent {
        #[doc = "Asia Pacific."]
        SourceContinentAsiaPacific,
        #[doc = "Australia."]
        SourceContinentAustralia,
        #[doc = "Europe."]
        SourceContinentEurope,
        #[doc = "North America."]
        SourceContinentNorthAmerica,
        #[doc = "South America."]
        SourceContinentSouthAmerica,
        #[doc = "Not specified."]
        SourceContinentUnspecified,
    }
    impl CloudStorageEgressWorkloadSourceContinent {
        pub fn as_str(self) -> &'static str {
            match self {
                CloudStorageEgressWorkloadSourceContinent::SourceContinentAsiaPacific => {
                    "SOURCE_CONTINENT_ASIA_PACIFIC"
                }
                CloudStorageEgressWorkloadSourceContinent::SourceContinentAustralia => {
                    "SOURCE_CONTINENT_AUSTRALIA"
                }
                CloudStorageEgressWorkloadSourceContinent::SourceContinentEurope => {
                    "SOURCE_CONTINENT_EUROPE"
                }
                CloudStorageEgressWorkloadSourceContinent::SourceContinentNorthAmerica => {
                    "SOURCE_CONTINENT_NORTH_AMERICA"
                }
                CloudStorageEgressWorkloadSourceContinent::SourceContinentSouthAmerica => {
                    "SOURCE_CONTINENT_SOUTH_AMERICA"
                }
                CloudStorageEgressWorkloadSourceContinent::SourceContinentUnspecified => {
                    "SOURCE_CONTINENT_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::convert::AsRef<str> for CloudStorageEgressWorkloadSourceContinent {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for CloudStorageEgressWorkloadSourceContinent {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<CloudStorageEgressWorkloadSourceContinent, ()> {
            Ok(match s {
                "SOURCE_CONTINENT_ASIA_PACIFIC" => {
                    CloudStorageEgressWorkloadSourceContinent::SourceContinentAsiaPacific
                }
                "SOURCE_CONTINENT_AUSTRALIA" => {
                    CloudStorageEgressWorkloadSourceContinent::SourceContinentAustralia
                }
                "SOURCE_CONTINENT_EUROPE" => {
                    CloudStorageEgressWorkloadSourceContinent::SourceContinentEurope
                }
                "SOURCE_CONTINENT_NORTH_AMERICA" => {
                    CloudStorageEgressWorkloadSourceContinent::SourceContinentNorthAmerica
                }
                "SOURCE_CONTINENT_SOUTH_AMERICA" => {
                    CloudStorageEgressWorkloadSourceContinent::SourceContinentSouthAmerica
                }
                "SOURCE_CONTINENT_UNSPECIFIED" => {
                    CloudStorageEgressWorkloadSourceContinent::SourceContinentUnspecified
                }
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for CloudStorageEgressWorkloadSourceContinent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CloudStorageEgressWorkloadSourceContinent {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CloudStorageEgressWorkloadSourceContinent {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SOURCE_CONTINENT_ASIA_PACIFIC" => {
                    CloudStorageEgressWorkloadSourceContinent::SourceContinentAsiaPacific
                }
                "SOURCE_CONTINENT_AUSTRALIA" => {
                    CloudStorageEgressWorkloadSourceContinent::SourceContinentAustralia
                }
                "SOURCE_CONTINENT_EUROPE" => {
                    CloudStorageEgressWorkloadSourceContinent::SourceContinentEurope
                }
                "SOURCE_CONTINENT_NORTH_AMERICA" => {
                    CloudStorageEgressWorkloadSourceContinent::SourceContinentNorthAmerica
                }
                "SOURCE_CONTINENT_SOUTH_AMERICA" => {
                    CloudStorageEgressWorkloadSourceContinent::SourceContinentSouthAmerica
                }
                "SOURCE_CONTINENT_UNSPECIFIED" => {
                    CloudStorageEgressWorkloadSourceContinent::SourceContinentUnspecified
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
    impl ::google_field_selector::FieldSelector for CloudStorageEgressWorkloadSourceContinent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CloudStorageEgressWorkloadSourceContinent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CloudStorageWorkload {
        #[doc = "Data retrieval usage. A retrieval cost applies when data or metadata is read, copied, or rewritten . For example: units such as “GiBy/s” or “By/s”."]
        #[serde(
            rename = "dataRetrieval",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_retrieval: ::std::option::Option<crate::schemas::Usage>,
        #[doc = "Data storage usage. The amount of data stored in buckets. For example: units such as “GiBy/s” or “TBy/mo”."]
        #[serde(
            rename = "dataStored",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_stored: ::std::option::Option<crate::schemas::Usage>,
        #[doc = "Specify dual regions."]
        #[serde(
            rename = "dualRegion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dual_region: ::std::option::Option<crate::schemas::DualRegional>,
        #[doc = "Specify multi regions."]
        #[serde(
            rename = "multiRegion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub multi_region: ::std::option::Option<crate::schemas::MultiRegional>,
        #[doc = "Class A operation usage in Cloud Storage, such as listing the objects in buckets. See the [operations pricing](https://cloud.google.com/storage/pricing#operations-pricing) tables for a list of which operations fall into each class. For example: units such as “1/s”."]
        #[serde(
            rename = "operationA",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation_a: ::std::option::Option<crate::schemas::Usage>,
        #[doc = "Class B operation usage in Cloud Storage, such as `getIamPolicy`. See the [operations pricing](https://cloud.google.com/storage/pricing#operations-pricing) tables for a list of which operations fall into each class. For example: units such as “1/s”."]
        #[serde(
            rename = "operationB",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub operation_b: ::std::option::Option<crate::schemas::Usage>,
        #[doc = "Specify a single region."]
        #[serde(
            rename = "region",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region: ::std::option::Option<crate::schemas::Regional>,
        #[doc = "The [storage class](https://cloud.google.com/storage/docs/storage-classes#classes) of the data and operation. For example: “standard” or “nearline”."]
        #[serde(
            rename = "storageClass",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub storage_class: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CloudStorageWorkload {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CloudStorageWorkload {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Commitment {
        #[doc = "Required. A name for this commitment. All commitments in a CostScenario must have unique names. Each name may be at most 128 characters long."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "A resource-based committed use discount (CUD)."]
        #[serde(
            rename = "vmResourceBasedCud",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vm_resource_based_cud: ::std::option::Option<crate::schemas::VmResourceBasedCud>,
    }
    impl ::google_field_selector::FieldSelector for Commitment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Commitment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CommitmentCostEstimate {
        #[doc = "Total estimated costs for the commitment."]
        #[serde(
            rename = "commitmentTotalCostEstimate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub commitment_total_cost_estimate: ::std::option::Option<crate::schemas::CostEstimate>,
        #[doc = "The name of the commitment, as specified in the `CostScenario`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Estimated costs for each SKU in the commitment."]
        #[serde(
            rename = "skuCostEstimates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sku_cost_estimates: ::std::option::Option<Vec<crate::schemas::SkuCostEstimate>>,
    }
    impl ::google_field_selector::FieldSelector for CommitmentCostEstimate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CommitmentCostEstimate {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ComputeVmWorkload {
        #[doc = "Defines whether each instance has confidential compute enabled."]
        #[serde(
            rename = "enableConfidentialCompute",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub enable_confidential_compute: ::std::option::Option<bool>,
        #[doc = "Guest accelerators attached to each machine."]
        #[serde(
            rename = "guestAccelerator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub guest_accelerator: ::std::option::Option<crate::schemas::GuestAccelerator>,
        #[doc = "VM usage. This is specified as a unitless quantity which indicates the number of instances running."]
        #[serde(
            rename = "instancesRunning",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub instances_running: ::std::option::Option<crate::schemas::Usage>,
        #[doc = "Premium image licenses used by each instance."]
        #[serde(
            rename = "licenses",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub licenses: ::std::option::Option<Vec<String>>,
        #[doc = "The machine type."]
        #[serde(
            rename = "machineType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub machine_type: ::std::option::Option<crate::schemas::MachineType>,
        #[doc = "Persistent disks attached to each instance. Must include a boot disk."]
        #[serde(
            rename = "persistentDisks",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub persistent_disks: ::std::option::Option<Vec<crate::schemas::PersistentDisk>>,
        #[doc = "Defines whether each instance is preemptible."]
        #[serde(
            rename = "preemptible",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub preemptible: ::std::option::Option<bool>,
        #[doc = "The [region](https://cloud.google.com/compute/docs/regions-zones) where the VMs run. For example: “us-central1”."]
        #[serde(
            rename = "region",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ComputeVmWorkload {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ComputeVmWorkload {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CostEstimate {
        #[doc = "The estimated credits applied."]
        #[serde(
            rename = "creditEstimates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub credit_estimates: ::std::option::Option<Vec<crate::schemas::CreditEstimate>>,
        #[doc = "The estimated net cost after applying credits."]
        #[serde(
            rename = "netCostEstimate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub net_cost_estimate: ::std::option::Option<crate::schemas::Money>,
        #[doc = "The estimated cost prior to applying credits."]
        #[serde(
            rename = "preCreditCostEstimate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pre_credit_cost_estimate: ::std::option::Option<crate::schemas::Money>,
    }
    impl ::google_field_selector::FieldSelector for CostEstimate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CostEstimate {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CostEstimationResult {
        #[doc = "Required. The ISO 4217 currency code for the cost estimate."]
        #[serde(
            rename = "currencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency_code: ::std::option::Option<String>,
        #[doc = "Required. Estimated costs for each idealized month of a `CostScenario`."]
        #[serde(
            rename = "segmentCostEstimates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment_cost_estimates: ::std::option::Option<Vec<crate::schemas::SegmentCostEstimate>>,
        #[doc = "Required. Information about SKUs used in the estimate."]
        #[serde(
            rename = "skus",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub skus: ::std::option::Option<Vec<crate::schemas::Sku>>,
    }
    impl ::google_field_selector::FieldSelector for CostEstimationResult {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CostEstimationResult {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CostScenario {
        #[doc = "New commitments to estimate the costs for. The cost of the commitments will be included in the estimate result and discounts the commitment entitles will be included in the workload cost estimates. A maximum of 100 workloads can be provided."]
        #[serde(
            rename = "commitments",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub commitments: ::std::option::Option<Vec<crate::schemas::Commitment>>,
        #[doc = "Configuration for the scenario."]
        #[serde(
            rename = "scenarioConfig",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scenario_config: ::std::option::Option<crate::schemas::ScenarioConfig>,
        #[doc = "The Google Cloud usage whose costs are estimated. A maximum of 100 workloads can be provided."]
        #[serde(
            rename = "workloads",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub workloads: ::std::option::Option<Vec<crate::schemas::Workload>>,
    }
    impl ::google_field_selector::FieldSelector for CostScenario {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CostScenario {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CreditEstimate {
        #[doc = "The estimated credit amount."]
        #[serde(
            rename = "creditAmount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub credit_amount: ::std::option::Option<crate::schemas::Money>,
        #[doc = "The credit description."]
        #[serde(
            rename = "creditDescription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub credit_description: ::std::option::Option<String>,
        #[doc = "The credit type."]
        #[serde(
            rename = "creditType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub credit_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for CreditEstimate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CreditEstimate {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct CustomMachineType {
        #[doc = "Required. The machine series. Only certain [machine series](https://cloud.google.com/compute/docs/general-purpose-machines#custom_machine_types) support custom configurations. For example: “n1”."]
        #[serde(
            rename = "machineSeries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub machine_series: ::std::option::Option<String>,
        #[doc = "Required. Memory size of the VM in GB (2^30 bytes). Must be an increment of 0.25 (256 MB). Each [machine series](https://cloud.google.com/compute/docs/machine-types#machine_type_comparison) has limitations on allowed values for the ratio of memory-to-vCPU count."]
        #[serde(
            rename = "memorySizeGb",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub memory_size_gb: ::std::option::Option<f64>,
        #[doc = "Required. The number of vCPUs. The allowed values depend on the [machine series](https://cloud.google.com/compute/docs/machine-types#machine_type_comparison)."]
        #[serde(
            rename = "virtualCpuCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub virtual_cpu_count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for CustomMachineType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for CustomMachineType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DualRegional {
        #[doc = "The [location name](https://cloud.google.com/storage/docs/locations#available-locations) where the data is stored. For example: “asia1” for dual region."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for DualRegional {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DualRegional {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct EstimateCostScenarioForBillingAccountRequest {
        #[doc = "The scenario to estimate costs for."]
        #[serde(
            rename = "costScenario",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cost_scenario: ::std::option::Option<crate::schemas::CostScenario>,
    }
    impl ::google_field_selector::FieldSelector for EstimateCostScenarioForBillingAccountRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EstimateCostScenarioForBillingAccountRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct EstimateCostScenarioForBillingAccountResponse {
        #[doc = "The result of the cost estimation."]
        #[serde(
            rename = "costEstimationResult",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cost_estimation_result: ::std::option::Option<crate::schemas::CostEstimationResult>,
    }
    impl ::google_field_selector::FieldSelector for EstimateCostScenarioForBillingAccountResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EstimateCostScenarioForBillingAccountResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct EstimateCostScenarioWithListPriceRequest {
        #[doc = "The scenario to estimate costs for."]
        #[serde(
            rename = "costScenario",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cost_scenario: ::std::option::Option<crate::schemas::CostScenario>,
    }
    impl ::google_field_selector::FieldSelector for EstimateCostScenarioWithListPriceRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EstimateCostScenarioWithListPriceRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct EstimateCostScenarioWithListPriceResponse {
        #[doc = "The result of the cost estimation."]
        #[serde(
            rename = "costEstimationResult",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cost_estimation_result: ::std::option::Option<crate::schemas::CostEstimationResult>,
    }
    impl ::google_field_selector::FieldSelector for EstimateCostScenarioWithListPriceResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EstimateCostScenarioWithListPriceResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct EstimationTimePoint {
        #[doc = "The point in time, relative to the start of the time frame covered by the cost estimate."]
        #[serde(
            rename = "estimationTimeFrameOffset",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub estimation_time_frame_offset: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for EstimationTimePoint {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EstimationTimePoint {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GuestAccelerator {
        #[doc = "The number of the guest accelerator cards exposed to each instance."]
        #[serde(
            rename = "acceleratorCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub accelerator_count: ::std::option::Option<i64>,
        #[doc = "The type of the guest accelerator cards. For example: “nvidia-tesla-t4”."]
        #[serde(
            rename = "acceleratorType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub accelerator_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for GuestAccelerator {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for GuestAccelerator {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct MachineType {
        #[serde(
            rename = "customMachineType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub custom_machine_type: ::std::option::Option<crate::schemas::CustomMachineType>,
        #[serde(
            rename = "predefinedMachineType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub predefined_machine_type: ::std::option::Option<crate::schemas::PredefinedMachineType>,
    }
    impl ::google_field_selector::FieldSelector for MachineType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MachineType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Money {
        #[doc = "The three-letter currency code defined in ISO 4217."]
        #[serde(
            rename = "currencyCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency_code: ::std::option::Option<String>,
        #[doc = "Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000."]
        #[serde(
            rename = "nanos",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub nanos: ::std::option::Option<i32>,
        #[doc = "The whole units of the amount. For example if `currencyCode` is `\"USD\"`, then 1 unit is one US dollar."]
        #[serde(
            rename = "units",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub units: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for Money {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Money {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MultiRegional {
        #[doc = "The [location name](https://cloud.google.com/storage/docs/locations#available-locations) where the data is stored. For example: “us” for multi-region."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for MultiRegional {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for MultiRegional {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PersistentDisk {
        #[doc = "Specifies the size of disk. Must be at least 10 GB."]
        #[serde(
            rename = "diskSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disk_size: ::std::option::Option<crate::schemas::Usage>,
        #[doc = "The [disk type](https://cloud.google.com/compute/docs/disks#disk-types). For example: “pd-standard”."]
        #[serde(
            rename = "diskType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disk_type: ::std::option::Option<String>,
        #[doc = "Indicates how many IOPS to provision for the disk for extreme persistent disks. This sets the number of I/O operations per second that the disk can handle. Values must be between 10,000 and 120,000."]
        #[serde(
            rename = "provisionedIops",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub provisioned_iops: ::std::option::Option<crate::schemas::Usage>,
        #[doc = "The geographic scope of the disk. Defaults to `SCOPE_ZONAL` if not specified."]
        #[serde(
            rename = "scope",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scope: ::std::option::Option<crate::schemas::PersistentDiskScope>,
    }
    impl ::google_field_selector::FieldSelector for PersistentDisk {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PersistentDisk {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PersistentDiskScope {
        #[doc = "The disk is replicated in a secondary zone within the same region."]
        ScopeRegional,
        #[doc = "Unspecified."]
        ScopeUnspecified,
        #[doc = "The disk exists in a single zone."]
        ScopeZonal,
    }
    impl PersistentDiskScope {
        pub fn as_str(self) -> &'static str {
            match self {
                PersistentDiskScope::ScopeRegional => "SCOPE_REGIONAL",
                PersistentDiskScope::ScopeUnspecified => "SCOPE_UNSPECIFIED",
                PersistentDiskScope::ScopeZonal => "SCOPE_ZONAL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PersistentDiskScope {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PersistentDiskScope {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PersistentDiskScope, ()> {
            Ok(match s {
                "SCOPE_REGIONAL" => PersistentDiskScope::ScopeRegional,
                "SCOPE_UNSPECIFIED" => PersistentDiskScope::ScopeUnspecified,
                "SCOPE_ZONAL" => PersistentDiskScope::ScopeZonal,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PersistentDiskScope {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PersistentDiskScope {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PersistentDiskScope {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SCOPE_REGIONAL" => PersistentDiskScope::ScopeRegional,
                "SCOPE_UNSPECIFIED" => PersistentDiskScope::ScopeUnspecified,
                "SCOPE_ZONAL" => PersistentDiskScope::ScopeZonal,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PersistentDiskScope {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PersistentDiskScope {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PredefinedMachineType {
        #[doc = "The [machine type](https://cloud.google.com/compute/docs/machine-types). For example: “n1-standard1”."]
        #[serde(
            rename = "machineType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub machine_type: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PredefinedMachineType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PredefinedMachineType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct PremiumTierEgressWorkload {
        #[doc = "Where the data is sent to."]
        #[serde(
            rename = "destinationContinent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination_continent:
            ::std::option::Option<crate::schemas::PremiumTierEgressWorkloadDestinationContinent>,
        #[doc = "Premium Tier egress usage. Expected units such as “GiBy/s, By/s, etc.”"]
        #[serde(
            rename = "egressRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub egress_rate: ::std::option::Option<crate::schemas::Usage>,
        #[doc = "Which [region](https://cloud.google.com/compute/docs/regions-zones) the egress data comes from."]
        #[serde(
            rename = "sourceRegion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_region: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PremiumTierEgressWorkload {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PremiumTierEgressWorkload {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PremiumTierEgressWorkloadDestinationContinent {
        #[doc = "Africa."]
        DestinationContinentAfrica,
        #[doc = "Asia Pacific."]
        DestinationContinentAsiaPacific,
        #[doc = "Australia."]
        DestinationContinentAutralia,
        #[doc = "Central America."]
        DestinationContinentCentralAmerica,
        #[doc = "China."]
        DestinationContinentChina,
        #[doc = "Eastern Europe."]
        DestinationContinentEasternEurope,
        #[doc = "Other regions in Europe, Middle East and Africa."]
        DestinationContinentEmea,
        #[doc = "India"]
        DestinationContinentIndia,
        #[doc = "Middle East."]
        DestinationContinentMiddleEast,
        #[doc = "North America."]
        DestinationContinentNorthAmerica,
        #[doc = "South America."]
        DestinationContinentSouthAmerica,
        #[doc = "Not specified."]
        DestinationContinentUnspecified,
        #[doc = "Western Europe."]
        DestinationContinentWesternEurope,
    }
    impl PremiumTierEgressWorkloadDestinationContinent {
        pub fn as_str(self) -> &'static str {
            match self { PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentAfrica => "DESTINATION_CONTINENT_AFRICA" , PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentAsiaPacific => "DESTINATION_CONTINENT_ASIA_PACIFIC" , PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentAutralia => "DESTINATION_CONTINENT_AUTRALIA" , PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentCentralAmerica => "DESTINATION_CONTINENT_CENTRAL_AMERICA" , PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentChina => "DESTINATION_CONTINENT_CHINA" , PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentEasternEurope => "DESTINATION_CONTINENT_EASTERN_EUROPE" , PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentEmea => "DESTINATION_CONTINENT_EMEA" , PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentIndia => "DESTINATION_CONTINENT_INDIA" , PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentMiddleEast => "DESTINATION_CONTINENT_MIDDLE_EAST" , PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentNorthAmerica => "DESTINATION_CONTINENT_NORTH_AMERICA" , PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentSouthAmerica => "DESTINATION_CONTINENT_SOUTH_AMERICA" , PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentUnspecified => "DESTINATION_CONTINENT_UNSPECIFIED" , PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentWesternEurope => "DESTINATION_CONTINENT_WESTERN_EUROPE" , }
        }
    }
    impl ::std::convert::AsRef<str> for PremiumTierEgressWorkloadDestinationContinent {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PremiumTierEgressWorkloadDestinationContinent {
        type Err = ();
        fn from_str(
            s: &str,
        ) -> ::std::result::Result<PremiumTierEgressWorkloadDestinationContinent, ()> {
            Ok (match s { "DESTINATION_CONTINENT_AFRICA" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentAfrica , "DESTINATION_CONTINENT_ASIA_PACIFIC" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentAsiaPacific , "DESTINATION_CONTINENT_AUTRALIA" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentAutralia , "DESTINATION_CONTINENT_CENTRAL_AMERICA" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentCentralAmerica , "DESTINATION_CONTINENT_CHINA" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentChina , "DESTINATION_CONTINENT_EASTERN_EUROPE" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentEasternEurope , "DESTINATION_CONTINENT_EMEA" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentEmea , "DESTINATION_CONTINENT_INDIA" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentIndia , "DESTINATION_CONTINENT_MIDDLE_EAST" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentMiddleEast , "DESTINATION_CONTINENT_NORTH_AMERICA" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentNorthAmerica , "DESTINATION_CONTINENT_SOUTH_AMERICA" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentSouthAmerica , "DESTINATION_CONTINENT_UNSPECIFIED" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentUnspecified , "DESTINATION_CONTINENT_WESTERN_EUROPE" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentWesternEurope , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for PremiumTierEgressWorkloadDestinationContinent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PremiumTierEgressWorkloadDestinationContinent {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PremiumTierEgressWorkloadDestinationContinent {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "DESTINATION_CONTINENT_AFRICA" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentAfrica , "DESTINATION_CONTINENT_ASIA_PACIFIC" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentAsiaPacific , "DESTINATION_CONTINENT_AUTRALIA" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentAutralia , "DESTINATION_CONTINENT_CENTRAL_AMERICA" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentCentralAmerica , "DESTINATION_CONTINENT_CHINA" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentChina , "DESTINATION_CONTINENT_EASTERN_EUROPE" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentEasternEurope , "DESTINATION_CONTINENT_EMEA" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentEmea , "DESTINATION_CONTINENT_INDIA" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentIndia , "DESTINATION_CONTINENT_MIDDLE_EAST" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentMiddleEast , "DESTINATION_CONTINENT_NORTH_AMERICA" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentNorthAmerica , "DESTINATION_CONTINENT_SOUTH_AMERICA" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentSouthAmerica , "DESTINATION_CONTINENT_UNSPECIFIED" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentUnspecified , "DESTINATION_CONTINENT_WESTERN_EUROPE" => PremiumTierEgressWorkloadDestinationContinent :: DestinationContinentWesternEurope , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector for PremiumTierEgressWorkloadDestinationContinent {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PremiumTierEgressWorkloadDestinationContinent {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Price {
        #[doc = "The timestamp within the estimation time frame when the price was set."]
        #[serde(
            rename = "effectiveTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub effective_time: ::std::option::Option<crate::schemas::EstimationTimePoint>,
        #[doc = "The type of price. Possible values: “RATE”"]
        #[serde(
            rename = "priceType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub price_type: ::std::option::Option<String>,
        #[doc = "A set of tiered rates."]
        #[serde(
            rename = "rate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rate: ::std::option::Option<crate::schemas::Rate>,
    }
    impl ::google_field_selector::FieldSelector for Price {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Price {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Rate {
        #[doc = "The service tiers."]
        #[serde(
            rename = "tiers",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub tiers: ::std::option::Option<Vec<crate::schemas::RateTier>>,
        #[doc = "The SKU’s pricing unit. For example, if the tier price is $1 per 1000000 Bytes, then this field will show ‘By’. The `start_amount` field in each tier will be in this unit."]
        #[serde(
            rename = "unit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unit: ::std::option::Option<String>,
        #[doc = "The SKU’s count for the pricing unit. For example, if the tier price is $1 per 1000000 Bytes, then this column will show 1000000."]
        #[serde(
            rename = "unitCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unit_count: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for Rate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Rate {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct RateTier {
        #[doc = "The price for this tier."]
        #[serde(
            rename = "price",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub price: ::std::option::Option<crate::schemas::Money>,
        #[doc = "The magnitude of usage in which the tier interval begins. Example: “From 100 GiBi the price is $1 per byte” implies `start_amount` = 100"]
        #[serde(
            rename = "startAmount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_amount: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for RateTier {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RateTier {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Regional {
        #[doc = "The [location name](https://cloud.google.com/storage/docs/locations#available-locations). For example: “us-central1” for region."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Regional {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Regional {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ScenarioConfig {
        #[doc = "Time frame for the estimate. Workloads must specify usage for this duration. Duration must be at least 1 hour (3,600 seconds) and at most 10 years (315,360,000 seconds). The calculations for years and months are based on a 730-hour (2,628,000-second) month. For durations longer than one month (2,628,000 seconds), the duration is rounded up to the next month, so the estimate shows you the costs for full months. For example, a duration of 3,232,800 seconds (roughly 5 weeks) is rounded up to 2 months."]
        #[serde(
            rename = "estimateDuration",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub estimate_duration: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ScenarioConfig {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ScenarioConfig {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SegmentCostEstimate {
        #[doc = "Estimated costs for each commitment."]
        #[serde(
            rename = "commitmentCostEstimates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub commitment_cost_estimates:
            ::std::option::Option<Vec<crate::schemas::CommitmentCostEstimate>>,
        #[doc = "Timestamp for the start of the segment."]
        #[serde(
            rename = "segmentStartTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment_start_time: ::std::option::Option<crate::schemas::EstimationTimePoint>,
        #[doc = "Total estimated costs for the time segment."]
        #[serde(
            rename = "segmentTotalCostEstimate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub segment_total_cost_estimate: ::std::option::Option<crate::schemas::CostEstimate>,
        #[doc = "Estimated costs for each workload."]
        #[serde(
            rename = "workloadCostEstimates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub workload_cost_estimates:
            ::std::option::Option<Vec<crate::schemas::WorkloadCostEstimate>>,
    }
    impl ::google_field_selector::FieldSelector for SegmentCostEstimate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SegmentCostEstimate {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Sku {
        #[doc = "The display name for the SKU. Example: A2 Instance Core running in Americas"]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "A timeline of prices for a SKU in chronological order. Note: The API currently only supports using a constant price for the entire estimation time frame so this list will contain a single value."]
        #[serde(
            rename = "prices",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub prices: ::std::option::Option<Vec<crate::schemas::Price>>,
        #[doc = "The resource name for the SKU. Example: “services/DA34-426B-A397/skus/AA95-CD31-42FE”"]
        #[serde(
            rename = "sku",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sku: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Sku {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Sku {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SkuCostEstimate {
        #[doc = "The estimated cost for the usage on this SKU."]
        #[serde(
            rename = "costEstimate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cost_estimate: ::std::option::Option<crate::schemas::CostEstimate>,
        #[doc = "The resource name for the SKU. Example: “services/DA34-426B-A397/skus/AA95-CD31-42FE” More information about the SKU can be found in the `skus` field of the `CostEstimationResult`."]
        #[serde(
            rename = "sku",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sku: ::std::option::Option<String>,
        #[doc = "The amount of usage on this SKU."]
        #[serde(
            rename = "usageAmount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub usage_amount: ::std::option::Option<f64>,
        #[doc = "The unit for the usage on this SKU."]
        #[serde(
            rename = "usageUnit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub usage_unit: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SkuCostEstimate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SkuCostEstimate {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct StandardTierEgressWorkload {
        #[doc = "Standard tier egress usage. Expected units such as “GiBy/s, By/s, etc.”"]
        #[serde(
            rename = "egressRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub egress_rate: ::std::option::Option<crate::schemas::Usage>,
        #[doc = "Which [region](https://cloud.google.com/compute/docs/regions-zones) the egress data comes from."]
        #[serde(
            rename = "sourceRegion",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source_region: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for StandardTierEgressWorkload {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for StandardTierEgressWorkload {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Usage {
        #[doc = "A timeline of usage rates over the estimate interval."]
        #[serde(
            rename = "usageRateTimeline",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub usage_rate_timeline: ::std::option::Option<crate::schemas::UsageRateTimeline>,
    }
    impl ::google_field_selector::FieldSelector for Usage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Usage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UsageRateTimeline {
        #[doc = "The unit for the usage rate in each timeline entry. If you provide an incorrect unit for an instance, the correct unit is provided in the error message. The supported units are a subset of [The Unified Code for Units of Measure](https://ucum.org/ucum.html) standard: * **Time units (TIME-UNIT)** * `s` second * `min` minute * `h` hour * `d` day * `wk` week * `mo` month * `yr` year * `ms` millisecond * `us` microsecond * `ns` nanosecond * **Basic storage units (BASIC-STORAGE-UNIT)** * `bit` bit * `By` byte * **Count units (COUNT-UNIT)** * `count` count * **Prefixes (PREFIX)** * `k` kilo (10^3) * `M` mega (10^6) * `G` giga (10^9) * `T` tera (10^12) * `P` peta (10^15) * `Ki` kibi (2^10) * `Mi` mebi (2^20) * `Gi` gibi (2^30) * `Ti` tebi (2^40) * `Pi` pebi (2^50) **Grammar** The grammar also includes these connectors: * `/` division or ratio (as an infix operator). For example: `kBy/{email}` or `MiBy/10ms`. * `.` multiplication or composition (as an infix operator). For example: `GBy.d` or `k{watt}.h`. The grammar for a unit is as follows: `Expression = Component { \".\" Component } { \"/\" Component } ; Component = ( [ PREFIX ] UNIT | \"%\" ) [ Annotation ] | Annotation | \"1\" ; UNIT = TIME-UNIT | STORAGE-UNIT | DATA-UNIT | COUNT-UNIT Annotation = \"{\" NAME \"}\" ;` Examples: * Request per second: `1/s` or `{requests}/s` * GibiBytes: `GiBy` * GibiBytes * seconds: `GiBy.s`"]
        #[serde(
            rename = "unit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unit: ::std::option::Option<String>,
        #[doc = "The timeline entries. Each entry has a start time and usage rate. The start time specifies the effective time of the usage rate. The entries must be sorted by start time in an increasing order."]
        #[serde(
            rename = "usageRateTimelineEntries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub usage_rate_timeline_entries:
            ::std::option::Option<Vec<crate::schemas::UsageRateTimelineEntry>>,
    }
    impl ::google_field_selector::FieldSelector for UsageRateTimeline {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UsageRateTimeline {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct UsageRateTimelineEntry {
        #[doc = "The effective time for this entry. The usage rate is in effect starting at this time until the effective time of the subsequent entry in the timeline. The last entry defines the usage rate until the end of the `Usage` time frame. Must correspond to an integer number of hours."]
        #[serde(
            rename = "effectiveTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub effective_time: ::std::option::Option<crate::schemas::EstimationTimePoint>,
        #[doc = "The usage rate."]
        #[serde(
            rename = "usageRate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub usage_rate: ::std::option::Option<f64>,
    }
    impl ::google_field_selector::FieldSelector for UsageRateTimelineEntry {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for UsageRateTimelineEntry {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct VlanAttachment {
        #[doc = "Capacities in the [pricing table](https://cloud.google.com/vpc/network-pricing#interconnect-pricing) Examples of capacity are: 50/100/200/300/400/500-Mbps, 1/2/5/10/20/50-Gbps."]
        #[serde(
            rename = "bandwidth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub bandwidth: ::std::option::Option<crate::schemas::VlanAttachmentBandwidth>,
        #[doc = "VLAN usage. This is specified as a unitless quantity which indicates the number of VLAN attachment used in interconnect."]
        #[serde(
            rename = "vlanCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vlan_count: ::std::option::Option<crate::schemas::Usage>,
    }
    impl ::google_field_selector::FieldSelector for VlanAttachment {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VlanAttachment {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum VlanAttachmentBandwidth {
        #[doc = "100 Mbit/s"]
        BandwidthBps100M,
        #[doc = "10 Gbit/s"]
        BandwidthBps10G,
        #[doc = "1 Gbit/s"]
        BandwidthBps1G,
        #[doc = "200 Mbit/s"]
        BandwidthBps200M,
        #[doc = "20 Gbit/s"]
        BandwidthBps20G,
        #[doc = "2 Gbit/s"]
        BandwidthBps2G,
        #[doc = "300 Mbit/s"]
        BandwidthBps300M,
        #[doc = "400 Mbit/s"]
        BandwidthBps400M,
        #[doc = "500 Mbit/s"]
        BandwidthBps500M,
        #[doc = "50 Gbit/s"]
        BandwidthBps50G,
        #[doc = "50 Mbit/s"]
        BandwidthBps50M,
        #[doc = "5 Gbit/s"]
        BandwidthBps5G,
        #[doc = "Unspecified."]
        BandwidthUnspecified,
    }
    impl VlanAttachmentBandwidth {
        pub fn as_str(self) -> &'static str {
            match self {
                VlanAttachmentBandwidth::BandwidthBps100M => "BANDWIDTH_BPS_100M",
                VlanAttachmentBandwidth::BandwidthBps10G => "BANDWIDTH_BPS_10G",
                VlanAttachmentBandwidth::BandwidthBps1G => "BANDWIDTH_BPS_1G",
                VlanAttachmentBandwidth::BandwidthBps200M => "BANDWIDTH_BPS_200M",
                VlanAttachmentBandwidth::BandwidthBps20G => "BANDWIDTH_BPS_20G",
                VlanAttachmentBandwidth::BandwidthBps2G => "BANDWIDTH_BPS_2G",
                VlanAttachmentBandwidth::BandwidthBps300M => "BANDWIDTH_BPS_300M",
                VlanAttachmentBandwidth::BandwidthBps400M => "BANDWIDTH_BPS_400M",
                VlanAttachmentBandwidth::BandwidthBps500M => "BANDWIDTH_BPS_500M",
                VlanAttachmentBandwidth::BandwidthBps50G => "BANDWIDTH_BPS_50G",
                VlanAttachmentBandwidth::BandwidthBps50M => "BANDWIDTH_BPS_50M",
                VlanAttachmentBandwidth::BandwidthBps5G => "BANDWIDTH_BPS_5G",
                VlanAttachmentBandwidth::BandwidthUnspecified => "BANDWIDTH_UNSPECIFIED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for VlanAttachmentBandwidth {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for VlanAttachmentBandwidth {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<VlanAttachmentBandwidth, ()> {
            Ok(match s {
                "BANDWIDTH_BPS_100M" => VlanAttachmentBandwidth::BandwidthBps100M,
                "BANDWIDTH_BPS_10G" => VlanAttachmentBandwidth::BandwidthBps10G,
                "BANDWIDTH_BPS_1G" => VlanAttachmentBandwidth::BandwidthBps1G,
                "BANDWIDTH_BPS_200M" => VlanAttachmentBandwidth::BandwidthBps200M,
                "BANDWIDTH_BPS_20G" => VlanAttachmentBandwidth::BandwidthBps20G,
                "BANDWIDTH_BPS_2G" => VlanAttachmentBandwidth::BandwidthBps2G,
                "BANDWIDTH_BPS_300M" => VlanAttachmentBandwidth::BandwidthBps300M,
                "BANDWIDTH_BPS_400M" => VlanAttachmentBandwidth::BandwidthBps400M,
                "BANDWIDTH_BPS_500M" => VlanAttachmentBandwidth::BandwidthBps500M,
                "BANDWIDTH_BPS_50G" => VlanAttachmentBandwidth::BandwidthBps50G,
                "BANDWIDTH_BPS_50M" => VlanAttachmentBandwidth::BandwidthBps50M,
                "BANDWIDTH_BPS_5G" => VlanAttachmentBandwidth::BandwidthBps5G,
                "BANDWIDTH_UNSPECIFIED" => VlanAttachmentBandwidth::BandwidthUnspecified,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for VlanAttachmentBandwidth {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for VlanAttachmentBandwidth {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for VlanAttachmentBandwidth {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BANDWIDTH_BPS_100M" => VlanAttachmentBandwidth::BandwidthBps100M,
                "BANDWIDTH_BPS_10G" => VlanAttachmentBandwidth::BandwidthBps10G,
                "BANDWIDTH_BPS_1G" => VlanAttachmentBandwidth::BandwidthBps1G,
                "BANDWIDTH_BPS_200M" => VlanAttachmentBandwidth::BandwidthBps200M,
                "BANDWIDTH_BPS_20G" => VlanAttachmentBandwidth::BandwidthBps20G,
                "BANDWIDTH_BPS_2G" => VlanAttachmentBandwidth::BandwidthBps2G,
                "BANDWIDTH_BPS_300M" => VlanAttachmentBandwidth::BandwidthBps300M,
                "BANDWIDTH_BPS_400M" => VlanAttachmentBandwidth::BandwidthBps400M,
                "BANDWIDTH_BPS_500M" => VlanAttachmentBandwidth::BandwidthBps500M,
                "BANDWIDTH_BPS_50G" => VlanAttachmentBandwidth::BandwidthBps50G,
                "BANDWIDTH_BPS_50M" => VlanAttachmentBandwidth::BandwidthBps50M,
                "BANDWIDTH_BPS_5G" => VlanAttachmentBandwidth::BandwidthBps5G,
                "BANDWIDTH_UNSPECIFIED" => VlanAttachmentBandwidth::BandwidthUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for VlanAttachmentBandwidth {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VlanAttachmentBandwidth {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct VmResourceBasedCud {
        #[doc = "Guest accelerator, known as GPU."]
        #[serde(
            rename = "guestAccelerator",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub guest_accelerator: ::std::option::Option<crate::schemas::GuestAccelerator>,
        #[doc = "The machine series for CUD. For example: “n1” for general purpose N1 machine type commitments. “n2” for general purpose N2 machine type commitments. “e2” for general purpose E2 machine type commitments. “n2d” for general purpose N2D machine type commitments. “t2d” for general purpose T2D machine type commitments. “c2”/“c2d” for compute-optimized commitments. “m1”/“m2” for the memory-optimized commitments. “a2’ for the accelerator-optimized commitments."]
        #[serde(
            rename = "machineSeries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub machine_series: ::std::option::Option<String>,
        #[doc = "Memory size of the VM in GB (2^30 bytes). Must be an increment of 0.25 (256 MB)."]
        #[serde(
            rename = "memorySizeGb",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub memory_size_gb: ::std::option::Option<f64>,
        #[doc = "Commitment usage plan."]
        #[serde(
            rename = "plan",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub plan: ::std::option::Option<crate::schemas::VmResourceBasedCudPlan>,
        #[doc = "The region where the VM runs. For example: “us-central1”"]
        #[serde(
            rename = "region",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub region: ::std::option::Option<String>,
        #[doc = "The number of vCPUs. The number of vCPUs must be an integer of 0 or more and can be even or odd."]
        #[serde(
            rename = "virtualCpuCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub virtual_cpu_count: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for VmResourceBasedCud {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VmResourceBasedCud {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum VmResourceBasedCudPlan {
        #[doc = "Not specified commitment plan."]
        CommitmentPlanUnspecified,
        #[doc = "3 years commitment."]
        ThirtySixMonth,
        #[doc = "1 year commitment."]
        TwelveMonth,
    }
    impl VmResourceBasedCudPlan {
        pub fn as_str(self) -> &'static str {
            match self {
                VmResourceBasedCudPlan::CommitmentPlanUnspecified => "COMMITMENT_PLAN_UNSPECIFIED",
                VmResourceBasedCudPlan::ThirtySixMonth => "THIRTY_SIX_MONTH",
                VmResourceBasedCudPlan::TwelveMonth => "TWELVE_MONTH",
            }
        }
    }
    impl ::std::convert::AsRef<str> for VmResourceBasedCudPlan {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for VmResourceBasedCudPlan {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<VmResourceBasedCudPlan, ()> {
            Ok(match s {
                "COMMITMENT_PLAN_UNSPECIFIED" => VmResourceBasedCudPlan::CommitmentPlanUnspecified,
                "THIRTY_SIX_MONTH" => VmResourceBasedCudPlan::ThirtySixMonth,
                "TWELVE_MONTH" => VmResourceBasedCudPlan::TwelveMonth,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for VmResourceBasedCudPlan {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for VmResourceBasedCudPlan {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for VmResourceBasedCudPlan {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "COMMITMENT_PLAN_UNSPECIFIED" => VmResourceBasedCudPlan::CommitmentPlanUnspecified,
                "THIRTY_SIX_MONTH" => VmResourceBasedCudPlan::ThirtySixMonth,
                "TWELVE_MONTH" => VmResourceBasedCudPlan::TwelveMonth,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for VmResourceBasedCudPlan {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VmResourceBasedCudPlan {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Workload {
        #[doc = "Usage on Google Cloud CDN Egress."]
        #[serde(
            rename = "cloudCdnEgressWorkload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cloud_cdn_egress_workload:
            ::std::option::Option<crate::schemas::CloudCdnEgressWorkload>,
        #[doc = "Usage on Google Cloud CDN."]
        #[serde(
            rename = "cloudCdnWorkload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cloud_cdn_workload: ::std::option::Option<crate::schemas::CloudCdnWorkload>,
        #[doc = "Usage on Google Cloud Interconnect Egress."]
        #[serde(
            rename = "cloudInterconnectEgressWorkload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cloud_interconnect_egress_workload:
            ::std::option::Option<crate::schemas::CloudInterconnectEgressWorkload>,
        #[doc = "Usage on Google Cloud Interconnect."]
        #[serde(
            rename = "cloudInterconnectWorkload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cloud_interconnect_workload:
            ::std::option::Option<crate::schemas::CloudInterconnectWorkload>,
        #[doc = "Usage on a cloud storage egress."]
        #[serde(
            rename = "cloudStorageEgressWorkload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cloud_storage_egress_workload:
            ::std::option::Option<crate::schemas::CloudStorageEgressWorkload>,
        #[doc = "Usage on Google Cloud Storage."]
        #[serde(
            rename = "cloudStorageWorkload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cloud_storage_workload: ::std::option::Option<crate::schemas::CloudStorageWorkload>,
        #[doc = "Usage of a Google Compute Engine Virtual Machine."]
        #[serde(
            rename = "computeVmWorkload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub compute_vm_workload: ::std::option::Option<crate::schemas::ComputeVmWorkload>,
        #[doc = "Required. A name for this workload. All workloads in a `CostScenario` must have a unique `name`. Each `name` may be at most 128 characters long."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Usage on Premium Tier Internet Egress."]
        #[serde(
            rename = "premiumTierEgressWorkload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub premium_tier_egress_workload:
            ::std::option::Option<crate::schemas::PremiumTierEgressWorkload>,
        #[doc = "Usage on Standard Tier Internet Egress."]
        #[serde(
            rename = "standardTierEgressWorkload",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub standard_tier_egress_workload:
            ::std::option::Option<crate::schemas::StandardTierEgressWorkload>,
    }
    impl ::google_field_selector::FieldSelector for Workload {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Workload {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct WorkloadCostEstimate {
        #[doc = "The name of the workload, as specified in the `CostScenario`."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Estimated costs for each SKU in the workload."]
        #[serde(
            rename = "skuCostEstimates",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sku_cost_estimates: ::std::option::Option<Vec<crate::schemas::SkuCostEstimate>>,
        #[doc = "Total estimated costs for the workload."]
        #[serde(
            rename = "workloadTotalCostEstimate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub workload_total_cost_estimate: ::std::option::Option<crate::schemas::CostEstimate>,
    }
    impl ::google_field_selector::FieldSelector for WorkloadCostEstimate {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for WorkloadCostEstimate {
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
    #[doc = "Actions that can be performed on the billing_accounts resource"]
    pub fn billing_accounts(&self) -> crate::resources::billing_accounts::BillingAccountsActions {
        crate::resources::billing_accounts::BillingAccountsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the v_1beta resource"]
    pub fn v_1beta(&self) -> crate::resources::v_1beta::V1BetaActions {
        crate::resources::v_1beta::V1BetaActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod billing_accounts {
        pub mod params {}
        pub struct BillingAccountsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> BillingAccountsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Use custom pricing in the estimate, using a `CostScenario` with a defined `billingAccount`."]
            pub fn estimate_cost_scenario(
                &self,
                request: crate::schemas::EstimateCostScenarioForBillingAccountRequest,
                billing_account: impl Into<String>,
            ) -> EstimateCostScenarioRequestBuilder {
                EstimateCostScenarioRequestBuilder {
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
                    billing_account: billing_account.into(),
                }
            }
        }
        #[doc = "Created via [BillingAccountsActions::estimate_cost_scenario()](struct.BillingAccountsActions.html#method.estimate_cost_scenario)"]
        #[derive(Debug, Clone)]
        pub struct EstimateCostScenarioRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::EstimateCostScenarioForBillingAccountRequest,
            billing_account: String,
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
        impl<'a> EstimateCostScenarioRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::EstimateCostScenarioForBillingAccountResponse, crate::Error>
            {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::EstimateCostScenarioForBillingAccountResponse, crate::Error>
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
                let req = req.json(&self.request);
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://cloudbilling.googleapis.com/".to_owned();
                output.push_str("v1beta/");
                {
                    let var_as_str = &self.billing_account;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::RESERVED,
                    ));
                }
                output.push_str(":estimateCostScenario");
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
    pub mod v_1beta {
        pub mod params {}
        pub struct V1BetaActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> V1BetaActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Estimate list prices using a `CostScenario` without a defined `billingAccount`."]
            pub fn estimate_cost_scenario(
                &self,
                request: crate::schemas::EstimateCostScenarioWithListPriceRequest,
            ) -> EstimateCostScenarioRequestBuilder {
                EstimateCostScenarioRequestBuilder {
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
        #[doc = "Created via [V1BetaActions::estimate_cost_scenario()](struct.V1BetaActions.html#method.estimate_cost_scenario)"]
        #[derive(Debug, Clone)]
        pub struct EstimateCostScenarioRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::EstimateCostScenarioWithListPriceRequest,
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
        impl<'a> EstimateCostScenarioRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::EstimateCostScenarioWithListPriceResponse, crate::Error>
            {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::EstimateCostScenarioWithListPriceResponse, crate::Error>
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
                let req = req.json(&self.request);
                Ok(req.send().await?.error_for_status()?.json().await?)
            }
            fn _path(&self) -> String {
                let mut output = "https://cloudbilling.googleapis.com/".to_owned();
                output.push_str("v1beta:estimateCostScenario");
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
