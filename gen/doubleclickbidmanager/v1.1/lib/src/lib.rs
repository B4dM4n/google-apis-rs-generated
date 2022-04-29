#![doc = "# Resources and Methods\n* [queries](resources/queries/struct.QueriesActions.html)\n  * [*createquery*](resources/queries/struct.CreatequeryRequestBuilder.html), [*deletequery*](resources/queries/struct.DeletequeryRequestBuilder.html), [*getquery*](resources/queries/struct.GetqueryRequestBuilder.html), [*listqueries*](resources/queries/struct.ListqueriesRequestBuilder.html), [*runquery*](resources/queries/struct.RunqueryRequestBuilder.html)\n* [reports](resources/reports/struct.ReportsActions.html)\n  * [*listreports*](resources/reports/struct.ListreportsRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "View and manage your reports in DoubleClick Bid Manager\n\n`https://www.googleapis.com/auth/doubleclickbidmanager`"]
    pub const DOUBLECLICKBIDMANAGER: &str = "https://www.googleapis.com/auth/doubleclickbidmanager";
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
    pub struct ChannelGrouping {
        #[doc = "The name to apply to an event that does not match any of the rules in the channel grouping."]
        #[serde(
            rename = "fallbackName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub fallback_name: ::std::option::Option<String>,
        #[doc = "Channel Grouping name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Rules within Channel Grouping. There is a limit of 100 rules that can be set per channel grouping."]
        #[serde(
            rename = "rules",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rules: ::std::option::Option<Vec<crate::schemas::Rule>>,
    }
    impl ::google_field_selector::FieldSelector for ChannelGrouping {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ChannelGrouping {
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
    pub struct DisjunctiveMatchStatement {
        #[doc = "Filters. There is a limit of 100 filters that can be set per disjunctive match statement."]
        #[serde(
            rename = "eventFilters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub event_filters: ::std::option::Option<Vec<crate::schemas::EventFilter>>,
    }
    impl ::google_field_selector::FieldSelector for DisjunctiveMatchStatement {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DisjunctiveMatchStatement {
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
    pub struct EventFilter {
        #[doc = "Filter on a dimension."]
        #[serde(
            rename = "dimensionFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dimension_filter: ::std::option::Option<crate::schemas::PathQueryOptionsFilter>,
    }
    impl ::google_field_selector::FieldSelector for EventFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for EventFilter {
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
    pub struct FilterPair {
        #[doc = "Filter type."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::FilterPairType>,
        #[doc = "Filter value."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for FilterPair {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FilterPair {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FilterPairType {
        FilterActiveViewCustomMetricId,
        FilterActiveViewCustomMetricName,
        FilterActiveViewExpectedViewability,
        FilterAdPosition,
        FilterAdType,
        FilterAdvertiser,
        FilterAdvertiserCurrency,
        FilterAdvertiserIntegrationCode,
        FilterAdvertiserIntegrationStatus,
        FilterAdvertiserName,
        FilterAdvertiserTimezone,
        FilterAge,
        FilterAlgorithm,
        FilterAlgorithmId,
        FilterAmpPageRequest,
        FilterAnonymousInventoryModeling,
        FilterAppUrl,
        FilterAppUrlExcluded,
        FilterAttributedUserlist,
        FilterAttributedUserlistCost,
        FilterAttributedUserlistType,
        FilterAttributionModel,
        FilterAudienceList,
        FilterAudienceListCost,
        FilterAudienceListType,
        FilterAudienceName,
        FilterAudienceType,
        FilterAudioFeedTypeName,
        FilterAuthorizedSellerState,
        FilterBillableOutcome,
        FilterBrandLiftType,
        FilterBrowser,
        FilterBudgetSegmentBudget,
        FilterBudgetSegmentDescription,
        FilterBudgetSegmentEndDate,
        FilterBudgetSegmentPacingPercentage,
        FilterBudgetSegmentStartDate,
        FilterBudgetSegmentType,
        FilterCampaignDailyFrequency,
        FilterCarrier,
        FilterCarrierName,
        FilterChannelGrouping,
        FilterChannelId,
        FilterChannelName,
        FilterChannelType,
        FilterCity,
        FilterCityName,
        FilterCm360PlacementId,
        FilterCmPlacementId,
        FilterCompanionCreativeId,
        FilterCompanionCreativeName,
        FilterConversionDelay,
        FilterConversionSource,
        FilterConversionSourceId,
        FilterCountry,
        FilterCountryId,
        FilterCreative,
        FilterCreativeAsset,
        FilterCreativeAttribute,
        FilterCreativeHeight,
        FilterCreativeId,
        FilterCreativeIntegrationCode,
        FilterCreativeRenderedInAmp,
        FilterCreativeSize,
        FilterCreativeSource,
        FilterCreativeStatus,
        FilterCreativeType,
        FilterCreativeWidth,
        FilterDataProvider,
        FilterDataProviderName,
        FilterDataSource,
        FilterDate,
        FilterDayOfWeek,
        FilterDetailedDemographics,
        FilterDetailedDemographicsId,
        FilterDevice,
        FilterDeviceMake,
        FilterDeviceModel,
        FilterDeviceType,
        FilterDfpOrderId,
        FilterDigitalContentLabel,
        FilterDma,
        FilterDmaName,
        FilterDomain,
        FilterEligibleCookiesOnFirstPartyAudienceList,
        FilterEligibleCookiesOnThirdPartyAudienceListAndInterest,
        FilterEventType,
        FilterExchange,
        FilterExchangeCode,
        FilterExchangeId,
        FilterExtension,
        FilterExtensionStatus,
        FilterExtensionType,
        FilterFirstPartyAudienceListCost,
        FilterFirstPartyAudienceListType,
        FilterFloodlightActivity,
        FilterFloodlightActivityId,
        FilterFormat,
        FilterGamInsertionOrder,
        FilterGamLineItem,
        FilterGamLineItemId,
        FilterGender,
        FilterGmailAge,
        FilterGmailCity,
        FilterGmailCountry,
        FilterGmailCountryName,
        FilterGmailDeviceType,
        FilterGmailDeviceTypeName,
        FilterGmailGender,
        FilterGmailRegion,
        FilterGmailRemarketingList,
        FilterHouseholdIncome,
        FilterImpressionCountingMethod,
        FilterImpressionLossRejectionReason,
        FilterInsertionOrder,
        FilterInsertionOrderGoalType,
        FilterInsertionOrderGoalValue,
        FilterInsertionOrderIntegrationCode,
        FilterInsertionOrderName,
        FilterInsertionOrderStatus,
        FilterInterest,
        FilterInventoryCommitmentType,
        FilterInventoryDeliveryMethod,
        FilterInventoryFormat,
        FilterInventoryRateType,
        FilterInventorySource,
        FilterInventorySourceExternalId,
        FilterInventorySourceGroup,
        FilterInventorySourceGroupId,
        FilterInventorySourceId,
        FilterInventorySourceName,
        FilterInventorySourceType,
        FilterKeyword,
        FilterLifeEvent,
        FilterLifeEvents,
        FilterLineItem,
        FilterLineItemBudget,
        FilterLineItemDailyFrequency,
        FilterLineItemEndDate,
        FilterLineItemIntegrationCode,
        FilterLineItemLifetimeFrequency,
        FilterLineItemName,
        FilterLineItemPacingPercentage,
        FilterLineItemStartDate,
        FilterLineItemStatus,
        FilterLineItemType,
        FilterMatchRatio,
        FilterMatchedGenreTarget,
        FilterMeasurementSource,
        FilterMediaPlan,
        FilterMediaPlanName,
        FilterMediaType,
        FilterMobileGeo,
        FilterMonth,
        FilterMraidSupport,
        FilterNielsenAge,
        FilterNielsenCountryCode,
        FilterNielsenDateRange,
        FilterNielsenDeviceId,
        FilterNielsenGender,
        FilterNielsenRestatementDate,
        FilterNotSupported,
        FilterOmSdkAvailable,
        FilterOmidCapable,
        FilterOrderId,
        FilterOs,
        FilterPageCategory,
        FilterPageLayout,
        FilterParentalStatus,
        FilterPartner,
        FilterPartnerCurrency,
        FilterPartnerName,
        FilterPartnerStatus,
        FilterPathEventIndex,
        FilterPathPatternId,
        FilterPlacementAllYoutubeChannels,
        FilterPlacementNameAllYoutubeChannels,
        FilterPlatform,
        FilterPlaybackMethod,
        FilterPositionInContent,
        FilterPublicInventory,
        FilterPublisherProperty,
        FilterPublisherPropertyId,
        FilterPublisherPropertySection,
        FilterPublisherPropertySectionId,
        FilterQuarter,
        FilterRefundReason,
        FilterRegion,
        FilterRegionName,
        FilterRemarketingList,
        FilterRewarded,
        FilterSensitiveCategory,
        FilterServedPixelDensity,
        FilterSiteId,
        FilterSiteLanguage,
        FilterSkippableSupport,
        FilterTargetedDataProviders,
        FilterTargetedUserList,
        FilterThirdPartyAudienceListCost,
        FilterThirdPartyAudienceListType,
        FilterTimeOfDay,
        FilterTrueviewAd,
        FilterTrueviewAdGroup,
        FilterTrueviewAdGroupAdId,
        FilterTrueviewAdGroupId,
        FilterTrueviewAdTypeName,
        FilterTrueviewAge,
        FilterTrueviewCategory,
        FilterTrueviewCity,
        FilterTrueviewClickTypeName,
        FilterTrueviewConversionType,
        FilterTrueviewCountry,
        FilterTrueviewCustomAffinity,
        FilterTrueviewDetailedDemographics,
        FilterTrueviewDetailedDemographicsId,
        FilterTrueviewDma,
        FilterTrueviewDmaName,
        FilterTrueviewGender,
        FilterTrueviewHouseholdIncome,
        FilterTrueviewIarAge,
        FilterTrueviewIarCategory,
        FilterTrueviewIarCity,
        FilterTrueviewIarCountry,
        FilterTrueviewIarCountryName,
        FilterTrueviewIarGender,
        FilterTrueviewIarInterest,
        FilterTrueviewIarLanguage,
        FilterTrueviewIarParentalStatus,
        FilterTrueviewIarRegion,
        FilterTrueviewIarRegionName,
        FilterTrueviewIarRemarketingList,
        FilterTrueviewIarTimeOfDay,
        FilterTrueviewIarYoutubeChannel,
        FilterTrueviewIarYoutubeVideo,
        FilterTrueviewIarZipcode,
        FilterTrueviewInterest,
        FilterTrueviewKeyword,
        FilterTrueviewParentalStatus,
        FilterTrueviewPlacement,
        FilterTrueviewPlacementId,
        FilterTrueviewRegion,
        FilterTrueviewRegionName,
        FilterTrueviewRemarketingList,
        FilterTrueviewRemarketingListName,
        FilterTrueviewUrl,
        FilterTrueviewZipcode,
        FilterUnknown,
        FilterUserList,
        FilterUserListFirstParty,
        FilterUserListFirstPartyName,
        FilterUserListThirdParty,
        FilterUserListThirdPartyName,
        FilterVariantId,
        FilterVariantName,
        FilterVariantVersion,
        FilterVendorMeasurementMode,
        FilterVerificationAudibilityComplete,
        FilterVerificationAudibilityStart,
        FilterVerificationVideoPlayerSize,
        FilterVerificationVideoPlayerSizeComplete,
        FilterVerificationVideoPlayerSizeFirstQuartile,
        FilterVerificationVideoPlayerSizeMidPoint,
        FilterVerificationVideoPlayerSizeStart,
        FilterVerificationVideoPlayerSizeThirdQuartile,
        FilterVerificationVideoPosition,
        FilterVerificationVideoResized,
        FilterVideoAdPositionInStream,
        FilterVideoCompanionCreativeSize,
        FilterVideoContentDuration,
        FilterVideoContentLiveStream,
        FilterVideoContinuousPlay,
        FilterVideoCreativeDuration,
        FilterVideoCreativeDurationSkippable,
        FilterVideoDuration,
        FilterVideoDurationSeconds,
        FilterVideoDurationSecondsRange,
        FilterVideoFormatSupport,
        FilterVideoPlayerSize,
        FilterVideoRatingTier,
        FilterVideoSkippableSupport,
        FilterWeek,
        FilterYear,
        FilterYoutubeAdVideo,
        FilterYoutubeAdVideoId,
        FilterYoutubeAdaptedAudienceList,
        FilterYoutubeChannel,
        FilterYoutubeProgrammaticGuaranteedAdvertiser,
        FilterYoutubeProgrammaticGuaranteedInsertionOrder,
        FilterYoutubeProgrammaticGuaranteedPartner,
        FilterYoutubeVideo,
        FilterZipCode,
        FilterZipPostalCode,
    }
    impl FilterPairType {
        pub fn as_str(self) -> &'static str {
            match self {
                FilterPairType::FilterActiveViewCustomMetricId => {
                    "FILTER_ACTIVE_VIEW_CUSTOM_METRIC_ID"
                }
                FilterPairType::FilterActiveViewCustomMetricName => {
                    "FILTER_ACTIVE_VIEW_CUSTOM_METRIC_NAME"
                }
                FilterPairType::FilterActiveViewExpectedViewability => {
                    "FILTER_ACTIVE_VIEW_EXPECTED_VIEWABILITY"
                }
                FilterPairType::FilterAdPosition => "FILTER_AD_POSITION",
                FilterPairType::FilterAdType => "FILTER_AD_TYPE",
                FilterPairType::FilterAdvertiser => "FILTER_ADVERTISER",
                FilterPairType::FilterAdvertiserCurrency => "FILTER_ADVERTISER_CURRENCY",
                FilterPairType::FilterAdvertiserIntegrationCode => {
                    "FILTER_ADVERTISER_INTEGRATION_CODE"
                }
                FilterPairType::FilterAdvertiserIntegrationStatus => {
                    "FILTER_ADVERTISER_INTEGRATION_STATUS"
                }
                FilterPairType::FilterAdvertiserName => "FILTER_ADVERTISER_NAME",
                FilterPairType::FilterAdvertiserTimezone => "FILTER_ADVERTISER_TIMEZONE",
                FilterPairType::FilterAge => "FILTER_AGE",
                FilterPairType::FilterAlgorithm => "FILTER_ALGORITHM",
                FilterPairType::FilterAlgorithmId => "FILTER_ALGORITHM_ID",
                FilterPairType::FilterAmpPageRequest => "FILTER_AMP_PAGE_REQUEST",
                FilterPairType::FilterAnonymousInventoryModeling => {
                    "FILTER_ANONYMOUS_INVENTORY_MODELING"
                }
                FilterPairType::FilterAppUrl => "FILTER_APP_URL",
                FilterPairType::FilterAppUrlExcluded => "FILTER_APP_URL_EXCLUDED",
                FilterPairType::FilterAttributedUserlist => "FILTER_ATTRIBUTED_USERLIST",
                FilterPairType::FilterAttributedUserlistCost => "FILTER_ATTRIBUTED_USERLIST_COST",
                FilterPairType::FilterAttributedUserlistType => "FILTER_ATTRIBUTED_USERLIST_TYPE",
                FilterPairType::FilterAttributionModel => "FILTER_ATTRIBUTION_MODEL",
                FilterPairType::FilterAudienceList => "FILTER_AUDIENCE_LIST",
                FilterPairType::FilterAudienceListCost => "FILTER_AUDIENCE_LIST_COST",
                FilterPairType::FilterAudienceListType => "FILTER_AUDIENCE_LIST_TYPE",
                FilterPairType::FilterAudienceName => "FILTER_AUDIENCE_NAME",
                FilterPairType::FilterAudienceType => "FILTER_AUDIENCE_TYPE",
                FilterPairType::FilterAudioFeedTypeName => "FILTER_AUDIO_FEED_TYPE_NAME",
                FilterPairType::FilterAuthorizedSellerState => "FILTER_AUTHORIZED_SELLER_STATE",
                FilterPairType::FilterBillableOutcome => "FILTER_BILLABLE_OUTCOME",
                FilterPairType::FilterBrandLiftType => "FILTER_BRAND_LIFT_TYPE",
                FilterPairType::FilterBrowser => "FILTER_BROWSER",
                FilterPairType::FilterBudgetSegmentBudget => "FILTER_BUDGET_SEGMENT_BUDGET",
                FilterPairType::FilterBudgetSegmentDescription => {
                    "FILTER_BUDGET_SEGMENT_DESCRIPTION"
                }
                FilterPairType::FilterBudgetSegmentEndDate => "FILTER_BUDGET_SEGMENT_END_DATE",
                FilterPairType::FilterBudgetSegmentPacingPercentage => {
                    "FILTER_BUDGET_SEGMENT_PACING_PERCENTAGE"
                }
                FilterPairType::FilterBudgetSegmentStartDate => "FILTER_BUDGET_SEGMENT_START_DATE",
                FilterPairType::FilterBudgetSegmentType => "FILTER_BUDGET_SEGMENT_TYPE",
                FilterPairType::FilterCampaignDailyFrequency => "FILTER_CAMPAIGN_DAILY_FREQUENCY",
                FilterPairType::FilterCarrier => "FILTER_CARRIER",
                FilterPairType::FilterCarrierName => "FILTER_CARRIER_NAME",
                FilterPairType::FilterChannelGrouping => "FILTER_CHANNEL_GROUPING",
                FilterPairType::FilterChannelId => "FILTER_CHANNEL_ID",
                FilterPairType::FilterChannelName => "FILTER_CHANNEL_NAME",
                FilterPairType::FilterChannelType => "FILTER_CHANNEL_TYPE",
                FilterPairType::FilterCity => "FILTER_CITY",
                FilterPairType::FilterCityName => "FILTER_CITY_NAME",
                FilterPairType::FilterCm360PlacementId => "FILTER_CM360_PLACEMENT_ID",
                FilterPairType::FilterCmPlacementId => "FILTER_CM_PLACEMENT_ID",
                FilterPairType::FilterCompanionCreativeId => "FILTER_COMPANION_CREATIVE_ID",
                FilterPairType::FilterCompanionCreativeName => "FILTER_COMPANION_CREATIVE_NAME",
                FilterPairType::FilterConversionDelay => "FILTER_CONVERSION_DELAY",
                FilterPairType::FilterConversionSource => "FILTER_CONVERSION_SOURCE",
                FilterPairType::FilterConversionSourceId => "FILTER_CONVERSION_SOURCE_ID",
                FilterPairType::FilterCountry => "FILTER_COUNTRY",
                FilterPairType::FilterCountryId => "FILTER_COUNTRY_ID",
                FilterPairType::FilterCreative => "FILTER_CREATIVE",
                FilterPairType::FilterCreativeAsset => "FILTER_CREATIVE_ASSET",
                FilterPairType::FilterCreativeAttribute => "FILTER_CREATIVE_ATTRIBUTE",
                FilterPairType::FilterCreativeHeight => "FILTER_CREATIVE_HEIGHT",
                FilterPairType::FilterCreativeId => "FILTER_CREATIVE_ID",
                FilterPairType::FilterCreativeIntegrationCode => "FILTER_CREATIVE_INTEGRATION_CODE",
                FilterPairType::FilterCreativeRenderedInAmp => "FILTER_CREATIVE_RENDERED_IN_AMP",
                FilterPairType::FilterCreativeSize => "FILTER_CREATIVE_SIZE",
                FilterPairType::FilterCreativeSource => "FILTER_CREATIVE_SOURCE",
                FilterPairType::FilterCreativeStatus => "FILTER_CREATIVE_STATUS",
                FilterPairType::FilterCreativeType => "FILTER_CREATIVE_TYPE",
                FilterPairType::FilterCreativeWidth => "FILTER_CREATIVE_WIDTH",
                FilterPairType::FilterDataProvider => "FILTER_DATA_PROVIDER",
                FilterPairType::FilterDataProviderName => "FILTER_DATA_PROVIDER_NAME",
                FilterPairType::FilterDataSource => "FILTER_DATA_SOURCE",
                FilterPairType::FilterDate => "FILTER_DATE",
                FilterPairType::FilterDayOfWeek => "FILTER_DAY_OF_WEEK",
                FilterPairType::FilterDetailedDemographics => "FILTER_DETAILED_DEMOGRAPHICS",
                FilterPairType::FilterDetailedDemographicsId => "FILTER_DETAILED_DEMOGRAPHICS_ID",
                FilterPairType::FilterDevice => "FILTER_DEVICE",
                FilterPairType::FilterDeviceMake => "FILTER_DEVICE_MAKE",
                FilterPairType::FilterDeviceModel => "FILTER_DEVICE_MODEL",
                FilterPairType::FilterDeviceType => "FILTER_DEVICE_TYPE",
                FilterPairType::FilterDfpOrderId => "FILTER_DFP_ORDER_ID",
                FilterPairType::FilterDigitalContentLabel => "FILTER_DIGITAL_CONTENT_LABEL",
                FilterPairType::FilterDma => "FILTER_DMA",
                FilterPairType::FilterDmaName => "FILTER_DMA_NAME",
                FilterPairType::FilterDomain => "FILTER_DOMAIN",
                FilterPairType::FilterEligibleCookiesOnFirstPartyAudienceList => {
                    "FILTER_ELIGIBLE_COOKIES_ON_FIRST_PARTY_AUDIENCE_LIST"
                }
                FilterPairType::FilterEligibleCookiesOnThirdPartyAudienceListAndInterest => {
                    "FILTER_ELIGIBLE_COOKIES_ON_THIRD_PARTY_AUDIENCE_LIST_AND_INTEREST"
                }
                FilterPairType::FilterEventType => "FILTER_EVENT_TYPE",
                FilterPairType::FilterExchange => "FILTER_EXCHANGE",
                FilterPairType::FilterExchangeCode => "FILTER_EXCHANGE_CODE",
                FilterPairType::FilterExchangeId => "FILTER_EXCHANGE_ID",
                FilterPairType::FilterExtension => "FILTER_EXTENSION",
                FilterPairType::FilterExtensionStatus => "FILTER_EXTENSION_STATUS",
                FilterPairType::FilterExtensionType => "FILTER_EXTENSION_TYPE",
                FilterPairType::FilterFirstPartyAudienceListCost => {
                    "FILTER_FIRST_PARTY_AUDIENCE_LIST_COST"
                }
                FilterPairType::FilterFirstPartyAudienceListType => {
                    "FILTER_FIRST_PARTY_AUDIENCE_LIST_TYPE"
                }
                FilterPairType::FilterFloodlightActivity => "FILTER_FLOODLIGHT_ACTIVITY",
                FilterPairType::FilterFloodlightActivityId => "FILTER_FLOODLIGHT_ACTIVITY_ID",
                FilterPairType::FilterFormat => "FILTER_FORMAT",
                FilterPairType::FilterGamInsertionOrder => "FILTER_GAM_INSERTION_ORDER",
                FilterPairType::FilterGamLineItem => "FILTER_GAM_LINE_ITEM",
                FilterPairType::FilterGamLineItemId => "FILTER_GAM_LINE_ITEM_ID",
                FilterPairType::FilterGender => "FILTER_GENDER",
                FilterPairType::FilterGmailAge => "FILTER_GMAIL_AGE",
                FilterPairType::FilterGmailCity => "FILTER_GMAIL_CITY",
                FilterPairType::FilterGmailCountry => "FILTER_GMAIL_COUNTRY",
                FilterPairType::FilterGmailCountryName => "FILTER_GMAIL_COUNTRY_NAME",
                FilterPairType::FilterGmailDeviceType => "FILTER_GMAIL_DEVICE_TYPE",
                FilterPairType::FilterGmailDeviceTypeName => "FILTER_GMAIL_DEVICE_TYPE_NAME",
                FilterPairType::FilterGmailGender => "FILTER_GMAIL_GENDER",
                FilterPairType::FilterGmailRegion => "FILTER_GMAIL_REGION",
                FilterPairType::FilterGmailRemarketingList => "FILTER_GMAIL_REMARKETING_LIST",
                FilterPairType::FilterHouseholdIncome => "FILTER_HOUSEHOLD_INCOME",
                FilterPairType::FilterImpressionCountingMethod => {
                    "FILTER_IMPRESSION_COUNTING_METHOD"
                }
                FilterPairType::FilterImpressionLossRejectionReason => {
                    "FILTER_IMPRESSION_LOSS_REJECTION_REASON"
                }
                FilterPairType::FilterInsertionOrder => "FILTER_INSERTION_ORDER",
                FilterPairType::FilterInsertionOrderGoalType => "FILTER_INSERTION_ORDER_GOAL_TYPE",
                FilterPairType::FilterInsertionOrderGoalValue => {
                    "FILTER_INSERTION_ORDER_GOAL_VALUE"
                }
                FilterPairType::FilterInsertionOrderIntegrationCode => {
                    "FILTER_INSERTION_ORDER_INTEGRATION_CODE"
                }
                FilterPairType::FilterInsertionOrderName => "FILTER_INSERTION_ORDER_NAME",
                FilterPairType::FilterInsertionOrderStatus => "FILTER_INSERTION_ORDER_STATUS",
                FilterPairType::FilterInterest => "FILTER_INTEREST",
                FilterPairType::FilterInventoryCommitmentType => "FILTER_INVENTORY_COMMITMENT_TYPE",
                FilterPairType::FilterInventoryDeliveryMethod => "FILTER_INVENTORY_DELIVERY_METHOD",
                FilterPairType::FilterInventoryFormat => "FILTER_INVENTORY_FORMAT",
                FilterPairType::FilterInventoryRateType => "FILTER_INVENTORY_RATE_TYPE",
                FilterPairType::FilterInventorySource => "FILTER_INVENTORY_SOURCE",
                FilterPairType::FilterInventorySourceExternalId => {
                    "FILTER_INVENTORY_SOURCE_EXTERNAL_ID"
                }
                FilterPairType::FilterInventorySourceGroup => "FILTER_INVENTORY_SOURCE_GROUP",
                FilterPairType::FilterInventorySourceGroupId => "FILTER_INVENTORY_SOURCE_GROUP_ID",
                FilterPairType::FilterInventorySourceId => "FILTER_INVENTORY_SOURCE_ID",
                FilterPairType::FilterInventorySourceName => "FILTER_INVENTORY_SOURCE_NAME",
                FilterPairType::FilterInventorySourceType => "FILTER_INVENTORY_SOURCE_TYPE",
                FilterPairType::FilterKeyword => "FILTER_KEYWORD",
                FilterPairType::FilterLifeEvent => "FILTER_LIFE_EVENT",
                FilterPairType::FilterLifeEvents => "FILTER_LIFE_EVENTS",
                FilterPairType::FilterLineItem => "FILTER_LINE_ITEM",
                FilterPairType::FilterLineItemBudget => "FILTER_LINE_ITEM_BUDGET",
                FilterPairType::FilterLineItemDailyFrequency => "FILTER_LINE_ITEM_DAILY_FREQUENCY",
                FilterPairType::FilterLineItemEndDate => "FILTER_LINE_ITEM_END_DATE",
                FilterPairType::FilterLineItemIntegrationCode => {
                    "FILTER_LINE_ITEM_INTEGRATION_CODE"
                }
                FilterPairType::FilterLineItemLifetimeFrequency => {
                    "FILTER_LINE_ITEM_LIFETIME_FREQUENCY"
                }
                FilterPairType::FilterLineItemName => "FILTER_LINE_ITEM_NAME",
                FilterPairType::FilterLineItemPacingPercentage => {
                    "FILTER_LINE_ITEM_PACING_PERCENTAGE"
                }
                FilterPairType::FilterLineItemStartDate => "FILTER_LINE_ITEM_START_DATE",
                FilterPairType::FilterLineItemStatus => "FILTER_LINE_ITEM_STATUS",
                FilterPairType::FilterLineItemType => "FILTER_LINE_ITEM_TYPE",
                FilterPairType::FilterMatchRatio => "FILTER_MATCH_RATIO",
                FilterPairType::FilterMatchedGenreTarget => "FILTER_MATCHED_GENRE_TARGET",
                FilterPairType::FilterMeasurementSource => "FILTER_MEASUREMENT_SOURCE",
                FilterPairType::FilterMediaPlan => "FILTER_MEDIA_PLAN",
                FilterPairType::FilterMediaPlanName => "FILTER_MEDIA_PLAN_NAME",
                FilterPairType::FilterMediaType => "FILTER_MEDIA_TYPE",
                FilterPairType::FilterMobileGeo => "FILTER_MOBILE_GEO",
                FilterPairType::FilterMonth => "FILTER_MONTH",
                FilterPairType::FilterMraidSupport => "FILTER_MRAID_SUPPORT",
                FilterPairType::FilterNielsenAge => "FILTER_NIELSEN_AGE",
                FilterPairType::FilterNielsenCountryCode => "FILTER_NIELSEN_COUNTRY_CODE",
                FilterPairType::FilterNielsenDateRange => "FILTER_NIELSEN_DATE_RANGE",
                FilterPairType::FilterNielsenDeviceId => "FILTER_NIELSEN_DEVICE_ID",
                FilterPairType::FilterNielsenGender => "FILTER_NIELSEN_GENDER",
                FilterPairType::FilterNielsenRestatementDate => "FILTER_NIELSEN_RESTATEMENT_DATE",
                FilterPairType::FilterNotSupported => "FILTER_NOT_SUPPORTED",
                FilterPairType::FilterOmSdkAvailable => "FILTER_OM_SDK_AVAILABLE",
                FilterPairType::FilterOmidCapable => "FILTER_OMID_CAPABLE",
                FilterPairType::FilterOrderId => "FILTER_ORDER_ID",
                FilterPairType::FilterOs => "FILTER_OS",
                FilterPairType::FilterPageCategory => "FILTER_PAGE_CATEGORY",
                FilterPairType::FilterPageLayout => "FILTER_PAGE_LAYOUT",
                FilterPairType::FilterParentalStatus => "FILTER_PARENTAL_STATUS",
                FilterPairType::FilterPartner => "FILTER_PARTNER",
                FilterPairType::FilterPartnerCurrency => "FILTER_PARTNER_CURRENCY",
                FilterPairType::FilterPartnerName => "FILTER_PARTNER_NAME",
                FilterPairType::FilterPartnerStatus => "FILTER_PARTNER_STATUS",
                FilterPairType::FilterPathEventIndex => "FILTER_PATH_EVENT_INDEX",
                FilterPairType::FilterPathPatternId => "FILTER_PATH_PATTERN_ID",
                FilterPairType::FilterPlacementAllYoutubeChannels => {
                    "FILTER_PLACEMENT_ALL_YOUTUBE_CHANNELS"
                }
                FilterPairType::FilterPlacementNameAllYoutubeChannels => {
                    "FILTER_PLACEMENT_NAME_ALL_YOUTUBE_CHANNELS"
                }
                FilterPairType::FilterPlatform => "FILTER_PLATFORM",
                FilterPairType::FilterPlaybackMethod => "FILTER_PLAYBACK_METHOD",
                FilterPairType::FilterPositionInContent => "FILTER_POSITION_IN_CONTENT",
                FilterPairType::FilterPublicInventory => "FILTER_PUBLIC_INVENTORY",
                FilterPairType::FilterPublisherProperty => "FILTER_PUBLISHER_PROPERTY",
                FilterPairType::FilterPublisherPropertyId => "FILTER_PUBLISHER_PROPERTY_ID",
                FilterPairType::FilterPublisherPropertySection => {
                    "FILTER_PUBLISHER_PROPERTY_SECTION"
                }
                FilterPairType::FilterPublisherPropertySectionId => {
                    "FILTER_PUBLISHER_PROPERTY_SECTION_ID"
                }
                FilterPairType::FilterQuarter => "FILTER_QUARTER",
                FilterPairType::FilterRefundReason => "FILTER_REFUND_REASON",
                FilterPairType::FilterRegion => "FILTER_REGION",
                FilterPairType::FilterRegionName => "FILTER_REGION_NAME",
                FilterPairType::FilterRemarketingList => "FILTER_REMARKETING_LIST",
                FilterPairType::FilterRewarded => "FILTER_REWARDED",
                FilterPairType::FilterSensitiveCategory => "FILTER_SENSITIVE_CATEGORY",
                FilterPairType::FilterServedPixelDensity => "FILTER_SERVED_PIXEL_DENSITY",
                FilterPairType::FilterSiteId => "FILTER_SITE_ID",
                FilterPairType::FilterSiteLanguage => "FILTER_SITE_LANGUAGE",
                FilterPairType::FilterSkippableSupport => "FILTER_SKIPPABLE_SUPPORT",
                FilterPairType::FilterTargetedDataProviders => "FILTER_TARGETED_DATA_PROVIDERS",
                FilterPairType::FilterTargetedUserList => "FILTER_TARGETED_USER_LIST",
                FilterPairType::FilterThirdPartyAudienceListCost => {
                    "FILTER_THIRD_PARTY_AUDIENCE_LIST_COST"
                }
                FilterPairType::FilterThirdPartyAudienceListType => {
                    "FILTER_THIRD_PARTY_AUDIENCE_LIST_TYPE"
                }
                FilterPairType::FilterTimeOfDay => "FILTER_TIME_OF_DAY",
                FilterPairType::FilterTrueviewAd => "FILTER_TRUEVIEW_AD",
                FilterPairType::FilterTrueviewAdGroup => "FILTER_TRUEVIEW_AD_GROUP",
                FilterPairType::FilterTrueviewAdGroupAdId => "FILTER_TRUEVIEW_AD_GROUP_AD_ID",
                FilterPairType::FilterTrueviewAdGroupId => "FILTER_TRUEVIEW_AD_GROUP_ID",
                FilterPairType::FilterTrueviewAdTypeName => "FILTER_TRUEVIEW_AD_TYPE_NAME",
                FilterPairType::FilterTrueviewAge => "FILTER_TRUEVIEW_AGE",
                FilterPairType::FilterTrueviewCategory => "FILTER_TRUEVIEW_CATEGORY",
                FilterPairType::FilterTrueviewCity => "FILTER_TRUEVIEW_CITY",
                FilterPairType::FilterTrueviewClickTypeName => "FILTER_TRUEVIEW_CLICK_TYPE_NAME",
                FilterPairType::FilterTrueviewConversionType => "FILTER_TRUEVIEW_CONVERSION_TYPE",
                FilterPairType::FilterTrueviewCountry => "FILTER_TRUEVIEW_COUNTRY",
                FilterPairType::FilterTrueviewCustomAffinity => "FILTER_TRUEVIEW_CUSTOM_AFFINITY",
                FilterPairType::FilterTrueviewDetailedDemographics => {
                    "FILTER_TRUEVIEW_DETAILED_DEMOGRAPHICS"
                }
                FilterPairType::FilterTrueviewDetailedDemographicsId => {
                    "FILTER_TRUEVIEW_DETAILED_DEMOGRAPHICS_ID"
                }
                FilterPairType::FilterTrueviewDma => "FILTER_TRUEVIEW_DMA",
                FilterPairType::FilterTrueviewDmaName => "FILTER_TRUEVIEW_DMA_NAME",
                FilterPairType::FilterTrueviewGender => "FILTER_TRUEVIEW_GENDER",
                FilterPairType::FilterTrueviewHouseholdIncome => "FILTER_TRUEVIEW_HOUSEHOLD_INCOME",
                FilterPairType::FilterTrueviewIarAge => "FILTER_TRUEVIEW_IAR_AGE",
                FilterPairType::FilterTrueviewIarCategory => "FILTER_TRUEVIEW_IAR_CATEGORY",
                FilterPairType::FilterTrueviewIarCity => "FILTER_TRUEVIEW_IAR_CITY",
                FilterPairType::FilterTrueviewIarCountry => "FILTER_TRUEVIEW_IAR_COUNTRY",
                FilterPairType::FilterTrueviewIarCountryName => "FILTER_TRUEVIEW_IAR_COUNTRY_NAME",
                FilterPairType::FilterTrueviewIarGender => "FILTER_TRUEVIEW_IAR_GENDER",
                FilterPairType::FilterTrueviewIarInterest => "FILTER_TRUEVIEW_IAR_INTEREST",
                FilterPairType::FilterTrueviewIarLanguage => "FILTER_TRUEVIEW_IAR_LANGUAGE",
                FilterPairType::FilterTrueviewIarParentalStatus => {
                    "FILTER_TRUEVIEW_IAR_PARENTAL_STATUS"
                }
                FilterPairType::FilterTrueviewIarRegion => "FILTER_TRUEVIEW_IAR_REGION",
                FilterPairType::FilterTrueviewIarRegionName => "FILTER_TRUEVIEW_IAR_REGION_NAME",
                FilterPairType::FilterTrueviewIarRemarketingList => {
                    "FILTER_TRUEVIEW_IAR_REMARKETING_LIST"
                }
                FilterPairType::FilterTrueviewIarTimeOfDay => "FILTER_TRUEVIEW_IAR_TIME_OF_DAY",
                FilterPairType::FilterTrueviewIarYoutubeChannel => {
                    "FILTER_TRUEVIEW_IAR_YOUTUBE_CHANNEL"
                }
                FilterPairType::FilterTrueviewIarYoutubeVideo => {
                    "FILTER_TRUEVIEW_IAR_YOUTUBE_VIDEO"
                }
                FilterPairType::FilterTrueviewIarZipcode => "FILTER_TRUEVIEW_IAR_ZIPCODE",
                FilterPairType::FilterTrueviewInterest => "FILTER_TRUEVIEW_INTEREST",
                FilterPairType::FilterTrueviewKeyword => "FILTER_TRUEVIEW_KEYWORD",
                FilterPairType::FilterTrueviewParentalStatus => "FILTER_TRUEVIEW_PARENTAL_STATUS",
                FilterPairType::FilterTrueviewPlacement => "FILTER_TRUEVIEW_PLACEMENT",
                FilterPairType::FilterTrueviewPlacementId => "FILTER_TRUEVIEW_PLACEMENT_ID",
                FilterPairType::FilterTrueviewRegion => "FILTER_TRUEVIEW_REGION",
                FilterPairType::FilterTrueviewRegionName => "FILTER_TRUEVIEW_REGION_NAME",
                FilterPairType::FilterTrueviewRemarketingList => "FILTER_TRUEVIEW_REMARKETING_LIST",
                FilterPairType::FilterTrueviewRemarketingListName => {
                    "FILTER_TRUEVIEW_REMARKETING_LIST_NAME"
                }
                FilterPairType::FilterTrueviewUrl => "FILTER_TRUEVIEW_URL",
                FilterPairType::FilterTrueviewZipcode => "FILTER_TRUEVIEW_ZIPCODE",
                FilterPairType::FilterUnknown => "FILTER_UNKNOWN",
                FilterPairType::FilterUserList => "FILTER_USER_LIST",
                FilterPairType::FilterUserListFirstParty => "FILTER_USER_LIST_FIRST_PARTY",
                FilterPairType::FilterUserListFirstPartyName => "FILTER_USER_LIST_FIRST_PARTY_NAME",
                FilterPairType::FilterUserListThirdParty => "FILTER_USER_LIST_THIRD_PARTY",
                FilterPairType::FilterUserListThirdPartyName => "FILTER_USER_LIST_THIRD_PARTY_NAME",
                FilterPairType::FilterVariantId => "FILTER_VARIANT_ID",
                FilterPairType::FilterVariantName => "FILTER_VARIANT_NAME",
                FilterPairType::FilterVariantVersion => "FILTER_VARIANT_VERSION",
                FilterPairType::FilterVendorMeasurementMode => "FILTER_VENDOR_MEASUREMENT_MODE",
                FilterPairType::FilterVerificationAudibilityComplete => {
                    "FILTER_VERIFICATION_AUDIBILITY_COMPLETE"
                }
                FilterPairType::FilterVerificationAudibilityStart => {
                    "FILTER_VERIFICATION_AUDIBILITY_START"
                }
                FilterPairType::FilterVerificationVideoPlayerSize => {
                    "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE"
                }
                FilterPairType::FilterVerificationVideoPlayerSizeComplete => {
                    "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_COMPLETE"
                }
                FilterPairType::FilterVerificationVideoPlayerSizeFirstQuartile => {
                    "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_FIRST_QUARTILE"
                }
                FilterPairType::FilterVerificationVideoPlayerSizeMidPoint => {
                    "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_MID_POINT"
                }
                FilterPairType::FilterVerificationVideoPlayerSizeStart => {
                    "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_START"
                }
                FilterPairType::FilterVerificationVideoPlayerSizeThirdQuartile => {
                    "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_THIRD_QUARTILE"
                }
                FilterPairType::FilterVerificationVideoPosition => {
                    "FILTER_VERIFICATION_VIDEO_POSITION"
                }
                FilterPairType::FilterVerificationVideoResized => {
                    "FILTER_VERIFICATION_VIDEO_RESIZED"
                }
                FilterPairType::FilterVideoAdPositionInStream => {
                    "FILTER_VIDEO_AD_POSITION_IN_STREAM"
                }
                FilterPairType::FilterVideoCompanionCreativeSize => {
                    "FILTER_VIDEO_COMPANION_CREATIVE_SIZE"
                }
                FilterPairType::FilterVideoContentDuration => "FILTER_VIDEO_CONTENT_DURATION",
                FilterPairType::FilterVideoContentLiveStream => "FILTER_VIDEO_CONTENT_LIVE_STREAM",
                FilterPairType::FilterVideoContinuousPlay => "FILTER_VIDEO_CONTINUOUS_PLAY",
                FilterPairType::FilterVideoCreativeDuration => "FILTER_VIDEO_CREATIVE_DURATION",
                FilterPairType::FilterVideoCreativeDurationSkippable => {
                    "FILTER_VIDEO_CREATIVE_DURATION_SKIPPABLE"
                }
                FilterPairType::FilterVideoDuration => "FILTER_VIDEO_DURATION",
                FilterPairType::FilterVideoDurationSeconds => "FILTER_VIDEO_DURATION_SECONDS",
                FilterPairType::FilterVideoDurationSecondsRange => {
                    "FILTER_VIDEO_DURATION_SECONDS_RANGE"
                }
                FilterPairType::FilterVideoFormatSupport => "FILTER_VIDEO_FORMAT_SUPPORT",
                FilterPairType::FilterVideoPlayerSize => "FILTER_VIDEO_PLAYER_SIZE",
                FilterPairType::FilterVideoRatingTier => "FILTER_VIDEO_RATING_TIER",
                FilterPairType::FilterVideoSkippableSupport => "FILTER_VIDEO_SKIPPABLE_SUPPORT",
                FilterPairType::FilterWeek => "FILTER_WEEK",
                FilterPairType::FilterYear => "FILTER_YEAR",
                FilterPairType::FilterYoutubeAdVideo => "FILTER_YOUTUBE_AD_VIDEO",
                FilterPairType::FilterYoutubeAdVideoId => "FILTER_YOUTUBE_AD_VIDEO_ID",
                FilterPairType::FilterYoutubeAdaptedAudienceList => {
                    "FILTER_YOUTUBE_ADAPTED_AUDIENCE_LIST"
                }
                FilterPairType::FilterYoutubeChannel => "FILTER_YOUTUBE_CHANNEL",
                FilterPairType::FilterYoutubeProgrammaticGuaranteedAdvertiser => {
                    "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_ADVERTISER"
                }
                FilterPairType::FilterYoutubeProgrammaticGuaranteedInsertionOrder => {
                    "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_INSERTION_ORDER"
                }
                FilterPairType::FilterYoutubeProgrammaticGuaranteedPartner => {
                    "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_PARTNER"
                }
                FilterPairType::FilterYoutubeVideo => "FILTER_YOUTUBE_VIDEO",
                FilterPairType::FilterZipCode => "FILTER_ZIP_CODE",
                FilterPairType::FilterZipPostalCode => "FILTER_ZIP_POSTAL_CODE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for FilterPairType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for FilterPairType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<FilterPairType, ()> {
            Ok(match s {
                "FILTER_ACTIVE_VIEW_CUSTOM_METRIC_ID" => {
                    FilterPairType::FilterActiveViewCustomMetricId
                }
                "FILTER_ACTIVE_VIEW_CUSTOM_METRIC_NAME" => {
                    FilterPairType::FilterActiveViewCustomMetricName
                }
                "FILTER_ACTIVE_VIEW_EXPECTED_VIEWABILITY" => {
                    FilterPairType::FilterActiveViewExpectedViewability
                }
                "FILTER_AD_POSITION" => FilterPairType::FilterAdPosition,
                "FILTER_AD_TYPE" => FilterPairType::FilterAdType,
                "FILTER_ADVERTISER" => FilterPairType::FilterAdvertiser,
                "FILTER_ADVERTISER_CURRENCY" => FilterPairType::FilterAdvertiserCurrency,
                "FILTER_ADVERTISER_INTEGRATION_CODE" => {
                    FilterPairType::FilterAdvertiserIntegrationCode
                }
                "FILTER_ADVERTISER_INTEGRATION_STATUS" => {
                    FilterPairType::FilterAdvertiserIntegrationStatus
                }
                "FILTER_ADVERTISER_NAME" => FilterPairType::FilterAdvertiserName,
                "FILTER_ADVERTISER_TIMEZONE" => FilterPairType::FilterAdvertiserTimezone,
                "FILTER_AGE" => FilterPairType::FilterAge,
                "FILTER_ALGORITHM" => FilterPairType::FilterAlgorithm,
                "FILTER_ALGORITHM_ID" => FilterPairType::FilterAlgorithmId,
                "FILTER_AMP_PAGE_REQUEST" => FilterPairType::FilterAmpPageRequest,
                "FILTER_ANONYMOUS_INVENTORY_MODELING" => {
                    FilterPairType::FilterAnonymousInventoryModeling
                }
                "FILTER_APP_URL" => FilterPairType::FilterAppUrl,
                "FILTER_APP_URL_EXCLUDED" => FilterPairType::FilterAppUrlExcluded,
                "FILTER_ATTRIBUTED_USERLIST" => FilterPairType::FilterAttributedUserlist,
                "FILTER_ATTRIBUTED_USERLIST_COST" => FilterPairType::FilterAttributedUserlistCost,
                "FILTER_ATTRIBUTED_USERLIST_TYPE" => FilterPairType::FilterAttributedUserlistType,
                "FILTER_ATTRIBUTION_MODEL" => FilterPairType::FilterAttributionModel,
                "FILTER_AUDIENCE_LIST" => FilterPairType::FilterAudienceList,
                "FILTER_AUDIENCE_LIST_COST" => FilterPairType::FilterAudienceListCost,
                "FILTER_AUDIENCE_LIST_TYPE" => FilterPairType::FilterAudienceListType,
                "FILTER_AUDIENCE_NAME" => FilterPairType::FilterAudienceName,
                "FILTER_AUDIENCE_TYPE" => FilterPairType::FilterAudienceType,
                "FILTER_AUDIO_FEED_TYPE_NAME" => FilterPairType::FilterAudioFeedTypeName,
                "FILTER_AUTHORIZED_SELLER_STATE" => FilterPairType::FilterAuthorizedSellerState,
                "FILTER_BILLABLE_OUTCOME" => FilterPairType::FilterBillableOutcome,
                "FILTER_BRAND_LIFT_TYPE" => FilterPairType::FilterBrandLiftType,
                "FILTER_BROWSER" => FilterPairType::FilterBrowser,
                "FILTER_BUDGET_SEGMENT_BUDGET" => FilterPairType::FilterBudgetSegmentBudget,
                "FILTER_BUDGET_SEGMENT_DESCRIPTION" => {
                    FilterPairType::FilterBudgetSegmentDescription
                }
                "FILTER_BUDGET_SEGMENT_END_DATE" => FilterPairType::FilterBudgetSegmentEndDate,
                "FILTER_BUDGET_SEGMENT_PACING_PERCENTAGE" => {
                    FilterPairType::FilterBudgetSegmentPacingPercentage
                }
                "FILTER_BUDGET_SEGMENT_START_DATE" => FilterPairType::FilterBudgetSegmentStartDate,
                "FILTER_BUDGET_SEGMENT_TYPE" => FilterPairType::FilterBudgetSegmentType,
                "FILTER_CAMPAIGN_DAILY_FREQUENCY" => FilterPairType::FilterCampaignDailyFrequency,
                "FILTER_CARRIER" => FilterPairType::FilterCarrier,
                "FILTER_CARRIER_NAME" => FilterPairType::FilterCarrierName,
                "FILTER_CHANNEL_GROUPING" => FilterPairType::FilterChannelGrouping,
                "FILTER_CHANNEL_ID" => FilterPairType::FilterChannelId,
                "FILTER_CHANNEL_NAME" => FilterPairType::FilterChannelName,
                "FILTER_CHANNEL_TYPE" => FilterPairType::FilterChannelType,
                "FILTER_CITY" => FilterPairType::FilterCity,
                "FILTER_CITY_NAME" => FilterPairType::FilterCityName,
                "FILTER_CM360_PLACEMENT_ID" => FilterPairType::FilterCm360PlacementId,
                "FILTER_CM_PLACEMENT_ID" => FilterPairType::FilterCmPlacementId,
                "FILTER_COMPANION_CREATIVE_ID" => FilterPairType::FilterCompanionCreativeId,
                "FILTER_COMPANION_CREATIVE_NAME" => FilterPairType::FilterCompanionCreativeName,
                "FILTER_CONVERSION_DELAY" => FilterPairType::FilterConversionDelay,
                "FILTER_CONVERSION_SOURCE" => FilterPairType::FilterConversionSource,
                "FILTER_CONVERSION_SOURCE_ID" => FilterPairType::FilterConversionSourceId,
                "FILTER_COUNTRY" => FilterPairType::FilterCountry,
                "FILTER_COUNTRY_ID" => FilterPairType::FilterCountryId,
                "FILTER_CREATIVE" => FilterPairType::FilterCreative,
                "FILTER_CREATIVE_ASSET" => FilterPairType::FilterCreativeAsset,
                "FILTER_CREATIVE_ATTRIBUTE" => FilterPairType::FilterCreativeAttribute,
                "FILTER_CREATIVE_HEIGHT" => FilterPairType::FilterCreativeHeight,
                "FILTER_CREATIVE_ID" => FilterPairType::FilterCreativeId,
                "FILTER_CREATIVE_INTEGRATION_CODE" => FilterPairType::FilterCreativeIntegrationCode,
                "FILTER_CREATIVE_RENDERED_IN_AMP" => FilterPairType::FilterCreativeRenderedInAmp,
                "FILTER_CREATIVE_SIZE" => FilterPairType::FilterCreativeSize,
                "FILTER_CREATIVE_SOURCE" => FilterPairType::FilterCreativeSource,
                "FILTER_CREATIVE_STATUS" => FilterPairType::FilterCreativeStatus,
                "FILTER_CREATIVE_TYPE" => FilterPairType::FilterCreativeType,
                "FILTER_CREATIVE_WIDTH" => FilterPairType::FilterCreativeWidth,
                "FILTER_DATA_PROVIDER" => FilterPairType::FilterDataProvider,
                "FILTER_DATA_PROVIDER_NAME" => FilterPairType::FilterDataProviderName,
                "FILTER_DATA_SOURCE" => FilterPairType::FilterDataSource,
                "FILTER_DATE" => FilterPairType::FilterDate,
                "FILTER_DAY_OF_WEEK" => FilterPairType::FilterDayOfWeek,
                "FILTER_DETAILED_DEMOGRAPHICS" => FilterPairType::FilterDetailedDemographics,
                "FILTER_DETAILED_DEMOGRAPHICS_ID" => FilterPairType::FilterDetailedDemographicsId,
                "FILTER_DEVICE" => FilterPairType::FilterDevice,
                "FILTER_DEVICE_MAKE" => FilterPairType::FilterDeviceMake,
                "FILTER_DEVICE_MODEL" => FilterPairType::FilterDeviceModel,
                "FILTER_DEVICE_TYPE" => FilterPairType::FilterDeviceType,
                "FILTER_DFP_ORDER_ID" => FilterPairType::FilterDfpOrderId,
                "FILTER_DIGITAL_CONTENT_LABEL" => FilterPairType::FilterDigitalContentLabel,
                "FILTER_DMA" => FilterPairType::FilterDma,
                "FILTER_DMA_NAME" => FilterPairType::FilterDmaName,
                "FILTER_DOMAIN" => FilterPairType::FilterDomain,
                "FILTER_ELIGIBLE_COOKIES_ON_FIRST_PARTY_AUDIENCE_LIST" => {
                    FilterPairType::FilterEligibleCookiesOnFirstPartyAudienceList
                }
                "FILTER_ELIGIBLE_COOKIES_ON_THIRD_PARTY_AUDIENCE_LIST_AND_INTEREST" => {
                    FilterPairType::FilterEligibleCookiesOnThirdPartyAudienceListAndInterest
                }
                "FILTER_EVENT_TYPE" => FilterPairType::FilterEventType,
                "FILTER_EXCHANGE" => FilterPairType::FilterExchange,
                "FILTER_EXCHANGE_CODE" => FilterPairType::FilterExchangeCode,
                "FILTER_EXCHANGE_ID" => FilterPairType::FilterExchangeId,
                "FILTER_EXTENSION" => FilterPairType::FilterExtension,
                "FILTER_EXTENSION_STATUS" => FilterPairType::FilterExtensionStatus,
                "FILTER_EXTENSION_TYPE" => FilterPairType::FilterExtensionType,
                "FILTER_FIRST_PARTY_AUDIENCE_LIST_COST" => {
                    FilterPairType::FilterFirstPartyAudienceListCost
                }
                "FILTER_FIRST_PARTY_AUDIENCE_LIST_TYPE" => {
                    FilterPairType::FilterFirstPartyAudienceListType
                }
                "FILTER_FLOODLIGHT_ACTIVITY" => FilterPairType::FilterFloodlightActivity,
                "FILTER_FLOODLIGHT_ACTIVITY_ID" => FilterPairType::FilterFloodlightActivityId,
                "FILTER_FORMAT" => FilterPairType::FilterFormat,
                "FILTER_GAM_INSERTION_ORDER" => FilterPairType::FilterGamInsertionOrder,
                "FILTER_GAM_LINE_ITEM" => FilterPairType::FilterGamLineItem,
                "FILTER_GAM_LINE_ITEM_ID" => FilterPairType::FilterGamLineItemId,
                "FILTER_GENDER" => FilterPairType::FilterGender,
                "FILTER_GMAIL_AGE" => FilterPairType::FilterGmailAge,
                "FILTER_GMAIL_CITY" => FilterPairType::FilterGmailCity,
                "FILTER_GMAIL_COUNTRY" => FilterPairType::FilterGmailCountry,
                "FILTER_GMAIL_COUNTRY_NAME" => FilterPairType::FilterGmailCountryName,
                "FILTER_GMAIL_DEVICE_TYPE" => FilterPairType::FilterGmailDeviceType,
                "FILTER_GMAIL_DEVICE_TYPE_NAME" => FilterPairType::FilterGmailDeviceTypeName,
                "FILTER_GMAIL_GENDER" => FilterPairType::FilterGmailGender,
                "FILTER_GMAIL_REGION" => FilterPairType::FilterGmailRegion,
                "FILTER_GMAIL_REMARKETING_LIST" => FilterPairType::FilterGmailRemarketingList,
                "FILTER_HOUSEHOLD_INCOME" => FilterPairType::FilterHouseholdIncome,
                "FILTER_IMPRESSION_COUNTING_METHOD" => {
                    FilterPairType::FilterImpressionCountingMethod
                }
                "FILTER_IMPRESSION_LOSS_REJECTION_REASON" => {
                    FilterPairType::FilterImpressionLossRejectionReason
                }
                "FILTER_INSERTION_ORDER" => FilterPairType::FilterInsertionOrder,
                "FILTER_INSERTION_ORDER_GOAL_TYPE" => FilterPairType::FilterInsertionOrderGoalType,
                "FILTER_INSERTION_ORDER_GOAL_VALUE" => {
                    FilterPairType::FilterInsertionOrderGoalValue
                }
                "FILTER_INSERTION_ORDER_INTEGRATION_CODE" => {
                    FilterPairType::FilterInsertionOrderIntegrationCode
                }
                "FILTER_INSERTION_ORDER_NAME" => FilterPairType::FilterInsertionOrderName,
                "FILTER_INSERTION_ORDER_STATUS" => FilterPairType::FilterInsertionOrderStatus,
                "FILTER_INTEREST" => FilterPairType::FilterInterest,
                "FILTER_INVENTORY_COMMITMENT_TYPE" => FilterPairType::FilterInventoryCommitmentType,
                "FILTER_INVENTORY_DELIVERY_METHOD" => FilterPairType::FilterInventoryDeliveryMethod,
                "FILTER_INVENTORY_FORMAT" => FilterPairType::FilterInventoryFormat,
                "FILTER_INVENTORY_RATE_TYPE" => FilterPairType::FilterInventoryRateType,
                "FILTER_INVENTORY_SOURCE" => FilterPairType::FilterInventorySource,
                "FILTER_INVENTORY_SOURCE_EXTERNAL_ID" => {
                    FilterPairType::FilterInventorySourceExternalId
                }
                "FILTER_INVENTORY_SOURCE_GROUP" => FilterPairType::FilterInventorySourceGroup,
                "FILTER_INVENTORY_SOURCE_GROUP_ID" => FilterPairType::FilterInventorySourceGroupId,
                "FILTER_INVENTORY_SOURCE_ID" => FilterPairType::FilterInventorySourceId,
                "FILTER_INVENTORY_SOURCE_NAME" => FilterPairType::FilterInventorySourceName,
                "FILTER_INVENTORY_SOURCE_TYPE" => FilterPairType::FilterInventorySourceType,
                "FILTER_KEYWORD" => FilterPairType::FilterKeyword,
                "FILTER_LIFE_EVENT" => FilterPairType::FilterLifeEvent,
                "FILTER_LIFE_EVENTS" => FilterPairType::FilterLifeEvents,
                "FILTER_LINE_ITEM" => FilterPairType::FilterLineItem,
                "FILTER_LINE_ITEM_BUDGET" => FilterPairType::FilterLineItemBudget,
                "FILTER_LINE_ITEM_DAILY_FREQUENCY" => FilterPairType::FilterLineItemDailyFrequency,
                "FILTER_LINE_ITEM_END_DATE" => FilterPairType::FilterLineItemEndDate,
                "FILTER_LINE_ITEM_INTEGRATION_CODE" => {
                    FilterPairType::FilterLineItemIntegrationCode
                }
                "FILTER_LINE_ITEM_LIFETIME_FREQUENCY" => {
                    FilterPairType::FilterLineItemLifetimeFrequency
                }
                "FILTER_LINE_ITEM_NAME" => FilterPairType::FilterLineItemName,
                "FILTER_LINE_ITEM_PACING_PERCENTAGE" => {
                    FilterPairType::FilterLineItemPacingPercentage
                }
                "FILTER_LINE_ITEM_START_DATE" => FilterPairType::FilterLineItemStartDate,
                "FILTER_LINE_ITEM_STATUS" => FilterPairType::FilterLineItemStatus,
                "FILTER_LINE_ITEM_TYPE" => FilterPairType::FilterLineItemType,
                "FILTER_MATCH_RATIO" => FilterPairType::FilterMatchRatio,
                "FILTER_MATCHED_GENRE_TARGET" => FilterPairType::FilterMatchedGenreTarget,
                "FILTER_MEASUREMENT_SOURCE" => FilterPairType::FilterMeasurementSource,
                "FILTER_MEDIA_PLAN" => FilterPairType::FilterMediaPlan,
                "FILTER_MEDIA_PLAN_NAME" => FilterPairType::FilterMediaPlanName,
                "FILTER_MEDIA_TYPE" => FilterPairType::FilterMediaType,
                "FILTER_MOBILE_GEO" => FilterPairType::FilterMobileGeo,
                "FILTER_MONTH" => FilterPairType::FilterMonth,
                "FILTER_MRAID_SUPPORT" => FilterPairType::FilterMraidSupport,
                "FILTER_NIELSEN_AGE" => FilterPairType::FilterNielsenAge,
                "FILTER_NIELSEN_COUNTRY_CODE" => FilterPairType::FilterNielsenCountryCode,
                "FILTER_NIELSEN_DATE_RANGE" => FilterPairType::FilterNielsenDateRange,
                "FILTER_NIELSEN_DEVICE_ID" => FilterPairType::FilterNielsenDeviceId,
                "FILTER_NIELSEN_GENDER" => FilterPairType::FilterNielsenGender,
                "FILTER_NIELSEN_RESTATEMENT_DATE" => FilterPairType::FilterNielsenRestatementDate,
                "FILTER_NOT_SUPPORTED" => FilterPairType::FilterNotSupported,
                "FILTER_OM_SDK_AVAILABLE" => FilterPairType::FilterOmSdkAvailable,
                "FILTER_OMID_CAPABLE" => FilterPairType::FilterOmidCapable,
                "FILTER_ORDER_ID" => FilterPairType::FilterOrderId,
                "FILTER_OS" => FilterPairType::FilterOs,
                "FILTER_PAGE_CATEGORY" => FilterPairType::FilterPageCategory,
                "FILTER_PAGE_LAYOUT" => FilterPairType::FilterPageLayout,
                "FILTER_PARENTAL_STATUS" => FilterPairType::FilterParentalStatus,
                "FILTER_PARTNER" => FilterPairType::FilterPartner,
                "FILTER_PARTNER_CURRENCY" => FilterPairType::FilterPartnerCurrency,
                "FILTER_PARTNER_NAME" => FilterPairType::FilterPartnerName,
                "FILTER_PARTNER_STATUS" => FilterPairType::FilterPartnerStatus,
                "FILTER_PATH_EVENT_INDEX" => FilterPairType::FilterPathEventIndex,
                "FILTER_PATH_PATTERN_ID" => FilterPairType::FilterPathPatternId,
                "FILTER_PLACEMENT_ALL_YOUTUBE_CHANNELS" => {
                    FilterPairType::FilterPlacementAllYoutubeChannels
                }
                "FILTER_PLACEMENT_NAME_ALL_YOUTUBE_CHANNELS" => {
                    FilterPairType::FilterPlacementNameAllYoutubeChannels
                }
                "FILTER_PLATFORM" => FilterPairType::FilterPlatform,
                "FILTER_PLAYBACK_METHOD" => FilterPairType::FilterPlaybackMethod,
                "FILTER_POSITION_IN_CONTENT" => FilterPairType::FilterPositionInContent,
                "FILTER_PUBLIC_INVENTORY" => FilterPairType::FilterPublicInventory,
                "FILTER_PUBLISHER_PROPERTY" => FilterPairType::FilterPublisherProperty,
                "FILTER_PUBLISHER_PROPERTY_ID" => FilterPairType::FilterPublisherPropertyId,
                "FILTER_PUBLISHER_PROPERTY_SECTION" => {
                    FilterPairType::FilterPublisherPropertySection
                }
                "FILTER_PUBLISHER_PROPERTY_SECTION_ID" => {
                    FilterPairType::FilterPublisherPropertySectionId
                }
                "FILTER_QUARTER" => FilterPairType::FilterQuarter,
                "FILTER_REFUND_REASON" => FilterPairType::FilterRefundReason,
                "FILTER_REGION" => FilterPairType::FilterRegion,
                "FILTER_REGION_NAME" => FilterPairType::FilterRegionName,
                "FILTER_REMARKETING_LIST" => FilterPairType::FilterRemarketingList,
                "FILTER_REWARDED" => FilterPairType::FilterRewarded,
                "FILTER_SENSITIVE_CATEGORY" => FilterPairType::FilterSensitiveCategory,
                "FILTER_SERVED_PIXEL_DENSITY" => FilterPairType::FilterServedPixelDensity,
                "FILTER_SITE_ID" => FilterPairType::FilterSiteId,
                "FILTER_SITE_LANGUAGE" => FilterPairType::FilterSiteLanguage,
                "FILTER_SKIPPABLE_SUPPORT" => FilterPairType::FilterSkippableSupport,
                "FILTER_TARGETED_DATA_PROVIDERS" => FilterPairType::FilterTargetedDataProviders,
                "FILTER_TARGETED_USER_LIST" => FilterPairType::FilterTargetedUserList,
                "FILTER_THIRD_PARTY_AUDIENCE_LIST_COST" => {
                    FilterPairType::FilterThirdPartyAudienceListCost
                }
                "FILTER_THIRD_PARTY_AUDIENCE_LIST_TYPE" => {
                    FilterPairType::FilterThirdPartyAudienceListType
                }
                "FILTER_TIME_OF_DAY" => FilterPairType::FilterTimeOfDay,
                "FILTER_TRUEVIEW_AD" => FilterPairType::FilterTrueviewAd,
                "FILTER_TRUEVIEW_AD_GROUP" => FilterPairType::FilterTrueviewAdGroup,
                "FILTER_TRUEVIEW_AD_GROUP_AD_ID" => FilterPairType::FilterTrueviewAdGroupAdId,
                "FILTER_TRUEVIEW_AD_GROUP_ID" => FilterPairType::FilterTrueviewAdGroupId,
                "FILTER_TRUEVIEW_AD_TYPE_NAME" => FilterPairType::FilterTrueviewAdTypeName,
                "FILTER_TRUEVIEW_AGE" => FilterPairType::FilterTrueviewAge,
                "FILTER_TRUEVIEW_CATEGORY" => FilterPairType::FilterTrueviewCategory,
                "FILTER_TRUEVIEW_CITY" => FilterPairType::FilterTrueviewCity,
                "FILTER_TRUEVIEW_CLICK_TYPE_NAME" => FilterPairType::FilterTrueviewClickTypeName,
                "FILTER_TRUEVIEW_CONVERSION_TYPE" => FilterPairType::FilterTrueviewConversionType,
                "FILTER_TRUEVIEW_COUNTRY" => FilterPairType::FilterTrueviewCountry,
                "FILTER_TRUEVIEW_CUSTOM_AFFINITY" => FilterPairType::FilterTrueviewCustomAffinity,
                "FILTER_TRUEVIEW_DETAILED_DEMOGRAPHICS" => {
                    FilterPairType::FilterTrueviewDetailedDemographics
                }
                "FILTER_TRUEVIEW_DETAILED_DEMOGRAPHICS_ID" => {
                    FilterPairType::FilterTrueviewDetailedDemographicsId
                }
                "FILTER_TRUEVIEW_DMA" => FilterPairType::FilterTrueviewDma,
                "FILTER_TRUEVIEW_DMA_NAME" => FilterPairType::FilterTrueviewDmaName,
                "FILTER_TRUEVIEW_GENDER" => FilterPairType::FilterTrueviewGender,
                "FILTER_TRUEVIEW_HOUSEHOLD_INCOME" => FilterPairType::FilterTrueviewHouseholdIncome,
                "FILTER_TRUEVIEW_IAR_AGE" => FilterPairType::FilterTrueviewIarAge,
                "FILTER_TRUEVIEW_IAR_CATEGORY" => FilterPairType::FilterTrueviewIarCategory,
                "FILTER_TRUEVIEW_IAR_CITY" => FilterPairType::FilterTrueviewIarCity,
                "FILTER_TRUEVIEW_IAR_COUNTRY" => FilterPairType::FilterTrueviewIarCountry,
                "FILTER_TRUEVIEW_IAR_COUNTRY_NAME" => FilterPairType::FilterTrueviewIarCountryName,
                "FILTER_TRUEVIEW_IAR_GENDER" => FilterPairType::FilterTrueviewIarGender,
                "FILTER_TRUEVIEW_IAR_INTEREST" => FilterPairType::FilterTrueviewIarInterest,
                "FILTER_TRUEVIEW_IAR_LANGUAGE" => FilterPairType::FilterTrueviewIarLanguage,
                "FILTER_TRUEVIEW_IAR_PARENTAL_STATUS" => {
                    FilterPairType::FilterTrueviewIarParentalStatus
                }
                "FILTER_TRUEVIEW_IAR_REGION" => FilterPairType::FilterTrueviewIarRegion,
                "FILTER_TRUEVIEW_IAR_REGION_NAME" => FilterPairType::FilterTrueviewIarRegionName,
                "FILTER_TRUEVIEW_IAR_REMARKETING_LIST" => {
                    FilterPairType::FilterTrueviewIarRemarketingList
                }
                "FILTER_TRUEVIEW_IAR_TIME_OF_DAY" => FilterPairType::FilterTrueviewIarTimeOfDay,
                "FILTER_TRUEVIEW_IAR_YOUTUBE_CHANNEL" => {
                    FilterPairType::FilterTrueviewIarYoutubeChannel
                }
                "FILTER_TRUEVIEW_IAR_YOUTUBE_VIDEO" => {
                    FilterPairType::FilterTrueviewIarYoutubeVideo
                }
                "FILTER_TRUEVIEW_IAR_ZIPCODE" => FilterPairType::FilterTrueviewIarZipcode,
                "FILTER_TRUEVIEW_INTEREST" => FilterPairType::FilterTrueviewInterest,
                "FILTER_TRUEVIEW_KEYWORD" => FilterPairType::FilterTrueviewKeyword,
                "FILTER_TRUEVIEW_PARENTAL_STATUS" => FilterPairType::FilterTrueviewParentalStatus,
                "FILTER_TRUEVIEW_PLACEMENT" => FilterPairType::FilterTrueviewPlacement,
                "FILTER_TRUEVIEW_PLACEMENT_ID" => FilterPairType::FilterTrueviewPlacementId,
                "FILTER_TRUEVIEW_REGION" => FilterPairType::FilterTrueviewRegion,
                "FILTER_TRUEVIEW_REGION_NAME" => FilterPairType::FilterTrueviewRegionName,
                "FILTER_TRUEVIEW_REMARKETING_LIST" => FilterPairType::FilterTrueviewRemarketingList,
                "FILTER_TRUEVIEW_REMARKETING_LIST_NAME" => {
                    FilterPairType::FilterTrueviewRemarketingListName
                }
                "FILTER_TRUEVIEW_URL" => FilterPairType::FilterTrueviewUrl,
                "FILTER_TRUEVIEW_ZIPCODE" => FilterPairType::FilterTrueviewZipcode,
                "FILTER_UNKNOWN" => FilterPairType::FilterUnknown,
                "FILTER_USER_LIST" => FilterPairType::FilterUserList,
                "FILTER_USER_LIST_FIRST_PARTY" => FilterPairType::FilterUserListFirstParty,
                "FILTER_USER_LIST_FIRST_PARTY_NAME" => FilterPairType::FilterUserListFirstPartyName,
                "FILTER_USER_LIST_THIRD_PARTY" => FilterPairType::FilterUserListThirdParty,
                "FILTER_USER_LIST_THIRD_PARTY_NAME" => FilterPairType::FilterUserListThirdPartyName,
                "FILTER_VARIANT_ID" => FilterPairType::FilterVariantId,
                "FILTER_VARIANT_NAME" => FilterPairType::FilterVariantName,
                "FILTER_VARIANT_VERSION" => FilterPairType::FilterVariantVersion,
                "FILTER_VENDOR_MEASUREMENT_MODE" => FilterPairType::FilterVendorMeasurementMode,
                "FILTER_VERIFICATION_AUDIBILITY_COMPLETE" => {
                    FilterPairType::FilterVerificationAudibilityComplete
                }
                "FILTER_VERIFICATION_AUDIBILITY_START" => {
                    FilterPairType::FilterVerificationAudibilityStart
                }
                "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE" => {
                    FilterPairType::FilterVerificationVideoPlayerSize
                }
                "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_COMPLETE" => {
                    FilterPairType::FilterVerificationVideoPlayerSizeComplete
                }
                "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_FIRST_QUARTILE" => {
                    FilterPairType::FilterVerificationVideoPlayerSizeFirstQuartile
                }
                "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_MID_POINT" => {
                    FilterPairType::FilterVerificationVideoPlayerSizeMidPoint
                }
                "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_START" => {
                    FilterPairType::FilterVerificationVideoPlayerSizeStart
                }
                "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_THIRD_QUARTILE" => {
                    FilterPairType::FilterVerificationVideoPlayerSizeThirdQuartile
                }
                "FILTER_VERIFICATION_VIDEO_POSITION" => {
                    FilterPairType::FilterVerificationVideoPosition
                }
                "FILTER_VERIFICATION_VIDEO_RESIZED" => {
                    FilterPairType::FilterVerificationVideoResized
                }
                "FILTER_VIDEO_AD_POSITION_IN_STREAM" => {
                    FilterPairType::FilterVideoAdPositionInStream
                }
                "FILTER_VIDEO_COMPANION_CREATIVE_SIZE" => {
                    FilterPairType::FilterVideoCompanionCreativeSize
                }
                "FILTER_VIDEO_CONTENT_DURATION" => FilterPairType::FilterVideoContentDuration,
                "FILTER_VIDEO_CONTENT_LIVE_STREAM" => FilterPairType::FilterVideoContentLiveStream,
                "FILTER_VIDEO_CONTINUOUS_PLAY" => FilterPairType::FilterVideoContinuousPlay,
                "FILTER_VIDEO_CREATIVE_DURATION" => FilterPairType::FilterVideoCreativeDuration,
                "FILTER_VIDEO_CREATIVE_DURATION_SKIPPABLE" => {
                    FilterPairType::FilterVideoCreativeDurationSkippable
                }
                "FILTER_VIDEO_DURATION" => FilterPairType::FilterVideoDuration,
                "FILTER_VIDEO_DURATION_SECONDS" => FilterPairType::FilterVideoDurationSeconds,
                "FILTER_VIDEO_DURATION_SECONDS_RANGE" => {
                    FilterPairType::FilterVideoDurationSecondsRange
                }
                "FILTER_VIDEO_FORMAT_SUPPORT" => FilterPairType::FilterVideoFormatSupport,
                "FILTER_VIDEO_PLAYER_SIZE" => FilterPairType::FilterVideoPlayerSize,
                "FILTER_VIDEO_RATING_TIER" => FilterPairType::FilterVideoRatingTier,
                "FILTER_VIDEO_SKIPPABLE_SUPPORT" => FilterPairType::FilterVideoSkippableSupport,
                "FILTER_WEEK" => FilterPairType::FilterWeek,
                "FILTER_YEAR" => FilterPairType::FilterYear,
                "FILTER_YOUTUBE_AD_VIDEO" => FilterPairType::FilterYoutubeAdVideo,
                "FILTER_YOUTUBE_AD_VIDEO_ID" => FilterPairType::FilterYoutubeAdVideoId,
                "FILTER_YOUTUBE_ADAPTED_AUDIENCE_LIST" => {
                    FilterPairType::FilterYoutubeAdaptedAudienceList
                }
                "FILTER_YOUTUBE_CHANNEL" => FilterPairType::FilterYoutubeChannel,
                "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_ADVERTISER" => {
                    FilterPairType::FilterYoutubeProgrammaticGuaranteedAdvertiser
                }
                "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_INSERTION_ORDER" => {
                    FilterPairType::FilterYoutubeProgrammaticGuaranteedInsertionOrder
                }
                "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_PARTNER" => {
                    FilterPairType::FilterYoutubeProgrammaticGuaranteedPartner
                }
                "FILTER_YOUTUBE_VIDEO" => FilterPairType::FilterYoutubeVideo,
                "FILTER_ZIP_CODE" => FilterPairType::FilterZipCode,
                "FILTER_ZIP_POSTAL_CODE" => FilterPairType::FilterZipPostalCode,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for FilterPairType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FilterPairType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FilterPairType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FILTER_ACTIVE_VIEW_CUSTOM_METRIC_ID" => {
                    FilterPairType::FilterActiveViewCustomMetricId
                }
                "FILTER_ACTIVE_VIEW_CUSTOM_METRIC_NAME" => {
                    FilterPairType::FilterActiveViewCustomMetricName
                }
                "FILTER_ACTIVE_VIEW_EXPECTED_VIEWABILITY" => {
                    FilterPairType::FilterActiveViewExpectedViewability
                }
                "FILTER_AD_POSITION" => FilterPairType::FilterAdPosition,
                "FILTER_AD_TYPE" => FilterPairType::FilterAdType,
                "FILTER_ADVERTISER" => FilterPairType::FilterAdvertiser,
                "FILTER_ADVERTISER_CURRENCY" => FilterPairType::FilterAdvertiserCurrency,
                "FILTER_ADVERTISER_INTEGRATION_CODE" => {
                    FilterPairType::FilterAdvertiserIntegrationCode
                }
                "FILTER_ADVERTISER_INTEGRATION_STATUS" => {
                    FilterPairType::FilterAdvertiserIntegrationStatus
                }
                "FILTER_ADVERTISER_NAME" => FilterPairType::FilterAdvertiserName,
                "FILTER_ADVERTISER_TIMEZONE" => FilterPairType::FilterAdvertiserTimezone,
                "FILTER_AGE" => FilterPairType::FilterAge,
                "FILTER_ALGORITHM" => FilterPairType::FilterAlgorithm,
                "FILTER_ALGORITHM_ID" => FilterPairType::FilterAlgorithmId,
                "FILTER_AMP_PAGE_REQUEST" => FilterPairType::FilterAmpPageRequest,
                "FILTER_ANONYMOUS_INVENTORY_MODELING" => {
                    FilterPairType::FilterAnonymousInventoryModeling
                }
                "FILTER_APP_URL" => FilterPairType::FilterAppUrl,
                "FILTER_APP_URL_EXCLUDED" => FilterPairType::FilterAppUrlExcluded,
                "FILTER_ATTRIBUTED_USERLIST" => FilterPairType::FilterAttributedUserlist,
                "FILTER_ATTRIBUTED_USERLIST_COST" => FilterPairType::FilterAttributedUserlistCost,
                "FILTER_ATTRIBUTED_USERLIST_TYPE" => FilterPairType::FilterAttributedUserlistType,
                "FILTER_ATTRIBUTION_MODEL" => FilterPairType::FilterAttributionModel,
                "FILTER_AUDIENCE_LIST" => FilterPairType::FilterAudienceList,
                "FILTER_AUDIENCE_LIST_COST" => FilterPairType::FilterAudienceListCost,
                "FILTER_AUDIENCE_LIST_TYPE" => FilterPairType::FilterAudienceListType,
                "FILTER_AUDIENCE_NAME" => FilterPairType::FilterAudienceName,
                "FILTER_AUDIENCE_TYPE" => FilterPairType::FilterAudienceType,
                "FILTER_AUDIO_FEED_TYPE_NAME" => FilterPairType::FilterAudioFeedTypeName,
                "FILTER_AUTHORIZED_SELLER_STATE" => FilterPairType::FilterAuthorizedSellerState,
                "FILTER_BILLABLE_OUTCOME" => FilterPairType::FilterBillableOutcome,
                "FILTER_BRAND_LIFT_TYPE" => FilterPairType::FilterBrandLiftType,
                "FILTER_BROWSER" => FilterPairType::FilterBrowser,
                "FILTER_BUDGET_SEGMENT_BUDGET" => FilterPairType::FilterBudgetSegmentBudget,
                "FILTER_BUDGET_SEGMENT_DESCRIPTION" => {
                    FilterPairType::FilterBudgetSegmentDescription
                }
                "FILTER_BUDGET_SEGMENT_END_DATE" => FilterPairType::FilterBudgetSegmentEndDate,
                "FILTER_BUDGET_SEGMENT_PACING_PERCENTAGE" => {
                    FilterPairType::FilterBudgetSegmentPacingPercentage
                }
                "FILTER_BUDGET_SEGMENT_START_DATE" => FilterPairType::FilterBudgetSegmentStartDate,
                "FILTER_BUDGET_SEGMENT_TYPE" => FilterPairType::FilterBudgetSegmentType,
                "FILTER_CAMPAIGN_DAILY_FREQUENCY" => FilterPairType::FilterCampaignDailyFrequency,
                "FILTER_CARRIER" => FilterPairType::FilterCarrier,
                "FILTER_CARRIER_NAME" => FilterPairType::FilterCarrierName,
                "FILTER_CHANNEL_GROUPING" => FilterPairType::FilterChannelGrouping,
                "FILTER_CHANNEL_ID" => FilterPairType::FilterChannelId,
                "FILTER_CHANNEL_NAME" => FilterPairType::FilterChannelName,
                "FILTER_CHANNEL_TYPE" => FilterPairType::FilterChannelType,
                "FILTER_CITY" => FilterPairType::FilterCity,
                "FILTER_CITY_NAME" => FilterPairType::FilterCityName,
                "FILTER_CM360_PLACEMENT_ID" => FilterPairType::FilterCm360PlacementId,
                "FILTER_CM_PLACEMENT_ID" => FilterPairType::FilterCmPlacementId,
                "FILTER_COMPANION_CREATIVE_ID" => FilterPairType::FilterCompanionCreativeId,
                "FILTER_COMPANION_CREATIVE_NAME" => FilterPairType::FilterCompanionCreativeName,
                "FILTER_CONVERSION_DELAY" => FilterPairType::FilterConversionDelay,
                "FILTER_CONVERSION_SOURCE" => FilterPairType::FilterConversionSource,
                "FILTER_CONVERSION_SOURCE_ID" => FilterPairType::FilterConversionSourceId,
                "FILTER_COUNTRY" => FilterPairType::FilterCountry,
                "FILTER_COUNTRY_ID" => FilterPairType::FilterCountryId,
                "FILTER_CREATIVE" => FilterPairType::FilterCreative,
                "FILTER_CREATIVE_ASSET" => FilterPairType::FilterCreativeAsset,
                "FILTER_CREATIVE_ATTRIBUTE" => FilterPairType::FilterCreativeAttribute,
                "FILTER_CREATIVE_HEIGHT" => FilterPairType::FilterCreativeHeight,
                "FILTER_CREATIVE_ID" => FilterPairType::FilterCreativeId,
                "FILTER_CREATIVE_INTEGRATION_CODE" => FilterPairType::FilterCreativeIntegrationCode,
                "FILTER_CREATIVE_RENDERED_IN_AMP" => FilterPairType::FilterCreativeRenderedInAmp,
                "FILTER_CREATIVE_SIZE" => FilterPairType::FilterCreativeSize,
                "FILTER_CREATIVE_SOURCE" => FilterPairType::FilterCreativeSource,
                "FILTER_CREATIVE_STATUS" => FilterPairType::FilterCreativeStatus,
                "FILTER_CREATIVE_TYPE" => FilterPairType::FilterCreativeType,
                "FILTER_CREATIVE_WIDTH" => FilterPairType::FilterCreativeWidth,
                "FILTER_DATA_PROVIDER" => FilterPairType::FilterDataProvider,
                "FILTER_DATA_PROVIDER_NAME" => FilterPairType::FilterDataProviderName,
                "FILTER_DATA_SOURCE" => FilterPairType::FilterDataSource,
                "FILTER_DATE" => FilterPairType::FilterDate,
                "FILTER_DAY_OF_WEEK" => FilterPairType::FilterDayOfWeek,
                "FILTER_DETAILED_DEMOGRAPHICS" => FilterPairType::FilterDetailedDemographics,
                "FILTER_DETAILED_DEMOGRAPHICS_ID" => FilterPairType::FilterDetailedDemographicsId,
                "FILTER_DEVICE" => FilterPairType::FilterDevice,
                "FILTER_DEVICE_MAKE" => FilterPairType::FilterDeviceMake,
                "FILTER_DEVICE_MODEL" => FilterPairType::FilterDeviceModel,
                "FILTER_DEVICE_TYPE" => FilterPairType::FilterDeviceType,
                "FILTER_DFP_ORDER_ID" => FilterPairType::FilterDfpOrderId,
                "FILTER_DIGITAL_CONTENT_LABEL" => FilterPairType::FilterDigitalContentLabel,
                "FILTER_DMA" => FilterPairType::FilterDma,
                "FILTER_DMA_NAME" => FilterPairType::FilterDmaName,
                "FILTER_DOMAIN" => FilterPairType::FilterDomain,
                "FILTER_ELIGIBLE_COOKIES_ON_FIRST_PARTY_AUDIENCE_LIST" => {
                    FilterPairType::FilterEligibleCookiesOnFirstPartyAudienceList
                }
                "FILTER_ELIGIBLE_COOKIES_ON_THIRD_PARTY_AUDIENCE_LIST_AND_INTEREST" => {
                    FilterPairType::FilterEligibleCookiesOnThirdPartyAudienceListAndInterest
                }
                "FILTER_EVENT_TYPE" => FilterPairType::FilterEventType,
                "FILTER_EXCHANGE" => FilterPairType::FilterExchange,
                "FILTER_EXCHANGE_CODE" => FilterPairType::FilterExchangeCode,
                "FILTER_EXCHANGE_ID" => FilterPairType::FilterExchangeId,
                "FILTER_EXTENSION" => FilterPairType::FilterExtension,
                "FILTER_EXTENSION_STATUS" => FilterPairType::FilterExtensionStatus,
                "FILTER_EXTENSION_TYPE" => FilterPairType::FilterExtensionType,
                "FILTER_FIRST_PARTY_AUDIENCE_LIST_COST" => {
                    FilterPairType::FilterFirstPartyAudienceListCost
                }
                "FILTER_FIRST_PARTY_AUDIENCE_LIST_TYPE" => {
                    FilterPairType::FilterFirstPartyAudienceListType
                }
                "FILTER_FLOODLIGHT_ACTIVITY" => FilterPairType::FilterFloodlightActivity,
                "FILTER_FLOODLIGHT_ACTIVITY_ID" => FilterPairType::FilterFloodlightActivityId,
                "FILTER_FORMAT" => FilterPairType::FilterFormat,
                "FILTER_GAM_INSERTION_ORDER" => FilterPairType::FilterGamInsertionOrder,
                "FILTER_GAM_LINE_ITEM" => FilterPairType::FilterGamLineItem,
                "FILTER_GAM_LINE_ITEM_ID" => FilterPairType::FilterGamLineItemId,
                "FILTER_GENDER" => FilterPairType::FilterGender,
                "FILTER_GMAIL_AGE" => FilterPairType::FilterGmailAge,
                "FILTER_GMAIL_CITY" => FilterPairType::FilterGmailCity,
                "FILTER_GMAIL_COUNTRY" => FilterPairType::FilterGmailCountry,
                "FILTER_GMAIL_COUNTRY_NAME" => FilterPairType::FilterGmailCountryName,
                "FILTER_GMAIL_DEVICE_TYPE" => FilterPairType::FilterGmailDeviceType,
                "FILTER_GMAIL_DEVICE_TYPE_NAME" => FilterPairType::FilterGmailDeviceTypeName,
                "FILTER_GMAIL_GENDER" => FilterPairType::FilterGmailGender,
                "FILTER_GMAIL_REGION" => FilterPairType::FilterGmailRegion,
                "FILTER_GMAIL_REMARKETING_LIST" => FilterPairType::FilterGmailRemarketingList,
                "FILTER_HOUSEHOLD_INCOME" => FilterPairType::FilterHouseholdIncome,
                "FILTER_IMPRESSION_COUNTING_METHOD" => {
                    FilterPairType::FilterImpressionCountingMethod
                }
                "FILTER_IMPRESSION_LOSS_REJECTION_REASON" => {
                    FilterPairType::FilterImpressionLossRejectionReason
                }
                "FILTER_INSERTION_ORDER" => FilterPairType::FilterInsertionOrder,
                "FILTER_INSERTION_ORDER_GOAL_TYPE" => FilterPairType::FilterInsertionOrderGoalType,
                "FILTER_INSERTION_ORDER_GOAL_VALUE" => {
                    FilterPairType::FilterInsertionOrderGoalValue
                }
                "FILTER_INSERTION_ORDER_INTEGRATION_CODE" => {
                    FilterPairType::FilterInsertionOrderIntegrationCode
                }
                "FILTER_INSERTION_ORDER_NAME" => FilterPairType::FilterInsertionOrderName,
                "FILTER_INSERTION_ORDER_STATUS" => FilterPairType::FilterInsertionOrderStatus,
                "FILTER_INTEREST" => FilterPairType::FilterInterest,
                "FILTER_INVENTORY_COMMITMENT_TYPE" => FilterPairType::FilterInventoryCommitmentType,
                "FILTER_INVENTORY_DELIVERY_METHOD" => FilterPairType::FilterInventoryDeliveryMethod,
                "FILTER_INVENTORY_FORMAT" => FilterPairType::FilterInventoryFormat,
                "FILTER_INVENTORY_RATE_TYPE" => FilterPairType::FilterInventoryRateType,
                "FILTER_INVENTORY_SOURCE" => FilterPairType::FilterInventorySource,
                "FILTER_INVENTORY_SOURCE_EXTERNAL_ID" => {
                    FilterPairType::FilterInventorySourceExternalId
                }
                "FILTER_INVENTORY_SOURCE_GROUP" => FilterPairType::FilterInventorySourceGroup,
                "FILTER_INVENTORY_SOURCE_GROUP_ID" => FilterPairType::FilterInventorySourceGroupId,
                "FILTER_INVENTORY_SOURCE_ID" => FilterPairType::FilterInventorySourceId,
                "FILTER_INVENTORY_SOURCE_NAME" => FilterPairType::FilterInventorySourceName,
                "FILTER_INVENTORY_SOURCE_TYPE" => FilterPairType::FilterInventorySourceType,
                "FILTER_KEYWORD" => FilterPairType::FilterKeyword,
                "FILTER_LIFE_EVENT" => FilterPairType::FilterLifeEvent,
                "FILTER_LIFE_EVENTS" => FilterPairType::FilterLifeEvents,
                "FILTER_LINE_ITEM" => FilterPairType::FilterLineItem,
                "FILTER_LINE_ITEM_BUDGET" => FilterPairType::FilterLineItemBudget,
                "FILTER_LINE_ITEM_DAILY_FREQUENCY" => FilterPairType::FilterLineItemDailyFrequency,
                "FILTER_LINE_ITEM_END_DATE" => FilterPairType::FilterLineItemEndDate,
                "FILTER_LINE_ITEM_INTEGRATION_CODE" => {
                    FilterPairType::FilterLineItemIntegrationCode
                }
                "FILTER_LINE_ITEM_LIFETIME_FREQUENCY" => {
                    FilterPairType::FilterLineItemLifetimeFrequency
                }
                "FILTER_LINE_ITEM_NAME" => FilterPairType::FilterLineItemName,
                "FILTER_LINE_ITEM_PACING_PERCENTAGE" => {
                    FilterPairType::FilterLineItemPacingPercentage
                }
                "FILTER_LINE_ITEM_START_DATE" => FilterPairType::FilterLineItemStartDate,
                "FILTER_LINE_ITEM_STATUS" => FilterPairType::FilterLineItemStatus,
                "FILTER_LINE_ITEM_TYPE" => FilterPairType::FilterLineItemType,
                "FILTER_MATCH_RATIO" => FilterPairType::FilterMatchRatio,
                "FILTER_MATCHED_GENRE_TARGET" => FilterPairType::FilterMatchedGenreTarget,
                "FILTER_MEASUREMENT_SOURCE" => FilterPairType::FilterMeasurementSource,
                "FILTER_MEDIA_PLAN" => FilterPairType::FilterMediaPlan,
                "FILTER_MEDIA_PLAN_NAME" => FilterPairType::FilterMediaPlanName,
                "FILTER_MEDIA_TYPE" => FilterPairType::FilterMediaType,
                "FILTER_MOBILE_GEO" => FilterPairType::FilterMobileGeo,
                "FILTER_MONTH" => FilterPairType::FilterMonth,
                "FILTER_MRAID_SUPPORT" => FilterPairType::FilterMraidSupport,
                "FILTER_NIELSEN_AGE" => FilterPairType::FilterNielsenAge,
                "FILTER_NIELSEN_COUNTRY_CODE" => FilterPairType::FilterNielsenCountryCode,
                "FILTER_NIELSEN_DATE_RANGE" => FilterPairType::FilterNielsenDateRange,
                "FILTER_NIELSEN_DEVICE_ID" => FilterPairType::FilterNielsenDeviceId,
                "FILTER_NIELSEN_GENDER" => FilterPairType::FilterNielsenGender,
                "FILTER_NIELSEN_RESTATEMENT_DATE" => FilterPairType::FilterNielsenRestatementDate,
                "FILTER_NOT_SUPPORTED" => FilterPairType::FilterNotSupported,
                "FILTER_OM_SDK_AVAILABLE" => FilterPairType::FilterOmSdkAvailable,
                "FILTER_OMID_CAPABLE" => FilterPairType::FilterOmidCapable,
                "FILTER_ORDER_ID" => FilterPairType::FilterOrderId,
                "FILTER_OS" => FilterPairType::FilterOs,
                "FILTER_PAGE_CATEGORY" => FilterPairType::FilterPageCategory,
                "FILTER_PAGE_LAYOUT" => FilterPairType::FilterPageLayout,
                "FILTER_PARENTAL_STATUS" => FilterPairType::FilterParentalStatus,
                "FILTER_PARTNER" => FilterPairType::FilterPartner,
                "FILTER_PARTNER_CURRENCY" => FilterPairType::FilterPartnerCurrency,
                "FILTER_PARTNER_NAME" => FilterPairType::FilterPartnerName,
                "FILTER_PARTNER_STATUS" => FilterPairType::FilterPartnerStatus,
                "FILTER_PATH_EVENT_INDEX" => FilterPairType::FilterPathEventIndex,
                "FILTER_PATH_PATTERN_ID" => FilterPairType::FilterPathPatternId,
                "FILTER_PLACEMENT_ALL_YOUTUBE_CHANNELS" => {
                    FilterPairType::FilterPlacementAllYoutubeChannels
                }
                "FILTER_PLACEMENT_NAME_ALL_YOUTUBE_CHANNELS" => {
                    FilterPairType::FilterPlacementNameAllYoutubeChannels
                }
                "FILTER_PLATFORM" => FilterPairType::FilterPlatform,
                "FILTER_PLAYBACK_METHOD" => FilterPairType::FilterPlaybackMethod,
                "FILTER_POSITION_IN_CONTENT" => FilterPairType::FilterPositionInContent,
                "FILTER_PUBLIC_INVENTORY" => FilterPairType::FilterPublicInventory,
                "FILTER_PUBLISHER_PROPERTY" => FilterPairType::FilterPublisherProperty,
                "FILTER_PUBLISHER_PROPERTY_ID" => FilterPairType::FilterPublisherPropertyId,
                "FILTER_PUBLISHER_PROPERTY_SECTION" => {
                    FilterPairType::FilterPublisherPropertySection
                }
                "FILTER_PUBLISHER_PROPERTY_SECTION_ID" => {
                    FilterPairType::FilterPublisherPropertySectionId
                }
                "FILTER_QUARTER" => FilterPairType::FilterQuarter,
                "FILTER_REFUND_REASON" => FilterPairType::FilterRefundReason,
                "FILTER_REGION" => FilterPairType::FilterRegion,
                "FILTER_REGION_NAME" => FilterPairType::FilterRegionName,
                "FILTER_REMARKETING_LIST" => FilterPairType::FilterRemarketingList,
                "FILTER_REWARDED" => FilterPairType::FilterRewarded,
                "FILTER_SENSITIVE_CATEGORY" => FilterPairType::FilterSensitiveCategory,
                "FILTER_SERVED_PIXEL_DENSITY" => FilterPairType::FilterServedPixelDensity,
                "FILTER_SITE_ID" => FilterPairType::FilterSiteId,
                "FILTER_SITE_LANGUAGE" => FilterPairType::FilterSiteLanguage,
                "FILTER_SKIPPABLE_SUPPORT" => FilterPairType::FilterSkippableSupport,
                "FILTER_TARGETED_DATA_PROVIDERS" => FilterPairType::FilterTargetedDataProviders,
                "FILTER_TARGETED_USER_LIST" => FilterPairType::FilterTargetedUserList,
                "FILTER_THIRD_PARTY_AUDIENCE_LIST_COST" => {
                    FilterPairType::FilterThirdPartyAudienceListCost
                }
                "FILTER_THIRD_PARTY_AUDIENCE_LIST_TYPE" => {
                    FilterPairType::FilterThirdPartyAudienceListType
                }
                "FILTER_TIME_OF_DAY" => FilterPairType::FilterTimeOfDay,
                "FILTER_TRUEVIEW_AD" => FilterPairType::FilterTrueviewAd,
                "FILTER_TRUEVIEW_AD_GROUP" => FilterPairType::FilterTrueviewAdGroup,
                "FILTER_TRUEVIEW_AD_GROUP_AD_ID" => FilterPairType::FilterTrueviewAdGroupAdId,
                "FILTER_TRUEVIEW_AD_GROUP_ID" => FilterPairType::FilterTrueviewAdGroupId,
                "FILTER_TRUEVIEW_AD_TYPE_NAME" => FilterPairType::FilterTrueviewAdTypeName,
                "FILTER_TRUEVIEW_AGE" => FilterPairType::FilterTrueviewAge,
                "FILTER_TRUEVIEW_CATEGORY" => FilterPairType::FilterTrueviewCategory,
                "FILTER_TRUEVIEW_CITY" => FilterPairType::FilterTrueviewCity,
                "FILTER_TRUEVIEW_CLICK_TYPE_NAME" => FilterPairType::FilterTrueviewClickTypeName,
                "FILTER_TRUEVIEW_CONVERSION_TYPE" => FilterPairType::FilterTrueviewConversionType,
                "FILTER_TRUEVIEW_COUNTRY" => FilterPairType::FilterTrueviewCountry,
                "FILTER_TRUEVIEW_CUSTOM_AFFINITY" => FilterPairType::FilterTrueviewCustomAffinity,
                "FILTER_TRUEVIEW_DETAILED_DEMOGRAPHICS" => {
                    FilterPairType::FilterTrueviewDetailedDemographics
                }
                "FILTER_TRUEVIEW_DETAILED_DEMOGRAPHICS_ID" => {
                    FilterPairType::FilterTrueviewDetailedDemographicsId
                }
                "FILTER_TRUEVIEW_DMA" => FilterPairType::FilterTrueviewDma,
                "FILTER_TRUEVIEW_DMA_NAME" => FilterPairType::FilterTrueviewDmaName,
                "FILTER_TRUEVIEW_GENDER" => FilterPairType::FilterTrueviewGender,
                "FILTER_TRUEVIEW_HOUSEHOLD_INCOME" => FilterPairType::FilterTrueviewHouseholdIncome,
                "FILTER_TRUEVIEW_IAR_AGE" => FilterPairType::FilterTrueviewIarAge,
                "FILTER_TRUEVIEW_IAR_CATEGORY" => FilterPairType::FilterTrueviewIarCategory,
                "FILTER_TRUEVIEW_IAR_CITY" => FilterPairType::FilterTrueviewIarCity,
                "FILTER_TRUEVIEW_IAR_COUNTRY" => FilterPairType::FilterTrueviewIarCountry,
                "FILTER_TRUEVIEW_IAR_COUNTRY_NAME" => FilterPairType::FilterTrueviewIarCountryName,
                "FILTER_TRUEVIEW_IAR_GENDER" => FilterPairType::FilterTrueviewIarGender,
                "FILTER_TRUEVIEW_IAR_INTEREST" => FilterPairType::FilterTrueviewIarInterest,
                "FILTER_TRUEVIEW_IAR_LANGUAGE" => FilterPairType::FilterTrueviewIarLanguage,
                "FILTER_TRUEVIEW_IAR_PARENTAL_STATUS" => {
                    FilterPairType::FilterTrueviewIarParentalStatus
                }
                "FILTER_TRUEVIEW_IAR_REGION" => FilterPairType::FilterTrueviewIarRegion,
                "FILTER_TRUEVIEW_IAR_REGION_NAME" => FilterPairType::FilterTrueviewIarRegionName,
                "FILTER_TRUEVIEW_IAR_REMARKETING_LIST" => {
                    FilterPairType::FilterTrueviewIarRemarketingList
                }
                "FILTER_TRUEVIEW_IAR_TIME_OF_DAY" => FilterPairType::FilterTrueviewIarTimeOfDay,
                "FILTER_TRUEVIEW_IAR_YOUTUBE_CHANNEL" => {
                    FilterPairType::FilterTrueviewIarYoutubeChannel
                }
                "FILTER_TRUEVIEW_IAR_YOUTUBE_VIDEO" => {
                    FilterPairType::FilterTrueviewIarYoutubeVideo
                }
                "FILTER_TRUEVIEW_IAR_ZIPCODE" => FilterPairType::FilterTrueviewIarZipcode,
                "FILTER_TRUEVIEW_INTEREST" => FilterPairType::FilterTrueviewInterest,
                "FILTER_TRUEVIEW_KEYWORD" => FilterPairType::FilterTrueviewKeyword,
                "FILTER_TRUEVIEW_PARENTAL_STATUS" => FilterPairType::FilterTrueviewParentalStatus,
                "FILTER_TRUEVIEW_PLACEMENT" => FilterPairType::FilterTrueviewPlacement,
                "FILTER_TRUEVIEW_PLACEMENT_ID" => FilterPairType::FilterTrueviewPlacementId,
                "FILTER_TRUEVIEW_REGION" => FilterPairType::FilterTrueviewRegion,
                "FILTER_TRUEVIEW_REGION_NAME" => FilterPairType::FilterTrueviewRegionName,
                "FILTER_TRUEVIEW_REMARKETING_LIST" => FilterPairType::FilterTrueviewRemarketingList,
                "FILTER_TRUEVIEW_REMARKETING_LIST_NAME" => {
                    FilterPairType::FilterTrueviewRemarketingListName
                }
                "FILTER_TRUEVIEW_URL" => FilterPairType::FilterTrueviewUrl,
                "FILTER_TRUEVIEW_ZIPCODE" => FilterPairType::FilterTrueviewZipcode,
                "FILTER_UNKNOWN" => FilterPairType::FilterUnknown,
                "FILTER_USER_LIST" => FilterPairType::FilterUserList,
                "FILTER_USER_LIST_FIRST_PARTY" => FilterPairType::FilterUserListFirstParty,
                "FILTER_USER_LIST_FIRST_PARTY_NAME" => FilterPairType::FilterUserListFirstPartyName,
                "FILTER_USER_LIST_THIRD_PARTY" => FilterPairType::FilterUserListThirdParty,
                "FILTER_USER_LIST_THIRD_PARTY_NAME" => FilterPairType::FilterUserListThirdPartyName,
                "FILTER_VARIANT_ID" => FilterPairType::FilterVariantId,
                "FILTER_VARIANT_NAME" => FilterPairType::FilterVariantName,
                "FILTER_VARIANT_VERSION" => FilterPairType::FilterVariantVersion,
                "FILTER_VENDOR_MEASUREMENT_MODE" => FilterPairType::FilterVendorMeasurementMode,
                "FILTER_VERIFICATION_AUDIBILITY_COMPLETE" => {
                    FilterPairType::FilterVerificationAudibilityComplete
                }
                "FILTER_VERIFICATION_AUDIBILITY_START" => {
                    FilterPairType::FilterVerificationAudibilityStart
                }
                "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE" => {
                    FilterPairType::FilterVerificationVideoPlayerSize
                }
                "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_COMPLETE" => {
                    FilterPairType::FilterVerificationVideoPlayerSizeComplete
                }
                "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_FIRST_QUARTILE" => {
                    FilterPairType::FilterVerificationVideoPlayerSizeFirstQuartile
                }
                "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_MID_POINT" => {
                    FilterPairType::FilterVerificationVideoPlayerSizeMidPoint
                }
                "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_START" => {
                    FilterPairType::FilterVerificationVideoPlayerSizeStart
                }
                "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_THIRD_QUARTILE" => {
                    FilterPairType::FilterVerificationVideoPlayerSizeThirdQuartile
                }
                "FILTER_VERIFICATION_VIDEO_POSITION" => {
                    FilterPairType::FilterVerificationVideoPosition
                }
                "FILTER_VERIFICATION_VIDEO_RESIZED" => {
                    FilterPairType::FilterVerificationVideoResized
                }
                "FILTER_VIDEO_AD_POSITION_IN_STREAM" => {
                    FilterPairType::FilterVideoAdPositionInStream
                }
                "FILTER_VIDEO_COMPANION_CREATIVE_SIZE" => {
                    FilterPairType::FilterVideoCompanionCreativeSize
                }
                "FILTER_VIDEO_CONTENT_DURATION" => FilterPairType::FilterVideoContentDuration,
                "FILTER_VIDEO_CONTENT_LIVE_STREAM" => FilterPairType::FilterVideoContentLiveStream,
                "FILTER_VIDEO_CONTINUOUS_PLAY" => FilterPairType::FilterVideoContinuousPlay,
                "FILTER_VIDEO_CREATIVE_DURATION" => FilterPairType::FilterVideoCreativeDuration,
                "FILTER_VIDEO_CREATIVE_DURATION_SKIPPABLE" => {
                    FilterPairType::FilterVideoCreativeDurationSkippable
                }
                "FILTER_VIDEO_DURATION" => FilterPairType::FilterVideoDuration,
                "FILTER_VIDEO_DURATION_SECONDS" => FilterPairType::FilterVideoDurationSeconds,
                "FILTER_VIDEO_DURATION_SECONDS_RANGE" => {
                    FilterPairType::FilterVideoDurationSecondsRange
                }
                "FILTER_VIDEO_FORMAT_SUPPORT" => FilterPairType::FilterVideoFormatSupport,
                "FILTER_VIDEO_PLAYER_SIZE" => FilterPairType::FilterVideoPlayerSize,
                "FILTER_VIDEO_RATING_TIER" => FilterPairType::FilterVideoRatingTier,
                "FILTER_VIDEO_SKIPPABLE_SUPPORT" => FilterPairType::FilterVideoSkippableSupport,
                "FILTER_WEEK" => FilterPairType::FilterWeek,
                "FILTER_YEAR" => FilterPairType::FilterYear,
                "FILTER_YOUTUBE_AD_VIDEO" => FilterPairType::FilterYoutubeAdVideo,
                "FILTER_YOUTUBE_AD_VIDEO_ID" => FilterPairType::FilterYoutubeAdVideoId,
                "FILTER_YOUTUBE_ADAPTED_AUDIENCE_LIST" => {
                    FilterPairType::FilterYoutubeAdaptedAudienceList
                }
                "FILTER_YOUTUBE_CHANNEL" => FilterPairType::FilterYoutubeChannel,
                "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_ADVERTISER" => {
                    FilterPairType::FilterYoutubeProgrammaticGuaranteedAdvertiser
                }
                "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_INSERTION_ORDER" => {
                    FilterPairType::FilterYoutubeProgrammaticGuaranteedInsertionOrder
                }
                "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_PARTNER" => {
                    FilterPairType::FilterYoutubeProgrammaticGuaranteedPartner
                }
                "FILTER_YOUTUBE_VIDEO" => FilterPairType::FilterYoutubeVideo,
                "FILTER_ZIP_CODE" => FilterPairType::FilterZipCode,
                "FILTER_ZIP_POSTAL_CODE" => FilterPairType::FilterZipPostalCode,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for FilterPairType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FilterPairType {
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
    pub struct ListQueriesResponse {
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"doubleclickbidmanager#listQueriesResponse\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Next page's pagination token if one exists."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Retrieved queries."]
        #[serde(
            rename = "queries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub queries: ::std::option::Option<Vec<crate::schemas::Query>>,
    }
    impl ::google_field_selector::FieldSelector for ListQueriesResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListQueriesResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken for ListQueriesResponse {
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
    pub struct ListReportsResponse {
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"doubleclickbidmanager#listReportsResponse\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Next page's pagination token if one exists."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Retrieved reports."]
        #[serde(
            rename = "reports",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub reports: ::std::option::Option<Vec<crate::schemas::Report>>,
    }
    impl ::google_field_selector::FieldSelector for ListReportsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListReportsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken for ListReportsResponse {
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
    pub struct Options {
        #[doc = "Set to true and filter your report by `FILTER_INSERTION_ORDER` or `FILTER_LINE_ITEM` to include data for audience lists specifically targeted by those items."]
        #[serde(
            rename = "includeOnlyTargetedUserLists",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub include_only_targeted_user_lists: ::std::option::Option<bool>,
        #[doc = "Options that contain Path Filters and Custom Channel Groupings."]
        #[serde(
            rename = "pathQueryOptions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub path_query_options: ::std::option::Option<crate::schemas::PathQueryOptions>,
    }
    impl ::google_field_selector::FieldSelector for Options {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Options {
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
    pub struct Parameters {
        #[doc = "Filters used to match traffic data in your report."]
        #[serde(
            rename = "filters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filters: ::std::option::Option<Vec<crate::schemas::FilterPair>>,
        #[doc = "Data is grouped by the filters listed in this field."]
        #[serde(
            rename = "groupBys",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub group_bys: ::std::option::Option<Vec<crate::schemas::ParametersGroupBysItems>>,
        #[doc = "Deprecated. This field is no longer in use."]
        #[serde(
            rename = "includeInviteData",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub include_invite_data: ::std::option::Option<bool>,
        #[doc = "Metrics to include as columns in your report."]
        #[serde(
            rename = "metrics",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metrics: ::std::option::Option<Vec<crate::schemas::ParametersMetricsItems>>,
        #[doc = "Additional query options."]
        #[serde(
            rename = "options",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub options: ::std::option::Option<crate::schemas::Options>,
        #[doc = "Report type."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::ParametersType>,
    }
    impl ::google_field_selector::FieldSelector for Parameters {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Parameters {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ParametersGroupBysItems {
        FilterActiveViewCustomMetricId,
        FilterActiveViewCustomMetricName,
        FilterActiveViewExpectedViewability,
        FilterAdPosition,
        FilterAdType,
        FilterAdvertiser,
        FilterAdvertiserCurrency,
        FilterAdvertiserIntegrationCode,
        FilterAdvertiserIntegrationStatus,
        FilterAdvertiserName,
        FilterAdvertiserTimezone,
        FilterAge,
        FilterAlgorithm,
        FilterAlgorithmId,
        FilterAmpPageRequest,
        FilterAnonymousInventoryModeling,
        FilterAppUrl,
        FilterAppUrlExcluded,
        FilterAttributedUserlist,
        FilterAttributedUserlistCost,
        FilterAttributedUserlistType,
        FilterAttributionModel,
        FilterAudienceList,
        FilterAudienceListCost,
        FilterAudienceListType,
        FilterAudienceName,
        FilterAudienceType,
        FilterAudioFeedTypeName,
        FilterAuthorizedSellerState,
        FilterBillableOutcome,
        FilterBrandLiftType,
        FilterBrowser,
        FilterBudgetSegmentBudget,
        FilterBudgetSegmentDescription,
        FilterBudgetSegmentEndDate,
        FilterBudgetSegmentPacingPercentage,
        FilterBudgetSegmentStartDate,
        FilterBudgetSegmentType,
        FilterCampaignDailyFrequency,
        FilterCarrier,
        FilterCarrierName,
        FilterChannelGrouping,
        FilterChannelId,
        FilterChannelName,
        FilterChannelType,
        FilterCity,
        FilterCityName,
        FilterCm360PlacementId,
        FilterCmPlacementId,
        FilterCompanionCreativeId,
        FilterCompanionCreativeName,
        FilterConversionDelay,
        FilterConversionSource,
        FilterConversionSourceId,
        FilterCountry,
        FilterCountryId,
        FilterCreative,
        FilterCreativeAsset,
        FilterCreativeAttribute,
        FilterCreativeHeight,
        FilterCreativeId,
        FilterCreativeIntegrationCode,
        FilterCreativeRenderedInAmp,
        FilterCreativeSize,
        FilterCreativeSource,
        FilterCreativeStatus,
        FilterCreativeType,
        FilterCreativeWidth,
        FilterDataProvider,
        FilterDataProviderName,
        FilterDataSource,
        FilterDate,
        FilterDayOfWeek,
        FilterDetailedDemographics,
        FilterDetailedDemographicsId,
        FilterDevice,
        FilterDeviceMake,
        FilterDeviceModel,
        FilterDeviceType,
        FilterDfpOrderId,
        FilterDigitalContentLabel,
        FilterDma,
        FilterDmaName,
        FilterDomain,
        FilterEligibleCookiesOnFirstPartyAudienceList,
        FilterEligibleCookiesOnThirdPartyAudienceListAndInterest,
        FilterEventType,
        FilterExchange,
        FilterExchangeCode,
        FilterExchangeId,
        FilterExtension,
        FilterExtensionStatus,
        FilterExtensionType,
        FilterFirstPartyAudienceListCost,
        FilterFirstPartyAudienceListType,
        FilterFloodlightActivity,
        FilterFloodlightActivityId,
        FilterFormat,
        FilterGamInsertionOrder,
        FilterGamLineItem,
        FilterGamLineItemId,
        FilterGender,
        FilterGmailAge,
        FilterGmailCity,
        FilterGmailCountry,
        FilterGmailCountryName,
        FilterGmailDeviceType,
        FilterGmailDeviceTypeName,
        FilterGmailGender,
        FilterGmailRegion,
        FilterGmailRemarketingList,
        FilterHouseholdIncome,
        FilterImpressionCountingMethod,
        FilterImpressionLossRejectionReason,
        FilterInsertionOrder,
        FilterInsertionOrderGoalType,
        FilterInsertionOrderGoalValue,
        FilterInsertionOrderIntegrationCode,
        FilterInsertionOrderName,
        FilterInsertionOrderStatus,
        FilterInterest,
        FilterInventoryCommitmentType,
        FilterInventoryDeliveryMethod,
        FilterInventoryFormat,
        FilterInventoryRateType,
        FilterInventorySource,
        FilterInventorySourceExternalId,
        FilterInventorySourceGroup,
        FilterInventorySourceGroupId,
        FilterInventorySourceId,
        FilterInventorySourceName,
        FilterInventorySourceType,
        FilterKeyword,
        FilterLifeEvent,
        FilterLifeEvents,
        FilterLineItem,
        FilterLineItemBudget,
        FilterLineItemDailyFrequency,
        FilterLineItemEndDate,
        FilterLineItemIntegrationCode,
        FilterLineItemLifetimeFrequency,
        FilterLineItemName,
        FilterLineItemPacingPercentage,
        FilterLineItemStartDate,
        FilterLineItemStatus,
        FilterLineItemType,
        FilterMatchRatio,
        FilterMatchedGenreTarget,
        FilterMeasurementSource,
        FilterMediaPlan,
        FilterMediaPlanName,
        FilterMediaType,
        FilterMobileGeo,
        FilterMonth,
        FilterMraidSupport,
        FilterNielsenAge,
        FilterNielsenCountryCode,
        FilterNielsenDateRange,
        FilterNielsenDeviceId,
        FilterNielsenGender,
        FilterNielsenRestatementDate,
        FilterNotSupported,
        FilterOmSdkAvailable,
        FilterOmidCapable,
        FilterOrderId,
        FilterOs,
        FilterPageCategory,
        FilterPageLayout,
        FilterParentalStatus,
        FilterPartner,
        FilterPartnerCurrency,
        FilterPartnerName,
        FilterPartnerStatus,
        FilterPathEventIndex,
        FilterPathPatternId,
        FilterPlacementAllYoutubeChannels,
        FilterPlacementNameAllYoutubeChannels,
        FilterPlatform,
        FilterPlaybackMethod,
        FilterPositionInContent,
        FilterPublicInventory,
        FilterPublisherProperty,
        FilterPublisherPropertyId,
        FilterPublisherPropertySection,
        FilterPublisherPropertySectionId,
        FilterQuarter,
        FilterRefundReason,
        FilterRegion,
        FilterRegionName,
        FilterRemarketingList,
        FilterRewarded,
        FilterSensitiveCategory,
        FilterServedPixelDensity,
        FilterSiteId,
        FilterSiteLanguage,
        FilterSkippableSupport,
        FilterTargetedDataProviders,
        FilterTargetedUserList,
        FilterThirdPartyAudienceListCost,
        FilterThirdPartyAudienceListType,
        FilterTimeOfDay,
        FilterTrueviewAd,
        FilterTrueviewAdGroup,
        FilterTrueviewAdGroupAdId,
        FilterTrueviewAdGroupId,
        FilterTrueviewAdTypeName,
        FilterTrueviewAge,
        FilterTrueviewCategory,
        FilterTrueviewCity,
        FilterTrueviewClickTypeName,
        FilterTrueviewConversionType,
        FilterTrueviewCountry,
        FilterTrueviewCustomAffinity,
        FilterTrueviewDetailedDemographics,
        FilterTrueviewDetailedDemographicsId,
        FilterTrueviewDma,
        FilterTrueviewDmaName,
        FilterTrueviewGender,
        FilterTrueviewHouseholdIncome,
        FilterTrueviewIarAge,
        FilterTrueviewIarCategory,
        FilterTrueviewIarCity,
        FilterTrueviewIarCountry,
        FilterTrueviewIarCountryName,
        FilterTrueviewIarGender,
        FilterTrueviewIarInterest,
        FilterTrueviewIarLanguage,
        FilterTrueviewIarParentalStatus,
        FilterTrueviewIarRegion,
        FilterTrueviewIarRegionName,
        FilterTrueviewIarRemarketingList,
        FilterTrueviewIarTimeOfDay,
        FilterTrueviewIarYoutubeChannel,
        FilterTrueviewIarYoutubeVideo,
        FilterTrueviewIarZipcode,
        FilterTrueviewInterest,
        FilterTrueviewKeyword,
        FilterTrueviewParentalStatus,
        FilterTrueviewPlacement,
        FilterTrueviewPlacementId,
        FilterTrueviewRegion,
        FilterTrueviewRegionName,
        FilterTrueviewRemarketingList,
        FilterTrueviewRemarketingListName,
        FilterTrueviewUrl,
        FilterTrueviewZipcode,
        FilterUnknown,
        FilterUserList,
        FilterUserListFirstParty,
        FilterUserListFirstPartyName,
        FilterUserListThirdParty,
        FilterUserListThirdPartyName,
        FilterVariantId,
        FilterVariantName,
        FilterVariantVersion,
        FilterVendorMeasurementMode,
        FilterVerificationAudibilityComplete,
        FilterVerificationAudibilityStart,
        FilterVerificationVideoPlayerSize,
        FilterVerificationVideoPlayerSizeComplete,
        FilterVerificationVideoPlayerSizeFirstQuartile,
        FilterVerificationVideoPlayerSizeMidPoint,
        FilterVerificationVideoPlayerSizeStart,
        FilterVerificationVideoPlayerSizeThirdQuartile,
        FilterVerificationVideoPosition,
        FilterVerificationVideoResized,
        FilterVideoAdPositionInStream,
        FilterVideoCompanionCreativeSize,
        FilterVideoContentDuration,
        FilterVideoContentLiveStream,
        FilterVideoContinuousPlay,
        FilterVideoCreativeDuration,
        FilterVideoCreativeDurationSkippable,
        FilterVideoDuration,
        FilterVideoDurationSeconds,
        FilterVideoDurationSecondsRange,
        FilterVideoFormatSupport,
        FilterVideoPlayerSize,
        FilterVideoRatingTier,
        FilterVideoSkippableSupport,
        FilterWeek,
        FilterYear,
        FilterYoutubeAdVideo,
        FilterYoutubeAdVideoId,
        FilterYoutubeAdaptedAudienceList,
        FilterYoutubeChannel,
        FilterYoutubeProgrammaticGuaranteedAdvertiser,
        FilterYoutubeProgrammaticGuaranteedInsertionOrder,
        FilterYoutubeProgrammaticGuaranteedPartner,
        FilterYoutubeVideo,
        FilterZipCode,
        FilterZipPostalCode,
    }
    impl ParametersGroupBysItems {
        pub fn as_str(self) -> &'static str {
            match self { ParametersGroupBysItems :: FilterActiveViewCustomMetricId => "FILTER_ACTIVE_VIEW_CUSTOM_METRIC_ID" , ParametersGroupBysItems :: FilterActiveViewCustomMetricName => "FILTER_ACTIVE_VIEW_CUSTOM_METRIC_NAME" , ParametersGroupBysItems :: FilterActiveViewExpectedViewability => "FILTER_ACTIVE_VIEW_EXPECTED_VIEWABILITY" , ParametersGroupBysItems :: FilterAdPosition => "FILTER_AD_POSITION" , ParametersGroupBysItems :: FilterAdType => "FILTER_AD_TYPE" , ParametersGroupBysItems :: FilterAdvertiser => "FILTER_ADVERTISER" , ParametersGroupBysItems :: FilterAdvertiserCurrency => "FILTER_ADVERTISER_CURRENCY" , ParametersGroupBysItems :: FilterAdvertiserIntegrationCode => "FILTER_ADVERTISER_INTEGRATION_CODE" , ParametersGroupBysItems :: FilterAdvertiserIntegrationStatus => "FILTER_ADVERTISER_INTEGRATION_STATUS" , ParametersGroupBysItems :: FilterAdvertiserName => "FILTER_ADVERTISER_NAME" , ParametersGroupBysItems :: FilterAdvertiserTimezone => "FILTER_ADVERTISER_TIMEZONE" , ParametersGroupBysItems :: FilterAge => "FILTER_AGE" , ParametersGroupBysItems :: FilterAlgorithm => "FILTER_ALGORITHM" , ParametersGroupBysItems :: FilterAlgorithmId => "FILTER_ALGORITHM_ID" , ParametersGroupBysItems :: FilterAmpPageRequest => "FILTER_AMP_PAGE_REQUEST" , ParametersGroupBysItems :: FilterAnonymousInventoryModeling => "FILTER_ANONYMOUS_INVENTORY_MODELING" , ParametersGroupBysItems :: FilterAppUrl => "FILTER_APP_URL" , ParametersGroupBysItems :: FilterAppUrlExcluded => "FILTER_APP_URL_EXCLUDED" , ParametersGroupBysItems :: FilterAttributedUserlist => "FILTER_ATTRIBUTED_USERLIST" , ParametersGroupBysItems :: FilterAttributedUserlistCost => "FILTER_ATTRIBUTED_USERLIST_COST" , ParametersGroupBysItems :: FilterAttributedUserlistType => "FILTER_ATTRIBUTED_USERLIST_TYPE" , ParametersGroupBysItems :: FilterAttributionModel => "FILTER_ATTRIBUTION_MODEL" , ParametersGroupBysItems :: FilterAudienceList => "FILTER_AUDIENCE_LIST" , ParametersGroupBysItems :: FilterAudienceListCost => "FILTER_AUDIENCE_LIST_COST" , ParametersGroupBysItems :: FilterAudienceListType => "FILTER_AUDIENCE_LIST_TYPE" , ParametersGroupBysItems :: FilterAudienceName => "FILTER_AUDIENCE_NAME" , ParametersGroupBysItems :: FilterAudienceType => "FILTER_AUDIENCE_TYPE" , ParametersGroupBysItems :: FilterAudioFeedTypeName => "FILTER_AUDIO_FEED_TYPE_NAME" , ParametersGroupBysItems :: FilterAuthorizedSellerState => "FILTER_AUTHORIZED_SELLER_STATE" , ParametersGroupBysItems :: FilterBillableOutcome => "FILTER_BILLABLE_OUTCOME" , ParametersGroupBysItems :: FilterBrandLiftType => "FILTER_BRAND_LIFT_TYPE" , ParametersGroupBysItems :: FilterBrowser => "FILTER_BROWSER" , ParametersGroupBysItems :: FilterBudgetSegmentBudget => "FILTER_BUDGET_SEGMENT_BUDGET" , ParametersGroupBysItems :: FilterBudgetSegmentDescription => "FILTER_BUDGET_SEGMENT_DESCRIPTION" , ParametersGroupBysItems :: FilterBudgetSegmentEndDate => "FILTER_BUDGET_SEGMENT_END_DATE" , ParametersGroupBysItems :: FilterBudgetSegmentPacingPercentage => "FILTER_BUDGET_SEGMENT_PACING_PERCENTAGE" , ParametersGroupBysItems :: FilterBudgetSegmentStartDate => "FILTER_BUDGET_SEGMENT_START_DATE" , ParametersGroupBysItems :: FilterBudgetSegmentType => "FILTER_BUDGET_SEGMENT_TYPE" , ParametersGroupBysItems :: FilterCampaignDailyFrequency => "FILTER_CAMPAIGN_DAILY_FREQUENCY" , ParametersGroupBysItems :: FilterCarrier => "FILTER_CARRIER" , ParametersGroupBysItems :: FilterCarrierName => "FILTER_CARRIER_NAME" , ParametersGroupBysItems :: FilterChannelGrouping => "FILTER_CHANNEL_GROUPING" , ParametersGroupBysItems :: FilterChannelId => "FILTER_CHANNEL_ID" , ParametersGroupBysItems :: FilterChannelName => "FILTER_CHANNEL_NAME" , ParametersGroupBysItems :: FilterChannelType => "FILTER_CHANNEL_TYPE" , ParametersGroupBysItems :: FilterCity => "FILTER_CITY" , ParametersGroupBysItems :: FilterCityName => "FILTER_CITY_NAME" , ParametersGroupBysItems :: FilterCm360PlacementId => "FILTER_CM360_PLACEMENT_ID" , ParametersGroupBysItems :: FilterCmPlacementId => "FILTER_CM_PLACEMENT_ID" , ParametersGroupBysItems :: FilterCompanionCreativeId => "FILTER_COMPANION_CREATIVE_ID" , ParametersGroupBysItems :: FilterCompanionCreativeName => "FILTER_COMPANION_CREATIVE_NAME" , ParametersGroupBysItems :: FilterConversionDelay => "FILTER_CONVERSION_DELAY" , ParametersGroupBysItems :: FilterConversionSource => "FILTER_CONVERSION_SOURCE" , ParametersGroupBysItems :: FilterConversionSourceId => "FILTER_CONVERSION_SOURCE_ID" , ParametersGroupBysItems :: FilterCountry => "FILTER_COUNTRY" , ParametersGroupBysItems :: FilterCountryId => "FILTER_COUNTRY_ID" , ParametersGroupBysItems :: FilterCreative => "FILTER_CREATIVE" , ParametersGroupBysItems :: FilterCreativeAsset => "FILTER_CREATIVE_ASSET" , ParametersGroupBysItems :: FilterCreativeAttribute => "FILTER_CREATIVE_ATTRIBUTE" , ParametersGroupBysItems :: FilterCreativeHeight => "FILTER_CREATIVE_HEIGHT" , ParametersGroupBysItems :: FilterCreativeId => "FILTER_CREATIVE_ID" , ParametersGroupBysItems :: FilterCreativeIntegrationCode => "FILTER_CREATIVE_INTEGRATION_CODE" , ParametersGroupBysItems :: FilterCreativeRenderedInAmp => "FILTER_CREATIVE_RENDERED_IN_AMP" , ParametersGroupBysItems :: FilterCreativeSize => "FILTER_CREATIVE_SIZE" , ParametersGroupBysItems :: FilterCreativeSource => "FILTER_CREATIVE_SOURCE" , ParametersGroupBysItems :: FilterCreativeStatus => "FILTER_CREATIVE_STATUS" , ParametersGroupBysItems :: FilterCreativeType => "FILTER_CREATIVE_TYPE" , ParametersGroupBysItems :: FilterCreativeWidth => "FILTER_CREATIVE_WIDTH" , ParametersGroupBysItems :: FilterDataProvider => "FILTER_DATA_PROVIDER" , ParametersGroupBysItems :: FilterDataProviderName => "FILTER_DATA_PROVIDER_NAME" , ParametersGroupBysItems :: FilterDataSource => "FILTER_DATA_SOURCE" , ParametersGroupBysItems :: FilterDate => "FILTER_DATE" , ParametersGroupBysItems :: FilterDayOfWeek => "FILTER_DAY_OF_WEEK" , ParametersGroupBysItems :: FilterDetailedDemographics => "FILTER_DETAILED_DEMOGRAPHICS" , ParametersGroupBysItems :: FilterDetailedDemographicsId => "FILTER_DETAILED_DEMOGRAPHICS_ID" , ParametersGroupBysItems :: FilterDevice => "FILTER_DEVICE" , ParametersGroupBysItems :: FilterDeviceMake => "FILTER_DEVICE_MAKE" , ParametersGroupBysItems :: FilterDeviceModel => "FILTER_DEVICE_MODEL" , ParametersGroupBysItems :: FilterDeviceType => "FILTER_DEVICE_TYPE" , ParametersGroupBysItems :: FilterDfpOrderId => "FILTER_DFP_ORDER_ID" , ParametersGroupBysItems :: FilterDigitalContentLabel => "FILTER_DIGITAL_CONTENT_LABEL" , ParametersGroupBysItems :: FilterDma => "FILTER_DMA" , ParametersGroupBysItems :: FilterDmaName => "FILTER_DMA_NAME" , ParametersGroupBysItems :: FilterDomain => "FILTER_DOMAIN" , ParametersGroupBysItems :: FilterEligibleCookiesOnFirstPartyAudienceList => "FILTER_ELIGIBLE_COOKIES_ON_FIRST_PARTY_AUDIENCE_LIST" , ParametersGroupBysItems :: FilterEligibleCookiesOnThirdPartyAudienceListAndInterest => "FILTER_ELIGIBLE_COOKIES_ON_THIRD_PARTY_AUDIENCE_LIST_AND_INTEREST" , ParametersGroupBysItems :: FilterEventType => "FILTER_EVENT_TYPE" , ParametersGroupBysItems :: FilterExchange => "FILTER_EXCHANGE" , ParametersGroupBysItems :: FilterExchangeCode => "FILTER_EXCHANGE_CODE" , ParametersGroupBysItems :: FilterExchangeId => "FILTER_EXCHANGE_ID" , ParametersGroupBysItems :: FilterExtension => "FILTER_EXTENSION" , ParametersGroupBysItems :: FilterExtensionStatus => "FILTER_EXTENSION_STATUS" , ParametersGroupBysItems :: FilterExtensionType => "FILTER_EXTENSION_TYPE" , ParametersGroupBysItems :: FilterFirstPartyAudienceListCost => "FILTER_FIRST_PARTY_AUDIENCE_LIST_COST" , ParametersGroupBysItems :: FilterFirstPartyAudienceListType => "FILTER_FIRST_PARTY_AUDIENCE_LIST_TYPE" , ParametersGroupBysItems :: FilterFloodlightActivity => "FILTER_FLOODLIGHT_ACTIVITY" , ParametersGroupBysItems :: FilterFloodlightActivityId => "FILTER_FLOODLIGHT_ACTIVITY_ID" , ParametersGroupBysItems :: FilterFormat => "FILTER_FORMAT" , ParametersGroupBysItems :: FilterGamInsertionOrder => "FILTER_GAM_INSERTION_ORDER" , ParametersGroupBysItems :: FilterGamLineItem => "FILTER_GAM_LINE_ITEM" , ParametersGroupBysItems :: FilterGamLineItemId => "FILTER_GAM_LINE_ITEM_ID" , ParametersGroupBysItems :: FilterGender => "FILTER_GENDER" , ParametersGroupBysItems :: FilterGmailAge => "FILTER_GMAIL_AGE" , ParametersGroupBysItems :: FilterGmailCity => "FILTER_GMAIL_CITY" , ParametersGroupBysItems :: FilterGmailCountry => "FILTER_GMAIL_COUNTRY" , ParametersGroupBysItems :: FilterGmailCountryName => "FILTER_GMAIL_COUNTRY_NAME" , ParametersGroupBysItems :: FilterGmailDeviceType => "FILTER_GMAIL_DEVICE_TYPE" , ParametersGroupBysItems :: FilterGmailDeviceTypeName => "FILTER_GMAIL_DEVICE_TYPE_NAME" , ParametersGroupBysItems :: FilterGmailGender => "FILTER_GMAIL_GENDER" , ParametersGroupBysItems :: FilterGmailRegion => "FILTER_GMAIL_REGION" , ParametersGroupBysItems :: FilterGmailRemarketingList => "FILTER_GMAIL_REMARKETING_LIST" , ParametersGroupBysItems :: FilterHouseholdIncome => "FILTER_HOUSEHOLD_INCOME" , ParametersGroupBysItems :: FilterImpressionCountingMethod => "FILTER_IMPRESSION_COUNTING_METHOD" , ParametersGroupBysItems :: FilterImpressionLossRejectionReason => "FILTER_IMPRESSION_LOSS_REJECTION_REASON" , ParametersGroupBysItems :: FilterInsertionOrder => "FILTER_INSERTION_ORDER" , ParametersGroupBysItems :: FilterInsertionOrderGoalType => "FILTER_INSERTION_ORDER_GOAL_TYPE" , ParametersGroupBysItems :: FilterInsertionOrderGoalValue => "FILTER_INSERTION_ORDER_GOAL_VALUE" , ParametersGroupBysItems :: FilterInsertionOrderIntegrationCode => "FILTER_INSERTION_ORDER_INTEGRATION_CODE" , ParametersGroupBysItems :: FilterInsertionOrderName => "FILTER_INSERTION_ORDER_NAME" , ParametersGroupBysItems :: FilterInsertionOrderStatus => "FILTER_INSERTION_ORDER_STATUS" , ParametersGroupBysItems :: FilterInterest => "FILTER_INTEREST" , ParametersGroupBysItems :: FilterInventoryCommitmentType => "FILTER_INVENTORY_COMMITMENT_TYPE" , ParametersGroupBysItems :: FilterInventoryDeliveryMethod => "FILTER_INVENTORY_DELIVERY_METHOD" , ParametersGroupBysItems :: FilterInventoryFormat => "FILTER_INVENTORY_FORMAT" , ParametersGroupBysItems :: FilterInventoryRateType => "FILTER_INVENTORY_RATE_TYPE" , ParametersGroupBysItems :: FilterInventorySource => "FILTER_INVENTORY_SOURCE" , ParametersGroupBysItems :: FilterInventorySourceExternalId => "FILTER_INVENTORY_SOURCE_EXTERNAL_ID" , ParametersGroupBysItems :: FilterInventorySourceGroup => "FILTER_INVENTORY_SOURCE_GROUP" , ParametersGroupBysItems :: FilterInventorySourceGroupId => "FILTER_INVENTORY_SOURCE_GROUP_ID" , ParametersGroupBysItems :: FilterInventorySourceId => "FILTER_INVENTORY_SOURCE_ID" , ParametersGroupBysItems :: FilterInventorySourceName => "FILTER_INVENTORY_SOURCE_NAME" , ParametersGroupBysItems :: FilterInventorySourceType => "FILTER_INVENTORY_SOURCE_TYPE" , ParametersGroupBysItems :: FilterKeyword => "FILTER_KEYWORD" , ParametersGroupBysItems :: FilterLifeEvent => "FILTER_LIFE_EVENT" , ParametersGroupBysItems :: FilterLifeEvents => "FILTER_LIFE_EVENTS" , ParametersGroupBysItems :: FilterLineItem => "FILTER_LINE_ITEM" , ParametersGroupBysItems :: FilterLineItemBudget => "FILTER_LINE_ITEM_BUDGET" , ParametersGroupBysItems :: FilterLineItemDailyFrequency => "FILTER_LINE_ITEM_DAILY_FREQUENCY" , ParametersGroupBysItems :: FilterLineItemEndDate => "FILTER_LINE_ITEM_END_DATE" , ParametersGroupBysItems :: FilterLineItemIntegrationCode => "FILTER_LINE_ITEM_INTEGRATION_CODE" , ParametersGroupBysItems :: FilterLineItemLifetimeFrequency => "FILTER_LINE_ITEM_LIFETIME_FREQUENCY" , ParametersGroupBysItems :: FilterLineItemName => "FILTER_LINE_ITEM_NAME" , ParametersGroupBysItems :: FilterLineItemPacingPercentage => "FILTER_LINE_ITEM_PACING_PERCENTAGE" , ParametersGroupBysItems :: FilterLineItemStartDate => "FILTER_LINE_ITEM_START_DATE" , ParametersGroupBysItems :: FilterLineItemStatus => "FILTER_LINE_ITEM_STATUS" , ParametersGroupBysItems :: FilterLineItemType => "FILTER_LINE_ITEM_TYPE" , ParametersGroupBysItems :: FilterMatchRatio => "FILTER_MATCH_RATIO" , ParametersGroupBysItems :: FilterMatchedGenreTarget => "FILTER_MATCHED_GENRE_TARGET" , ParametersGroupBysItems :: FilterMeasurementSource => "FILTER_MEASUREMENT_SOURCE" , ParametersGroupBysItems :: FilterMediaPlan => "FILTER_MEDIA_PLAN" , ParametersGroupBysItems :: FilterMediaPlanName => "FILTER_MEDIA_PLAN_NAME" , ParametersGroupBysItems :: FilterMediaType => "FILTER_MEDIA_TYPE" , ParametersGroupBysItems :: FilterMobileGeo => "FILTER_MOBILE_GEO" , ParametersGroupBysItems :: FilterMonth => "FILTER_MONTH" , ParametersGroupBysItems :: FilterMraidSupport => "FILTER_MRAID_SUPPORT" , ParametersGroupBysItems :: FilterNielsenAge => "FILTER_NIELSEN_AGE" , ParametersGroupBysItems :: FilterNielsenCountryCode => "FILTER_NIELSEN_COUNTRY_CODE" , ParametersGroupBysItems :: FilterNielsenDateRange => "FILTER_NIELSEN_DATE_RANGE" , ParametersGroupBysItems :: FilterNielsenDeviceId => "FILTER_NIELSEN_DEVICE_ID" , ParametersGroupBysItems :: FilterNielsenGender => "FILTER_NIELSEN_GENDER" , ParametersGroupBysItems :: FilterNielsenRestatementDate => "FILTER_NIELSEN_RESTATEMENT_DATE" , ParametersGroupBysItems :: FilterNotSupported => "FILTER_NOT_SUPPORTED" , ParametersGroupBysItems :: FilterOmSdkAvailable => "FILTER_OM_SDK_AVAILABLE" , ParametersGroupBysItems :: FilterOmidCapable => "FILTER_OMID_CAPABLE" , ParametersGroupBysItems :: FilterOrderId => "FILTER_ORDER_ID" , ParametersGroupBysItems :: FilterOs => "FILTER_OS" , ParametersGroupBysItems :: FilterPageCategory => "FILTER_PAGE_CATEGORY" , ParametersGroupBysItems :: FilterPageLayout => "FILTER_PAGE_LAYOUT" , ParametersGroupBysItems :: FilterParentalStatus => "FILTER_PARENTAL_STATUS" , ParametersGroupBysItems :: FilterPartner => "FILTER_PARTNER" , ParametersGroupBysItems :: FilterPartnerCurrency => "FILTER_PARTNER_CURRENCY" , ParametersGroupBysItems :: FilterPartnerName => "FILTER_PARTNER_NAME" , ParametersGroupBysItems :: FilterPartnerStatus => "FILTER_PARTNER_STATUS" , ParametersGroupBysItems :: FilterPathEventIndex => "FILTER_PATH_EVENT_INDEX" , ParametersGroupBysItems :: FilterPathPatternId => "FILTER_PATH_PATTERN_ID" , ParametersGroupBysItems :: FilterPlacementAllYoutubeChannels => "FILTER_PLACEMENT_ALL_YOUTUBE_CHANNELS" , ParametersGroupBysItems :: FilterPlacementNameAllYoutubeChannels => "FILTER_PLACEMENT_NAME_ALL_YOUTUBE_CHANNELS" , ParametersGroupBysItems :: FilterPlatform => "FILTER_PLATFORM" , ParametersGroupBysItems :: FilterPlaybackMethod => "FILTER_PLAYBACK_METHOD" , ParametersGroupBysItems :: FilterPositionInContent => "FILTER_POSITION_IN_CONTENT" , ParametersGroupBysItems :: FilterPublicInventory => "FILTER_PUBLIC_INVENTORY" , ParametersGroupBysItems :: FilterPublisherProperty => "FILTER_PUBLISHER_PROPERTY" , ParametersGroupBysItems :: FilterPublisherPropertyId => "FILTER_PUBLISHER_PROPERTY_ID" , ParametersGroupBysItems :: FilterPublisherPropertySection => "FILTER_PUBLISHER_PROPERTY_SECTION" , ParametersGroupBysItems :: FilterPublisherPropertySectionId => "FILTER_PUBLISHER_PROPERTY_SECTION_ID" , ParametersGroupBysItems :: FilterQuarter => "FILTER_QUARTER" , ParametersGroupBysItems :: FilterRefundReason => "FILTER_REFUND_REASON" , ParametersGroupBysItems :: FilterRegion => "FILTER_REGION" , ParametersGroupBysItems :: FilterRegionName => "FILTER_REGION_NAME" , ParametersGroupBysItems :: FilterRemarketingList => "FILTER_REMARKETING_LIST" , ParametersGroupBysItems :: FilterRewarded => "FILTER_REWARDED" , ParametersGroupBysItems :: FilterSensitiveCategory => "FILTER_SENSITIVE_CATEGORY" , ParametersGroupBysItems :: FilterServedPixelDensity => "FILTER_SERVED_PIXEL_DENSITY" , ParametersGroupBysItems :: FilterSiteId => "FILTER_SITE_ID" , ParametersGroupBysItems :: FilterSiteLanguage => "FILTER_SITE_LANGUAGE" , ParametersGroupBysItems :: FilterSkippableSupport => "FILTER_SKIPPABLE_SUPPORT" , ParametersGroupBysItems :: FilterTargetedDataProviders => "FILTER_TARGETED_DATA_PROVIDERS" , ParametersGroupBysItems :: FilterTargetedUserList => "FILTER_TARGETED_USER_LIST" , ParametersGroupBysItems :: FilterThirdPartyAudienceListCost => "FILTER_THIRD_PARTY_AUDIENCE_LIST_COST" , ParametersGroupBysItems :: FilterThirdPartyAudienceListType => "FILTER_THIRD_PARTY_AUDIENCE_LIST_TYPE" , ParametersGroupBysItems :: FilterTimeOfDay => "FILTER_TIME_OF_DAY" , ParametersGroupBysItems :: FilterTrueviewAd => "FILTER_TRUEVIEW_AD" , ParametersGroupBysItems :: FilterTrueviewAdGroup => "FILTER_TRUEVIEW_AD_GROUP" , ParametersGroupBysItems :: FilterTrueviewAdGroupAdId => "FILTER_TRUEVIEW_AD_GROUP_AD_ID" , ParametersGroupBysItems :: FilterTrueviewAdGroupId => "FILTER_TRUEVIEW_AD_GROUP_ID" , ParametersGroupBysItems :: FilterTrueviewAdTypeName => "FILTER_TRUEVIEW_AD_TYPE_NAME" , ParametersGroupBysItems :: FilterTrueviewAge => "FILTER_TRUEVIEW_AGE" , ParametersGroupBysItems :: FilterTrueviewCategory => "FILTER_TRUEVIEW_CATEGORY" , ParametersGroupBysItems :: FilterTrueviewCity => "FILTER_TRUEVIEW_CITY" , ParametersGroupBysItems :: FilterTrueviewClickTypeName => "FILTER_TRUEVIEW_CLICK_TYPE_NAME" , ParametersGroupBysItems :: FilterTrueviewConversionType => "FILTER_TRUEVIEW_CONVERSION_TYPE" , ParametersGroupBysItems :: FilterTrueviewCountry => "FILTER_TRUEVIEW_COUNTRY" , ParametersGroupBysItems :: FilterTrueviewCustomAffinity => "FILTER_TRUEVIEW_CUSTOM_AFFINITY" , ParametersGroupBysItems :: FilterTrueviewDetailedDemographics => "FILTER_TRUEVIEW_DETAILED_DEMOGRAPHICS" , ParametersGroupBysItems :: FilterTrueviewDetailedDemographicsId => "FILTER_TRUEVIEW_DETAILED_DEMOGRAPHICS_ID" , ParametersGroupBysItems :: FilterTrueviewDma => "FILTER_TRUEVIEW_DMA" , ParametersGroupBysItems :: FilterTrueviewDmaName => "FILTER_TRUEVIEW_DMA_NAME" , ParametersGroupBysItems :: FilterTrueviewGender => "FILTER_TRUEVIEW_GENDER" , ParametersGroupBysItems :: FilterTrueviewHouseholdIncome => "FILTER_TRUEVIEW_HOUSEHOLD_INCOME" , ParametersGroupBysItems :: FilterTrueviewIarAge => "FILTER_TRUEVIEW_IAR_AGE" , ParametersGroupBysItems :: FilterTrueviewIarCategory => "FILTER_TRUEVIEW_IAR_CATEGORY" , ParametersGroupBysItems :: FilterTrueviewIarCity => "FILTER_TRUEVIEW_IAR_CITY" , ParametersGroupBysItems :: FilterTrueviewIarCountry => "FILTER_TRUEVIEW_IAR_COUNTRY" , ParametersGroupBysItems :: FilterTrueviewIarCountryName => "FILTER_TRUEVIEW_IAR_COUNTRY_NAME" , ParametersGroupBysItems :: FilterTrueviewIarGender => "FILTER_TRUEVIEW_IAR_GENDER" , ParametersGroupBysItems :: FilterTrueviewIarInterest => "FILTER_TRUEVIEW_IAR_INTEREST" , ParametersGroupBysItems :: FilterTrueviewIarLanguage => "FILTER_TRUEVIEW_IAR_LANGUAGE" , ParametersGroupBysItems :: FilterTrueviewIarParentalStatus => "FILTER_TRUEVIEW_IAR_PARENTAL_STATUS" , ParametersGroupBysItems :: FilterTrueviewIarRegion => "FILTER_TRUEVIEW_IAR_REGION" , ParametersGroupBysItems :: FilterTrueviewIarRegionName => "FILTER_TRUEVIEW_IAR_REGION_NAME" , ParametersGroupBysItems :: FilterTrueviewIarRemarketingList => "FILTER_TRUEVIEW_IAR_REMARKETING_LIST" , ParametersGroupBysItems :: FilterTrueviewIarTimeOfDay => "FILTER_TRUEVIEW_IAR_TIME_OF_DAY" , ParametersGroupBysItems :: FilterTrueviewIarYoutubeChannel => "FILTER_TRUEVIEW_IAR_YOUTUBE_CHANNEL" , ParametersGroupBysItems :: FilterTrueviewIarYoutubeVideo => "FILTER_TRUEVIEW_IAR_YOUTUBE_VIDEO" , ParametersGroupBysItems :: FilterTrueviewIarZipcode => "FILTER_TRUEVIEW_IAR_ZIPCODE" , ParametersGroupBysItems :: FilterTrueviewInterest => "FILTER_TRUEVIEW_INTEREST" , ParametersGroupBysItems :: FilterTrueviewKeyword => "FILTER_TRUEVIEW_KEYWORD" , ParametersGroupBysItems :: FilterTrueviewParentalStatus => "FILTER_TRUEVIEW_PARENTAL_STATUS" , ParametersGroupBysItems :: FilterTrueviewPlacement => "FILTER_TRUEVIEW_PLACEMENT" , ParametersGroupBysItems :: FilterTrueviewPlacementId => "FILTER_TRUEVIEW_PLACEMENT_ID" , ParametersGroupBysItems :: FilterTrueviewRegion => "FILTER_TRUEVIEW_REGION" , ParametersGroupBysItems :: FilterTrueviewRegionName => "FILTER_TRUEVIEW_REGION_NAME" , ParametersGroupBysItems :: FilterTrueviewRemarketingList => "FILTER_TRUEVIEW_REMARKETING_LIST" , ParametersGroupBysItems :: FilterTrueviewRemarketingListName => "FILTER_TRUEVIEW_REMARKETING_LIST_NAME" , ParametersGroupBysItems :: FilterTrueviewUrl => "FILTER_TRUEVIEW_URL" , ParametersGroupBysItems :: FilterTrueviewZipcode => "FILTER_TRUEVIEW_ZIPCODE" , ParametersGroupBysItems :: FilterUnknown => "FILTER_UNKNOWN" , ParametersGroupBysItems :: FilterUserList => "FILTER_USER_LIST" , ParametersGroupBysItems :: FilterUserListFirstParty => "FILTER_USER_LIST_FIRST_PARTY" , ParametersGroupBysItems :: FilterUserListFirstPartyName => "FILTER_USER_LIST_FIRST_PARTY_NAME" , ParametersGroupBysItems :: FilterUserListThirdParty => "FILTER_USER_LIST_THIRD_PARTY" , ParametersGroupBysItems :: FilterUserListThirdPartyName => "FILTER_USER_LIST_THIRD_PARTY_NAME" , ParametersGroupBysItems :: FilterVariantId => "FILTER_VARIANT_ID" , ParametersGroupBysItems :: FilterVariantName => "FILTER_VARIANT_NAME" , ParametersGroupBysItems :: FilterVariantVersion => "FILTER_VARIANT_VERSION" , ParametersGroupBysItems :: FilterVendorMeasurementMode => "FILTER_VENDOR_MEASUREMENT_MODE" , ParametersGroupBysItems :: FilterVerificationAudibilityComplete => "FILTER_VERIFICATION_AUDIBILITY_COMPLETE" , ParametersGroupBysItems :: FilterVerificationAudibilityStart => "FILTER_VERIFICATION_AUDIBILITY_START" , ParametersGroupBysItems :: FilterVerificationVideoPlayerSize => "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE" , ParametersGroupBysItems :: FilterVerificationVideoPlayerSizeComplete => "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_COMPLETE" , ParametersGroupBysItems :: FilterVerificationVideoPlayerSizeFirstQuartile => "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_FIRST_QUARTILE" , ParametersGroupBysItems :: FilterVerificationVideoPlayerSizeMidPoint => "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_MID_POINT" , ParametersGroupBysItems :: FilterVerificationVideoPlayerSizeStart => "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_START" , ParametersGroupBysItems :: FilterVerificationVideoPlayerSizeThirdQuartile => "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_THIRD_QUARTILE" , ParametersGroupBysItems :: FilterVerificationVideoPosition => "FILTER_VERIFICATION_VIDEO_POSITION" , ParametersGroupBysItems :: FilterVerificationVideoResized => "FILTER_VERIFICATION_VIDEO_RESIZED" , ParametersGroupBysItems :: FilterVideoAdPositionInStream => "FILTER_VIDEO_AD_POSITION_IN_STREAM" , ParametersGroupBysItems :: FilterVideoCompanionCreativeSize => "FILTER_VIDEO_COMPANION_CREATIVE_SIZE" , ParametersGroupBysItems :: FilterVideoContentDuration => "FILTER_VIDEO_CONTENT_DURATION" , ParametersGroupBysItems :: FilterVideoContentLiveStream => "FILTER_VIDEO_CONTENT_LIVE_STREAM" , ParametersGroupBysItems :: FilterVideoContinuousPlay => "FILTER_VIDEO_CONTINUOUS_PLAY" , ParametersGroupBysItems :: FilterVideoCreativeDuration => "FILTER_VIDEO_CREATIVE_DURATION" , ParametersGroupBysItems :: FilterVideoCreativeDurationSkippable => "FILTER_VIDEO_CREATIVE_DURATION_SKIPPABLE" , ParametersGroupBysItems :: FilterVideoDuration => "FILTER_VIDEO_DURATION" , ParametersGroupBysItems :: FilterVideoDurationSeconds => "FILTER_VIDEO_DURATION_SECONDS" , ParametersGroupBysItems :: FilterVideoDurationSecondsRange => "FILTER_VIDEO_DURATION_SECONDS_RANGE" , ParametersGroupBysItems :: FilterVideoFormatSupport => "FILTER_VIDEO_FORMAT_SUPPORT" , ParametersGroupBysItems :: FilterVideoPlayerSize => "FILTER_VIDEO_PLAYER_SIZE" , ParametersGroupBysItems :: FilterVideoRatingTier => "FILTER_VIDEO_RATING_TIER" , ParametersGroupBysItems :: FilterVideoSkippableSupport => "FILTER_VIDEO_SKIPPABLE_SUPPORT" , ParametersGroupBysItems :: FilterWeek => "FILTER_WEEK" , ParametersGroupBysItems :: FilterYear => "FILTER_YEAR" , ParametersGroupBysItems :: FilterYoutubeAdVideo => "FILTER_YOUTUBE_AD_VIDEO" , ParametersGroupBysItems :: FilterYoutubeAdVideoId => "FILTER_YOUTUBE_AD_VIDEO_ID" , ParametersGroupBysItems :: FilterYoutubeAdaptedAudienceList => "FILTER_YOUTUBE_ADAPTED_AUDIENCE_LIST" , ParametersGroupBysItems :: FilterYoutubeChannel => "FILTER_YOUTUBE_CHANNEL" , ParametersGroupBysItems :: FilterYoutubeProgrammaticGuaranteedAdvertiser => "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_ADVERTISER" , ParametersGroupBysItems :: FilterYoutubeProgrammaticGuaranteedInsertionOrder => "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_INSERTION_ORDER" , ParametersGroupBysItems :: FilterYoutubeProgrammaticGuaranteedPartner => "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_PARTNER" , ParametersGroupBysItems :: FilterYoutubeVideo => "FILTER_YOUTUBE_VIDEO" , ParametersGroupBysItems :: FilterZipCode => "FILTER_ZIP_CODE" , ParametersGroupBysItems :: FilterZipPostalCode => "FILTER_ZIP_POSTAL_CODE" , }
        }
    }
    impl ::std::convert::AsRef<str> for ParametersGroupBysItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ParametersGroupBysItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ParametersGroupBysItems, ()> {
            Ok (match s { "FILTER_ACTIVE_VIEW_CUSTOM_METRIC_ID" => ParametersGroupBysItems :: FilterActiveViewCustomMetricId , "FILTER_ACTIVE_VIEW_CUSTOM_METRIC_NAME" => ParametersGroupBysItems :: FilterActiveViewCustomMetricName , "FILTER_ACTIVE_VIEW_EXPECTED_VIEWABILITY" => ParametersGroupBysItems :: FilterActiveViewExpectedViewability , "FILTER_AD_POSITION" => ParametersGroupBysItems :: FilterAdPosition , "FILTER_AD_TYPE" => ParametersGroupBysItems :: FilterAdType , "FILTER_ADVERTISER" => ParametersGroupBysItems :: FilterAdvertiser , "FILTER_ADVERTISER_CURRENCY" => ParametersGroupBysItems :: FilterAdvertiserCurrency , "FILTER_ADVERTISER_INTEGRATION_CODE" => ParametersGroupBysItems :: FilterAdvertiserIntegrationCode , "FILTER_ADVERTISER_INTEGRATION_STATUS" => ParametersGroupBysItems :: FilterAdvertiserIntegrationStatus , "FILTER_ADVERTISER_NAME" => ParametersGroupBysItems :: FilterAdvertiserName , "FILTER_ADVERTISER_TIMEZONE" => ParametersGroupBysItems :: FilterAdvertiserTimezone , "FILTER_AGE" => ParametersGroupBysItems :: FilterAge , "FILTER_ALGORITHM" => ParametersGroupBysItems :: FilterAlgorithm , "FILTER_ALGORITHM_ID" => ParametersGroupBysItems :: FilterAlgorithmId , "FILTER_AMP_PAGE_REQUEST" => ParametersGroupBysItems :: FilterAmpPageRequest , "FILTER_ANONYMOUS_INVENTORY_MODELING" => ParametersGroupBysItems :: FilterAnonymousInventoryModeling , "FILTER_APP_URL" => ParametersGroupBysItems :: FilterAppUrl , "FILTER_APP_URL_EXCLUDED" => ParametersGroupBysItems :: FilterAppUrlExcluded , "FILTER_ATTRIBUTED_USERLIST" => ParametersGroupBysItems :: FilterAttributedUserlist , "FILTER_ATTRIBUTED_USERLIST_COST" => ParametersGroupBysItems :: FilterAttributedUserlistCost , "FILTER_ATTRIBUTED_USERLIST_TYPE" => ParametersGroupBysItems :: FilterAttributedUserlistType , "FILTER_ATTRIBUTION_MODEL" => ParametersGroupBysItems :: FilterAttributionModel , "FILTER_AUDIENCE_LIST" => ParametersGroupBysItems :: FilterAudienceList , "FILTER_AUDIENCE_LIST_COST" => ParametersGroupBysItems :: FilterAudienceListCost , "FILTER_AUDIENCE_LIST_TYPE" => ParametersGroupBysItems :: FilterAudienceListType , "FILTER_AUDIENCE_NAME" => ParametersGroupBysItems :: FilterAudienceName , "FILTER_AUDIENCE_TYPE" => ParametersGroupBysItems :: FilterAudienceType , "FILTER_AUDIO_FEED_TYPE_NAME" => ParametersGroupBysItems :: FilterAudioFeedTypeName , "FILTER_AUTHORIZED_SELLER_STATE" => ParametersGroupBysItems :: FilterAuthorizedSellerState , "FILTER_BILLABLE_OUTCOME" => ParametersGroupBysItems :: FilterBillableOutcome , "FILTER_BRAND_LIFT_TYPE" => ParametersGroupBysItems :: FilterBrandLiftType , "FILTER_BROWSER" => ParametersGroupBysItems :: FilterBrowser , "FILTER_BUDGET_SEGMENT_BUDGET" => ParametersGroupBysItems :: FilterBudgetSegmentBudget , "FILTER_BUDGET_SEGMENT_DESCRIPTION" => ParametersGroupBysItems :: FilterBudgetSegmentDescription , "FILTER_BUDGET_SEGMENT_END_DATE" => ParametersGroupBysItems :: FilterBudgetSegmentEndDate , "FILTER_BUDGET_SEGMENT_PACING_PERCENTAGE" => ParametersGroupBysItems :: FilterBudgetSegmentPacingPercentage , "FILTER_BUDGET_SEGMENT_START_DATE" => ParametersGroupBysItems :: FilterBudgetSegmentStartDate , "FILTER_BUDGET_SEGMENT_TYPE" => ParametersGroupBysItems :: FilterBudgetSegmentType , "FILTER_CAMPAIGN_DAILY_FREQUENCY" => ParametersGroupBysItems :: FilterCampaignDailyFrequency , "FILTER_CARRIER" => ParametersGroupBysItems :: FilterCarrier , "FILTER_CARRIER_NAME" => ParametersGroupBysItems :: FilterCarrierName , "FILTER_CHANNEL_GROUPING" => ParametersGroupBysItems :: FilterChannelGrouping , "FILTER_CHANNEL_ID" => ParametersGroupBysItems :: FilterChannelId , "FILTER_CHANNEL_NAME" => ParametersGroupBysItems :: FilterChannelName , "FILTER_CHANNEL_TYPE" => ParametersGroupBysItems :: FilterChannelType , "FILTER_CITY" => ParametersGroupBysItems :: FilterCity , "FILTER_CITY_NAME" => ParametersGroupBysItems :: FilterCityName , "FILTER_CM360_PLACEMENT_ID" => ParametersGroupBysItems :: FilterCm360PlacementId , "FILTER_CM_PLACEMENT_ID" => ParametersGroupBysItems :: FilterCmPlacementId , "FILTER_COMPANION_CREATIVE_ID" => ParametersGroupBysItems :: FilterCompanionCreativeId , "FILTER_COMPANION_CREATIVE_NAME" => ParametersGroupBysItems :: FilterCompanionCreativeName , "FILTER_CONVERSION_DELAY" => ParametersGroupBysItems :: FilterConversionDelay , "FILTER_CONVERSION_SOURCE" => ParametersGroupBysItems :: FilterConversionSource , "FILTER_CONVERSION_SOURCE_ID" => ParametersGroupBysItems :: FilterConversionSourceId , "FILTER_COUNTRY" => ParametersGroupBysItems :: FilterCountry , "FILTER_COUNTRY_ID" => ParametersGroupBysItems :: FilterCountryId , "FILTER_CREATIVE" => ParametersGroupBysItems :: FilterCreative , "FILTER_CREATIVE_ASSET" => ParametersGroupBysItems :: FilterCreativeAsset , "FILTER_CREATIVE_ATTRIBUTE" => ParametersGroupBysItems :: FilterCreativeAttribute , "FILTER_CREATIVE_HEIGHT" => ParametersGroupBysItems :: FilterCreativeHeight , "FILTER_CREATIVE_ID" => ParametersGroupBysItems :: FilterCreativeId , "FILTER_CREATIVE_INTEGRATION_CODE" => ParametersGroupBysItems :: FilterCreativeIntegrationCode , "FILTER_CREATIVE_RENDERED_IN_AMP" => ParametersGroupBysItems :: FilterCreativeRenderedInAmp , "FILTER_CREATIVE_SIZE" => ParametersGroupBysItems :: FilterCreativeSize , "FILTER_CREATIVE_SOURCE" => ParametersGroupBysItems :: FilterCreativeSource , "FILTER_CREATIVE_STATUS" => ParametersGroupBysItems :: FilterCreativeStatus , "FILTER_CREATIVE_TYPE" => ParametersGroupBysItems :: FilterCreativeType , "FILTER_CREATIVE_WIDTH" => ParametersGroupBysItems :: FilterCreativeWidth , "FILTER_DATA_PROVIDER" => ParametersGroupBysItems :: FilterDataProvider , "FILTER_DATA_PROVIDER_NAME" => ParametersGroupBysItems :: FilterDataProviderName , "FILTER_DATA_SOURCE" => ParametersGroupBysItems :: FilterDataSource , "FILTER_DATE" => ParametersGroupBysItems :: FilterDate , "FILTER_DAY_OF_WEEK" => ParametersGroupBysItems :: FilterDayOfWeek , "FILTER_DETAILED_DEMOGRAPHICS" => ParametersGroupBysItems :: FilterDetailedDemographics , "FILTER_DETAILED_DEMOGRAPHICS_ID" => ParametersGroupBysItems :: FilterDetailedDemographicsId , "FILTER_DEVICE" => ParametersGroupBysItems :: FilterDevice , "FILTER_DEVICE_MAKE" => ParametersGroupBysItems :: FilterDeviceMake , "FILTER_DEVICE_MODEL" => ParametersGroupBysItems :: FilterDeviceModel , "FILTER_DEVICE_TYPE" => ParametersGroupBysItems :: FilterDeviceType , "FILTER_DFP_ORDER_ID" => ParametersGroupBysItems :: FilterDfpOrderId , "FILTER_DIGITAL_CONTENT_LABEL" => ParametersGroupBysItems :: FilterDigitalContentLabel , "FILTER_DMA" => ParametersGroupBysItems :: FilterDma , "FILTER_DMA_NAME" => ParametersGroupBysItems :: FilterDmaName , "FILTER_DOMAIN" => ParametersGroupBysItems :: FilterDomain , "FILTER_ELIGIBLE_COOKIES_ON_FIRST_PARTY_AUDIENCE_LIST" => ParametersGroupBysItems :: FilterEligibleCookiesOnFirstPartyAudienceList , "FILTER_ELIGIBLE_COOKIES_ON_THIRD_PARTY_AUDIENCE_LIST_AND_INTEREST" => ParametersGroupBysItems :: FilterEligibleCookiesOnThirdPartyAudienceListAndInterest , "FILTER_EVENT_TYPE" => ParametersGroupBysItems :: FilterEventType , "FILTER_EXCHANGE" => ParametersGroupBysItems :: FilterExchange , "FILTER_EXCHANGE_CODE" => ParametersGroupBysItems :: FilterExchangeCode , "FILTER_EXCHANGE_ID" => ParametersGroupBysItems :: FilterExchangeId , "FILTER_EXTENSION" => ParametersGroupBysItems :: FilterExtension , "FILTER_EXTENSION_STATUS" => ParametersGroupBysItems :: FilterExtensionStatus , "FILTER_EXTENSION_TYPE" => ParametersGroupBysItems :: FilterExtensionType , "FILTER_FIRST_PARTY_AUDIENCE_LIST_COST" => ParametersGroupBysItems :: FilterFirstPartyAudienceListCost , "FILTER_FIRST_PARTY_AUDIENCE_LIST_TYPE" => ParametersGroupBysItems :: FilterFirstPartyAudienceListType , "FILTER_FLOODLIGHT_ACTIVITY" => ParametersGroupBysItems :: FilterFloodlightActivity , "FILTER_FLOODLIGHT_ACTIVITY_ID" => ParametersGroupBysItems :: FilterFloodlightActivityId , "FILTER_FORMAT" => ParametersGroupBysItems :: FilterFormat , "FILTER_GAM_INSERTION_ORDER" => ParametersGroupBysItems :: FilterGamInsertionOrder , "FILTER_GAM_LINE_ITEM" => ParametersGroupBysItems :: FilterGamLineItem , "FILTER_GAM_LINE_ITEM_ID" => ParametersGroupBysItems :: FilterGamLineItemId , "FILTER_GENDER" => ParametersGroupBysItems :: FilterGender , "FILTER_GMAIL_AGE" => ParametersGroupBysItems :: FilterGmailAge , "FILTER_GMAIL_CITY" => ParametersGroupBysItems :: FilterGmailCity , "FILTER_GMAIL_COUNTRY" => ParametersGroupBysItems :: FilterGmailCountry , "FILTER_GMAIL_COUNTRY_NAME" => ParametersGroupBysItems :: FilterGmailCountryName , "FILTER_GMAIL_DEVICE_TYPE" => ParametersGroupBysItems :: FilterGmailDeviceType , "FILTER_GMAIL_DEVICE_TYPE_NAME" => ParametersGroupBysItems :: FilterGmailDeviceTypeName , "FILTER_GMAIL_GENDER" => ParametersGroupBysItems :: FilterGmailGender , "FILTER_GMAIL_REGION" => ParametersGroupBysItems :: FilterGmailRegion , "FILTER_GMAIL_REMARKETING_LIST" => ParametersGroupBysItems :: FilterGmailRemarketingList , "FILTER_HOUSEHOLD_INCOME" => ParametersGroupBysItems :: FilterHouseholdIncome , "FILTER_IMPRESSION_COUNTING_METHOD" => ParametersGroupBysItems :: FilterImpressionCountingMethod , "FILTER_IMPRESSION_LOSS_REJECTION_REASON" => ParametersGroupBysItems :: FilterImpressionLossRejectionReason , "FILTER_INSERTION_ORDER" => ParametersGroupBysItems :: FilterInsertionOrder , "FILTER_INSERTION_ORDER_GOAL_TYPE" => ParametersGroupBysItems :: FilterInsertionOrderGoalType , "FILTER_INSERTION_ORDER_GOAL_VALUE" => ParametersGroupBysItems :: FilterInsertionOrderGoalValue , "FILTER_INSERTION_ORDER_INTEGRATION_CODE" => ParametersGroupBysItems :: FilterInsertionOrderIntegrationCode , "FILTER_INSERTION_ORDER_NAME" => ParametersGroupBysItems :: FilterInsertionOrderName , "FILTER_INSERTION_ORDER_STATUS" => ParametersGroupBysItems :: FilterInsertionOrderStatus , "FILTER_INTEREST" => ParametersGroupBysItems :: FilterInterest , "FILTER_INVENTORY_COMMITMENT_TYPE" => ParametersGroupBysItems :: FilterInventoryCommitmentType , "FILTER_INVENTORY_DELIVERY_METHOD" => ParametersGroupBysItems :: FilterInventoryDeliveryMethod , "FILTER_INVENTORY_FORMAT" => ParametersGroupBysItems :: FilterInventoryFormat , "FILTER_INVENTORY_RATE_TYPE" => ParametersGroupBysItems :: FilterInventoryRateType , "FILTER_INVENTORY_SOURCE" => ParametersGroupBysItems :: FilterInventorySource , "FILTER_INVENTORY_SOURCE_EXTERNAL_ID" => ParametersGroupBysItems :: FilterInventorySourceExternalId , "FILTER_INVENTORY_SOURCE_GROUP" => ParametersGroupBysItems :: FilterInventorySourceGroup , "FILTER_INVENTORY_SOURCE_GROUP_ID" => ParametersGroupBysItems :: FilterInventorySourceGroupId , "FILTER_INVENTORY_SOURCE_ID" => ParametersGroupBysItems :: FilterInventorySourceId , "FILTER_INVENTORY_SOURCE_NAME" => ParametersGroupBysItems :: FilterInventorySourceName , "FILTER_INVENTORY_SOURCE_TYPE" => ParametersGroupBysItems :: FilterInventorySourceType , "FILTER_KEYWORD" => ParametersGroupBysItems :: FilterKeyword , "FILTER_LIFE_EVENT" => ParametersGroupBysItems :: FilterLifeEvent , "FILTER_LIFE_EVENTS" => ParametersGroupBysItems :: FilterLifeEvents , "FILTER_LINE_ITEM" => ParametersGroupBysItems :: FilterLineItem , "FILTER_LINE_ITEM_BUDGET" => ParametersGroupBysItems :: FilterLineItemBudget , "FILTER_LINE_ITEM_DAILY_FREQUENCY" => ParametersGroupBysItems :: FilterLineItemDailyFrequency , "FILTER_LINE_ITEM_END_DATE" => ParametersGroupBysItems :: FilterLineItemEndDate , "FILTER_LINE_ITEM_INTEGRATION_CODE" => ParametersGroupBysItems :: FilterLineItemIntegrationCode , "FILTER_LINE_ITEM_LIFETIME_FREQUENCY" => ParametersGroupBysItems :: FilterLineItemLifetimeFrequency , "FILTER_LINE_ITEM_NAME" => ParametersGroupBysItems :: FilterLineItemName , "FILTER_LINE_ITEM_PACING_PERCENTAGE" => ParametersGroupBysItems :: FilterLineItemPacingPercentage , "FILTER_LINE_ITEM_START_DATE" => ParametersGroupBysItems :: FilterLineItemStartDate , "FILTER_LINE_ITEM_STATUS" => ParametersGroupBysItems :: FilterLineItemStatus , "FILTER_LINE_ITEM_TYPE" => ParametersGroupBysItems :: FilterLineItemType , "FILTER_MATCH_RATIO" => ParametersGroupBysItems :: FilterMatchRatio , "FILTER_MATCHED_GENRE_TARGET" => ParametersGroupBysItems :: FilterMatchedGenreTarget , "FILTER_MEASUREMENT_SOURCE" => ParametersGroupBysItems :: FilterMeasurementSource , "FILTER_MEDIA_PLAN" => ParametersGroupBysItems :: FilterMediaPlan , "FILTER_MEDIA_PLAN_NAME" => ParametersGroupBysItems :: FilterMediaPlanName , "FILTER_MEDIA_TYPE" => ParametersGroupBysItems :: FilterMediaType , "FILTER_MOBILE_GEO" => ParametersGroupBysItems :: FilterMobileGeo , "FILTER_MONTH" => ParametersGroupBysItems :: FilterMonth , "FILTER_MRAID_SUPPORT" => ParametersGroupBysItems :: FilterMraidSupport , "FILTER_NIELSEN_AGE" => ParametersGroupBysItems :: FilterNielsenAge , "FILTER_NIELSEN_COUNTRY_CODE" => ParametersGroupBysItems :: FilterNielsenCountryCode , "FILTER_NIELSEN_DATE_RANGE" => ParametersGroupBysItems :: FilterNielsenDateRange , "FILTER_NIELSEN_DEVICE_ID" => ParametersGroupBysItems :: FilterNielsenDeviceId , "FILTER_NIELSEN_GENDER" => ParametersGroupBysItems :: FilterNielsenGender , "FILTER_NIELSEN_RESTATEMENT_DATE" => ParametersGroupBysItems :: FilterNielsenRestatementDate , "FILTER_NOT_SUPPORTED" => ParametersGroupBysItems :: FilterNotSupported , "FILTER_OM_SDK_AVAILABLE" => ParametersGroupBysItems :: FilterOmSdkAvailable , "FILTER_OMID_CAPABLE" => ParametersGroupBysItems :: FilterOmidCapable , "FILTER_ORDER_ID" => ParametersGroupBysItems :: FilterOrderId , "FILTER_OS" => ParametersGroupBysItems :: FilterOs , "FILTER_PAGE_CATEGORY" => ParametersGroupBysItems :: FilterPageCategory , "FILTER_PAGE_LAYOUT" => ParametersGroupBysItems :: FilterPageLayout , "FILTER_PARENTAL_STATUS" => ParametersGroupBysItems :: FilterParentalStatus , "FILTER_PARTNER" => ParametersGroupBysItems :: FilterPartner , "FILTER_PARTNER_CURRENCY" => ParametersGroupBysItems :: FilterPartnerCurrency , "FILTER_PARTNER_NAME" => ParametersGroupBysItems :: FilterPartnerName , "FILTER_PARTNER_STATUS" => ParametersGroupBysItems :: FilterPartnerStatus , "FILTER_PATH_EVENT_INDEX" => ParametersGroupBysItems :: FilterPathEventIndex , "FILTER_PATH_PATTERN_ID" => ParametersGroupBysItems :: FilterPathPatternId , "FILTER_PLACEMENT_ALL_YOUTUBE_CHANNELS" => ParametersGroupBysItems :: FilterPlacementAllYoutubeChannels , "FILTER_PLACEMENT_NAME_ALL_YOUTUBE_CHANNELS" => ParametersGroupBysItems :: FilterPlacementNameAllYoutubeChannels , "FILTER_PLATFORM" => ParametersGroupBysItems :: FilterPlatform , "FILTER_PLAYBACK_METHOD" => ParametersGroupBysItems :: FilterPlaybackMethod , "FILTER_POSITION_IN_CONTENT" => ParametersGroupBysItems :: FilterPositionInContent , "FILTER_PUBLIC_INVENTORY" => ParametersGroupBysItems :: FilterPublicInventory , "FILTER_PUBLISHER_PROPERTY" => ParametersGroupBysItems :: FilterPublisherProperty , "FILTER_PUBLISHER_PROPERTY_ID" => ParametersGroupBysItems :: FilterPublisherPropertyId , "FILTER_PUBLISHER_PROPERTY_SECTION" => ParametersGroupBysItems :: FilterPublisherPropertySection , "FILTER_PUBLISHER_PROPERTY_SECTION_ID" => ParametersGroupBysItems :: FilterPublisherPropertySectionId , "FILTER_QUARTER" => ParametersGroupBysItems :: FilterQuarter , "FILTER_REFUND_REASON" => ParametersGroupBysItems :: FilterRefundReason , "FILTER_REGION" => ParametersGroupBysItems :: FilterRegion , "FILTER_REGION_NAME" => ParametersGroupBysItems :: FilterRegionName , "FILTER_REMARKETING_LIST" => ParametersGroupBysItems :: FilterRemarketingList , "FILTER_REWARDED" => ParametersGroupBysItems :: FilterRewarded , "FILTER_SENSITIVE_CATEGORY" => ParametersGroupBysItems :: FilterSensitiveCategory , "FILTER_SERVED_PIXEL_DENSITY" => ParametersGroupBysItems :: FilterServedPixelDensity , "FILTER_SITE_ID" => ParametersGroupBysItems :: FilterSiteId , "FILTER_SITE_LANGUAGE" => ParametersGroupBysItems :: FilterSiteLanguage , "FILTER_SKIPPABLE_SUPPORT" => ParametersGroupBysItems :: FilterSkippableSupport , "FILTER_TARGETED_DATA_PROVIDERS" => ParametersGroupBysItems :: FilterTargetedDataProviders , "FILTER_TARGETED_USER_LIST" => ParametersGroupBysItems :: FilterTargetedUserList , "FILTER_THIRD_PARTY_AUDIENCE_LIST_COST" => ParametersGroupBysItems :: FilterThirdPartyAudienceListCost , "FILTER_THIRD_PARTY_AUDIENCE_LIST_TYPE" => ParametersGroupBysItems :: FilterThirdPartyAudienceListType , "FILTER_TIME_OF_DAY" => ParametersGroupBysItems :: FilterTimeOfDay , "FILTER_TRUEVIEW_AD" => ParametersGroupBysItems :: FilterTrueviewAd , "FILTER_TRUEVIEW_AD_GROUP" => ParametersGroupBysItems :: FilterTrueviewAdGroup , "FILTER_TRUEVIEW_AD_GROUP_AD_ID" => ParametersGroupBysItems :: FilterTrueviewAdGroupAdId , "FILTER_TRUEVIEW_AD_GROUP_ID" => ParametersGroupBysItems :: FilterTrueviewAdGroupId , "FILTER_TRUEVIEW_AD_TYPE_NAME" => ParametersGroupBysItems :: FilterTrueviewAdTypeName , "FILTER_TRUEVIEW_AGE" => ParametersGroupBysItems :: FilterTrueviewAge , "FILTER_TRUEVIEW_CATEGORY" => ParametersGroupBysItems :: FilterTrueviewCategory , "FILTER_TRUEVIEW_CITY" => ParametersGroupBysItems :: FilterTrueviewCity , "FILTER_TRUEVIEW_CLICK_TYPE_NAME" => ParametersGroupBysItems :: FilterTrueviewClickTypeName , "FILTER_TRUEVIEW_CONVERSION_TYPE" => ParametersGroupBysItems :: FilterTrueviewConversionType , "FILTER_TRUEVIEW_COUNTRY" => ParametersGroupBysItems :: FilterTrueviewCountry , "FILTER_TRUEVIEW_CUSTOM_AFFINITY" => ParametersGroupBysItems :: FilterTrueviewCustomAffinity , "FILTER_TRUEVIEW_DETAILED_DEMOGRAPHICS" => ParametersGroupBysItems :: FilterTrueviewDetailedDemographics , "FILTER_TRUEVIEW_DETAILED_DEMOGRAPHICS_ID" => ParametersGroupBysItems :: FilterTrueviewDetailedDemographicsId , "FILTER_TRUEVIEW_DMA" => ParametersGroupBysItems :: FilterTrueviewDma , "FILTER_TRUEVIEW_DMA_NAME" => ParametersGroupBysItems :: FilterTrueviewDmaName , "FILTER_TRUEVIEW_GENDER" => ParametersGroupBysItems :: FilterTrueviewGender , "FILTER_TRUEVIEW_HOUSEHOLD_INCOME" => ParametersGroupBysItems :: FilterTrueviewHouseholdIncome , "FILTER_TRUEVIEW_IAR_AGE" => ParametersGroupBysItems :: FilterTrueviewIarAge , "FILTER_TRUEVIEW_IAR_CATEGORY" => ParametersGroupBysItems :: FilterTrueviewIarCategory , "FILTER_TRUEVIEW_IAR_CITY" => ParametersGroupBysItems :: FilterTrueviewIarCity , "FILTER_TRUEVIEW_IAR_COUNTRY" => ParametersGroupBysItems :: FilterTrueviewIarCountry , "FILTER_TRUEVIEW_IAR_COUNTRY_NAME" => ParametersGroupBysItems :: FilterTrueviewIarCountryName , "FILTER_TRUEVIEW_IAR_GENDER" => ParametersGroupBysItems :: FilterTrueviewIarGender , "FILTER_TRUEVIEW_IAR_INTEREST" => ParametersGroupBysItems :: FilterTrueviewIarInterest , "FILTER_TRUEVIEW_IAR_LANGUAGE" => ParametersGroupBysItems :: FilterTrueviewIarLanguage , "FILTER_TRUEVIEW_IAR_PARENTAL_STATUS" => ParametersGroupBysItems :: FilterTrueviewIarParentalStatus , "FILTER_TRUEVIEW_IAR_REGION" => ParametersGroupBysItems :: FilterTrueviewIarRegion , "FILTER_TRUEVIEW_IAR_REGION_NAME" => ParametersGroupBysItems :: FilterTrueviewIarRegionName , "FILTER_TRUEVIEW_IAR_REMARKETING_LIST" => ParametersGroupBysItems :: FilterTrueviewIarRemarketingList , "FILTER_TRUEVIEW_IAR_TIME_OF_DAY" => ParametersGroupBysItems :: FilterTrueviewIarTimeOfDay , "FILTER_TRUEVIEW_IAR_YOUTUBE_CHANNEL" => ParametersGroupBysItems :: FilterTrueviewIarYoutubeChannel , "FILTER_TRUEVIEW_IAR_YOUTUBE_VIDEO" => ParametersGroupBysItems :: FilterTrueviewIarYoutubeVideo , "FILTER_TRUEVIEW_IAR_ZIPCODE" => ParametersGroupBysItems :: FilterTrueviewIarZipcode , "FILTER_TRUEVIEW_INTEREST" => ParametersGroupBysItems :: FilterTrueviewInterest , "FILTER_TRUEVIEW_KEYWORD" => ParametersGroupBysItems :: FilterTrueviewKeyword , "FILTER_TRUEVIEW_PARENTAL_STATUS" => ParametersGroupBysItems :: FilterTrueviewParentalStatus , "FILTER_TRUEVIEW_PLACEMENT" => ParametersGroupBysItems :: FilterTrueviewPlacement , "FILTER_TRUEVIEW_PLACEMENT_ID" => ParametersGroupBysItems :: FilterTrueviewPlacementId , "FILTER_TRUEVIEW_REGION" => ParametersGroupBysItems :: FilterTrueviewRegion , "FILTER_TRUEVIEW_REGION_NAME" => ParametersGroupBysItems :: FilterTrueviewRegionName , "FILTER_TRUEVIEW_REMARKETING_LIST" => ParametersGroupBysItems :: FilterTrueviewRemarketingList , "FILTER_TRUEVIEW_REMARKETING_LIST_NAME" => ParametersGroupBysItems :: FilterTrueviewRemarketingListName , "FILTER_TRUEVIEW_URL" => ParametersGroupBysItems :: FilterTrueviewUrl , "FILTER_TRUEVIEW_ZIPCODE" => ParametersGroupBysItems :: FilterTrueviewZipcode , "FILTER_UNKNOWN" => ParametersGroupBysItems :: FilterUnknown , "FILTER_USER_LIST" => ParametersGroupBysItems :: FilterUserList , "FILTER_USER_LIST_FIRST_PARTY" => ParametersGroupBysItems :: FilterUserListFirstParty , "FILTER_USER_LIST_FIRST_PARTY_NAME" => ParametersGroupBysItems :: FilterUserListFirstPartyName , "FILTER_USER_LIST_THIRD_PARTY" => ParametersGroupBysItems :: FilterUserListThirdParty , "FILTER_USER_LIST_THIRD_PARTY_NAME" => ParametersGroupBysItems :: FilterUserListThirdPartyName , "FILTER_VARIANT_ID" => ParametersGroupBysItems :: FilterVariantId , "FILTER_VARIANT_NAME" => ParametersGroupBysItems :: FilterVariantName , "FILTER_VARIANT_VERSION" => ParametersGroupBysItems :: FilterVariantVersion , "FILTER_VENDOR_MEASUREMENT_MODE" => ParametersGroupBysItems :: FilterVendorMeasurementMode , "FILTER_VERIFICATION_AUDIBILITY_COMPLETE" => ParametersGroupBysItems :: FilterVerificationAudibilityComplete , "FILTER_VERIFICATION_AUDIBILITY_START" => ParametersGroupBysItems :: FilterVerificationAudibilityStart , "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE" => ParametersGroupBysItems :: FilterVerificationVideoPlayerSize , "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_COMPLETE" => ParametersGroupBysItems :: FilterVerificationVideoPlayerSizeComplete , "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_FIRST_QUARTILE" => ParametersGroupBysItems :: FilterVerificationVideoPlayerSizeFirstQuartile , "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_MID_POINT" => ParametersGroupBysItems :: FilterVerificationVideoPlayerSizeMidPoint , "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_START" => ParametersGroupBysItems :: FilterVerificationVideoPlayerSizeStart , "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_THIRD_QUARTILE" => ParametersGroupBysItems :: FilterVerificationVideoPlayerSizeThirdQuartile , "FILTER_VERIFICATION_VIDEO_POSITION" => ParametersGroupBysItems :: FilterVerificationVideoPosition , "FILTER_VERIFICATION_VIDEO_RESIZED" => ParametersGroupBysItems :: FilterVerificationVideoResized , "FILTER_VIDEO_AD_POSITION_IN_STREAM" => ParametersGroupBysItems :: FilterVideoAdPositionInStream , "FILTER_VIDEO_COMPANION_CREATIVE_SIZE" => ParametersGroupBysItems :: FilterVideoCompanionCreativeSize , "FILTER_VIDEO_CONTENT_DURATION" => ParametersGroupBysItems :: FilterVideoContentDuration , "FILTER_VIDEO_CONTENT_LIVE_STREAM" => ParametersGroupBysItems :: FilterVideoContentLiveStream , "FILTER_VIDEO_CONTINUOUS_PLAY" => ParametersGroupBysItems :: FilterVideoContinuousPlay , "FILTER_VIDEO_CREATIVE_DURATION" => ParametersGroupBysItems :: FilterVideoCreativeDuration , "FILTER_VIDEO_CREATIVE_DURATION_SKIPPABLE" => ParametersGroupBysItems :: FilterVideoCreativeDurationSkippable , "FILTER_VIDEO_DURATION" => ParametersGroupBysItems :: FilterVideoDuration , "FILTER_VIDEO_DURATION_SECONDS" => ParametersGroupBysItems :: FilterVideoDurationSeconds , "FILTER_VIDEO_DURATION_SECONDS_RANGE" => ParametersGroupBysItems :: FilterVideoDurationSecondsRange , "FILTER_VIDEO_FORMAT_SUPPORT" => ParametersGroupBysItems :: FilterVideoFormatSupport , "FILTER_VIDEO_PLAYER_SIZE" => ParametersGroupBysItems :: FilterVideoPlayerSize , "FILTER_VIDEO_RATING_TIER" => ParametersGroupBysItems :: FilterVideoRatingTier , "FILTER_VIDEO_SKIPPABLE_SUPPORT" => ParametersGroupBysItems :: FilterVideoSkippableSupport , "FILTER_WEEK" => ParametersGroupBysItems :: FilterWeek , "FILTER_YEAR" => ParametersGroupBysItems :: FilterYear , "FILTER_YOUTUBE_AD_VIDEO" => ParametersGroupBysItems :: FilterYoutubeAdVideo , "FILTER_YOUTUBE_AD_VIDEO_ID" => ParametersGroupBysItems :: FilterYoutubeAdVideoId , "FILTER_YOUTUBE_ADAPTED_AUDIENCE_LIST" => ParametersGroupBysItems :: FilterYoutubeAdaptedAudienceList , "FILTER_YOUTUBE_CHANNEL" => ParametersGroupBysItems :: FilterYoutubeChannel , "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_ADVERTISER" => ParametersGroupBysItems :: FilterYoutubeProgrammaticGuaranteedAdvertiser , "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_INSERTION_ORDER" => ParametersGroupBysItems :: FilterYoutubeProgrammaticGuaranteedInsertionOrder , "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_PARTNER" => ParametersGroupBysItems :: FilterYoutubeProgrammaticGuaranteedPartner , "FILTER_YOUTUBE_VIDEO" => ParametersGroupBysItems :: FilterYoutubeVideo , "FILTER_ZIP_CODE" => ParametersGroupBysItems :: FilterZipCode , "FILTER_ZIP_POSTAL_CODE" => ParametersGroupBysItems :: FilterZipPostalCode , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for ParametersGroupBysItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ParametersGroupBysItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ParametersGroupBysItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "FILTER_ACTIVE_VIEW_CUSTOM_METRIC_ID" => ParametersGroupBysItems :: FilterActiveViewCustomMetricId , "FILTER_ACTIVE_VIEW_CUSTOM_METRIC_NAME" => ParametersGroupBysItems :: FilterActiveViewCustomMetricName , "FILTER_ACTIVE_VIEW_EXPECTED_VIEWABILITY" => ParametersGroupBysItems :: FilterActiveViewExpectedViewability , "FILTER_AD_POSITION" => ParametersGroupBysItems :: FilterAdPosition , "FILTER_AD_TYPE" => ParametersGroupBysItems :: FilterAdType , "FILTER_ADVERTISER" => ParametersGroupBysItems :: FilterAdvertiser , "FILTER_ADVERTISER_CURRENCY" => ParametersGroupBysItems :: FilterAdvertiserCurrency , "FILTER_ADVERTISER_INTEGRATION_CODE" => ParametersGroupBysItems :: FilterAdvertiserIntegrationCode , "FILTER_ADVERTISER_INTEGRATION_STATUS" => ParametersGroupBysItems :: FilterAdvertiserIntegrationStatus , "FILTER_ADVERTISER_NAME" => ParametersGroupBysItems :: FilterAdvertiserName , "FILTER_ADVERTISER_TIMEZONE" => ParametersGroupBysItems :: FilterAdvertiserTimezone , "FILTER_AGE" => ParametersGroupBysItems :: FilterAge , "FILTER_ALGORITHM" => ParametersGroupBysItems :: FilterAlgorithm , "FILTER_ALGORITHM_ID" => ParametersGroupBysItems :: FilterAlgorithmId , "FILTER_AMP_PAGE_REQUEST" => ParametersGroupBysItems :: FilterAmpPageRequest , "FILTER_ANONYMOUS_INVENTORY_MODELING" => ParametersGroupBysItems :: FilterAnonymousInventoryModeling , "FILTER_APP_URL" => ParametersGroupBysItems :: FilterAppUrl , "FILTER_APP_URL_EXCLUDED" => ParametersGroupBysItems :: FilterAppUrlExcluded , "FILTER_ATTRIBUTED_USERLIST" => ParametersGroupBysItems :: FilterAttributedUserlist , "FILTER_ATTRIBUTED_USERLIST_COST" => ParametersGroupBysItems :: FilterAttributedUserlistCost , "FILTER_ATTRIBUTED_USERLIST_TYPE" => ParametersGroupBysItems :: FilterAttributedUserlistType , "FILTER_ATTRIBUTION_MODEL" => ParametersGroupBysItems :: FilterAttributionModel , "FILTER_AUDIENCE_LIST" => ParametersGroupBysItems :: FilterAudienceList , "FILTER_AUDIENCE_LIST_COST" => ParametersGroupBysItems :: FilterAudienceListCost , "FILTER_AUDIENCE_LIST_TYPE" => ParametersGroupBysItems :: FilterAudienceListType , "FILTER_AUDIENCE_NAME" => ParametersGroupBysItems :: FilterAudienceName , "FILTER_AUDIENCE_TYPE" => ParametersGroupBysItems :: FilterAudienceType , "FILTER_AUDIO_FEED_TYPE_NAME" => ParametersGroupBysItems :: FilterAudioFeedTypeName , "FILTER_AUTHORIZED_SELLER_STATE" => ParametersGroupBysItems :: FilterAuthorizedSellerState , "FILTER_BILLABLE_OUTCOME" => ParametersGroupBysItems :: FilterBillableOutcome , "FILTER_BRAND_LIFT_TYPE" => ParametersGroupBysItems :: FilterBrandLiftType , "FILTER_BROWSER" => ParametersGroupBysItems :: FilterBrowser , "FILTER_BUDGET_SEGMENT_BUDGET" => ParametersGroupBysItems :: FilterBudgetSegmentBudget , "FILTER_BUDGET_SEGMENT_DESCRIPTION" => ParametersGroupBysItems :: FilterBudgetSegmentDescription , "FILTER_BUDGET_SEGMENT_END_DATE" => ParametersGroupBysItems :: FilterBudgetSegmentEndDate , "FILTER_BUDGET_SEGMENT_PACING_PERCENTAGE" => ParametersGroupBysItems :: FilterBudgetSegmentPacingPercentage , "FILTER_BUDGET_SEGMENT_START_DATE" => ParametersGroupBysItems :: FilterBudgetSegmentStartDate , "FILTER_BUDGET_SEGMENT_TYPE" => ParametersGroupBysItems :: FilterBudgetSegmentType , "FILTER_CAMPAIGN_DAILY_FREQUENCY" => ParametersGroupBysItems :: FilterCampaignDailyFrequency , "FILTER_CARRIER" => ParametersGroupBysItems :: FilterCarrier , "FILTER_CARRIER_NAME" => ParametersGroupBysItems :: FilterCarrierName , "FILTER_CHANNEL_GROUPING" => ParametersGroupBysItems :: FilterChannelGrouping , "FILTER_CHANNEL_ID" => ParametersGroupBysItems :: FilterChannelId , "FILTER_CHANNEL_NAME" => ParametersGroupBysItems :: FilterChannelName , "FILTER_CHANNEL_TYPE" => ParametersGroupBysItems :: FilterChannelType , "FILTER_CITY" => ParametersGroupBysItems :: FilterCity , "FILTER_CITY_NAME" => ParametersGroupBysItems :: FilterCityName , "FILTER_CM360_PLACEMENT_ID" => ParametersGroupBysItems :: FilterCm360PlacementId , "FILTER_CM_PLACEMENT_ID" => ParametersGroupBysItems :: FilterCmPlacementId , "FILTER_COMPANION_CREATIVE_ID" => ParametersGroupBysItems :: FilterCompanionCreativeId , "FILTER_COMPANION_CREATIVE_NAME" => ParametersGroupBysItems :: FilterCompanionCreativeName , "FILTER_CONVERSION_DELAY" => ParametersGroupBysItems :: FilterConversionDelay , "FILTER_CONVERSION_SOURCE" => ParametersGroupBysItems :: FilterConversionSource , "FILTER_CONVERSION_SOURCE_ID" => ParametersGroupBysItems :: FilterConversionSourceId , "FILTER_COUNTRY" => ParametersGroupBysItems :: FilterCountry , "FILTER_COUNTRY_ID" => ParametersGroupBysItems :: FilterCountryId , "FILTER_CREATIVE" => ParametersGroupBysItems :: FilterCreative , "FILTER_CREATIVE_ASSET" => ParametersGroupBysItems :: FilterCreativeAsset , "FILTER_CREATIVE_ATTRIBUTE" => ParametersGroupBysItems :: FilterCreativeAttribute , "FILTER_CREATIVE_HEIGHT" => ParametersGroupBysItems :: FilterCreativeHeight , "FILTER_CREATIVE_ID" => ParametersGroupBysItems :: FilterCreativeId , "FILTER_CREATIVE_INTEGRATION_CODE" => ParametersGroupBysItems :: FilterCreativeIntegrationCode , "FILTER_CREATIVE_RENDERED_IN_AMP" => ParametersGroupBysItems :: FilterCreativeRenderedInAmp , "FILTER_CREATIVE_SIZE" => ParametersGroupBysItems :: FilterCreativeSize , "FILTER_CREATIVE_SOURCE" => ParametersGroupBysItems :: FilterCreativeSource , "FILTER_CREATIVE_STATUS" => ParametersGroupBysItems :: FilterCreativeStatus , "FILTER_CREATIVE_TYPE" => ParametersGroupBysItems :: FilterCreativeType , "FILTER_CREATIVE_WIDTH" => ParametersGroupBysItems :: FilterCreativeWidth , "FILTER_DATA_PROVIDER" => ParametersGroupBysItems :: FilterDataProvider , "FILTER_DATA_PROVIDER_NAME" => ParametersGroupBysItems :: FilterDataProviderName , "FILTER_DATA_SOURCE" => ParametersGroupBysItems :: FilterDataSource , "FILTER_DATE" => ParametersGroupBysItems :: FilterDate , "FILTER_DAY_OF_WEEK" => ParametersGroupBysItems :: FilterDayOfWeek , "FILTER_DETAILED_DEMOGRAPHICS" => ParametersGroupBysItems :: FilterDetailedDemographics , "FILTER_DETAILED_DEMOGRAPHICS_ID" => ParametersGroupBysItems :: FilterDetailedDemographicsId , "FILTER_DEVICE" => ParametersGroupBysItems :: FilterDevice , "FILTER_DEVICE_MAKE" => ParametersGroupBysItems :: FilterDeviceMake , "FILTER_DEVICE_MODEL" => ParametersGroupBysItems :: FilterDeviceModel , "FILTER_DEVICE_TYPE" => ParametersGroupBysItems :: FilterDeviceType , "FILTER_DFP_ORDER_ID" => ParametersGroupBysItems :: FilterDfpOrderId , "FILTER_DIGITAL_CONTENT_LABEL" => ParametersGroupBysItems :: FilterDigitalContentLabel , "FILTER_DMA" => ParametersGroupBysItems :: FilterDma , "FILTER_DMA_NAME" => ParametersGroupBysItems :: FilterDmaName , "FILTER_DOMAIN" => ParametersGroupBysItems :: FilterDomain , "FILTER_ELIGIBLE_COOKIES_ON_FIRST_PARTY_AUDIENCE_LIST" => ParametersGroupBysItems :: FilterEligibleCookiesOnFirstPartyAudienceList , "FILTER_ELIGIBLE_COOKIES_ON_THIRD_PARTY_AUDIENCE_LIST_AND_INTEREST" => ParametersGroupBysItems :: FilterEligibleCookiesOnThirdPartyAudienceListAndInterest , "FILTER_EVENT_TYPE" => ParametersGroupBysItems :: FilterEventType , "FILTER_EXCHANGE" => ParametersGroupBysItems :: FilterExchange , "FILTER_EXCHANGE_CODE" => ParametersGroupBysItems :: FilterExchangeCode , "FILTER_EXCHANGE_ID" => ParametersGroupBysItems :: FilterExchangeId , "FILTER_EXTENSION" => ParametersGroupBysItems :: FilterExtension , "FILTER_EXTENSION_STATUS" => ParametersGroupBysItems :: FilterExtensionStatus , "FILTER_EXTENSION_TYPE" => ParametersGroupBysItems :: FilterExtensionType , "FILTER_FIRST_PARTY_AUDIENCE_LIST_COST" => ParametersGroupBysItems :: FilterFirstPartyAudienceListCost , "FILTER_FIRST_PARTY_AUDIENCE_LIST_TYPE" => ParametersGroupBysItems :: FilterFirstPartyAudienceListType , "FILTER_FLOODLIGHT_ACTIVITY" => ParametersGroupBysItems :: FilterFloodlightActivity , "FILTER_FLOODLIGHT_ACTIVITY_ID" => ParametersGroupBysItems :: FilterFloodlightActivityId , "FILTER_FORMAT" => ParametersGroupBysItems :: FilterFormat , "FILTER_GAM_INSERTION_ORDER" => ParametersGroupBysItems :: FilterGamInsertionOrder , "FILTER_GAM_LINE_ITEM" => ParametersGroupBysItems :: FilterGamLineItem , "FILTER_GAM_LINE_ITEM_ID" => ParametersGroupBysItems :: FilterGamLineItemId , "FILTER_GENDER" => ParametersGroupBysItems :: FilterGender , "FILTER_GMAIL_AGE" => ParametersGroupBysItems :: FilterGmailAge , "FILTER_GMAIL_CITY" => ParametersGroupBysItems :: FilterGmailCity , "FILTER_GMAIL_COUNTRY" => ParametersGroupBysItems :: FilterGmailCountry , "FILTER_GMAIL_COUNTRY_NAME" => ParametersGroupBysItems :: FilterGmailCountryName , "FILTER_GMAIL_DEVICE_TYPE" => ParametersGroupBysItems :: FilterGmailDeviceType , "FILTER_GMAIL_DEVICE_TYPE_NAME" => ParametersGroupBysItems :: FilterGmailDeviceTypeName , "FILTER_GMAIL_GENDER" => ParametersGroupBysItems :: FilterGmailGender , "FILTER_GMAIL_REGION" => ParametersGroupBysItems :: FilterGmailRegion , "FILTER_GMAIL_REMARKETING_LIST" => ParametersGroupBysItems :: FilterGmailRemarketingList , "FILTER_HOUSEHOLD_INCOME" => ParametersGroupBysItems :: FilterHouseholdIncome , "FILTER_IMPRESSION_COUNTING_METHOD" => ParametersGroupBysItems :: FilterImpressionCountingMethod , "FILTER_IMPRESSION_LOSS_REJECTION_REASON" => ParametersGroupBysItems :: FilterImpressionLossRejectionReason , "FILTER_INSERTION_ORDER" => ParametersGroupBysItems :: FilterInsertionOrder , "FILTER_INSERTION_ORDER_GOAL_TYPE" => ParametersGroupBysItems :: FilterInsertionOrderGoalType , "FILTER_INSERTION_ORDER_GOAL_VALUE" => ParametersGroupBysItems :: FilterInsertionOrderGoalValue , "FILTER_INSERTION_ORDER_INTEGRATION_CODE" => ParametersGroupBysItems :: FilterInsertionOrderIntegrationCode , "FILTER_INSERTION_ORDER_NAME" => ParametersGroupBysItems :: FilterInsertionOrderName , "FILTER_INSERTION_ORDER_STATUS" => ParametersGroupBysItems :: FilterInsertionOrderStatus , "FILTER_INTEREST" => ParametersGroupBysItems :: FilterInterest , "FILTER_INVENTORY_COMMITMENT_TYPE" => ParametersGroupBysItems :: FilterInventoryCommitmentType , "FILTER_INVENTORY_DELIVERY_METHOD" => ParametersGroupBysItems :: FilterInventoryDeliveryMethod , "FILTER_INVENTORY_FORMAT" => ParametersGroupBysItems :: FilterInventoryFormat , "FILTER_INVENTORY_RATE_TYPE" => ParametersGroupBysItems :: FilterInventoryRateType , "FILTER_INVENTORY_SOURCE" => ParametersGroupBysItems :: FilterInventorySource , "FILTER_INVENTORY_SOURCE_EXTERNAL_ID" => ParametersGroupBysItems :: FilterInventorySourceExternalId , "FILTER_INVENTORY_SOURCE_GROUP" => ParametersGroupBysItems :: FilterInventorySourceGroup , "FILTER_INVENTORY_SOURCE_GROUP_ID" => ParametersGroupBysItems :: FilterInventorySourceGroupId , "FILTER_INVENTORY_SOURCE_ID" => ParametersGroupBysItems :: FilterInventorySourceId , "FILTER_INVENTORY_SOURCE_NAME" => ParametersGroupBysItems :: FilterInventorySourceName , "FILTER_INVENTORY_SOURCE_TYPE" => ParametersGroupBysItems :: FilterInventorySourceType , "FILTER_KEYWORD" => ParametersGroupBysItems :: FilterKeyword , "FILTER_LIFE_EVENT" => ParametersGroupBysItems :: FilterLifeEvent , "FILTER_LIFE_EVENTS" => ParametersGroupBysItems :: FilterLifeEvents , "FILTER_LINE_ITEM" => ParametersGroupBysItems :: FilterLineItem , "FILTER_LINE_ITEM_BUDGET" => ParametersGroupBysItems :: FilterLineItemBudget , "FILTER_LINE_ITEM_DAILY_FREQUENCY" => ParametersGroupBysItems :: FilterLineItemDailyFrequency , "FILTER_LINE_ITEM_END_DATE" => ParametersGroupBysItems :: FilterLineItemEndDate , "FILTER_LINE_ITEM_INTEGRATION_CODE" => ParametersGroupBysItems :: FilterLineItemIntegrationCode , "FILTER_LINE_ITEM_LIFETIME_FREQUENCY" => ParametersGroupBysItems :: FilterLineItemLifetimeFrequency , "FILTER_LINE_ITEM_NAME" => ParametersGroupBysItems :: FilterLineItemName , "FILTER_LINE_ITEM_PACING_PERCENTAGE" => ParametersGroupBysItems :: FilterLineItemPacingPercentage , "FILTER_LINE_ITEM_START_DATE" => ParametersGroupBysItems :: FilterLineItemStartDate , "FILTER_LINE_ITEM_STATUS" => ParametersGroupBysItems :: FilterLineItemStatus , "FILTER_LINE_ITEM_TYPE" => ParametersGroupBysItems :: FilterLineItemType , "FILTER_MATCH_RATIO" => ParametersGroupBysItems :: FilterMatchRatio , "FILTER_MATCHED_GENRE_TARGET" => ParametersGroupBysItems :: FilterMatchedGenreTarget , "FILTER_MEASUREMENT_SOURCE" => ParametersGroupBysItems :: FilterMeasurementSource , "FILTER_MEDIA_PLAN" => ParametersGroupBysItems :: FilterMediaPlan , "FILTER_MEDIA_PLAN_NAME" => ParametersGroupBysItems :: FilterMediaPlanName , "FILTER_MEDIA_TYPE" => ParametersGroupBysItems :: FilterMediaType , "FILTER_MOBILE_GEO" => ParametersGroupBysItems :: FilterMobileGeo , "FILTER_MONTH" => ParametersGroupBysItems :: FilterMonth , "FILTER_MRAID_SUPPORT" => ParametersGroupBysItems :: FilterMraidSupport , "FILTER_NIELSEN_AGE" => ParametersGroupBysItems :: FilterNielsenAge , "FILTER_NIELSEN_COUNTRY_CODE" => ParametersGroupBysItems :: FilterNielsenCountryCode , "FILTER_NIELSEN_DATE_RANGE" => ParametersGroupBysItems :: FilterNielsenDateRange , "FILTER_NIELSEN_DEVICE_ID" => ParametersGroupBysItems :: FilterNielsenDeviceId , "FILTER_NIELSEN_GENDER" => ParametersGroupBysItems :: FilterNielsenGender , "FILTER_NIELSEN_RESTATEMENT_DATE" => ParametersGroupBysItems :: FilterNielsenRestatementDate , "FILTER_NOT_SUPPORTED" => ParametersGroupBysItems :: FilterNotSupported , "FILTER_OM_SDK_AVAILABLE" => ParametersGroupBysItems :: FilterOmSdkAvailable , "FILTER_OMID_CAPABLE" => ParametersGroupBysItems :: FilterOmidCapable , "FILTER_ORDER_ID" => ParametersGroupBysItems :: FilterOrderId , "FILTER_OS" => ParametersGroupBysItems :: FilterOs , "FILTER_PAGE_CATEGORY" => ParametersGroupBysItems :: FilterPageCategory , "FILTER_PAGE_LAYOUT" => ParametersGroupBysItems :: FilterPageLayout , "FILTER_PARENTAL_STATUS" => ParametersGroupBysItems :: FilterParentalStatus , "FILTER_PARTNER" => ParametersGroupBysItems :: FilterPartner , "FILTER_PARTNER_CURRENCY" => ParametersGroupBysItems :: FilterPartnerCurrency , "FILTER_PARTNER_NAME" => ParametersGroupBysItems :: FilterPartnerName , "FILTER_PARTNER_STATUS" => ParametersGroupBysItems :: FilterPartnerStatus , "FILTER_PATH_EVENT_INDEX" => ParametersGroupBysItems :: FilterPathEventIndex , "FILTER_PATH_PATTERN_ID" => ParametersGroupBysItems :: FilterPathPatternId , "FILTER_PLACEMENT_ALL_YOUTUBE_CHANNELS" => ParametersGroupBysItems :: FilterPlacementAllYoutubeChannels , "FILTER_PLACEMENT_NAME_ALL_YOUTUBE_CHANNELS" => ParametersGroupBysItems :: FilterPlacementNameAllYoutubeChannels , "FILTER_PLATFORM" => ParametersGroupBysItems :: FilterPlatform , "FILTER_PLAYBACK_METHOD" => ParametersGroupBysItems :: FilterPlaybackMethod , "FILTER_POSITION_IN_CONTENT" => ParametersGroupBysItems :: FilterPositionInContent , "FILTER_PUBLIC_INVENTORY" => ParametersGroupBysItems :: FilterPublicInventory , "FILTER_PUBLISHER_PROPERTY" => ParametersGroupBysItems :: FilterPublisherProperty , "FILTER_PUBLISHER_PROPERTY_ID" => ParametersGroupBysItems :: FilterPublisherPropertyId , "FILTER_PUBLISHER_PROPERTY_SECTION" => ParametersGroupBysItems :: FilterPublisherPropertySection , "FILTER_PUBLISHER_PROPERTY_SECTION_ID" => ParametersGroupBysItems :: FilterPublisherPropertySectionId , "FILTER_QUARTER" => ParametersGroupBysItems :: FilterQuarter , "FILTER_REFUND_REASON" => ParametersGroupBysItems :: FilterRefundReason , "FILTER_REGION" => ParametersGroupBysItems :: FilterRegion , "FILTER_REGION_NAME" => ParametersGroupBysItems :: FilterRegionName , "FILTER_REMARKETING_LIST" => ParametersGroupBysItems :: FilterRemarketingList , "FILTER_REWARDED" => ParametersGroupBysItems :: FilterRewarded , "FILTER_SENSITIVE_CATEGORY" => ParametersGroupBysItems :: FilterSensitiveCategory , "FILTER_SERVED_PIXEL_DENSITY" => ParametersGroupBysItems :: FilterServedPixelDensity , "FILTER_SITE_ID" => ParametersGroupBysItems :: FilterSiteId , "FILTER_SITE_LANGUAGE" => ParametersGroupBysItems :: FilterSiteLanguage , "FILTER_SKIPPABLE_SUPPORT" => ParametersGroupBysItems :: FilterSkippableSupport , "FILTER_TARGETED_DATA_PROVIDERS" => ParametersGroupBysItems :: FilterTargetedDataProviders , "FILTER_TARGETED_USER_LIST" => ParametersGroupBysItems :: FilterTargetedUserList , "FILTER_THIRD_PARTY_AUDIENCE_LIST_COST" => ParametersGroupBysItems :: FilterThirdPartyAudienceListCost , "FILTER_THIRD_PARTY_AUDIENCE_LIST_TYPE" => ParametersGroupBysItems :: FilterThirdPartyAudienceListType , "FILTER_TIME_OF_DAY" => ParametersGroupBysItems :: FilterTimeOfDay , "FILTER_TRUEVIEW_AD" => ParametersGroupBysItems :: FilterTrueviewAd , "FILTER_TRUEVIEW_AD_GROUP" => ParametersGroupBysItems :: FilterTrueviewAdGroup , "FILTER_TRUEVIEW_AD_GROUP_AD_ID" => ParametersGroupBysItems :: FilterTrueviewAdGroupAdId , "FILTER_TRUEVIEW_AD_GROUP_ID" => ParametersGroupBysItems :: FilterTrueviewAdGroupId , "FILTER_TRUEVIEW_AD_TYPE_NAME" => ParametersGroupBysItems :: FilterTrueviewAdTypeName , "FILTER_TRUEVIEW_AGE" => ParametersGroupBysItems :: FilterTrueviewAge , "FILTER_TRUEVIEW_CATEGORY" => ParametersGroupBysItems :: FilterTrueviewCategory , "FILTER_TRUEVIEW_CITY" => ParametersGroupBysItems :: FilterTrueviewCity , "FILTER_TRUEVIEW_CLICK_TYPE_NAME" => ParametersGroupBysItems :: FilterTrueviewClickTypeName , "FILTER_TRUEVIEW_CONVERSION_TYPE" => ParametersGroupBysItems :: FilterTrueviewConversionType , "FILTER_TRUEVIEW_COUNTRY" => ParametersGroupBysItems :: FilterTrueviewCountry , "FILTER_TRUEVIEW_CUSTOM_AFFINITY" => ParametersGroupBysItems :: FilterTrueviewCustomAffinity , "FILTER_TRUEVIEW_DETAILED_DEMOGRAPHICS" => ParametersGroupBysItems :: FilterTrueviewDetailedDemographics , "FILTER_TRUEVIEW_DETAILED_DEMOGRAPHICS_ID" => ParametersGroupBysItems :: FilterTrueviewDetailedDemographicsId , "FILTER_TRUEVIEW_DMA" => ParametersGroupBysItems :: FilterTrueviewDma , "FILTER_TRUEVIEW_DMA_NAME" => ParametersGroupBysItems :: FilterTrueviewDmaName , "FILTER_TRUEVIEW_GENDER" => ParametersGroupBysItems :: FilterTrueviewGender , "FILTER_TRUEVIEW_HOUSEHOLD_INCOME" => ParametersGroupBysItems :: FilterTrueviewHouseholdIncome , "FILTER_TRUEVIEW_IAR_AGE" => ParametersGroupBysItems :: FilterTrueviewIarAge , "FILTER_TRUEVIEW_IAR_CATEGORY" => ParametersGroupBysItems :: FilterTrueviewIarCategory , "FILTER_TRUEVIEW_IAR_CITY" => ParametersGroupBysItems :: FilterTrueviewIarCity , "FILTER_TRUEVIEW_IAR_COUNTRY" => ParametersGroupBysItems :: FilterTrueviewIarCountry , "FILTER_TRUEVIEW_IAR_COUNTRY_NAME" => ParametersGroupBysItems :: FilterTrueviewIarCountryName , "FILTER_TRUEVIEW_IAR_GENDER" => ParametersGroupBysItems :: FilterTrueviewIarGender , "FILTER_TRUEVIEW_IAR_INTEREST" => ParametersGroupBysItems :: FilterTrueviewIarInterest , "FILTER_TRUEVIEW_IAR_LANGUAGE" => ParametersGroupBysItems :: FilterTrueviewIarLanguage , "FILTER_TRUEVIEW_IAR_PARENTAL_STATUS" => ParametersGroupBysItems :: FilterTrueviewIarParentalStatus , "FILTER_TRUEVIEW_IAR_REGION" => ParametersGroupBysItems :: FilterTrueviewIarRegion , "FILTER_TRUEVIEW_IAR_REGION_NAME" => ParametersGroupBysItems :: FilterTrueviewIarRegionName , "FILTER_TRUEVIEW_IAR_REMARKETING_LIST" => ParametersGroupBysItems :: FilterTrueviewIarRemarketingList , "FILTER_TRUEVIEW_IAR_TIME_OF_DAY" => ParametersGroupBysItems :: FilterTrueviewIarTimeOfDay , "FILTER_TRUEVIEW_IAR_YOUTUBE_CHANNEL" => ParametersGroupBysItems :: FilterTrueviewIarYoutubeChannel , "FILTER_TRUEVIEW_IAR_YOUTUBE_VIDEO" => ParametersGroupBysItems :: FilterTrueviewIarYoutubeVideo , "FILTER_TRUEVIEW_IAR_ZIPCODE" => ParametersGroupBysItems :: FilterTrueviewIarZipcode , "FILTER_TRUEVIEW_INTEREST" => ParametersGroupBysItems :: FilterTrueviewInterest , "FILTER_TRUEVIEW_KEYWORD" => ParametersGroupBysItems :: FilterTrueviewKeyword , "FILTER_TRUEVIEW_PARENTAL_STATUS" => ParametersGroupBysItems :: FilterTrueviewParentalStatus , "FILTER_TRUEVIEW_PLACEMENT" => ParametersGroupBysItems :: FilterTrueviewPlacement , "FILTER_TRUEVIEW_PLACEMENT_ID" => ParametersGroupBysItems :: FilterTrueviewPlacementId , "FILTER_TRUEVIEW_REGION" => ParametersGroupBysItems :: FilterTrueviewRegion , "FILTER_TRUEVIEW_REGION_NAME" => ParametersGroupBysItems :: FilterTrueviewRegionName , "FILTER_TRUEVIEW_REMARKETING_LIST" => ParametersGroupBysItems :: FilterTrueviewRemarketingList , "FILTER_TRUEVIEW_REMARKETING_LIST_NAME" => ParametersGroupBysItems :: FilterTrueviewRemarketingListName , "FILTER_TRUEVIEW_URL" => ParametersGroupBysItems :: FilterTrueviewUrl , "FILTER_TRUEVIEW_ZIPCODE" => ParametersGroupBysItems :: FilterTrueviewZipcode , "FILTER_UNKNOWN" => ParametersGroupBysItems :: FilterUnknown , "FILTER_USER_LIST" => ParametersGroupBysItems :: FilterUserList , "FILTER_USER_LIST_FIRST_PARTY" => ParametersGroupBysItems :: FilterUserListFirstParty , "FILTER_USER_LIST_FIRST_PARTY_NAME" => ParametersGroupBysItems :: FilterUserListFirstPartyName , "FILTER_USER_LIST_THIRD_PARTY" => ParametersGroupBysItems :: FilterUserListThirdParty , "FILTER_USER_LIST_THIRD_PARTY_NAME" => ParametersGroupBysItems :: FilterUserListThirdPartyName , "FILTER_VARIANT_ID" => ParametersGroupBysItems :: FilterVariantId , "FILTER_VARIANT_NAME" => ParametersGroupBysItems :: FilterVariantName , "FILTER_VARIANT_VERSION" => ParametersGroupBysItems :: FilterVariantVersion , "FILTER_VENDOR_MEASUREMENT_MODE" => ParametersGroupBysItems :: FilterVendorMeasurementMode , "FILTER_VERIFICATION_AUDIBILITY_COMPLETE" => ParametersGroupBysItems :: FilterVerificationAudibilityComplete , "FILTER_VERIFICATION_AUDIBILITY_START" => ParametersGroupBysItems :: FilterVerificationAudibilityStart , "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE" => ParametersGroupBysItems :: FilterVerificationVideoPlayerSize , "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_COMPLETE" => ParametersGroupBysItems :: FilterVerificationVideoPlayerSizeComplete , "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_FIRST_QUARTILE" => ParametersGroupBysItems :: FilterVerificationVideoPlayerSizeFirstQuartile , "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_MID_POINT" => ParametersGroupBysItems :: FilterVerificationVideoPlayerSizeMidPoint , "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_START" => ParametersGroupBysItems :: FilterVerificationVideoPlayerSizeStart , "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_THIRD_QUARTILE" => ParametersGroupBysItems :: FilterVerificationVideoPlayerSizeThirdQuartile , "FILTER_VERIFICATION_VIDEO_POSITION" => ParametersGroupBysItems :: FilterVerificationVideoPosition , "FILTER_VERIFICATION_VIDEO_RESIZED" => ParametersGroupBysItems :: FilterVerificationVideoResized , "FILTER_VIDEO_AD_POSITION_IN_STREAM" => ParametersGroupBysItems :: FilterVideoAdPositionInStream , "FILTER_VIDEO_COMPANION_CREATIVE_SIZE" => ParametersGroupBysItems :: FilterVideoCompanionCreativeSize , "FILTER_VIDEO_CONTENT_DURATION" => ParametersGroupBysItems :: FilterVideoContentDuration , "FILTER_VIDEO_CONTENT_LIVE_STREAM" => ParametersGroupBysItems :: FilterVideoContentLiveStream , "FILTER_VIDEO_CONTINUOUS_PLAY" => ParametersGroupBysItems :: FilterVideoContinuousPlay , "FILTER_VIDEO_CREATIVE_DURATION" => ParametersGroupBysItems :: FilterVideoCreativeDuration , "FILTER_VIDEO_CREATIVE_DURATION_SKIPPABLE" => ParametersGroupBysItems :: FilterVideoCreativeDurationSkippable , "FILTER_VIDEO_DURATION" => ParametersGroupBysItems :: FilterVideoDuration , "FILTER_VIDEO_DURATION_SECONDS" => ParametersGroupBysItems :: FilterVideoDurationSeconds , "FILTER_VIDEO_DURATION_SECONDS_RANGE" => ParametersGroupBysItems :: FilterVideoDurationSecondsRange , "FILTER_VIDEO_FORMAT_SUPPORT" => ParametersGroupBysItems :: FilterVideoFormatSupport , "FILTER_VIDEO_PLAYER_SIZE" => ParametersGroupBysItems :: FilterVideoPlayerSize , "FILTER_VIDEO_RATING_TIER" => ParametersGroupBysItems :: FilterVideoRatingTier , "FILTER_VIDEO_SKIPPABLE_SUPPORT" => ParametersGroupBysItems :: FilterVideoSkippableSupport , "FILTER_WEEK" => ParametersGroupBysItems :: FilterWeek , "FILTER_YEAR" => ParametersGroupBysItems :: FilterYear , "FILTER_YOUTUBE_AD_VIDEO" => ParametersGroupBysItems :: FilterYoutubeAdVideo , "FILTER_YOUTUBE_AD_VIDEO_ID" => ParametersGroupBysItems :: FilterYoutubeAdVideoId , "FILTER_YOUTUBE_ADAPTED_AUDIENCE_LIST" => ParametersGroupBysItems :: FilterYoutubeAdaptedAudienceList , "FILTER_YOUTUBE_CHANNEL" => ParametersGroupBysItems :: FilterYoutubeChannel , "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_ADVERTISER" => ParametersGroupBysItems :: FilterYoutubeProgrammaticGuaranteedAdvertiser , "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_INSERTION_ORDER" => ParametersGroupBysItems :: FilterYoutubeProgrammaticGuaranteedInsertionOrder , "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_PARTNER" => ParametersGroupBysItems :: FilterYoutubeProgrammaticGuaranteedPartner , "FILTER_YOUTUBE_VIDEO" => ParametersGroupBysItems :: FilterYoutubeVideo , "FILTER_ZIP_CODE" => ParametersGroupBysItems :: FilterZipCode , "FILTER_ZIP_POSTAL_CODE" => ParametersGroupBysItems :: FilterZipPostalCode , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector for ParametersGroupBysItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ParametersGroupBysItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ParametersMetricsItems {
        MetricActiveViewAudibleFullyOnScreenHalfOfDurationImpressions,
        MetricActiveViewAudibleFullyOnScreenHalfOfDurationMeasurableImpressions,
        MetricActiveViewAudibleFullyOnScreenHalfOfDurationRate,
        MetricActiveViewAudibleFullyOnScreenHalfOfDurationTrueviewImpressions,
        MetricActiveViewAudibleFullyOnScreenHalfOfDurationTrueviewMeasurableImpressions,
        MetricActiveViewAudibleFullyOnScreenHalfOfDurationTrueviewRate,
        MetricActiveViewAudibleVisibleOnCompleteImpressions,
        MetricActiveViewAverageViewableTime,
        MetricActiveViewCustomMetricMeasurableImpressions,
        MetricActiveViewCustomMetricViewableImpressions,
        MetricActiveViewCustomMetricViewableRate,
        MetricActiveViewDistributionUnmeasurable,
        MetricActiveViewDistributionUnviewable,
        MetricActiveViewDistributionViewable,
        MetricActiveViewEligibleImpressions,
        MetricActiveViewMeasurableImpressions,
        MetricActiveViewPctMeasurableImpressions,
        MetricActiveViewPctViewableImpressions,
        MetricActiveViewPercentAudibleImpressions,
        MetricActiveViewPercentAudibleVisibleAtStart,
        MetricActiveViewPercentAudibleVisibleFirstQuar,
        MetricActiveViewPercentAudibleVisibleOnComplete,
        MetricActiveViewPercentAudibleVisibleSecondQuar,
        MetricActiveViewPercentAudibleVisibleThirdQuar,
        MetricActiveViewPercentFullScreen,
        MetricActiveViewPercentFullyOnScreen2Sec,
        MetricActiveViewPercentInBackground,
        MetricActiveViewPercentOfAdPlayed,
        MetricActiveViewPercentOfCompletedImpressionsAudibleAndVisible,
        MetricActiveViewPercentOfCompletedImpressionsVisible,
        MetricActiveViewPercentOfFirstQuartileImpressionsAudibleAndVisible,
        MetricActiveViewPercentOfFirstQuartileImpressionsVisible,
        MetricActiveViewPercentOfMidpointImpressionsAudibleAndVisible,
        MetricActiveViewPercentOfMidpointImpressionsVisible,
        MetricActiveViewPercentOfThirdQuartileImpressionsAudibleAndVisible,
        MetricActiveViewPercentOfThirdQuartileImpressionsVisible,
        MetricActiveViewPercentPlayTimeAudible,
        MetricActiveViewPercentPlayTimeAudibleAndVisible,
        MetricActiveViewPercentPlayTimeVisible,
        MetricActiveViewPercentViewableForTimeThreshold,
        MetricActiveViewPercentVisibleAtStart,
        MetricActiveViewPercentVisibleFirstQuar,
        MetricActiveViewPercentVisibleOnComplete,
        MetricActiveViewPercentVisibleSecondQuar,
        MetricActiveViewPercentVisibleThirdQuar,
        MetricActiveViewUnmeasurableImpressions,
        MetricActiveViewUnviewableImpressions,
        MetricActiveViewViewableForTimeThreshold,
        MetricActiveViewViewableImpressions,
        MetricActivityRevenue,
        MetricAdaptedAudienceFrequency,
        MetricAdlingoFeeAdvertiserCurrency,
        MetricAudioClientCostEcpclAdvertiserCurrency,
        MetricAudioMediaCostEcpclAdvertiserCurrency,
        MetricAudioMutesAudio,
        MetricAudioRevenueEcpclAdvertiserCurrency,
        MetricAudioUnmutesAudio,
        MetricAudioUnmutesVideo,
        MetricAverageDisplayTime,
        MetricAverageImpressionFrequencyPerUser,
        MetricAverageInteractionTime,
        MetricAverageWatchTimePerImpression,
        MetricBeginToRenderEligibleImpressions,
        MetricBeginToRenderImpressions,
        MetricBenchmarkFrequency,
        MetricBidRequests,
        MetricBillableCostAdvertiser,
        MetricBillableCostPartner,
        MetricBillableCostUsd,
        MetricBillableImpressions,
        MetricBrandLiftAbsoluteBrandLift,
        MetricBrandLiftAllSurveyResponses,
        MetricBrandLiftBaselinePositiveResponseRate,
        MetricBrandLiftBaselineSurveyResponses,
        MetricBrandLiftCostPerLiftedUser,
        MetricBrandLiftExposedSurveyResponses,
        MetricBrandLiftHeadroomBrandLift,
        MetricBrandLiftRelativeBrandLift,
        MetricBrandLiftUsers,
        MetricCardClicks,
        MetricClickToPostClickConversionRate,
        MetricClicks,
        MetricClientCostAdvertiserCurrency,
        MetricClientCostEcpaAdvertiserCurrency,
        MetricClientCostEcpaPcAdvertiserCurrency,
        MetricClientCostEcpaPvAdvertiserCurrency,
        MetricClientCostEcpcAdvertiserCurrency,
        MetricClientCostEcpmAdvertiserCurrency,
        MetricClientCostViewableEcpmAdvertiserCurrency,
        MetricCm360PostClickRevenue,
        MetricCm360PostClickRevenueCrossEnvironment,
        MetricCm360PostViewRevenue,
        MetricCm360PostViewRevenueCrossEnvironment,
        MetricCmPostClickRevenue,
        MetricCmPostClickRevenueCrossEnvironment,
        MetricCmPostViewRevenue,
        MetricCmPostViewRevenueCrossEnvironment,
        MetricCompanionClicksAudio,
        MetricCompanionImpressionsAudio,
        MetricCompleteListensAudio,
        MetricCompletionRateAudio,
        MetricConversionsPerMille,
        MetricConvertingPaths,
        MetricCookieConsentedFloodlightImpressions,
        MetricCookieReachAverageImpressionFrequency,
        MetricCookieReachImpressionReach,
        MetricCookieUnconsentedFloodlightImpressions,
        MetricCounters,
        MetricCpmFee1Advertiser,
        MetricCpmFee1Partner,
        MetricCpmFee1Usd,
        MetricCpmFee2Advertiser,
        MetricCpmFee2Partner,
        MetricCpmFee2Usd,
        MetricCpmFee3Advertiser,
        MetricCpmFee3Partner,
        MetricCpmFee3Usd,
        MetricCpmFee4Advertiser,
        MetricCpmFee4Partner,
        MetricCpmFee4Usd,
        MetricCpmFee5Advertiser,
        MetricCpmFee5Partner,
        MetricCpmFee5Usd,
        MetricCtr,
        MetricCustomFee1AdvertiserCurrency,
        MetricCustomFee2AdvertiserCurrency,
        MetricCustomFee3AdvertiserCurrency,
        MetricCustomFee4AdvertiserCurrency,
        MetricCustomFee5AdvertiserCurrency,
        MetricCustomValuePer1000Impressions,
        MetricDataCostAdvertiser,
        MetricDataCostPartner,
        MetricDataCostUsd,
        MetricDbmEngagementRate,
        MetricDemoCompositionImpression,
        MetricDemoCorrectedClicks,
        MetricDemoPopulation,
        MetricDuplicateFloodlightImpressions,
        MetricEngagementRate,
        MetricEngagements,
        MetricEstimatedCpmForImpressionsWithCustomValueAdvertiserCurrency,
        MetricEstimatedTotalCostForImpressionsWithCustomValueAdvertiserCurrency,
        MetricExits,
        MetricExpansions,
        MetricFee10Advertiser,
        MetricFee10Partner,
        MetricFee10Usd,
        MetricFee11Advertiser,
        MetricFee11Partner,
        MetricFee11Usd,
        MetricFee12Advertiser,
        MetricFee12Partner,
        MetricFee12Usd,
        MetricFee13Advertiser,
        MetricFee13Partner,
        MetricFee13Usd,
        MetricFee14Advertiser,
        MetricFee14Partner,
        MetricFee14Usd,
        MetricFee15Advertiser,
        MetricFee15Partner,
        MetricFee15Usd,
        MetricFee16Advertiser,
        MetricFee16Partner,
        MetricFee16Usd,
        MetricFee17Advertiser,
        MetricFee17Partner,
        MetricFee17Usd,
        MetricFee18Advertiser,
        MetricFee18Partner,
        MetricFee18Usd,
        MetricFee19Advertiser,
        MetricFee19Partner,
        MetricFee19Usd,
        MetricFee20Advertiser,
        MetricFee20Partner,
        MetricFee20Usd,
        MetricFee21Advertiser,
        MetricFee21Partner,
        MetricFee21Usd,
        MetricFee22Advertiser,
        MetricFee22Partner,
        MetricFee22Usd,
        MetricFee2Advertiser,
        MetricFee2Partner,
        MetricFee2Usd,
        MetricFee3Advertiser,
        MetricFee3Partner,
        MetricFee3Usd,
        MetricFee4Advertiser,
        MetricFee4Partner,
        MetricFee4Usd,
        MetricFee5Advertiser,
        MetricFee5Partner,
        MetricFee5Usd,
        MetricFee6Advertiser,
        MetricFee6Partner,
        MetricFee6Usd,
        MetricFee7Advertiser,
        MetricFee7Partner,
        MetricFee7Usd,
        MetricFee8Advertiser,
        MetricFee8Partner,
        MetricFee8Usd,
        MetricFee9Advertiser,
        MetricFee9Partner,
        MetricFee9Usd,
        MetricFirstQuartileAudio,
        MetricFloodlightImpressions,
        MetricGeneralInvalidTrafficGivtImpressions,
        MetricGeneralInvalidTrafficGivtTrackedAds,
        MetricGivtActiveViewEligibleImpressions,
        MetricGivtActiveViewMeasurableImpressions,
        MetricGivtActiveViewViewableImpressions,
        MetricGivtBeginToRenderImpressions,
        MetricGivtClicks,
        MetricGmailConversions,
        MetricGmailPostClickConversions,
        MetricGmailPostViewConversions,
        MetricGmailPotentialViews,
        MetricGrpCorrectedImpressions,
        MetricGrpCorrectedViewableImpressions,
        MetricGrpCorrectedViewableImpressionsSharePercent,
        MetricImpressionCustomValueCost,
        MetricImpressionLossTargetedImpressions,
        MetricImpressions,
        MetricImpressionsToConversionRate,
        MetricImpressionsWithCustomValue,
        MetricImpressionsWithPositiveCustomValue,
        MetricInteractiveImpressions,
        MetricInvalidActiveViewEligibleImpressions,
        MetricInvalidActiveViewMeasurableImpressions,
        MetricInvalidActiveViewViewableImpressions,
        MetricInvalidBeginToRenderImpressions,
        MetricInvalidClicks,
        MetricInvalidImpressions,
        MetricInvalidTrackedAds,
        MetricLastClicks,
        MetricLastImpressions,
        MetricLastTouchClickThroughConversions,
        MetricLastTouchTotalConversions,
        MetricLastTouchViewThroughConversions,
        MetricLineitemBidResponseCount,
        MetricMediaCostAdvertiser,
        MetricMediaCostEcpaAdvertiser,
        MetricMediaCostEcpaPartner,
        MetricMediaCostEcpaUsd,
        MetricMediaCostEcpapcAdvertiser,
        MetricMediaCostEcpapcPartner,
        MetricMediaCostEcpapcUsd,
        MetricMediaCostEcpapvAdvertiser,
        MetricMediaCostEcpapvPartner,
        MetricMediaCostEcpapvUsd,
        MetricMediaCostEcpcAdvertiser,
        MetricMediaCostEcpcPartner,
        MetricMediaCostEcpcUsd,
        MetricMediaCostEcpcvAdvertiser,
        MetricMediaCostEcpcvPartner,
        MetricMediaCostEcpcvUsd,
        MetricMediaCostEcpmAdvertiser,
        MetricMediaCostEcpmPartner,
        MetricMediaCostEcpmUsd,
        MetricMediaCostPartner,
        MetricMediaCostUsd,
        MetricMediaCostViewableEcpmAdvertiser,
        MetricMediaCostViewableEcpmPartner,
        MetricMediaCostViewableEcpmUsd,
        MetricMediaFee1Advertiser,
        MetricMediaFee1Partner,
        MetricMediaFee1Usd,
        MetricMediaFee2Advertiser,
        MetricMediaFee2Partner,
        MetricMediaFee2Usd,
        MetricMediaFee3Advertiser,
        MetricMediaFee3Partner,
        MetricMediaFee3Usd,
        MetricMediaFee4Advertiser,
        MetricMediaFee4Partner,
        MetricMediaFee4Usd,
        MetricMediaFee5Advertiser,
        MetricMediaFee5Partner,
        MetricMediaFee5Usd,
        MetricMidpointAudio,
        MetricNielsenAverageFrequency,
        MetricNielsenGrp,
        MetricNielsenImpressionIndex,
        MetricNielsenImpressions,
        MetricNielsenImpressionsShare,
        MetricNielsenPopulation,
        MetricNielsenPopulationReach,
        MetricNielsenPopulationShare,
        MetricNielsenReachIndex,
        MetricNielsenReachShare,
        MetricNielsenUniqueAudience,
        MetricOriginalAudienceFrequency,
        MetricPathConversionRate,
        MetricPausesAudio,
        MetricPercentImpressionsWithPositiveCustomValue,
        MetricPercentInvalidImpressionsPrebid,
        MetricPercentageFromCurrentIoGoal,
        MetricPlatformFeeAdvertiser,
        MetricPlatformFeePartner,
        MetricPlatformFeeRate,
        MetricPlatformFeeUsd,
        MetricPostClickConversionsCrossEnvironment,
        MetricPostViewConversionsCrossEnvironment,
        MetricPotentialImpressions,
        MetricPotentialViews,
        MetricPremiumFeeAdvertiserCurrency,
        MetricProfitAdvertiser,
        MetricProfitEcpmAdvertiser,
        MetricProfitEcpmPartner,
        MetricProfitEcpmUsd,
        MetricProfitMargin,
        MetricProfitPartner,
        MetricProfitUsd,
        MetricProfitViewableEcpmAdvertiser,
        MetricProfitViewableEcpmPartner,
        MetricProfitViewableEcpmUsd,
        MetricProgrammaticGuaranteedImpressionsPassedDueToFrequency,
        MetricProgrammaticGuaranteedSavingsReInvestedDueToFrequencyAdvertiserCurrency,
        MetricProvisionalImpressions,
        MetricRefundBillableCostAdvertiserCurrency,
        MetricRefundMediaCostAdvertiserCurrency,
        MetricRefundPlatformFeeAdvertiserCurrency,
        MetricRevenueAdvertiser,
        MetricRevenueEcpaAdvertiser,
        MetricRevenueEcpaPartner,
        MetricRevenueEcpaUsd,
        MetricRevenueEcpapcAdvertiser,
        MetricRevenueEcpapcPartner,
        MetricRevenueEcpapcUsd,
        MetricRevenueEcpapvAdvertiser,
        MetricRevenueEcpapvPartner,
        MetricRevenueEcpapvUsd,
        MetricRevenueEcpcAdvertiser,
        MetricRevenueEcpcPartner,
        MetricRevenueEcpcUsd,
        MetricRevenueEcpcvAdvertiser,
        MetricRevenueEcpcvPartner,
        MetricRevenueEcpcvUsd,
        MetricRevenueEcpmAdvertiser,
        MetricRevenueEcpmPartner,
        MetricRevenueEcpmUsd,
        MetricRevenuePartner,
        MetricRevenueUsd,
        MetricRevenueViewableEcpmAdvertiser,
        MetricRevenueViewableEcpmPartner,
        MetricRevenueViewableEcpmUsd,
        MetricRichMediaEngagements,
        MetricRichMediaScrolls,
        MetricRichMediaVideoCompletions,
        MetricRichMediaVideoFirstQuartileCompletes,
        MetricRichMediaVideoFullScreens,
        MetricRichMediaVideoMidpoints,
        MetricRichMediaVideoMutes,
        MetricRichMediaVideoPauses,
        MetricRichMediaVideoPlays,
        MetricRichMediaVideoSkips,
        MetricRichMediaVideoThirdQuartileCompletes,
        MetricStartsAudio,
        MetricStopsAudio,
        MetricStoreVisitConversions,
        MetricTargetRatingPoints,
        MetricTeaTrueviewImpressions,
        MetricTeaTrueviewUniqueCookies,
        MetricThirdQuartileAudio,
        MetricTimers,
        MetricTotalAudioMediaCostEcpclAdvertiserCurrency,
        MetricTotalConversions,
        MetricTotalConversionsCrossEnvironment,
        MetricTotalDisplayTime,
        MetricTotalExposures,
        MetricTotalImpressionCustomValue,
        MetricTotalInteractionTime,
        MetricTotalMediaCostAdvertiser,
        MetricTotalMediaCostEcpaAdvertiser,
        MetricTotalMediaCostEcpaPartner,
        MetricTotalMediaCostEcpaUsd,
        MetricTotalMediaCostEcpapcAdvertiser,
        MetricTotalMediaCostEcpapcPartner,
        MetricTotalMediaCostEcpapcUsd,
        MetricTotalMediaCostEcpapvAdvertiser,
        MetricTotalMediaCostEcpapvPartner,
        MetricTotalMediaCostEcpapvUsd,
        MetricTotalMediaCostEcpcAdvertiser,
        MetricTotalMediaCostEcpcPartner,
        MetricTotalMediaCostEcpcUsd,
        MetricTotalMediaCostEcpcvAdvertiser,
        MetricTotalMediaCostEcpcvPartner,
        MetricTotalMediaCostEcpcvUsd,
        MetricTotalMediaCostEcpmAdvertiser,
        MetricTotalMediaCostEcpmPartner,
        MetricTotalMediaCostEcpmUsd,
        MetricTotalMediaCostPartner,
        MetricTotalMediaCostUsd,
        MetricTotalMediaCostViewableEcpmAdvertiser,
        MetricTotalMediaCostViewableEcpmPartner,
        MetricTotalMediaCostViewableEcpmUsd,
        MetricTotalPaths,
        MetricTotalUsers,
        MetricTrackedAds,
        MetricTrackingUnconsentedClicks,
        MetricTrueviewAllAdSequenceImpressions,
        MetricTrueviewAverageCpeAdvertiser,
        MetricTrueviewAverageCpePartner,
        MetricTrueviewAverageCpeUsd,
        MetricTrueviewConversionCostManyPerViewAdvertiser,
        MetricTrueviewConversionCostManyPerViewPartner,
        MetricTrueviewConversionCostManyPerViewUsd,
        MetricTrueviewConversionManyPerView,
        MetricTrueviewConversionRateOnePerView,
        MetricTrueviewCpvAdvertiser,
        MetricTrueviewCpvPartner,
        MetricTrueviewCpvUsd,
        MetricTrueviewEarnedLikes,
        MetricTrueviewEarnedPlaylistAdditions,
        MetricTrueviewEarnedShares,
        MetricTrueviewEarnedSubscribers,
        MetricTrueviewEarnedViews,
        MetricTrueviewEngagementRate,
        MetricTrueviewEngagements,
        MetricTrueviewGeneralInvalidTrafficGivtViews,
        MetricTrueviewImpressionShare,
        MetricTrueviewInvalidViews,
        MetricTrueviewLostIsBudget,
        MetricTrueviewLostIsRank,
        MetricTrueviewTotalConversionValuesAdvertiser,
        MetricTrueviewTotalConversionValuesPartner,
        MetricTrueviewTotalConversionValuesUsd,
        MetricTrueviewUniqueViewers,
        MetricTrueviewViewRate,
        MetricTrueviewViewThroughConversion,
        MetricTrueviewViews,
        MetricUniqueCookiesWithImpressions,
        MetricUniqueReachAverageImpressionFrequency,
        MetricUniqueReachClickReach,
        MetricUniqueReachImpressionReach,
        MetricUniqueReachTotalReach,
        MetricUniqueVisitorsCookies,
        MetricUnknown,
        MetricVendorBlockedAds,
        MetricVerifiableImpressions,
        MetricVerificationVideoPlayerSizeMeasurableImpressions,
        MetricVideoClientCostEcpcvAdvertiserCurrency,
        MetricVideoCompanionClicks,
        MetricVideoCompanionImpressions,
        MetricVideoCompletionRate,
        MetricViewableBidRequests,
        MetricViewableGrossRatingPoints,
        MetricVirtualPeopleAverageImpressionFrequencyByDemo,
        MetricVirtualPeopleAverageViewableImpressionFrequencyByDemo,
        MetricVirtualPeopleClickReachByDemo,
        MetricVirtualPeopleImpressionReachByDemo,
        MetricVirtualPeopleImpressionReachPercent,
        MetricVirtualPeopleImpressionReachSharePercent,
        MetricVirtualPeopleViewableImpressionReachByDemo,
        MetricVirtualPeopleViewableImpressionReachPercent,
        MetricVirtualPeopleViewableImpressionReachSharePercent,
        MetricWatchTime,
        MetricWinLossDealAvailableRequests,
        MetricWinLossDealTargetedImpressions,
        MetricWinLossLineitemAvailableRequests,
        MetricWinLossLineitemTargetedImpressions,
        MetricWinLossRate,
    }
    impl ParametersMetricsItems {
        pub fn as_str(self) -> &'static str {
            match self { ParametersMetricsItems :: MetricActiveViewAudibleFullyOnScreenHalfOfDurationImpressions => "METRIC_ACTIVE_VIEW_AUDIBLE_FULLY_ON_SCREEN_HALF_OF_DURATION_IMPRESSIONS" , ParametersMetricsItems :: MetricActiveViewAudibleFullyOnScreenHalfOfDurationMeasurableImpressions => "METRIC_ACTIVE_VIEW_AUDIBLE_FULLY_ON_SCREEN_HALF_OF_DURATION_MEASURABLE_IMPRESSIONS" , ParametersMetricsItems :: MetricActiveViewAudibleFullyOnScreenHalfOfDurationRate => "METRIC_ACTIVE_VIEW_AUDIBLE_FULLY_ON_SCREEN_HALF_OF_DURATION_RATE" , ParametersMetricsItems :: MetricActiveViewAudibleFullyOnScreenHalfOfDurationTrueviewImpressions => "METRIC_ACTIVE_VIEW_AUDIBLE_FULLY_ON_SCREEN_HALF_OF_DURATION_TRUEVIEW_IMPRESSIONS" , ParametersMetricsItems :: MetricActiveViewAudibleFullyOnScreenHalfOfDurationTrueviewMeasurableImpressions => "METRIC_ACTIVE_VIEW_AUDIBLE_FULLY_ON_SCREEN_HALF_OF_DURATION_TRUEVIEW_MEASURABLE_IMPRESSIONS" , ParametersMetricsItems :: MetricActiveViewAudibleFullyOnScreenHalfOfDurationTrueviewRate => "METRIC_ACTIVE_VIEW_AUDIBLE_FULLY_ON_SCREEN_HALF_OF_DURATION_TRUEVIEW_RATE" , ParametersMetricsItems :: MetricActiveViewAudibleVisibleOnCompleteImpressions => "METRIC_ACTIVE_VIEW_AUDIBLE_VISIBLE_ON_COMPLETE_IMPRESSIONS" , ParametersMetricsItems :: MetricActiveViewAverageViewableTime => "METRIC_ACTIVE_VIEW_AVERAGE_VIEWABLE_TIME" , ParametersMetricsItems :: MetricActiveViewCustomMetricMeasurableImpressions => "METRIC_ACTIVE_VIEW_CUSTOM_METRIC_MEASURABLE_IMPRESSIONS" , ParametersMetricsItems :: MetricActiveViewCustomMetricViewableImpressions => "METRIC_ACTIVE_VIEW_CUSTOM_METRIC_VIEWABLE_IMPRESSIONS" , ParametersMetricsItems :: MetricActiveViewCustomMetricViewableRate => "METRIC_ACTIVE_VIEW_CUSTOM_METRIC_VIEWABLE_RATE" , ParametersMetricsItems :: MetricActiveViewDistributionUnmeasurable => "METRIC_ACTIVE_VIEW_DISTRIBUTION_UNMEASURABLE" , ParametersMetricsItems :: MetricActiveViewDistributionUnviewable => "METRIC_ACTIVE_VIEW_DISTRIBUTION_UNVIEWABLE" , ParametersMetricsItems :: MetricActiveViewDistributionViewable => "METRIC_ACTIVE_VIEW_DISTRIBUTION_VIEWABLE" , ParametersMetricsItems :: MetricActiveViewEligibleImpressions => "METRIC_ACTIVE_VIEW_ELIGIBLE_IMPRESSIONS" , ParametersMetricsItems :: MetricActiveViewMeasurableImpressions => "METRIC_ACTIVE_VIEW_MEASURABLE_IMPRESSIONS" , ParametersMetricsItems :: MetricActiveViewPctMeasurableImpressions => "METRIC_ACTIVE_VIEW_PCT_MEASURABLE_IMPRESSIONS" , ParametersMetricsItems :: MetricActiveViewPctViewableImpressions => "METRIC_ACTIVE_VIEW_PCT_VIEWABLE_IMPRESSIONS" , ParametersMetricsItems :: MetricActiveViewPercentAudibleImpressions => "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_IMPRESSIONS" , ParametersMetricsItems :: MetricActiveViewPercentAudibleVisibleAtStart => "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_AT_START" , ParametersMetricsItems :: MetricActiveViewPercentAudibleVisibleFirstQuar => "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_FIRST_QUAR" , ParametersMetricsItems :: MetricActiveViewPercentAudibleVisibleOnComplete => "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_ON_COMPLETE" , ParametersMetricsItems :: MetricActiveViewPercentAudibleVisibleSecondQuar => "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_SECOND_QUAR" , ParametersMetricsItems :: MetricActiveViewPercentAudibleVisibleThirdQuar => "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_THIRD_QUAR" , ParametersMetricsItems :: MetricActiveViewPercentFullScreen => "METRIC_ACTIVE_VIEW_PERCENT_FULL_SCREEN" , ParametersMetricsItems :: MetricActiveViewPercentFullyOnScreen2Sec => "METRIC_ACTIVE_VIEW_PERCENT_FULLY_ON_SCREEN_2_SEC" , ParametersMetricsItems :: MetricActiveViewPercentInBackground => "METRIC_ACTIVE_VIEW_PERCENT_IN_BACKGROUND" , ParametersMetricsItems :: MetricActiveViewPercentOfAdPlayed => "METRIC_ACTIVE_VIEW_PERCENT_OF_AD_PLAYED" , ParametersMetricsItems :: MetricActiveViewPercentOfCompletedImpressionsAudibleAndVisible => "METRIC_ACTIVE_VIEW_PERCENT_OF_COMPLETED_IMPRESSIONS_AUDIBLE_AND_VISIBLE" , ParametersMetricsItems :: MetricActiveViewPercentOfCompletedImpressionsVisible => "METRIC_ACTIVE_VIEW_PERCENT_OF_COMPLETED_IMPRESSIONS_VISIBLE" , ParametersMetricsItems :: MetricActiveViewPercentOfFirstQuartileImpressionsAudibleAndVisible => "METRIC_ACTIVE_VIEW_PERCENT_OF_FIRST_QUARTILE_IMPRESSIONS_AUDIBLE_AND_VISIBLE" , ParametersMetricsItems :: MetricActiveViewPercentOfFirstQuartileImpressionsVisible => "METRIC_ACTIVE_VIEW_PERCENT_OF_FIRST_QUARTILE_IMPRESSIONS_VISIBLE" , ParametersMetricsItems :: MetricActiveViewPercentOfMidpointImpressionsAudibleAndVisible => "METRIC_ACTIVE_VIEW_PERCENT_OF_MIDPOINT_IMPRESSIONS_AUDIBLE_AND_VISIBLE" , ParametersMetricsItems :: MetricActiveViewPercentOfMidpointImpressionsVisible => "METRIC_ACTIVE_VIEW_PERCENT_OF_MIDPOINT_IMPRESSIONS_VISIBLE" , ParametersMetricsItems :: MetricActiveViewPercentOfThirdQuartileImpressionsAudibleAndVisible => "METRIC_ACTIVE_VIEW_PERCENT_OF_THIRD_QUARTILE_IMPRESSIONS_AUDIBLE_AND_VISIBLE" , ParametersMetricsItems :: MetricActiveViewPercentOfThirdQuartileImpressionsVisible => "METRIC_ACTIVE_VIEW_PERCENT_OF_THIRD_QUARTILE_IMPRESSIONS_VISIBLE" , ParametersMetricsItems :: MetricActiveViewPercentPlayTimeAudible => "METRIC_ACTIVE_VIEW_PERCENT_PLAY_TIME_AUDIBLE" , ParametersMetricsItems :: MetricActiveViewPercentPlayTimeAudibleAndVisible => "METRIC_ACTIVE_VIEW_PERCENT_PLAY_TIME_AUDIBLE_AND_VISIBLE" , ParametersMetricsItems :: MetricActiveViewPercentPlayTimeVisible => "METRIC_ACTIVE_VIEW_PERCENT_PLAY_TIME_VISIBLE" , ParametersMetricsItems :: MetricActiveViewPercentViewableForTimeThreshold => "METRIC_ACTIVE_VIEW_PERCENT_VIEWABLE_FOR_TIME_THRESHOLD" , ParametersMetricsItems :: MetricActiveViewPercentVisibleAtStart => "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_AT_START" , ParametersMetricsItems :: MetricActiveViewPercentVisibleFirstQuar => "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_FIRST_QUAR" , ParametersMetricsItems :: MetricActiveViewPercentVisibleOnComplete => "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_ON_COMPLETE" , ParametersMetricsItems :: MetricActiveViewPercentVisibleSecondQuar => "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_SECOND_QUAR" , ParametersMetricsItems :: MetricActiveViewPercentVisibleThirdQuar => "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_THIRD_QUAR" , ParametersMetricsItems :: MetricActiveViewUnmeasurableImpressions => "METRIC_ACTIVE_VIEW_UNMEASURABLE_IMPRESSIONS" , ParametersMetricsItems :: MetricActiveViewUnviewableImpressions => "METRIC_ACTIVE_VIEW_UNVIEWABLE_IMPRESSIONS" , ParametersMetricsItems :: MetricActiveViewViewableForTimeThreshold => "METRIC_ACTIVE_VIEW_VIEWABLE_FOR_TIME_THRESHOLD" , ParametersMetricsItems :: MetricActiveViewViewableImpressions => "METRIC_ACTIVE_VIEW_VIEWABLE_IMPRESSIONS" , ParametersMetricsItems :: MetricActivityRevenue => "METRIC_ACTIVITY_REVENUE" , ParametersMetricsItems :: MetricAdaptedAudienceFrequency => "METRIC_ADAPTED_AUDIENCE_FREQUENCY" , ParametersMetricsItems :: MetricAdlingoFeeAdvertiserCurrency => "METRIC_ADLINGO_FEE_ADVERTISER_CURRENCY" , ParametersMetricsItems :: MetricAudioClientCostEcpclAdvertiserCurrency => "METRIC_AUDIO_CLIENT_COST_ECPCL_ADVERTISER_CURRENCY" , ParametersMetricsItems :: MetricAudioMediaCostEcpclAdvertiserCurrency => "METRIC_AUDIO_MEDIA_COST_ECPCL_ADVERTISER_CURRENCY" , ParametersMetricsItems :: MetricAudioMutesAudio => "METRIC_AUDIO_MUTES_AUDIO" , ParametersMetricsItems :: MetricAudioRevenueEcpclAdvertiserCurrency => "METRIC_AUDIO_REVENUE_ECPCL_ADVERTISER_CURRENCY" , ParametersMetricsItems :: MetricAudioUnmutesAudio => "METRIC_AUDIO_UNMUTES_AUDIO" , ParametersMetricsItems :: MetricAudioUnmutesVideo => "METRIC_AUDIO_UNMUTES_VIDEO" , ParametersMetricsItems :: MetricAverageDisplayTime => "METRIC_AVERAGE_DISPLAY_TIME" , ParametersMetricsItems :: MetricAverageImpressionFrequencyPerUser => "METRIC_AVERAGE_IMPRESSION_FREQUENCY_PER_USER" , ParametersMetricsItems :: MetricAverageInteractionTime => "METRIC_AVERAGE_INTERACTION_TIME" , ParametersMetricsItems :: MetricAverageWatchTimePerImpression => "METRIC_AVERAGE_WATCH_TIME_PER_IMPRESSION" , ParametersMetricsItems :: MetricBeginToRenderEligibleImpressions => "METRIC_BEGIN_TO_RENDER_ELIGIBLE_IMPRESSIONS" , ParametersMetricsItems :: MetricBeginToRenderImpressions => "METRIC_BEGIN_TO_RENDER_IMPRESSIONS" , ParametersMetricsItems :: MetricBenchmarkFrequency => "METRIC_BENCHMARK_FREQUENCY" , ParametersMetricsItems :: MetricBidRequests => "METRIC_BID_REQUESTS" , ParametersMetricsItems :: MetricBillableCostAdvertiser => "METRIC_BILLABLE_COST_ADVERTISER" , ParametersMetricsItems :: MetricBillableCostPartner => "METRIC_BILLABLE_COST_PARTNER" , ParametersMetricsItems :: MetricBillableCostUsd => "METRIC_BILLABLE_COST_USD" , ParametersMetricsItems :: MetricBillableImpressions => "METRIC_BILLABLE_IMPRESSIONS" , ParametersMetricsItems :: MetricBrandLiftAbsoluteBrandLift => "METRIC_BRAND_LIFT_ABSOLUTE_BRAND_LIFT" , ParametersMetricsItems :: MetricBrandLiftAllSurveyResponses => "METRIC_BRAND_LIFT_ALL_SURVEY_RESPONSES" , ParametersMetricsItems :: MetricBrandLiftBaselinePositiveResponseRate => "METRIC_BRAND_LIFT_BASELINE_POSITIVE_RESPONSE_RATE" , ParametersMetricsItems :: MetricBrandLiftBaselineSurveyResponses => "METRIC_BRAND_LIFT_BASELINE_SURVEY_RESPONSES" , ParametersMetricsItems :: MetricBrandLiftCostPerLiftedUser => "METRIC_BRAND_LIFT_COST_PER_LIFTED_USER" , ParametersMetricsItems :: MetricBrandLiftExposedSurveyResponses => "METRIC_BRAND_LIFT_EXPOSED_SURVEY_RESPONSES" , ParametersMetricsItems :: MetricBrandLiftHeadroomBrandLift => "METRIC_BRAND_LIFT_HEADROOM_BRAND_LIFT" , ParametersMetricsItems :: MetricBrandLiftRelativeBrandLift => "METRIC_BRAND_LIFT_RELATIVE_BRAND_LIFT" , ParametersMetricsItems :: MetricBrandLiftUsers => "METRIC_BRAND_LIFT_USERS" , ParametersMetricsItems :: MetricCardClicks => "METRIC_CARD_CLICKS" , ParametersMetricsItems :: MetricClickToPostClickConversionRate => "METRIC_CLICK_TO_POST_CLICK_CONVERSION_RATE" , ParametersMetricsItems :: MetricClicks => "METRIC_CLICKS" , ParametersMetricsItems :: MetricClientCostAdvertiserCurrency => "METRIC_CLIENT_COST_ADVERTISER_CURRENCY" , ParametersMetricsItems :: MetricClientCostEcpaAdvertiserCurrency => "METRIC_CLIENT_COST_ECPA_ADVERTISER_CURRENCY" , ParametersMetricsItems :: MetricClientCostEcpaPcAdvertiserCurrency => "METRIC_CLIENT_COST_ECPA_PC_ADVERTISER_CURRENCY" , ParametersMetricsItems :: MetricClientCostEcpaPvAdvertiserCurrency => "METRIC_CLIENT_COST_ECPA_PV_ADVERTISER_CURRENCY" , ParametersMetricsItems :: MetricClientCostEcpcAdvertiserCurrency => "METRIC_CLIENT_COST_ECPC_ADVERTISER_CURRENCY" , ParametersMetricsItems :: MetricClientCostEcpmAdvertiserCurrency => "METRIC_CLIENT_COST_ECPM_ADVERTISER_CURRENCY" , ParametersMetricsItems :: MetricClientCostViewableEcpmAdvertiserCurrency => "METRIC_CLIENT_COST_VIEWABLE_ECPM_ADVERTISER_CURRENCY" , ParametersMetricsItems :: MetricCm360PostClickRevenue => "METRIC_CM360_POST_CLICK_REVENUE" , ParametersMetricsItems :: MetricCm360PostClickRevenueCrossEnvironment => "METRIC_CM360_POST_CLICK_REVENUE_CROSS_ENVIRONMENT" , ParametersMetricsItems :: MetricCm360PostViewRevenue => "METRIC_CM360_POST_VIEW_REVENUE" , ParametersMetricsItems :: MetricCm360PostViewRevenueCrossEnvironment => "METRIC_CM360_POST_VIEW_REVENUE_CROSS_ENVIRONMENT" , ParametersMetricsItems :: MetricCmPostClickRevenue => "METRIC_CM_POST_CLICK_REVENUE" , ParametersMetricsItems :: MetricCmPostClickRevenueCrossEnvironment => "METRIC_CM_POST_CLICK_REVENUE_CROSS_ENVIRONMENT" , ParametersMetricsItems :: MetricCmPostViewRevenue => "METRIC_CM_POST_VIEW_REVENUE" , ParametersMetricsItems :: MetricCmPostViewRevenueCrossEnvironment => "METRIC_CM_POST_VIEW_REVENUE_CROSS_ENVIRONMENT" , ParametersMetricsItems :: MetricCompanionClicksAudio => "METRIC_COMPANION_CLICKS_AUDIO" , ParametersMetricsItems :: MetricCompanionImpressionsAudio => "METRIC_COMPANION_IMPRESSIONS_AUDIO" , ParametersMetricsItems :: MetricCompleteListensAudio => "METRIC_COMPLETE_LISTENS_AUDIO" , ParametersMetricsItems :: MetricCompletionRateAudio => "METRIC_COMPLETION_RATE_AUDIO" , ParametersMetricsItems :: MetricConversionsPerMille => "METRIC_CONVERSIONS_PER_MILLE" , ParametersMetricsItems :: MetricConvertingPaths => "METRIC_CONVERTING_PATHS" , ParametersMetricsItems :: MetricCookieConsentedFloodlightImpressions => "METRIC_COOKIE_CONSENTED_FLOODLIGHT_IMPRESSIONS" , ParametersMetricsItems :: MetricCookieReachAverageImpressionFrequency => "METRIC_COOKIE_REACH_AVERAGE_IMPRESSION_FREQUENCY" , ParametersMetricsItems :: MetricCookieReachImpressionReach => "METRIC_COOKIE_REACH_IMPRESSION_REACH" , ParametersMetricsItems :: MetricCookieUnconsentedFloodlightImpressions => "METRIC_COOKIE_UNCONSENTED_FLOODLIGHT_IMPRESSIONS" , ParametersMetricsItems :: MetricCounters => "METRIC_COUNTERS" , ParametersMetricsItems :: MetricCpmFee1Advertiser => "METRIC_CPM_FEE1_ADVERTISER" , ParametersMetricsItems :: MetricCpmFee1Partner => "METRIC_CPM_FEE1_PARTNER" , ParametersMetricsItems :: MetricCpmFee1Usd => "METRIC_CPM_FEE1_USD" , ParametersMetricsItems :: MetricCpmFee2Advertiser => "METRIC_CPM_FEE2_ADVERTISER" , ParametersMetricsItems :: MetricCpmFee2Partner => "METRIC_CPM_FEE2_PARTNER" , ParametersMetricsItems :: MetricCpmFee2Usd => "METRIC_CPM_FEE2_USD" , ParametersMetricsItems :: MetricCpmFee3Advertiser => "METRIC_CPM_FEE3_ADVERTISER" , ParametersMetricsItems :: MetricCpmFee3Partner => "METRIC_CPM_FEE3_PARTNER" , ParametersMetricsItems :: MetricCpmFee3Usd => "METRIC_CPM_FEE3_USD" , ParametersMetricsItems :: MetricCpmFee4Advertiser => "METRIC_CPM_FEE4_ADVERTISER" , ParametersMetricsItems :: MetricCpmFee4Partner => "METRIC_CPM_FEE4_PARTNER" , ParametersMetricsItems :: MetricCpmFee4Usd => "METRIC_CPM_FEE4_USD" , ParametersMetricsItems :: MetricCpmFee5Advertiser => "METRIC_CPM_FEE5_ADVERTISER" , ParametersMetricsItems :: MetricCpmFee5Partner => "METRIC_CPM_FEE5_PARTNER" , ParametersMetricsItems :: MetricCpmFee5Usd => "METRIC_CPM_FEE5_USD" , ParametersMetricsItems :: MetricCtr => "METRIC_CTR" , ParametersMetricsItems :: MetricCustomFee1AdvertiserCurrency => "METRIC_CUSTOM_FEE_1_ADVERTISER_CURRENCY" , ParametersMetricsItems :: MetricCustomFee2AdvertiserCurrency => "METRIC_CUSTOM_FEE_2_ADVERTISER_CURRENCY" , ParametersMetricsItems :: MetricCustomFee3AdvertiserCurrency => "METRIC_CUSTOM_FEE_3_ADVERTISER_CURRENCY" , ParametersMetricsItems :: MetricCustomFee4AdvertiserCurrency => "METRIC_CUSTOM_FEE_4_ADVERTISER_CURRENCY" , ParametersMetricsItems :: MetricCustomFee5AdvertiserCurrency => "METRIC_CUSTOM_FEE_5_ADVERTISER_CURRENCY" , ParametersMetricsItems :: MetricCustomValuePer1000Impressions => "METRIC_CUSTOM_VALUE_PER_1000_IMPRESSIONS" , ParametersMetricsItems :: MetricDataCostAdvertiser => "METRIC_DATA_COST_ADVERTISER" , ParametersMetricsItems :: MetricDataCostPartner => "METRIC_DATA_COST_PARTNER" , ParametersMetricsItems :: MetricDataCostUsd => "METRIC_DATA_COST_USD" , ParametersMetricsItems :: MetricDbmEngagementRate => "METRIC_DBM_ENGAGEMENT_RATE" , ParametersMetricsItems :: MetricDemoCompositionImpression => "METRIC_DEMO_COMPOSITION_IMPRESSION" , ParametersMetricsItems :: MetricDemoCorrectedClicks => "METRIC_DEMO_CORRECTED_CLICKS" , ParametersMetricsItems :: MetricDemoPopulation => "METRIC_DEMO_POPULATION" , ParametersMetricsItems :: MetricDuplicateFloodlightImpressions => "METRIC_DUPLICATE_FLOODLIGHT_IMPRESSIONS" , ParametersMetricsItems :: MetricEngagementRate => "METRIC_ENGAGEMENT_RATE" , ParametersMetricsItems :: MetricEngagements => "METRIC_ENGAGEMENTS" , ParametersMetricsItems :: MetricEstimatedCpmForImpressionsWithCustomValueAdvertiserCurrency => "METRIC_ESTIMATED_CPM_FOR_IMPRESSIONS_WITH_CUSTOM_VALUE_ADVERTISER_CURRENCY" , ParametersMetricsItems :: MetricEstimatedTotalCostForImpressionsWithCustomValueAdvertiserCurrency => "METRIC_ESTIMATED_TOTAL_COST_FOR_IMPRESSIONS_WITH_CUSTOM_VALUE_ADVERTISER_CURRENCY" , ParametersMetricsItems :: MetricExits => "METRIC_EXITS" , ParametersMetricsItems :: MetricExpansions => "METRIC_EXPANSIONS" , ParametersMetricsItems :: MetricFee10Advertiser => "METRIC_FEE10_ADVERTISER" , ParametersMetricsItems :: MetricFee10Partner => "METRIC_FEE10_PARTNER" , ParametersMetricsItems :: MetricFee10Usd => "METRIC_FEE10_USD" , ParametersMetricsItems :: MetricFee11Advertiser => "METRIC_FEE11_ADVERTISER" , ParametersMetricsItems :: MetricFee11Partner => "METRIC_FEE11_PARTNER" , ParametersMetricsItems :: MetricFee11Usd => "METRIC_FEE11_USD" , ParametersMetricsItems :: MetricFee12Advertiser => "METRIC_FEE12_ADVERTISER" , ParametersMetricsItems :: MetricFee12Partner => "METRIC_FEE12_PARTNER" , ParametersMetricsItems :: MetricFee12Usd => "METRIC_FEE12_USD" , ParametersMetricsItems :: MetricFee13Advertiser => "METRIC_FEE13_ADVERTISER" , ParametersMetricsItems :: MetricFee13Partner => "METRIC_FEE13_PARTNER" , ParametersMetricsItems :: MetricFee13Usd => "METRIC_FEE13_USD" , ParametersMetricsItems :: MetricFee14Advertiser => "METRIC_FEE14_ADVERTISER" , ParametersMetricsItems :: MetricFee14Partner => "METRIC_FEE14_PARTNER" , ParametersMetricsItems :: MetricFee14Usd => "METRIC_FEE14_USD" , ParametersMetricsItems :: MetricFee15Advertiser => "METRIC_FEE15_ADVERTISER" , ParametersMetricsItems :: MetricFee15Partner => "METRIC_FEE15_PARTNER" , ParametersMetricsItems :: MetricFee15Usd => "METRIC_FEE15_USD" , ParametersMetricsItems :: MetricFee16Advertiser => "METRIC_FEE16_ADVERTISER" , ParametersMetricsItems :: MetricFee16Partner => "METRIC_FEE16_PARTNER" , ParametersMetricsItems :: MetricFee16Usd => "METRIC_FEE16_USD" , ParametersMetricsItems :: MetricFee17Advertiser => "METRIC_FEE17_ADVERTISER" , ParametersMetricsItems :: MetricFee17Partner => "METRIC_FEE17_PARTNER" , ParametersMetricsItems :: MetricFee17Usd => "METRIC_FEE17_USD" , ParametersMetricsItems :: MetricFee18Advertiser => "METRIC_FEE18_ADVERTISER" , ParametersMetricsItems :: MetricFee18Partner => "METRIC_FEE18_PARTNER" , ParametersMetricsItems :: MetricFee18Usd => "METRIC_FEE18_USD" , ParametersMetricsItems :: MetricFee19Advertiser => "METRIC_FEE19_ADVERTISER" , ParametersMetricsItems :: MetricFee19Partner => "METRIC_FEE19_PARTNER" , ParametersMetricsItems :: MetricFee19Usd => "METRIC_FEE19_USD" , ParametersMetricsItems :: MetricFee20Advertiser => "METRIC_FEE20_ADVERTISER" , ParametersMetricsItems :: MetricFee20Partner => "METRIC_FEE20_PARTNER" , ParametersMetricsItems :: MetricFee20Usd => "METRIC_FEE20_USD" , ParametersMetricsItems :: MetricFee21Advertiser => "METRIC_FEE21_ADVERTISER" , ParametersMetricsItems :: MetricFee21Partner => "METRIC_FEE21_PARTNER" , ParametersMetricsItems :: MetricFee21Usd => "METRIC_FEE21_USD" , ParametersMetricsItems :: MetricFee22Advertiser => "METRIC_FEE22_ADVERTISER" , ParametersMetricsItems :: MetricFee22Partner => "METRIC_FEE22_PARTNER" , ParametersMetricsItems :: MetricFee22Usd => "METRIC_FEE22_USD" , ParametersMetricsItems :: MetricFee2Advertiser => "METRIC_FEE2_ADVERTISER" , ParametersMetricsItems :: MetricFee2Partner => "METRIC_FEE2_PARTNER" , ParametersMetricsItems :: MetricFee2Usd => "METRIC_FEE2_USD" , ParametersMetricsItems :: MetricFee3Advertiser => "METRIC_FEE3_ADVERTISER" , ParametersMetricsItems :: MetricFee3Partner => "METRIC_FEE3_PARTNER" , ParametersMetricsItems :: MetricFee3Usd => "METRIC_FEE3_USD" , ParametersMetricsItems :: MetricFee4Advertiser => "METRIC_FEE4_ADVERTISER" , ParametersMetricsItems :: MetricFee4Partner => "METRIC_FEE4_PARTNER" , ParametersMetricsItems :: MetricFee4Usd => "METRIC_FEE4_USD" , ParametersMetricsItems :: MetricFee5Advertiser => "METRIC_FEE5_ADVERTISER" , ParametersMetricsItems :: MetricFee5Partner => "METRIC_FEE5_PARTNER" , ParametersMetricsItems :: MetricFee5Usd => "METRIC_FEE5_USD" , ParametersMetricsItems :: MetricFee6Advertiser => "METRIC_FEE6_ADVERTISER" , ParametersMetricsItems :: MetricFee6Partner => "METRIC_FEE6_PARTNER" , ParametersMetricsItems :: MetricFee6Usd => "METRIC_FEE6_USD" , ParametersMetricsItems :: MetricFee7Advertiser => "METRIC_FEE7_ADVERTISER" , ParametersMetricsItems :: MetricFee7Partner => "METRIC_FEE7_PARTNER" , ParametersMetricsItems :: MetricFee7Usd => "METRIC_FEE7_USD" , ParametersMetricsItems :: MetricFee8Advertiser => "METRIC_FEE8_ADVERTISER" , ParametersMetricsItems :: MetricFee8Partner => "METRIC_FEE8_PARTNER" , ParametersMetricsItems :: MetricFee8Usd => "METRIC_FEE8_USD" , ParametersMetricsItems :: MetricFee9Advertiser => "METRIC_FEE9_ADVERTISER" , ParametersMetricsItems :: MetricFee9Partner => "METRIC_FEE9_PARTNER" , ParametersMetricsItems :: MetricFee9Usd => "METRIC_FEE9_USD" , ParametersMetricsItems :: MetricFirstQuartileAudio => "METRIC_FIRST_QUARTILE_AUDIO" , ParametersMetricsItems :: MetricFloodlightImpressions => "METRIC_FLOODLIGHT_IMPRESSIONS" , ParametersMetricsItems :: MetricGeneralInvalidTrafficGivtImpressions => "METRIC_GENERAL_INVALID_TRAFFIC_GIVT_IMPRESSIONS" , ParametersMetricsItems :: MetricGeneralInvalidTrafficGivtTrackedAds => "METRIC_GENERAL_INVALID_TRAFFIC_GIVT_TRACKED_ADS" , ParametersMetricsItems :: MetricGivtActiveViewEligibleImpressions => "METRIC_GIVT_ACTIVE_VIEW_ELIGIBLE_IMPRESSIONS" , ParametersMetricsItems :: MetricGivtActiveViewMeasurableImpressions => "METRIC_GIVT_ACTIVE_VIEW_MEASURABLE_IMPRESSIONS" , ParametersMetricsItems :: MetricGivtActiveViewViewableImpressions => "METRIC_GIVT_ACTIVE_VIEW_VIEWABLE_IMPRESSIONS" , ParametersMetricsItems :: MetricGivtBeginToRenderImpressions => "METRIC_GIVT_BEGIN_TO_RENDER_IMPRESSIONS" , ParametersMetricsItems :: MetricGivtClicks => "METRIC_GIVT_CLICKS" , ParametersMetricsItems :: MetricGmailConversions => "METRIC_GMAIL_CONVERSIONS" , ParametersMetricsItems :: MetricGmailPostClickConversions => "METRIC_GMAIL_POST_CLICK_CONVERSIONS" , ParametersMetricsItems :: MetricGmailPostViewConversions => "METRIC_GMAIL_POST_VIEW_CONVERSIONS" , ParametersMetricsItems :: MetricGmailPotentialViews => "METRIC_GMAIL_POTENTIAL_VIEWS" , ParametersMetricsItems :: MetricGrpCorrectedImpressions => "METRIC_GRP_CORRECTED_IMPRESSIONS" , ParametersMetricsItems :: MetricGrpCorrectedViewableImpressions => "METRIC_GRP_CORRECTED_VIEWABLE_IMPRESSIONS" , ParametersMetricsItems :: MetricGrpCorrectedViewableImpressionsSharePercent => "METRIC_GRP_CORRECTED_VIEWABLE_IMPRESSIONS_SHARE_PERCENT" , ParametersMetricsItems :: MetricImpressionCustomValueCost => "METRIC_IMPRESSION_CUSTOM_VALUE_COST" , ParametersMetricsItems :: MetricImpressionLossTargetedImpressions => "METRIC_IMPRESSION_LOSS_TARGETED_IMPRESSIONS" , ParametersMetricsItems :: MetricImpressions => "METRIC_IMPRESSIONS" , ParametersMetricsItems :: MetricImpressionsToConversionRate => "METRIC_IMPRESSIONS_TO_CONVERSION_RATE" , ParametersMetricsItems :: MetricImpressionsWithCustomValue => "METRIC_IMPRESSIONS_WITH_CUSTOM_VALUE" , ParametersMetricsItems :: MetricImpressionsWithPositiveCustomValue => "METRIC_IMPRESSIONS_WITH_POSITIVE_CUSTOM_VALUE" , ParametersMetricsItems :: MetricInteractiveImpressions => "METRIC_INTERACTIVE_IMPRESSIONS" , ParametersMetricsItems :: MetricInvalidActiveViewEligibleImpressions => "METRIC_INVALID_ACTIVE_VIEW_ELIGIBLE_IMPRESSIONS" , ParametersMetricsItems :: MetricInvalidActiveViewMeasurableImpressions => "METRIC_INVALID_ACTIVE_VIEW_MEASURABLE_IMPRESSIONS" , ParametersMetricsItems :: MetricInvalidActiveViewViewableImpressions => "METRIC_INVALID_ACTIVE_VIEW_VIEWABLE_IMPRESSIONS" , ParametersMetricsItems :: MetricInvalidBeginToRenderImpressions => "METRIC_INVALID_BEGIN_TO_RENDER_IMPRESSIONS" , ParametersMetricsItems :: MetricInvalidClicks => "METRIC_INVALID_CLICKS" , ParametersMetricsItems :: MetricInvalidImpressions => "METRIC_INVALID_IMPRESSIONS" , ParametersMetricsItems :: MetricInvalidTrackedAds => "METRIC_INVALID_TRACKED_ADS" , ParametersMetricsItems :: MetricLastClicks => "METRIC_LAST_CLICKS" , ParametersMetricsItems :: MetricLastImpressions => "METRIC_LAST_IMPRESSIONS" , ParametersMetricsItems :: MetricLastTouchClickThroughConversions => "METRIC_LAST_TOUCH_CLICK_THROUGH_CONVERSIONS" , ParametersMetricsItems :: MetricLastTouchTotalConversions => "METRIC_LAST_TOUCH_TOTAL_CONVERSIONS" , ParametersMetricsItems :: MetricLastTouchViewThroughConversions => "METRIC_LAST_TOUCH_VIEW_THROUGH_CONVERSIONS" , ParametersMetricsItems :: MetricLineitemBidResponseCount => "METRIC_LINEITEM_BID_RESPONSE_COUNT" , ParametersMetricsItems :: MetricMediaCostAdvertiser => "METRIC_MEDIA_COST_ADVERTISER" , ParametersMetricsItems :: MetricMediaCostEcpaAdvertiser => "METRIC_MEDIA_COST_ECPA_ADVERTISER" , ParametersMetricsItems :: MetricMediaCostEcpaPartner => "METRIC_MEDIA_COST_ECPA_PARTNER" , ParametersMetricsItems :: MetricMediaCostEcpaUsd => "METRIC_MEDIA_COST_ECPA_USD" , ParametersMetricsItems :: MetricMediaCostEcpapcAdvertiser => "METRIC_MEDIA_COST_ECPAPC_ADVERTISER" , ParametersMetricsItems :: MetricMediaCostEcpapcPartner => "METRIC_MEDIA_COST_ECPAPC_PARTNER" , ParametersMetricsItems :: MetricMediaCostEcpapcUsd => "METRIC_MEDIA_COST_ECPAPC_USD" , ParametersMetricsItems :: MetricMediaCostEcpapvAdvertiser => "METRIC_MEDIA_COST_ECPAPV_ADVERTISER" , ParametersMetricsItems :: MetricMediaCostEcpapvPartner => "METRIC_MEDIA_COST_ECPAPV_PARTNER" , ParametersMetricsItems :: MetricMediaCostEcpapvUsd => "METRIC_MEDIA_COST_ECPAPV_USD" , ParametersMetricsItems :: MetricMediaCostEcpcAdvertiser => "METRIC_MEDIA_COST_ECPC_ADVERTISER" , ParametersMetricsItems :: MetricMediaCostEcpcPartner => "METRIC_MEDIA_COST_ECPC_PARTNER" , ParametersMetricsItems :: MetricMediaCostEcpcUsd => "METRIC_MEDIA_COST_ECPC_USD" , ParametersMetricsItems :: MetricMediaCostEcpcvAdvertiser => "METRIC_MEDIA_COST_ECPCV_ADVERTISER" , ParametersMetricsItems :: MetricMediaCostEcpcvPartner => "METRIC_MEDIA_COST_ECPCV_PARTNER" , ParametersMetricsItems :: MetricMediaCostEcpcvUsd => "METRIC_MEDIA_COST_ECPCV_USD" , ParametersMetricsItems :: MetricMediaCostEcpmAdvertiser => "METRIC_MEDIA_COST_ECPM_ADVERTISER" , ParametersMetricsItems :: MetricMediaCostEcpmPartner => "METRIC_MEDIA_COST_ECPM_PARTNER" , ParametersMetricsItems :: MetricMediaCostEcpmUsd => "METRIC_MEDIA_COST_ECPM_USD" , ParametersMetricsItems :: MetricMediaCostPartner => "METRIC_MEDIA_COST_PARTNER" , ParametersMetricsItems :: MetricMediaCostUsd => "METRIC_MEDIA_COST_USD" , ParametersMetricsItems :: MetricMediaCostViewableEcpmAdvertiser => "METRIC_MEDIA_COST_VIEWABLE_ECPM_ADVERTISER" , ParametersMetricsItems :: MetricMediaCostViewableEcpmPartner => "METRIC_MEDIA_COST_VIEWABLE_ECPM_PARTNER" , ParametersMetricsItems :: MetricMediaCostViewableEcpmUsd => "METRIC_MEDIA_COST_VIEWABLE_ECPM_USD" , ParametersMetricsItems :: MetricMediaFee1Advertiser => "METRIC_MEDIA_FEE1_ADVERTISER" , ParametersMetricsItems :: MetricMediaFee1Partner => "METRIC_MEDIA_FEE1_PARTNER" , ParametersMetricsItems :: MetricMediaFee1Usd => "METRIC_MEDIA_FEE1_USD" , ParametersMetricsItems :: MetricMediaFee2Advertiser => "METRIC_MEDIA_FEE2_ADVERTISER" , ParametersMetricsItems :: MetricMediaFee2Partner => "METRIC_MEDIA_FEE2_PARTNER" , ParametersMetricsItems :: MetricMediaFee2Usd => "METRIC_MEDIA_FEE2_USD" , ParametersMetricsItems :: MetricMediaFee3Advertiser => "METRIC_MEDIA_FEE3_ADVERTISER" , ParametersMetricsItems :: MetricMediaFee3Partner => "METRIC_MEDIA_FEE3_PARTNER" , ParametersMetricsItems :: MetricMediaFee3Usd => "METRIC_MEDIA_FEE3_USD" , ParametersMetricsItems :: MetricMediaFee4Advertiser => "METRIC_MEDIA_FEE4_ADVERTISER" , ParametersMetricsItems :: MetricMediaFee4Partner => "METRIC_MEDIA_FEE4_PARTNER" , ParametersMetricsItems :: MetricMediaFee4Usd => "METRIC_MEDIA_FEE4_USD" , ParametersMetricsItems :: MetricMediaFee5Advertiser => "METRIC_MEDIA_FEE5_ADVERTISER" , ParametersMetricsItems :: MetricMediaFee5Partner => "METRIC_MEDIA_FEE5_PARTNER" , ParametersMetricsItems :: MetricMediaFee5Usd => "METRIC_MEDIA_FEE5_USD" , ParametersMetricsItems :: MetricMidpointAudio => "METRIC_MIDPOINT_AUDIO" , ParametersMetricsItems :: MetricNielsenAverageFrequency => "METRIC_NIELSEN_AVERAGE_FREQUENCY" , ParametersMetricsItems :: MetricNielsenGrp => "METRIC_NIELSEN_GRP" , ParametersMetricsItems :: MetricNielsenImpressionIndex => "METRIC_NIELSEN_IMPRESSION_INDEX" , ParametersMetricsItems :: MetricNielsenImpressions => "METRIC_NIELSEN_IMPRESSIONS" , ParametersMetricsItems :: MetricNielsenImpressionsShare => "METRIC_NIELSEN_IMPRESSIONS_SHARE" , ParametersMetricsItems :: MetricNielsenPopulation => "METRIC_NIELSEN_POPULATION" , ParametersMetricsItems :: MetricNielsenPopulationReach => "METRIC_NIELSEN_POPULATION_REACH" , ParametersMetricsItems :: MetricNielsenPopulationShare => "METRIC_NIELSEN_POPULATION_SHARE" , ParametersMetricsItems :: MetricNielsenReachIndex => "METRIC_NIELSEN_REACH_INDEX" , ParametersMetricsItems :: MetricNielsenReachShare => "METRIC_NIELSEN_REACH_SHARE" , ParametersMetricsItems :: MetricNielsenUniqueAudience => "METRIC_NIELSEN_UNIQUE_AUDIENCE" , ParametersMetricsItems :: MetricOriginalAudienceFrequency => "METRIC_ORIGINAL_AUDIENCE_FREQUENCY" , ParametersMetricsItems :: MetricPathConversionRate => "METRIC_PATH_CONVERSION_RATE" , ParametersMetricsItems :: MetricPausesAudio => "METRIC_PAUSES_AUDIO" , ParametersMetricsItems :: MetricPercentImpressionsWithPositiveCustomValue => "METRIC_PERCENT_IMPRESSIONS_WITH_POSITIVE_CUSTOM_VALUE" , ParametersMetricsItems :: MetricPercentInvalidImpressionsPrebid => "METRIC_PERCENT_INVALID_IMPRESSIONS_PREBID" , ParametersMetricsItems :: MetricPercentageFromCurrentIoGoal => "METRIC_PERCENTAGE_FROM_CURRENT_IO_GOAL" , ParametersMetricsItems :: MetricPlatformFeeAdvertiser => "METRIC_PLATFORM_FEE_ADVERTISER" , ParametersMetricsItems :: MetricPlatformFeePartner => "METRIC_PLATFORM_FEE_PARTNER" , ParametersMetricsItems :: MetricPlatformFeeRate => "METRIC_PLATFORM_FEE_RATE" , ParametersMetricsItems :: MetricPlatformFeeUsd => "METRIC_PLATFORM_FEE_USD" , ParametersMetricsItems :: MetricPostClickConversionsCrossEnvironment => "METRIC_POST_CLICK_CONVERSIONS_CROSS_ENVIRONMENT" , ParametersMetricsItems :: MetricPostViewConversionsCrossEnvironment => "METRIC_POST_VIEW_CONVERSIONS_CROSS_ENVIRONMENT" , ParametersMetricsItems :: MetricPotentialImpressions => "METRIC_POTENTIAL_IMPRESSIONS" , ParametersMetricsItems :: MetricPotentialViews => "METRIC_POTENTIAL_VIEWS" , ParametersMetricsItems :: MetricPremiumFeeAdvertiserCurrency => "METRIC_PREMIUM_FEE_ADVERTISER_CURRENCY" , ParametersMetricsItems :: MetricProfitAdvertiser => "METRIC_PROFIT_ADVERTISER" , ParametersMetricsItems :: MetricProfitEcpmAdvertiser => "METRIC_PROFIT_ECPM_ADVERTISER" , ParametersMetricsItems :: MetricProfitEcpmPartner => "METRIC_PROFIT_ECPM_PARTNER" , ParametersMetricsItems :: MetricProfitEcpmUsd => "METRIC_PROFIT_ECPM_USD" , ParametersMetricsItems :: MetricProfitMargin => "METRIC_PROFIT_MARGIN" , ParametersMetricsItems :: MetricProfitPartner => "METRIC_PROFIT_PARTNER" , ParametersMetricsItems :: MetricProfitUsd => "METRIC_PROFIT_USD" , ParametersMetricsItems :: MetricProfitViewableEcpmAdvertiser => "METRIC_PROFIT_VIEWABLE_ECPM_ADVERTISER" , ParametersMetricsItems :: MetricProfitViewableEcpmPartner => "METRIC_PROFIT_VIEWABLE_ECPM_PARTNER" , ParametersMetricsItems :: MetricProfitViewableEcpmUsd => "METRIC_PROFIT_VIEWABLE_ECPM_USD" , ParametersMetricsItems :: MetricProgrammaticGuaranteedImpressionsPassedDueToFrequency => "METRIC_PROGRAMMATIC_GUARANTEED_IMPRESSIONS_PASSED_DUE_TO_FREQUENCY" , ParametersMetricsItems :: MetricProgrammaticGuaranteedSavingsReInvestedDueToFrequencyAdvertiserCurrency => "METRIC_PROGRAMMATIC_GUARANTEED_SAVINGS_RE_INVESTED_DUE_TO_FREQUENCY_ADVERTISER_CURRENCY" , ParametersMetricsItems :: MetricProvisionalImpressions => "METRIC_PROVISIONAL_IMPRESSIONS" , ParametersMetricsItems :: MetricRefundBillableCostAdvertiserCurrency => "METRIC_REFUND_BILLABLE_COST_ADVERTISER_CURRENCY" , ParametersMetricsItems :: MetricRefundMediaCostAdvertiserCurrency => "METRIC_REFUND_MEDIA_COST_ADVERTISER_CURRENCY" , ParametersMetricsItems :: MetricRefundPlatformFeeAdvertiserCurrency => "METRIC_REFUND_PLATFORM_FEE_ADVERTISER_CURRENCY" , ParametersMetricsItems :: MetricRevenueAdvertiser => "METRIC_REVENUE_ADVERTISER" , ParametersMetricsItems :: MetricRevenueEcpaAdvertiser => "METRIC_REVENUE_ECPA_ADVERTISER" , ParametersMetricsItems :: MetricRevenueEcpaPartner => "METRIC_REVENUE_ECPA_PARTNER" , ParametersMetricsItems :: MetricRevenueEcpaUsd => "METRIC_REVENUE_ECPA_USD" , ParametersMetricsItems :: MetricRevenueEcpapcAdvertiser => "METRIC_REVENUE_ECPAPC_ADVERTISER" , ParametersMetricsItems :: MetricRevenueEcpapcPartner => "METRIC_REVENUE_ECPAPC_PARTNER" , ParametersMetricsItems :: MetricRevenueEcpapcUsd => "METRIC_REVENUE_ECPAPC_USD" , ParametersMetricsItems :: MetricRevenueEcpapvAdvertiser => "METRIC_REVENUE_ECPAPV_ADVERTISER" , ParametersMetricsItems :: MetricRevenueEcpapvPartner => "METRIC_REVENUE_ECPAPV_PARTNER" , ParametersMetricsItems :: MetricRevenueEcpapvUsd => "METRIC_REVENUE_ECPAPV_USD" , ParametersMetricsItems :: MetricRevenueEcpcAdvertiser => "METRIC_REVENUE_ECPC_ADVERTISER" , ParametersMetricsItems :: MetricRevenueEcpcPartner => "METRIC_REVENUE_ECPC_PARTNER" , ParametersMetricsItems :: MetricRevenueEcpcUsd => "METRIC_REVENUE_ECPC_USD" , ParametersMetricsItems :: MetricRevenueEcpcvAdvertiser => "METRIC_REVENUE_ECPCV_ADVERTISER" , ParametersMetricsItems :: MetricRevenueEcpcvPartner => "METRIC_REVENUE_ECPCV_PARTNER" , ParametersMetricsItems :: MetricRevenueEcpcvUsd => "METRIC_REVENUE_ECPCV_USD" , ParametersMetricsItems :: MetricRevenueEcpmAdvertiser => "METRIC_REVENUE_ECPM_ADVERTISER" , ParametersMetricsItems :: MetricRevenueEcpmPartner => "METRIC_REVENUE_ECPM_PARTNER" , ParametersMetricsItems :: MetricRevenueEcpmUsd => "METRIC_REVENUE_ECPM_USD" , ParametersMetricsItems :: MetricRevenuePartner => "METRIC_REVENUE_PARTNER" , ParametersMetricsItems :: MetricRevenueUsd => "METRIC_REVENUE_USD" , ParametersMetricsItems :: MetricRevenueViewableEcpmAdvertiser => "METRIC_REVENUE_VIEWABLE_ECPM_ADVERTISER" , ParametersMetricsItems :: MetricRevenueViewableEcpmPartner => "METRIC_REVENUE_VIEWABLE_ECPM_PARTNER" , ParametersMetricsItems :: MetricRevenueViewableEcpmUsd => "METRIC_REVENUE_VIEWABLE_ECPM_USD" , ParametersMetricsItems :: MetricRichMediaEngagements => "METRIC_RICH_MEDIA_ENGAGEMENTS" , ParametersMetricsItems :: MetricRichMediaScrolls => "METRIC_RICH_MEDIA_SCROLLS" , ParametersMetricsItems :: MetricRichMediaVideoCompletions => "METRIC_RICH_MEDIA_VIDEO_COMPLETIONS" , ParametersMetricsItems :: MetricRichMediaVideoFirstQuartileCompletes => "METRIC_RICH_MEDIA_VIDEO_FIRST_QUARTILE_COMPLETES" , ParametersMetricsItems :: MetricRichMediaVideoFullScreens => "METRIC_RICH_MEDIA_VIDEO_FULL_SCREENS" , ParametersMetricsItems :: MetricRichMediaVideoMidpoints => "METRIC_RICH_MEDIA_VIDEO_MIDPOINTS" , ParametersMetricsItems :: MetricRichMediaVideoMutes => "METRIC_RICH_MEDIA_VIDEO_MUTES" , ParametersMetricsItems :: MetricRichMediaVideoPauses => "METRIC_RICH_MEDIA_VIDEO_PAUSES" , ParametersMetricsItems :: MetricRichMediaVideoPlays => "METRIC_RICH_MEDIA_VIDEO_PLAYS" , ParametersMetricsItems :: MetricRichMediaVideoSkips => "METRIC_RICH_MEDIA_VIDEO_SKIPS" , ParametersMetricsItems :: MetricRichMediaVideoThirdQuartileCompletes => "METRIC_RICH_MEDIA_VIDEO_THIRD_QUARTILE_COMPLETES" , ParametersMetricsItems :: MetricStartsAudio => "METRIC_STARTS_AUDIO" , ParametersMetricsItems :: MetricStopsAudio => "METRIC_STOPS_AUDIO" , ParametersMetricsItems :: MetricStoreVisitConversions => "METRIC_STORE_VISIT_CONVERSIONS" , ParametersMetricsItems :: MetricTargetRatingPoints => "METRIC_TARGET_RATING_POINTS" , ParametersMetricsItems :: MetricTeaTrueviewImpressions => "METRIC_TEA_TRUEVIEW_IMPRESSIONS" , ParametersMetricsItems :: MetricTeaTrueviewUniqueCookies => "METRIC_TEA_TRUEVIEW_UNIQUE_COOKIES" , ParametersMetricsItems :: MetricThirdQuartileAudio => "METRIC_THIRD_QUARTILE_AUDIO" , ParametersMetricsItems :: MetricTimers => "METRIC_TIMERS" , ParametersMetricsItems :: MetricTotalAudioMediaCostEcpclAdvertiserCurrency => "METRIC_TOTAL_AUDIO_MEDIA_COST_ECPCL_ADVERTISER_CURRENCY" , ParametersMetricsItems :: MetricTotalConversions => "METRIC_TOTAL_CONVERSIONS" , ParametersMetricsItems :: MetricTotalConversionsCrossEnvironment => "METRIC_TOTAL_CONVERSIONS_CROSS_ENVIRONMENT" , ParametersMetricsItems :: MetricTotalDisplayTime => "METRIC_TOTAL_DISPLAY_TIME" , ParametersMetricsItems :: MetricTotalExposures => "METRIC_TOTAL_EXPOSURES" , ParametersMetricsItems :: MetricTotalImpressionCustomValue => "METRIC_TOTAL_IMPRESSION_CUSTOM_VALUE" , ParametersMetricsItems :: MetricTotalInteractionTime => "METRIC_TOTAL_INTERACTION_TIME" , ParametersMetricsItems :: MetricTotalMediaCostAdvertiser => "METRIC_TOTAL_MEDIA_COST_ADVERTISER" , ParametersMetricsItems :: MetricTotalMediaCostEcpaAdvertiser => "METRIC_TOTAL_MEDIA_COST_ECPA_ADVERTISER" , ParametersMetricsItems :: MetricTotalMediaCostEcpaPartner => "METRIC_TOTAL_MEDIA_COST_ECPA_PARTNER" , ParametersMetricsItems :: MetricTotalMediaCostEcpaUsd => "METRIC_TOTAL_MEDIA_COST_ECPA_USD" , ParametersMetricsItems :: MetricTotalMediaCostEcpapcAdvertiser => "METRIC_TOTAL_MEDIA_COST_ECPAPC_ADVERTISER" , ParametersMetricsItems :: MetricTotalMediaCostEcpapcPartner => "METRIC_TOTAL_MEDIA_COST_ECPAPC_PARTNER" , ParametersMetricsItems :: MetricTotalMediaCostEcpapcUsd => "METRIC_TOTAL_MEDIA_COST_ECPAPC_USD" , ParametersMetricsItems :: MetricTotalMediaCostEcpapvAdvertiser => "METRIC_TOTAL_MEDIA_COST_ECPAPV_ADVERTISER" , ParametersMetricsItems :: MetricTotalMediaCostEcpapvPartner => "METRIC_TOTAL_MEDIA_COST_ECPAPV_PARTNER" , ParametersMetricsItems :: MetricTotalMediaCostEcpapvUsd => "METRIC_TOTAL_MEDIA_COST_ECPAPV_USD" , ParametersMetricsItems :: MetricTotalMediaCostEcpcAdvertiser => "METRIC_TOTAL_MEDIA_COST_ECPC_ADVERTISER" , ParametersMetricsItems :: MetricTotalMediaCostEcpcPartner => "METRIC_TOTAL_MEDIA_COST_ECPC_PARTNER" , ParametersMetricsItems :: MetricTotalMediaCostEcpcUsd => "METRIC_TOTAL_MEDIA_COST_ECPC_USD" , ParametersMetricsItems :: MetricTotalMediaCostEcpcvAdvertiser => "METRIC_TOTAL_MEDIA_COST_ECPCV_ADVERTISER" , ParametersMetricsItems :: MetricTotalMediaCostEcpcvPartner => "METRIC_TOTAL_MEDIA_COST_ECPCV_PARTNER" , ParametersMetricsItems :: MetricTotalMediaCostEcpcvUsd => "METRIC_TOTAL_MEDIA_COST_ECPCV_USD" , ParametersMetricsItems :: MetricTotalMediaCostEcpmAdvertiser => "METRIC_TOTAL_MEDIA_COST_ECPM_ADVERTISER" , ParametersMetricsItems :: MetricTotalMediaCostEcpmPartner => "METRIC_TOTAL_MEDIA_COST_ECPM_PARTNER" , ParametersMetricsItems :: MetricTotalMediaCostEcpmUsd => "METRIC_TOTAL_MEDIA_COST_ECPM_USD" , ParametersMetricsItems :: MetricTotalMediaCostPartner => "METRIC_TOTAL_MEDIA_COST_PARTNER" , ParametersMetricsItems :: MetricTotalMediaCostUsd => "METRIC_TOTAL_MEDIA_COST_USD" , ParametersMetricsItems :: MetricTotalMediaCostViewableEcpmAdvertiser => "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_ADVERTISER" , ParametersMetricsItems :: MetricTotalMediaCostViewableEcpmPartner => "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_PARTNER" , ParametersMetricsItems :: MetricTotalMediaCostViewableEcpmUsd => "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_USD" , ParametersMetricsItems :: MetricTotalPaths => "METRIC_TOTAL_PATHS" , ParametersMetricsItems :: MetricTotalUsers => "METRIC_TOTAL_USERS" , ParametersMetricsItems :: MetricTrackedAds => "METRIC_TRACKED_ADS" , ParametersMetricsItems :: MetricTrackingUnconsentedClicks => "METRIC_TRACKING_UNCONSENTED_CLICKS" , ParametersMetricsItems :: MetricTrueviewAllAdSequenceImpressions => "METRIC_TRUEVIEW_ALL_AD_SEQUENCE_IMPRESSIONS" , ParametersMetricsItems :: MetricTrueviewAverageCpeAdvertiser => "METRIC_TRUEVIEW_AVERAGE_CPE_ADVERTISER" , ParametersMetricsItems :: MetricTrueviewAverageCpePartner => "METRIC_TRUEVIEW_AVERAGE_CPE_PARTNER" , ParametersMetricsItems :: MetricTrueviewAverageCpeUsd => "METRIC_TRUEVIEW_AVERAGE_CPE_USD" , ParametersMetricsItems :: MetricTrueviewConversionCostManyPerViewAdvertiser => "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_ADVERTISER" , ParametersMetricsItems :: MetricTrueviewConversionCostManyPerViewPartner => "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_PARTNER" , ParametersMetricsItems :: MetricTrueviewConversionCostManyPerViewUsd => "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_USD" , ParametersMetricsItems :: MetricTrueviewConversionManyPerView => "METRIC_TRUEVIEW_CONVERSION_MANY_PER_VIEW" , ParametersMetricsItems :: MetricTrueviewConversionRateOnePerView => "METRIC_TRUEVIEW_CONVERSION_RATE_ONE_PER_VIEW" , ParametersMetricsItems :: MetricTrueviewCpvAdvertiser => "METRIC_TRUEVIEW_CPV_ADVERTISER" , ParametersMetricsItems :: MetricTrueviewCpvPartner => "METRIC_TRUEVIEW_CPV_PARTNER" , ParametersMetricsItems :: MetricTrueviewCpvUsd => "METRIC_TRUEVIEW_CPV_USD" , ParametersMetricsItems :: MetricTrueviewEarnedLikes => "METRIC_TRUEVIEW_EARNED_LIKES" , ParametersMetricsItems :: MetricTrueviewEarnedPlaylistAdditions => "METRIC_TRUEVIEW_EARNED_PLAYLIST_ADDITIONS" , ParametersMetricsItems :: MetricTrueviewEarnedShares => "METRIC_TRUEVIEW_EARNED_SHARES" , ParametersMetricsItems :: MetricTrueviewEarnedSubscribers => "METRIC_TRUEVIEW_EARNED_SUBSCRIBERS" , ParametersMetricsItems :: MetricTrueviewEarnedViews => "METRIC_TRUEVIEW_EARNED_VIEWS" , ParametersMetricsItems :: MetricTrueviewEngagementRate => "METRIC_TRUEVIEW_ENGAGEMENT_RATE" , ParametersMetricsItems :: MetricTrueviewEngagements => "METRIC_TRUEVIEW_ENGAGEMENTS" , ParametersMetricsItems :: MetricTrueviewGeneralInvalidTrafficGivtViews => "METRIC_TRUEVIEW_GENERAL_INVALID_TRAFFIC_GIVT_VIEWS" , ParametersMetricsItems :: MetricTrueviewImpressionShare => "METRIC_TRUEVIEW_IMPRESSION_SHARE" , ParametersMetricsItems :: MetricTrueviewInvalidViews => "METRIC_TRUEVIEW_INVALID_VIEWS" , ParametersMetricsItems :: MetricTrueviewLostIsBudget => "METRIC_TRUEVIEW_LOST_IS_BUDGET" , ParametersMetricsItems :: MetricTrueviewLostIsRank => "METRIC_TRUEVIEW_LOST_IS_RANK" , ParametersMetricsItems :: MetricTrueviewTotalConversionValuesAdvertiser => "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_ADVERTISER" , ParametersMetricsItems :: MetricTrueviewTotalConversionValuesPartner => "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_PARTNER" , ParametersMetricsItems :: MetricTrueviewTotalConversionValuesUsd => "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_USD" , ParametersMetricsItems :: MetricTrueviewUniqueViewers => "METRIC_TRUEVIEW_UNIQUE_VIEWERS" , ParametersMetricsItems :: MetricTrueviewViewRate => "METRIC_TRUEVIEW_VIEW_RATE" , ParametersMetricsItems :: MetricTrueviewViewThroughConversion => "METRIC_TRUEVIEW_VIEW_THROUGH_CONVERSION" , ParametersMetricsItems :: MetricTrueviewViews => "METRIC_TRUEVIEW_VIEWS" , ParametersMetricsItems :: MetricUniqueCookiesWithImpressions => "METRIC_UNIQUE_COOKIES_WITH_IMPRESSIONS" , ParametersMetricsItems :: MetricUniqueReachAverageImpressionFrequency => "METRIC_UNIQUE_REACH_AVERAGE_IMPRESSION_FREQUENCY" , ParametersMetricsItems :: MetricUniqueReachClickReach => "METRIC_UNIQUE_REACH_CLICK_REACH" , ParametersMetricsItems :: MetricUniqueReachImpressionReach => "METRIC_UNIQUE_REACH_IMPRESSION_REACH" , ParametersMetricsItems :: MetricUniqueReachTotalReach => "METRIC_UNIQUE_REACH_TOTAL_REACH" , ParametersMetricsItems :: MetricUniqueVisitorsCookies => "METRIC_UNIQUE_VISITORS_COOKIES" , ParametersMetricsItems :: MetricUnknown => "METRIC_UNKNOWN" , ParametersMetricsItems :: MetricVendorBlockedAds => "METRIC_VENDOR_BLOCKED_ADS" , ParametersMetricsItems :: MetricVerifiableImpressions => "METRIC_VERIFIABLE_IMPRESSIONS" , ParametersMetricsItems :: MetricVerificationVideoPlayerSizeMeasurableImpressions => "METRIC_VERIFICATION_VIDEO_PLAYER_SIZE_MEASURABLE_IMPRESSIONS" , ParametersMetricsItems :: MetricVideoClientCostEcpcvAdvertiserCurrency => "METRIC_VIDEO_CLIENT_COST_ECPCV_ADVERTISER_CURRENCY" , ParametersMetricsItems :: MetricVideoCompanionClicks => "METRIC_VIDEO_COMPANION_CLICKS" , ParametersMetricsItems :: MetricVideoCompanionImpressions => "METRIC_VIDEO_COMPANION_IMPRESSIONS" , ParametersMetricsItems :: MetricVideoCompletionRate => "METRIC_VIDEO_COMPLETION_RATE" , ParametersMetricsItems :: MetricViewableBidRequests => "METRIC_VIEWABLE_BID_REQUESTS" , ParametersMetricsItems :: MetricViewableGrossRatingPoints => "METRIC_VIEWABLE_GROSS_RATING_POINTS" , ParametersMetricsItems :: MetricVirtualPeopleAverageImpressionFrequencyByDemo => "METRIC_VIRTUAL_PEOPLE_AVERAGE_IMPRESSION_FREQUENCY_BY_DEMO" , ParametersMetricsItems :: MetricVirtualPeopleAverageViewableImpressionFrequencyByDemo => "METRIC_VIRTUAL_PEOPLE_AVERAGE_VIEWABLE_IMPRESSION_FREQUENCY_BY_DEMO" , ParametersMetricsItems :: MetricVirtualPeopleClickReachByDemo => "METRIC_VIRTUAL_PEOPLE_CLICK_REACH_BY_DEMO" , ParametersMetricsItems :: MetricVirtualPeopleImpressionReachByDemo => "METRIC_VIRTUAL_PEOPLE_IMPRESSION_REACH_BY_DEMO" , ParametersMetricsItems :: MetricVirtualPeopleImpressionReachPercent => "METRIC_VIRTUAL_PEOPLE_IMPRESSION_REACH_PERCENT" , ParametersMetricsItems :: MetricVirtualPeopleImpressionReachSharePercent => "METRIC_VIRTUAL_PEOPLE_IMPRESSION_REACH_SHARE_PERCENT" , ParametersMetricsItems :: MetricVirtualPeopleViewableImpressionReachByDemo => "METRIC_VIRTUAL_PEOPLE_VIEWABLE_IMPRESSION_REACH_BY_DEMO" , ParametersMetricsItems :: MetricVirtualPeopleViewableImpressionReachPercent => "METRIC_VIRTUAL_PEOPLE_VIEWABLE_IMPRESSION_REACH_PERCENT" , ParametersMetricsItems :: MetricVirtualPeopleViewableImpressionReachSharePercent => "METRIC_VIRTUAL_PEOPLE_VIEWABLE_IMPRESSION_REACH_SHARE_PERCENT" , ParametersMetricsItems :: MetricWatchTime => "METRIC_WATCH_TIME" , ParametersMetricsItems :: MetricWinLossDealAvailableRequests => "METRIC_WIN_LOSS_DEAL_AVAILABLE_REQUESTS" , ParametersMetricsItems :: MetricWinLossDealTargetedImpressions => "METRIC_WIN_LOSS_DEAL_TARGETED_IMPRESSIONS" , ParametersMetricsItems :: MetricWinLossLineitemAvailableRequests => "METRIC_WIN_LOSS_LINEITEM_AVAILABLE_REQUESTS" , ParametersMetricsItems :: MetricWinLossLineitemTargetedImpressions => "METRIC_WIN_LOSS_LINEITEM_TARGETED_IMPRESSIONS" , ParametersMetricsItems :: MetricWinLossRate => "METRIC_WIN_LOSS_RATE" , }
        }
    }
    impl ::std::convert::AsRef<str> for ParametersMetricsItems {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ParametersMetricsItems {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ParametersMetricsItems, ()> {
            Ok (match s { "METRIC_ACTIVE_VIEW_AUDIBLE_FULLY_ON_SCREEN_HALF_OF_DURATION_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewAudibleFullyOnScreenHalfOfDurationImpressions , "METRIC_ACTIVE_VIEW_AUDIBLE_FULLY_ON_SCREEN_HALF_OF_DURATION_MEASURABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewAudibleFullyOnScreenHalfOfDurationMeasurableImpressions , "METRIC_ACTIVE_VIEW_AUDIBLE_FULLY_ON_SCREEN_HALF_OF_DURATION_RATE" => ParametersMetricsItems :: MetricActiveViewAudibleFullyOnScreenHalfOfDurationRate , "METRIC_ACTIVE_VIEW_AUDIBLE_FULLY_ON_SCREEN_HALF_OF_DURATION_TRUEVIEW_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewAudibleFullyOnScreenHalfOfDurationTrueviewImpressions , "METRIC_ACTIVE_VIEW_AUDIBLE_FULLY_ON_SCREEN_HALF_OF_DURATION_TRUEVIEW_MEASURABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewAudibleFullyOnScreenHalfOfDurationTrueviewMeasurableImpressions , "METRIC_ACTIVE_VIEW_AUDIBLE_FULLY_ON_SCREEN_HALF_OF_DURATION_TRUEVIEW_RATE" => ParametersMetricsItems :: MetricActiveViewAudibleFullyOnScreenHalfOfDurationTrueviewRate , "METRIC_ACTIVE_VIEW_AUDIBLE_VISIBLE_ON_COMPLETE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewAudibleVisibleOnCompleteImpressions , "METRIC_ACTIVE_VIEW_AVERAGE_VIEWABLE_TIME" => ParametersMetricsItems :: MetricActiveViewAverageViewableTime , "METRIC_ACTIVE_VIEW_CUSTOM_METRIC_MEASURABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewCustomMetricMeasurableImpressions , "METRIC_ACTIVE_VIEW_CUSTOM_METRIC_VIEWABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewCustomMetricViewableImpressions , "METRIC_ACTIVE_VIEW_CUSTOM_METRIC_VIEWABLE_RATE" => ParametersMetricsItems :: MetricActiveViewCustomMetricViewableRate , "METRIC_ACTIVE_VIEW_DISTRIBUTION_UNMEASURABLE" => ParametersMetricsItems :: MetricActiveViewDistributionUnmeasurable , "METRIC_ACTIVE_VIEW_DISTRIBUTION_UNVIEWABLE" => ParametersMetricsItems :: MetricActiveViewDistributionUnviewable , "METRIC_ACTIVE_VIEW_DISTRIBUTION_VIEWABLE" => ParametersMetricsItems :: MetricActiveViewDistributionViewable , "METRIC_ACTIVE_VIEW_ELIGIBLE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewEligibleImpressions , "METRIC_ACTIVE_VIEW_MEASURABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewMeasurableImpressions , "METRIC_ACTIVE_VIEW_PCT_MEASURABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewPctMeasurableImpressions , "METRIC_ACTIVE_VIEW_PCT_VIEWABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewPctViewableImpressions , "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewPercentAudibleImpressions , "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_AT_START" => ParametersMetricsItems :: MetricActiveViewPercentAudibleVisibleAtStart , "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_FIRST_QUAR" => ParametersMetricsItems :: MetricActiveViewPercentAudibleVisibleFirstQuar , "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_ON_COMPLETE" => ParametersMetricsItems :: MetricActiveViewPercentAudibleVisibleOnComplete , "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_SECOND_QUAR" => ParametersMetricsItems :: MetricActiveViewPercentAudibleVisibleSecondQuar , "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_THIRD_QUAR" => ParametersMetricsItems :: MetricActiveViewPercentAudibleVisibleThirdQuar , "METRIC_ACTIVE_VIEW_PERCENT_FULL_SCREEN" => ParametersMetricsItems :: MetricActiveViewPercentFullScreen , "METRIC_ACTIVE_VIEW_PERCENT_FULLY_ON_SCREEN_2_SEC" => ParametersMetricsItems :: MetricActiveViewPercentFullyOnScreen2Sec , "METRIC_ACTIVE_VIEW_PERCENT_IN_BACKGROUND" => ParametersMetricsItems :: MetricActiveViewPercentInBackground , "METRIC_ACTIVE_VIEW_PERCENT_OF_AD_PLAYED" => ParametersMetricsItems :: MetricActiveViewPercentOfAdPlayed , "METRIC_ACTIVE_VIEW_PERCENT_OF_COMPLETED_IMPRESSIONS_AUDIBLE_AND_VISIBLE" => ParametersMetricsItems :: MetricActiveViewPercentOfCompletedImpressionsAudibleAndVisible , "METRIC_ACTIVE_VIEW_PERCENT_OF_COMPLETED_IMPRESSIONS_VISIBLE" => ParametersMetricsItems :: MetricActiveViewPercentOfCompletedImpressionsVisible , "METRIC_ACTIVE_VIEW_PERCENT_OF_FIRST_QUARTILE_IMPRESSIONS_AUDIBLE_AND_VISIBLE" => ParametersMetricsItems :: MetricActiveViewPercentOfFirstQuartileImpressionsAudibleAndVisible , "METRIC_ACTIVE_VIEW_PERCENT_OF_FIRST_QUARTILE_IMPRESSIONS_VISIBLE" => ParametersMetricsItems :: MetricActiveViewPercentOfFirstQuartileImpressionsVisible , "METRIC_ACTIVE_VIEW_PERCENT_OF_MIDPOINT_IMPRESSIONS_AUDIBLE_AND_VISIBLE" => ParametersMetricsItems :: MetricActiveViewPercentOfMidpointImpressionsAudibleAndVisible , "METRIC_ACTIVE_VIEW_PERCENT_OF_MIDPOINT_IMPRESSIONS_VISIBLE" => ParametersMetricsItems :: MetricActiveViewPercentOfMidpointImpressionsVisible , "METRIC_ACTIVE_VIEW_PERCENT_OF_THIRD_QUARTILE_IMPRESSIONS_AUDIBLE_AND_VISIBLE" => ParametersMetricsItems :: MetricActiveViewPercentOfThirdQuartileImpressionsAudibleAndVisible , "METRIC_ACTIVE_VIEW_PERCENT_OF_THIRD_QUARTILE_IMPRESSIONS_VISIBLE" => ParametersMetricsItems :: MetricActiveViewPercentOfThirdQuartileImpressionsVisible , "METRIC_ACTIVE_VIEW_PERCENT_PLAY_TIME_AUDIBLE" => ParametersMetricsItems :: MetricActiveViewPercentPlayTimeAudible , "METRIC_ACTIVE_VIEW_PERCENT_PLAY_TIME_AUDIBLE_AND_VISIBLE" => ParametersMetricsItems :: MetricActiveViewPercentPlayTimeAudibleAndVisible , "METRIC_ACTIVE_VIEW_PERCENT_PLAY_TIME_VISIBLE" => ParametersMetricsItems :: MetricActiveViewPercentPlayTimeVisible , "METRIC_ACTIVE_VIEW_PERCENT_VIEWABLE_FOR_TIME_THRESHOLD" => ParametersMetricsItems :: MetricActiveViewPercentViewableForTimeThreshold , "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_AT_START" => ParametersMetricsItems :: MetricActiveViewPercentVisibleAtStart , "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_FIRST_QUAR" => ParametersMetricsItems :: MetricActiveViewPercentVisibleFirstQuar , "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_ON_COMPLETE" => ParametersMetricsItems :: MetricActiveViewPercentVisibleOnComplete , "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_SECOND_QUAR" => ParametersMetricsItems :: MetricActiveViewPercentVisibleSecondQuar , "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_THIRD_QUAR" => ParametersMetricsItems :: MetricActiveViewPercentVisibleThirdQuar , "METRIC_ACTIVE_VIEW_UNMEASURABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewUnmeasurableImpressions , "METRIC_ACTIVE_VIEW_UNVIEWABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewUnviewableImpressions , "METRIC_ACTIVE_VIEW_VIEWABLE_FOR_TIME_THRESHOLD" => ParametersMetricsItems :: MetricActiveViewViewableForTimeThreshold , "METRIC_ACTIVE_VIEW_VIEWABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewViewableImpressions , "METRIC_ACTIVITY_REVENUE" => ParametersMetricsItems :: MetricActivityRevenue , "METRIC_ADAPTED_AUDIENCE_FREQUENCY" => ParametersMetricsItems :: MetricAdaptedAudienceFrequency , "METRIC_ADLINGO_FEE_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricAdlingoFeeAdvertiserCurrency , "METRIC_AUDIO_CLIENT_COST_ECPCL_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricAudioClientCostEcpclAdvertiserCurrency , "METRIC_AUDIO_MEDIA_COST_ECPCL_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricAudioMediaCostEcpclAdvertiserCurrency , "METRIC_AUDIO_MUTES_AUDIO" => ParametersMetricsItems :: MetricAudioMutesAudio , "METRIC_AUDIO_REVENUE_ECPCL_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricAudioRevenueEcpclAdvertiserCurrency , "METRIC_AUDIO_UNMUTES_AUDIO" => ParametersMetricsItems :: MetricAudioUnmutesAudio , "METRIC_AUDIO_UNMUTES_VIDEO" => ParametersMetricsItems :: MetricAudioUnmutesVideo , "METRIC_AVERAGE_DISPLAY_TIME" => ParametersMetricsItems :: MetricAverageDisplayTime , "METRIC_AVERAGE_IMPRESSION_FREQUENCY_PER_USER" => ParametersMetricsItems :: MetricAverageImpressionFrequencyPerUser , "METRIC_AVERAGE_INTERACTION_TIME" => ParametersMetricsItems :: MetricAverageInteractionTime , "METRIC_AVERAGE_WATCH_TIME_PER_IMPRESSION" => ParametersMetricsItems :: MetricAverageWatchTimePerImpression , "METRIC_BEGIN_TO_RENDER_ELIGIBLE_IMPRESSIONS" => ParametersMetricsItems :: MetricBeginToRenderEligibleImpressions , "METRIC_BEGIN_TO_RENDER_IMPRESSIONS" => ParametersMetricsItems :: MetricBeginToRenderImpressions , "METRIC_BENCHMARK_FREQUENCY" => ParametersMetricsItems :: MetricBenchmarkFrequency , "METRIC_BID_REQUESTS" => ParametersMetricsItems :: MetricBidRequests , "METRIC_BILLABLE_COST_ADVERTISER" => ParametersMetricsItems :: MetricBillableCostAdvertiser , "METRIC_BILLABLE_COST_PARTNER" => ParametersMetricsItems :: MetricBillableCostPartner , "METRIC_BILLABLE_COST_USD" => ParametersMetricsItems :: MetricBillableCostUsd , "METRIC_BILLABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricBillableImpressions , "METRIC_BRAND_LIFT_ABSOLUTE_BRAND_LIFT" => ParametersMetricsItems :: MetricBrandLiftAbsoluteBrandLift , "METRIC_BRAND_LIFT_ALL_SURVEY_RESPONSES" => ParametersMetricsItems :: MetricBrandLiftAllSurveyResponses , "METRIC_BRAND_LIFT_BASELINE_POSITIVE_RESPONSE_RATE" => ParametersMetricsItems :: MetricBrandLiftBaselinePositiveResponseRate , "METRIC_BRAND_LIFT_BASELINE_SURVEY_RESPONSES" => ParametersMetricsItems :: MetricBrandLiftBaselineSurveyResponses , "METRIC_BRAND_LIFT_COST_PER_LIFTED_USER" => ParametersMetricsItems :: MetricBrandLiftCostPerLiftedUser , "METRIC_BRAND_LIFT_EXPOSED_SURVEY_RESPONSES" => ParametersMetricsItems :: MetricBrandLiftExposedSurveyResponses , "METRIC_BRAND_LIFT_HEADROOM_BRAND_LIFT" => ParametersMetricsItems :: MetricBrandLiftHeadroomBrandLift , "METRIC_BRAND_LIFT_RELATIVE_BRAND_LIFT" => ParametersMetricsItems :: MetricBrandLiftRelativeBrandLift , "METRIC_BRAND_LIFT_USERS" => ParametersMetricsItems :: MetricBrandLiftUsers , "METRIC_CARD_CLICKS" => ParametersMetricsItems :: MetricCardClicks , "METRIC_CLICK_TO_POST_CLICK_CONVERSION_RATE" => ParametersMetricsItems :: MetricClickToPostClickConversionRate , "METRIC_CLICKS" => ParametersMetricsItems :: MetricClicks , "METRIC_CLIENT_COST_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricClientCostAdvertiserCurrency , "METRIC_CLIENT_COST_ECPA_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricClientCostEcpaAdvertiserCurrency , "METRIC_CLIENT_COST_ECPA_PC_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricClientCostEcpaPcAdvertiserCurrency , "METRIC_CLIENT_COST_ECPA_PV_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricClientCostEcpaPvAdvertiserCurrency , "METRIC_CLIENT_COST_ECPC_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricClientCostEcpcAdvertiserCurrency , "METRIC_CLIENT_COST_ECPM_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricClientCostEcpmAdvertiserCurrency , "METRIC_CLIENT_COST_VIEWABLE_ECPM_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricClientCostViewableEcpmAdvertiserCurrency , "METRIC_CM360_POST_CLICK_REVENUE" => ParametersMetricsItems :: MetricCm360PostClickRevenue , "METRIC_CM360_POST_CLICK_REVENUE_CROSS_ENVIRONMENT" => ParametersMetricsItems :: MetricCm360PostClickRevenueCrossEnvironment , "METRIC_CM360_POST_VIEW_REVENUE" => ParametersMetricsItems :: MetricCm360PostViewRevenue , "METRIC_CM360_POST_VIEW_REVENUE_CROSS_ENVIRONMENT" => ParametersMetricsItems :: MetricCm360PostViewRevenueCrossEnvironment , "METRIC_CM_POST_CLICK_REVENUE" => ParametersMetricsItems :: MetricCmPostClickRevenue , "METRIC_CM_POST_CLICK_REVENUE_CROSS_ENVIRONMENT" => ParametersMetricsItems :: MetricCmPostClickRevenueCrossEnvironment , "METRIC_CM_POST_VIEW_REVENUE" => ParametersMetricsItems :: MetricCmPostViewRevenue , "METRIC_CM_POST_VIEW_REVENUE_CROSS_ENVIRONMENT" => ParametersMetricsItems :: MetricCmPostViewRevenueCrossEnvironment , "METRIC_COMPANION_CLICKS_AUDIO" => ParametersMetricsItems :: MetricCompanionClicksAudio , "METRIC_COMPANION_IMPRESSIONS_AUDIO" => ParametersMetricsItems :: MetricCompanionImpressionsAudio , "METRIC_COMPLETE_LISTENS_AUDIO" => ParametersMetricsItems :: MetricCompleteListensAudio , "METRIC_COMPLETION_RATE_AUDIO" => ParametersMetricsItems :: MetricCompletionRateAudio , "METRIC_CONVERSIONS_PER_MILLE" => ParametersMetricsItems :: MetricConversionsPerMille , "METRIC_CONVERTING_PATHS" => ParametersMetricsItems :: MetricConvertingPaths , "METRIC_COOKIE_CONSENTED_FLOODLIGHT_IMPRESSIONS" => ParametersMetricsItems :: MetricCookieConsentedFloodlightImpressions , "METRIC_COOKIE_REACH_AVERAGE_IMPRESSION_FREQUENCY" => ParametersMetricsItems :: MetricCookieReachAverageImpressionFrequency , "METRIC_COOKIE_REACH_IMPRESSION_REACH" => ParametersMetricsItems :: MetricCookieReachImpressionReach , "METRIC_COOKIE_UNCONSENTED_FLOODLIGHT_IMPRESSIONS" => ParametersMetricsItems :: MetricCookieUnconsentedFloodlightImpressions , "METRIC_COUNTERS" => ParametersMetricsItems :: MetricCounters , "METRIC_CPM_FEE1_ADVERTISER" => ParametersMetricsItems :: MetricCpmFee1Advertiser , "METRIC_CPM_FEE1_PARTNER" => ParametersMetricsItems :: MetricCpmFee1Partner , "METRIC_CPM_FEE1_USD" => ParametersMetricsItems :: MetricCpmFee1Usd , "METRIC_CPM_FEE2_ADVERTISER" => ParametersMetricsItems :: MetricCpmFee2Advertiser , "METRIC_CPM_FEE2_PARTNER" => ParametersMetricsItems :: MetricCpmFee2Partner , "METRIC_CPM_FEE2_USD" => ParametersMetricsItems :: MetricCpmFee2Usd , "METRIC_CPM_FEE3_ADVERTISER" => ParametersMetricsItems :: MetricCpmFee3Advertiser , "METRIC_CPM_FEE3_PARTNER" => ParametersMetricsItems :: MetricCpmFee3Partner , "METRIC_CPM_FEE3_USD" => ParametersMetricsItems :: MetricCpmFee3Usd , "METRIC_CPM_FEE4_ADVERTISER" => ParametersMetricsItems :: MetricCpmFee4Advertiser , "METRIC_CPM_FEE4_PARTNER" => ParametersMetricsItems :: MetricCpmFee4Partner , "METRIC_CPM_FEE4_USD" => ParametersMetricsItems :: MetricCpmFee4Usd , "METRIC_CPM_FEE5_ADVERTISER" => ParametersMetricsItems :: MetricCpmFee5Advertiser , "METRIC_CPM_FEE5_PARTNER" => ParametersMetricsItems :: MetricCpmFee5Partner , "METRIC_CPM_FEE5_USD" => ParametersMetricsItems :: MetricCpmFee5Usd , "METRIC_CTR" => ParametersMetricsItems :: MetricCtr , "METRIC_CUSTOM_FEE_1_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricCustomFee1AdvertiserCurrency , "METRIC_CUSTOM_FEE_2_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricCustomFee2AdvertiserCurrency , "METRIC_CUSTOM_FEE_3_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricCustomFee3AdvertiserCurrency , "METRIC_CUSTOM_FEE_4_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricCustomFee4AdvertiserCurrency , "METRIC_CUSTOM_FEE_5_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricCustomFee5AdvertiserCurrency , "METRIC_CUSTOM_VALUE_PER_1000_IMPRESSIONS" => ParametersMetricsItems :: MetricCustomValuePer1000Impressions , "METRIC_DATA_COST_ADVERTISER" => ParametersMetricsItems :: MetricDataCostAdvertiser , "METRIC_DATA_COST_PARTNER" => ParametersMetricsItems :: MetricDataCostPartner , "METRIC_DATA_COST_USD" => ParametersMetricsItems :: MetricDataCostUsd , "METRIC_DBM_ENGAGEMENT_RATE" => ParametersMetricsItems :: MetricDbmEngagementRate , "METRIC_DEMO_COMPOSITION_IMPRESSION" => ParametersMetricsItems :: MetricDemoCompositionImpression , "METRIC_DEMO_CORRECTED_CLICKS" => ParametersMetricsItems :: MetricDemoCorrectedClicks , "METRIC_DEMO_POPULATION" => ParametersMetricsItems :: MetricDemoPopulation , "METRIC_DUPLICATE_FLOODLIGHT_IMPRESSIONS" => ParametersMetricsItems :: MetricDuplicateFloodlightImpressions , "METRIC_ENGAGEMENT_RATE" => ParametersMetricsItems :: MetricEngagementRate , "METRIC_ENGAGEMENTS" => ParametersMetricsItems :: MetricEngagements , "METRIC_ESTIMATED_CPM_FOR_IMPRESSIONS_WITH_CUSTOM_VALUE_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricEstimatedCpmForImpressionsWithCustomValueAdvertiserCurrency , "METRIC_ESTIMATED_TOTAL_COST_FOR_IMPRESSIONS_WITH_CUSTOM_VALUE_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricEstimatedTotalCostForImpressionsWithCustomValueAdvertiserCurrency , "METRIC_EXITS" => ParametersMetricsItems :: MetricExits , "METRIC_EXPANSIONS" => ParametersMetricsItems :: MetricExpansions , "METRIC_FEE10_ADVERTISER" => ParametersMetricsItems :: MetricFee10Advertiser , "METRIC_FEE10_PARTNER" => ParametersMetricsItems :: MetricFee10Partner , "METRIC_FEE10_USD" => ParametersMetricsItems :: MetricFee10Usd , "METRIC_FEE11_ADVERTISER" => ParametersMetricsItems :: MetricFee11Advertiser , "METRIC_FEE11_PARTNER" => ParametersMetricsItems :: MetricFee11Partner , "METRIC_FEE11_USD" => ParametersMetricsItems :: MetricFee11Usd , "METRIC_FEE12_ADVERTISER" => ParametersMetricsItems :: MetricFee12Advertiser , "METRIC_FEE12_PARTNER" => ParametersMetricsItems :: MetricFee12Partner , "METRIC_FEE12_USD" => ParametersMetricsItems :: MetricFee12Usd , "METRIC_FEE13_ADVERTISER" => ParametersMetricsItems :: MetricFee13Advertiser , "METRIC_FEE13_PARTNER" => ParametersMetricsItems :: MetricFee13Partner , "METRIC_FEE13_USD" => ParametersMetricsItems :: MetricFee13Usd , "METRIC_FEE14_ADVERTISER" => ParametersMetricsItems :: MetricFee14Advertiser , "METRIC_FEE14_PARTNER" => ParametersMetricsItems :: MetricFee14Partner , "METRIC_FEE14_USD" => ParametersMetricsItems :: MetricFee14Usd , "METRIC_FEE15_ADVERTISER" => ParametersMetricsItems :: MetricFee15Advertiser , "METRIC_FEE15_PARTNER" => ParametersMetricsItems :: MetricFee15Partner , "METRIC_FEE15_USD" => ParametersMetricsItems :: MetricFee15Usd , "METRIC_FEE16_ADVERTISER" => ParametersMetricsItems :: MetricFee16Advertiser , "METRIC_FEE16_PARTNER" => ParametersMetricsItems :: MetricFee16Partner , "METRIC_FEE16_USD" => ParametersMetricsItems :: MetricFee16Usd , "METRIC_FEE17_ADVERTISER" => ParametersMetricsItems :: MetricFee17Advertiser , "METRIC_FEE17_PARTNER" => ParametersMetricsItems :: MetricFee17Partner , "METRIC_FEE17_USD" => ParametersMetricsItems :: MetricFee17Usd , "METRIC_FEE18_ADVERTISER" => ParametersMetricsItems :: MetricFee18Advertiser , "METRIC_FEE18_PARTNER" => ParametersMetricsItems :: MetricFee18Partner , "METRIC_FEE18_USD" => ParametersMetricsItems :: MetricFee18Usd , "METRIC_FEE19_ADVERTISER" => ParametersMetricsItems :: MetricFee19Advertiser , "METRIC_FEE19_PARTNER" => ParametersMetricsItems :: MetricFee19Partner , "METRIC_FEE19_USD" => ParametersMetricsItems :: MetricFee19Usd , "METRIC_FEE20_ADVERTISER" => ParametersMetricsItems :: MetricFee20Advertiser , "METRIC_FEE20_PARTNER" => ParametersMetricsItems :: MetricFee20Partner , "METRIC_FEE20_USD" => ParametersMetricsItems :: MetricFee20Usd , "METRIC_FEE21_ADVERTISER" => ParametersMetricsItems :: MetricFee21Advertiser , "METRIC_FEE21_PARTNER" => ParametersMetricsItems :: MetricFee21Partner , "METRIC_FEE21_USD" => ParametersMetricsItems :: MetricFee21Usd , "METRIC_FEE22_ADVERTISER" => ParametersMetricsItems :: MetricFee22Advertiser , "METRIC_FEE22_PARTNER" => ParametersMetricsItems :: MetricFee22Partner , "METRIC_FEE22_USD" => ParametersMetricsItems :: MetricFee22Usd , "METRIC_FEE2_ADVERTISER" => ParametersMetricsItems :: MetricFee2Advertiser , "METRIC_FEE2_PARTNER" => ParametersMetricsItems :: MetricFee2Partner , "METRIC_FEE2_USD" => ParametersMetricsItems :: MetricFee2Usd , "METRIC_FEE3_ADVERTISER" => ParametersMetricsItems :: MetricFee3Advertiser , "METRIC_FEE3_PARTNER" => ParametersMetricsItems :: MetricFee3Partner , "METRIC_FEE3_USD" => ParametersMetricsItems :: MetricFee3Usd , "METRIC_FEE4_ADVERTISER" => ParametersMetricsItems :: MetricFee4Advertiser , "METRIC_FEE4_PARTNER" => ParametersMetricsItems :: MetricFee4Partner , "METRIC_FEE4_USD" => ParametersMetricsItems :: MetricFee4Usd , "METRIC_FEE5_ADVERTISER" => ParametersMetricsItems :: MetricFee5Advertiser , "METRIC_FEE5_PARTNER" => ParametersMetricsItems :: MetricFee5Partner , "METRIC_FEE5_USD" => ParametersMetricsItems :: MetricFee5Usd , "METRIC_FEE6_ADVERTISER" => ParametersMetricsItems :: MetricFee6Advertiser , "METRIC_FEE6_PARTNER" => ParametersMetricsItems :: MetricFee6Partner , "METRIC_FEE6_USD" => ParametersMetricsItems :: MetricFee6Usd , "METRIC_FEE7_ADVERTISER" => ParametersMetricsItems :: MetricFee7Advertiser , "METRIC_FEE7_PARTNER" => ParametersMetricsItems :: MetricFee7Partner , "METRIC_FEE7_USD" => ParametersMetricsItems :: MetricFee7Usd , "METRIC_FEE8_ADVERTISER" => ParametersMetricsItems :: MetricFee8Advertiser , "METRIC_FEE8_PARTNER" => ParametersMetricsItems :: MetricFee8Partner , "METRIC_FEE8_USD" => ParametersMetricsItems :: MetricFee8Usd , "METRIC_FEE9_ADVERTISER" => ParametersMetricsItems :: MetricFee9Advertiser , "METRIC_FEE9_PARTNER" => ParametersMetricsItems :: MetricFee9Partner , "METRIC_FEE9_USD" => ParametersMetricsItems :: MetricFee9Usd , "METRIC_FIRST_QUARTILE_AUDIO" => ParametersMetricsItems :: MetricFirstQuartileAudio , "METRIC_FLOODLIGHT_IMPRESSIONS" => ParametersMetricsItems :: MetricFloodlightImpressions , "METRIC_GENERAL_INVALID_TRAFFIC_GIVT_IMPRESSIONS" => ParametersMetricsItems :: MetricGeneralInvalidTrafficGivtImpressions , "METRIC_GENERAL_INVALID_TRAFFIC_GIVT_TRACKED_ADS" => ParametersMetricsItems :: MetricGeneralInvalidTrafficGivtTrackedAds , "METRIC_GIVT_ACTIVE_VIEW_ELIGIBLE_IMPRESSIONS" => ParametersMetricsItems :: MetricGivtActiveViewEligibleImpressions , "METRIC_GIVT_ACTIVE_VIEW_MEASURABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricGivtActiveViewMeasurableImpressions , "METRIC_GIVT_ACTIVE_VIEW_VIEWABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricGivtActiveViewViewableImpressions , "METRIC_GIVT_BEGIN_TO_RENDER_IMPRESSIONS" => ParametersMetricsItems :: MetricGivtBeginToRenderImpressions , "METRIC_GIVT_CLICKS" => ParametersMetricsItems :: MetricGivtClicks , "METRIC_GMAIL_CONVERSIONS" => ParametersMetricsItems :: MetricGmailConversions , "METRIC_GMAIL_POST_CLICK_CONVERSIONS" => ParametersMetricsItems :: MetricGmailPostClickConversions , "METRIC_GMAIL_POST_VIEW_CONVERSIONS" => ParametersMetricsItems :: MetricGmailPostViewConversions , "METRIC_GMAIL_POTENTIAL_VIEWS" => ParametersMetricsItems :: MetricGmailPotentialViews , "METRIC_GRP_CORRECTED_IMPRESSIONS" => ParametersMetricsItems :: MetricGrpCorrectedImpressions , "METRIC_GRP_CORRECTED_VIEWABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricGrpCorrectedViewableImpressions , "METRIC_GRP_CORRECTED_VIEWABLE_IMPRESSIONS_SHARE_PERCENT" => ParametersMetricsItems :: MetricGrpCorrectedViewableImpressionsSharePercent , "METRIC_IMPRESSION_CUSTOM_VALUE_COST" => ParametersMetricsItems :: MetricImpressionCustomValueCost , "METRIC_IMPRESSION_LOSS_TARGETED_IMPRESSIONS" => ParametersMetricsItems :: MetricImpressionLossTargetedImpressions , "METRIC_IMPRESSIONS" => ParametersMetricsItems :: MetricImpressions , "METRIC_IMPRESSIONS_TO_CONVERSION_RATE" => ParametersMetricsItems :: MetricImpressionsToConversionRate , "METRIC_IMPRESSIONS_WITH_CUSTOM_VALUE" => ParametersMetricsItems :: MetricImpressionsWithCustomValue , "METRIC_IMPRESSIONS_WITH_POSITIVE_CUSTOM_VALUE" => ParametersMetricsItems :: MetricImpressionsWithPositiveCustomValue , "METRIC_INTERACTIVE_IMPRESSIONS" => ParametersMetricsItems :: MetricInteractiveImpressions , "METRIC_INVALID_ACTIVE_VIEW_ELIGIBLE_IMPRESSIONS" => ParametersMetricsItems :: MetricInvalidActiveViewEligibleImpressions , "METRIC_INVALID_ACTIVE_VIEW_MEASURABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricInvalidActiveViewMeasurableImpressions , "METRIC_INVALID_ACTIVE_VIEW_VIEWABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricInvalidActiveViewViewableImpressions , "METRIC_INVALID_BEGIN_TO_RENDER_IMPRESSIONS" => ParametersMetricsItems :: MetricInvalidBeginToRenderImpressions , "METRIC_INVALID_CLICKS" => ParametersMetricsItems :: MetricInvalidClicks , "METRIC_INVALID_IMPRESSIONS" => ParametersMetricsItems :: MetricInvalidImpressions , "METRIC_INVALID_TRACKED_ADS" => ParametersMetricsItems :: MetricInvalidTrackedAds , "METRIC_LAST_CLICKS" => ParametersMetricsItems :: MetricLastClicks , "METRIC_LAST_IMPRESSIONS" => ParametersMetricsItems :: MetricLastImpressions , "METRIC_LAST_TOUCH_CLICK_THROUGH_CONVERSIONS" => ParametersMetricsItems :: MetricLastTouchClickThroughConversions , "METRIC_LAST_TOUCH_TOTAL_CONVERSIONS" => ParametersMetricsItems :: MetricLastTouchTotalConversions , "METRIC_LAST_TOUCH_VIEW_THROUGH_CONVERSIONS" => ParametersMetricsItems :: MetricLastTouchViewThroughConversions , "METRIC_LINEITEM_BID_RESPONSE_COUNT" => ParametersMetricsItems :: MetricLineitemBidResponseCount , "METRIC_MEDIA_COST_ADVERTISER" => ParametersMetricsItems :: MetricMediaCostAdvertiser , "METRIC_MEDIA_COST_ECPA_ADVERTISER" => ParametersMetricsItems :: MetricMediaCostEcpaAdvertiser , "METRIC_MEDIA_COST_ECPA_PARTNER" => ParametersMetricsItems :: MetricMediaCostEcpaPartner , "METRIC_MEDIA_COST_ECPA_USD" => ParametersMetricsItems :: MetricMediaCostEcpaUsd , "METRIC_MEDIA_COST_ECPAPC_ADVERTISER" => ParametersMetricsItems :: MetricMediaCostEcpapcAdvertiser , "METRIC_MEDIA_COST_ECPAPC_PARTNER" => ParametersMetricsItems :: MetricMediaCostEcpapcPartner , "METRIC_MEDIA_COST_ECPAPC_USD" => ParametersMetricsItems :: MetricMediaCostEcpapcUsd , "METRIC_MEDIA_COST_ECPAPV_ADVERTISER" => ParametersMetricsItems :: MetricMediaCostEcpapvAdvertiser , "METRIC_MEDIA_COST_ECPAPV_PARTNER" => ParametersMetricsItems :: MetricMediaCostEcpapvPartner , "METRIC_MEDIA_COST_ECPAPV_USD" => ParametersMetricsItems :: MetricMediaCostEcpapvUsd , "METRIC_MEDIA_COST_ECPC_ADVERTISER" => ParametersMetricsItems :: MetricMediaCostEcpcAdvertiser , "METRIC_MEDIA_COST_ECPC_PARTNER" => ParametersMetricsItems :: MetricMediaCostEcpcPartner , "METRIC_MEDIA_COST_ECPC_USD" => ParametersMetricsItems :: MetricMediaCostEcpcUsd , "METRIC_MEDIA_COST_ECPCV_ADVERTISER" => ParametersMetricsItems :: MetricMediaCostEcpcvAdvertiser , "METRIC_MEDIA_COST_ECPCV_PARTNER" => ParametersMetricsItems :: MetricMediaCostEcpcvPartner , "METRIC_MEDIA_COST_ECPCV_USD" => ParametersMetricsItems :: MetricMediaCostEcpcvUsd , "METRIC_MEDIA_COST_ECPM_ADVERTISER" => ParametersMetricsItems :: MetricMediaCostEcpmAdvertiser , "METRIC_MEDIA_COST_ECPM_PARTNER" => ParametersMetricsItems :: MetricMediaCostEcpmPartner , "METRIC_MEDIA_COST_ECPM_USD" => ParametersMetricsItems :: MetricMediaCostEcpmUsd , "METRIC_MEDIA_COST_PARTNER" => ParametersMetricsItems :: MetricMediaCostPartner , "METRIC_MEDIA_COST_USD" => ParametersMetricsItems :: MetricMediaCostUsd , "METRIC_MEDIA_COST_VIEWABLE_ECPM_ADVERTISER" => ParametersMetricsItems :: MetricMediaCostViewableEcpmAdvertiser , "METRIC_MEDIA_COST_VIEWABLE_ECPM_PARTNER" => ParametersMetricsItems :: MetricMediaCostViewableEcpmPartner , "METRIC_MEDIA_COST_VIEWABLE_ECPM_USD" => ParametersMetricsItems :: MetricMediaCostViewableEcpmUsd , "METRIC_MEDIA_FEE1_ADVERTISER" => ParametersMetricsItems :: MetricMediaFee1Advertiser , "METRIC_MEDIA_FEE1_PARTNER" => ParametersMetricsItems :: MetricMediaFee1Partner , "METRIC_MEDIA_FEE1_USD" => ParametersMetricsItems :: MetricMediaFee1Usd , "METRIC_MEDIA_FEE2_ADVERTISER" => ParametersMetricsItems :: MetricMediaFee2Advertiser , "METRIC_MEDIA_FEE2_PARTNER" => ParametersMetricsItems :: MetricMediaFee2Partner , "METRIC_MEDIA_FEE2_USD" => ParametersMetricsItems :: MetricMediaFee2Usd , "METRIC_MEDIA_FEE3_ADVERTISER" => ParametersMetricsItems :: MetricMediaFee3Advertiser , "METRIC_MEDIA_FEE3_PARTNER" => ParametersMetricsItems :: MetricMediaFee3Partner , "METRIC_MEDIA_FEE3_USD" => ParametersMetricsItems :: MetricMediaFee3Usd , "METRIC_MEDIA_FEE4_ADVERTISER" => ParametersMetricsItems :: MetricMediaFee4Advertiser , "METRIC_MEDIA_FEE4_PARTNER" => ParametersMetricsItems :: MetricMediaFee4Partner , "METRIC_MEDIA_FEE4_USD" => ParametersMetricsItems :: MetricMediaFee4Usd , "METRIC_MEDIA_FEE5_ADVERTISER" => ParametersMetricsItems :: MetricMediaFee5Advertiser , "METRIC_MEDIA_FEE5_PARTNER" => ParametersMetricsItems :: MetricMediaFee5Partner , "METRIC_MEDIA_FEE5_USD" => ParametersMetricsItems :: MetricMediaFee5Usd , "METRIC_MIDPOINT_AUDIO" => ParametersMetricsItems :: MetricMidpointAudio , "METRIC_NIELSEN_AVERAGE_FREQUENCY" => ParametersMetricsItems :: MetricNielsenAverageFrequency , "METRIC_NIELSEN_GRP" => ParametersMetricsItems :: MetricNielsenGrp , "METRIC_NIELSEN_IMPRESSION_INDEX" => ParametersMetricsItems :: MetricNielsenImpressionIndex , "METRIC_NIELSEN_IMPRESSIONS" => ParametersMetricsItems :: MetricNielsenImpressions , "METRIC_NIELSEN_IMPRESSIONS_SHARE" => ParametersMetricsItems :: MetricNielsenImpressionsShare , "METRIC_NIELSEN_POPULATION" => ParametersMetricsItems :: MetricNielsenPopulation , "METRIC_NIELSEN_POPULATION_REACH" => ParametersMetricsItems :: MetricNielsenPopulationReach , "METRIC_NIELSEN_POPULATION_SHARE" => ParametersMetricsItems :: MetricNielsenPopulationShare , "METRIC_NIELSEN_REACH_INDEX" => ParametersMetricsItems :: MetricNielsenReachIndex , "METRIC_NIELSEN_REACH_SHARE" => ParametersMetricsItems :: MetricNielsenReachShare , "METRIC_NIELSEN_UNIQUE_AUDIENCE" => ParametersMetricsItems :: MetricNielsenUniqueAudience , "METRIC_ORIGINAL_AUDIENCE_FREQUENCY" => ParametersMetricsItems :: MetricOriginalAudienceFrequency , "METRIC_PATH_CONVERSION_RATE" => ParametersMetricsItems :: MetricPathConversionRate , "METRIC_PAUSES_AUDIO" => ParametersMetricsItems :: MetricPausesAudio , "METRIC_PERCENT_IMPRESSIONS_WITH_POSITIVE_CUSTOM_VALUE" => ParametersMetricsItems :: MetricPercentImpressionsWithPositiveCustomValue , "METRIC_PERCENT_INVALID_IMPRESSIONS_PREBID" => ParametersMetricsItems :: MetricPercentInvalidImpressionsPrebid , "METRIC_PERCENTAGE_FROM_CURRENT_IO_GOAL" => ParametersMetricsItems :: MetricPercentageFromCurrentIoGoal , "METRIC_PLATFORM_FEE_ADVERTISER" => ParametersMetricsItems :: MetricPlatformFeeAdvertiser , "METRIC_PLATFORM_FEE_PARTNER" => ParametersMetricsItems :: MetricPlatformFeePartner , "METRIC_PLATFORM_FEE_RATE" => ParametersMetricsItems :: MetricPlatformFeeRate , "METRIC_PLATFORM_FEE_USD" => ParametersMetricsItems :: MetricPlatformFeeUsd , "METRIC_POST_CLICK_CONVERSIONS_CROSS_ENVIRONMENT" => ParametersMetricsItems :: MetricPostClickConversionsCrossEnvironment , "METRIC_POST_VIEW_CONVERSIONS_CROSS_ENVIRONMENT" => ParametersMetricsItems :: MetricPostViewConversionsCrossEnvironment , "METRIC_POTENTIAL_IMPRESSIONS" => ParametersMetricsItems :: MetricPotentialImpressions , "METRIC_POTENTIAL_VIEWS" => ParametersMetricsItems :: MetricPotentialViews , "METRIC_PREMIUM_FEE_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricPremiumFeeAdvertiserCurrency , "METRIC_PROFIT_ADVERTISER" => ParametersMetricsItems :: MetricProfitAdvertiser , "METRIC_PROFIT_ECPM_ADVERTISER" => ParametersMetricsItems :: MetricProfitEcpmAdvertiser , "METRIC_PROFIT_ECPM_PARTNER" => ParametersMetricsItems :: MetricProfitEcpmPartner , "METRIC_PROFIT_ECPM_USD" => ParametersMetricsItems :: MetricProfitEcpmUsd , "METRIC_PROFIT_MARGIN" => ParametersMetricsItems :: MetricProfitMargin , "METRIC_PROFIT_PARTNER" => ParametersMetricsItems :: MetricProfitPartner , "METRIC_PROFIT_USD" => ParametersMetricsItems :: MetricProfitUsd , "METRIC_PROFIT_VIEWABLE_ECPM_ADVERTISER" => ParametersMetricsItems :: MetricProfitViewableEcpmAdvertiser , "METRIC_PROFIT_VIEWABLE_ECPM_PARTNER" => ParametersMetricsItems :: MetricProfitViewableEcpmPartner , "METRIC_PROFIT_VIEWABLE_ECPM_USD" => ParametersMetricsItems :: MetricProfitViewableEcpmUsd , "METRIC_PROGRAMMATIC_GUARANTEED_IMPRESSIONS_PASSED_DUE_TO_FREQUENCY" => ParametersMetricsItems :: MetricProgrammaticGuaranteedImpressionsPassedDueToFrequency , "METRIC_PROGRAMMATIC_GUARANTEED_SAVINGS_RE_INVESTED_DUE_TO_FREQUENCY_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricProgrammaticGuaranteedSavingsReInvestedDueToFrequencyAdvertiserCurrency , "METRIC_PROVISIONAL_IMPRESSIONS" => ParametersMetricsItems :: MetricProvisionalImpressions , "METRIC_REFUND_BILLABLE_COST_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricRefundBillableCostAdvertiserCurrency , "METRIC_REFUND_MEDIA_COST_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricRefundMediaCostAdvertiserCurrency , "METRIC_REFUND_PLATFORM_FEE_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricRefundPlatformFeeAdvertiserCurrency , "METRIC_REVENUE_ADVERTISER" => ParametersMetricsItems :: MetricRevenueAdvertiser , "METRIC_REVENUE_ECPA_ADVERTISER" => ParametersMetricsItems :: MetricRevenueEcpaAdvertiser , "METRIC_REVENUE_ECPA_PARTNER" => ParametersMetricsItems :: MetricRevenueEcpaPartner , "METRIC_REVENUE_ECPA_USD" => ParametersMetricsItems :: MetricRevenueEcpaUsd , "METRIC_REVENUE_ECPAPC_ADVERTISER" => ParametersMetricsItems :: MetricRevenueEcpapcAdvertiser , "METRIC_REVENUE_ECPAPC_PARTNER" => ParametersMetricsItems :: MetricRevenueEcpapcPartner , "METRIC_REVENUE_ECPAPC_USD" => ParametersMetricsItems :: MetricRevenueEcpapcUsd , "METRIC_REVENUE_ECPAPV_ADVERTISER" => ParametersMetricsItems :: MetricRevenueEcpapvAdvertiser , "METRIC_REVENUE_ECPAPV_PARTNER" => ParametersMetricsItems :: MetricRevenueEcpapvPartner , "METRIC_REVENUE_ECPAPV_USD" => ParametersMetricsItems :: MetricRevenueEcpapvUsd , "METRIC_REVENUE_ECPC_ADVERTISER" => ParametersMetricsItems :: MetricRevenueEcpcAdvertiser , "METRIC_REVENUE_ECPC_PARTNER" => ParametersMetricsItems :: MetricRevenueEcpcPartner , "METRIC_REVENUE_ECPC_USD" => ParametersMetricsItems :: MetricRevenueEcpcUsd , "METRIC_REVENUE_ECPCV_ADVERTISER" => ParametersMetricsItems :: MetricRevenueEcpcvAdvertiser , "METRIC_REVENUE_ECPCV_PARTNER" => ParametersMetricsItems :: MetricRevenueEcpcvPartner , "METRIC_REVENUE_ECPCV_USD" => ParametersMetricsItems :: MetricRevenueEcpcvUsd , "METRIC_REVENUE_ECPM_ADVERTISER" => ParametersMetricsItems :: MetricRevenueEcpmAdvertiser , "METRIC_REVENUE_ECPM_PARTNER" => ParametersMetricsItems :: MetricRevenueEcpmPartner , "METRIC_REVENUE_ECPM_USD" => ParametersMetricsItems :: MetricRevenueEcpmUsd , "METRIC_REVENUE_PARTNER" => ParametersMetricsItems :: MetricRevenuePartner , "METRIC_REVENUE_USD" => ParametersMetricsItems :: MetricRevenueUsd , "METRIC_REVENUE_VIEWABLE_ECPM_ADVERTISER" => ParametersMetricsItems :: MetricRevenueViewableEcpmAdvertiser , "METRIC_REVENUE_VIEWABLE_ECPM_PARTNER" => ParametersMetricsItems :: MetricRevenueViewableEcpmPartner , "METRIC_REVENUE_VIEWABLE_ECPM_USD" => ParametersMetricsItems :: MetricRevenueViewableEcpmUsd , "METRIC_RICH_MEDIA_ENGAGEMENTS" => ParametersMetricsItems :: MetricRichMediaEngagements , "METRIC_RICH_MEDIA_SCROLLS" => ParametersMetricsItems :: MetricRichMediaScrolls , "METRIC_RICH_MEDIA_VIDEO_COMPLETIONS" => ParametersMetricsItems :: MetricRichMediaVideoCompletions , "METRIC_RICH_MEDIA_VIDEO_FIRST_QUARTILE_COMPLETES" => ParametersMetricsItems :: MetricRichMediaVideoFirstQuartileCompletes , "METRIC_RICH_MEDIA_VIDEO_FULL_SCREENS" => ParametersMetricsItems :: MetricRichMediaVideoFullScreens , "METRIC_RICH_MEDIA_VIDEO_MIDPOINTS" => ParametersMetricsItems :: MetricRichMediaVideoMidpoints , "METRIC_RICH_MEDIA_VIDEO_MUTES" => ParametersMetricsItems :: MetricRichMediaVideoMutes , "METRIC_RICH_MEDIA_VIDEO_PAUSES" => ParametersMetricsItems :: MetricRichMediaVideoPauses , "METRIC_RICH_MEDIA_VIDEO_PLAYS" => ParametersMetricsItems :: MetricRichMediaVideoPlays , "METRIC_RICH_MEDIA_VIDEO_SKIPS" => ParametersMetricsItems :: MetricRichMediaVideoSkips , "METRIC_RICH_MEDIA_VIDEO_THIRD_QUARTILE_COMPLETES" => ParametersMetricsItems :: MetricRichMediaVideoThirdQuartileCompletes , "METRIC_STARTS_AUDIO" => ParametersMetricsItems :: MetricStartsAudio , "METRIC_STOPS_AUDIO" => ParametersMetricsItems :: MetricStopsAudio , "METRIC_STORE_VISIT_CONVERSIONS" => ParametersMetricsItems :: MetricStoreVisitConversions , "METRIC_TARGET_RATING_POINTS" => ParametersMetricsItems :: MetricTargetRatingPoints , "METRIC_TEA_TRUEVIEW_IMPRESSIONS" => ParametersMetricsItems :: MetricTeaTrueviewImpressions , "METRIC_TEA_TRUEVIEW_UNIQUE_COOKIES" => ParametersMetricsItems :: MetricTeaTrueviewUniqueCookies , "METRIC_THIRD_QUARTILE_AUDIO" => ParametersMetricsItems :: MetricThirdQuartileAudio , "METRIC_TIMERS" => ParametersMetricsItems :: MetricTimers , "METRIC_TOTAL_AUDIO_MEDIA_COST_ECPCL_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricTotalAudioMediaCostEcpclAdvertiserCurrency , "METRIC_TOTAL_CONVERSIONS" => ParametersMetricsItems :: MetricTotalConversions , "METRIC_TOTAL_CONVERSIONS_CROSS_ENVIRONMENT" => ParametersMetricsItems :: MetricTotalConversionsCrossEnvironment , "METRIC_TOTAL_DISPLAY_TIME" => ParametersMetricsItems :: MetricTotalDisplayTime , "METRIC_TOTAL_EXPOSURES" => ParametersMetricsItems :: MetricTotalExposures , "METRIC_TOTAL_IMPRESSION_CUSTOM_VALUE" => ParametersMetricsItems :: MetricTotalImpressionCustomValue , "METRIC_TOTAL_INTERACTION_TIME" => ParametersMetricsItems :: MetricTotalInteractionTime , "METRIC_TOTAL_MEDIA_COST_ADVERTISER" => ParametersMetricsItems :: MetricTotalMediaCostAdvertiser , "METRIC_TOTAL_MEDIA_COST_ECPA_ADVERTISER" => ParametersMetricsItems :: MetricTotalMediaCostEcpaAdvertiser , "METRIC_TOTAL_MEDIA_COST_ECPA_PARTNER" => ParametersMetricsItems :: MetricTotalMediaCostEcpaPartner , "METRIC_TOTAL_MEDIA_COST_ECPA_USD" => ParametersMetricsItems :: MetricTotalMediaCostEcpaUsd , "METRIC_TOTAL_MEDIA_COST_ECPAPC_ADVERTISER" => ParametersMetricsItems :: MetricTotalMediaCostEcpapcAdvertiser , "METRIC_TOTAL_MEDIA_COST_ECPAPC_PARTNER" => ParametersMetricsItems :: MetricTotalMediaCostEcpapcPartner , "METRIC_TOTAL_MEDIA_COST_ECPAPC_USD" => ParametersMetricsItems :: MetricTotalMediaCostEcpapcUsd , "METRIC_TOTAL_MEDIA_COST_ECPAPV_ADVERTISER" => ParametersMetricsItems :: MetricTotalMediaCostEcpapvAdvertiser , "METRIC_TOTAL_MEDIA_COST_ECPAPV_PARTNER" => ParametersMetricsItems :: MetricTotalMediaCostEcpapvPartner , "METRIC_TOTAL_MEDIA_COST_ECPAPV_USD" => ParametersMetricsItems :: MetricTotalMediaCostEcpapvUsd , "METRIC_TOTAL_MEDIA_COST_ECPC_ADVERTISER" => ParametersMetricsItems :: MetricTotalMediaCostEcpcAdvertiser , "METRIC_TOTAL_MEDIA_COST_ECPC_PARTNER" => ParametersMetricsItems :: MetricTotalMediaCostEcpcPartner , "METRIC_TOTAL_MEDIA_COST_ECPC_USD" => ParametersMetricsItems :: MetricTotalMediaCostEcpcUsd , "METRIC_TOTAL_MEDIA_COST_ECPCV_ADVERTISER" => ParametersMetricsItems :: MetricTotalMediaCostEcpcvAdvertiser , "METRIC_TOTAL_MEDIA_COST_ECPCV_PARTNER" => ParametersMetricsItems :: MetricTotalMediaCostEcpcvPartner , "METRIC_TOTAL_MEDIA_COST_ECPCV_USD" => ParametersMetricsItems :: MetricTotalMediaCostEcpcvUsd , "METRIC_TOTAL_MEDIA_COST_ECPM_ADVERTISER" => ParametersMetricsItems :: MetricTotalMediaCostEcpmAdvertiser , "METRIC_TOTAL_MEDIA_COST_ECPM_PARTNER" => ParametersMetricsItems :: MetricTotalMediaCostEcpmPartner , "METRIC_TOTAL_MEDIA_COST_ECPM_USD" => ParametersMetricsItems :: MetricTotalMediaCostEcpmUsd , "METRIC_TOTAL_MEDIA_COST_PARTNER" => ParametersMetricsItems :: MetricTotalMediaCostPartner , "METRIC_TOTAL_MEDIA_COST_USD" => ParametersMetricsItems :: MetricTotalMediaCostUsd , "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_ADVERTISER" => ParametersMetricsItems :: MetricTotalMediaCostViewableEcpmAdvertiser , "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_PARTNER" => ParametersMetricsItems :: MetricTotalMediaCostViewableEcpmPartner , "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_USD" => ParametersMetricsItems :: MetricTotalMediaCostViewableEcpmUsd , "METRIC_TOTAL_PATHS" => ParametersMetricsItems :: MetricTotalPaths , "METRIC_TOTAL_USERS" => ParametersMetricsItems :: MetricTotalUsers , "METRIC_TRACKED_ADS" => ParametersMetricsItems :: MetricTrackedAds , "METRIC_TRACKING_UNCONSENTED_CLICKS" => ParametersMetricsItems :: MetricTrackingUnconsentedClicks , "METRIC_TRUEVIEW_ALL_AD_SEQUENCE_IMPRESSIONS" => ParametersMetricsItems :: MetricTrueviewAllAdSequenceImpressions , "METRIC_TRUEVIEW_AVERAGE_CPE_ADVERTISER" => ParametersMetricsItems :: MetricTrueviewAverageCpeAdvertiser , "METRIC_TRUEVIEW_AVERAGE_CPE_PARTNER" => ParametersMetricsItems :: MetricTrueviewAverageCpePartner , "METRIC_TRUEVIEW_AVERAGE_CPE_USD" => ParametersMetricsItems :: MetricTrueviewAverageCpeUsd , "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_ADVERTISER" => ParametersMetricsItems :: MetricTrueviewConversionCostManyPerViewAdvertiser , "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_PARTNER" => ParametersMetricsItems :: MetricTrueviewConversionCostManyPerViewPartner , "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_USD" => ParametersMetricsItems :: MetricTrueviewConversionCostManyPerViewUsd , "METRIC_TRUEVIEW_CONVERSION_MANY_PER_VIEW" => ParametersMetricsItems :: MetricTrueviewConversionManyPerView , "METRIC_TRUEVIEW_CONVERSION_RATE_ONE_PER_VIEW" => ParametersMetricsItems :: MetricTrueviewConversionRateOnePerView , "METRIC_TRUEVIEW_CPV_ADVERTISER" => ParametersMetricsItems :: MetricTrueviewCpvAdvertiser , "METRIC_TRUEVIEW_CPV_PARTNER" => ParametersMetricsItems :: MetricTrueviewCpvPartner , "METRIC_TRUEVIEW_CPV_USD" => ParametersMetricsItems :: MetricTrueviewCpvUsd , "METRIC_TRUEVIEW_EARNED_LIKES" => ParametersMetricsItems :: MetricTrueviewEarnedLikes , "METRIC_TRUEVIEW_EARNED_PLAYLIST_ADDITIONS" => ParametersMetricsItems :: MetricTrueviewEarnedPlaylistAdditions , "METRIC_TRUEVIEW_EARNED_SHARES" => ParametersMetricsItems :: MetricTrueviewEarnedShares , "METRIC_TRUEVIEW_EARNED_SUBSCRIBERS" => ParametersMetricsItems :: MetricTrueviewEarnedSubscribers , "METRIC_TRUEVIEW_EARNED_VIEWS" => ParametersMetricsItems :: MetricTrueviewEarnedViews , "METRIC_TRUEVIEW_ENGAGEMENT_RATE" => ParametersMetricsItems :: MetricTrueviewEngagementRate , "METRIC_TRUEVIEW_ENGAGEMENTS" => ParametersMetricsItems :: MetricTrueviewEngagements , "METRIC_TRUEVIEW_GENERAL_INVALID_TRAFFIC_GIVT_VIEWS" => ParametersMetricsItems :: MetricTrueviewGeneralInvalidTrafficGivtViews , "METRIC_TRUEVIEW_IMPRESSION_SHARE" => ParametersMetricsItems :: MetricTrueviewImpressionShare , "METRIC_TRUEVIEW_INVALID_VIEWS" => ParametersMetricsItems :: MetricTrueviewInvalidViews , "METRIC_TRUEVIEW_LOST_IS_BUDGET" => ParametersMetricsItems :: MetricTrueviewLostIsBudget , "METRIC_TRUEVIEW_LOST_IS_RANK" => ParametersMetricsItems :: MetricTrueviewLostIsRank , "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_ADVERTISER" => ParametersMetricsItems :: MetricTrueviewTotalConversionValuesAdvertiser , "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_PARTNER" => ParametersMetricsItems :: MetricTrueviewTotalConversionValuesPartner , "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_USD" => ParametersMetricsItems :: MetricTrueviewTotalConversionValuesUsd , "METRIC_TRUEVIEW_UNIQUE_VIEWERS" => ParametersMetricsItems :: MetricTrueviewUniqueViewers , "METRIC_TRUEVIEW_VIEW_RATE" => ParametersMetricsItems :: MetricTrueviewViewRate , "METRIC_TRUEVIEW_VIEW_THROUGH_CONVERSION" => ParametersMetricsItems :: MetricTrueviewViewThroughConversion , "METRIC_TRUEVIEW_VIEWS" => ParametersMetricsItems :: MetricTrueviewViews , "METRIC_UNIQUE_COOKIES_WITH_IMPRESSIONS" => ParametersMetricsItems :: MetricUniqueCookiesWithImpressions , "METRIC_UNIQUE_REACH_AVERAGE_IMPRESSION_FREQUENCY" => ParametersMetricsItems :: MetricUniqueReachAverageImpressionFrequency , "METRIC_UNIQUE_REACH_CLICK_REACH" => ParametersMetricsItems :: MetricUniqueReachClickReach , "METRIC_UNIQUE_REACH_IMPRESSION_REACH" => ParametersMetricsItems :: MetricUniqueReachImpressionReach , "METRIC_UNIQUE_REACH_TOTAL_REACH" => ParametersMetricsItems :: MetricUniqueReachTotalReach , "METRIC_UNIQUE_VISITORS_COOKIES" => ParametersMetricsItems :: MetricUniqueVisitorsCookies , "METRIC_UNKNOWN" => ParametersMetricsItems :: MetricUnknown , "METRIC_VENDOR_BLOCKED_ADS" => ParametersMetricsItems :: MetricVendorBlockedAds , "METRIC_VERIFIABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricVerifiableImpressions , "METRIC_VERIFICATION_VIDEO_PLAYER_SIZE_MEASURABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricVerificationVideoPlayerSizeMeasurableImpressions , "METRIC_VIDEO_CLIENT_COST_ECPCV_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricVideoClientCostEcpcvAdvertiserCurrency , "METRIC_VIDEO_COMPANION_CLICKS" => ParametersMetricsItems :: MetricVideoCompanionClicks , "METRIC_VIDEO_COMPANION_IMPRESSIONS" => ParametersMetricsItems :: MetricVideoCompanionImpressions , "METRIC_VIDEO_COMPLETION_RATE" => ParametersMetricsItems :: MetricVideoCompletionRate , "METRIC_VIEWABLE_BID_REQUESTS" => ParametersMetricsItems :: MetricViewableBidRequests , "METRIC_VIEWABLE_GROSS_RATING_POINTS" => ParametersMetricsItems :: MetricViewableGrossRatingPoints , "METRIC_VIRTUAL_PEOPLE_AVERAGE_IMPRESSION_FREQUENCY_BY_DEMO" => ParametersMetricsItems :: MetricVirtualPeopleAverageImpressionFrequencyByDemo , "METRIC_VIRTUAL_PEOPLE_AVERAGE_VIEWABLE_IMPRESSION_FREQUENCY_BY_DEMO" => ParametersMetricsItems :: MetricVirtualPeopleAverageViewableImpressionFrequencyByDemo , "METRIC_VIRTUAL_PEOPLE_CLICK_REACH_BY_DEMO" => ParametersMetricsItems :: MetricVirtualPeopleClickReachByDemo , "METRIC_VIRTUAL_PEOPLE_IMPRESSION_REACH_BY_DEMO" => ParametersMetricsItems :: MetricVirtualPeopleImpressionReachByDemo , "METRIC_VIRTUAL_PEOPLE_IMPRESSION_REACH_PERCENT" => ParametersMetricsItems :: MetricVirtualPeopleImpressionReachPercent , "METRIC_VIRTUAL_PEOPLE_IMPRESSION_REACH_SHARE_PERCENT" => ParametersMetricsItems :: MetricVirtualPeopleImpressionReachSharePercent , "METRIC_VIRTUAL_PEOPLE_VIEWABLE_IMPRESSION_REACH_BY_DEMO" => ParametersMetricsItems :: MetricVirtualPeopleViewableImpressionReachByDemo , "METRIC_VIRTUAL_PEOPLE_VIEWABLE_IMPRESSION_REACH_PERCENT" => ParametersMetricsItems :: MetricVirtualPeopleViewableImpressionReachPercent , "METRIC_VIRTUAL_PEOPLE_VIEWABLE_IMPRESSION_REACH_SHARE_PERCENT" => ParametersMetricsItems :: MetricVirtualPeopleViewableImpressionReachSharePercent , "METRIC_WATCH_TIME" => ParametersMetricsItems :: MetricWatchTime , "METRIC_WIN_LOSS_DEAL_AVAILABLE_REQUESTS" => ParametersMetricsItems :: MetricWinLossDealAvailableRequests , "METRIC_WIN_LOSS_DEAL_TARGETED_IMPRESSIONS" => ParametersMetricsItems :: MetricWinLossDealTargetedImpressions , "METRIC_WIN_LOSS_LINEITEM_AVAILABLE_REQUESTS" => ParametersMetricsItems :: MetricWinLossLineitemAvailableRequests , "METRIC_WIN_LOSS_LINEITEM_TARGETED_IMPRESSIONS" => ParametersMetricsItems :: MetricWinLossLineitemTargetedImpressions , "METRIC_WIN_LOSS_RATE" => ParametersMetricsItems :: MetricWinLossRate , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for ParametersMetricsItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ParametersMetricsItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ParametersMetricsItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "METRIC_ACTIVE_VIEW_AUDIBLE_FULLY_ON_SCREEN_HALF_OF_DURATION_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewAudibleFullyOnScreenHalfOfDurationImpressions , "METRIC_ACTIVE_VIEW_AUDIBLE_FULLY_ON_SCREEN_HALF_OF_DURATION_MEASURABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewAudibleFullyOnScreenHalfOfDurationMeasurableImpressions , "METRIC_ACTIVE_VIEW_AUDIBLE_FULLY_ON_SCREEN_HALF_OF_DURATION_RATE" => ParametersMetricsItems :: MetricActiveViewAudibleFullyOnScreenHalfOfDurationRate , "METRIC_ACTIVE_VIEW_AUDIBLE_FULLY_ON_SCREEN_HALF_OF_DURATION_TRUEVIEW_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewAudibleFullyOnScreenHalfOfDurationTrueviewImpressions , "METRIC_ACTIVE_VIEW_AUDIBLE_FULLY_ON_SCREEN_HALF_OF_DURATION_TRUEVIEW_MEASURABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewAudibleFullyOnScreenHalfOfDurationTrueviewMeasurableImpressions , "METRIC_ACTIVE_VIEW_AUDIBLE_FULLY_ON_SCREEN_HALF_OF_DURATION_TRUEVIEW_RATE" => ParametersMetricsItems :: MetricActiveViewAudibleFullyOnScreenHalfOfDurationTrueviewRate , "METRIC_ACTIVE_VIEW_AUDIBLE_VISIBLE_ON_COMPLETE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewAudibleVisibleOnCompleteImpressions , "METRIC_ACTIVE_VIEW_AVERAGE_VIEWABLE_TIME" => ParametersMetricsItems :: MetricActiveViewAverageViewableTime , "METRIC_ACTIVE_VIEW_CUSTOM_METRIC_MEASURABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewCustomMetricMeasurableImpressions , "METRIC_ACTIVE_VIEW_CUSTOM_METRIC_VIEWABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewCustomMetricViewableImpressions , "METRIC_ACTIVE_VIEW_CUSTOM_METRIC_VIEWABLE_RATE" => ParametersMetricsItems :: MetricActiveViewCustomMetricViewableRate , "METRIC_ACTIVE_VIEW_DISTRIBUTION_UNMEASURABLE" => ParametersMetricsItems :: MetricActiveViewDistributionUnmeasurable , "METRIC_ACTIVE_VIEW_DISTRIBUTION_UNVIEWABLE" => ParametersMetricsItems :: MetricActiveViewDistributionUnviewable , "METRIC_ACTIVE_VIEW_DISTRIBUTION_VIEWABLE" => ParametersMetricsItems :: MetricActiveViewDistributionViewable , "METRIC_ACTIVE_VIEW_ELIGIBLE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewEligibleImpressions , "METRIC_ACTIVE_VIEW_MEASURABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewMeasurableImpressions , "METRIC_ACTIVE_VIEW_PCT_MEASURABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewPctMeasurableImpressions , "METRIC_ACTIVE_VIEW_PCT_VIEWABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewPctViewableImpressions , "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewPercentAudibleImpressions , "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_AT_START" => ParametersMetricsItems :: MetricActiveViewPercentAudibleVisibleAtStart , "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_FIRST_QUAR" => ParametersMetricsItems :: MetricActiveViewPercentAudibleVisibleFirstQuar , "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_ON_COMPLETE" => ParametersMetricsItems :: MetricActiveViewPercentAudibleVisibleOnComplete , "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_SECOND_QUAR" => ParametersMetricsItems :: MetricActiveViewPercentAudibleVisibleSecondQuar , "METRIC_ACTIVE_VIEW_PERCENT_AUDIBLE_VISIBLE_THIRD_QUAR" => ParametersMetricsItems :: MetricActiveViewPercentAudibleVisibleThirdQuar , "METRIC_ACTIVE_VIEW_PERCENT_FULL_SCREEN" => ParametersMetricsItems :: MetricActiveViewPercentFullScreen , "METRIC_ACTIVE_VIEW_PERCENT_FULLY_ON_SCREEN_2_SEC" => ParametersMetricsItems :: MetricActiveViewPercentFullyOnScreen2Sec , "METRIC_ACTIVE_VIEW_PERCENT_IN_BACKGROUND" => ParametersMetricsItems :: MetricActiveViewPercentInBackground , "METRIC_ACTIVE_VIEW_PERCENT_OF_AD_PLAYED" => ParametersMetricsItems :: MetricActiveViewPercentOfAdPlayed , "METRIC_ACTIVE_VIEW_PERCENT_OF_COMPLETED_IMPRESSIONS_AUDIBLE_AND_VISIBLE" => ParametersMetricsItems :: MetricActiveViewPercentOfCompletedImpressionsAudibleAndVisible , "METRIC_ACTIVE_VIEW_PERCENT_OF_COMPLETED_IMPRESSIONS_VISIBLE" => ParametersMetricsItems :: MetricActiveViewPercentOfCompletedImpressionsVisible , "METRIC_ACTIVE_VIEW_PERCENT_OF_FIRST_QUARTILE_IMPRESSIONS_AUDIBLE_AND_VISIBLE" => ParametersMetricsItems :: MetricActiveViewPercentOfFirstQuartileImpressionsAudibleAndVisible , "METRIC_ACTIVE_VIEW_PERCENT_OF_FIRST_QUARTILE_IMPRESSIONS_VISIBLE" => ParametersMetricsItems :: MetricActiveViewPercentOfFirstQuartileImpressionsVisible , "METRIC_ACTIVE_VIEW_PERCENT_OF_MIDPOINT_IMPRESSIONS_AUDIBLE_AND_VISIBLE" => ParametersMetricsItems :: MetricActiveViewPercentOfMidpointImpressionsAudibleAndVisible , "METRIC_ACTIVE_VIEW_PERCENT_OF_MIDPOINT_IMPRESSIONS_VISIBLE" => ParametersMetricsItems :: MetricActiveViewPercentOfMidpointImpressionsVisible , "METRIC_ACTIVE_VIEW_PERCENT_OF_THIRD_QUARTILE_IMPRESSIONS_AUDIBLE_AND_VISIBLE" => ParametersMetricsItems :: MetricActiveViewPercentOfThirdQuartileImpressionsAudibleAndVisible , "METRIC_ACTIVE_VIEW_PERCENT_OF_THIRD_QUARTILE_IMPRESSIONS_VISIBLE" => ParametersMetricsItems :: MetricActiveViewPercentOfThirdQuartileImpressionsVisible , "METRIC_ACTIVE_VIEW_PERCENT_PLAY_TIME_AUDIBLE" => ParametersMetricsItems :: MetricActiveViewPercentPlayTimeAudible , "METRIC_ACTIVE_VIEW_PERCENT_PLAY_TIME_AUDIBLE_AND_VISIBLE" => ParametersMetricsItems :: MetricActiveViewPercentPlayTimeAudibleAndVisible , "METRIC_ACTIVE_VIEW_PERCENT_PLAY_TIME_VISIBLE" => ParametersMetricsItems :: MetricActiveViewPercentPlayTimeVisible , "METRIC_ACTIVE_VIEW_PERCENT_VIEWABLE_FOR_TIME_THRESHOLD" => ParametersMetricsItems :: MetricActiveViewPercentViewableForTimeThreshold , "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_AT_START" => ParametersMetricsItems :: MetricActiveViewPercentVisibleAtStart , "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_FIRST_QUAR" => ParametersMetricsItems :: MetricActiveViewPercentVisibleFirstQuar , "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_ON_COMPLETE" => ParametersMetricsItems :: MetricActiveViewPercentVisibleOnComplete , "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_SECOND_QUAR" => ParametersMetricsItems :: MetricActiveViewPercentVisibleSecondQuar , "METRIC_ACTIVE_VIEW_PERCENT_VISIBLE_THIRD_QUAR" => ParametersMetricsItems :: MetricActiveViewPercentVisibleThirdQuar , "METRIC_ACTIVE_VIEW_UNMEASURABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewUnmeasurableImpressions , "METRIC_ACTIVE_VIEW_UNVIEWABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewUnviewableImpressions , "METRIC_ACTIVE_VIEW_VIEWABLE_FOR_TIME_THRESHOLD" => ParametersMetricsItems :: MetricActiveViewViewableForTimeThreshold , "METRIC_ACTIVE_VIEW_VIEWABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricActiveViewViewableImpressions , "METRIC_ACTIVITY_REVENUE" => ParametersMetricsItems :: MetricActivityRevenue , "METRIC_ADAPTED_AUDIENCE_FREQUENCY" => ParametersMetricsItems :: MetricAdaptedAudienceFrequency , "METRIC_ADLINGO_FEE_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricAdlingoFeeAdvertiserCurrency , "METRIC_AUDIO_CLIENT_COST_ECPCL_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricAudioClientCostEcpclAdvertiserCurrency , "METRIC_AUDIO_MEDIA_COST_ECPCL_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricAudioMediaCostEcpclAdvertiserCurrency , "METRIC_AUDIO_MUTES_AUDIO" => ParametersMetricsItems :: MetricAudioMutesAudio , "METRIC_AUDIO_REVENUE_ECPCL_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricAudioRevenueEcpclAdvertiserCurrency , "METRIC_AUDIO_UNMUTES_AUDIO" => ParametersMetricsItems :: MetricAudioUnmutesAudio , "METRIC_AUDIO_UNMUTES_VIDEO" => ParametersMetricsItems :: MetricAudioUnmutesVideo , "METRIC_AVERAGE_DISPLAY_TIME" => ParametersMetricsItems :: MetricAverageDisplayTime , "METRIC_AVERAGE_IMPRESSION_FREQUENCY_PER_USER" => ParametersMetricsItems :: MetricAverageImpressionFrequencyPerUser , "METRIC_AVERAGE_INTERACTION_TIME" => ParametersMetricsItems :: MetricAverageInteractionTime , "METRIC_AVERAGE_WATCH_TIME_PER_IMPRESSION" => ParametersMetricsItems :: MetricAverageWatchTimePerImpression , "METRIC_BEGIN_TO_RENDER_ELIGIBLE_IMPRESSIONS" => ParametersMetricsItems :: MetricBeginToRenderEligibleImpressions , "METRIC_BEGIN_TO_RENDER_IMPRESSIONS" => ParametersMetricsItems :: MetricBeginToRenderImpressions , "METRIC_BENCHMARK_FREQUENCY" => ParametersMetricsItems :: MetricBenchmarkFrequency , "METRIC_BID_REQUESTS" => ParametersMetricsItems :: MetricBidRequests , "METRIC_BILLABLE_COST_ADVERTISER" => ParametersMetricsItems :: MetricBillableCostAdvertiser , "METRIC_BILLABLE_COST_PARTNER" => ParametersMetricsItems :: MetricBillableCostPartner , "METRIC_BILLABLE_COST_USD" => ParametersMetricsItems :: MetricBillableCostUsd , "METRIC_BILLABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricBillableImpressions , "METRIC_BRAND_LIFT_ABSOLUTE_BRAND_LIFT" => ParametersMetricsItems :: MetricBrandLiftAbsoluteBrandLift , "METRIC_BRAND_LIFT_ALL_SURVEY_RESPONSES" => ParametersMetricsItems :: MetricBrandLiftAllSurveyResponses , "METRIC_BRAND_LIFT_BASELINE_POSITIVE_RESPONSE_RATE" => ParametersMetricsItems :: MetricBrandLiftBaselinePositiveResponseRate , "METRIC_BRAND_LIFT_BASELINE_SURVEY_RESPONSES" => ParametersMetricsItems :: MetricBrandLiftBaselineSurveyResponses , "METRIC_BRAND_LIFT_COST_PER_LIFTED_USER" => ParametersMetricsItems :: MetricBrandLiftCostPerLiftedUser , "METRIC_BRAND_LIFT_EXPOSED_SURVEY_RESPONSES" => ParametersMetricsItems :: MetricBrandLiftExposedSurveyResponses , "METRIC_BRAND_LIFT_HEADROOM_BRAND_LIFT" => ParametersMetricsItems :: MetricBrandLiftHeadroomBrandLift , "METRIC_BRAND_LIFT_RELATIVE_BRAND_LIFT" => ParametersMetricsItems :: MetricBrandLiftRelativeBrandLift , "METRIC_BRAND_LIFT_USERS" => ParametersMetricsItems :: MetricBrandLiftUsers , "METRIC_CARD_CLICKS" => ParametersMetricsItems :: MetricCardClicks , "METRIC_CLICK_TO_POST_CLICK_CONVERSION_RATE" => ParametersMetricsItems :: MetricClickToPostClickConversionRate , "METRIC_CLICKS" => ParametersMetricsItems :: MetricClicks , "METRIC_CLIENT_COST_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricClientCostAdvertiserCurrency , "METRIC_CLIENT_COST_ECPA_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricClientCostEcpaAdvertiserCurrency , "METRIC_CLIENT_COST_ECPA_PC_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricClientCostEcpaPcAdvertiserCurrency , "METRIC_CLIENT_COST_ECPA_PV_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricClientCostEcpaPvAdvertiserCurrency , "METRIC_CLIENT_COST_ECPC_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricClientCostEcpcAdvertiserCurrency , "METRIC_CLIENT_COST_ECPM_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricClientCostEcpmAdvertiserCurrency , "METRIC_CLIENT_COST_VIEWABLE_ECPM_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricClientCostViewableEcpmAdvertiserCurrency , "METRIC_CM360_POST_CLICK_REVENUE" => ParametersMetricsItems :: MetricCm360PostClickRevenue , "METRIC_CM360_POST_CLICK_REVENUE_CROSS_ENVIRONMENT" => ParametersMetricsItems :: MetricCm360PostClickRevenueCrossEnvironment , "METRIC_CM360_POST_VIEW_REVENUE" => ParametersMetricsItems :: MetricCm360PostViewRevenue , "METRIC_CM360_POST_VIEW_REVENUE_CROSS_ENVIRONMENT" => ParametersMetricsItems :: MetricCm360PostViewRevenueCrossEnvironment , "METRIC_CM_POST_CLICK_REVENUE" => ParametersMetricsItems :: MetricCmPostClickRevenue , "METRIC_CM_POST_CLICK_REVENUE_CROSS_ENVIRONMENT" => ParametersMetricsItems :: MetricCmPostClickRevenueCrossEnvironment , "METRIC_CM_POST_VIEW_REVENUE" => ParametersMetricsItems :: MetricCmPostViewRevenue , "METRIC_CM_POST_VIEW_REVENUE_CROSS_ENVIRONMENT" => ParametersMetricsItems :: MetricCmPostViewRevenueCrossEnvironment , "METRIC_COMPANION_CLICKS_AUDIO" => ParametersMetricsItems :: MetricCompanionClicksAudio , "METRIC_COMPANION_IMPRESSIONS_AUDIO" => ParametersMetricsItems :: MetricCompanionImpressionsAudio , "METRIC_COMPLETE_LISTENS_AUDIO" => ParametersMetricsItems :: MetricCompleteListensAudio , "METRIC_COMPLETION_RATE_AUDIO" => ParametersMetricsItems :: MetricCompletionRateAudio , "METRIC_CONVERSIONS_PER_MILLE" => ParametersMetricsItems :: MetricConversionsPerMille , "METRIC_CONVERTING_PATHS" => ParametersMetricsItems :: MetricConvertingPaths , "METRIC_COOKIE_CONSENTED_FLOODLIGHT_IMPRESSIONS" => ParametersMetricsItems :: MetricCookieConsentedFloodlightImpressions , "METRIC_COOKIE_REACH_AVERAGE_IMPRESSION_FREQUENCY" => ParametersMetricsItems :: MetricCookieReachAverageImpressionFrequency , "METRIC_COOKIE_REACH_IMPRESSION_REACH" => ParametersMetricsItems :: MetricCookieReachImpressionReach , "METRIC_COOKIE_UNCONSENTED_FLOODLIGHT_IMPRESSIONS" => ParametersMetricsItems :: MetricCookieUnconsentedFloodlightImpressions , "METRIC_COUNTERS" => ParametersMetricsItems :: MetricCounters , "METRIC_CPM_FEE1_ADVERTISER" => ParametersMetricsItems :: MetricCpmFee1Advertiser , "METRIC_CPM_FEE1_PARTNER" => ParametersMetricsItems :: MetricCpmFee1Partner , "METRIC_CPM_FEE1_USD" => ParametersMetricsItems :: MetricCpmFee1Usd , "METRIC_CPM_FEE2_ADVERTISER" => ParametersMetricsItems :: MetricCpmFee2Advertiser , "METRIC_CPM_FEE2_PARTNER" => ParametersMetricsItems :: MetricCpmFee2Partner , "METRIC_CPM_FEE2_USD" => ParametersMetricsItems :: MetricCpmFee2Usd , "METRIC_CPM_FEE3_ADVERTISER" => ParametersMetricsItems :: MetricCpmFee3Advertiser , "METRIC_CPM_FEE3_PARTNER" => ParametersMetricsItems :: MetricCpmFee3Partner , "METRIC_CPM_FEE3_USD" => ParametersMetricsItems :: MetricCpmFee3Usd , "METRIC_CPM_FEE4_ADVERTISER" => ParametersMetricsItems :: MetricCpmFee4Advertiser , "METRIC_CPM_FEE4_PARTNER" => ParametersMetricsItems :: MetricCpmFee4Partner , "METRIC_CPM_FEE4_USD" => ParametersMetricsItems :: MetricCpmFee4Usd , "METRIC_CPM_FEE5_ADVERTISER" => ParametersMetricsItems :: MetricCpmFee5Advertiser , "METRIC_CPM_FEE5_PARTNER" => ParametersMetricsItems :: MetricCpmFee5Partner , "METRIC_CPM_FEE5_USD" => ParametersMetricsItems :: MetricCpmFee5Usd , "METRIC_CTR" => ParametersMetricsItems :: MetricCtr , "METRIC_CUSTOM_FEE_1_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricCustomFee1AdvertiserCurrency , "METRIC_CUSTOM_FEE_2_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricCustomFee2AdvertiserCurrency , "METRIC_CUSTOM_FEE_3_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricCustomFee3AdvertiserCurrency , "METRIC_CUSTOM_FEE_4_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricCustomFee4AdvertiserCurrency , "METRIC_CUSTOM_FEE_5_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricCustomFee5AdvertiserCurrency , "METRIC_CUSTOM_VALUE_PER_1000_IMPRESSIONS" => ParametersMetricsItems :: MetricCustomValuePer1000Impressions , "METRIC_DATA_COST_ADVERTISER" => ParametersMetricsItems :: MetricDataCostAdvertiser , "METRIC_DATA_COST_PARTNER" => ParametersMetricsItems :: MetricDataCostPartner , "METRIC_DATA_COST_USD" => ParametersMetricsItems :: MetricDataCostUsd , "METRIC_DBM_ENGAGEMENT_RATE" => ParametersMetricsItems :: MetricDbmEngagementRate , "METRIC_DEMO_COMPOSITION_IMPRESSION" => ParametersMetricsItems :: MetricDemoCompositionImpression , "METRIC_DEMO_CORRECTED_CLICKS" => ParametersMetricsItems :: MetricDemoCorrectedClicks , "METRIC_DEMO_POPULATION" => ParametersMetricsItems :: MetricDemoPopulation , "METRIC_DUPLICATE_FLOODLIGHT_IMPRESSIONS" => ParametersMetricsItems :: MetricDuplicateFloodlightImpressions , "METRIC_ENGAGEMENT_RATE" => ParametersMetricsItems :: MetricEngagementRate , "METRIC_ENGAGEMENTS" => ParametersMetricsItems :: MetricEngagements , "METRIC_ESTIMATED_CPM_FOR_IMPRESSIONS_WITH_CUSTOM_VALUE_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricEstimatedCpmForImpressionsWithCustomValueAdvertiserCurrency , "METRIC_ESTIMATED_TOTAL_COST_FOR_IMPRESSIONS_WITH_CUSTOM_VALUE_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricEstimatedTotalCostForImpressionsWithCustomValueAdvertiserCurrency , "METRIC_EXITS" => ParametersMetricsItems :: MetricExits , "METRIC_EXPANSIONS" => ParametersMetricsItems :: MetricExpansions , "METRIC_FEE10_ADVERTISER" => ParametersMetricsItems :: MetricFee10Advertiser , "METRIC_FEE10_PARTNER" => ParametersMetricsItems :: MetricFee10Partner , "METRIC_FEE10_USD" => ParametersMetricsItems :: MetricFee10Usd , "METRIC_FEE11_ADVERTISER" => ParametersMetricsItems :: MetricFee11Advertiser , "METRIC_FEE11_PARTNER" => ParametersMetricsItems :: MetricFee11Partner , "METRIC_FEE11_USD" => ParametersMetricsItems :: MetricFee11Usd , "METRIC_FEE12_ADVERTISER" => ParametersMetricsItems :: MetricFee12Advertiser , "METRIC_FEE12_PARTNER" => ParametersMetricsItems :: MetricFee12Partner , "METRIC_FEE12_USD" => ParametersMetricsItems :: MetricFee12Usd , "METRIC_FEE13_ADVERTISER" => ParametersMetricsItems :: MetricFee13Advertiser , "METRIC_FEE13_PARTNER" => ParametersMetricsItems :: MetricFee13Partner , "METRIC_FEE13_USD" => ParametersMetricsItems :: MetricFee13Usd , "METRIC_FEE14_ADVERTISER" => ParametersMetricsItems :: MetricFee14Advertiser , "METRIC_FEE14_PARTNER" => ParametersMetricsItems :: MetricFee14Partner , "METRIC_FEE14_USD" => ParametersMetricsItems :: MetricFee14Usd , "METRIC_FEE15_ADVERTISER" => ParametersMetricsItems :: MetricFee15Advertiser , "METRIC_FEE15_PARTNER" => ParametersMetricsItems :: MetricFee15Partner , "METRIC_FEE15_USD" => ParametersMetricsItems :: MetricFee15Usd , "METRIC_FEE16_ADVERTISER" => ParametersMetricsItems :: MetricFee16Advertiser , "METRIC_FEE16_PARTNER" => ParametersMetricsItems :: MetricFee16Partner , "METRIC_FEE16_USD" => ParametersMetricsItems :: MetricFee16Usd , "METRIC_FEE17_ADVERTISER" => ParametersMetricsItems :: MetricFee17Advertiser , "METRIC_FEE17_PARTNER" => ParametersMetricsItems :: MetricFee17Partner , "METRIC_FEE17_USD" => ParametersMetricsItems :: MetricFee17Usd , "METRIC_FEE18_ADVERTISER" => ParametersMetricsItems :: MetricFee18Advertiser , "METRIC_FEE18_PARTNER" => ParametersMetricsItems :: MetricFee18Partner , "METRIC_FEE18_USD" => ParametersMetricsItems :: MetricFee18Usd , "METRIC_FEE19_ADVERTISER" => ParametersMetricsItems :: MetricFee19Advertiser , "METRIC_FEE19_PARTNER" => ParametersMetricsItems :: MetricFee19Partner , "METRIC_FEE19_USD" => ParametersMetricsItems :: MetricFee19Usd , "METRIC_FEE20_ADVERTISER" => ParametersMetricsItems :: MetricFee20Advertiser , "METRIC_FEE20_PARTNER" => ParametersMetricsItems :: MetricFee20Partner , "METRIC_FEE20_USD" => ParametersMetricsItems :: MetricFee20Usd , "METRIC_FEE21_ADVERTISER" => ParametersMetricsItems :: MetricFee21Advertiser , "METRIC_FEE21_PARTNER" => ParametersMetricsItems :: MetricFee21Partner , "METRIC_FEE21_USD" => ParametersMetricsItems :: MetricFee21Usd , "METRIC_FEE22_ADVERTISER" => ParametersMetricsItems :: MetricFee22Advertiser , "METRIC_FEE22_PARTNER" => ParametersMetricsItems :: MetricFee22Partner , "METRIC_FEE22_USD" => ParametersMetricsItems :: MetricFee22Usd , "METRIC_FEE2_ADVERTISER" => ParametersMetricsItems :: MetricFee2Advertiser , "METRIC_FEE2_PARTNER" => ParametersMetricsItems :: MetricFee2Partner , "METRIC_FEE2_USD" => ParametersMetricsItems :: MetricFee2Usd , "METRIC_FEE3_ADVERTISER" => ParametersMetricsItems :: MetricFee3Advertiser , "METRIC_FEE3_PARTNER" => ParametersMetricsItems :: MetricFee3Partner , "METRIC_FEE3_USD" => ParametersMetricsItems :: MetricFee3Usd , "METRIC_FEE4_ADVERTISER" => ParametersMetricsItems :: MetricFee4Advertiser , "METRIC_FEE4_PARTNER" => ParametersMetricsItems :: MetricFee4Partner , "METRIC_FEE4_USD" => ParametersMetricsItems :: MetricFee4Usd , "METRIC_FEE5_ADVERTISER" => ParametersMetricsItems :: MetricFee5Advertiser , "METRIC_FEE5_PARTNER" => ParametersMetricsItems :: MetricFee5Partner , "METRIC_FEE5_USD" => ParametersMetricsItems :: MetricFee5Usd , "METRIC_FEE6_ADVERTISER" => ParametersMetricsItems :: MetricFee6Advertiser , "METRIC_FEE6_PARTNER" => ParametersMetricsItems :: MetricFee6Partner , "METRIC_FEE6_USD" => ParametersMetricsItems :: MetricFee6Usd , "METRIC_FEE7_ADVERTISER" => ParametersMetricsItems :: MetricFee7Advertiser , "METRIC_FEE7_PARTNER" => ParametersMetricsItems :: MetricFee7Partner , "METRIC_FEE7_USD" => ParametersMetricsItems :: MetricFee7Usd , "METRIC_FEE8_ADVERTISER" => ParametersMetricsItems :: MetricFee8Advertiser , "METRIC_FEE8_PARTNER" => ParametersMetricsItems :: MetricFee8Partner , "METRIC_FEE8_USD" => ParametersMetricsItems :: MetricFee8Usd , "METRIC_FEE9_ADVERTISER" => ParametersMetricsItems :: MetricFee9Advertiser , "METRIC_FEE9_PARTNER" => ParametersMetricsItems :: MetricFee9Partner , "METRIC_FEE9_USD" => ParametersMetricsItems :: MetricFee9Usd , "METRIC_FIRST_QUARTILE_AUDIO" => ParametersMetricsItems :: MetricFirstQuartileAudio , "METRIC_FLOODLIGHT_IMPRESSIONS" => ParametersMetricsItems :: MetricFloodlightImpressions , "METRIC_GENERAL_INVALID_TRAFFIC_GIVT_IMPRESSIONS" => ParametersMetricsItems :: MetricGeneralInvalidTrafficGivtImpressions , "METRIC_GENERAL_INVALID_TRAFFIC_GIVT_TRACKED_ADS" => ParametersMetricsItems :: MetricGeneralInvalidTrafficGivtTrackedAds , "METRIC_GIVT_ACTIVE_VIEW_ELIGIBLE_IMPRESSIONS" => ParametersMetricsItems :: MetricGivtActiveViewEligibleImpressions , "METRIC_GIVT_ACTIVE_VIEW_MEASURABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricGivtActiveViewMeasurableImpressions , "METRIC_GIVT_ACTIVE_VIEW_VIEWABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricGivtActiveViewViewableImpressions , "METRIC_GIVT_BEGIN_TO_RENDER_IMPRESSIONS" => ParametersMetricsItems :: MetricGivtBeginToRenderImpressions , "METRIC_GIVT_CLICKS" => ParametersMetricsItems :: MetricGivtClicks , "METRIC_GMAIL_CONVERSIONS" => ParametersMetricsItems :: MetricGmailConversions , "METRIC_GMAIL_POST_CLICK_CONVERSIONS" => ParametersMetricsItems :: MetricGmailPostClickConversions , "METRIC_GMAIL_POST_VIEW_CONVERSIONS" => ParametersMetricsItems :: MetricGmailPostViewConversions , "METRIC_GMAIL_POTENTIAL_VIEWS" => ParametersMetricsItems :: MetricGmailPotentialViews , "METRIC_GRP_CORRECTED_IMPRESSIONS" => ParametersMetricsItems :: MetricGrpCorrectedImpressions , "METRIC_GRP_CORRECTED_VIEWABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricGrpCorrectedViewableImpressions , "METRIC_GRP_CORRECTED_VIEWABLE_IMPRESSIONS_SHARE_PERCENT" => ParametersMetricsItems :: MetricGrpCorrectedViewableImpressionsSharePercent , "METRIC_IMPRESSION_CUSTOM_VALUE_COST" => ParametersMetricsItems :: MetricImpressionCustomValueCost , "METRIC_IMPRESSION_LOSS_TARGETED_IMPRESSIONS" => ParametersMetricsItems :: MetricImpressionLossTargetedImpressions , "METRIC_IMPRESSIONS" => ParametersMetricsItems :: MetricImpressions , "METRIC_IMPRESSIONS_TO_CONVERSION_RATE" => ParametersMetricsItems :: MetricImpressionsToConversionRate , "METRIC_IMPRESSIONS_WITH_CUSTOM_VALUE" => ParametersMetricsItems :: MetricImpressionsWithCustomValue , "METRIC_IMPRESSIONS_WITH_POSITIVE_CUSTOM_VALUE" => ParametersMetricsItems :: MetricImpressionsWithPositiveCustomValue , "METRIC_INTERACTIVE_IMPRESSIONS" => ParametersMetricsItems :: MetricInteractiveImpressions , "METRIC_INVALID_ACTIVE_VIEW_ELIGIBLE_IMPRESSIONS" => ParametersMetricsItems :: MetricInvalidActiveViewEligibleImpressions , "METRIC_INVALID_ACTIVE_VIEW_MEASURABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricInvalidActiveViewMeasurableImpressions , "METRIC_INVALID_ACTIVE_VIEW_VIEWABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricInvalidActiveViewViewableImpressions , "METRIC_INVALID_BEGIN_TO_RENDER_IMPRESSIONS" => ParametersMetricsItems :: MetricInvalidBeginToRenderImpressions , "METRIC_INVALID_CLICKS" => ParametersMetricsItems :: MetricInvalidClicks , "METRIC_INVALID_IMPRESSIONS" => ParametersMetricsItems :: MetricInvalidImpressions , "METRIC_INVALID_TRACKED_ADS" => ParametersMetricsItems :: MetricInvalidTrackedAds , "METRIC_LAST_CLICKS" => ParametersMetricsItems :: MetricLastClicks , "METRIC_LAST_IMPRESSIONS" => ParametersMetricsItems :: MetricLastImpressions , "METRIC_LAST_TOUCH_CLICK_THROUGH_CONVERSIONS" => ParametersMetricsItems :: MetricLastTouchClickThroughConversions , "METRIC_LAST_TOUCH_TOTAL_CONVERSIONS" => ParametersMetricsItems :: MetricLastTouchTotalConversions , "METRIC_LAST_TOUCH_VIEW_THROUGH_CONVERSIONS" => ParametersMetricsItems :: MetricLastTouchViewThroughConversions , "METRIC_LINEITEM_BID_RESPONSE_COUNT" => ParametersMetricsItems :: MetricLineitemBidResponseCount , "METRIC_MEDIA_COST_ADVERTISER" => ParametersMetricsItems :: MetricMediaCostAdvertiser , "METRIC_MEDIA_COST_ECPA_ADVERTISER" => ParametersMetricsItems :: MetricMediaCostEcpaAdvertiser , "METRIC_MEDIA_COST_ECPA_PARTNER" => ParametersMetricsItems :: MetricMediaCostEcpaPartner , "METRIC_MEDIA_COST_ECPA_USD" => ParametersMetricsItems :: MetricMediaCostEcpaUsd , "METRIC_MEDIA_COST_ECPAPC_ADVERTISER" => ParametersMetricsItems :: MetricMediaCostEcpapcAdvertiser , "METRIC_MEDIA_COST_ECPAPC_PARTNER" => ParametersMetricsItems :: MetricMediaCostEcpapcPartner , "METRIC_MEDIA_COST_ECPAPC_USD" => ParametersMetricsItems :: MetricMediaCostEcpapcUsd , "METRIC_MEDIA_COST_ECPAPV_ADVERTISER" => ParametersMetricsItems :: MetricMediaCostEcpapvAdvertiser , "METRIC_MEDIA_COST_ECPAPV_PARTNER" => ParametersMetricsItems :: MetricMediaCostEcpapvPartner , "METRIC_MEDIA_COST_ECPAPV_USD" => ParametersMetricsItems :: MetricMediaCostEcpapvUsd , "METRIC_MEDIA_COST_ECPC_ADVERTISER" => ParametersMetricsItems :: MetricMediaCostEcpcAdvertiser , "METRIC_MEDIA_COST_ECPC_PARTNER" => ParametersMetricsItems :: MetricMediaCostEcpcPartner , "METRIC_MEDIA_COST_ECPC_USD" => ParametersMetricsItems :: MetricMediaCostEcpcUsd , "METRIC_MEDIA_COST_ECPCV_ADVERTISER" => ParametersMetricsItems :: MetricMediaCostEcpcvAdvertiser , "METRIC_MEDIA_COST_ECPCV_PARTNER" => ParametersMetricsItems :: MetricMediaCostEcpcvPartner , "METRIC_MEDIA_COST_ECPCV_USD" => ParametersMetricsItems :: MetricMediaCostEcpcvUsd , "METRIC_MEDIA_COST_ECPM_ADVERTISER" => ParametersMetricsItems :: MetricMediaCostEcpmAdvertiser , "METRIC_MEDIA_COST_ECPM_PARTNER" => ParametersMetricsItems :: MetricMediaCostEcpmPartner , "METRIC_MEDIA_COST_ECPM_USD" => ParametersMetricsItems :: MetricMediaCostEcpmUsd , "METRIC_MEDIA_COST_PARTNER" => ParametersMetricsItems :: MetricMediaCostPartner , "METRIC_MEDIA_COST_USD" => ParametersMetricsItems :: MetricMediaCostUsd , "METRIC_MEDIA_COST_VIEWABLE_ECPM_ADVERTISER" => ParametersMetricsItems :: MetricMediaCostViewableEcpmAdvertiser , "METRIC_MEDIA_COST_VIEWABLE_ECPM_PARTNER" => ParametersMetricsItems :: MetricMediaCostViewableEcpmPartner , "METRIC_MEDIA_COST_VIEWABLE_ECPM_USD" => ParametersMetricsItems :: MetricMediaCostViewableEcpmUsd , "METRIC_MEDIA_FEE1_ADVERTISER" => ParametersMetricsItems :: MetricMediaFee1Advertiser , "METRIC_MEDIA_FEE1_PARTNER" => ParametersMetricsItems :: MetricMediaFee1Partner , "METRIC_MEDIA_FEE1_USD" => ParametersMetricsItems :: MetricMediaFee1Usd , "METRIC_MEDIA_FEE2_ADVERTISER" => ParametersMetricsItems :: MetricMediaFee2Advertiser , "METRIC_MEDIA_FEE2_PARTNER" => ParametersMetricsItems :: MetricMediaFee2Partner , "METRIC_MEDIA_FEE2_USD" => ParametersMetricsItems :: MetricMediaFee2Usd , "METRIC_MEDIA_FEE3_ADVERTISER" => ParametersMetricsItems :: MetricMediaFee3Advertiser , "METRIC_MEDIA_FEE3_PARTNER" => ParametersMetricsItems :: MetricMediaFee3Partner , "METRIC_MEDIA_FEE3_USD" => ParametersMetricsItems :: MetricMediaFee3Usd , "METRIC_MEDIA_FEE4_ADVERTISER" => ParametersMetricsItems :: MetricMediaFee4Advertiser , "METRIC_MEDIA_FEE4_PARTNER" => ParametersMetricsItems :: MetricMediaFee4Partner , "METRIC_MEDIA_FEE4_USD" => ParametersMetricsItems :: MetricMediaFee4Usd , "METRIC_MEDIA_FEE5_ADVERTISER" => ParametersMetricsItems :: MetricMediaFee5Advertiser , "METRIC_MEDIA_FEE5_PARTNER" => ParametersMetricsItems :: MetricMediaFee5Partner , "METRIC_MEDIA_FEE5_USD" => ParametersMetricsItems :: MetricMediaFee5Usd , "METRIC_MIDPOINT_AUDIO" => ParametersMetricsItems :: MetricMidpointAudio , "METRIC_NIELSEN_AVERAGE_FREQUENCY" => ParametersMetricsItems :: MetricNielsenAverageFrequency , "METRIC_NIELSEN_GRP" => ParametersMetricsItems :: MetricNielsenGrp , "METRIC_NIELSEN_IMPRESSION_INDEX" => ParametersMetricsItems :: MetricNielsenImpressionIndex , "METRIC_NIELSEN_IMPRESSIONS" => ParametersMetricsItems :: MetricNielsenImpressions , "METRIC_NIELSEN_IMPRESSIONS_SHARE" => ParametersMetricsItems :: MetricNielsenImpressionsShare , "METRIC_NIELSEN_POPULATION" => ParametersMetricsItems :: MetricNielsenPopulation , "METRIC_NIELSEN_POPULATION_REACH" => ParametersMetricsItems :: MetricNielsenPopulationReach , "METRIC_NIELSEN_POPULATION_SHARE" => ParametersMetricsItems :: MetricNielsenPopulationShare , "METRIC_NIELSEN_REACH_INDEX" => ParametersMetricsItems :: MetricNielsenReachIndex , "METRIC_NIELSEN_REACH_SHARE" => ParametersMetricsItems :: MetricNielsenReachShare , "METRIC_NIELSEN_UNIQUE_AUDIENCE" => ParametersMetricsItems :: MetricNielsenUniqueAudience , "METRIC_ORIGINAL_AUDIENCE_FREQUENCY" => ParametersMetricsItems :: MetricOriginalAudienceFrequency , "METRIC_PATH_CONVERSION_RATE" => ParametersMetricsItems :: MetricPathConversionRate , "METRIC_PAUSES_AUDIO" => ParametersMetricsItems :: MetricPausesAudio , "METRIC_PERCENT_IMPRESSIONS_WITH_POSITIVE_CUSTOM_VALUE" => ParametersMetricsItems :: MetricPercentImpressionsWithPositiveCustomValue , "METRIC_PERCENT_INVALID_IMPRESSIONS_PREBID" => ParametersMetricsItems :: MetricPercentInvalidImpressionsPrebid , "METRIC_PERCENTAGE_FROM_CURRENT_IO_GOAL" => ParametersMetricsItems :: MetricPercentageFromCurrentIoGoal , "METRIC_PLATFORM_FEE_ADVERTISER" => ParametersMetricsItems :: MetricPlatformFeeAdvertiser , "METRIC_PLATFORM_FEE_PARTNER" => ParametersMetricsItems :: MetricPlatformFeePartner , "METRIC_PLATFORM_FEE_RATE" => ParametersMetricsItems :: MetricPlatformFeeRate , "METRIC_PLATFORM_FEE_USD" => ParametersMetricsItems :: MetricPlatformFeeUsd , "METRIC_POST_CLICK_CONVERSIONS_CROSS_ENVIRONMENT" => ParametersMetricsItems :: MetricPostClickConversionsCrossEnvironment , "METRIC_POST_VIEW_CONVERSIONS_CROSS_ENVIRONMENT" => ParametersMetricsItems :: MetricPostViewConversionsCrossEnvironment , "METRIC_POTENTIAL_IMPRESSIONS" => ParametersMetricsItems :: MetricPotentialImpressions , "METRIC_POTENTIAL_VIEWS" => ParametersMetricsItems :: MetricPotentialViews , "METRIC_PREMIUM_FEE_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricPremiumFeeAdvertiserCurrency , "METRIC_PROFIT_ADVERTISER" => ParametersMetricsItems :: MetricProfitAdvertiser , "METRIC_PROFIT_ECPM_ADVERTISER" => ParametersMetricsItems :: MetricProfitEcpmAdvertiser , "METRIC_PROFIT_ECPM_PARTNER" => ParametersMetricsItems :: MetricProfitEcpmPartner , "METRIC_PROFIT_ECPM_USD" => ParametersMetricsItems :: MetricProfitEcpmUsd , "METRIC_PROFIT_MARGIN" => ParametersMetricsItems :: MetricProfitMargin , "METRIC_PROFIT_PARTNER" => ParametersMetricsItems :: MetricProfitPartner , "METRIC_PROFIT_USD" => ParametersMetricsItems :: MetricProfitUsd , "METRIC_PROFIT_VIEWABLE_ECPM_ADVERTISER" => ParametersMetricsItems :: MetricProfitViewableEcpmAdvertiser , "METRIC_PROFIT_VIEWABLE_ECPM_PARTNER" => ParametersMetricsItems :: MetricProfitViewableEcpmPartner , "METRIC_PROFIT_VIEWABLE_ECPM_USD" => ParametersMetricsItems :: MetricProfitViewableEcpmUsd , "METRIC_PROGRAMMATIC_GUARANTEED_IMPRESSIONS_PASSED_DUE_TO_FREQUENCY" => ParametersMetricsItems :: MetricProgrammaticGuaranteedImpressionsPassedDueToFrequency , "METRIC_PROGRAMMATIC_GUARANTEED_SAVINGS_RE_INVESTED_DUE_TO_FREQUENCY_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricProgrammaticGuaranteedSavingsReInvestedDueToFrequencyAdvertiserCurrency , "METRIC_PROVISIONAL_IMPRESSIONS" => ParametersMetricsItems :: MetricProvisionalImpressions , "METRIC_REFUND_BILLABLE_COST_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricRefundBillableCostAdvertiserCurrency , "METRIC_REFUND_MEDIA_COST_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricRefundMediaCostAdvertiserCurrency , "METRIC_REFUND_PLATFORM_FEE_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricRefundPlatformFeeAdvertiserCurrency , "METRIC_REVENUE_ADVERTISER" => ParametersMetricsItems :: MetricRevenueAdvertiser , "METRIC_REVENUE_ECPA_ADVERTISER" => ParametersMetricsItems :: MetricRevenueEcpaAdvertiser , "METRIC_REVENUE_ECPA_PARTNER" => ParametersMetricsItems :: MetricRevenueEcpaPartner , "METRIC_REVENUE_ECPA_USD" => ParametersMetricsItems :: MetricRevenueEcpaUsd , "METRIC_REVENUE_ECPAPC_ADVERTISER" => ParametersMetricsItems :: MetricRevenueEcpapcAdvertiser , "METRIC_REVENUE_ECPAPC_PARTNER" => ParametersMetricsItems :: MetricRevenueEcpapcPartner , "METRIC_REVENUE_ECPAPC_USD" => ParametersMetricsItems :: MetricRevenueEcpapcUsd , "METRIC_REVENUE_ECPAPV_ADVERTISER" => ParametersMetricsItems :: MetricRevenueEcpapvAdvertiser , "METRIC_REVENUE_ECPAPV_PARTNER" => ParametersMetricsItems :: MetricRevenueEcpapvPartner , "METRIC_REVENUE_ECPAPV_USD" => ParametersMetricsItems :: MetricRevenueEcpapvUsd , "METRIC_REVENUE_ECPC_ADVERTISER" => ParametersMetricsItems :: MetricRevenueEcpcAdvertiser , "METRIC_REVENUE_ECPC_PARTNER" => ParametersMetricsItems :: MetricRevenueEcpcPartner , "METRIC_REVENUE_ECPC_USD" => ParametersMetricsItems :: MetricRevenueEcpcUsd , "METRIC_REVENUE_ECPCV_ADVERTISER" => ParametersMetricsItems :: MetricRevenueEcpcvAdvertiser , "METRIC_REVENUE_ECPCV_PARTNER" => ParametersMetricsItems :: MetricRevenueEcpcvPartner , "METRIC_REVENUE_ECPCV_USD" => ParametersMetricsItems :: MetricRevenueEcpcvUsd , "METRIC_REVENUE_ECPM_ADVERTISER" => ParametersMetricsItems :: MetricRevenueEcpmAdvertiser , "METRIC_REVENUE_ECPM_PARTNER" => ParametersMetricsItems :: MetricRevenueEcpmPartner , "METRIC_REVENUE_ECPM_USD" => ParametersMetricsItems :: MetricRevenueEcpmUsd , "METRIC_REVENUE_PARTNER" => ParametersMetricsItems :: MetricRevenuePartner , "METRIC_REVENUE_USD" => ParametersMetricsItems :: MetricRevenueUsd , "METRIC_REVENUE_VIEWABLE_ECPM_ADVERTISER" => ParametersMetricsItems :: MetricRevenueViewableEcpmAdvertiser , "METRIC_REVENUE_VIEWABLE_ECPM_PARTNER" => ParametersMetricsItems :: MetricRevenueViewableEcpmPartner , "METRIC_REVENUE_VIEWABLE_ECPM_USD" => ParametersMetricsItems :: MetricRevenueViewableEcpmUsd , "METRIC_RICH_MEDIA_ENGAGEMENTS" => ParametersMetricsItems :: MetricRichMediaEngagements , "METRIC_RICH_MEDIA_SCROLLS" => ParametersMetricsItems :: MetricRichMediaScrolls , "METRIC_RICH_MEDIA_VIDEO_COMPLETIONS" => ParametersMetricsItems :: MetricRichMediaVideoCompletions , "METRIC_RICH_MEDIA_VIDEO_FIRST_QUARTILE_COMPLETES" => ParametersMetricsItems :: MetricRichMediaVideoFirstQuartileCompletes , "METRIC_RICH_MEDIA_VIDEO_FULL_SCREENS" => ParametersMetricsItems :: MetricRichMediaVideoFullScreens , "METRIC_RICH_MEDIA_VIDEO_MIDPOINTS" => ParametersMetricsItems :: MetricRichMediaVideoMidpoints , "METRIC_RICH_MEDIA_VIDEO_MUTES" => ParametersMetricsItems :: MetricRichMediaVideoMutes , "METRIC_RICH_MEDIA_VIDEO_PAUSES" => ParametersMetricsItems :: MetricRichMediaVideoPauses , "METRIC_RICH_MEDIA_VIDEO_PLAYS" => ParametersMetricsItems :: MetricRichMediaVideoPlays , "METRIC_RICH_MEDIA_VIDEO_SKIPS" => ParametersMetricsItems :: MetricRichMediaVideoSkips , "METRIC_RICH_MEDIA_VIDEO_THIRD_QUARTILE_COMPLETES" => ParametersMetricsItems :: MetricRichMediaVideoThirdQuartileCompletes , "METRIC_STARTS_AUDIO" => ParametersMetricsItems :: MetricStartsAudio , "METRIC_STOPS_AUDIO" => ParametersMetricsItems :: MetricStopsAudio , "METRIC_STORE_VISIT_CONVERSIONS" => ParametersMetricsItems :: MetricStoreVisitConversions , "METRIC_TARGET_RATING_POINTS" => ParametersMetricsItems :: MetricTargetRatingPoints , "METRIC_TEA_TRUEVIEW_IMPRESSIONS" => ParametersMetricsItems :: MetricTeaTrueviewImpressions , "METRIC_TEA_TRUEVIEW_UNIQUE_COOKIES" => ParametersMetricsItems :: MetricTeaTrueviewUniqueCookies , "METRIC_THIRD_QUARTILE_AUDIO" => ParametersMetricsItems :: MetricThirdQuartileAudio , "METRIC_TIMERS" => ParametersMetricsItems :: MetricTimers , "METRIC_TOTAL_AUDIO_MEDIA_COST_ECPCL_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricTotalAudioMediaCostEcpclAdvertiserCurrency , "METRIC_TOTAL_CONVERSIONS" => ParametersMetricsItems :: MetricTotalConversions , "METRIC_TOTAL_CONVERSIONS_CROSS_ENVIRONMENT" => ParametersMetricsItems :: MetricTotalConversionsCrossEnvironment , "METRIC_TOTAL_DISPLAY_TIME" => ParametersMetricsItems :: MetricTotalDisplayTime , "METRIC_TOTAL_EXPOSURES" => ParametersMetricsItems :: MetricTotalExposures , "METRIC_TOTAL_IMPRESSION_CUSTOM_VALUE" => ParametersMetricsItems :: MetricTotalImpressionCustomValue , "METRIC_TOTAL_INTERACTION_TIME" => ParametersMetricsItems :: MetricTotalInteractionTime , "METRIC_TOTAL_MEDIA_COST_ADVERTISER" => ParametersMetricsItems :: MetricTotalMediaCostAdvertiser , "METRIC_TOTAL_MEDIA_COST_ECPA_ADVERTISER" => ParametersMetricsItems :: MetricTotalMediaCostEcpaAdvertiser , "METRIC_TOTAL_MEDIA_COST_ECPA_PARTNER" => ParametersMetricsItems :: MetricTotalMediaCostEcpaPartner , "METRIC_TOTAL_MEDIA_COST_ECPA_USD" => ParametersMetricsItems :: MetricTotalMediaCostEcpaUsd , "METRIC_TOTAL_MEDIA_COST_ECPAPC_ADVERTISER" => ParametersMetricsItems :: MetricTotalMediaCostEcpapcAdvertiser , "METRIC_TOTAL_MEDIA_COST_ECPAPC_PARTNER" => ParametersMetricsItems :: MetricTotalMediaCostEcpapcPartner , "METRIC_TOTAL_MEDIA_COST_ECPAPC_USD" => ParametersMetricsItems :: MetricTotalMediaCostEcpapcUsd , "METRIC_TOTAL_MEDIA_COST_ECPAPV_ADVERTISER" => ParametersMetricsItems :: MetricTotalMediaCostEcpapvAdvertiser , "METRIC_TOTAL_MEDIA_COST_ECPAPV_PARTNER" => ParametersMetricsItems :: MetricTotalMediaCostEcpapvPartner , "METRIC_TOTAL_MEDIA_COST_ECPAPV_USD" => ParametersMetricsItems :: MetricTotalMediaCostEcpapvUsd , "METRIC_TOTAL_MEDIA_COST_ECPC_ADVERTISER" => ParametersMetricsItems :: MetricTotalMediaCostEcpcAdvertiser , "METRIC_TOTAL_MEDIA_COST_ECPC_PARTNER" => ParametersMetricsItems :: MetricTotalMediaCostEcpcPartner , "METRIC_TOTAL_MEDIA_COST_ECPC_USD" => ParametersMetricsItems :: MetricTotalMediaCostEcpcUsd , "METRIC_TOTAL_MEDIA_COST_ECPCV_ADVERTISER" => ParametersMetricsItems :: MetricTotalMediaCostEcpcvAdvertiser , "METRIC_TOTAL_MEDIA_COST_ECPCV_PARTNER" => ParametersMetricsItems :: MetricTotalMediaCostEcpcvPartner , "METRIC_TOTAL_MEDIA_COST_ECPCV_USD" => ParametersMetricsItems :: MetricTotalMediaCostEcpcvUsd , "METRIC_TOTAL_MEDIA_COST_ECPM_ADVERTISER" => ParametersMetricsItems :: MetricTotalMediaCostEcpmAdvertiser , "METRIC_TOTAL_MEDIA_COST_ECPM_PARTNER" => ParametersMetricsItems :: MetricTotalMediaCostEcpmPartner , "METRIC_TOTAL_MEDIA_COST_ECPM_USD" => ParametersMetricsItems :: MetricTotalMediaCostEcpmUsd , "METRIC_TOTAL_MEDIA_COST_PARTNER" => ParametersMetricsItems :: MetricTotalMediaCostPartner , "METRIC_TOTAL_MEDIA_COST_USD" => ParametersMetricsItems :: MetricTotalMediaCostUsd , "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_ADVERTISER" => ParametersMetricsItems :: MetricTotalMediaCostViewableEcpmAdvertiser , "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_PARTNER" => ParametersMetricsItems :: MetricTotalMediaCostViewableEcpmPartner , "METRIC_TOTAL_MEDIA_COST_VIEWABLE_ECPM_USD" => ParametersMetricsItems :: MetricTotalMediaCostViewableEcpmUsd , "METRIC_TOTAL_PATHS" => ParametersMetricsItems :: MetricTotalPaths , "METRIC_TOTAL_USERS" => ParametersMetricsItems :: MetricTotalUsers , "METRIC_TRACKED_ADS" => ParametersMetricsItems :: MetricTrackedAds , "METRIC_TRACKING_UNCONSENTED_CLICKS" => ParametersMetricsItems :: MetricTrackingUnconsentedClicks , "METRIC_TRUEVIEW_ALL_AD_SEQUENCE_IMPRESSIONS" => ParametersMetricsItems :: MetricTrueviewAllAdSequenceImpressions , "METRIC_TRUEVIEW_AVERAGE_CPE_ADVERTISER" => ParametersMetricsItems :: MetricTrueviewAverageCpeAdvertiser , "METRIC_TRUEVIEW_AVERAGE_CPE_PARTNER" => ParametersMetricsItems :: MetricTrueviewAverageCpePartner , "METRIC_TRUEVIEW_AVERAGE_CPE_USD" => ParametersMetricsItems :: MetricTrueviewAverageCpeUsd , "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_ADVERTISER" => ParametersMetricsItems :: MetricTrueviewConversionCostManyPerViewAdvertiser , "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_PARTNER" => ParametersMetricsItems :: MetricTrueviewConversionCostManyPerViewPartner , "METRIC_TRUEVIEW_CONVERSION_COST_MANY_PER_VIEW_USD" => ParametersMetricsItems :: MetricTrueviewConversionCostManyPerViewUsd , "METRIC_TRUEVIEW_CONVERSION_MANY_PER_VIEW" => ParametersMetricsItems :: MetricTrueviewConversionManyPerView , "METRIC_TRUEVIEW_CONVERSION_RATE_ONE_PER_VIEW" => ParametersMetricsItems :: MetricTrueviewConversionRateOnePerView , "METRIC_TRUEVIEW_CPV_ADVERTISER" => ParametersMetricsItems :: MetricTrueviewCpvAdvertiser , "METRIC_TRUEVIEW_CPV_PARTNER" => ParametersMetricsItems :: MetricTrueviewCpvPartner , "METRIC_TRUEVIEW_CPV_USD" => ParametersMetricsItems :: MetricTrueviewCpvUsd , "METRIC_TRUEVIEW_EARNED_LIKES" => ParametersMetricsItems :: MetricTrueviewEarnedLikes , "METRIC_TRUEVIEW_EARNED_PLAYLIST_ADDITIONS" => ParametersMetricsItems :: MetricTrueviewEarnedPlaylistAdditions , "METRIC_TRUEVIEW_EARNED_SHARES" => ParametersMetricsItems :: MetricTrueviewEarnedShares , "METRIC_TRUEVIEW_EARNED_SUBSCRIBERS" => ParametersMetricsItems :: MetricTrueviewEarnedSubscribers , "METRIC_TRUEVIEW_EARNED_VIEWS" => ParametersMetricsItems :: MetricTrueviewEarnedViews , "METRIC_TRUEVIEW_ENGAGEMENT_RATE" => ParametersMetricsItems :: MetricTrueviewEngagementRate , "METRIC_TRUEVIEW_ENGAGEMENTS" => ParametersMetricsItems :: MetricTrueviewEngagements , "METRIC_TRUEVIEW_GENERAL_INVALID_TRAFFIC_GIVT_VIEWS" => ParametersMetricsItems :: MetricTrueviewGeneralInvalidTrafficGivtViews , "METRIC_TRUEVIEW_IMPRESSION_SHARE" => ParametersMetricsItems :: MetricTrueviewImpressionShare , "METRIC_TRUEVIEW_INVALID_VIEWS" => ParametersMetricsItems :: MetricTrueviewInvalidViews , "METRIC_TRUEVIEW_LOST_IS_BUDGET" => ParametersMetricsItems :: MetricTrueviewLostIsBudget , "METRIC_TRUEVIEW_LOST_IS_RANK" => ParametersMetricsItems :: MetricTrueviewLostIsRank , "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_ADVERTISER" => ParametersMetricsItems :: MetricTrueviewTotalConversionValuesAdvertiser , "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_PARTNER" => ParametersMetricsItems :: MetricTrueviewTotalConversionValuesPartner , "METRIC_TRUEVIEW_TOTAL_CONVERSION_VALUES_USD" => ParametersMetricsItems :: MetricTrueviewTotalConversionValuesUsd , "METRIC_TRUEVIEW_UNIQUE_VIEWERS" => ParametersMetricsItems :: MetricTrueviewUniqueViewers , "METRIC_TRUEVIEW_VIEW_RATE" => ParametersMetricsItems :: MetricTrueviewViewRate , "METRIC_TRUEVIEW_VIEW_THROUGH_CONVERSION" => ParametersMetricsItems :: MetricTrueviewViewThroughConversion , "METRIC_TRUEVIEW_VIEWS" => ParametersMetricsItems :: MetricTrueviewViews , "METRIC_UNIQUE_COOKIES_WITH_IMPRESSIONS" => ParametersMetricsItems :: MetricUniqueCookiesWithImpressions , "METRIC_UNIQUE_REACH_AVERAGE_IMPRESSION_FREQUENCY" => ParametersMetricsItems :: MetricUniqueReachAverageImpressionFrequency , "METRIC_UNIQUE_REACH_CLICK_REACH" => ParametersMetricsItems :: MetricUniqueReachClickReach , "METRIC_UNIQUE_REACH_IMPRESSION_REACH" => ParametersMetricsItems :: MetricUniqueReachImpressionReach , "METRIC_UNIQUE_REACH_TOTAL_REACH" => ParametersMetricsItems :: MetricUniqueReachTotalReach , "METRIC_UNIQUE_VISITORS_COOKIES" => ParametersMetricsItems :: MetricUniqueVisitorsCookies , "METRIC_UNKNOWN" => ParametersMetricsItems :: MetricUnknown , "METRIC_VENDOR_BLOCKED_ADS" => ParametersMetricsItems :: MetricVendorBlockedAds , "METRIC_VERIFIABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricVerifiableImpressions , "METRIC_VERIFICATION_VIDEO_PLAYER_SIZE_MEASURABLE_IMPRESSIONS" => ParametersMetricsItems :: MetricVerificationVideoPlayerSizeMeasurableImpressions , "METRIC_VIDEO_CLIENT_COST_ECPCV_ADVERTISER_CURRENCY" => ParametersMetricsItems :: MetricVideoClientCostEcpcvAdvertiserCurrency , "METRIC_VIDEO_COMPANION_CLICKS" => ParametersMetricsItems :: MetricVideoCompanionClicks , "METRIC_VIDEO_COMPANION_IMPRESSIONS" => ParametersMetricsItems :: MetricVideoCompanionImpressions , "METRIC_VIDEO_COMPLETION_RATE" => ParametersMetricsItems :: MetricVideoCompletionRate , "METRIC_VIEWABLE_BID_REQUESTS" => ParametersMetricsItems :: MetricViewableBidRequests , "METRIC_VIEWABLE_GROSS_RATING_POINTS" => ParametersMetricsItems :: MetricViewableGrossRatingPoints , "METRIC_VIRTUAL_PEOPLE_AVERAGE_IMPRESSION_FREQUENCY_BY_DEMO" => ParametersMetricsItems :: MetricVirtualPeopleAverageImpressionFrequencyByDemo , "METRIC_VIRTUAL_PEOPLE_AVERAGE_VIEWABLE_IMPRESSION_FREQUENCY_BY_DEMO" => ParametersMetricsItems :: MetricVirtualPeopleAverageViewableImpressionFrequencyByDemo , "METRIC_VIRTUAL_PEOPLE_CLICK_REACH_BY_DEMO" => ParametersMetricsItems :: MetricVirtualPeopleClickReachByDemo , "METRIC_VIRTUAL_PEOPLE_IMPRESSION_REACH_BY_DEMO" => ParametersMetricsItems :: MetricVirtualPeopleImpressionReachByDemo , "METRIC_VIRTUAL_PEOPLE_IMPRESSION_REACH_PERCENT" => ParametersMetricsItems :: MetricVirtualPeopleImpressionReachPercent , "METRIC_VIRTUAL_PEOPLE_IMPRESSION_REACH_SHARE_PERCENT" => ParametersMetricsItems :: MetricVirtualPeopleImpressionReachSharePercent , "METRIC_VIRTUAL_PEOPLE_VIEWABLE_IMPRESSION_REACH_BY_DEMO" => ParametersMetricsItems :: MetricVirtualPeopleViewableImpressionReachByDemo , "METRIC_VIRTUAL_PEOPLE_VIEWABLE_IMPRESSION_REACH_PERCENT" => ParametersMetricsItems :: MetricVirtualPeopleViewableImpressionReachPercent , "METRIC_VIRTUAL_PEOPLE_VIEWABLE_IMPRESSION_REACH_SHARE_PERCENT" => ParametersMetricsItems :: MetricVirtualPeopleViewableImpressionReachSharePercent , "METRIC_WATCH_TIME" => ParametersMetricsItems :: MetricWatchTime , "METRIC_WIN_LOSS_DEAL_AVAILABLE_REQUESTS" => ParametersMetricsItems :: MetricWinLossDealAvailableRequests , "METRIC_WIN_LOSS_DEAL_TARGETED_IMPRESSIONS" => ParametersMetricsItems :: MetricWinLossDealTargetedImpressions , "METRIC_WIN_LOSS_LINEITEM_AVAILABLE_REQUESTS" => ParametersMetricsItems :: MetricWinLossLineitemAvailableRequests , "METRIC_WIN_LOSS_LINEITEM_TARGETED_IMPRESSIONS" => ParametersMetricsItems :: MetricWinLossLineitemTargetedImpressions , "METRIC_WIN_LOSS_RATE" => ParametersMetricsItems :: MetricWinLossRate , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector for ParametersMetricsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ParametersMetricsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ParametersType {
        TypeActiveGrp,
        TypeAudienceComposition,
        TypeAudiencePerformance,
        TypeClientSafe,
        TypeComscoreVce,
        TypeCrossFee,
        TypeCrossPartner,
        TypeCrossPartnerThirdPartyDataProvider,
        TypeEstimatedConversion,
        TypeFee,
        TypeGeneral,
        TypeInventoryAvailability,
        TypeKeyword,
        TypeLinearTvSearchLift,
        TypeNielsenAudienceProfile,
        TypeNielsenDailyReachBuild,
        TypeNielsenOnlineGlobalMarket,
        TypeNielsenSite,
        TypeNotSupported,
        TypeOrderId,
        TypePageCategory,
        TypePath,
        TypePathAttribution,
        TypePetraNielsenAudienceProfile,
        TypePetraNielsenDailyReachBuild,
        TypePetraNielsenOnlineGlobalMarket,
        TypePixelLoad,
        TypeReachAndFrequency,
        TypeReachAudience,
        TypeThirdPartyDataProvider,
        TypeTrueview,
        TypeTrueviewIar,
        TypeVerification,
        TypeYoutubeVertical,
    }
    impl ParametersType {
        pub fn as_str(self) -> &'static str {
            match self {
                ParametersType::TypeActiveGrp => "TYPE_ACTIVE_GRP",
                ParametersType::TypeAudienceComposition => "TYPE_AUDIENCE_COMPOSITION",
                ParametersType::TypeAudiencePerformance => "TYPE_AUDIENCE_PERFORMANCE",
                ParametersType::TypeClientSafe => "TYPE_CLIENT_SAFE",
                ParametersType::TypeComscoreVce => "TYPE_COMSCORE_VCE",
                ParametersType::TypeCrossFee => "TYPE_CROSS_FEE",
                ParametersType::TypeCrossPartner => "TYPE_CROSS_PARTNER",
                ParametersType::TypeCrossPartnerThirdPartyDataProvider => {
                    "TYPE_CROSS_PARTNER_THIRD_PARTY_DATA_PROVIDER"
                }
                ParametersType::TypeEstimatedConversion => "TYPE_ESTIMATED_CONVERSION",
                ParametersType::TypeFee => "TYPE_FEE",
                ParametersType::TypeGeneral => "TYPE_GENERAL",
                ParametersType::TypeInventoryAvailability => "TYPE_INVENTORY_AVAILABILITY",
                ParametersType::TypeKeyword => "TYPE_KEYWORD",
                ParametersType::TypeLinearTvSearchLift => "TYPE_LINEAR_TV_SEARCH_LIFT",
                ParametersType::TypeNielsenAudienceProfile => "TYPE_NIELSEN_AUDIENCE_PROFILE",
                ParametersType::TypeNielsenDailyReachBuild => "TYPE_NIELSEN_DAILY_REACH_BUILD",
                ParametersType::TypeNielsenOnlineGlobalMarket => {
                    "TYPE_NIELSEN_ONLINE_GLOBAL_MARKET"
                }
                ParametersType::TypeNielsenSite => "TYPE_NIELSEN_SITE",
                ParametersType::TypeNotSupported => "TYPE_NOT_SUPPORTED",
                ParametersType::TypeOrderId => "TYPE_ORDER_ID",
                ParametersType::TypePageCategory => "TYPE_PAGE_CATEGORY",
                ParametersType::TypePath => "TYPE_PATH",
                ParametersType::TypePathAttribution => "TYPE_PATH_ATTRIBUTION",
                ParametersType::TypePetraNielsenAudienceProfile => {
                    "TYPE_PETRA_NIELSEN_AUDIENCE_PROFILE"
                }
                ParametersType::TypePetraNielsenDailyReachBuild => {
                    "TYPE_PETRA_NIELSEN_DAILY_REACH_BUILD"
                }
                ParametersType::TypePetraNielsenOnlineGlobalMarket => {
                    "TYPE_PETRA_NIELSEN_ONLINE_GLOBAL_MARKET"
                }
                ParametersType::TypePixelLoad => "TYPE_PIXEL_LOAD",
                ParametersType::TypeReachAndFrequency => "TYPE_REACH_AND_FREQUENCY",
                ParametersType::TypeReachAudience => "TYPE_REACH_AUDIENCE",
                ParametersType::TypeThirdPartyDataProvider => "TYPE_THIRD_PARTY_DATA_PROVIDER",
                ParametersType::TypeTrueview => "TYPE_TRUEVIEW",
                ParametersType::TypeTrueviewIar => "TYPE_TRUEVIEW_IAR",
                ParametersType::TypeVerification => "TYPE_VERIFICATION",
                ParametersType::TypeYoutubeVertical => "TYPE_YOUTUBE_VERTICAL",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ParametersType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ParametersType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ParametersType, ()> {
            Ok(match s {
                "TYPE_ACTIVE_GRP" => ParametersType::TypeActiveGrp,
                "TYPE_AUDIENCE_COMPOSITION" => ParametersType::TypeAudienceComposition,
                "TYPE_AUDIENCE_PERFORMANCE" => ParametersType::TypeAudiencePerformance,
                "TYPE_CLIENT_SAFE" => ParametersType::TypeClientSafe,
                "TYPE_COMSCORE_VCE" => ParametersType::TypeComscoreVce,
                "TYPE_CROSS_FEE" => ParametersType::TypeCrossFee,
                "TYPE_CROSS_PARTNER" => ParametersType::TypeCrossPartner,
                "TYPE_CROSS_PARTNER_THIRD_PARTY_DATA_PROVIDER" => {
                    ParametersType::TypeCrossPartnerThirdPartyDataProvider
                }
                "TYPE_ESTIMATED_CONVERSION" => ParametersType::TypeEstimatedConversion,
                "TYPE_FEE" => ParametersType::TypeFee,
                "TYPE_GENERAL" => ParametersType::TypeGeneral,
                "TYPE_INVENTORY_AVAILABILITY" => ParametersType::TypeInventoryAvailability,
                "TYPE_KEYWORD" => ParametersType::TypeKeyword,
                "TYPE_LINEAR_TV_SEARCH_LIFT" => ParametersType::TypeLinearTvSearchLift,
                "TYPE_NIELSEN_AUDIENCE_PROFILE" => ParametersType::TypeNielsenAudienceProfile,
                "TYPE_NIELSEN_DAILY_REACH_BUILD" => ParametersType::TypeNielsenDailyReachBuild,
                "TYPE_NIELSEN_ONLINE_GLOBAL_MARKET" => {
                    ParametersType::TypeNielsenOnlineGlobalMarket
                }
                "TYPE_NIELSEN_SITE" => ParametersType::TypeNielsenSite,
                "TYPE_NOT_SUPPORTED" => ParametersType::TypeNotSupported,
                "TYPE_ORDER_ID" => ParametersType::TypeOrderId,
                "TYPE_PAGE_CATEGORY" => ParametersType::TypePageCategory,
                "TYPE_PATH" => ParametersType::TypePath,
                "TYPE_PATH_ATTRIBUTION" => ParametersType::TypePathAttribution,
                "TYPE_PETRA_NIELSEN_AUDIENCE_PROFILE" => {
                    ParametersType::TypePetraNielsenAudienceProfile
                }
                "TYPE_PETRA_NIELSEN_DAILY_REACH_BUILD" => {
                    ParametersType::TypePetraNielsenDailyReachBuild
                }
                "TYPE_PETRA_NIELSEN_ONLINE_GLOBAL_MARKET" => {
                    ParametersType::TypePetraNielsenOnlineGlobalMarket
                }
                "TYPE_PIXEL_LOAD" => ParametersType::TypePixelLoad,
                "TYPE_REACH_AND_FREQUENCY" => ParametersType::TypeReachAndFrequency,
                "TYPE_REACH_AUDIENCE" => ParametersType::TypeReachAudience,
                "TYPE_THIRD_PARTY_DATA_PROVIDER" => ParametersType::TypeThirdPartyDataProvider,
                "TYPE_TRUEVIEW" => ParametersType::TypeTrueview,
                "TYPE_TRUEVIEW_IAR" => ParametersType::TypeTrueviewIar,
                "TYPE_VERIFICATION" => ParametersType::TypeVerification,
                "TYPE_YOUTUBE_VERTICAL" => ParametersType::TypeYoutubeVertical,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ParametersType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ParametersType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ParametersType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TYPE_ACTIVE_GRP" => ParametersType::TypeActiveGrp,
                "TYPE_AUDIENCE_COMPOSITION" => ParametersType::TypeAudienceComposition,
                "TYPE_AUDIENCE_PERFORMANCE" => ParametersType::TypeAudiencePerformance,
                "TYPE_CLIENT_SAFE" => ParametersType::TypeClientSafe,
                "TYPE_COMSCORE_VCE" => ParametersType::TypeComscoreVce,
                "TYPE_CROSS_FEE" => ParametersType::TypeCrossFee,
                "TYPE_CROSS_PARTNER" => ParametersType::TypeCrossPartner,
                "TYPE_CROSS_PARTNER_THIRD_PARTY_DATA_PROVIDER" => {
                    ParametersType::TypeCrossPartnerThirdPartyDataProvider
                }
                "TYPE_ESTIMATED_CONVERSION" => ParametersType::TypeEstimatedConversion,
                "TYPE_FEE" => ParametersType::TypeFee,
                "TYPE_GENERAL" => ParametersType::TypeGeneral,
                "TYPE_INVENTORY_AVAILABILITY" => ParametersType::TypeInventoryAvailability,
                "TYPE_KEYWORD" => ParametersType::TypeKeyword,
                "TYPE_LINEAR_TV_SEARCH_LIFT" => ParametersType::TypeLinearTvSearchLift,
                "TYPE_NIELSEN_AUDIENCE_PROFILE" => ParametersType::TypeNielsenAudienceProfile,
                "TYPE_NIELSEN_DAILY_REACH_BUILD" => ParametersType::TypeNielsenDailyReachBuild,
                "TYPE_NIELSEN_ONLINE_GLOBAL_MARKET" => {
                    ParametersType::TypeNielsenOnlineGlobalMarket
                }
                "TYPE_NIELSEN_SITE" => ParametersType::TypeNielsenSite,
                "TYPE_NOT_SUPPORTED" => ParametersType::TypeNotSupported,
                "TYPE_ORDER_ID" => ParametersType::TypeOrderId,
                "TYPE_PAGE_CATEGORY" => ParametersType::TypePageCategory,
                "TYPE_PATH" => ParametersType::TypePath,
                "TYPE_PATH_ATTRIBUTION" => ParametersType::TypePathAttribution,
                "TYPE_PETRA_NIELSEN_AUDIENCE_PROFILE" => {
                    ParametersType::TypePetraNielsenAudienceProfile
                }
                "TYPE_PETRA_NIELSEN_DAILY_REACH_BUILD" => {
                    ParametersType::TypePetraNielsenDailyReachBuild
                }
                "TYPE_PETRA_NIELSEN_ONLINE_GLOBAL_MARKET" => {
                    ParametersType::TypePetraNielsenOnlineGlobalMarket
                }
                "TYPE_PIXEL_LOAD" => ParametersType::TypePixelLoad,
                "TYPE_REACH_AND_FREQUENCY" => ParametersType::TypeReachAndFrequency,
                "TYPE_REACH_AUDIENCE" => ParametersType::TypeReachAudience,
                "TYPE_THIRD_PARTY_DATA_PROVIDER" => ParametersType::TypeThirdPartyDataProvider,
                "TYPE_TRUEVIEW" => ParametersType::TypeTrueview,
                "TYPE_TRUEVIEW_IAR" => ParametersType::TypeTrueviewIar,
                "TYPE_VERIFICATION" => ParametersType::TypeVerification,
                "TYPE_YOUTUBE_VERTICAL" => ParametersType::TypeYoutubeVertical,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ParametersType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ParametersType {
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
    pub struct PathFilter {
        #[doc = "Filter on an event to be applied to some part of the path."]
        #[serde(
            rename = "eventFilters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub event_filters: ::std::option::Option<Vec<crate::schemas::EventFilter>>,
        #[doc = "Indicates the position of the path the filter should match to (first, last, or any event in path)."]
        #[serde(
            rename = "pathMatchPosition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub path_match_position: ::std::option::Option<crate::schemas::PathFilterPathMatchPosition>,
    }
    impl ::google_field_selector::FieldSelector for PathFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PathFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PathFilterPathMatchPosition {
        Any,
        First,
        Last,
    }
    impl PathFilterPathMatchPosition {
        pub fn as_str(self) -> &'static str {
            match self {
                PathFilterPathMatchPosition::Any => "ANY",
                PathFilterPathMatchPosition::First => "FIRST",
                PathFilterPathMatchPosition::Last => "LAST",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PathFilterPathMatchPosition {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PathFilterPathMatchPosition {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PathFilterPathMatchPosition, ()> {
            Ok(match s {
                "ANY" => PathFilterPathMatchPosition::Any,
                "FIRST" => PathFilterPathMatchPosition::First,
                "LAST" => PathFilterPathMatchPosition::Last,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PathFilterPathMatchPosition {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PathFilterPathMatchPosition {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PathFilterPathMatchPosition {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ANY" => PathFilterPathMatchPosition::Any,
                "FIRST" => PathFilterPathMatchPosition::First,
                "LAST" => PathFilterPathMatchPosition::Last,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PathFilterPathMatchPosition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PathFilterPathMatchPosition {
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
    pub struct PathQueryOptions {
        #[doc = "Custom Channel Groupings."]
        #[serde(
            rename = "channelGrouping",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub channel_grouping: ::std::option::Option<crate::schemas::ChannelGrouping>,
        #[doc = "Path Filters. There is a limit of 100 path filters that can be set per report."]
        #[serde(
            rename = "pathFilters",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub path_filters: ::std::option::Option<Vec<crate::schemas::PathFilter>>,
    }
    impl ::google_field_selector::FieldSelector for PathQueryOptions {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PathQueryOptions {
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
    pub struct PathQueryOptionsFilter {
        #[doc = "Dimension the filter is applied to."]
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter: ::std::option::Option<crate::schemas::PathQueryOptionsFilterFilter>,
        #[doc = "Indicates how the filter should be matched to the value."]
        #[serde(
            rename = "match",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#match: ::std::option::Option<crate::schemas::PathQueryOptionsFilterMatch>,
        #[doc = "Value to filter on."]
        #[serde(
            rename = "values",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub values: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for PathQueryOptionsFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PathQueryOptionsFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PathQueryOptionsFilterFilter {
        FilterActiveViewCustomMetricId,
        FilterActiveViewCustomMetricName,
        FilterActiveViewExpectedViewability,
        FilterAdPosition,
        FilterAdType,
        FilterAdvertiser,
        FilterAdvertiserCurrency,
        FilterAdvertiserIntegrationCode,
        FilterAdvertiserIntegrationStatus,
        FilterAdvertiserName,
        FilterAdvertiserTimezone,
        FilterAge,
        FilterAlgorithm,
        FilterAlgorithmId,
        FilterAmpPageRequest,
        FilterAnonymousInventoryModeling,
        FilterAppUrl,
        FilterAppUrlExcluded,
        FilterAttributedUserlist,
        FilterAttributedUserlistCost,
        FilterAttributedUserlistType,
        FilterAttributionModel,
        FilterAudienceList,
        FilterAudienceListCost,
        FilterAudienceListType,
        FilterAudienceName,
        FilterAudienceType,
        FilterAudioFeedTypeName,
        FilterAuthorizedSellerState,
        FilterBillableOutcome,
        FilterBrandLiftType,
        FilterBrowser,
        FilterBudgetSegmentBudget,
        FilterBudgetSegmentDescription,
        FilterBudgetSegmentEndDate,
        FilterBudgetSegmentPacingPercentage,
        FilterBudgetSegmentStartDate,
        FilterBudgetSegmentType,
        FilterCampaignDailyFrequency,
        FilterCarrier,
        FilterCarrierName,
        FilterChannelGrouping,
        FilterChannelId,
        FilterChannelName,
        FilterChannelType,
        FilterCity,
        FilterCityName,
        FilterCm360PlacementId,
        FilterCmPlacementId,
        FilterCompanionCreativeId,
        FilterCompanionCreativeName,
        FilterConversionDelay,
        FilterConversionSource,
        FilterConversionSourceId,
        FilterCountry,
        FilterCountryId,
        FilterCreative,
        FilterCreativeAsset,
        FilterCreativeAttribute,
        FilterCreativeHeight,
        FilterCreativeId,
        FilterCreativeIntegrationCode,
        FilterCreativeRenderedInAmp,
        FilterCreativeSize,
        FilterCreativeSource,
        FilterCreativeStatus,
        FilterCreativeType,
        FilterCreativeWidth,
        FilterDataProvider,
        FilterDataProviderName,
        FilterDataSource,
        FilterDate,
        FilterDayOfWeek,
        FilterDetailedDemographics,
        FilterDetailedDemographicsId,
        FilterDevice,
        FilterDeviceMake,
        FilterDeviceModel,
        FilterDeviceType,
        FilterDfpOrderId,
        FilterDigitalContentLabel,
        FilterDma,
        FilterDmaName,
        FilterDomain,
        FilterEligibleCookiesOnFirstPartyAudienceList,
        FilterEligibleCookiesOnThirdPartyAudienceListAndInterest,
        FilterEventType,
        FilterExchange,
        FilterExchangeCode,
        FilterExchangeId,
        FilterExtension,
        FilterExtensionStatus,
        FilterExtensionType,
        FilterFirstPartyAudienceListCost,
        FilterFirstPartyAudienceListType,
        FilterFloodlightActivity,
        FilterFloodlightActivityId,
        FilterFormat,
        FilterGamInsertionOrder,
        FilterGamLineItem,
        FilterGamLineItemId,
        FilterGender,
        FilterGmailAge,
        FilterGmailCity,
        FilterGmailCountry,
        FilterGmailCountryName,
        FilterGmailDeviceType,
        FilterGmailDeviceTypeName,
        FilterGmailGender,
        FilterGmailRegion,
        FilterGmailRemarketingList,
        FilterHouseholdIncome,
        FilterImpressionCountingMethod,
        FilterImpressionLossRejectionReason,
        FilterInsertionOrder,
        FilterInsertionOrderGoalType,
        FilterInsertionOrderGoalValue,
        FilterInsertionOrderIntegrationCode,
        FilterInsertionOrderName,
        FilterInsertionOrderStatus,
        FilterInterest,
        FilterInventoryCommitmentType,
        FilterInventoryDeliveryMethod,
        FilterInventoryFormat,
        FilterInventoryRateType,
        FilterInventorySource,
        FilterInventorySourceExternalId,
        FilterInventorySourceGroup,
        FilterInventorySourceGroupId,
        FilterInventorySourceId,
        FilterInventorySourceName,
        FilterInventorySourceType,
        FilterKeyword,
        FilterLifeEvent,
        FilterLifeEvents,
        FilterLineItem,
        FilterLineItemBudget,
        FilterLineItemDailyFrequency,
        FilterLineItemEndDate,
        FilterLineItemIntegrationCode,
        FilterLineItemLifetimeFrequency,
        FilterLineItemName,
        FilterLineItemPacingPercentage,
        FilterLineItemStartDate,
        FilterLineItemStatus,
        FilterLineItemType,
        FilterMatchRatio,
        FilterMatchedGenreTarget,
        FilterMeasurementSource,
        FilterMediaPlan,
        FilterMediaPlanName,
        FilterMediaType,
        FilterMobileGeo,
        FilterMonth,
        FilterMraidSupport,
        FilterNielsenAge,
        FilterNielsenCountryCode,
        FilterNielsenDateRange,
        FilterNielsenDeviceId,
        FilterNielsenGender,
        FilterNielsenRestatementDate,
        FilterNotSupported,
        FilterOmSdkAvailable,
        FilterOmidCapable,
        FilterOrderId,
        FilterOs,
        FilterPageCategory,
        FilterPageLayout,
        FilterParentalStatus,
        FilterPartner,
        FilterPartnerCurrency,
        FilterPartnerName,
        FilterPartnerStatus,
        FilterPathEventIndex,
        FilterPathPatternId,
        FilterPlacementAllYoutubeChannels,
        FilterPlacementNameAllYoutubeChannels,
        FilterPlatform,
        FilterPlaybackMethod,
        FilterPositionInContent,
        FilterPublicInventory,
        FilterPublisherProperty,
        FilterPublisherPropertyId,
        FilterPublisherPropertySection,
        FilterPublisherPropertySectionId,
        FilterQuarter,
        FilterRefundReason,
        FilterRegion,
        FilterRegionName,
        FilterRemarketingList,
        FilterRewarded,
        FilterSensitiveCategory,
        FilterServedPixelDensity,
        FilterSiteId,
        FilterSiteLanguage,
        FilterSkippableSupport,
        FilterTargetedDataProviders,
        FilterTargetedUserList,
        FilterThirdPartyAudienceListCost,
        FilterThirdPartyAudienceListType,
        FilterTimeOfDay,
        FilterTrueviewAd,
        FilterTrueviewAdGroup,
        FilterTrueviewAdGroupAdId,
        FilterTrueviewAdGroupId,
        FilterTrueviewAdTypeName,
        FilterTrueviewAge,
        FilterTrueviewCategory,
        FilterTrueviewCity,
        FilterTrueviewClickTypeName,
        FilterTrueviewConversionType,
        FilterTrueviewCountry,
        FilterTrueviewCustomAffinity,
        FilterTrueviewDetailedDemographics,
        FilterTrueviewDetailedDemographicsId,
        FilterTrueviewDma,
        FilterTrueviewDmaName,
        FilterTrueviewGender,
        FilterTrueviewHouseholdIncome,
        FilterTrueviewIarAge,
        FilterTrueviewIarCategory,
        FilterTrueviewIarCity,
        FilterTrueviewIarCountry,
        FilterTrueviewIarCountryName,
        FilterTrueviewIarGender,
        FilterTrueviewIarInterest,
        FilterTrueviewIarLanguage,
        FilterTrueviewIarParentalStatus,
        FilterTrueviewIarRegion,
        FilterTrueviewIarRegionName,
        FilterTrueviewIarRemarketingList,
        FilterTrueviewIarTimeOfDay,
        FilterTrueviewIarYoutubeChannel,
        FilterTrueviewIarYoutubeVideo,
        FilterTrueviewIarZipcode,
        FilterTrueviewInterest,
        FilterTrueviewKeyword,
        FilterTrueviewParentalStatus,
        FilterTrueviewPlacement,
        FilterTrueviewPlacementId,
        FilterTrueviewRegion,
        FilterTrueviewRegionName,
        FilterTrueviewRemarketingList,
        FilterTrueviewRemarketingListName,
        FilterTrueviewUrl,
        FilterTrueviewZipcode,
        FilterUnknown,
        FilterUserList,
        FilterUserListFirstParty,
        FilterUserListFirstPartyName,
        FilterUserListThirdParty,
        FilterUserListThirdPartyName,
        FilterVariantId,
        FilterVariantName,
        FilterVariantVersion,
        FilterVendorMeasurementMode,
        FilterVerificationAudibilityComplete,
        FilterVerificationAudibilityStart,
        FilterVerificationVideoPlayerSize,
        FilterVerificationVideoPlayerSizeComplete,
        FilterVerificationVideoPlayerSizeFirstQuartile,
        FilterVerificationVideoPlayerSizeMidPoint,
        FilterVerificationVideoPlayerSizeStart,
        FilterVerificationVideoPlayerSizeThirdQuartile,
        FilterVerificationVideoPosition,
        FilterVerificationVideoResized,
        FilterVideoAdPositionInStream,
        FilterVideoCompanionCreativeSize,
        FilterVideoContentDuration,
        FilterVideoContentLiveStream,
        FilterVideoContinuousPlay,
        FilterVideoCreativeDuration,
        FilterVideoCreativeDurationSkippable,
        FilterVideoDuration,
        FilterVideoDurationSeconds,
        FilterVideoDurationSecondsRange,
        FilterVideoFormatSupport,
        FilterVideoPlayerSize,
        FilterVideoRatingTier,
        FilterVideoSkippableSupport,
        FilterWeek,
        FilterYear,
        FilterYoutubeAdVideo,
        FilterYoutubeAdVideoId,
        FilterYoutubeAdaptedAudienceList,
        FilterYoutubeChannel,
        FilterYoutubeProgrammaticGuaranteedAdvertiser,
        FilterYoutubeProgrammaticGuaranteedInsertionOrder,
        FilterYoutubeProgrammaticGuaranteedPartner,
        FilterYoutubeVideo,
        FilterZipCode,
        FilterZipPostalCode,
    }
    impl PathQueryOptionsFilterFilter {
        pub fn as_str(self) -> &'static str {
            match self { PathQueryOptionsFilterFilter :: FilterActiveViewCustomMetricId => "FILTER_ACTIVE_VIEW_CUSTOM_METRIC_ID" , PathQueryOptionsFilterFilter :: FilterActiveViewCustomMetricName => "FILTER_ACTIVE_VIEW_CUSTOM_METRIC_NAME" , PathQueryOptionsFilterFilter :: FilterActiveViewExpectedViewability => "FILTER_ACTIVE_VIEW_EXPECTED_VIEWABILITY" , PathQueryOptionsFilterFilter :: FilterAdPosition => "FILTER_AD_POSITION" , PathQueryOptionsFilterFilter :: FilterAdType => "FILTER_AD_TYPE" , PathQueryOptionsFilterFilter :: FilterAdvertiser => "FILTER_ADVERTISER" , PathQueryOptionsFilterFilter :: FilterAdvertiserCurrency => "FILTER_ADVERTISER_CURRENCY" , PathQueryOptionsFilterFilter :: FilterAdvertiserIntegrationCode => "FILTER_ADVERTISER_INTEGRATION_CODE" , PathQueryOptionsFilterFilter :: FilterAdvertiserIntegrationStatus => "FILTER_ADVERTISER_INTEGRATION_STATUS" , PathQueryOptionsFilterFilter :: FilterAdvertiserName => "FILTER_ADVERTISER_NAME" , PathQueryOptionsFilterFilter :: FilterAdvertiserTimezone => "FILTER_ADVERTISER_TIMEZONE" , PathQueryOptionsFilterFilter :: FilterAge => "FILTER_AGE" , PathQueryOptionsFilterFilter :: FilterAlgorithm => "FILTER_ALGORITHM" , PathQueryOptionsFilterFilter :: FilterAlgorithmId => "FILTER_ALGORITHM_ID" , PathQueryOptionsFilterFilter :: FilterAmpPageRequest => "FILTER_AMP_PAGE_REQUEST" , PathQueryOptionsFilterFilter :: FilterAnonymousInventoryModeling => "FILTER_ANONYMOUS_INVENTORY_MODELING" , PathQueryOptionsFilterFilter :: FilterAppUrl => "FILTER_APP_URL" , PathQueryOptionsFilterFilter :: FilterAppUrlExcluded => "FILTER_APP_URL_EXCLUDED" , PathQueryOptionsFilterFilter :: FilterAttributedUserlist => "FILTER_ATTRIBUTED_USERLIST" , PathQueryOptionsFilterFilter :: FilterAttributedUserlistCost => "FILTER_ATTRIBUTED_USERLIST_COST" , PathQueryOptionsFilterFilter :: FilterAttributedUserlistType => "FILTER_ATTRIBUTED_USERLIST_TYPE" , PathQueryOptionsFilterFilter :: FilterAttributionModel => "FILTER_ATTRIBUTION_MODEL" , PathQueryOptionsFilterFilter :: FilterAudienceList => "FILTER_AUDIENCE_LIST" , PathQueryOptionsFilterFilter :: FilterAudienceListCost => "FILTER_AUDIENCE_LIST_COST" , PathQueryOptionsFilterFilter :: FilterAudienceListType => "FILTER_AUDIENCE_LIST_TYPE" , PathQueryOptionsFilterFilter :: FilterAudienceName => "FILTER_AUDIENCE_NAME" , PathQueryOptionsFilterFilter :: FilterAudienceType => "FILTER_AUDIENCE_TYPE" , PathQueryOptionsFilterFilter :: FilterAudioFeedTypeName => "FILTER_AUDIO_FEED_TYPE_NAME" , PathQueryOptionsFilterFilter :: FilterAuthorizedSellerState => "FILTER_AUTHORIZED_SELLER_STATE" , PathQueryOptionsFilterFilter :: FilterBillableOutcome => "FILTER_BILLABLE_OUTCOME" , PathQueryOptionsFilterFilter :: FilterBrandLiftType => "FILTER_BRAND_LIFT_TYPE" , PathQueryOptionsFilterFilter :: FilterBrowser => "FILTER_BROWSER" , PathQueryOptionsFilterFilter :: FilterBudgetSegmentBudget => "FILTER_BUDGET_SEGMENT_BUDGET" , PathQueryOptionsFilterFilter :: FilterBudgetSegmentDescription => "FILTER_BUDGET_SEGMENT_DESCRIPTION" , PathQueryOptionsFilterFilter :: FilterBudgetSegmentEndDate => "FILTER_BUDGET_SEGMENT_END_DATE" , PathQueryOptionsFilterFilter :: FilterBudgetSegmentPacingPercentage => "FILTER_BUDGET_SEGMENT_PACING_PERCENTAGE" , PathQueryOptionsFilterFilter :: FilterBudgetSegmentStartDate => "FILTER_BUDGET_SEGMENT_START_DATE" , PathQueryOptionsFilterFilter :: FilterBudgetSegmentType => "FILTER_BUDGET_SEGMENT_TYPE" , PathQueryOptionsFilterFilter :: FilterCampaignDailyFrequency => "FILTER_CAMPAIGN_DAILY_FREQUENCY" , PathQueryOptionsFilterFilter :: FilterCarrier => "FILTER_CARRIER" , PathQueryOptionsFilterFilter :: FilterCarrierName => "FILTER_CARRIER_NAME" , PathQueryOptionsFilterFilter :: FilterChannelGrouping => "FILTER_CHANNEL_GROUPING" , PathQueryOptionsFilterFilter :: FilterChannelId => "FILTER_CHANNEL_ID" , PathQueryOptionsFilterFilter :: FilterChannelName => "FILTER_CHANNEL_NAME" , PathQueryOptionsFilterFilter :: FilterChannelType => "FILTER_CHANNEL_TYPE" , PathQueryOptionsFilterFilter :: FilterCity => "FILTER_CITY" , PathQueryOptionsFilterFilter :: FilterCityName => "FILTER_CITY_NAME" , PathQueryOptionsFilterFilter :: FilterCm360PlacementId => "FILTER_CM360_PLACEMENT_ID" , PathQueryOptionsFilterFilter :: FilterCmPlacementId => "FILTER_CM_PLACEMENT_ID" , PathQueryOptionsFilterFilter :: FilterCompanionCreativeId => "FILTER_COMPANION_CREATIVE_ID" , PathQueryOptionsFilterFilter :: FilterCompanionCreativeName => "FILTER_COMPANION_CREATIVE_NAME" , PathQueryOptionsFilterFilter :: FilterConversionDelay => "FILTER_CONVERSION_DELAY" , PathQueryOptionsFilterFilter :: FilterConversionSource => "FILTER_CONVERSION_SOURCE" , PathQueryOptionsFilterFilter :: FilterConversionSourceId => "FILTER_CONVERSION_SOURCE_ID" , PathQueryOptionsFilterFilter :: FilterCountry => "FILTER_COUNTRY" , PathQueryOptionsFilterFilter :: FilterCountryId => "FILTER_COUNTRY_ID" , PathQueryOptionsFilterFilter :: FilterCreative => "FILTER_CREATIVE" , PathQueryOptionsFilterFilter :: FilterCreativeAsset => "FILTER_CREATIVE_ASSET" , PathQueryOptionsFilterFilter :: FilterCreativeAttribute => "FILTER_CREATIVE_ATTRIBUTE" , PathQueryOptionsFilterFilter :: FilterCreativeHeight => "FILTER_CREATIVE_HEIGHT" , PathQueryOptionsFilterFilter :: FilterCreativeId => "FILTER_CREATIVE_ID" , PathQueryOptionsFilterFilter :: FilterCreativeIntegrationCode => "FILTER_CREATIVE_INTEGRATION_CODE" , PathQueryOptionsFilterFilter :: FilterCreativeRenderedInAmp => "FILTER_CREATIVE_RENDERED_IN_AMP" , PathQueryOptionsFilterFilter :: FilterCreativeSize => "FILTER_CREATIVE_SIZE" , PathQueryOptionsFilterFilter :: FilterCreativeSource => "FILTER_CREATIVE_SOURCE" , PathQueryOptionsFilterFilter :: FilterCreativeStatus => "FILTER_CREATIVE_STATUS" , PathQueryOptionsFilterFilter :: FilterCreativeType => "FILTER_CREATIVE_TYPE" , PathQueryOptionsFilterFilter :: FilterCreativeWidth => "FILTER_CREATIVE_WIDTH" , PathQueryOptionsFilterFilter :: FilterDataProvider => "FILTER_DATA_PROVIDER" , PathQueryOptionsFilterFilter :: FilterDataProviderName => "FILTER_DATA_PROVIDER_NAME" , PathQueryOptionsFilterFilter :: FilterDataSource => "FILTER_DATA_SOURCE" , PathQueryOptionsFilterFilter :: FilterDate => "FILTER_DATE" , PathQueryOptionsFilterFilter :: FilterDayOfWeek => "FILTER_DAY_OF_WEEK" , PathQueryOptionsFilterFilter :: FilterDetailedDemographics => "FILTER_DETAILED_DEMOGRAPHICS" , PathQueryOptionsFilterFilter :: FilterDetailedDemographicsId => "FILTER_DETAILED_DEMOGRAPHICS_ID" , PathQueryOptionsFilterFilter :: FilterDevice => "FILTER_DEVICE" , PathQueryOptionsFilterFilter :: FilterDeviceMake => "FILTER_DEVICE_MAKE" , PathQueryOptionsFilterFilter :: FilterDeviceModel => "FILTER_DEVICE_MODEL" , PathQueryOptionsFilterFilter :: FilterDeviceType => "FILTER_DEVICE_TYPE" , PathQueryOptionsFilterFilter :: FilterDfpOrderId => "FILTER_DFP_ORDER_ID" , PathQueryOptionsFilterFilter :: FilterDigitalContentLabel => "FILTER_DIGITAL_CONTENT_LABEL" , PathQueryOptionsFilterFilter :: FilterDma => "FILTER_DMA" , PathQueryOptionsFilterFilter :: FilterDmaName => "FILTER_DMA_NAME" , PathQueryOptionsFilterFilter :: FilterDomain => "FILTER_DOMAIN" , PathQueryOptionsFilterFilter :: FilterEligibleCookiesOnFirstPartyAudienceList => "FILTER_ELIGIBLE_COOKIES_ON_FIRST_PARTY_AUDIENCE_LIST" , PathQueryOptionsFilterFilter :: FilterEligibleCookiesOnThirdPartyAudienceListAndInterest => "FILTER_ELIGIBLE_COOKIES_ON_THIRD_PARTY_AUDIENCE_LIST_AND_INTEREST" , PathQueryOptionsFilterFilter :: FilterEventType => "FILTER_EVENT_TYPE" , PathQueryOptionsFilterFilter :: FilterExchange => "FILTER_EXCHANGE" , PathQueryOptionsFilterFilter :: FilterExchangeCode => "FILTER_EXCHANGE_CODE" , PathQueryOptionsFilterFilter :: FilterExchangeId => "FILTER_EXCHANGE_ID" , PathQueryOptionsFilterFilter :: FilterExtension => "FILTER_EXTENSION" , PathQueryOptionsFilterFilter :: FilterExtensionStatus => "FILTER_EXTENSION_STATUS" , PathQueryOptionsFilterFilter :: FilterExtensionType => "FILTER_EXTENSION_TYPE" , PathQueryOptionsFilterFilter :: FilterFirstPartyAudienceListCost => "FILTER_FIRST_PARTY_AUDIENCE_LIST_COST" , PathQueryOptionsFilterFilter :: FilterFirstPartyAudienceListType => "FILTER_FIRST_PARTY_AUDIENCE_LIST_TYPE" , PathQueryOptionsFilterFilter :: FilterFloodlightActivity => "FILTER_FLOODLIGHT_ACTIVITY" , PathQueryOptionsFilterFilter :: FilterFloodlightActivityId => "FILTER_FLOODLIGHT_ACTIVITY_ID" , PathQueryOptionsFilterFilter :: FilterFormat => "FILTER_FORMAT" , PathQueryOptionsFilterFilter :: FilterGamInsertionOrder => "FILTER_GAM_INSERTION_ORDER" , PathQueryOptionsFilterFilter :: FilterGamLineItem => "FILTER_GAM_LINE_ITEM" , PathQueryOptionsFilterFilter :: FilterGamLineItemId => "FILTER_GAM_LINE_ITEM_ID" , PathQueryOptionsFilterFilter :: FilterGender => "FILTER_GENDER" , PathQueryOptionsFilterFilter :: FilterGmailAge => "FILTER_GMAIL_AGE" , PathQueryOptionsFilterFilter :: FilterGmailCity => "FILTER_GMAIL_CITY" , PathQueryOptionsFilterFilter :: FilterGmailCountry => "FILTER_GMAIL_COUNTRY" , PathQueryOptionsFilterFilter :: FilterGmailCountryName => "FILTER_GMAIL_COUNTRY_NAME" , PathQueryOptionsFilterFilter :: FilterGmailDeviceType => "FILTER_GMAIL_DEVICE_TYPE" , PathQueryOptionsFilterFilter :: FilterGmailDeviceTypeName => "FILTER_GMAIL_DEVICE_TYPE_NAME" , PathQueryOptionsFilterFilter :: FilterGmailGender => "FILTER_GMAIL_GENDER" , PathQueryOptionsFilterFilter :: FilterGmailRegion => "FILTER_GMAIL_REGION" , PathQueryOptionsFilterFilter :: FilterGmailRemarketingList => "FILTER_GMAIL_REMARKETING_LIST" , PathQueryOptionsFilterFilter :: FilterHouseholdIncome => "FILTER_HOUSEHOLD_INCOME" , PathQueryOptionsFilterFilter :: FilterImpressionCountingMethod => "FILTER_IMPRESSION_COUNTING_METHOD" , PathQueryOptionsFilterFilter :: FilterImpressionLossRejectionReason => "FILTER_IMPRESSION_LOSS_REJECTION_REASON" , PathQueryOptionsFilterFilter :: FilterInsertionOrder => "FILTER_INSERTION_ORDER" , PathQueryOptionsFilterFilter :: FilterInsertionOrderGoalType => "FILTER_INSERTION_ORDER_GOAL_TYPE" , PathQueryOptionsFilterFilter :: FilterInsertionOrderGoalValue => "FILTER_INSERTION_ORDER_GOAL_VALUE" , PathQueryOptionsFilterFilter :: FilterInsertionOrderIntegrationCode => "FILTER_INSERTION_ORDER_INTEGRATION_CODE" , PathQueryOptionsFilterFilter :: FilterInsertionOrderName => "FILTER_INSERTION_ORDER_NAME" , PathQueryOptionsFilterFilter :: FilterInsertionOrderStatus => "FILTER_INSERTION_ORDER_STATUS" , PathQueryOptionsFilterFilter :: FilterInterest => "FILTER_INTEREST" , PathQueryOptionsFilterFilter :: FilterInventoryCommitmentType => "FILTER_INVENTORY_COMMITMENT_TYPE" , PathQueryOptionsFilterFilter :: FilterInventoryDeliveryMethod => "FILTER_INVENTORY_DELIVERY_METHOD" , PathQueryOptionsFilterFilter :: FilterInventoryFormat => "FILTER_INVENTORY_FORMAT" , PathQueryOptionsFilterFilter :: FilterInventoryRateType => "FILTER_INVENTORY_RATE_TYPE" , PathQueryOptionsFilterFilter :: FilterInventorySource => "FILTER_INVENTORY_SOURCE" , PathQueryOptionsFilterFilter :: FilterInventorySourceExternalId => "FILTER_INVENTORY_SOURCE_EXTERNAL_ID" , PathQueryOptionsFilterFilter :: FilterInventorySourceGroup => "FILTER_INVENTORY_SOURCE_GROUP" , PathQueryOptionsFilterFilter :: FilterInventorySourceGroupId => "FILTER_INVENTORY_SOURCE_GROUP_ID" , PathQueryOptionsFilterFilter :: FilterInventorySourceId => "FILTER_INVENTORY_SOURCE_ID" , PathQueryOptionsFilterFilter :: FilterInventorySourceName => "FILTER_INVENTORY_SOURCE_NAME" , PathQueryOptionsFilterFilter :: FilterInventorySourceType => "FILTER_INVENTORY_SOURCE_TYPE" , PathQueryOptionsFilterFilter :: FilterKeyword => "FILTER_KEYWORD" , PathQueryOptionsFilterFilter :: FilterLifeEvent => "FILTER_LIFE_EVENT" , PathQueryOptionsFilterFilter :: FilterLifeEvents => "FILTER_LIFE_EVENTS" , PathQueryOptionsFilterFilter :: FilterLineItem => "FILTER_LINE_ITEM" , PathQueryOptionsFilterFilter :: FilterLineItemBudget => "FILTER_LINE_ITEM_BUDGET" , PathQueryOptionsFilterFilter :: FilterLineItemDailyFrequency => "FILTER_LINE_ITEM_DAILY_FREQUENCY" , PathQueryOptionsFilterFilter :: FilterLineItemEndDate => "FILTER_LINE_ITEM_END_DATE" , PathQueryOptionsFilterFilter :: FilterLineItemIntegrationCode => "FILTER_LINE_ITEM_INTEGRATION_CODE" , PathQueryOptionsFilterFilter :: FilterLineItemLifetimeFrequency => "FILTER_LINE_ITEM_LIFETIME_FREQUENCY" , PathQueryOptionsFilterFilter :: FilterLineItemName => "FILTER_LINE_ITEM_NAME" , PathQueryOptionsFilterFilter :: FilterLineItemPacingPercentage => "FILTER_LINE_ITEM_PACING_PERCENTAGE" , PathQueryOptionsFilterFilter :: FilterLineItemStartDate => "FILTER_LINE_ITEM_START_DATE" , PathQueryOptionsFilterFilter :: FilterLineItemStatus => "FILTER_LINE_ITEM_STATUS" , PathQueryOptionsFilterFilter :: FilterLineItemType => "FILTER_LINE_ITEM_TYPE" , PathQueryOptionsFilterFilter :: FilterMatchRatio => "FILTER_MATCH_RATIO" , PathQueryOptionsFilterFilter :: FilterMatchedGenreTarget => "FILTER_MATCHED_GENRE_TARGET" , PathQueryOptionsFilterFilter :: FilterMeasurementSource => "FILTER_MEASUREMENT_SOURCE" , PathQueryOptionsFilterFilter :: FilterMediaPlan => "FILTER_MEDIA_PLAN" , PathQueryOptionsFilterFilter :: FilterMediaPlanName => "FILTER_MEDIA_PLAN_NAME" , PathQueryOptionsFilterFilter :: FilterMediaType => "FILTER_MEDIA_TYPE" , PathQueryOptionsFilterFilter :: FilterMobileGeo => "FILTER_MOBILE_GEO" , PathQueryOptionsFilterFilter :: FilterMonth => "FILTER_MONTH" , PathQueryOptionsFilterFilter :: FilterMraidSupport => "FILTER_MRAID_SUPPORT" , PathQueryOptionsFilterFilter :: FilterNielsenAge => "FILTER_NIELSEN_AGE" , PathQueryOptionsFilterFilter :: FilterNielsenCountryCode => "FILTER_NIELSEN_COUNTRY_CODE" , PathQueryOptionsFilterFilter :: FilterNielsenDateRange => "FILTER_NIELSEN_DATE_RANGE" , PathQueryOptionsFilterFilter :: FilterNielsenDeviceId => "FILTER_NIELSEN_DEVICE_ID" , PathQueryOptionsFilterFilter :: FilterNielsenGender => "FILTER_NIELSEN_GENDER" , PathQueryOptionsFilterFilter :: FilterNielsenRestatementDate => "FILTER_NIELSEN_RESTATEMENT_DATE" , PathQueryOptionsFilterFilter :: FilterNotSupported => "FILTER_NOT_SUPPORTED" , PathQueryOptionsFilterFilter :: FilterOmSdkAvailable => "FILTER_OM_SDK_AVAILABLE" , PathQueryOptionsFilterFilter :: FilterOmidCapable => "FILTER_OMID_CAPABLE" , PathQueryOptionsFilterFilter :: FilterOrderId => "FILTER_ORDER_ID" , PathQueryOptionsFilterFilter :: FilterOs => "FILTER_OS" , PathQueryOptionsFilterFilter :: FilterPageCategory => "FILTER_PAGE_CATEGORY" , PathQueryOptionsFilterFilter :: FilterPageLayout => "FILTER_PAGE_LAYOUT" , PathQueryOptionsFilterFilter :: FilterParentalStatus => "FILTER_PARENTAL_STATUS" , PathQueryOptionsFilterFilter :: FilterPartner => "FILTER_PARTNER" , PathQueryOptionsFilterFilter :: FilterPartnerCurrency => "FILTER_PARTNER_CURRENCY" , PathQueryOptionsFilterFilter :: FilterPartnerName => "FILTER_PARTNER_NAME" , PathQueryOptionsFilterFilter :: FilterPartnerStatus => "FILTER_PARTNER_STATUS" , PathQueryOptionsFilterFilter :: FilterPathEventIndex => "FILTER_PATH_EVENT_INDEX" , PathQueryOptionsFilterFilter :: FilterPathPatternId => "FILTER_PATH_PATTERN_ID" , PathQueryOptionsFilterFilter :: FilterPlacementAllYoutubeChannels => "FILTER_PLACEMENT_ALL_YOUTUBE_CHANNELS" , PathQueryOptionsFilterFilter :: FilterPlacementNameAllYoutubeChannels => "FILTER_PLACEMENT_NAME_ALL_YOUTUBE_CHANNELS" , PathQueryOptionsFilterFilter :: FilterPlatform => "FILTER_PLATFORM" , PathQueryOptionsFilterFilter :: FilterPlaybackMethod => "FILTER_PLAYBACK_METHOD" , PathQueryOptionsFilterFilter :: FilterPositionInContent => "FILTER_POSITION_IN_CONTENT" , PathQueryOptionsFilterFilter :: FilterPublicInventory => "FILTER_PUBLIC_INVENTORY" , PathQueryOptionsFilterFilter :: FilterPublisherProperty => "FILTER_PUBLISHER_PROPERTY" , PathQueryOptionsFilterFilter :: FilterPublisherPropertyId => "FILTER_PUBLISHER_PROPERTY_ID" , PathQueryOptionsFilterFilter :: FilterPublisherPropertySection => "FILTER_PUBLISHER_PROPERTY_SECTION" , PathQueryOptionsFilterFilter :: FilterPublisherPropertySectionId => "FILTER_PUBLISHER_PROPERTY_SECTION_ID" , PathQueryOptionsFilterFilter :: FilterQuarter => "FILTER_QUARTER" , PathQueryOptionsFilterFilter :: FilterRefundReason => "FILTER_REFUND_REASON" , PathQueryOptionsFilterFilter :: FilterRegion => "FILTER_REGION" , PathQueryOptionsFilterFilter :: FilterRegionName => "FILTER_REGION_NAME" , PathQueryOptionsFilterFilter :: FilterRemarketingList => "FILTER_REMARKETING_LIST" , PathQueryOptionsFilterFilter :: FilterRewarded => "FILTER_REWARDED" , PathQueryOptionsFilterFilter :: FilterSensitiveCategory => "FILTER_SENSITIVE_CATEGORY" , PathQueryOptionsFilterFilter :: FilterServedPixelDensity => "FILTER_SERVED_PIXEL_DENSITY" , PathQueryOptionsFilterFilter :: FilterSiteId => "FILTER_SITE_ID" , PathQueryOptionsFilterFilter :: FilterSiteLanguage => "FILTER_SITE_LANGUAGE" , PathQueryOptionsFilterFilter :: FilterSkippableSupport => "FILTER_SKIPPABLE_SUPPORT" , PathQueryOptionsFilterFilter :: FilterTargetedDataProviders => "FILTER_TARGETED_DATA_PROVIDERS" , PathQueryOptionsFilterFilter :: FilterTargetedUserList => "FILTER_TARGETED_USER_LIST" , PathQueryOptionsFilterFilter :: FilterThirdPartyAudienceListCost => "FILTER_THIRD_PARTY_AUDIENCE_LIST_COST" , PathQueryOptionsFilterFilter :: FilterThirdPartyAudienceListType => "FILTER_THIRD_PARTY_AUDIENCE_LIST_TYPE" , PathQueryOptionsFilterFilter :: FilterTimeOfDay => "FILTER_TIME_OF_DAY" , PathQueryOptionsFilterFilter :: FilterTrueviewAd => "FILTER_TRUEVIEW_AD" , PathQueryOptionsFilterFilter :: FilterTrueviewAdGroup => "FILTER_TRUEVIEW_AD_GROUP" , PathQueryOptionsFilterFilter :: FilterTrueviewAdGroupAdId => "FILTER_TRUEVIEW_AD_GROUP_AD_ID" , PathQueryOptionsFilterFilter :: FilterTrueviewAdGroupId => "FILTER_TRUEVIEW_AD_GROUP_ID" , PathQueryOptionsFilterFilter :: FilterTrueviewAdTypeName => "FILTER_TRUEVIEW_AD_TYPE_NAME" , PathQueryOptionsFilterFilter :: FilterTrueviewAge => "FILTER_TRUEVIEW_AGE" , PathQueryOptionsFilterFilter :: FilterTrueviewCategory => "FILTER_TRUEVIEW_CATEGORY" , PathQueryOptionsFilterFilter :: FilterTrueviewCity => "FILTER_TRUEVIEW_CITY" , PathQueryOptionsFilterFilter :: FilterTrueviewClickTypeName => "FILTER_TRUEVIEW_CLICK_TYPE_NAME" , PathQueryOptionsFilterFilter :: FilterTrueviewConversionType => "FILTER_TRUEVIEW_CONVERSION_TYPE" , PathQueryOptionsFilterFilter :: FilterTrueviewCountry => "FILTER_TRUEVIEW_COUNTRY" , PathQueryOptionsFilterFilter :: FilterTrueviewCustomAffinity => "FILTER_TRUEVIEW_CUSTOM_AFFINITY" , PathQueryOptionsFilterFilter :: FilterTrueviewDetailedDemographics => "FILTER_TRUEVIEW_DETAILED_DEMOGRAPHICS" , PathQueryOptionsFilterFilter :: FilterTrueviewDetailedDemographicsId => "FILTER_TRUEVIEW_DETAILED_DEMOGRAPHICS_ID" , PathQueryOptionsFilterFilter :: FilterTrueviewDma => "FILTER_TRUEVIEW_DMA" , PathQueryOptionsFilterFilter :: FilterTrueviewDmaName => "FILTER_TRUEVIEW_DMA_NAME" , PathQueryOptionsFilterFilter :: FilterTrueviewGender => "FILTER_TRUEVIEW_GENDER" , PathQueryOptionsFilterFilter :: FilterTrueviewHouseholdIncome => "FILTER_TRUEVIEW_HOUSEHOLD_INCOME" , PathQueryOptionsFilterFilter :: FilterTrueviewIarAge => "FILTER_TRUEVIEW_IAR_AGE" , PathQueryOptionsFilterFilter :: FilterTrueviewIarCategory => "FILTER_TRUEVIEW_IAR_CATEGORY" , PathQueryOptionsFilterFilter :: FilterTrueviewIarCity => "FILTER_TRUEVIEW_IAR_CITY" , PathQueryOptionsFilterFilter :: FilterTrueviewIarCountry => "FILTER_TRUEVIEW_IAR_COUNTRY" , PathQueryOptionsFilterFilter :: FilterTrueviewIarCountryName => "FILTER_TRUEVIEW_IAR_COUNTRY_NAME" , PathQueryOptionsFilterFilter :: FilterTrueviewIarGender => "FILTER_TRUEVIEW_IAR_GENDER" , PathQueryOptionsFilterFilter :: FilterTrueviewIarInterest => "FILTER_TRUEVIEW_IAR_INTEREST" , PathQueryOptionsFilterFilter :: FilterTrueviewIarLanguage => "FILTER_TRUEVIEW_IAR_LANGUAGE" , PathQueryOptionsFilterFilter :: FilterTrueviewIarParentalStatus => "FILTER_TRUEVIEW_IAR_PARENTAL_STATUS" , PathQueryOptionsFilterFilter :: FilterTrueviewIarRegion => "FILTER_TRUEVIEW_IAR_REGION" , PathQueryOptionsFilterFilter :: FilterTrueviewIarRegionName => "FILTER_TRUEVIEW_IAR_REGION_NAME" , PathQueryOptionsFilterFilter :: FilterTrueviewIarRemarketingList => "FILTER_TRUEVIEW_IAR_REMARKETING_LIST" , PathQueryOptionsFilterFilter :: FilterTrueviewIarTimeOfDay => "FILTER_TRUEVIEW_IAR_TIME_OF_DAY" , PathQueryOptionsFilterFilter :: FilterTrueviewIarYoutubeChannel => "FILTER_TRUEVIEW_IAR_YOUTUBE_CHANNEL" , PathQueryOptionsFilterFilter :: FilterTrueviewIarYoutubeVideo => "FILTER_TRUEVIEW_IAR_YOUTUBE_VIDEO" , PathQueryOptionsFilterFilter :: FilterTrueviewIarZipcode => "FILTER_TRUEVIEW_IAR_ZIPCODE" , PathQueryOptionsFilterFilter :: FilterTrueviewInterest => "FILTER_TRUEVIEW_INTEREST" , PathQueryOptionsFilterFilter :: FilterTrueviewKeyword => "FILTER_TRUEVIEW_KEYWORD" , PathQueryOptionsFilterFilter :: FilterTrueviewParentalStatus => "FILTER_TRUEVIEW_PARENTAL_STATUS" , PathQueryOptionsFilterFilter :: FilterTrueviewPlacement => "FILTER_TRUEVIEW_PLACEMENT" , PathQueryOptionsFilterFilter :: FilterTrueviewPlacementId => "FILTER_TRUEVIEW_PLACEMENT_ID" , PathQueryOptionsFilterFilter :: FilterTrueviewRegion => "FILTER_TRUEVIEW_REGION" , PathQueryOptionsFilterFilter :: FilterTrueviewRegionName => "FILTER_TRUEVIEW_REGION_NAME" , PathQueryOptionsFilterFilter :: FilterTrueviewRemarketingList => "FILTER_TRUEVIEW_REMARKETING_LIST" , PathQueryOptionsFilterFilter :: FilterTrueviewRemarketingListName => "FILTER_TRUEVIEW_REMARKETING_LIST_NAME" , PathQueryOptionsFilterFilter :: FilterTrueviewUrl => "FILTER_TRUEVIEW_URL" , PathQueryOptionsFilterFilter :: FilterTrueviewZipcode => "FILTER_TRUEVIEW_ZIPCODE" , PathQueryOptionsFilterFilter :: FilterUnknown => "FILTER_UNKNOWN" , PathQueryOptionsFilterFilter :: FilterUserList => "FILTER_USER_LIST" , PathQueryOptionsFilterFilter :: FilterUserListFirstParty => "FILTER_USER_LIST_FIRST_PARTY" , PathQueryOptionsFilterFilter :: FilterUserListFirstPartyName => "FILTER_USER_LIST_FIRST_PARTY_NAME" , PathQueryOptionsFilterFilter :: FilterUserListThirdParty => "FILTER_USER_LIST_THIRD_PARTY" , PathQueryOptionsFilterFilter :: FilterUserListThirdPartyName => "FILTER_USER_LIST_THIRD_PARTY_NAME" , PathQueryOptionsFilterFilter :: FilterVariantId => "FILTER_VARIANT_ID" , PathQueryOptionsFilterFilter :: FilterVariantName => "FILTER_VARIANT_NAME" , PathQueryOptionsFilterFilter :: FilterVariantVersion => "FILTER_VARIANT_VERSION" , PathQueryOptionsFilterFilter :: FilterVendorMeasurementMode => "FILTER_VENDOR_MEASUREMENT_MODE" , PathQueryOptionsFilterFilter :: FilterVerificationAudibilityComplete => "FILTER_VERIFICATION_AUDIBILITY_COMPLETE" , PathQueryOptionsFilterFilter :: FilterVerificationAudibilityStart => "FILTER_VERIFICATION_AUDIBILITY_START" , PathQueryOptionsFilterFilter :: FilterVerificationVideoPlayerSize => "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE" , PathQueryOptionsFilterFilter :: FilterVerificationVideoPlayerSizeComplete => "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_COMPLETE" , PathQueryOptionsFilterFilter :: FilterVerificationVideoPlayerSizeFirstQuartile => "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_FIRST_QUARTILE" , PathQueryOptionsFilterFilter :: FilterVerificationVideoPlayerSizeMidPoint => "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_MID_POINT" , PathQueryOptionsFilterFilter :: FilterVerificationVideoPlayerSizeStart => "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_START" , PathQueryOptionsFilterFilter :: FilterVerificationVideoPlayerSizeThirdQuartile => "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_THIRD_QUARTILE" , PathQueryOptionsFilterFilter :: FilterVerificationVideoPosition => "FILTER_VERIFICATION_VIDEO_POSITION" , PathQueryOptionsFilterFilter :: FilterVerificationVideoResized => "FILTER_VERIFICATION_VIDEO_RESIZED" , PathQueryOptionsFilterFilter :: FilterVideoAdPositionInStream => "FILTER_VIDEO_AD_POSITION_IN_STREAM" , PathQueryOptionsFilterFilter :: FilterVideoCompanionCreativeSize => "FILTER_VIDEO_COMPANION_CREATIVE_SIZE" , PathQueryOptionsFilterFilter :: FilterVideoContentDuration => "FILTER_VIDEO_CONTENT_DURATION" , PathQueryOptionsFilterFilter :: FilterVideoContentLiveStream => "FILTER_VIDEO_CONTENT_LIVE_STREAM" , PathQueryOptionsFilterFilter :: FilterVideoContinuousPlay => "FILTER_VIDEO_CONTINUOUS_PLAY" , PathQueryOptionsFilterFilter :: FilterVideoCreativeDuration => "FILTER_VIDEO_CREATIVE_DURATION" , PathQueryOptionsFilterFilter :: FilterVideoCreativeDurationSkippable => "FILTER_VIDEO_CREATIVE_DURATION_SKIPPABLE" , PathQueryOptionsFilterFilter :: FilterVideoDuration => "FILTER_VIDEO_DURATION" , PathQueryOptionsFilterFilter :: FilterVideoDurationSeconds => "FILTER_VIDEO_DURATION_SECONDS" , PathQueryOptionsFilterFilter :: FilterVideoDurationSecondsRange => "FILTER_VIDEO_DURATION_SECONDS_RANGE" , PathQueryOptionsFilterFilter :: FilterVideoFormatSupport => "FILTER_VIDEO_FORMAT_SUPPORT" , PathQueryOptionsFilterFilter :: FilterVideoPlayerSize => "FILTER_VIDEO_PLAYER_SIZE" , PathQueryOptionsFilterFilter :: FilterVideoRatingTier => "FILTER_VIDEO_RATING_TIER" , PathQueryOptionsFilterFilter :: FilterVideoSkippableSupport => "FILTER_VIDEO_SKIPPABLE_SUPPORT" , PathQueryOptionsFilterFilter :: FilterWeek => "FILTER_WEEK" , PathQueryOptionsFilterFilter :: FilterYear => "FILTER_YEAR" , PathQueryOptionsFilterFilter :: FilterYoutubeAdVideo => "FILTER_YOUTUBE_AD_VIDEO" , PathQueryOptionsFilterFilter :: FilterYoutubeAdVideoId => "FILTER_YOUTUBE_AD_VIDEO_ID" , PathQueryOptionsFilterFilter :: FilterYoutubeAdaptedAudienceList => "FILTER_YOUTUBE_ADAPTED_AUDIENCE_LIST" , PathQueryOptionsFilterFilter :: FilterYoutubeChannel => "FILTER_YOUTUBE_CHANNEL" , PathQueryOptionsFilterFilter :: FilterYoutubeProgrammaticGuaranteedAdvertiser => "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_ADVERTISER" , PathQueryOptionsFilterFilter :: FilterYoutubeProgrammaticGuaranteedInsertionOrder => "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_INSERTION_ORDER" , PathQueryOptionsFilterFilter :: FilterYoutubeProgrammaticGuaranteedPartner => "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_PARTNER" , PathQueryOptionsFilterFilter :: FilterYoutubeVideo => "FILTER_YOUTUBE_VIDEO" , PathQueryOptionsFilterFilter :: FilterZipCode => "FILTER_ZIP_CODE" , PathQueryOptionsFilterFilter :: FilterZipPostalCode => "FILTER_ZIP_POSTAL_CODE" , }
        }
    }
    impl ::std::convert::AsRef<str> for PathQueryOptionsFilterFilter {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PathQueryOptionsFilterFilter {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PathQueryOptionsFilterFilter, ()> {
            Ok (match s { "FILTER_ACTIVE_VIEW_CUSTOM_METRIC_ID" => PathQueryOptionsFilterFilter :: FilterActiveViewCustomMetricId , "FILTER_ACTIVE_VIEW_CUSTOM_METRIC_NAME" => PathQueryOptionsFilterFilter :: FilterActiveViewCustomMetricName , "FILTER_ACTIVE_VIEW_EXPECTED_VIEWABILITY" => PathQueryOptionsFilterFilter :: FilterActiveViewExpectedViewability , "FILTER_AD_POSITION" => PathQueryOptionsFilterFilter :: FilterAdPosition , "FILTER_AD_TYPE" => PathQueryOptionsFilterFilter :: FilterAdType , "FILTER_ADVERTISER" => PathQueryOptionsFilterFilter :: FilterAdvertiser , "FILTER_ADVERTISER_CURRENCY" => PathQueryOptionsFilterFilter :: FilterAdvertiserCurrency , "FILTER_ADVERTISER_INTEGRATION_CODE" => PathQueryOptionsFilterFilter :: FilterAdvertiserIntegrationCode , "FILTER_ADVERTISER_INTEGRATION_STATUS" => PathQueryOptionsFilterFilter :: FilterAdvertiserIntegrationStatus , "FILTER_ADVERTISER_NAME" => PathQueryOptionsFilterFilter :: FilterAdvertiserName , "FILTER_ADVERTISER_TIMEZONE" => PathQueryOptionsFilterFilter :: FilterAdvertiserTimezone , "FILTER_AGE" => PathQueryOptionsFilterFilter :: FilterAge , "FILTER_ALGORITHM" => PathQueryOptionsFilterFilter :: FilterAlgorithm , "FILTER_ALGORITHM_ID" => PathQueryOptionsFilterFilter :: FilterAlgorithmId , "FILTER_AMP_PAGE_REQUEST" => PathQueryOptionsFilterFilter :: FilterAmpPageRequest , "FILTER_ANONYMOUS_INVENTORY_MODELING" => PathQueryOptionsFilterFilter :: FilterAnonymousInventoryModeling , "FILTER_APP_URL" => PathQueryOptionsFilterFilter :: FilterAppUrl , "FILTER_APP_URL_EXCLUDED" => PathQueryOptionsFilterFilter :: FilterAppUrlExcluded , "FILTER_ATTRIBUTED_USERLIST" => PathQueryOptionsFilterFilter :: FilterAttributedUserlist , "FILTER_ATTRIBUTED_USERLIST_COST" => PathQueryOptionsFilterFilter :: FilterAttributedUserlistCost , "FILTER_ATTRIBUTED_USERLIST_TYPE" => PathQueryOptionsFilterFilter :: FilterAttributedUserlistType , "FILTER_ATTRIBUTION_MODEL" => PathQueryOptionsFilterFilter :: FilterAttributionModel , "FILTER_AUDIENCE_LIST" => PathQueryOptionsFilterFilter :: FilterAudienceList , "FILTER_AUDIENCE_LIST_COST" => PathQueryOptionsFilterFilter :: FilterAudienceListCost , "FILTER_AUDIENCE_LIST_TYPE" => PathQueryOptionsFilterFilter :: FilterAudienceListType , "FILTER_AUDIENCE_NAME" => PathQueryOptionsFilterFilter :: FilterAudienceName , "FILTER_AUDIENCE_TYPE" => PathQueryOptionsFilterFilter :: FilterAudienceType , "FILTER_AUDIO_FEED_TYPE_NAME" => PathQueryOptionsFilterFilter :: FilterAudioFeedTypeName , "FILTER_AUTHORIZED_SELLER_STATE" => PathQueryOptionsFilterFilter :: FilterAuthorizedSellerState , "FILTER_BILLABLE_OUTCOME" => PathQueryOptionsFilterFilter :: FilterBillableOutcome , "FILTER_BRAND_LIFT_TYPE" => PathQueryOptionsFilterFilter :: FilterBrandLiftType , "FILTER_BROWSER" => PathQueryOptionsFilterFilter :: FilterBrowser , "FILTER_BUDGET_SEGMENT_BUDGET" => PathQueryOptionsFilterFilter :: FilterBudgetSegmentBudget , "FILTER_BUDGET_SEGMENT_DESCRIPTION" => PathQueryOptionsFilterFilter :: FilterBudgetSegmentDescription , "FILTER_BUDGET_SEGMENT_END_DATE" => PathQueryOptionsFilterFilter :: FilterBudgetSegmentEndDate , "FILTER_BUDGET_SEGMENT_PACING_PERCENTAGE" => PathQueryOptionsFilterFilter :: FilterBudgetSegmentPacingPercentage , "FILTER_BUDGET_SEGMENT_START_DATE" => PathQueryOptionsFilterFilter :: FilterBudgetSegmentStartDate , "FILTER_BUDGET_SEGMENT_TYPE" => PathQueryOptionsFilterFilter :: FilterBudgetSegmentType , "FILTER_CAMPAIGN_DAILY_FREQUENCY" => PathQueryOptionsFilterFilter :: FilterCampaignDailyFrequency , "FILTER_CARRIER" => PathQueryOptionsFilterFilter :: FilterCarrier , "FILTER_CARRIER_NAME" => PathQueryOptionsFilterFilter :: FilterCarrierName , "FILTER_CHANNEL_GROUPING" => PathQueryOptionsFilterFilter :: FilterChannelGrouping , "FILTER_CHANNEL_ID" => PathQueryOptionsFilterFilter :: FilterChannelId , "FILTER_CHANNEL_NAME" => PathQueryOptionsFilterFilter :: FilterChannelName , "FILTER_CHANNEL_TYPE" => PathQueryOptionsFilterFilter :: FilterChannelType , "FILTER_CITY" => PathQueryOptionsFilterFilter :: FilterCity , "FILTER_CITY_NAME" => PathQueryOptionsFilterFilter :: FilterCityName , "FILTER_CM360_PLACEMENT_ID" => PathQueryOptionsFilterFilter :: FilterCm360PlacementId , "FILTER_CM_PLACEMENT_ID" => PathQueryOptionsFilterFilter :: FilterCmPlacementId , "FILTER_COMPANION_CREATIVE_ID" => PathQueryOptionsFilterFilter :: FilterCompanionCreativeId , "FILTER_COMPANION_CREATIVE_NAME" => PathQueryOptionsFilterFilter :: FilterCompanionCreativeName , "FILTER_CONVERSION_DELAY" => PathQueryOptionsFilterFilter :: FilterConversionDelay , "FILTER_CONVERSION_SOURCE" => PathQueryOptionsFilterFilter :: FilterConversionSource , "FILTER_CONVERSION_SOURCE_ID" => PathQueryOptionsFilterFilter :: FilterConversionSourceId , "FILTER_COUNTRY" => PathQueryOptionsFilterFilter :: FilterCountry , "FILTER_COUNTRY_ID" => PathQueryOptionsFilterFilter :: FilterCountryId , "FILTER_CREATIVE" => PathQueryOptionsFilterFilter :: FilterCreative , "FILTER_CREATIVE_ASSET" => PathQueryOptionsFilterFilter :: FilterCreativeAsset , "FILTER_CREATIVE_ATTRIBUTE" => PathQueryOptionsFilterFilter :: FilterCreativeAttribute , "FILTER_CREATIVE_HEIGHT" => PathQueryOptionsFilterFilter :: FilterCreativeHeight , "FILTER_CREATIVE_ID" => PathQueryOptionsFilterFilter :: FilterCreativeId , "FILTER_CREATIVE_INTEGRATION_CODE" => PathQueryOptionsFilterFilter :: FilterCreativeIntegrationCode , "FILTER_CREATIVE_RENDERED_IN_AMP" => PathQueryOptionsFilterFilter :: FilterCreativeRenderedInAmp , "FILTER_CREATIVE_SIZE" => PathQueryOptionsFilterFilter :: FilterCreativeSize , "FILTER_CREATIVE_SOURCE" => PathQueryOptionsFilterFilter :: FilterCreativeSource , "FILTER_CREATIVE_STATUS" => PathQueryOptionsFilterFilter :: FilterCreativeStatus , "FILTER_CREATIVE_TYPE" => PathQueryOptionsFilterFilter :: FilterCreativeType , "FILTER_CREATIVE_WIDTH" => PathQueryOptionsFilterFilter :: FilterCreativeWidth , "FILTER_DATA_PROVIDER" => PathQueryOptionsFilterFilter :: FilterDataProvider , "FILTER_DATA_PROVIDER_NAME" => PathQueryOptionsFilterFilter :: FilterDataProviderName , "FILTER_DATA_SOURCE" => PathQueryOptionsFilterFilter :: FilterDataSource , "FILTER_DATE" => PathQueryOptionsFilterFilter :: FilterDate , "FILTER_DAY_OF_WEEK" => PathQueryOptionsFilterFilter :: FilterDayOfWeek , "FILTER_DETAILED_DEMOGRAPHICS" => PathQueryOptionsFilterFilter :: FilterDetailedDemographics , "FILTER_DETAILED_DEMOGRAPHICS_ID" => PathQueryOptionsFilterFilter :: FilterDetailedDemographicsId , "FILTER_DEVICE" => PathQueryOptionsFilterFilter :: FilterDevice , "FILTER_DEVICE_MAKE" => PathQueryOptionsFilterFilter :: FilterDeviceMake , "FILTER_DEVICE_MODEL" => PathQueryOptionsFilterFilter :: FilterDeviceModel , "FILTER_DEVICE_TYPE" => PathQueryOptionsFilterFilter :: FilterDeviceType , "FILTER_DFP_ORDER_ID" => PathQueryOptionsFilterFilter :: FilterDfpOrderId , "FILTER_DIGITAL_CONTENT_LABEL" => PathQueryOptionsFilterFilter :: FilterDigitalContentLabel , "FILTER_DMA" => PathQueryOptionsFilterFilter :: FilterDma , "FILTER_DMA_NAME" => PathQueryOptionsFilterFilter :: FilterDmaName , "FILTER_DOMAIN" => PathQueryOptionsFilterFilter :: FilterDomain , "FILTER_ELIGIBLE_COOKIES_ON_FIRST_PARTY_AUDIENCE_LIST" => PathQueryOptionsFilterFilter :: FilterEligibleCookiesOnFirstPartyAudienceList , "FILTER_ELIGIBLE_COOKIES_ON_THIRD_PARTY_AUDIENCE_LIST_AND_INTEREST" => PathQueryOptionsFilterFilter :: FilterEligibleCookiesOnThirdPartyAudienceListAndInterest , "FILTER_EVENT_TYPE" => PathQueryOptionsFilterFilter :: FilterEventType , "FILTER_EXCHANGE" => PathQueryOptionsFilterFilter :: FilterExchange , "FILTER_EXCHANGE_CODE" => PathQueryOptionsFilterFilter :: FilterExchangeCode , "FILTER_EXCHANGE_ID" => PathQueryOptionsFilterFilter :: FilterExchangeId , "FILTER_EXTENSION" => PathQueryOptionsFilterFilter :: FilterExtension , "FILTER_EXTENSION_STATUS" => PathQueryOptionsFilterFilter :: FilterExtensionStatus , "FILTER_EXTENSION_TYPE" => PathQueryOptionsFilterFilter :: FilterExtensionType , "FILTER_FIRST_PARTY_AUDIENCE_LIST_COST" => PathQueryOptionsFilterFilter :: FilterFirstPartyAudienceListCost , "FILTER_FIRST_PARTY_AUDIENCE_LIST_TYPE" => PathQueryOptionsFilterFilter :: FilterFirstPartyAudienceListType , "FILTER_FLOODLIGHT_ACTIVITY" => PathQueryOptionsFilterFilter :: FilterFloodlightActivity , "FILTER_FLOODLIGHT_ACTIVITY_ID" => PathQueryOptionsFilterFilter :: FilterFloodlightActivityId , "FILTER_FORMAT" => PathQueryOptionsFilterFilter :: FilterFormat , "FILTER_GAM_INSERTION_ORDER" => PathQueryOptionsFilterFilter :: FilterGamInsertionOrder , "FILTER_GAM_LINE_ITEM" => PathQueryOptionsFilterFilter :: FilterGamLineItem , "FILTER_GAM_LINE_ITEM_ID" => PathQueryOptionsFilterFilter :: FilterGamLineItemId , "FILTER_GENDER" => PathQueryOptionsFilterFilter :: FilterGender , "FILTER_GMAIL_AGE" => PathQueryOptionsFilterFilter :: FilterGmailAge , "FILTER_GMAIL_CITY" => PathQueryOptionsFilterFilter :: FilterGmailCity , "FILTER_GMAIL_COUNTRY" => PathQueryOptionsFilterFilter :: FilterGmailCountry , "FILTER_GMAIL_COUNTRY_NAME" => PathQueryOptionsFilterFilter :: FilterGmailCountryName , "FILTER_GMAIL_DEVICE_TYPE" => PathQueryOptionsFilterFilter :: FilterGmailDeviceType , "FILTER_GMAIL_DEVICE_TYPE_NAME" => PathQueryOptionsFilterFilter :: FilterGmailDeviceTypeName , "FILTER_GMAIL_GENDER" => PathQueryOptionsFilterFilter :: FilterGmailGender , "FILTER_GMAIL_REGION" => PathQueryOptionsFilterFilter :: FilterGmailRegion , "FILTER_GMAIL_REMARKETING_LIST" => PathQueryOptionsFilterFilter :: FilterGmailRemarketingList , "FILTER_HOUSEHOLD_INCOME" => PathQueryOptionsFilterFilter :: FilterHouseholdIncome , "FILTER_IMPRESSION_COUNTING_METHOD" => PathQueryOptionsFilterFilter :: FilterImpressionCountingMethod , "FILTER_IMPRESSION_LOSS_REJECTION_REASON" => PathQueryOptionsFilterFilter :: FilterImpressionLossRejectionReason , "FILTER_INSERTION_ORDER" => PathQueryOptionsFilterFilter :: FilterInsertionOrder , "FILTER_INSERTION_ORDER_GOAL_TYPE" => PathQueryOptionsFilterFilter :: FilterInsertionOrderGoalType , "FILTER_INSERTION_ORDER_GOAL_VALUE" => PathQueryOptionsFilterFilter :: FilterInsertionOrderGoalValue , "FILTER_INSERTION_ORDER_INTEGRATION_CODE" => PathQueryOptionsFilterFilter :: FilterInsertionOrderIntegrationCode , "FILTER_INSERTION_ORDER_NAME" => PathQueryOptionsFilterFilter :: FilterInsertionOrderName , "FILTER_INSERTION_ORDER_STATUS" => PathQueryOptionsFilterFilter :: FilterInsertionOrderStatus , "FILTER_INTEREST" => PathQueryOptionsFilterFilter :: FilterInterest , "FILTER_INVENTORY_COMMITMENT_TYPE" => PathQueryOptionsFilterFilter :: FilterInventoryCommitmentType , "FILTER_INVENTORY_DELIVERY_METHOD" => PathQueryOptionsFilterFilter :: FilterInventoryDeliveryMethod , "FILTER_INVENTORY_FORMAT" => PathQueryOptionsFilterFilter :: FilterInventoryFormat , "FILTER_INVENTORY_RATE_TYPE" => PathQueryOptionsFilterFilter :: FilterInventoryRateType , "FILTER_INVENTORY_SOURCE" => PathQueryOptionsFilterFilter :: FilterInventorySource , "FILTER_INVENTORY_SOURCE_EXTERNAL_ID" => PathQueryOptionsFilterFilter :: FilterInventorySourceExternalId , "FILTER_INVENTORY_SOURCE_GROUP" => PathQueryOptionsFilterFilter :: FilterInventorySourceGroup , "FILTER_INVENTORY_SOURCE_GROUP_ID" => PathQueryOptionsFilterFilter :: FilterInventorySourceGroupId , "FILTER_INVENTORY_SOURCE_ID" => PathQueryOptionsFilterFilter :: FilterInventorySourceId , "FILTER_INVENTORY_SOURCE_NAME" => PathQueryOptionsFilterFilter :: FilterInventorySourceName , "FILTER_INVENTORY_SOURCE_TYPE" => PathQueryOptionsFilterFilter :: FilterInventorySourceType , "FILTER_KEYWORD" => PathQueryOptionsFilterFilter :: FilterKeyword , "FILTER_LIFE_EVENT" => PathQueryOptionsFilterFilter :: FilterLifeEvent , "FILTER_LIFE_EVENTS" => PathQueryOptionsFilterFilter :: FilterLifeEvents , "FILTER_LINE_ITEM" => PathQueryOptionsFilterFilter :: FilterLineItem , "FILTER_LINE_ITEM_BUDGET" => PathQueryOptionsFilterFilter :: FilterLineItemBudget , "FILTER_LINE_ITEM_DAILY_FREQUENCY" => PathQueryOptionsFilterFilter :: FilterLineItemDailyFrequency , "FILTER_LINE_ITEM_END_DATE" => PathQueryOptionsFilterFilter :: FilterLineItemEndDate , "FILTER_LINE_ITEM_INTEGRATION_CODE" => PathQueryOptionsFilterFilter :: FilterLineItemIntegrationCode , "FILTER_LINE_ITEM_LIFETIME_FREQUENCY" => PathQueryOptionsFilterFilter :: FilterLineItemLifetimeFrequency , "FILTER_LINE_ITEM_NAME" => PathQueryOptionsFilterFilter :: FilterLineItemName , "FILTER_LINE_ITEM_PACING_PERCENTAGE" => PathQueryOptionsFilterFilter :: FilterLineItemPacingPercentage , "FILTER_LINE_ITEM_START_DATE" => PathQueryOptionsFilterFilter :: FilterLineItemStartDate , "FILTER_LINE_ITEM_STATUS" => PathQueryOptionsFilterFilter :: FilterLineItemStatus , "FILTER_LINE_ITEM_TYPE" => PathQueryOptionsFilterFilter :: FilterLineItemType , "FILTER_MATCH_RATIO" => PathQueryOptionsFilterFilter :: FilterMatchRatio , "FILTER_MATCHED_GENRE_TARGET" => PathQueryOptionsFilterFilter :: FilterMatchedGenreTarget , "FILTER_MEASUREMENT_SOURCE" => PathQueryOptionsFilterFilter :: FilterMeasurementSource , "FILTER_MEDIA_PLAN" => PathQueryOptionsFilterFilter :: FilterMediaPlan , "FILTER_MEDIA_PLAN_NAME" => PathQueryOptionsFilterFilter :: FilterMediaPlanName , "FILTER_MEDIA_TYPE" => PathQueryOptionsFilterFilter :: FilterMediaType , "FILTER_MOBILE_GEO" => PathQueryOptionsFilterFilter :: FilterMobileGeo , "FILTER_MONTH" => PathQueryOptionsFilterFilter :: FilterMonth , "FILTER_MRAID_SUPPORT" => PathQueryOptionsFilterFilter :: FilterMraidSupport , "FILTER_NIELSEN_AGE" => PathQueryOptionsFilterFilter :: FilterNielsenAge , "FILTER_NIELSEN_COUNTRY_CODE" => PathQueryOptionsFilterFilter :: FilterNielsenCountryCode , "FILTER_NIELSEN_DATE_RANGE" => PathQueryOptionsFilterFilter :: FilterNielsenDateRange , "FILTER_NIELSEN_DEVICE_ID" => PathQueryOptionsFilterFilter :: FilterNielsenDeviceId , "FILTER_NIELSEN_GENDER" => PathQueryOptionsFilterFilter :: FilterNielsenGender , "FILTER_NIELSEN_RESTATEMENT_DATE" => PathQueryOptionsFilterFilter :: FilterNielsenRestatementDate , "FILTER_NOT_SUPPORTED" => PathQueryOptionsFilterFilter :: FilterNotSupported , "FILTER_OM_SDK_AVAILABLE" => PathQueryOptionsFilterFilter :: FilterOmSdkAvailable , "FILTER_OMID_CAPABLE" => PathQueryOptionsFilterFilter :: FilterOmidCapable , "FILTER_ORDER_ID" => PathQueryOptionsFilterFilter :: FilterOrderId , "FILTER_OS" => PathQueryOptionsFilterFilter :: FilterOs , "FILTER_PAGE_CATEGORY" => PathQueryOptionsFilterFilter :: FilterPageCategory , "FILTER_PAGE_LAYOUT" => PathQueryOptionsFilterFilter :: FilterPageLayout , "FILTER_PARENTAL_STATUS" => PathQueryOptionsFilterFilter :: FilterParentalStatus , "FILTER_PARTNER" => PathQueryOptionsFilterFilter :: FilterPartner , "FILTER_PARTNER_CURRENCY" => PathQueryOptionsFilterFilter :: FilterPartnerCurrency , "FILTER_PARTNER_NAME" => PathQueryOptionsFilterFilter :: FilterPartnerName , "FILTER_PARTNER_STATUS" => PathQueryOptionsFilterFilter :: FilterPartnerStatus , "FILTER_PATH_EVENT_INDEX" => PathQueryOptionsFilterFilter :: FilterPathEventIndex , "FILTER_PATH_PATTERN_ID" => PathQueryOptionsFilterFilter :: FilterPathPatternId , "FILTER_PLACEMENT_ALL_YOUTUBE_CHANNELS" => PathQueryOptionsFilterFilter :: FilterPlacementAllYoutubeChannels , "FILTER_PLACEMENT_NAME_ALL_YOUTUBE_CHANNELS" => PathQueryOptionsFilterFilter :: FilterPlacementNameAllYoutubeChannels , "FILTER_PLATFORM" => PathQueryOptionsFilterFilter :: FilterPlatform , "FILTER_PLAYBACK_METHOD" => PathQueryOptionsFilterFilter :: FilterPlaybackMethod , "FILTER_POSITION_IN_CONTENT" => PathQueryOptionsFilterFilter :: FilterPositionInContent , "FILTER_PUBLIC_INVENTORY" => PathQueryOptionsFilterFilter :: FilterPublicInventory , "FILTER_PUBLISHER_PROPERTY" => PathQueryOptionsFilterFilter :: FilterPublisherProperty , "FILTER_PUBLISHER_PROPERTY_ID" => PathQueryOptionsFilterFilter :: FilterPublisherPropertyId , "FILTER_PUBLISHER_PROPERTY_SECTION" => PathQueryOptionsFilterFilter :: FilterPublisherPropertySection , "FILTER_PUBLISHER_PROPERTY_SECTION_ID" => PathQueryOptionsFilterFilter :: FilterPublisherPropertySectionId , "FILTER_QUARTER" => PathQueryOptionsFilterFilter :: FilterQuarter , "FILTER_REFUND_REASON" => PathQueryOptionsFilterFilter :: FilterRefundReason , "FILTER_REGION" => PathQueryOptionsFilterFilter :: FilterRegion , "FILTER_REGION_NAME" => PathQueryOptionsFilterFilter :: FilterRegionName , "FILTER_REMARKETING_LIST" => PathQueryOptionsFilterFilter :: FilterRemarketingList , "FILTER_REWARDED" => PathQueryOptionsFilterFilter :: FilterRewarded , "FILTER_SENSITIVE_CATEGORY" => PathQueryOptionsFilterFilter :: FilterSensitiveCategory , "FILTER_SERVED_PIXEL_DENSITY" => PathQueryOptionsFilterFilter :: FilterServedPixelDensity , "FILTER_SITE_ID" => PathQueryOptionsFilterFilter :: FilterSiteId , "FILTER_SITE_LANGUAGE" => PathQueryOptionsFilterFilter :: FilterSiteLanguage , "FILTER_SKIPPABLE_SUPPORT" => PathQueryOptionsFilterFilter :: FilterSkippableSupport , "FILTER_TARGETED_DATA_PROVIDERS" => PathQueryOptionsFilterFilter :: FilterTargetedDataProviders , "FILTER_TARGETED_USER_LIST" => PathQueryOptionsFilterFilter :: FilterTargetedUserList , "FILTER_THIRD_PARTY_AUDIENCE_LIST_COST" => PathQueryOptionsFilterFilter :: FilterThirdPartyAudienceListCost , "FILTER_THIRD_PARTY_AUDIENCE_LIST_TYPE" => PathQueryOptionsFilterFilter :: FilterThirdPartyAudienceListType , "FILTER_TIME_OF_DAY" => PathQueryOptionsFilterFilter :: FilterTimeOfDay , "FILTER_TRUEVIEW_AD" => PathQueryOptionsFilterFilter :: FilterTrueviewAd , "FILTER_TRUEVIEW_AD_GROUP" => PathQueryOptionsFilterFilter :: FilterTrueviewAdGroup , "FILTER_TRUEVIEW_AD_GROUP_AD_ID" => PathQueryOptionsFilterFilter :: FilterTrueviewAdGroupAdId , "FILTER_TRUEVIEW_AD_GROUP_ID" => PathQueryOptionsFilterFilter :: FilterTrueviewAdGroupId , "FILTER_TRUEVIEW_AD_TYPE_NAME" => PathQueryOptionsFilterFilter :: FilterTrueviewAdTypeName , "FILTER_TRUEVIEW_AGE" => PathQueryOptionsFilterFilter :: FilterTrueviewAge , "FILTER_TRUEVIEW_CATEGORY" => PathQueryOptionsFilterFilter :: FilterTrueviewCategory , "FILTER_TRUEVIEW_CITY" => PathQueryOptionsFilterFilter :: FilterTrueviewCity , "FILTER_TRUEVIEW_CLICK_TYPE_NAME" => PathQueryOptionsFilterFilter :: FilterTrueviewClickTypeName , "FILTER_TRUEVIEW_CONVERSION_TYPE" => PathQueryOptionsFilterFilter :: FilterTrueviewConversionType , "FILTER_TRUEVIEW_COUNTRY" => PathQueryOptionsFilterFilter :: FilterTrueviewCountry , "FILTER_TRUEVIEW_CUSTOM_AFFINITY" => PathQueryOptionsFilterFilter :: FilterTrueviewCustomAffinity , "FILTER_TRUEVIEW_DETAILED_DEMOGRAPHICS" => PathQueryOptionsFilterFilter :: FilterTrueviewDetailedDemographics , "FILTER_TRUEVIEW_DETAILED_DEMOGRAPHICS_ID" => PathQueryOptionsFilterFilter :: FilterTrueviewDetailedDemographicsId , "FILTER_TRUEVIEW_DMA" => PathQueryOptionsFilterFilter :: FilterTrueviewDma , "FILTER_TRUEVIEW_DMA_NAME" => PathQueryOptionsFilterFilter :: FilterTrueviewDmaName , "FILTER_TRUEVIEW_GENDER" => PathQueryOptionsFilterFilter :: FilterTrueviewGender , "FILTER_TRUEVIEW_HOUSEHOLD_INCOME" => PathQueryOptionsFilterFilter :: FilterTrueviewHouseholdIncome , "FILTER_TRUEVIEW_IAR_AGE" => PathQueryOptionsFilterFilter :: FilterTrueviewIarAge , "FILTER_TRUEVIEW_IAR_CATEGORY" => PathQueryOptionsFilterFilter :: FilterTrueviewIarCategory , "FILTER_TRUEVIEW_IAR_CITY" => PathQueryOptionsFilterFilter :: FilterTrueviewIarCity , "FILTER_TRUEVIEW_IAR_COUNTRY" => PathQueryOptionsFilterFilter :: FilterTrueviewIarCountry , "FILTER_TRUEVIEW_IAR_COUNTRY_NAME" => PathQueryOptionsFilterFilter :: FilterTrueviewIarCountryName , "FILTER_TRUEVIEW_IAR_GENDER" => PathQueryOptionsFilterFilter :: FilterTrueviewIarGender , "FILTER_TRUEVIEW_IAR_INTEREST" => PathQueryOptionsFilterFilter :: FilterTrueviewIarInterest , "FILTER_TRUEVIEW_IAR_LANGUAGE" => PathQueryOptionsFilterFilter :: FilterTrueviewIarLanguage , "FILTER_TRUEVIEW_IAR_PARENTAL_STATUS" => PathQueryOptionsFilterFilter :: FilterTrueviewIarParentalStatus , "FILTER_TRUEVIEW_IAR_REGION" => PathQueryOptionsFilterFilter :: FilterTrueviewIarRegion , "FILTER_TRUEVIEW_IAR_REGION_NAME" => PathQueryOptionsFilterFilter :: FilterTrueviewIarRegionName , "FILTER_TRUEVIEW_IAR_REMARKETING_LIST" => PathQueryOptionsFilterFilter :: FilterTrueviewIarRemarketingList , "FILTER_TRUEVIEW_IAR_TIME_OF_DAY" => PathQueryOptionsFilterFilter :: FilterTrueviewIarTimeOfDay , "FILTER_TRUEVIEW_IAR_YOUTUBE_CHANNEL" => PathQueryOptionsFilterFilter :: FilterTrueviewIarYoutubeChannel , "FILTER_TRUEVIEW_IAR_YOUTUBE_VIDEO" => PathQueryOptionsFilterFilter :: FilterTrueviewIarYoutubeVideo , "FILTER_TRUEVIEW_IAR_ZIPCODE" => PathQueryOptionsFilterFilter :: FilterTrueviewIarZipcode , "FILTER_TRUEVIEW_INTEREST" => PathQueryOptionsFilterFilter :: FilterTrueviewInterest , "FILTER_TRUEVIEW_KEYWORD" => PathQueryOptionsFilterFilter :: FilterTrueviewKeyword , "FILTER_TRUEVIEW_PARENTAL_STATUS" => PathQueryOptionsFilterFilter :: FilterTrueviewParentalStatus , "FILTER_TRUEVIEW_PLACEMENT" => PathQueryOptionsFilterFilter :: FilterTrueviewPlacement , "FILTER_TRUEVIEW_PLACEMENT_ID" => PathQueryOptionsFilterFilter :: FilterTrueviewPlacementId , "FILTER_TRUEVIEW_REGION" => PathQueryOptionsFilterFilter :: FilterTrueviewRegion , "FILTER_TRUEVIEW_REGION_NAME" => PathQueryOptionsFilterFilter :: FilterTrueviewRegionName , "FILTER_TRUEVIEW_REMARKETING_LIST" => PathQueryOptionsFilterFilter :: FilterTrueviewRemarketingList , "FILTER_TRUEVIEW_REMARKETING_LIST_NAME" => PathQueryOptionsFilterFilter :: FilterTrueviewRemarketingListName , "FILTER_TRUEVIEW_URL" => PathQueryOptionsFilterFilter :: FilterTrueviewUrl , "FILTER_TRUEVIEW_ZIPCODE" => PathQueryOptionsFilterFilter :: FilterTrueviewZipcode , "FILTER_UNKNOWN" => PathQueryOptionsFilterFilter :: FilterUnknown , "FILTER_USER_LIST" => PathQueryOptionsFilterFilter :: FilterUserList , "FILTER_USER_LIST_FIRST_PARTY" => PathQueryOptionsFilterFilter :: FilterUserListFirstParty , "FILTER_USER_LIST_FIRST_PARTY_NAME" => PathQueryOptionsFilterFilter :: FilterUserListFirstPartyName , "FILTER_USER_LIST_THIRD_PARTY" => PathQueryOptionsFilterFilter :: FilterUserListThirdParty , "FILTER_USER_LIST_THIRD_PARTY_NAME" => PathQueryOptionsFilterFilter :: FilterUserListThirdPartyName , "FILTER_VARIANT_ID" => PathQueryOptionsFilterFilter :: FilterVariantId , "FILTER_VARIANT_NAME" => PathQueryOptionsFilterFilter :: FilterVariantName , "FILTER_VARIANT_VERSION" => PathQueryOptionsFilterFilter :: FilterVariantVersion , "FILTER_VENDOR_MEASUREMENT_MODE" => PathQueryOptionsFilterFilter :: FilterVendorMeasurementMode , "FILTER_VERIFICATION_AUDIBILITY_COMPLETE" => PathQueryOptionsFilterFilter :: FilterVerificationAudibilityComplete , "FILTER_VERIFICATION_AUDIBILITY_START" => PathQueryOptionsFilterFilter :: FilterVerificationAudibilityStart , "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE" => PathQueryOptionsFilterFilter :: FilterVerificationVideoPlayerSize , "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_COMPLETE" => PathQueryOptionsFilterFilter :: FilterVerificationVideoPlayerSizeComplete , "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_FIRST_QUARTILE" => PathQueryOptionsFilterFilter :: FilterVerificationVideoPlayerSizeFirstQuartile , "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_MID_POINT" => PathQueryOptionsFilterFilter :: FilterVerificationVideoPlayerSizeMidPoint , "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_START" => PathQueryOptionsFilterFilter :: FilterVerificationVideoPlayerSizeStart , "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_THIRD_QUARTILE" => PathQueryOptionsFilterFilter :: FilterVerificationVideoPlayerSizeThirdQuartile , "FILTER_VERIFICATION_VIDEO_POSITION" => PathQueryOptionsFilterFilter :: FilterVerificationVideoPosition , "FILTER_VERIFICATION_VIDEO_RESIZED" => PathQueryOptionsFilterFilter :: FilterVerificationVideoResized , "FILTER_VIDEO_AD_POSITION_IN_STREAM" => PathQueryOptionsFilterFilter :: FilterVideoAdPositionInStream , "FILTER_VIDEO_COMPANION_CREATIVE_SIZE" => PathQueryOptionsFilterFilter :: FilterVideoCompanionCreativeSize , "FILTER_VIDEO_CONTENT_DURATION" => PathQueryOptionsFilterFilter :: FilterVideoContentDuration , "FILTER_VIDEO_CONTENT_LIVE_STREAM" => PathQueryOptionsFilterFilter :: FilterVideoContentLiveStream , "FILTER_VIDEO_CONTINUOUS_PLAY" => PathQueryOptionsFilterFilter :: FilterVideoContinuousPlay , "FILTER_VIDEO_CREATIVE_DURATION" => PathQueryOptionsFilterFilter :: FilterVideoCreativeDuration , "FILTER_VIDEO_CREATIVE_DURATION_SKIPPABLE" => PathQueryOptionsFilterFilter :: FilterVideoCreativeDurationSkippable , "FILTER_VIDEO_DURATION" => PathQueryOptionsFilterFilter :: FilterVideoDuration , "FILTER_VIDEO_DURATION_SECONDS" => PathQueryOptionsFilterFilter :: FilterVideoDurationSeconds , "FILTER_VIDEO_DURATION_SECONDS_RANGE" => PathQueryOptionsFilterFilter :: FilterVideoDurationSecondsRange , "FILTER_VIDEO_FORMAT_SUPPORT" => PathQueryOptionsFilterFilter :: FilterVideoFormatSupport , "FILTER_VIDEO_PLAYER_SIZE" => PathQueryOptionsFilterFilter :: FilterVideoPlayerSize , "FILTER_VIDEO_RATING_TIER" => PathQueryOptionsFilterFilter :: FilterVideoRatingTier , "FILTER_VIDEO_SKIPPABLE_SUPPORT" => PathQueryOptionsFilterFilter :: FilterVideoSkippableSupport , "FILTER_WEEK" => PathQueryOptionsFilterFilter :: FilterWeek , "FILTER_YEAR" => PathQueryOptionsFilterFilter :: FilterYear , "FILTER_YOUTUBE_AD_VIDEO" => PathQueryOptionsFilterFilter :: FilterYoutubeAdVideo , "FILTER_YOUTUBE_AD_VIDEO_ID" => PathQueryOptionsFilterFilter :: FilterYoutubeAdVideoId , "FILTER_YOUTUBE_ADAPTED_AUDIENCE_LIST" => PathQueryOptionsFilterFilter :: FilterYoutubeAdaptedAudienceList , "FILTER_YOUTUBE_CHANNEL" => PathQueryOptionsFilterFilter :: FilterYoutubeChannel , "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_ADVERTISER" => PathQueryOptionsFilterFilter :: FilterYoutubeProgrammaticGuaranteedAdvertiser , "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_INSERTION_ORDER" => PathQueryOptionsFilterFilter :: FilterYoutubeProgrammaticGuaranteedInsertionOrder , "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_PARTNER" => PathQueryOptionsFilterFilter :: FilterYoutubeProgrammaticGuaranteedPartner , "FILTER_YOUTUBE_VIDEO" => PathQueryOptionsFilterFilter :: FilterYoutubeVideo , "FILTER_ZIP_CODE" => PathQueryOptionsFilterFilter :: FilterZipCode , "FILTER_ZIP_POSTAL_CODE" => PathQueryOptionsFilterFilter :: FilterZipPostalCode , _ => return Err (()) , })
        }
    }
    impl ::std::fmt::Display for PathQueryOptionsFilterFilter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PathQueryOptionsFilterFilter {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PathQueryOptionsFilterFilter {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok (match value { "FILTER_ACTIVE_VIEW_CUSTOM_METRIC_ID" => PathQueryOptionsFilterFilter :: FilterActiveViewCustomMetricId , "FILTER_ACTIVE_VIEW_CUSTOM_METRIC_NAME" => PathQueryOptionsFilterFilter :: FilterActiveViewCustomMetricName , "FILTER_ACTIVE_VIEW_EXPECTED_VIEWABILITY" => PathQueryOptionsFilterFilter :: FilterActiveViewExpectedViewability , "FILTER_AD_POSITION" => PathQueryOptionsFilterFilter :: FilterAdPosition , "FILTER_AD_TYPE" => PathQueryOptionsFilterFilter :: FilterAdType , "FILTER_ADVERTISER" => PathQueryOptionsFilterFilter :: FilterAdvertiser , "FILTER_ADVERTISER_CURRENCY" => PathQueryOptionsFilterFilter :: FilterAdvertiserCurrency , "FILTER_ADVERTISER_INTEGRATION_CODE" => PathQueryOptionsFilterFilter :: FilterAdvertiserIntegrationCode , "FILTER_ADVERTISER_INTEGRATION_STATUS" => PathQueryOptionsFilterFilter :: FilterAdvertiserIntegrationStatus , "FILTER_ADVERTISER_NAME" => PathQueryOptionsFilterFilter :: FilterAdvertiserName , "FILTER_ADVERTISER_TIMEZONE" => PathQueryOptionsFilterFilter :: FilterAdvertiserTimezone , "FILTER_AGE" => PathQueryOptionsFilterFilter :: FilterAge , "FILTER_ALGORITHM" => PathQueryOptionsFilterFilter :: FilterAlgorithm , "FILTER_ALGORITHM_ID" => PathQueryOptionsFilterFilter :: FilterAlgorithmId , "FILTER_AMP_PAGE_REQUEST" => PathQueryOptionsFilterFilter :: FilterAmpPageRequest , "FILTER_ANONYMOUS_INVENTORY_MODELING" => PathQueryOptionsFilterFilter :: FilterAnonymousInventoryModeling , "FILTER_APP_URL" => PathQueryOptionsFilterFilter :: FilterAppUrl , "FILTER_APP_URL_EXCLUDED" => PathQueryOptionsFilterFilter :: FilterAppUrlExcluded , "FILTER_ATTRIBUTED_USERLIST" => PathQueryOptionsFilterFilter :: FilterAttributedUserlist , "FILTER_ATTRIBUTED_USERLIST_COST" => PathQueryOptionsFilterFilter :: FilterAttributedUserlistCost , "FILTER_ATTRIBUTED_USERLIST_TYPE" => PathQueryOptionsFilterFilter :: FilterAttributedUserlistType , "FILTER_ATTRIBUTION_MODEL" => PathQueryOptionsFilterFilter :: FilterAttributionModel , "FILTER_AUDIENCE_LIST" => PathQueryOptionsFilterFilter :: FilterAudienceList , "FILTER_AUDIENCE_LIST_COST" => PathQueryOptionsFilterFilter :: FilterAudienceListCost , "FILTER_AUDIENCE_LIST_TYPE" => PathQueryOptionsFilterFilter :: FilterAudienceListType , "FILTER_AUDIENCE_NAME" => PathQueryOptionsFilterFilter :: FilterAudienceName , "FILTER_AUDIENCE_TYPE" => PathQueryOptionsFilterFilter :: FilterAudienceType , "FILTER_AUDIO_FEED_TYPE_NAME" => PathQueryOptionsFilterFilter :: FilterAudioFeedTypeName , "FILTER_AUTHORIZED_SELLER_STATE" => PathQueryOptionsFilterFilter :: FilterAuthorizedSellerState , "FILTER_BILLABLE_OUTCOME" => PathQueryOptionsFilterFilter :: FilterBillableOutcome , "FILTER_BRAND_LIFT_TYPE" => PathQueryOptionsFilterFilter :: FilterBrandLiftType , "FILTER_BROWSER" => PathQueryOptionsFilterFilter :: FilterBrowser , "FILTER_BUDGET_SEGMENT_BUDGET" => PathQueryOptionsFilterFilter :: FilterBudgetSegmentBudget , "FILTER_BUDGET_SEGMENT_DESCRIPTION" => PathQueryOptionsFilterFilter :: FilterBudgetSegmentDescription , "FILTER_BUDGET_SEGMENT_END_DATE" => PathQueryOptionsFilterFilter :: FilterBudgetSegmentEndDate , "FILTER_BUDGET_SEGMENT_PACING_PERCENTAGE" => PathQueryOptionsFilterFilter :: FilterBudgetSegmentPacingPercentage , "FILTER_BUDGET_SEGMENT_START_DATE" => PathQueryOptionsFilterFilter :: FilterBudgetSegmentStartDate , "FILTER_BUDGET_SEGMENT_TYPE" => PathQueryOptionsFilterFilter :: FilterBudgetSegmentType , "FILTER_CAMPAIGN_DAILY_FREQUENCY" => PathQueryOptionsFilterFilter :: FilterCampaignDailyFrequency , "FILTER_CARRIER" => PathQueryOptionsFilterFilter :: FilterCarrier , "FILTER_CARRIER_NAME" => PathQueryOptionsFilterFilter :: FilterCarrierName , "FILTER_CHANNEL_GROUPING" => PathQueryOptionsFilterFilter :: FilterChannelGrouping , "FILTER_CHANNEL_ID" => PathQueryOptionsFilterFilter :: FilterChannelId , "FILTER_CHANNEL_NAME" => PathQueryOptionsFilterFilter :: FilterChannelName , "FILTER_CHANNEL_TYPE" => PathQueryOptionsFilterFilter :: FilterChannelType , "FILTER_CITY" => PathQueryOptionsFilterFilter :: FilterCity , "FILTER_CITY_NAME" => PathQueryOptionsFilterFilter :: FilterCityName , "FILTER_CM360_PLACEMENT_ID" => PathQueryOptionsFilterFilter :: FilterCm360PlacementId , "FILTER_CM_PLACEMENT_ID" => PathQueryOptionsFilterFilter :: FilterCmPlacementId , "FILTER_COMPANION_CREATIVE_ID" => PathQueryOptionsFilterFilter :: FilterCompanionCreativeId , "FILTER_COMPANION_CREATIVE_NAME" => PathQueryOptionsFilterFilter :: FilterCompanionCreativeName , "FILTER_CONVERSION_DELAY" => PathQueryOptionsFilterFilter :: FilterConversionDelay , "FILTER_CONVERSION_SOURCE" => PathQueryOptionsFilterFilter :: FilterConversionSource , "FILTER_CONVERSION_SOURCE_ID" => PathQueryOptionsFilterFilter :: FilterConversionSourceId , "FILTER_COUNTRY" => PathQueryOptionsFilterFilter :: FilterCountry , "FILTER_COUNTRY_ID" => PathQueryOptionsFilterFilter :: FilterCountryId , "FILTER_CREATIVE" => PathQueryOptionsFilterFilter :: FilterCreative , "FILTER_CREATIVE_ASSET" => PathQueryOptionsFilterFilter :: FilterCreativeAsset , "FILTER_CREATIVE_ATTRIBUTE" => PathQueryOptionsFilterFilter :: FilterCreativeAttribute , "FILTER_CREATIVE_HEIGHT" => PathQueryOptionsFilterFilter :: FilterCreativeHeight , "FILTER_CREATIVE_ID" => PathQueryOptionsFilterFilter :: FilterCreativeId , "FILTER_CREATIVE_INTEGRATION_CODE" => PathQueryOptionsFilterFilter :: FilterCreativeIntegrationCode , "FILTER_CREATIVE_RENDERED_IN_AMP" => PathQueryOptionsFilterFilter :: FilterCreativeRenderedInAmp , "FILTER_CREATIVE_SIZE" => PathQueryOptionsFilterFilter :: FilterCreativeSize , "FILTER_CREATIVE_SOURCE" => PathQueryOptionsFilterFilter :: FilterCreativeSource , "FILTER_CREATIVE_STATUS" => PathQueryOptionsFilterFilter :: FilterCreativeStatus , "FILTER_CREATIVE_TYPE" => PathQueryOptionsFilterFilter :: FilterCreativeType , "FILTER_CREATIVE_WIDTH" => PathQueryOptionsFilterFilter :: FilterCreativeWidth , "FILTER_DATA_PROVIDER" => PathQueryOptionsFilterFilter :: FilterDataProvider , "FILTER_DATA_PROVIDER_NAME" => PathQueryOptionsFilterFilter :: FilterDataProviderName , "FILTER_DATA_SOURCE" => PathQueryOptionsFilterFilter :: FilterDataSource , "FILTER_DATE" => PathQueryOptionsFilterFilter :: FilterDate , "FILTER_DAY_OF_WEEK" => PathQueryOptionsFilterFilter :: FilterDayOfWeek , "FILTER_DETAILED_DEMOGRAPHICS" => PathQueryOptionsFilterFilter :: FilterDetailedDemographics , "FILTER_DETAILED_DEMOGRAPHICS_ID" => PathQueryOptionsFilterFilter :: FilterDetailedDemographicsId , "FILTER_DEVICE" => PathQueryOptionsFilterFilter :: FilterDevice , "FILTER_DEVICE_MAKE" => PathQueryOptionsFilterFilter :: FilterDeviceMake , "FILTER_DEVICE_MODEL" => PathQueryOptionsFilterFilter :: FilterDeviceModel , "FILTER_DEVICE_TYPE" => PathQueryOptionsFilterFilter :: FilterDeviceType , "FILTER_DFP_ORDER_ID" => PathQueryOptionsFilterFilter :: FilterDfpOrderId , "FILTER_DIGITAL_CONTENT_LABEL" => PathQueryOptionsFilterFilter :: FilterDigitalContentLabel , "FILTER_DMA" => PathQueryOptionsFilterFilter :: FilterDma , "FILTER_DMA_NAME" => PathQueryOptionsFilterFilter :: FilterDmaName , "FILTER_DOMAIN" => PathQueryOptionsFilterFilter :: FilterDomain , "FILTER_ELIGIBLE_COOKIES_ON_FIRST_PARTY_AUDIENCE_LIST" => PathQueryOptionsFilterFilter :: FilterEligibleCookiesOnFirstPartyAudienceList , "FILTER_ELIGIBLE_COOKIES_ON_THIRD_PARTY_AUDIENCE_LIST_AND_INTEREST" => PathQueryOptionsFilterFilter :: FilterEligibleCookiesOnThirdPartyAudienceListAndInterest , "FILTER_EVENT_TYPE" => PathQueryOptionsFilterFilter :: FilterEventType , "FILTER_EXCHANGE" => PathQueryOptionsFilterFilter :: FilterExchange , "FILTER_EXCHANGE_CODE" => PathQueryOptionsFilterFilter :: FilterExchangeCode , "FILTER_EXCHANGE_ID" => PathQueryOptionsFilterFilter :: FilterExchangeId , "FILTER_EXTENSION" => PathQueryOptionsFilterFilter :: FilterExtension , "FILTER_EXTENSION_STATUS" => PathQueryOptionsFilterFilter :: FilterExtensionStatus , "FILTER_EXTENSION_TYPE" => PathQueryOptionsFilterFilter :: FilterExtensionType , "FILTER_FIRST_PARTY_AUDIENCE_LIST_COST" => PathQueryOptionsFilterFilter :: FilterFirstPartyAudienceListCost , "FILTER_FIRST_PARTY_AUDIENCE_LIST_TYPE" => PathQueryOptionsFilterFilter :: FilterFirstPartyAudienceListType , "FILTER_FLOODLIGHT_ACTIVITY" => PathQueryOptionsFilterFilter :: FilterFloodlightActivity , "FILTER_FLOODLIGHT_ACTIVITY_ID" => PathQueryOptionsFilterFilter :: FilterFloodlightActivityId , "FILTER_FORMAT" => PathQueryOptionsFilterFilter :: FilterFormat , "FILTER_GAM_INSERTION_ORDER" => PathQueryOptionsFilterFilter :: FilterGamInsertionOrder , "FILTER_GAM_LINE_ITEM" => PathQueryOptionsFilterFilter :: FilterGamLineItem , "FILTER_GAM_LINE_ITEM_ID" => PathQueryOptionsFilterFilter :: FilterGamLineItemId , "FILTER_GENDER" => PathQueryOptionsFilterFilter :: FilterGender , "FILTER_GMAIL_AGE" => PathQueryOptionsFilterFilter :: FilterGmailAge , "FILTER_GMAIL_CITY" => PathQueryOptionsFilterFilter :: FilterGmailCity , "FILTER_GMAIL_COUNTRY" => PathQueryOptionsFilterFilter :: FilterGmailCountry , "FILTER_GMAIL_COUNTRY_NAME" => PathQueryOptionsFilterFilter :: FilterGmailCountryName , "FILTER_GMAIL_DEVICE_TYPE" => PathQueryOptionsFilterFilter :: FilterGmailDeviceType , "FILTER_GMAIL_DEVICE_TYPE_NAME" => PathQueryOptionsFilterFilter :: FilterGmailDeviceTypeName , "FILTER_GMAIL_GENDER" => PathQueryOptionsFilterFilter :: FilterGmailGender , "FILTER_GMAIL_REGION" => PathQueryOptionsFilterFilter :: FilterGmailRegion , "FILTER_GMAIL_REMARKETING_LIST" => PathQueryOptionsFilterFilter :: FilterGmailRemarketingList , "FILTER_HOUSEHOLD_INCOME" => PathQueryOptionsFilterFilter :: FilterHouseholdIncome , "FILTER_IMPRESSION_COUNTING_METHOD" => PathQueryOptionsFilterFilter :: FilterImpressionCountingMethod , "FILTER_IMPRESSION_LOSS_REJECTION_REASON" => PathQueryOptionsFilterFilter :: FilterImpressionLossRejectionReason , "FILTER_INSERTION_ORDER" => PathQueryOptionsFilterFilter :: FilterInsertionOrder , "FILTER_INSERTION_ORDER_GOAL_TYPE" => PathQueryOptionsFilterFilter :: FilterInsertionOrderGoalType , "FILTER_INSERTION_ORDER_GOAL_VALUE" => PathQueryOptionsFilterFilter :: FilterInsertionOrderGoalValue , "FILTER_INSERTION_ORDER_INTEGRATION_CODE" => PathQueryOptionsFilterFilter :: FilterInsertionOrderIntegrationCode , "FILTER_INSERTION_ORDER_NAME" => PathQueryOptionsFilterFilter :: FilterInsertionOrderName , "FILTER_INSERTION_ORDER_STATUS" => PathQueryOptionsFilterFilter :: FilterInsertionOrderStatus , "FILTER_INTEREST" => PathQueryOptionsFilterFilter :: FilterInterest , "FILTER_INVENTORY_COMMITMENT_TYPE" => PathQueryOptionsFilterFilter :: FilterInventoryCommitmentType , "FILTER_INVENTORY_DELIVERY_METHOD" => PathQueryOptionsFilterFilter :: FilterInventoryDeliveryMethod , "FILTER_INVENTORY_FORMAT" => PathQueryOptionsFilterFilter :: FilterInventoryFormat , "FILTER_INVENTORY_RATE_TYPE" => PathQueryOptionsFilterFilter :: FilterInventoryRateType , "FILTER_INVENTORY_SOURCE" => PathQueryOptionsFilterFilter :: FilterInventorySource , "FILTER_INVENTORY_SOURCE_EXTERNAL_ID" => PathQueryOptionsFilterFilter :: FilterInventorySourceExternalId , "FILTER_INVENTORY_SOURCE_GROUP" => PathQueryOptionsFilterFilter :: FilterInventorySourceGroup , "FILTER_INVENTORY_SOURCE_GROUP_ID" => PathQueryOptionsFilterFilter :: FilterInventorySourceGroupId , "FILTER_INVENTORY_SOURCE_ID" => PathQueryOptionsFilterFilter :: FilterInventorySourceId , "FILTER_INVENTORY_SOURCE_NAME" => PathQueryOptionsFilterFilter :: FilterInventorySourceName , "FILTER_INVENTORY_SOURCE_TYPE" => PathQueryOptionsFilterFilter :: FilterInventorySourceType , "FILTER_KEYWORD" => PathQueryOptionsFilterFilter :: FilterKeyword , "FILTER_LIFE_EVENT" => PathQueryOptionsFilterFilter :: FilterLifeEvent , "FILTER_LIFE_EVENTS" => PathQueryOptionsFilterFilter :: FilterLifeEvents , "FILTER_LINE_ITEM" => PathQueryOptionsFilterFilter :: FilterLineItem , "FILTER_LINE_ITEM_BUDGET" => PathQueryOptionsFilterFilter :: FilterLineItemBudget , "FILTER_LINE_ITEM_DAILY_FREQUENCY" => PathQueryOptionsFilterFilter :: FilterLineItemDailyFrequency , "FILTER_LINE_ITEM_END_DATE" => PathQueryOptionsFilterFilter :: FilterLineItemEndDate , "FILTER_LINE_ITEM_INTEGRATION_CODE" => PathQueryOptionsFilterFilter :: FilterLineItemIntegrationCode , "FILTER_LINE_ITEM_LIFETIME_FREQUENCY" => PathQueryOptionsFilterFilter :: FilterLineItemLifetimeFrequency , "FILTER_LINE_ITEM_NAME" => PathQueryOptionsFilterFilter :: FilterLineItemName , "FILTER_LINE_ITEM_PACING_PERCENTAGE" => PathQueryOptionsFilterFilter :: FilterLineItemPacingPercentage , "FILTER_LINE_ITEM_START_DATE" => PathQueryOptionsFilterFilter :: FilterLineItemStartDate , "FILTER_LINE_ITEM_STATUS" => PathQueryOptionsFilterFilter :: FilterLineItemStatus , "FILTER_LINE_ITEM_TYPE" => PathQueryOptionsFilterFilter :: FilterLineItemType , "FILTER_MATCH_RATIO" => PathQueryOptionsFilterFilter :: FilterMatchRatio , "FILTER_MATCHED_GENRE_TARGET" => PathQueryOptionsFilterFilter :: FilterMatchedGenreTarget , "FILTER_MEASUREMENT_SOURCE" => PathQueryOptionsFilterFilter :: FilterMeasurementSource , "FILTER_MEDIA_PLAN" => PathQueryOptionsFilterFilter :: FilterMediaPlan , "FILTER_MEDIA_PLAN_NAME" => PathQueryOptionsFilterFilter :: FilterMediaPlanName , "FILTER_MEDIA_TYPE" => PathQueryOptionsFilterFilter :: FilterMediaType , "FILTER_MOBILE_GEO" => PathQueryOptionsFilterFilter :: FilterMobileGeo , "FILTER_MONTH" => PathQueryOptionsFilterFilter :: FilterMonth , "FILTER_MRAID_SUPPORT" => PathQueryOptionsFilterFilter :: FilterMraidSupport , "FILTER_NIELSEN_AGE" => PathQueryOptionsFilterFilter :: FilterNielsenAge , "FILTER_NIELSEN_COUNTRY_CODE" => PathQueryOptionsFilterFilter :: FilterNielsenCountryCode , "FILTER_NIELSEN_DATE_RANGE" => PathQueryOptionsFilterFilter :: FilterNielsenDateRange , "FILTER_NIELSEN_DEVICE_ID" => PathQueryOptionsFilterFilter :: FilterNielsenDeviceId , "FILTER_NIELSEN_GENDER" => PathQueryOptionsFilterFilter :: FilterNielsenGender , "FILTER_NIELSEN_RESTATEMENT_DATE" => PathQueryOptionsFilterFilter :: FilterNielsenRestatementDate , "FILTER_NOT_SUPPORTED" => PathQueryOptionsFilterFilter :: FilterNotSupported , "FILTER_OM_SDK_AVAILABLE" => PathQueryOptionsFilterFilter :: FilterOmSdkAvailable , "FILTER_OMID_CAPABLE" => PathQueryOptionsFilterFilter :: FilterOmidCapable , "FILTER_ORDER_ID" => PathQueryOptionsFilterFilter :: FilterOrderId , "FILTER_OS" => PathQueryOptionsFilterFilter :: FilterOs , "FILTER_PAGE_CATEGORY" => PathQueryOptionsFilterFilter :: FilterPageCategory , "FILTER_PAGE_LAYOUT" => PathQueryOptionsFilterFilter :: FilterPageLayout , "FILTER_PARENTAL_STATUS" => PathQueryOptionsFilterFilter :: FilterParentalStatus , "FILTER_PARTNER" => PathQueryOptionsFilterFilter :: FilterPartner , "FILTER_PARTNER_CURRENCY" => PathQueryOptionsFilterFilter :: FilterPartnerCurrency , "FILTER_PARTNER_NAME" => PathQueryOptionsFilterFilter :: FilterPartnerName , "FILTER_PARTNER_STATUS" => PathQueryOptionsFilterFilter :: FilterPartnerStatus , "FILTER_PATH_EVENT_INDEX" => PathQueryOptionsFilterFilter :: FilterPathEventIndex , "FILTER_PATH_PATTERN_ID" => PathQueryOptionsFilterFilter :: FilterPathPatternId , "FILTER_PLACEMENT_ALL_YOUTUBE_CHANNELS" => PathQueryOptionsFilterFilter :: FilterPlacementAllYoutubeChannels , "FILTER_PLACEMENT_NAME_ALL_YOUTUBE_CHANNELS" => PathQueryOptionsFilterFilter :: FilterPlacementNameAllYoutubeChannels , "FILTER_PLATFORM" => PathQueryOptionsFilterFilter :: FilterPlatform , "FILTER_PLAYBACK_METHOD" => PathQueryOptionsFilterFilter :: FilterPlaybackMethod , "FILTER_POSITION_IN_CONTENT" => PathQueryOptionsFilterFilter :: FilterPositionInContent , "FILTER_PUBLIC_INVENTORY" => PathQueryOptionsFilterFilter :: FilterPublicInventory , "FILTER_PUBLISHER_PROPERTY" => PathQueryOptionsFilterFilter :: FilterPublisherProperty , "FILTER_PUBLISHER_PROPERTY_ID" => PathQueryOptionsFilterFilter :: FilterPublisherPropertyId , "FILTER_PUBLISHER_PROPERTY_SECTION" => PathQueryOptionsFilterFilter :: FilterPublisherPropertySection , "FILTER_PUBLISHER_PROPERTY_SECTION_ID" => PathQueryOptionsFilterFilter :: FilterPublisherPropertySectionId , "FILTER_QUARTER" => PathQueryOptionsFilterFilter :: FilterQuarter , "FILTER_REFUND_REASON" => PathQueryOptionsFilterFilter :: FilterRefundReason , "FILTER_REGION" => PathQueryOptionsFilterFilter :: FilterRegion , "FILTER_REGION_NAME" => PathQueryOptionsFilterFilter :: FilterRegionName , "FILTER_REMARKETING_LIST" => PathQueryOptionsFilterFilter :: FilterRemarketingList , "FILTER_REWARDED" => PathQueryOptionsFilterFilter :: FilterRewarded , "FILTER_SENSITIVE_CATEGORY" => PathQueryOptionsFilterFilter :: FilterSensitiveCategory , "FILTER_SERVED_PIXEL_DENSITY" => PathQueryOptionsFilterFilter :: FilterServedPixelDensity , "FILTER_SITE_ID" => PathQueryOptionsFilterFilter :: FilterSiteId , "FILTER_SITE_LANGUAGE" => PathQueryOptionsFilterFilter :: FilterSiteLanguage , "FILTER_SKIPPABLE_SUPPORT" => PathQueryOptionsFilterFilter :: FilterSkippableSupport , "FILTER_TARGETED_DATA_PROVIDERS" => PathQueryOptionsFilterFilter :: FilterTargetedDataProviders , "FILTER_TARGETED_USER_LIST" => PathQueryOptionsFilterFilter :: FilterTargetedUserList , "FILTER_THIRD_PARTY_AUDIENCE_LIST_COST" => PathQueryOptionsFilterFilter :: FilterThirdPartyAudienceListCost , "FILTER_THIRD_PARTY_AUDIENCE_LIST_TYPE" => PathQueryOptionsFilterFilter :: FilterThirdPartyAudienceListType , "FILTER_TIME_OF_DAY" => PathQueryOptionsFilterFilter :: FilterTimeOfDay , "FILTER_TRUEVIEW_AD" => PathQueryOptionsFilterFilter :: FilterTrueviewAd , "FILTER_TRUEVIEW_AD_GROUP" => PathQueryOptionsFilterFilter :: FilterTrueviewAdGroup , "FILTER_TRUEVIEW_AD_GROUP_AD_ID" => PathQueryOptionsFilterFilter :: FilterTrueviewAdGroupAdId , "FILTER_TRUEVIEW_AD_GROUP_ID" => PathQueryOptionsFilterFilter :: FilterTrueviewAdGroupId , "FILTER_TRUEVIEW_AD_TYPE_NAME" => PathQueryOptionsFilterFilter :: FilterTrueviewAdTypeName , "FILTER_TRUEVIEW_AGE" => PathQueryOptionsFilterFilter :: FilterTrueviewAge , "FILTER_TRUEVIEW_CATEGORY" => PathQueryOptionsFilterFilter :: FilterTrueviewCategory , "FILTER_TRUEVIEW_CITY" => PathQueryOptionsFilterFilter :: FilterTrueviewCity , "FILTER_TRUEVIEW_CLICK_TYPE_NAME" => PathQueryOptionsFilterFilter :: FilterTrueviewClickTypeName , "FILTER_TRUEVIEW_CONVERSION_TYPE" => PathQueryOptionsFilterFilter :: FilterTrueviewConversionType , "FILTER_TRUEVIEW_COUNTRY" => PathQueryOptionsFilterFilter :: FilterTrueviewCountry , "FILTER_TRUEVIEW_CUSTOM_AFFINITY" => PathQueryOptionsFilterFilter :: FilterTrueviewCustomAffinity , "FILTER_TRUEVIEW_DETAILED_DEMOGRAPHICS" => PathQueryOptionsFilterFilter :: FilterTrueviewDetailedDemographics , "FILTER_TRUEVIEW_DETAILED_DEMOGRAPHICS_ID" => PathQueryOptionsFilterFilter :: FilterTrueviewDetailedDemographicsId , "FILTER_TRUEVIEW_DMA" => PathQueryOptionsFilterFilter :: FilterTrueviewDma , "FILTER_TRUEVIEW_DMA_NAME" => PathQueryOptionsFilterFilter :: FilterTrueviewDmaName , "FILTER_TRUEVIEW_GENDER" => PathQueryOptionsFilterFilter :: FilterTrueviewGender , "FILTER_TRUEVIEW_HOUSEHOLD_INCOME" => PathQueryOptionsFilterFilter :: FilterTrueviewHouseholdIncome , "FILTER_TRUEVIEW_IAR_AGE" => PathQueryOptionsFilterFilter :: FilterTrueviewIarAge , "FILTER_TRUEVIEW_IAR_CATEGORY" => PathQueryOptionsFilterFilter :: FilterTrueviewIarCategory , "FILTER_TRUEVIEW_IAR_CITY" => PathQueryOptionsFilterFilter :: FilterTrueviewIarCity , "FILTER_TRUEVIEW_IAR_COUNTRY" => PathQueryOptionsFilterFilter :: FilterTrueviewIarCountry , "FILTER_TRUEVIEW_IAR_COUNTRY_NAME" => PathQueryOptionsFilterFilter :: FilterTrueviewIarCountryName , "FILTER_TRUEVIEW_IAR_GENDER" => PathQueryOptionsFilterFilter :: FilterTrueviewIarGender , "FILTER_TRUEVIEW_IAR_INTEREST" => PathQueryOptionsFilterFilter :: FilterTrueviewIarInterest , "FILTER_TRUEVIEW_IAR_LANGUAGE" => PathQueryOptionsFilterFilter :: FilterTrueviewIarLanguage , "FILTER_TRUEVIEW_IAR_PARENTAL_STATUS" => PathQueryOptionsFilterFilter :: FilterTrueviewIarParentalStatus , "FILTER_TRUEVIEW_IAR_REGION" => PathQueryOptionsFilterFilter :: FilterTrueviewIarRegion , "FILTER_TRUEVIEW_IAR_REGION_NAME" => PathQueryOptionsFilterFilter :: FilterTrueviewIarRegionName , "FILTER_TRUEVIEW_IAR_REMARKETING_LIST" => PathQueryOptionsFilterFilter :: FilterTrueviewIarRemarketingList , "FILTER_TRUEVIEW_IAR_TIME_OF_DAY" => PathQueryOptionsFilterFilter :: FilterTrueviewIarTimeOfDay , "FILTER_TRUEVIEW_IAR_YOUTUBE_CHANNEL" => PathQueryOptionsFilterFilter :: FilterTrueviewIarYoutubeChannel , "FILTER_TRUEVIEW_IAR_YOUTUBE_VIDEO" => PathQueryOptionsFilterFilter :: FilterTrueviewIarYoutubeVideo , "FILTER_TRUEVIEW_IAR_ZIPCODE" => PathQueryOptionsFilterFilter :: FilterTrueviewIarZipcode , "FILTER_TRUEVIEW_INTEREST" => PathQueryOptionsFilterFilter :: FilterTrueviewInterest , "FILTER_TRUEVIEW_KEYWORD" => PathQueryOptionsFilterFilter :: FilterTrueviewKeyword , "FILTER_TRUEVIEW_PARENTAL_STATUS" => PathQueryOptionsFilterFilter :: FilterTrueviewParentalStatus , "FILTER_TRUEVIEW_PLACEMENT" => PathQueryOptionsFilterFilter :: FilterTrueviewPlacement , "FILTER_TRUEVIEW_PLACEMENT_ID" => PathQueryOptionsFilterFilter :: FilterTrueviewPlacementId , "FILTER_TRUEVIEW_REGION" => PathQueryOptionsFilterFilter :: FilterTrueviewRegion , "FILTER_TRUEVIEW_REGION_NAME" => PathQueryOptionsFilterFilter :: FilterTrueviewRegionName , "FILTER_TRUEVIEW_REMARKETING_LIST" => PathQueryOptionsFilterFilter :: FilterTrueviewRemarketingList , "FILTER_TRUEVIEW_REMARKETING_LIST_NAME" => PathQueryOptionsFilterFilter :: FilterTrueviewRemarketingListName , "FILTER_TRUEVIEW_URL" => PathQueryOptionsFilterFilter :: FilterTrueviewUrl , "FILTER_TRUEVIEW_ZIPCODE" => PathQueryOptionsFilterFilter :: FilterTrueviewZipcode , "FILTER_UNKNOWN" => PathQueryOptionsFilterFilter :: FilterUnknown , "FILTER_USER_LIST" => PathQueryOptionsFilterFilter :: FilterUserList , "FILTER_USER_LIST_FIRST_PARTY" => PathQueryOptionsFilterFilter :: FilterUserListFirstParty , "FILTER_USER_LIST_FIRST_PARTY_NAME" => PathQueryOptionsFilterFilter :: FilterUserListFirstPartyName , "FILTER_USER_LIST_THIRD_PARTY" => PathQueryOptionsFilterFilter :: FilterUserListThirdParty , "FILTER_USER_LIST_THIRD_PARTY_NAME" => PathQueryOptionsFilterFilter :: FilterUserListThirdPartyName , "FILTER_VARIANT_ID" => PathQueryOptionsFilterFilter :: FilterVariantId , "FILTER_VARIANT_NAME" => PathQueryOptionsFilterFilter :: FilterVariantName , "FILTER_VARIANT_VERSION" => PathQueryOptionsFilterFilter :: FilterVariantVersion , "FILTER_VENDOR_MEASUREMENT_MODE" => PathQueryOptionsFilterFilter :: FilterVendorMeasurementMode , "FILTER_VERIFICATION_AUDIBILITY_COMPLETE" => PathQueryOptionsFilterFilter :: FilterVerificationAudibilityComplete , "FILTER_VERIFICATION_AUDIBILITY_START" => PathQueryOptionsFilterFilter :: FilterVerificationAudibilityStart , "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE" => PathQueryOptionsFilterFilter :: FilterVerificationVideoPlayerSize , "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_COMPLETE" => PathQueryOptionsFilterFilter :: FilterVerificationVideoPlayerSizeComplete , "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_FIRST_QUARTILE" => PathQueryOptionsFilterFilter :: FilterVerificationVideoPlayerSizeFirstQuartile , "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_MID_POINT" => PathQueryOptionsFilterFilter :: FilterVerificationVideoPlayerSizeMidPoint , "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_START" => PathQueryOptionsFilterFilter :: FilterVerificationVideoPlayerSizeStart , "FILTER_VERIFICATION_VIDEO_PLAYER_SIZE_THIRD_QUARTILE" => PathQueryOptionsFilterFilter :: FilterVerificationVideoPlayerSizeThirdQuartile , "FILTER_VERIFICATION_VIDEO_POSITION" => PathQueryOptionsFilterFilter :: FilterVerificationVideoPosition , "FILTER_VERIFICATION_VIDEO_RESIZED" => PathQueryOptionsFilterFilter :: FilterVerificationVideoResized , "FILTER_VIDEO_AD_POSITION_IN_STREAM" => PathQueryOptionsFilterFilter :: FilterVideoAdPositionInStream , "FILTER_VIDEO_COMPANION_CREATIVE_SIZE" => PathQueryOptionsFilterFilter :: FilterVideoCompanionCreativeSize , "FILTER_VIDEO_CONTENT_DURATION" => PathQueryOptionsFilterFilter :: FilterVideoContentDuration , "FILTER_VIDEO_CONTENT_LIVE_STREAM" => PathQueryOptionsFilterFilter :: FilterVideoContentLiveStream , "FILTER_VIDEO_CONTINUOUS_PLAY" => PathQueryOptionsFilterFilter :: FilterVideoContinuousPlay , "FILTER_VIDEO_CREATIVE_DURATION" => PathQueryOptionsFilterFilter :: FilterVideoCreativeDuration , "FILTER_VIDEO_CREATIVE_DURATION_SKIPPABLE" => PathQueryOptionsFilterFilter :: FilterVideoCreativeDurationSkippable , "FILTER_VIDEO_DURATION" => PathQueryOptionsFilterFilter :: FilterVideoDuration , "FILTER_VIDEO_DURATION_SECONDS" => PathQueryOptionsFilterFilter :: FilterVideoDurationSeconds , "FILTER_VIDEO_DURATION_SECONDS_RANGE" => PathQueryOptionsFilterFilter :: FilterVideoDurationSecondsRange , "FILTER_VIDEO_FORMAT_SUPPORT" => PathQueryOptionsFilterFilter :: FilterVideoFormatSupport , "FILTER_VIDEO_PLAYER_SIZE" => PathQueryOptionsFilterFilter :: FilterVideoPlayerSize , "FILTER_VIDEO_RATING_TIER" => PathQueryOptionsFilterFilter :: FilterVideoRatingTier , "FILTER_VIDEO_SKIPPABLE_SUPPORT" => PathQueryOptionsFilterFilter :: FilterVideoSkippableSupport , "FILTER_WEEK" => PathQueryOptionsFilterFilter :: FilterWeek , "FILTER_YEAR" => PathQueryOptionsFilterFilter :: FilterYear , "FILTER_YOUTUBE_AD_VIDEO" => PathQueryOptionsFilterFilter :: FilterYoutubeAdVideo , "FILTER_YOUTUBE_AD_VIDEO_ID" => PathQueryOptionsFilterFilter :: FilterYoutubeAdVideoId , "FILTER_YOUTUBE_ADAPTED_AUDIENCE_LIST" => PathQueryOptionsFilterFilter :: FilterYoutubeAdaptedAudienceList , "FILTER_YOUTUBE_CHANNEL" => PathQueryOptionsFilterFilter :: FilterYoutubeChannel , "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_ADVERTISER" => PathQueryOptionsFilterFilter :: FilterYoutubeProgrammaticGuaranteedAdvertiser , "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_INSERTION_ORDER" => PathQueryOptionsFilterFilter :: FilterYoutubeProgrammaticGuaranteedInsertionOrder , "FILTER_YOUTUBE_PROGRAMMATIC_GUARANTEED_PARTNER" => PathQueryOptionsFilterFilter :: FilterYoutubeProgrammaticGuaranteedPartner , "FILTER_YOUTUBE_VIDEO" => PathQueryOptionsFilterFilter :: FilterYoutubeVideo , "FILTER_ZIP_CODE" => PathQueryOptionsFilterFilter :: FilterZipCode , "FILTER_ZIP_POSTAL_CODE" => PathQueryOptionsFilterFilter :: FilterZipPostalCode , _ => return Err (:: serde :: de :: Error :: custom (format ! ("invalid enum for #name: {}" , value))) , })
        }
    }
    impl ::google_field_selector::FieldSelector for PathQueryOptionsFilterFilter {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PathQueryOptionsFilterFilter {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PathQueryOptionsFilterMatch {
        BeginsWith,
        Exact,
        Partial,
        Unknown,
        WildcardExpression,
    }
    impl PathQueryOptionsFilterMatch {
        pub fn as_str(self) -> &'static str {
            match self {
                PathQueryOptionsFilterMatch::BeginsWith => "BEGINS_WITH",
                PathQueryOptionsFilterMatch::Exact => "EXACT",
                PathQueryOptionsFilterMatch::Partial => "PARTIAL",
                PathQueryOptionsFilterMatch::Unknown => "UNKNOWN",
                PathQueryOptionsFilterMatch::WildcardExpression => "WILDCARD_EXPRESSION",
            }
        }
    }
    impl ::std::convert::AsRef<str> for PathQueryOptionsFilterMatch {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for PathQueryOptionsFilterMatch {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<PathQueryOptionsFilterMatch, ()> {
            Ok(match s {
                "BEGINS_WITH" => PathQueryOptionsFilterMatch::BeginsWith,
                "EXACT" => PathQueryOptionsFilterMatch::Exact,
                "PARTIAL" => PathQueryOptionsFilterMatch::Partial,
                "UNKNOWN" => PathQueryOptionsFilterMatch::Unknown,
                "WILDCARD_EXPRESSION" => PathQueryOptionsFilterMatch::WildcardExpression,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for PathQueryOptionsFilterMatch {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PathQueryOptionsFilterMatch {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PathQueryOptionsFilterMatch {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BEGINS_WITH" => PathQueryOptionsFilterMatch::BeginsWith,
                "EXACT" => PathQueryOptionsFilterMatch::Exact,
                "PARTIAL" => PathQueryOptionsFilterMatch::Partial,
                "UNKNOWN" => PathQueryOptionsFilterMatch::Unknown,
                "WILDCARD_EXPRESSION" => PathQueryOptionsFilterMatch::WildcardExpression,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for PathQueryOptionsFilterMatch {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PathQueryOptionsFilterMatch {
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
    pub struct Query {
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"doubleclickbidmanager#query\"."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Query metadata."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::QueryMetadata>,
        #[doc = "Query parameters."]
        #[serde(
            rename = "params",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub params: ::std::option::Option<crate::schemas::Parameters>,
        #[doc = "Query ID."]
        #[serde(
            rename = "queryId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub query_id: ::std::option::Option<i64>,
        #[doc = "The ending time for the data that is shown in the report. Note, reportDataEndTimeMs is required if metadata.dataRange is CUSTOM_DATES and ignored otherwise."]
        #[serde(
            rename = "reportDataEndTimeMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub report_data_end_time_ms: ::std::option::Option<i64>,
        #[doc = "The starting time for the data that is shown in the report. Note, reportDataStartTimeMs is required if metadata.dataRange is CUSTOM_DATES and ignored otherwise."]
        #[serde(
            rename = "reportDataStartTimeMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub report_data_start_time_ms: ::std::option::Option<i64>,
        #[doc = "Information on how often and when to run a query."]
        #[serde(
            rename = "schedule",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub schedule: ::std::option::Option<crate::schemas::QuerySchedule>,
        #[doc = "Canonical timezone code for report data time. Defaults to America/New_York."]
        #[serde(
            rename = "timezoneCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timezone_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Query {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Query {
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
    pub struct QueryMetadata {
        #[doc = "Range of report data."]
        #[serde(
            rename = "dataRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_range: ::std::option::Option<crate::schemas::QueryMetadataDataRange>,
        #[doc = "Format of the generated report."]
        #[serde(
            rename = "format",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub format: ::std::option::Option<crate::schemas::QueryMetadataFormat>,
        #[doc = "The path to the location in Google Cloud Storage where the latest report is stored."]
        #[serde(
            rename = "googleCloudStoragePathForLatestReport",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_cloud_storage_path_for_latest_report: ::std::option::Option<String>,
        #[doc = "The path in Google Drive for the latest report."]
        #[serde(
            rename = "googleDrivePathForLatestReport",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_drive_path_for_latest_report: ::std::option::Option<String>,
        #[doc = "The time when the latest report started to run."]
        #[serde(
            rename = "latestReportRunTimeMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub latest_report_run_time_ms: ::std::option::Option<i64>,
        #[doc = "Locale of the generated reports. Valid values are cs CZECH de GERMAN en ENGLISH es SPANISH fr FRENCH it ITALIAN ja JAPANESE ko KOREAN pl POLISH pt-BR BRAZILIAN_PORTUGUESE ru RUSSIAN tr TURKISH uk UKRAINIAN zh-CN CHINA_CHINESE zh-TW TAIWAN_CHINESE An locale string not in the list above will generate reports in English."]
        #[serde(
            rename = "locale",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub locale: ::std::option::Option<String>,
        #[doc = "Number of reports that have been generated for the query."]
        #[serde(
            rename = "reportCount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub report_count: ::std::option::Option<i32>,
        #[doc = "Whether the latest report is currently running."]
        #[serde(
            rename = "running",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub running: ::std::option::Option<bool>,
        #[doc = "Whether to send an email notification when a report is ready. Default to false."]
        #[serde(
            rename = "sendNotification",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub send_notification: ::std::option::Option<bool>,
        #[doc = "List of email addresses which are sent email notifications when the report is finished. Separate from sendNotification."]
        #[serde(
            rename = "shareEmailAddress",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub share_email_address: ::std::option::Option<Vec<String>>,
        #[doc = "Query title. It is used to name the reports generated from this query."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for QueryMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryMetadata {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum QueryMetadataDataRange {
        AllTime,
        CurrentDay,
        CustomDates,
        Last14Days,
        Last30Days,
        Last365Days,
        Last60Days,
        Last7Days,
        Last90Days,
        MonthToDate,
        PreviousDay,
        PreviousHalfMonth,
        PreviousMonth,
        PreviousQuarter,
        PreviousWeek,
        PreviousYear,
        QuarterToDate,
        TypeNotSupported,
        WeekToDate,
        YearToDate,
    }
    impl QueryMetadataDataRange {
        pub fn as_str(self) -> &'static str {
            match self {
                QueryMetadataDataRange::AllTime => "ALL_TIME",
                QueryMetadataDataRange::CurrentDay => "CURRENT_DAY",
                QueryMetadataDataRange::CustomDates => "CUSTOM_DATES",
                QueryMetadataDataRange::Last14Days => "LAST_14_DAYS",
                QueryMetadataDataRange::Last30Days => "LAST_30_DAYS",
                QueryMetadataDataRange::Last365Days => "LAST_365_DAYS",
                QueryMetadataDataRange::Last60Days => "LAST_60_DAYS",
                QueryMetadataDataRange::Last7Days => "LAST_7_DAYS",
                QueryMetadataDataRange::Last90Days => "LAST_90_DAYS",
                QueryMetadataDataRange::MonthToDate => "MONTH_TO_DATE",
                QueryMetadataDataRange::PreviousDay => "PREVIOUS_DAY",
                QueryMetadataDataRange::PreviousHalfMonth => "PREVIOUS_HALF_MONTH",
                QueryMetadataDataRange::PreviousMonth => "PREVIOUS_MONTH",
                QueryMetadataDataRange::PreviousQuarter => "PREVIOUS_QUARTER",
                QueryMetadataDataRange::PreviousWeek => "PREVIOUS_WEEK",
                QueryMetadataDataRange::PreviousYear => "PREVIOUS_YEAR",
                QueryMetadataDataRange::QuarterToDate => "QUARTER_TO_DATE",
                QueryMetadataDataRange::TypeNotSupported => "TYPE_NOT_SUPPORTED",
                QueryMetadataDataRange::WeekToDate => "WEEK_TO_DATE",
                QueryMetadataDataRange::YearToDate => "YEAR_TO_DATE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for QueryMetadataDataRange {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for QueryMetadataDataRange {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<QueryMetadataDataRange, ()> {
            Ok(match s {
                "ALL_TIME" => QueryMetadataDataRange::AllTime,
                "CURRENT_DAY" => QueryMetadataDataRange::CurrentDay,
                "CUSTOM_DATES" => QueryMetadataDataRange::CustomDates,
                "LAST_14_DAYS" => QueryMetadataDataRange::Last14Days,
                "LAST_30_DAYS" => QueryMetadataDataRange::Last30Days,
                "LAST_365_DAYS" => QueryMetadataDataRange::Last365Days,
                "LAST_60_DAYS" => QueryMetadataDataRange::Last60Days,
                "LAST_7_DAYS" => QueryMetadataDataRange::Last7Days,
                "LAST_90_DAYS" => QueryMetadataDataRange::Last90Days,
                "MONTH_TO_DATE" => QueryMetadataDataRange::MonthToDate,
                "PREVIOUS_DAY" => QueryMetadataDataRange::PreviousDay,
                "PREVIOUS_HALF_MONTH" => QueryMetadataDataRange::PreviousHalfMonth,
                "PREVIOUS_MONTH" => QueryMetadataDataRange::PreviousMonth,
                "PREVIOUS_QUARTER" => QueryMetadataDataRange::PreviousQuarter,
                "PREVIOUS_WEEK" => QueryMetadataDataRange::PreviousWeek,
                "PREVIOUS_YEAR" => QueryMetadataDataRange::PreviousYear,
                "QUARTER_TO_DATE" => QueryMetadataDataRange::QuarterToDate,
                "TYPE_NOT_SUPPORTED" => QueryMetadataDataRange::TypeNotSupported,
                "WEEK_TO_DATE" => QueryMetadataDataRange::WeekToDate,
                "YEAR_TO_DATE" => QueryMetadataDataRange::YearToDate,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for QueryMetadataDataRange {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for QueryMetadataDataRange {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for QueryMetadataDataRange {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL_TIME" => QueryMetadataDataRange::AllTime,
                "CURRENT_DAY" => QueryMetadataDataRange::CurrentDay,
                "CUSTOM_DATES" => QueryMetadataDataRange::CustomDates,
                "LAST_14_DAYS" => QueryMetadataDataRange::Last14Days,
                "LAST_30_DAYS" => QueryMetadataDataRange::Last30Days,
                "LAST_365_DAYS" => QueryMetadataDataRange::Last365Days,
                "LAST_60_DAYS" => QueryMetadataDataRange::Last60Days,
                "LAST_7_DAYS" => QueryMetadataDataRange::Last7Days,
                "LAST_90_DAYS" => QueryMetadataDataRange::Last90Days,
                "MONTH_TO_DATE" => QueryMetadataDataRange::MonthToDate,
                "PREVIOUS_DAY" => QueryMetadataDataRange::PreviousDay,
                "PREVIOUS_HALF_MONTH" => QueryMetadataDataRange::PreviousHalfMonth,
                "PREVIOUS_MONTH" => QueryMetadataDataRange::PreviousMonth,
                "PREVIOUS_QUARTER" => QueryMetadataDataRange::PreviousQuarter,
                "PREVIOUS_WEEK" => QueryMetadataDataRange::PreviousWeek,
                "PREVIOUS_YEAR" => QueryMetadataDataRange::PreviousYear,
                "QUARTER_TO_DATE" => QueryMetadataDataRange::QuarterToDate,
                "TYPE_NOT_SUPPORTED" => QueryMetadataDataRange::TypeNotSupported,
                "WEEK_TO_DATE" => QueryMetadataDataRange::WeekToDate,
                "YEAR_TO_DATE" => QueryMetadataDataRange::YearToDate,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for QueryMetadataDataRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryMetadataDataRange {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum QueryMetadataFormat {
        Csv,
        ExcelCsv,
        Xlsx,
    }
    impl QueryMetadataFormat {
        pub fn as_str(self) -> &'static str {
            match self {
                QueryMetadataFormat::Csv => "CSV",
                QueryMetadataFormat::ExcelCsv => "EXCEL_CSV",
                QueryMetadataFormat::Xlsx => "XLSX",
            }
        }
    }
    impl ::std::convert::AsRef<str> for QueryMetadataFormat {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for QueryMetadataFormat {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<QueryMetadataFormat, ()> {
            Ok(match s {
                "CSV" => QueryMetadataFormat::Csv,
                "EXCEL_CSV" => QueryMetadataFormat::ExcelCsv,
                "XLSX" => QueryMetadataFormat::Xlsx,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for QueryMetadataFormat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for QueryMetadataFormat {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for QueryMetadataFormat {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CSV" => QueryMetadataFormat::Csv,
                "EXCEL_CSV" => QueryMetadataFormat::ExcelCsv,
                "XLSX" => QueryMetadataFormat::Xlsx,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for QueryMetadataFormat {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryMetadataFormat {
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
    pub struct QuerySchedule {
        #[doc = "Datetime to periodically run the query until."]
        #[serde(
            rename = "endTimeMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub end_time_ms: ::std::option::Option<i64>,
        #[doc = "How often the query is run."]
        #[serde(
            rename = "frequency",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub frequency: ::std::option::Option<crate::schemas::QueryScheduleFrequency>,
        #[doc = "Time of day at which a new report will be generated, represented as minutes past midnight. Range is 0 to 1439. Only applies to scheduled reports."]
        #[serde(
            rename = "nextRunMinuteOfDay",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_run_minute_of_day: ::std::option::Option<i32>,
        #[doc = "Canonical timezone code for report generation time. Defaults to America/New_York."]
        #[serde(
            rename = "nextRunTimezoneCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_run_timezone_code: ::std::option::Option<String>,
        #[doc = "When to start running the query. Not applicable to `ONE_TIME` frequency."]
        #[serde(
            rename = "startTimeMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub start_time_ms: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for QuerySchedule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QuerySchedule {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum QueryScheduleFrequency {
        Daily,
        Monthly,
        OneTime,
        Quarterly,
        SemiMonthly,
        Weekly,
        Yearly,
    }
    impl QueryScheduleFrequency {
        pub fn as_str(self) -> &'static str {
            match self {
                QueryScheduleFrequency::Daily => "DAILY",
                QueryScheduleFrequency::Monthly => "MONTHLY",
                QueryScheduleFrequency::OneTime => "ONE_TIME",
                QueryScheduleFrequency::Quarterly => "QUARTERLY",
                QueryScheduleFrequency::SemiMonthly => "SEMI_MONTHLY",
                QueryScheduleFrequency::Weekly => "WEEKLY",
                QueryScheduleFrequency::Yearly => "YEARLY",
            }
        }
    }
    impl ::std::convert::AsRef<str> for QueryScheduleFrequency {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for QueryScheduleFrequency {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<QueryScheduleFrequency, ()> {
            Ok(match s {
                "DAILY" => QueryScheduleFrequency::Daily,
                "MONTHLY" => QueryScheduleFrequency::Monthly,
                "ONE_TIME" => QueryScheduleFrequency::OneTime,
                "QUARTERLY" => QueryScheduleFrequency::Quarterly,
                "SEMI_MONTHLY" => QueryScheduleFrequency::SemiMonthly,
                "WEEKLY" => QueryScheduleFrequency::Weekly,
                "YEARLY" => QueryScheduleFrequency::Yearly,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for QueryScheduleFrequency {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for QueryScheduleFrequency {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for QueryScheduleFrequency {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DAILY" => QueryScheduleFrequency::Daily,
                "MONTHLY" => QueryScheduleFrequency::Monthly,
                "ONE_TIME" => QueryScheduleFrequency::OneTime,
                "QUARTERLY" => QueryScheduleFrequency::Quarterly,
                "SEMI_MONTHLY" => QueryScheduleFrequency::SemiMonthly,
                "WEEKLY" => QueryScheduleFrequency::Weekly,
                "YEARLY" => QueryScheduleFrequency::Yearly,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for QueryScheduleFrequency {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for QueryScheduleFrequency {
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
    pub struct Report {
        #[doc = "Key used to identify a report."]
        #[serde(
            rename = "key",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub key: ::std::option::Option<crate::schemas::ReportKey>,
        #[doc = "Report metadata."]
        #[serde(
            rename = "metadata",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub metadata: ::std::option::Option<crate::schemas::ReportMetadata>,
        #[doc = "Report parameters."]
        #[serde(
            rename = "params",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub params: ::std::option::Option<crate::schemas::Parameters>,
    }
    impl ::google_field_selector::FieldSelector for Report {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Report {
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
    pub struct ReportFailure {
        #[doc = "Error code that shows why the report was not created."]
        #[serde(
            rename = "errorCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub error_code: ::std::option::Option<crate::schemas::ReportFailureErrorCode>,
    }
    impl ::google_field_selector::FieldSelector for ReportFailure {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportFailure {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ReportFailureErrorCode {
        AuthenticationError,
        DeprecatedReportingInvalidQuery,
        ReportingBucketNotFound,
        ReportingCreateBucketFailed,
        ReportingDeleteBucketFailed,
        ReportingFatalError,
        ReportingIllegalFilename,
        ReportingImcompatibleMetrics,
        ReportingInvalidQueryMissingPartnerAndAdvertiserFilters,
        ReportingInvalidQueryTitleMissing,
        ReportingInvalidQueryTooManyUnfilteredLargeGroupBys,
        ReportingQueryNotFound,
        ReportingTransientError,
        ReportingUpdateBucketPermissionFailed,
        ReportingWriteBucketObjectFailed,
        ServerError,
        UnauthorizedApiAccess,
        ValidationError,
    }
    impl ReportFailureErrorCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ReportFailureErrorCode::AuthenticationError => "AUTHENTICATION_ERROR",
                ReportFailureErrorCode::DeprecatedReportingInvalidQuery => {
                    "DEPRECATED_REPORTING_INVALID_QUERY"
                }
                ReportFailureErrorCode::ReportingBucketNotFound => "REPORTING_BUCKET_NOT_FOUND",
                ReportFailureErrorCode::ReportingCreateBucketFailed => {
                    "REPORTING_CREATE_BUCKET_FAILED"
                }
                ReportFailureErrorCode::ReportingDeleteBucketFailed => {
                    "REPORTING_DELETE_BUCKET_FAILED"
                }
                ReportFailureErrorCode::ReportingFatalError => "REPORTING_FATAL_ERROR",
                ReportFailureErrorCode::ReportingIllegalFilename => "REPORTING_ILLEGAL_FILENAME",
                ReportFailureErrorCode::ReportingImcompatibleMetrics => {
                    "REPORTING_IMCOMPATIBLE_METRICS"
                }
                ReportFailureErrorCode::ReportingInvalidQueryMissingPartnerAndAdvertiserFilters => {
                    "REPORTING_INVALID_QUERY_MISSING_PARTNER_AND_ADVERTISER_FILTERS"
                }
                ReportFailureErrorCode::ReportingInvalidQueryTitleMissing => {
                    "REPORTING_INVALID_QUERY_TITLE_MISSING"
                }
                ReportFailureErrorCode::ReportingInvalidQueryTooManyUnfilteredLargeGroupBys => {
                    "REPORTING_INVALID_QUERY_TOO_MANY_UNFILTERED_LARGE_GROUP_BYS"
                }
                ReportFailureErrorCode::ReportingQueryNotFound => "REPORTING_QUERY_NOT_FOUND",
                ReportFailureErrorCode::ReportingTransientError => "REPORTING_TRANSIENT_ERROR",
                ReportFailureErrorCode::ReportingUpdateBucketPermissionFailed => {
                    "REPORTING_UPDATE_BUCKET_PERMISSION_FAILED"
                }
                ReportFailureErrorCode::ReportingWriteBucketObjectFailed => {
                    "REPORTING_WRITE_BUCKET_OBJECT_FAILED"
                }
                ReportFailureErrorCode::ServerError => "SERVER_ERROR",
                ReportFailureErrorCode::UnauthorizedApiAccess => "UNAUTHORIZED_API_ACCESS",
                ReportFailureErrorCode::ValidationError => "VALIDATION_ERROR",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ReportFailureErrorCode {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ReportFailureErrorCode {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ReportFailureErrorCode, ()> {
            Ok(match s {
                "AUTHENTICATION_ERROR" => ReportFailureErrorCode::AuthenticationError,
                "DEPRECATED_REPORTING_INVALID_QUERY" => {
                    ReportFailureErrorCode::DeprecatedReportingInvalidQuery
                }
                "REPORTING_BUCKET_NOT_FOUND" => ReportFailureErrorCode::ReportingBucketNotFound,
                "REPORTING_CREATE_BUCKET_FAILED" => {
                    ReportFailureErrorCode::ReportingCreateBucketFailed
                }
                "REPORTING_DELETE_BUCKET_FAILED" => {
                    ReportFailureErrorCode::ReportingDeleteBucketFailed
                }
                "REPORTING_FATAL_ERROR" => ReportFailureErrorCode::ReportingFatalError,
                "REPORTING_ILLEGAL_FILENAME" => ReportFailureErrorCode::ReportingIllegalFilename,
                "REPORTING_IMCOMPATIBLE_METRICS" => {
                    ReportFailureErrorCode::ReportingImcompatibleMetrics
                }
                "REPORTING_INVALID_QUERY_MISSING_PARTNER_AND_ADVERTISER_FILTERS" => {
                    ReportFailureErrorCode::ReportingInvalidQueryMissingPartnerAndAdvertiserFilters
                }
                "REPORTING_INVALID_QUERY_TITLE_MISSING" => {
                    ReportFailureErrorCode::ReportingInvalidQueryTitleMissing
                }
                "REPORTING_INVALID_QUERY_TOO_MANY_UNFILTERED_LARGE_GROUP_BYS" => {
                    ReportFailureErrorCode::ReportingInvalidQueryTooManyUnfilteredLargeGroupBys
                }
                "REPORTING_QUERY_NOT_FOUND" => ReportFailureErrorCode::ReportingQueryNotFound,
                "REPORTING_TRANSIENT_ERROR" => ReportFailureErrorCode::ReportingTransientError,
                "REPORTING_UPDATE_BUCKET_PERMISSION_FAILED" => {
                    ReportFailureErrorCode::ReportingUpdateBucketPermissionFailed
                }
                "REPORTING_WRITE_BUCKET_OBJECT_FAILED" => {
                    ReportFailureErrorCode::ReportingWriteBucketObjectFailed
                }
                "SERVER_ERROR" => ReportFailureErrorCode::ServerError,
                "UNAUTHORIZED_API_ACCESS" => ReportFailureErrorCode::UnauthorizedApiAccess,
                "VALIDATION_ERROR" => ReportFailureErrorCode::ValidationError,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ReportFailureErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ReportFailureErrorCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReportFailureErrorCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AUTHENTICATION_ERROR" => ReportFailureErrorCode::AuthenticationError,
                "DEPRECATED_REPORTING_INVALID_QUERY" => {
                    ReportFailureErrorCode::DeprecatedReportingInvalidQuery
                }
                "REPORTING_BUCKET_NOT_FOUND" => ReportFailureErrorCode::ReportingBucketNotFound,
                "REPORTING_CREATE_BUCKET_FAILED" => {
                    ReportFailureErrorCode::ReportingCreateBucketFailed
                }
                "REPORTING_DELETE_BUCKET_FAILED" => {
                    ReportFailureErrorCode::ReportingDeleteBucketFailed
                }
                "REPORTING_FATAL_ERROR" => ReportFailureErrorCode::ReportingFatalError,
                "REPORTING_ILLEGAL_FILENAME" => ReportFailureErrorCode::ReportingIllegalFilename,
                "REPORTING_IMCOMPATIBLE_METRICS" => {
                    ReportFailureErrorCode::ReportingImcompatibleMetrics
                }
                "REPORTING_INVALID_QUERY_MISSING_PARTNER_AND_ADVERTISER_FILTERS" => {
                    ReportFailureErrorCode::ReportingInvalidQueryMissingPartnerAndAdvertiserFilters
                }
                "REPORTING_INVALID_QUERY_TITLE_MISSING" => {
                    ReportFailureErrorCode::ReportingInvalidQueryTitleMissing
                }
                "REPORTING_INVALID_QUERY_TOO_MANY_UNFILTERED_LARGE_GROUP_BYS" => {
                    ReportFailureErrorCode::ReportingInvalidQueryTooManyUnfilteredLargeGroupBys
                }
                "REPORTING_QUERY_NOT_FOUND" => ReportFailureErrorCode::ReportingQueryNotFound,
                "REPORTING_TRANSIENT_ERROR" => ReportFailureErrorCode::ReportingTransientError,
                "REPORTING_UPDATE_BUCKET_PERMISSION_FAILED" => {
                    ReportFailureErrorCode::ReportingUpdateBucketPermissionFailed
                }
                "REPORTING_WRITE_BUCKET_OBJECT_FAILED" => {
                    ReportFailureErrorCode::ReportingWriteBucketObjectFailed
                }
                "SERVER_ERROR" => ReportFailureErrorCode::ServerError,
                "UNAUTHORIZED_API_ACCESS" => ReportFailureErrorCode::UnauthorizedApiAccess,
                "VALIDATION_ERROR" => ReportFailureErrorCode::ValidationError,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ReportFailureErrorCode {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportFailureErrorCode {
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
    pub struct ReportKey {
        #[doc = "Query ID."]
        #[serde(
            rename = "queryId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub query_id: ::std::option::Option<i64>,
        #[doc = "Report ID."]
        #[serde(
            rename = "reportId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub report_id: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for ReportKey {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportKey {
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
    pub struct ReportMetadata {
        #[doc = "The path to the location in Google Cloud Storage where the report is stored."]
        #[serde(
            rename = "googleCloudStoragePath",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_cloud_storage_path: ::std::option::Option<String>,
        #[doc = "The ending time for the data that is shown in the report."]
        #[serde(
            rename = "reportDataEndTimeMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub report_data_end_time_ms: ::std::option::Option<i64>,
        #[doc = "The starting time for the data that is shown in the report."]
        #[serde(
            rename = "reportDataStartTimeMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub report_data_start_time_ms: ::std::option::Option<i64>,
        #[doc = "Report status."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::ReportStatus>,
    }
    impl ::google_field_selector::FieldSelector for ReportMetadata {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportMetadata {
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
    pub struct ReportStatus {
        #[doc = "If the report failed, this records the cause."]
        #[serde(
            rename = "failure",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub failure: ::std::option::Option<crate::schemas::ReportFailure>,
        #[doc = "The time when this report either completed successfully or failed."]
        #[serde(
            rename = "finishTimeMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub finish_time_ms: ::std::option::Option<i64>,
        #[doc = "The file type of the report."]
        #[serde(
            rename = "format",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub format: ::std::option::Option<crate::schemas::ReportStatusFormat>,
        #[doc = "The state of the report."]
        #[serde(
            rename = "state",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub state: ::std::option::Option<crate::schemas::ReportStatusState>,
    }
    impl ::google_field_selector::FieldSelector for ReportStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ReportStatusFormat {
        Csv,
        ExcelCsv,
        Xlsx,
    }
    impl ReportStatusFormat {
        pub fn as_str(self) -> &'static str {
            match self {
                ReportStatusFormat::Csv => "CSV",
                ReportStatusFormat::ExcelCsv => "EXCEL_CSV",
                ReportStatusFormat::Xlsx => "XLSX",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ReportStatusFormat {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ReportStatusFormat {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ReportStatusFormat, ()> {
            Ok(match s {
                "CSV" => ReportStatusFormat::Csv,
                "EXCEL_CSV" => ReportStatusFormat::ExcelCsv,
                "XLSX" => ReportStatusFormat::Xlsx,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ReportStatusFormat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ReportStatusFormat {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReportStatusFormat {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CSV" => ReportStatusFormat::Csv,
                "EXCEL_CSV" => ReportStatusFormat::ExcelCsv,
                "XLSX" => ReportStatusFormat::Xlsx,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ReportStatusFormat {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportStatusFormat {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ReportStatusState {
        Done,
        Failed,
        Running,
    }
    impl ReportStatusState {
        pub fn as_str(self) -> &'static str {
            match self {
                ReportStatusState::Done => "DONE",
                ReportStatusState::Failed => "FAILED",
                ReportStatusState::Running => "RUNNING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ReportStatusState {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ReportStatusState {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ReportStatusState, ()> {
            Ok(match s {
                "DONE" => ReportStatusState::Done,
                "FAILED" => ReportStatusState::Failed,
                "RUNNING" => ReportStatusState::Running,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ReportStatusState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ReportStatusState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReportStatusState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DONE" => ReportStatusState::Done,
                "FAILED" => ReportStatusState::Failed,
                "RUNNING" => ReportStatusState::Running,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ReportStatusState {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ReportStatusState {
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
    pub struct Rule {
        #[serde(
            rename = "disjunctiveMatchStatements",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disjunctive_match_statements:
            ::std::option::Option<Vec<crate::schemas::DisjunctiveMatchStatement>>,
        #[doc = "Rule name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Rule {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Rule {
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
    pub struct RunQueryRequest {
        #[doc = "Report data range used to generate the report."]
        #[serde(
            rename = "dataRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub data_range: ::std::option::Option<crate::schemas::RunQueryRequestDataRange>,
        #[doc = "The ending time for the data that is shown in the report. Note, reportDataEndTimeMs is required if dataRange is CUSTOM_DATES and ignored otherwise."]
        #[serde(
            rename = "reportDataEndTimeMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub report_data_end_time_ms: ::std::option::Option<i64>,
        #[doc = "The starting time for the data that is shown in the report. Note, reportDataStartTimeMs is required if dataRange is CUSTOM_DATES and ignored otherwise."]
        #[serde(
            rename = "reportDataStartTimeMs",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub report_data_start_time_ms: ::std::option::Option<i64>,
        #[doc = "Canonical timezone code for report data time. Defaults to America/New_York."]
        #[serde(
            rename = "timezoneCode",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timezone_code: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for RunQueryRequest {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RunQueryRequest {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum RunQueryRequestDataRange {
        AllTime,
        CurrentDay,
        CustomDates,
        Last14Days,
        Last30Days,
        Last365Days,
        Last60Days,
        Last7Days,
        Last90Days,
        MonthToDate,
        PreviousDay,
        PreviousHalfMonth,
        PreviousMonth,
        PreviousQuarter,
        PreviousWeek,
        PreviousYear,
        QuarterToDate,
        TypeNotSupported,
        WeekToDate,
        YearToDate,
    }
    impl RunQueryRequestDataRange {
        pub fn as_str(self) -> &'static str {
            match self {
                RunQueryRequestDataRange::AllTime => "ALL_TIME",
                RunQueryRequestDataRange::CurrentDay => "CURRENT_DAY",
                RunQueryRequestDataRange::CustomDates => "CUSTOM_DATES",
                RunQueryRequestDataRange::Last14Days => "LAST_14_DAYS",
                RunQueryRequestDataRange::Last30Days => "LAST_30_DAYS",
                RunQueryRequestDataRange::Last365Days => "LAST_365_DAYS",
                RunQueryRequestDataRange::Last60Days => "LAST_60_DAYS",
                RunQueryRequestDataRange::Last7Days => "LAST_7_DAYS",
                RunQueryRequestDataRange::Last90Days => "LAST_90_DAYS",
                RunQueryRequestDataRange::MonthToDate => "MONTH_TO_DATE",
                RunQueryRequestDataRange::PreviousDay => "PREVIOUS_DAY",
                RunQueryRequestDataRange::PreviousHalfMonth => "PREVIOUS_HALF_MONTH",
                RunQueryRequestDataRange::PreviousMonth => "PREVIOUS_MONTH",
                RunQueryRequestDataRange::PreviousQuarter => "PREVIOUS_QUARTER",
                RunQueryRequestDataRange::PreviousWeek => "PREVIOUS_WEEK",
                RunQueryRequestDataRange::PreviousYear => "PREVIOUS_YEAR",
                RunQueryRequestDataRange::QuarterToDate => "QUARTER_TO_DATE",
                RunQueryRequestDataRange::TypeNotSupported => "TYPE_NOT_SUPPORTED",
                RunQueryRequestDataRange::WeekToDate => "WEEK_TO_DATE",
                RunQueryRequestDataRange::YearToDate => "YEAR_TO_DATE",
            }
        }
    }
    impl ::std::convert::AsRef<str> for RunQueryRequestDataRange {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for RunQueryRequestDataRange {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<RunQueryRequestDataRange, ()> {
            Ok(match s {
                "ALL_TIME" => RunQueryRequestDataRange::AllTime,
                "CURRENT_DAY" => RunQueryRequestDataRange::CurrentDay,
                "CUSTOM_DATES" => RunQueryRequestDataRange::CustomDates,
                "LAST_14_DAYS" => RunQueryRequestDataRange::Last14Days,
                "LAST_30_DAYS" => RunQueryRequestDataRange::Last30Days,
                "LAST_365_DAYS" => RunQueryRequestDataRange::Last365Days,
                "LAST_60_DAYS" => RunQueryRequestDataRange::Last60Days,
                "LAST_7_DAYS" => RunQueryRequestDataRange::Last7Days,
                "LAST_90_DAYS" => RunQueryRequestDataRange::Last90Days,
                "MONTH_TO_DATE" => RunQueryRequestDataRange::MonthToDate,
                "PREVIOUS_DAY" => RunQueryRequestDataRange::PreviousDay,
                "PREVIOUS_HALF_MONTH" => RunQueryRequestDataRange::PreviousHalfMonth,
                "PREVIOUS_MONTH" => RunQueryRequestDataRange::PreviousMonth,
                "PREVIOUS_QUARTER" => RunQueryRequestDataRange::PreviousQuarter,
                "PREVIOUS_WEEK" => RunQueryRequestDataRange::PreviousWeek,
                "PREVIOUS_YEAR" => RunQueryRequestDataRange::PreviousYear,
                "QUARTER_TO_DATE" => RunQueryRequestDataRange::QuarterToDate,
                "TYPE_NOT_SUPPORTED" => RunQueryRequestDataRange::TypeNotSupported,
                "WEEK_TO_DATE" => RunQueryRequestDataRange::WeekToDate,
                "YEAR_TO_DATE" => RunQueryRequestDataRange::YearToDate,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for RunQueryRequestDataRange {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for RunQueryRequestDataRange {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for RunQueryRequestDataRange {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL_TIME" => RunQueryRequestDataRange::AllTime,
                "CURRENT_DAY" => RunQueryRequestDataRange::CurrentDay,
                "CUSTOM_DATES" => RunQueryRequestDataRange::CustomDates,
                "LAST_14_DAYS" => RunQueryRequestDataRange::Last14Days,
                "LAST_30_DAYS" => RunQueryRequestDataRange::Last30Days,
                "LAST_365_DAYS" => RunQueryRequestDataRange::Last365Days,
                "LAST_60_DAYS" => RunQueryRequestDataRange::Last60Days,
                "LAST_7_DAYS" => RunQueryRequestDataRange::Last7Days,
                "LAST_90_DAYS" => RunQueryRequestDataRange::Last90Days,
                "MONTH_TO_DATE" => RunQueryRequestDataRange::MonthToDate,
                "PREVIOUS_DAY" => RunQueryRequestDataRange::PreviousDay,
                "PREVIOUS_HALF_MONTH" => RunQueryRequestDataRange::PreviousHalfMonth,
                "PREVIOUS_MONTH" => RunQueryRequestDataRange::PreviousMonth,
                "PREVIOUS_QUARTER" => RunQueryRequestDataRange::PreviousQuarter,
                "PREVIOUS_WEEK" => RunQueryRequestDataRange::PreviousWeek,
                "PREVIOUS_YEAR" => RunQueryRequestDataRange::PreviousYear,
                "QUARTER_TO_DATE" => RunQueryRequestDataRange::QuarterToDate,
                "TYPE_NOT_SUPPORTED" => RunQueryRequestDataRange::TypeNotSupported,
                "WEEK_TO_DATE" => RunQueryRequestDataRange::WeekToDate,
                "YEAR_TO_DATE" => RunQueryRequestDataRange::YearToDate,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for RunQueryRequestDataRange {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for RunQueryRequestDataRange {
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
    #[doc = "Actions that can be performed on the queries resource"]
    pub fn queries(&self) -> crate::resources::queries::QueriesActions {
        crate::resources::queries::QueriesActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
    #[doc = "Actions that can be performed on the reports resource"]
    pub fn reports(&self) -> crate::resources::reports::ReportsActions {
        crate::resources::reports::ReportsActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod queries {
        pub mod params {}
        pub struct QueriesActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> QueriesActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Creates a query."]
            pub fn createquery(&self, request: crate::schemas::Query) -> CreatequeryRequestBuilder {
                CreatequeryRequestBuilder {
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
                    asynchronous: None,
                }
            }
            #[doc = "Deletes a stored query as well as the associated stored reports."]
            pub fn deletequery(&self, query_id: i64) -> DeletequeryRequestBuilder {
                DeletequeryRequestBuilder {
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
                    query_id,
                }
            }
            #[doc = "Retrieves a stored query."]
            pub fn getquery(&self, query_id: i64) -> GetqueryRequestBuilder {
                GetqueryRequestBuilder {
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
                    query_id,
                }
            }
            #[doc = "Retrieves stored queries."]
            pub fn listqueries(&self) -> ListqueriesRequestBuilder {
                ListqueriesRequestBuilder {
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
            #[doc = "Runs a stored query to generate a report."]
            pub fn runquery(
                &self,
                request: crate::schemas::RunQueryRequest,
                query_id: i64,
            ) -> RunqueryRequestBuilder {
                RunqueryRequestBuilder {
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
                    query_id,
                    asynchronous: None,
                }
            }
        }
        #[doc = "Created via [QueriesActions::createquery()](struct.QueriesActions.html#method.createquery)"]
        #[derive(Debug, Clone)]
        pub struct CreatequeryRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::Query,
            asynchronous: ::std::option::Option<bool>,
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
        impl<'a> CreatequeryRequestBuilder<'a> {
            #[doc = "If true, tries to run the query asynchronously. Only applicable when the frequency is ONE_TIME."]
            pub fn asynchronous(mut self, value: bool) -> Self {
                self.asynchronous = Some(value);
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
            ) -> Result<crate::schemas::Query, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Query, crate::Error> {
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
                    "https://doubleclickbidmanager.googleapis.com/doubleclickbidmanager/v1.1/"
                        .to_owned();
                output.push_str("query");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("asynchronous", &self.asynchronous)]);
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
        #[doc = "Created via [QueriesActions::deletequery()](struct.QueriesActions.html#method.deletequery)"]
        #[derive(Debug, Clone)]
        pub struct DeletequeryRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            query_id: i64,
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
        impl<'a> DeletequeryRequestBuilder<'a> {
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
            pub async fn execute(self) -> Result<(), crate::Error> {
                let req = self._request(&self._path()).await?;
                req.send().await?.error_for_status()?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://doubleclickbidmanager.googleapis.com/doubleclickbidmanager/v1.1/"
                        .to_owned();
                output.push_str("query/");
                {
                    let var_as_string = self.query_id.to_string();
                    let var_as_str = &var_as_string;
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
        #[doc = "Created via [QueriesActions::getquery()](struct.QueriesActions.html#method.getquery)"]
        #[derive(Debug, Clone)]
        pub struct GetqueryRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            query_id: i64,
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
        impl<'a> GetqueryRequestBuilder<'a> {
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
            ) -> Result<crate::schemas::Query, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Query, crate::Error> {
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
                    "https://doubleclickbidmanager.googleapis.com/doubleclickbidmanager/v1.1/"
                        .to_owned();
                output.push_str("query/");
                {
                    let var_as_string = self.query_id.to_string();
                    let var_as_str = &var_as_string;
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
        #[doc = "Created via [QueriesActions::listqueries()](struct.QueriesActions.html#method.listqueries)"]
        #[derive(Debug, Clone)]
        pub struct ListqueriesRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
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
        impl<'a> ListqueriesRequestBuilder<'a> {
            #[doc = "Maximum number of results per page. Must be between 1 and 100. Defaults to 100 if unspecified."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "Optional pagination token."]
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
            #[doc = "\nExecute the request and yield each item in the `queries` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
            pub fn stream_queries<T>(
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
                self.stream_queries_with_fields(fields)
            }
            #[doc = "\nExecute the request and yield each item in the `queries` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
            pub fn stream_queries_with_default_fields(
                self,
            ) -> impl ::futures::Stream<Item = Result<crate::schemas::Query, crate::Error>> + 'a
            {
                self.stream_queries_with_fields(None::<String>)
            }
            #[doc = "\nExecute the request and yield each item in the `queries` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
            pub fn stream_queries_with_all_fields(
                self,
            ) -> impl ::futures::Stream<Item = Result<crate::schemas::Query, crate::Error>> + 'a
            {
                self.stream_queries_with_fields(Some("*"))
            }
            #[doc = "\nExecute the request and yield each item in the `queries` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
            pub fn stream_queries_with_fields<T, F>(
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
                    #[serde(rename = "queries")]
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
                    let mut selector = concat!("nextPageToken,", "queries").to_owned();
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
            #[doc = r" Requests the default set of fields from the server."]
            pub fn stream_with_default_fields(
                self,
            ) -> impl ::futures::Stream<
                Item = Result<crate::schemas::ListQueriesResponse, crate::Error>,
            > + 'a {
                self.stream_with_fields(None::<&str>)
            }
            #[doc = r" Execute the request and yield the returned value. If the response contains a"]
            #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
            #[doc = r" repeated until no page token is returned."]
            #[doc = r""]
            #[doc = r" Requests all fields from the server."]
            pub fn stream_with_all_fields(
                self,
            ) -> impl ::futures::Stream<
                Item = Result<crate::schemas::ListQueriesResponse, crate::Error>,
            > + 'a {
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
            ) -> Result<crate::schemas::ListQueriesResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListQueriesResponse, crate::Error> {
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
                    "https://doubleclickbidmanager.googleapis.com/doubleclickbidmanager/v1.1/"
                        .to_owned();
                output.push_str("queries");
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
        #[async_trait::async_trait]
        impl<'a> crate::stream::StreamableMethod for ListqueriesRequestBuilder<'a> {
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
        #[doc = "Created via [QueriesActions::runquery()](struct.QueriesActions.html#method.runquery)"]
        #[derive(Debug, Clone)]
        pub struct RunqueryRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            request: crate::schemas::RunQueryRequest,
            query_id: i64,
            asynchronous: ::std::option::Option<bool>,
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
        impl<'a> RunqueryRequestBuilder<'a> {
            #[doc = "If true, tries to run the query asynchronously."]
            pub fn asynchronous(mut self, value: bool) -> Self {
                self.asynchronous = Some(value);
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
            pub async fn execute(self) -> Result<(), crate::Error> {
                let req = self._request(&self._path()).await?;
                let req = req.json(&self.request);
                req.send().await?.error_for_status()?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://doubleclickbidmanager.googleapis.com/doubleclickbidmanager/v1.1/"
                        .to_owned();
                output.push_str("query/");
                {
                    let var_as_string = self.query_id.to_string();
                    let var_as_str = &var_as_string;
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
                let mut req = self.reqwest.request(::reqwest::Method::POST, path);
                req = req.query(&[("asynchronous", &self.asynchronous)]);
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
    pub mod reports {
        pub mod params {}
        pub struct ReportsActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> ReportsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Retrieves stored reports."]
            pub fn listreports(&self, query_id: i64) -> ListreportsRequestBuilder {
                ListreportsRequestBuilder {
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
                    query_id,
                    page_size: None,
                    page_token: None,
                }
            }
        }
        #[doc = "Created via [ReportsActions::listreports()](struct.ReportsActions.html#method.listreports)"]
        #[derive(Debug, Clone)]
        pub struct ListreportsRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            query_id: i64,
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
        impl<'a> ListreportsRequestBuilder<'a> {
            #[doc = "Maximum number of results per page. Must be between 1 and 100. Defaults to 100 if unspecified."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "Optional pagination token."]
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
            #[doc = "\nExecute the request and yield each item in the `reports` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the field given by the [`FieldSelector`] implementation from the server.\n\n[`FieldSelector`]: ::google_field_selector::FieldSelector\n"]
            pub fn stream_reports<T>(
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
                self.stream_reports_with_fields(fields)
            }
            #[doc = "\nExecute the request and yield each item in the `reports` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests the default set of fields from the server.\n"]
            pub fn stream_reports_with_default_fields(
                self,
            ) -> impl ::futures::Stream<Item = Result<crate::schemas::Report, crate::Error>> + 'a
            {
                self.stream_reports_with_fields(None::<String>)
            }
            #[doc = "\nExecute the request and yield each item in the `reports` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
            pub fn stream_reports_with_all_fields(
                self,
            ) -> impl ::futures::Stream<Item = Result<crate::schemas::Report, crate::Error>> + 'a
            {
                self.stream_reports_with_fields(Some("*"))
            }
            #[doc = "\nExecute the request and yield each item in the `reports` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nOnly the given `fields` are requested from the server.\n"]
            pub fn stream_reports_with_fields<T, F>(
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
                    #[serde(rename = "reports")]
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
                    let mut selector = concat!("nextPageToken,", "reports").to_owned();
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
            #[doc = r" Requests the default set of fields from the server."]
            pub fn stream_with_default_fields(
                self,
            ) -> impl ::futures::Stream<
                Item = Result<crate::schemas::ListReportsResponse, crate::Error>,
            > + 'a {
                self.stream_with_fields(None::<&str>)
            }
            #[doc = r" Execute the request and yield the returned value. If the response contains a"]
            #[doc = r" `nextPageToken`, the request is executed again with the new token. This process is"]
            #[doc = r" repeated until no page token is returned."]
            #[doc = r""]
            #[doc = r" Requests all fields from the server."]
            pub fn stream_with_all_fields(
                self,
            ) -> impl ::futures::Stream<
                Item = Result<crate::schemas::ListReportsResponse, crate::Error>,
            > + 'a {
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
            ) -> Result<crate::schemas::ListReportsResponse, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::ListReportsResponse, crate::Error> {
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
                    "https://doubleclickbidmanager.googleapis.com/doubleclickbidmanager/v1.1/"
                        .to_owned();
                output.push_str("queries/");
                {
                    let var_as_string = self.query_id.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/reports");
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
        #[async_trait::async_trait]
        impl<'a> crate::stream::StreamableMethod for ListreportsRequestBuilder<'a> {
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
