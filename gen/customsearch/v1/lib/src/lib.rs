#![allow(rustdoc::bare_urls)]
#![doc = "# Resources and Methods\n* [cse](resources/cse/struct.CseActions.html)\n  * [*list*](resources/cse/struct.ListRequestBuilder.html)\n  * [siterestrict](resources/cse/siterestrict/struct.SiterestrictActions.html)\n    * [*list*](resources/cse/siterestrict/struct.ListRequestBuilder.html)\n"]
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
    pub struct Promotion {
        #[doc = "An array of block objects for this promotion."]
        #[serde(
            rename = "bodyLines",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub body_lines: ::std::option::Option<Vec<crate::schemas::PromotionBodyLinesItems>>,
        #[doc = "An abridged version of this search’s result URL, e.g. www.example.com."]
        #[serde(
            rename = "displayLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_link: ::std::option::Option<String>,
        #[doc = "The title of the promotion, in HTML."]
        #[serde(
            rename = "htmlTitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub html_title: ::std::option::Option<String>,
        #[doc = "Image belonging to a promotion."]
        #[serde(
            rename = "image",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image: ::std::option::Option<crate::schemas::PromotionImage>,
        #[doc = "The URL of the promotion."]
        #[serde(
            rename = "link",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link: ::std::option::Option<String>,
        #[doc = "The title of the promotion."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Promotion {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Promotion {
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
    pub struct PromotionBodyLinesItems {
        #[doc = "The block object’s text in HTML, if it has text."]
        #[serde(
            rename = "htmlTitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub html_title: ::std::option::Option<String>,
        #[doc = "The anchor text of the block object’s link, if it has a link."]
        #[serde(
            rename = "link",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link: ::std::option::Option<String>,
        #[doc = "The block object’s text, if it has text."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "The URL of the block object’s link, if it has one."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for PromotionBodyLinesItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PromotionBodyLinesItems {
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
    pub struct PromotionImage {
        #[doc = "Image height in pixels."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[doc = "URL of the image for this promotion link."]
        #[serde(
            rename = "source",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub source: ::std::option::Option<String>,
        #[doc = "Image width in pixels."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for PromotionImage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for PromotionImage {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Result {
        #[doc = "Indicates the ID of Google’s cached version of the search result."]
        #[serde(
            rename = "cacheId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cache_id: ::std::option::Option<String>,
        #[doc = "An abridged version of this search result’s URL, e.g. www.example.com."]
        #[serde(
            rename = "displayLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_link: ::std::option::Option<String>,
        #[doc = "The file format of the search result."]
        #[serde(
            rename = "fileFormat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_format: ::std::option::Option<String>,
        #[doc = "The URL displayed after the snippet for each search result."]
        #[serde(
            rename = "formattedUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_url: ::std::option::Option<String>,
        #[doc = "The HTML-formatted URL displayed after the snippet for each search result."]
        #[serde(
            rename = "htmlFormattedUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub html_formatted_url: ::std::option::Option<String>,
        #[doc = "The snippet of the search result, in HTML."]
        #[serde(
            rename = "htmlSnippet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub html_snippet: ::std::option::Option<String>,
        #[doc = "The title of the search result, in HTML."]
        #[serde(
            rename = "htmlTitle",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub html_title: ::std::option::Option<String>,
        #[doc = "Image belonging to a custom search result."]
        #[serde(
            rename = "image",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image: ::std::option::Option<crate::schemas::ResultImage>,
        #[doc = "A unique identifier for the type of current object. For this API, it is `customsearch#result.`"]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "Encapsulates all information about refinement labels."]
        #[serde(
            rename = "labels",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub labels: ::std::option::Option<Vec<crate::schemas::ResultLabelsItems>>,
        #[doc = "The full URL to which the search result is pointing, e.g. http://www.example.com/foo/bar."]
        #[serde(
            rename = "link",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link: ::std::option::Option<String>,
        #[doc = "The MIME type of the search result."]
        #[serde(
            rename = "mime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mime: ::std::option::Option<String>,
        #[doc = "Contains [PageMap](https://developers.google.com/custom-search/docs/structured_data#pagemaps) information for this search result."]
        #[serde(
            rename = "pagemap",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pagemap:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The snippet of the search result, in plain text."]
        #[serde(
            rename = "snippet",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub snippet: ::std::option::Option<String>,
        #[doc = "The title of the search result, in plain text."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Result {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Result {
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
    pub struct ResultImage {
        #[doc = "The size of the image, in pixels."]
        #[serde(
            rename = "byteSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub byte_size: ::std::option::Option<i32>,
        #[doc = "A URL pointing to the webpage hosting the image."]
        #[serde(
            rename = "contextLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub context_link: ::std::option::Option<String>,
        #[doc = "The height of the image, in pixels."]
        #[serde(
            rename = "height",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub height: ::std::option::Option<i32>,
        #[doc = "The height of the thumbnail image, in pixels."]
        #[serde(
            rename = "thumbnailHeight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnail_height: ::std::option::Option<i32>,
        #[doc = "A URL to the thumbnail image."]
        #[serde(
            rename = "thumbnailLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnail_link: ::std::option::Option<String>,
        #[doc = "The width of the thumbnail image, in pixels."]
        #[serde(
            rename = "thumbnailWidth",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub thumbnail_width: ::std::option::Option<i32>,
        #[doc = "The width of the image, in pixels."]
        #[serde(
            rename = "width",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub width: ::std::option::Option<i32>,
    }
    impl ::google_field_selector::FieldSelector for ResultImage {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResultImage {
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
    pub struct ResultLabelsItems {
        #[doc = "The display name of a refinement label. This is the name you should display in your user interface."]
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub display_name: ::std::option::Option<String>,
        #[doc = "Refinement label and the associated refinement operation."]
        #[serde(
            rename = "label_with_op",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub label_with_op: ::std::option::Option<String>,
        #[doc = "The name of a refinement label, which you can use to refine searches. Don’t display this in your user interface; instead, use displayName."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ResultLabelsItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ResultLabelsItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Search {
        #[doc = "Metadata and refinements associated with the given search engine, including: * The name of the search engine that was used for the query. * A set of [facet objects](https://developers.google.com/custom-search/docs/refinements#create) (refinements) you can use for refining a search."]
        #[serde(
            rename = "context",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub context:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The current set of custom search results."]
        #[serde(
            rename = "items",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub items: ::std::option::Option<Vec<crate::schemas::Result>>,
        #[doc = "Unique identifier for the type of current object. For this API, it is customsearch#search."]
        #[serde(
            rename = "kind",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub kind: ::std::option::Option<String>,
        #[doc = "The set of [promotions](https://developers.google.com/custom-search/docs/promotions). Present only if the custom search engine’s configuration files define any promotions for the given query."]
        #[serde(
            rename = "promotions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub promotions: ::std::option::Option<Vec<crate::schemas::Promotion>>,
        #[doc = "Query metadata for the previous, current, and next pages of results."]
        #[serde(
            rename = "queries",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub queries: ::std::option::Option<crate::schemas::SearchQueries>,
        #[doc = "Metadata about a search operation."]
        #[serde(
            rename = "searchInformation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_information: ::std::option::Option<crate::schemas::SearchSearchInformation>,
        #[doc = "Spell correction information for a query."]
        #[serde(
            rename = "spelling",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub spelling: ::std::option::Option<crate::schemas::SearchSpelling>,
        #[doc = "OpenSearch template and URL."]
        #[serde(
            rename = "url",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub url: ::std::option::Option<crate::schemas::SearchUrl>,
    }
    impl ::google_field_selector::FieldSelector for Search {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Search {
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
    pub struct SearchQueries {
        #[doc = "Metadata representing the next page of results, if applicable."]
        #[serde(
            rename = "nextPage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page: ::std::option::Option<Vec<crate::schemas::SearchQueriesNextPageItems>>,
        #[doc = "Metadata representing the previous page of results, if applicable."]
        #[serde(
            rename = "previousPage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub previous_page:
            ::std::option::Option<Vec<crate::schemas::SearchQueriesPreviousPageItems>>,
        #[doc = "Metadata representing the current request."]
        #[serde(
            rename = "request",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub request: ::std::option::Option<Vec<crate::schemas::SearchQueriesRequestItems>>,
    }
    impl ::google_field_selector::FieldSelector for SearchQueries {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchQueries {
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
    pub struct SearchQueriesNextPageItems {
        #[doc = "Number of search results returned in this set."]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub count: ::std::option::Option<i32>,
        #[doc = "Restricts search results to documents originating in a particular country. You may use [Boolean operators](https://developers.google.com/custom-search/docs/json_api_reference#BooleanOrSearch) in the `cr` parameter’s value. Google WebSearch determines the country of a document by analyzing the following: * The top-level domain (TLD) of the document’s URL. * The geographic location of the web server’s IP address. See [Country (cr) Parameter Values](https://developers.google.com/custom-search/docs/json_api_reference#countryCollections) for a list of valid values for this parameter."]
        #[serde(
            rename = "cr",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cr: ::std::option::Option<String>,
        #[doc = "The identifier of an engine created using the Programmable Search Engine [Control Panel](https://programmablesearchengine.google.com/). This is a custom property not defined in the OpenSearch spec. This parameter is **required**."]
        #[serde(
            rename = "cx",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cx: ::std::option::Option<String>,
        #[doc = "Restricts results to URLs based on date. Supported values include: * `d[number]`: requests results from the specified number of past days. * `w[number]`: requests results from the specified number of past weeks. * `m[number]`: requests results from the specified number of past months. * `y[number]`: requests results from the specified number of past years."]
        #[serde(
            rename = "dateRestrict",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_restrict: ::std::option::Option<String>,
        #[doc = "Enables or disables the [Simplified and Traditional Chinese Search](https://developers.google.com/custom-search/docs/json_api_reference#chineseSearch) feature. Supported values are: * `0`: enabled (default) * `1`: disabled"]
        #[serde(
            rename = "disableCnTwTranslation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_cn_tw_translation: ::std::option::Option<String>,
        #[doc = "Identifies a phrase that all documents in the search results must contain."]
        #[serde(
            rename = "exactTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exact_terms: ::std::option::Option<String>,
        #[doc = "Identifies a word or phrase that should not appear in any documents in the search results."]
        #[serde(
            rename = "excludeTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exclude_terms: ::std::option::Option<String>,
        #[doc = "Restricts results to files of a specified extension. Filetypes supported by Google include: * Adobe Portable Document Format (`pdf`) * Adobe PostScript (`ps`) * Lotus 1-2-3 (`wk1`, `wk2`, `wk3`, `wk4`, `wk5`, `wki`, `wks`, `wku`) * Lotus WordPro (`lwp`) * Macwrite (`mw`) * Microsoft Excel (`xls`) * Microsoft PowerPoint (`ppt`) * Microsoft Word (`doc`) * Microsoft Works (`wks`, `wps`, `wdb`) * Microsoft Write (`wri`) * Rich Text Format (`rtf`) * Shockwave Flash (`swf`) * Text (`ans`, `txt`). Additional filetypes may be added in the future. An up-to-date list can always be found in Google’s [file type FAQ](https://support.google.com/webmasters/answer/35287)."]
        #[serde(
            rename = "fileType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_type: ::std::option::Option<String>,
        #[doc = "Activates or deactivates the automatic filtering of Google search results. See [Automatic Filtering](https://developers.google.com/custom-search/docs/json_api_reference#automaticFiltering) for more information about Google’s search results filters. Valid values for this parameter are: * `0`: Disabled * `1`: Enabled (default) **Note**: By default, Google applies filtering to all search results to improve the quality of those results."]
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter: ::std::option::Option<String>,
        #[doc = "Boosts search results whose country of origin matches the parameter value. See [Country Codes](https://developers.google.com/custom-search/docs/json_api_reference#countryCodes) for a list of valid values. Specifying a `gl` parameter value in WebSearch requests should improve the relevance of results. This is particularly true for international customers and, even more specifically, for customers in English-speaking countries other than the United States."]
        #[serde(
            rename = "gl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gl: ::std::option::Option<String>,
        #[doc = "Specifies the Google domain (for example, google.com, google.de, or google.fr) to which the search should be limited."]
        #[serde(
            rename = "googleHost",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_host: ::std::option::Option<String>,
        #[doc = "Specifies the ending value for a search range. Use `cse:lowRange` and `cse:highrange` to append an inclusive search range of `lowRange...highRange` to the query."]
        #[serde(
            rename = "highRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub high_range: ::std::option::Option<String>,
        #[doc = "Specifies the interface language (host language) of your user interface. Explicitly setting this parameter improves the performance and the quality of your search results. See the [Interface Languages](https://developers.google.com/custom-search/docs/json_api_reference#wsInterfaceLanguages) section of [Internationalizing Queries and Results Presentation](https://developers.google.com/custom-search/docs/json_api_reference#wsInternationalizing) for more information, and [Supported Interface Languages](https://developers.google.com/custom-search/docs/json_api_reference#interfaceLanguages) for a list of supported languages."]
        #[serde(
            rename = "hl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hl: ::std::option::Option<String>,
        #[doc = "Appends the specified query terms to the query, as if they were combined with a logical `AND` operator."]
        #[serde(
            rename = "hq",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hq: ::std::option::Option<String>,
        #[doc = "Restricts results to images of a specified color type. Supported values are: * `mono` (black and white) * `gray` (grayscale) * `color` (color)"]
        #[serde(
            rename = "imgColorType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_color_type: ::std::option::Option<String>,
        #[doc = "Restricts results to images with a specific dominant color. Supported values are: * `red` * `orange` * `yellow` * `green` * `teal` * `blue` * `purple` * `pink` * `white` * `gray` * `black` * `brown`"]
        #[serde(
            rename = "imgDominantColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_dominant_color: ::std::option::Option<String>,
        #[doc = "Restricts results to images of a specified size. Supported values are: * `icon` (small) * `small | medium | large | xlarge` (medium) * `xxlarge` (large) * `huge` (extra-large)"]
        #[serde(
            rename = "imgSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_size: ::std::option::Option<String>,
        #[doc = "Restricts results to images of a specified type. Supported values are: * `clipart` (Clip art) * `face` (Face) * `lineart` (Line drawing) * `photo` (Photo) * `animated` (Animated) * `stock` (Stock)"]
        #[serde(
            rename = "imgType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_type: ::std::option::Option<String>,
        #[doc = "The character encoding supported for search requests."]
        #[serde(
            rename = "inputEncoding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_encoding: ::std::option::Option<String>,
        #[doc = "The language of the search results."]
        #[serde(
            rename = "language",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language: ::std::option::Option<String>,
        #[doc = "Specifies that all results should contain a link to a specific URL."]
        #[serde(
            rename = "linkSite",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link_site: ::std::option::Option<String>,
        #[doc = "Specifies the starting value for a search range. Use `cse:lowRange` and `cse:highrange` to append an inclusive search range of `lowRange...highRange` to the query."]
        #[serde(
            rename = "lowRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub low_range: ::std::option::Option<String>,
        #[doc = "Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms. You can also use the [Boolean OR](https://developers.google.com/custom-search/docs/json_api_reference#BooleanOrSearch) query term for this type of query."]
        #[serde(
            rename = "orTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub or_terms: ::std::option::Option<String>,
        #[doc = "The character encoding supported for search results."]
        #[serde(
            rename = "outputEncoding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_encoding: ::std::option::Option<String>,
        #[doc = "Specifies that all search results should be pages that are related to the specified URL. The parameter value should be a URL."]
        #[serde(
            rename = "relatedSite",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub related_site: ::std::option::Option<String>,
        #[doc = "Filters based on licensing. Supported values include: * `cc_publicdomain` * `cc_attribute` * `cc_sharealike` * `cc_noncommercial` * `cc_nonderived`"]
        #[serde(
            rename = "rights",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rights: ::std::option::Option<String>,
        #[doc = "Specifies the [SafeSearch level](https://developers.google.com/custom-search/docs/json_api_reference#safeSearchLevels) used for filtering out adult results. This is a custom property not defined in the OpenSearch spec. Valid parameter values are: * `\"off\"`: Disable SafeSearch * `\"active\"`: Enable SafeSearch"]
        #[serde(
            rename = "safe",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub safe: ::std::option::Option<String>,
        #[doc = "The search terms entered by the user."]
        #[serde(
            rename = "searchTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_terms: ::std::option::Option<String>,
        #[doc = "Allowed values are `web` or `image`. If unspecified, results are limited to webpages."]
        #[serde(
            rename = "searchType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_type: ::std::option::Option<String>,
        #[doc = "Restricts results to URLs from a specified site."]
        #[serde(
            rename = "siteSearch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub site_search: ::std::option::Option<String>,
        #[doc = "Specifies whether to include or exclude results from the site named in the `sitesearch` parameter. Supported values are: * `i`: include content from site * `e`: exclude content from site"]
        #[serde(
            rename = "siteSearchFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub site_search_filter: ::std::option::Option<String>,
        #[doc = "Specifies that results should be sorted according to the specified expression. For example, sort by date."]
        #[serde(
            rename = "sort",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sort: ::std::option::Option<String>,
        #[doc = "The index of the current set of search results into the total set of results, where the index of the first result is 1."]
        #[serde(
            rename = "startIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_index: ::std::option::Option<i32>,
        #[doc = "The page number of this set of results, where the page length is set by the `count` property."]
        #[serde(
            rename = "startPage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_page: ::std::option::Option<i32>,
        #[doc = "A description of the query."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "Estimated number of total search results. May not be accurate."]
        #[serde(
            rename = "totalResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_results: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for SearchQueriesNextPageItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchQueriesNextPageItems {
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
    pub struct SearchQueriesPreviousPageItems {
        #[doc = "Number of search results returned in this set."]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub count: ::std::option::Option<i32>,
        #[doc = "Restricts search results to documents originating in a particular country. You may use [Boolean operators](https://developers.google.com/custom-search/docs/json_api_reference#BooleanOrSearch) in the `cr` parameter’s value. Google WebSearch determines the country of a document by analyzing the following: * The top-level domain (TLD) of the document’s URL. * The geographic location of the web server’s IP address. See [Country (cr) Parameter Values](https://developers.google.com/custom-search/docs/json_api_reference#countryCollections) for a list of valid values for this parameter."]
        #[serde(
            rename = "cr",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cr: ::std::option::Option<String>,
        #[doc = "The identifier of an engine created using the Programmable Search Engine [Control Panel](https://programmablesearchengine.google.com/). This is a custom property not defined in the OpenSearch spec. This parameter is **required**."]
        #[serde(
            rename = "cx",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cx: ::std::option::Option<String>,
        #[doc = "Restricts results to URLs based on date. Supported values include: * `d[number]`: requests results from the specified number of past days. * `w[number]`: requests results from the specified number of past weeks. * `m[number]`: requests results from the specified number of past months. * `y[number]`: requests results from the specified number of past years."]
        #[serde(
            rename = "dateRestrict",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_restrict: ::std::option::Option<String>,
        #[doc = "Enables or disables the [Simplified and Traditional Chinese Search](https://developers.google.com/custom-search/docs/json_api_reference#chineseSearch) feature. Supported values are: * `0`: enabled (default) * `1`: disabled"]
        #[serde(
            rename = "disableCnTwTranslation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_cn_tw_translation: ::std::option::Option<String>,
        #[doc = "Identifies a phrase that all documents in the search results must contain."]
        #[serde(
            rename = "exactTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exact_terms: ::std::option::Option<String>,
        #[doc = "Identifies a word or phrase that should not appear in any documents in the search results."]
        #[serde(
            rename = "excludeTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exclude_terms: ::std::option::Option<String>,
        #[doc = "Restricts results to files of a specified extension. Filetypes supported by Google include: * Adobe Portable Document Format (`pdf`) * Adobe PostScript (`ps`) * Lotus 1-2-3 (`wk1`, `wk2`, `wk3`, `wk4`, `wk5`, `wki`, `wks`, `wku`) * Lotus WordPro (`lwp`) * Macwrite (`mw`) * Microsoft Excel (`xls`) * Microsoft PowerPoint (`ppt`) * Microsoft Word (`doc`) * Microsoft Works (`wks`, `wps`, `wdb`) * Microsoft Write (`wri`) * Rich Text Format (`rtf`) * Shockwave Flash (`swf`) * Text (`ans`, `txt`). Additional filetypes may be added in the future. An up-to-date list can always be found in Google’s [file type FAQ](https://support.google.com/webmasters/answer/35287)."]
        #[serde(
            rename = "fileType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_type: ::std::option::Option<String>,
        #[doc = "Activates or deactivates the automatic filtering of Google search results. See [Automatic Filtering](https://developers.google.com/custom-search/docs/json_api_reference#automaticFiltering) for more information about Google’s search results filters. Valid values for this parameter are: * `0`: Disabled * `1`: Enabled (default) **Note**: By default, Google applies filtering to all search results to improve the quality of those results."]
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter: ::std::option::Option<String>,
        #[doc = "Boosts search results whose country of origin matches the parameter value. See [Country Codes](https://developers.google.com/custom-search/docs/json_api_reference#countryCodes) for a list of valid values. Specifying a `gl` parameter value in WebSearch requests should improve the relevance of results. This is particularly true for international customers and, even more specifically, for customers in English-speaking countries other than the United States."]
        #[serde(
            rename = "gl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gl: ::std::option::Option<String>,
        #[doc = "Specifies the Google domain (for example, google.com, google.de, or google.fr) to which the search should be limited."]
        #[serde(
            rename = "googleHost",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_host: ::std::option::Option<String>,
        #[doc = "Specifies the ending value for a search range. Use `cse:lowRange` and `cse:highrange` to append an inclusive search range of `lowRange...highRange` to the query."]
        #[serde(
            rename = "highRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub high_range: ::std::option::Option<String>,
        #[doc = "Specifies the interface language (host language) of your user interface. Explicitly setting this parameter improves the performance and the quality of your search results. See the [Interface Languages](https://developers.google.com/custom-search/docs/json_api_reference#wsInterfaceLanguages) section of [Internationalizing Queries and Results Presentation](https://developers.google.com/custom-search/docs/json_api_reference#wsInternationalizing) for more information, and [Supported Interface Languages](https://developers.google.com/custom-search/docs/json_api_reference#interfaceLanguages) for a list of supported languages."]
        #[serde(
            rename = "hl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hl: ::std::option::Option<String>,
        #[doc = "Appends the specified query terms to the query, as if they were combined with a logical `AND` operator."]
        #[serde(
            rename = "hq",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hq: ::std::option::Option<String>,
        #[doc = "Restricts results to images of a specified color type. Supported values are: * `mono` (black and white) * `gray` (grayscale) * `color` (color)"]
        #[serde(
            rename = "imgColorType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_color_type: ::std::option::Option<String>,
        #[doc = "Restricts results to images with a specific dominant color. Supported values are: * `red` * `orange` * `yellow` * `green` * `teal` * `blue` * `purple` * `pink` * `white` * `gray` * `black` * `brown`"]
        #[serde(
            rename = "imgDominantColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_dominant_color: ::std::option::Option<String>,
        #[doc = "Restricts results to images of a specified size. Supported values are: * `icon` (small) * `small | medium | large | xlarge` (medium) * `xxlarge` (large) * `huge` (extra-large)"]
        #[serde(
            rename = "imgSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_size: ::std::option::Option<String>,
        #[doc = "Restricts results to images of a specified type. Supported values are: * `clipart` (Clip art) * `face` (Face) * `lineart` (Line drawing) * `photo` (Photo) * `animated` (Animated) * `stock` (Stock)"]
        #[serde(
            rename = "imgType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_type: ::std::option::Option<String>,
        #[doc = "The character encoding supported for search requests."]
        #[serde(
            rename = "inputEncoding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_encoding: ::std::option::Option<String>,
        #[doc = "The language of the search results."]
        #[serde(
            rename = "language",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language: ::std::option::Option<String>,
        #[doc = "Specifies that all results should contain a link to a specific URL."]
        #[serde(
            rename = "linkSite",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link_site: ::std::option::Option<String>,
        #[doc = "Specifies the starting value for a search range. Use `cse:lowRange` and `cse:highrange` to append an inclusive search range of `lowRange...highRange` to the query."]
        #[serde(
            rename = "lowRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub low_range: ::std::option::Option<String>,
        #[doc = "Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms. You can also use the [Boolean OR](https://developers.google.com/custom-search/docs/json_api_reference#BooleanOrSearch) query term for this type of query."]
        #[serde(
            rename = "orTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub or_terms: ::std::option::Option<String>,
        #[doc = "The character encoding supported for search results."]
        #[serde(
            rename = "outputEncoding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_encoding: ::std::option::Option<String>,
        #[doc = "Specifies that all search results should be pages that are related to the specified URL. The parameter value should be a URL."]
        #[serde(
            rename = "relatedSite",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub related_site: ::std::option::Option<String>,
        #[doc = "Filters based on licensing. Supported values include: * `cc_publicdomain` * `cc_attribute` * `cc_sharealike` * `cc_noncommercial` * `cc_nonderived`"]
        #[serde(
            rename = "rights",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rights: ::std::option::Option<String>,
        #[doc = "Specifies the [SafeSearch level](https://developers.google.com/custom-search/docs/json_api_reference#safeSearchLevels) used for filtering out adult results. This is a custom property not defined in the OpenSearch spec. Valid parameter values are: * `\"off\"`: Disable SafeSearch * `\"active\"`: Enable SafeSearch"]
        #[serde(
            rename = "safe",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub safe: ::std::option::Option<String>,
        #[doc = "The search terms entered by the user."]
        #[serde(
            rename = "searchTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_terms: ::std::option::Option<String>,
        #[doc = "Allowed values are `web` or `image`. If unspecified, results are limited to webpages."]
        #[serde(
            rename = "searchType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_type: ::std::option::Option<String>,
        #[doc = "Restricts results to URLs from a specified site."]
        #[serde(
            rename = "siteSearch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub site_search: ::std::option::Option<String>,
        #[doc = "Specifies whether to include or exclude results from the site named in the `sitesearch` parameter. Supported values are: * `i`: include content from site * `e`: exclude content from site"]
        #[serde(
            rename = "siteSearchFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub site_search_filter: ::std::option::Option<String>,
        #[doc = "Specifies that results should be sorted according to the specified expression. For example, sort by date."]
        #[serde(
            rename = "sort",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sort: ::std::option::Option<String>,
        #[doc = "The index of the current set of search results into the total set of results, where the index of the first result is 1."]
        #[serde(
            rename = "startIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_index: ::std::option::Option<i32>,
        #[doc = "The page number of this set of results, where the page length is set by the `count` property."]
        #[serde(
            rename = "startPage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_page: ::std::option::Option<i32>,
        #[doc = "A description of the query."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "Estimated number of total search results. May not be accurate."]
        #[serde(
            rename = "totalResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_results: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for SearchQueriesPreviousPageItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchQueriesPreviousPageItems {
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
    pub struct SearchQueriesRequestItems {
        #[doc = "Number of search results returned in this set."]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub count: ::std::option::Option<i32>,
        #[doc = "Restricts search results to documents originating in a particular country. You may use [Boolean operators](https://developers.google.com/custom-search/docs/json_api_reference#BooleanOrSearch) in the `cr` parameter’s value. Google WebSearch determines the country of a document by analyzing the following: * The top-level domain (TLD) of the document’s URL. * The geographic location of the web server’s IP address. See [Country (cr) Parameter Values](https://developers.google.com/custom-search/docs/json_api_reference#countryCollections) for a list of valid values for this parameter."]
        #[serde(
            rename = "cr",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cr: ::std::option::Option<String>,
        #[doc = "The identifier of an engine created using the Programmable Search Engine [Control Panel](https://programmablesearchengine.google.com/). This is a custom property not defined in the OpenSearch spec. This parameter is **required**."]
        #[serde(
            rename = "cx",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cx: ::std::option::Option<String>,
        #[doc = "Restricts results to URLs based on date. Supported values include: * `d[number]`: requests results from the specified number of past days. * `w[number]`: requests results from the specified number of past weeks. * `m[number]`: requests results from the specified number of past months. * `y[number]`: requests results from the specified number of past years."]
        #[serde(
            rename = "dateRestrict",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub date_restrict: ::std::option::Option<String>,
        #[doc = "Enables or disables the [Simplified and Traditional Chinese Search](https://developers.google.com/custom-search/docs/json_api_reference#chineseSearch) feature. Supported values are: * `0`: enabled (default) * `1`: disabled"]
        #[serde(
            rename = "disableCnTwTranslation",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disable_cn_tw_translation: ::std::option::Option<String>,
        #[doc = "Identifies a phrase that all documents in the search results must contain."]
        #[serde(
            rename = "exactTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exact_terms: ::std::option::Option<String>,
        #[doc = "Identifies a word or phrase that should not appear in any documents in the search results."]
        #[serde(
            rename = "excludeTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub exclude_terms: ::std::option::Option<String>,
        #[doc = "Restricts results to files of a specified extension. Filetypes supported by Google include: * Adobe Portable Document Format (`pdf`) * Adobe PostScript (`ps`) * Lotus 1-2-3 (`wk1`, `wk2`, `wk3`, `wk4`, `wk5`, `wki`, `wks`, `wku`) * Lotus WordPro (`lwp`) * Macwrite (`mw`) * Microsoft Excel (`xls`) * Microsoft PowerPoint (`ppt`) * Microsoft Word (`doc`) * Microsoft Works (`wks`, `wps`, `wdb`) * Microsoft Write (`wri`) * Rich Text Format (`rtf`) * Shockwave Flash (`swf`) * Text (`ans`, `txt`). Additional filetypes may be added in the future. An up-to-date list can always be found in Google’s [file type FAQ](https://support.google.com/webmasters/answer/35287)."]
        #[serde(
            rename = "fileType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub file_type: ::std::option::Option<String>,
        #[doc = "Activates or deactivates the automatic filtering of Google search results. See [Automatic Filtering](https://developers.google.com/custom-search/docs/json_api_reference#automaticFiltering) for more information about Google’s search results filters. Valid values for this parameter are: * `0`: Disabled * `1`: Enabled (default) **Note**: By default, Google applies filtering to all search results to improve the quality of those results."]
        #[serde(
            rename = "filter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub filter: ::std::option::Option<String>,
        #[doc = "Boosts search results whose country of origin matches the parameter value. See [Country Codes](https://developers.google.com/custom-search/docs/json_api_reference#countryCodes) for a list of valid values. Specifying a `gl` parameter value in WebSearch requests should improve the relevance of results. This is particularly true for international customers and, even more specifically, for customers in English-speaking countries other than the United States."]
        #[serde(
            rename = "gl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gl: ::std::option::Option<String>,
        #[doc = "Specifies the Google domain (for example, google.com, google.de, or google.fr) to which the search should be limited."]
        #[serde(
            rename = "googleHost",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub google_host: ::std::option::Option<String>,
        #[doc = "Specifies the ending value for a search range. Use `cse:lowRange` and `cse:highrange` to append an inclusive search range of `lowRange...highRange` to the query."]
        #[serde(
            rename = "highRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub high_range: ::std::option::Option<String>,
        #[doc = "Specifies the interface language (host language) of your user interface. Explicitly setting this parameter improves the performance and the quality of your search results. See the [Interface Languages](https://developers.google.com/custom-search/docs/json_api_reference#wsInterfaceLanguages) section of [Internationalizing Queries and Results Presentation](https://developers.google.com/custom-search/docs/json_api_reference#wsInternationalizing) for more information, and [Supported Interface Languages](https://developers.google.com/custom-search/docs/json_api_reference#interfaceLanguages) for a list of supported languages."]
        #[serde(
            rename = "hl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hl: ::std::option::Option<String>,
        #[doc = "Appends the specified query terms to the query, as if they were combined with a logical `AND` operator."]
        #[serde(
            rename = "hq",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub hq: ::std::option::Option<String>,
        #[doc = "Restricts results to images of a specified color type. Supported values are: * `mono` (black and white) * `gray` (grayscale) * `color` (color)"]
        #[serde(
            rename = "imgColorType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_color_type: ::std::option::Option<String>,
        #[doc = "Restricts results to images with a specific dominant color. Supported values are: * `red` * `orange` * `yellow` * `green` * `teal` * `blue` * `purple` * `pink` * `white` * `gray` * `black` * `brown`"]
        #[serde(
            rename = "imgDominantColor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_dominant_color: ::std::option::Option<String>,
        #[doc = "Restricts results to images of a specified size. Supported values are: * `icon` (small) * `small | medium | large | xlarge` (medium) * `xxlarge` (large) * `huge` (extra-large)"]
        #[serde(
            rename = "imgSize",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_size: ::std::option::Option<String>,
        #[doc = "Restricts results to images of a specified type. Supported values are: * `clipart` (Clip art) * `face` (Face) * `lineart` (Line drawing) * `photo` (Photo) * `animated` (Animated) * `stock` (Stock)"]
        #[serde(
            rename = "imgType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub img_type: ::std::option::Option<String>,
        #[doc = "The character encoding supported for search requests."]
        #[serde(
            rename = "inputEncoding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub input_encoding: ::std::option::Option<String>,
        #[doc = "The language of the search results."]
        #[serde(
            rename = "language",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub language: ::std::option::Option<String>,
        #[doc = "Specifies that all results should contain a link to a specific URL."]
        #[serde(
            rename = "linkSite",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub link_site: ::std::option::Option<String>,
        #[doc = "Specifies the starting value for a search range. Use `cse:lowRange` and `cse:highrange` to append an inclusive search range of `lowRange...highRange` to the query."]
        #[serde(
            rename = "lowRange",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub low_range: ::std::option::Option<String>,
        #[doc = "Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms. You can also use the [Boolean OR](https://developers.google.com/custom-search/docs/json_api_reference#BooleanOrSearch) query term for this type of query."]
        #[serde(
            rename = "orTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub or_terms: ::std::option::Option<String>,
        #[doc = "The character encoding supported for search results."]
        #[serde(
            rename = "outputEncoding",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub output_encoding: ::std::option::Option<String>,
        #[doc = "Specifies that all search results should be pages that are related to the specified URL. The parameter value should be a URL."]
        #[serde(
            rename = "relatedSite",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub related_site: ::std::option::Option<String>,
        #[doc = "Filters based on licensing. Supported values include: * `cc_publicdomain` * `cc_attribute` * `cc_sharealike` * `cc_noncommercial` * `cc_nonderived`"]
        #[serde(
            rename = "rights",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rights: ::std::option::Option<String>,
        #[doc = "Specifies the [SafeSearch level](https://developers.google.com/custom-search/docs/json_api_reference#safeSearchLevels) used for filtering out adult results. This is a custom property not defined in the OpenSearch spec. Valid parameter values are: * `\"off\"`: Disable SafeSearch * `\"active\"`: Enable SafeSearch"]
        #[serde(
            rename = "safe",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub safe: ::std::option::Option<String>,
        #[doc = "The search terms entered by the user."]
        #[serde(
            rename = "searchTerms",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_terms: ::std::option::Option<String>,
        #[doc = "Allowed values are `web` or `image`. If unspecified, results are limited to webpages."]
        #[serde(
            rename = "searchType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_type: ::std::option::Option<String>,
        #[doc = "Restricts results to URLs from a specified site."]
        #[serde(
            rename = "siteSearch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub site_search: ::std::option::Option<String>,
        #[doc = "Specifies whether to include or exclude results from the site named in the `sitesearch` parameter. Supported values are: * `i`: include content from site * `e`: exclude content from site"]
        #[serde(
            rename = "siteSearchFilter",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub site_search_filter: ::std::option::Option<String>,
        #[doc = "Specifies that results should be sorted according to the specified expression. For example, sort by date."]
        #[serde(
            rename = "sort",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sort: ::std::option::Option<String>,
        #[doc = "The index of the current set of search results into the total set of results, where the index of the first result is 1."]
        #[serde(
            rename = "startIndex",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_index: ::std::option::Option<i32>,
        #[doc = "The page number of this set of results, where the page length is set by the `count` property."]
        #[serde(
            rename = "startPage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub start_page: ::std::option::Option<i32>,
        #[doc = "A description of the query."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "Estimated number of total search results. May not be accurate."]
        #[serde(
            rename = "totalResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub total_results: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for SearchQueriesRequestItems {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchQueriesRequestItems {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct SearchSearchInformation {
        #[doc = "The time taken for the server to return search results, formatted according to locale style."]
        #[serde(
            rename = "formattedSearchTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_search_time: ::std::option::Option<String>,
        #[doc = "The total number of search results, formatted according to locale style."]
        #[serde(
            rename = "formattedTotalResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub formatted_total_results: ::std::option::Option<String>,
        #[doc = "The time taken for the server to return search results."]
        #[serde(
            rename = "searchTime",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub search_time: ::std::option::Option<f64>,
        #[doc = "The total number of search results returned by the query."]
        #[serde(
            rename = "totalResults",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_results: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SearchSearchInformation {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchSearchInformation {
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
    pub struct SearchSpelling {
        #[doc = "The corrected query."]
        #[serde(
            rename = "correctedQuery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub corrected_query: ::std::option::Option<String>,
        #[doc = "The corrected query, formatted in HTML."]
        #[serde(
            rename = "htmlCorrectedQuery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub html_corrected_query: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SearchSpelling {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchSpelling {
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
    pub struct SearchUrl {
        #[doc = "The MIME type of the OpenSearch URL template for the Custom Search JSON API."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The actual [OpenSearch template](http://www.opensearch.org/specifications/opensearch/1.1#opensearch_url_template_syntax) for this API."]
        #[serde(
            rename = "template",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub template: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for SearchUrl {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for SearchUrl {
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
    #[doc = "Actions that can be performed on the cse resource"]
    pub fn cse(&self) -> crate::resources::cse::CseActions {
        crate::resources::cse::CseActions {
            reqwest: &self.reqwest,
            auth: self.auth_ref(),
        }
    }
}
pub mod resources {
    pub mod cse {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListImgColorType {
                #[doc = "Color images only."]
                Color,
                #[doc = "Grayscale images only."]
                Gray,
                #[doc = "No image color type specified."]
                ImgColorTypeUndefined,
                #[doc = "Black and white images only."]
                Mono,
                #[doc = "Images with transparent background"]
                Trans,
            }
            impl ListImgColorType {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListImgColorType::Color => "color",
                        ListImgColorType::Gray => "gray",
                        ListImgColorType::ImgColorTypeUndefined => "imgColorTypeUndefined",
                        ListImgColorType::Mono => "mono",
                        ListImgColorType::Trans => "trans",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListImgColorType {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListImgColorType {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListImgColorType, ()> {
                    Ok(match s {
                        "color" => ListImgColorType::Color,
                        "gray" => ListImgColorType::Gray,
                        "imgColorTypeUndefined" => ListImgColorType::ImgColorTypeUndefined,
                        "mono" => ListImgColorType::Mono,
                        "trans" => ListImgColorType::Trans,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListImgColorType {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListImgColorType {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListImgColorType {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "color" => ListImgColorType::Color,
                        "gray" => ListImgColorType::Gray,
                        "imgColorTypeUndefined" => ListImgColorType::ImgColorTypeUndefined,
                        "mono" => ListImgColorType::Mono,
                        "trans" => ListImgColorType::Trans,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListImgColorType {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListImgColorType {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListImgDominantColor {
                #[doc = "Predominantly black images only."]
                Black,
                #[doc = "Predominantly blue images only."]
                Blue,
                #[doc = "Predominantly brown images only."]
                Brown,
                #[doc = "Predominantly gray images only."]
                Gray,
                #[doc = "Predominantly green images only."]
                Green,
                #[doc = "No dominant color specified."]
                ImgDominantColorUndefined,
                #[doc = "Predominantly orange images only."]
                Orange,
                #[doc = "Predominantly pink images only."]
                Pink,
                #[doc = "Predominantly purple images only."]
                Purple,
                #[doc = "Predominantly red images only."]
                Red,
                #[doc = "Predominantly teal images only."]
                Teal,
                #[doc = "Predominantly white images only."]
                White,
                #[doc = "Predominantly yellow images only."]
                Yellow,
            }
            impl ListImgDominantColor {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListImgDominantColor::Black => "black",
                        ListImgDominantColor::Blue => "blue",
                        ListImgDominantColor::Brown => "brown",
                        ListImgDominantColor::Gray => "gray",
                        ListImgDominantColor::Green => "green",
                        ListImgDominantColor::ImgDominantColorUndefined => {
                            "imgDominantColorUndefined"
                        }
                        ListImgDominantColor::Orange => "orange",
                        ListImgDominantColor::Pink => "pink",
                        ListImgDominantColor::Purple => "purple",
                        ListImgDominantColor::Red => "red",
                        ListImgDominantColor::Teal => "teal",
                        ListImgDominantColor::White => "white",
                        ListImgDominantColor::Yellow => "yellow",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListImgDominantColor {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListImgDominantColor {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListImgDominantColor, ()> {
                    Ok(match s {
                        "black" => ListImgDominantColor::Black,
                        "blue" => ListImgDominantColor::Blue,
                        "brown" => ListImgDominantColor::Brown,
                        "gray" => ListImgDominantColor::Gray,
                        "green" => ListImgDominantColor::Green,
                        "imgDominantColorUndefined" => {
                            ListImgDominantColor::ImgDominantColorUndefined
                        }
                        "orange" => ListImgDominantColor::Orange,
                        "pink" => ListImgDominantColor::Pink,
                        "purple" => ListImgDominantColor::Purple,
                        "red" => ListImgDominantColor::Red,
                        "teal" => ListImgDominantColor::Teal,
                        "white" => ListImgDominantColor::White,
                        "yellow" => ListImgDominantColor::Yellow,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListImgDominantColor {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListImgDominantColor {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListImgDominantColor {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "black" => ListImgDominantColor::Black,
                        "blue" => ListImgDominantColor::Blue,
                        "brown" => ListImgDominantColor::Brown,
                        "gray" => ListImgDominantColor::Gray,
                        "green" => ListImgDominantColor::Green,
                        "imgDominantColorUndefined" => {
                            ListImgDominantColor::ImgDominantColorUndefined
                        }
                        "orange" => ListImgDominantColor::Orange,
                        "pink" => ListImgDominantColor::Pink,
                        "purple" => ListImgDominantColor::Purple,
                        "red" => ListImgDominantColor::Red,
                        "teal" => ListImgDominantColor::Teal,
                        "white" => ListImgDominantColor::White,
                        "yellow" => ListImgDominantColor::Yellow,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListImgDominantColor {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListImgDominantColor {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListImgSize {
                #[doc = "Only the largest possible images."]
                Huge,
                #[doc = "Only very small icon-sized images."]
                Icon,
                #[doc = "No image size specified."]
                ImgSizeUndefined,
                #[doc = "Only large images."]
                Large,
                #[doc = "Only medium images."]
                Medium,
                #[doc = "Only small images."]
                Small,
                #[doc = "Only very large images."]
                Xlarge,
                #[doc = "Only extremely large images."]
                Xxlarge,
            }
            impl ListImgSize {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListImgSize::Huge => "HUGE",
                        ListImgSize::Icon => "ICON",
                        ListImgSize::ImgSizeUndefined => "imgSizeUndefined",
                        ListImgSize::Large => "LARGE",
                        ListImgSize::Medium => "MEDIUM",
                        ListImgSize::Small => "SMALL",
                        ListImgSize::Xlarge => "XLARGE",
                        ListImgSize::Xxlarge => "XXLARGE",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListImgSize {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListImgSize {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListImgSize, ()> {
                    Ok(match s {
                        "HUGE" => ListImgSize::Huge,
                        "ICON" => ListImgSize::Icon,
                        "imgSizeUndefined" => ListImgSize::ImgSizeUndefined,
                        "LARGE" => ListImgSize::Large,
                        "MEDIUM" => ListImgSize::Medium,
                        "SMALL" => ListImgSize::Small,
                        "XLARGE" => ListImgSize::Xlarge,
                        "XXLARGE" => ListImgSize::Xxlarge,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListImgSize {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListImgSize {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListImgSize {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "HUGE" => ListImgSize::Huge,
                        "ICON" => ListImgSize::Icon,
                        "imgSizeUndefined" => ListImgSize::ImgSizeUndefined,
                        "LARGE" => ListImgSize::Large,
                        "MEDIUM" => ListImgSize::Medium,
                        "SMALL" => ListImgSize::Small,
                        "XLARGE" => ListImgSize::Xlarge,
                        "XXLARGE" => ListImgSize::Xxlarge,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListImgSize {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListImgSize {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListImgType {
                #[doc = "Animated images only."]
                Animated,
                #[doc = "Clipart-style images only."]
                Clipart,
                #[doc = "Images of faces only."]
                Face,
                #[doc = "No image type specified."]
                ImgTypeUndefined,
                #[doc = "Line art images only."]
                Lineart,
                #[doc = "Photo images only."]
                Photo,
                #[doc = "Stock images only."]
                Stock,
            }
            impl ListImgType {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListImgType::Animated => "animated",
                        ListImgType::Clipart => "clipart",
                        ListImgType::Face => "face",
                        ListImgType::ImgTypeUndefined => "imgTypeUndefined",
                        ListImgType::Lineart => "lineart",
                        ListImgType::Photo => "photo",
                        ListImgType::Stock => "stock",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListImgType {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListImgType {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListImgType, ()> {
                    Ok(match s {
                        "animated" => ListImgType::Animated,
                        "clipart" => ListImgType::Clipart,
                        "face" => ListImgType::Face,
                        "imgTypeUndefined" => ListImgType::ImgTypeUndefined,
                        "lineart" => ListImgType::Lineart,
                        "photo" => ListImgType::Photo,
                        "stock" => ListImgType::Stock,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListImgType {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListImgType {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListImgType {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "animated" => ListImgType::Animated,
                        "clipart" => ListImgType::Clipart,
                        "face" => ListImgType::Face,
                        "imgTypeUndefined" => ListImgType::ImgTypeUndefined,
                        "lineart" => ListImgType::Lineart,
                        "photo" => ListImgType::Photo,
                        "stock" => ListImgType::Stock,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListImgType {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListImgType {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListSafe {
                #[doc = "Turn SafeSearch on."]
                Active,
                #[doc = "Deprecated, equivalent to “active”."]
                High,
                #[doc = "Deprecated, equivalent to “active”."]
                Medium,
                #[doc = "Turn SafeSearch off."]
                Off,
                #[doc = "SafeSearch mode unspecified. (Falls back to engine’s configuration.)"]
                SafeUndefined,
            }
            impl ListSafe {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListSafe::Active => "active",
                        ListSafe::High => "high",
                        ListSafe::Medium => "medium",
                        ListSafe::Off => "off",
                        ListSafe::SafeUndefined => "safeUndefined",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListSafe {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListSafe {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListSafe, ()> {
                    Ok(match s {
                        "active" => ListSafe::Active,
                        "high" => ListSafe::High,
                        "medium" => ListSafe::Medium,
                        "off" => ListSafe::Off,
                        "safeUndefined" => ListSafe::SafeUndefined,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListSafe {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListSafe {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListSafe {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "active" => ListSafe::Active,
                        "high" => ListSafe::High,
                        "medium" => ListSafe::Medium,
                        "off" => ListSafe::Off,
                        "safeUndefined" => ListSafe::SafeUndefined,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListSafe {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListSafe {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListSearchType {
                #[doc = "Image search."]
                Image,
                #[doc = "Search type unspecified (defaults to web search)."]
                SearchTypeUndefined,
            }
            impl ListSearchType {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListSearchType::Image => "image",
                        ListSearchType::SearchTypeUndefined => "searchTypeUndefined",
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListSearchType {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListSearchType {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListSearchType, ()> {
                    Ok(match s {
                        "image" => ListSearchType::Image,
                        "searchTypeUndefined" => ListSearchType::SearchTypeUndefined,
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListSearchType {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListSearchType {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListSearchType {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "image" => ListSearchType::Image,
                        "searchTypeUndefined" => ListSearchType::SearchTypeUndefined,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::google_field_selector::FieldSelector for ListSearchType {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListSearchType {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListSiteSearchFilter {
                #[doc = "Exclude results from the listed sites."]
                E,
                #[doc = "Include only results from the listed sites."]
                I,
                #[doc = "Filter mode unspecified."]
                SiteSearchFilterUndefined,
            }
            impl ListSiteSearchFilter {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListSiteSearchFilter::E => "e",
                        ListSiteSearchFilter::I => "i",
                        ListSiteSearchFilter::SiteSearchFilterUndefined => {
                            "siteSearchFilterUndefined"
                        }
                    }
                }
            }
            impl ::std::convert::AsRef<str> for ListSiteSearchFilter {
                fn as_ref(&self) -> &str {
                    self.as_str()
                }
            }
            impl ::std::str::FromStr for ListSiteSearchFilter {
                type Err = ();
                fn from_str(s: &str) -> ::std::result::Result<ListSiteSearchFilter, ()> {
                    Ok(match s {
                        "e" => ListSiteSearchFilter::E,
                        "i" => ListSiteSearchFilter::I,
                        "siteSearchFilterUndefined" => {
                            ListSiteSearchFilter::SiteSearchFilterUndefined
                        }
                        _ => return Err(()),
                    })
                }
            }
            impl ::std::fmt::Display for ListSiteSearchFilter {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListSiteSearchFilter {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListSiteSearchFilter {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "e" => ListSiteSearchFilter::E,
                        "i" => ListSiteSearchFilter::I,
                        "siteSearchFilterUndefined" => {
                            ListSiteSearchFilter::SiteSearchFilterUndefined
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
            impl ::google_field_selector::FieldSelector for ListSiteSearchFilter {
                fn fields() -> Vec<::google_field_selector::Field> {
                    Vec::new()
                }
            }
            impl ::google_field_selector::ToFieldType for ListSiteSearchFilter {
                fn field_type() -> ::google_field_selector::FieldType {
                    ::google_field_selector::FieldType::Leaf
                }
            }
        }
        pub struct CseActions<'a> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> CseActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Returns metadata about the search performed, metadata about the engine used for the search, and the search results."]
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
                    c_2coff: None,
                    cr: None,
                    cx: None,
                    date_restrict: None,
                    exact_terms: None,
                    exclude_terms: None,
                    file_type: None,
                    filter: None,
                    gl: None,
                    googlehost: None,
                    high_range: None,
                    hl: None,
                    hq: None,
                    img_color_type: None,
                    img_dominant_color: None,
                    img_size: None,
                    img_type: None,
                    link_site: None,
                    low_range: None,
                    lr: None,
                    num: None,
                    or_terms: None,
                    q: None,
                    related_site: None,
                    rights: None,
                    safe: None,
                    search_type: None,
                    site_search: None,
                    site_search_filter: None,
                    sort: None,
                    start: None,
                }
            }
            #[doc = "Actions that can be performed on the siterestrict resource"]
            pub fn siterestrict(&self) -> crate::resources::cse::siterestrict::SiterestrictActions {
                crate::resources::cse::siterestrict::SiterestrictActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        #[doc = "Created via [CseActions::list()](struct.CseActions.html#method.list)"]
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            c_2coff: ::std::option::Option<String>,
            cr: ::std::option::Option<String>,
            cx: ::std::option::Option<String>,
            date_restrict: ::std::option::Option<String>,
            exact_terms: ::std::option::Option<String>,
            exclude_terms: ::std::option::Option<String>,
            file_type: ::std::option::Option<String>,
            filter: ::std::option::Option<String>,
            gl: ::std::option::Option<String>,
            googlehost: ::std::option::Option<String>,
            high_range: ::std::option::Option<String>,
            hl: ::std::option::Option<String>,
            hq: ::std::option::Option<String>,
            img_color_type: ::std::option::Option<crate::resources::cse::params::ListImgColorType>,
            img_dominant_color:
                ::std::option::Option<crate::resources::cse::params::ListImgDominantColor>,
            img_size: ::std::option::Option<crate::resources::cse::params::ListImgSize>,
            img_type: ::std::option::Option<crate::resources::cse::params::ListImgType>,
            link_site: ::std::option::Option<String>,
            low_range: ::std::option::Option<String>,
            lr: ::std::option::Option<String>,
            num: ::std::option::Option<i32>,
            or_terms: ::std::option::Option<String>,
            q: ::std::option::Option<String>,
            related_site: ::std::option::Option<String>,
            rights: ::std::option::Option<String>,
            safe: ::std::option::Option<crate::resources::cse::params::ListSafe>,
            search_type: ::std::option::Option<crate::resources::cse::params::ListSearchType>,
            site_search: ::std::option::Option<String>,
            site_search_filter:
                ::std::option::Option<crate::resources::cse::params::ListSiteSearchFilter>,
            sort: ::std::option::Option<String>,
            start: ::std::option::Option<u32>,
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
            #[doc = "Enables or disables [Simplified and Traditional Chinese Search](https://developers.google.com/custom-search/docs/json_api_reference#chineseSearch). The default value for this parameter is 0 (zero), meaning that the feature is enabled. Supported values are: * `1`: Disabled * `0`: Enabled (default)"]
            pub fn c_2coff(mut self, value: impl Into<String>) -> Self {
                self.c_2coff = Some(value.into());
                self
            }
            #[doc = "Restricts search results to documents originating in a particular country. You may use [Boolean operators](https://developers.google.com/custom-search/docs/json_api_reference#booleanOperators) in the cr parameter’s value. Google Search determines the country of a document by analyzing: * the top-level domain (TLD) of the document’s URL * the geographic location of the Web server’s IP address See the [Country Parameter Values](https://developers.google.com/custom-search/docs/json_api_reference#countryCollections) page for a list of valid values for this parameter."]
            pub fn cr(mut self, value: impl Into<String>) -> Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = "The Programmable Search Engine ID to use for this request."]
            pub fn cx(mut self, value: impl Into<String>) -> Self {
                self.cx = Some(value.into());
                self
            }
            #[doc = "Restricts results to URLs based on date. Supported values include: * `d[number]`: requests results from the specified number of past days. * `w[number]`: requests results from the specified number of past weeks. * `m[number]`: requests results from the specified number of past months. * `y[number]`: requests results from the specified number of past years."]
            pub fn date_restrict(mut self, value: impl Into<String>) -> Self {
                self.date_restrict = Some(value.into());
                self
            }
            #[doc = "Identifies a phrase that all documents in the search results must contain."]
            pub fn exact_terms(mut self, value: impl Into<String>) -> Self {
                self.exact_terms = Some(value.into());
                self
            }
            #[doc = "Identifies a word or phrase that should not appear in any documents in the search results."]
            pub fn exclude_terms(mut self, value: impl Into<String>) -> Self {
                self.exclude_terms = Some(value.into());
                self
            }
            #[doc = "Restricts results to files of a specified extension. A list of file types indexable by Google can be found in Search Console [Help Center](https://support.google.com/webmasters/answer/35287)."]
            pub fn file_type(mut self, value: impl Into<String>) -> Self {
                self.file_type = Some(value.into());
                self
            }
            #[doc = "Controls turning on or off the duplicate content filter. * See [Automatic Filtering](https://developers.google.com/custom-search/docs/json_api_reference#automaticFiltering) for more information about Google’s search results filters. Note that host crowding filtering applies only to multi-site searches. * By default, Google applies filtering to all search results to improve the quality of those results. Acceptable values are: * `0`: Turns off duplicate content filter. * `1`: Turns on duplicate content filter."]
            pub fn filter(mut self, value: impl Into<String>) -> Self {
                self.filter = Some(value.into());
                self
            }
            #[doc = "Geolocation of end user. * The `gl` parameter value is a two-letter country code. The `gl` parameter boosts search results whose country of origin matches the parameter value. See the [Country Codes](https://developers.google.com/custom-search/docs/json_api_reference#countryCodes) page for a list of valid values. * Specifying a `gl` parameter value should lead to more relevant results. This is particularly true for international customers and, even more specifically, for customers in English- speaking countries other than the United States."]
            pub fn gl(mut self, value: impl Into<String>) -> Self {
                self.gl = Some(value.into());
                self
            }
            #[doc = "**Deprecated**. Use the `gl` parameter for a similar effect. The local Google domain (for example, google.com, google.de, or google.fr) to use to perform the search."]
            pub fn googlehost(mut self, value: impl Into<String>) -> Self {
                self.googlehost = Some(value.into());
                self
            }
            #[doc = "Specifies the ending value for a search range. * Use `lowRange` and `highRange` to append an inclusive search range of `lowRange...highRange` to the query."]
            pub fn high_range(mut self, value: impl Into<String>) -> Self {
                self.high_range = Some(value.into());
                self
            }
            #[doc = "Sets the user interface language. * Explicitly setting this parameter improves the performance and the quality of your search results. * See the [Interface Languages](https://developers.google.com/custom-search/docs/json_api_reference#wsInterfaceLanguages) section of [Internationalizing Queries and Results Presentation](https://developers.google.com/custom-search/docs/json_api_reference#wsInternationalizing) for more information, and [Supported Interface Languages](https://developers.google.com/custom-search/docs/json_api_reference#interfaceLanguages) for a list of supported languages."]
            pub fn hl(mut self, value: impl Into<String>) -> Self {
                self.hl = Some(value.into());
                self
            }
            #[doc = "Appends the specified query terms to the query, as if they were combined with a logical AND operator."]
            pub fn hq(mut self, value: impl Into<String>) -> Self {
                self.hq = Some(value.into());
                self
            }
            #[doc = "Returns black and white, grayscale, transparent, or color images. Acceptable values are: * `\"color\"` * `\"gray\"` * `\"mono\"`: black and white * `\"trans\"`: transparent background"]
            pub fn img_color_type(
                mut self,
                value: crate::resources::cse::params::ListImgColorType,
            ) -> Self {
                self.img_color_type = Some(value);
                self
            }
            #[doc = "Returns images of a specific dominant color. Acceptable values are: * `\"black\"` * `\"blue\"` * `\"brown\"` * `\"gray\"` * `\"green\"` * `\"orange\"` * `\"pink\"` * `\"purple\"` * `\"red\"` * `\"teal\"` * `\"white\"` * `\"yellow\"`"]
            pub fn img_dominant_color(
                mut self,
                value: crate::resources::cse::params::ListImgDominantColor,
            ) -> Self {
                self.img_dominant_color = Some(value);
                self
            }
            #[doc = "Returns images of a specified size. Acceptable values are: * `\"huge\"` * `\"icon\"` * `\"large\"` * `\"medium\"` * `\"small\"` * `\"xlarge\"` * `\"xxlarge\"`"]
            pub fn img_size(mut self, value: crate::resources::cse::params::ListImgSize) -> Self {
                self.img_size = Some(value);
                self
            }
            #[doc = "Returns images of a type. Acceptable values are: * `\"clipart\"` * `\"face\"` * `\"lineart\"` * `\"stock\"` * `\"photo\"` * `\"animated\"`"]
            pub fn img_type(mut self, value: crate::resources::cse::params::ListImgType) -> Self {
                self.img_type = Some(value);
                self
            }
            #[doc = "Specifies that all search results should contain a link to a particular URL."]
            pub fn link_site(mut self, value: impl Into<String>) -> Self {
                self.link_site = Some(value.into());
                self
            }
            #[doc = "Specifies the starting value for a search range. Use `lowRange` and `highRange` to append an inclusive search range of `lowRange...highRange` to the query."]
            pub fn low_range(mut self, value: impl Into<String>) -> Self {
                self.low_range = Some(value.into());
                self
            }
            #[doc = "Restricts the search to documents written in a particular language (e.g., `lr=lang_ja`). Acceptable values are: * `\"lang_ar\"`: Arabic * `\"lang_bg\"`: Bulgarian * `\"lang_ca\"`: Catalan * `\"lang_cs\"`: Czech * `\"lang_da\"`: Danish * `\"lang_de\"`: German * `\"lang_el\"`: Greek * `\"lang_en\"`: English * `\"lang_es\"`: Spanish * `\"lang_et\"`: Estonian * `\"lang_fi\"`: Finnish * `\"lang_fr\"`: French * `\"lang_hr\"`: Croatian * `\"lang_hu\"`: Hungarian * `\"lang_id\"`: Indonesian * `\"lang_is\"`: Icelandic * `\"lang_it\"`: Italian * `\"lang_iw\"`: Hebrew * `\"lang_ja\"`: Japanese * `\"lang_ko\"`: Korean * `\"lang_lt\"`: Lithuanian * `\"lang_lv\"`: Latvian * `\"lang_nl\"`: Dutch * `\"lang_no\"`: Norwegian * `\"lang_pl\"`: Polish * `\"lang_pt\"`: Portuguese * `\"lang_ro\"`: Romanian * `\"lang_ru\"`: Russian * `\"lang_sk\"`: Slovak * `\"lang_sl\"`: Slovenian * `\"lang_sr\"`: Serbian * `\"lang_sv\"`: Swedish * `\"lang_tr\"`: Turkish * `\"lang_zh-CN\"`: Chinese (Simplified) * `\"lang_zh-TW\"`: Chinese (Traditional)"]
            pub fn lr(mut self, value: impl Into<String>) -> Self {
                self.lr = Some(value.into());
                self
            }
            #[doc = "Number of search results to return. * Valid values are integers between 1 and 10, inclusive."]
            pub fn num(mut self, value: i32) -> Self {
                self.num = Some(value);
                self
            }
            #[doc = "Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms."]
            pub fn or_terms(mut self, value: impl Into<String>) -> Self {
                self.or_terms = Some(value.into());
                self
            }
            #[doc = "Query"]
            pub fn q(mut self, value: impl Into<String>) -> Self {
                self.q = Some(value.into());
                self
            }
            #[doc = "Specifies that all search results should be pages that are related to the specified URL."]
            pub fn related_site(mut self, value: impl Into<String>) -> Self {
                self.related_site = Some(value.into());
                self
            }
            #[doc = "Filters based on licensing. Supported values include: `cc_publicdomain`, `cc_attribute`, `cc_sharealike`, `cc_noncommercial`, `cc_nonderived` and combinations of these. See [typical combinations](https://wiki.creativecommons.org/wiki/CC_Search_integration)."]
            pub fn rights(mut self, value: impl Into<String>) -> Self {
                self.rights = Some(value.into());
                self
            }
            #[doc = "Search safety level. Acceptable values are: * `\"active\"`: Enables SafeSearch filtering. * `\"off\"`: Disables SafeSearch filtering. (default)"]
            pub fn safe(mut self, value: crate::resources::cse::params::ListSafe) -> Self {
                self.safe = Some(value);
                self
            }
            #[doc = "Specifies the search type: `image`. If unspecified, results are limited to webpages. Acceptable values are: * `\"image\"`: custom image search."]
            pub fn search_type(
                mut self,
                value: crate::resources::cse::params::ListSearchType,
            ) -> Self {
                self.search_type = Some(value);
                self
            }
            #[doc = "Specifies a given site which should always be included or excluded from results (see `siteSearchFilter` parameter, below)."]
            pub fn site_search(mut self, value: impl Into<String>) -> Self {
                self.site_search = Some(value.into());
                self
            }
            #[doc = "Controls whether to include or exclude results from the site named in the `siteSearch` parameter. Acceptable values are: * `\"e\"`: exclude * `\"i\"`: include"]
            pub fn site_search_filter(
                mut self,
                value: crate::resources::cse::params::ListSiteSearchFilter,
            ) -> Self {
                self.site_search_filter = Some(value);
                self
            }
            #[doc = "The sort expression to apply to the results. The sort parameter specifies that the results be sorted according to the specified expression i.e. sort by date. [Example: sort=date](https://developers.google.com/custom-search/docs/structured_search#sort-by-attribute)."]
            pub fn sort(mut self, value: impl Into<String>) -> Self {
                self.sort = Some(value.into());
                self
            }
            #[doc = "The index of the first result to return. The default number of results per page is 10, so `&start=11` would start at the top of the second page of results. **Note**: The JSON API will never return more than 100 results, even if more than 100 documents match the query, so setting the sum of `start + num` to a number greater than 100 will produce an error. Also note that the maximum value for `num` is 10."]
            pub fn start(mut self, value: u32) -> Self {
                self.start = Some(value);
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
            ) -> Result<crate::schemas::Search, crate::Error> {
                self.execute_with_fields(None::<&str>).await
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub async fn execute_with_all_fields(
                self,
            ) -> Result<crate::schemas::Search, crate::Error> {
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
                let mut output = "https://customsearch.googleapis.com/".to_owned();
                output.push_str("customsearch/v1");
                output
            }
            async fn _request(
                &self,
                path: &str,
            ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                req = req.query(&[("c2coff", &self.c_2coff)]);
                req = req.query(&[("cr", &self.cr)]);
                req = req.query(&[("cx", &self.cx)]);
                req = req.query(&[("dateRestrict", &self.date_restrict)]);
                req = req.query(&[("exactTerms", &self.exact_terms)]);
                req = req.query(&[("excludeTerms", &self.exclude_terms)]);
                req = req.query(&[("fileType", &self.file_type)]);
                req = req.query(&[("filter", &self.filter)]);
                req = req.query(&[("gl", &self.gl)]);
                req = req.query(&[("googlehost", &self.googlehost)]);
                req = req.query(&[("highRange", &self.high_range)]);
                req = req.query(&[("hl", &self.hl)]);
                req = req.query(&[("hq", &self.hq)]);
                req = req.query(&[("imgColorType", &self.img_color_type)]);
                req = req.query(&[("imgDominantColor", &self.img_dominant_color)]);
                req = req.query(&[("imgSize", &self.img_size)]);
                req = req.query(&[("imgType", &self.img_type)]);
                req = req.query(&[("linkSite", &self.link_site)]);
                req = req.query(&[("lowRange", &self.low_range)]);
                req = req.query(&[("lr", &self.lr)]);
                req = req.query(&[("num", &self.num)]);
                req = req.query(&[("orTerms", &self.or_terms)]);
                req = req.query(&[("q", &self.q)]);
                req = req.query(&[("relatedSite", &self.related_site)]);
                req = req.query(&[("rights", &self.rights)]);
                req = req.query(&[("safe", &self.safe)]);
                req = req.query(&[("searchType", &self.search_type)]);
                req = req.query(&[("siteSearch", &self.site_search)]);
                req = req.query(&[("siteSearchFilter", &self.site_search_filter)]);
                req = req.query(&[("sort", &self.sort)]);
                req = req.query(&[("start", &self.start)]);
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
        pub mod siterestrict {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListImgColorType {
                    #[doc = "Color images only."]
                    Color,
                    #[doc = "Grayscale images only."]
                    Gray,
                    #[doc = "No image color type specified."]
                    ImgColorTypeUndefined,
                    #[doc = "Black and white images only."]
                    Mono,
                    #[doc = "Images with transparent background"]
                    Trans,
                }
                impl ListImgColorType {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListImgColorType::Color => "color",
                            ListImgColorType::Gray => "gray",
                            ListImgColorType::ImgColorTypeUndefined => "imgColorTypeUndefined",
                            ListImgColorType::Mono => "mono",
                            ListImgColorType::Trans => "trans",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListImgColorType {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListImgColorType {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListImgColorType, ()> {
                        Ok(match s {
                            "color" => ListImgColorType::Color,
                            "gray" => ListImgColorType::Gray,
                            "imgColorTypeUndefined" => ListImgColorType::ImgColorTypeUndefined,
                            "mono" => ListImgColorType::Mono,
                            "trans" => ListImgColorType::Trans,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListImgColorType {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListImgColorType {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListImgColorType {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "color" => ListImgColorType::Color,
                            "gray" => ListImgColorType::Gray,
                            "imgColorTypeUndefined" => ListImgColorType::ImgColorTypeUndefined,
                            "mono" => ListImgColorType::Mono,
                            "trans" => ListImgColorType::Trans,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListImgColorType {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListImgColorType {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListImgDominantColor {
                    #[doc = "Predominantly black images only."]
                    Black,
                    #[doc = "Predominantly blue images only."]
                    Blue,
                    #[doc = "Predominantly brown images only."]
                    Brown,
                    #[doc = "Predominantly gray images only."]
                    Gray,
                    #[doc = "Predominantly green images only."]
                    Green,
                    #[doc = "No dominant color specified."]
                    ImgDominantColorUndefined,
                    #[doc = "Predominantly orange images only."]
                    Orange,
                    #[doc = "Predominantly pink images only."]
                    Pink,
                    #[doc = "Predominantly purple images only."]
                    Purple,
                    #[doc = "Predominantly red images only."]
                    Red,
                    #[doc = "Predominantly teal images only."]
                    Teal,
                    #[doc = "Predominantly white images only."]
                    White,
                    #[doc = "Predominantly yellow images only."]
                    Yellow,
                }
                impl ListImgDominantColor {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListImgDominantColor::Black => "black",
                            ListImgDominantColor::Blue => "blue",
                            ListImgDominantColor::Brown => "brown",
                            ListImgDominantColor::Gray => "gray",
                            ListImgDominantColor::Green => "green",
                            ListImgDominantColor::ImgDominantColorUndefined => {
                                "imgDominantColorUndefined"
                            }
                            ListImgDominantColor::Orange => "orange",
                            ListImgDominantColor::Pink => "pink",
                            ListImgDominantColor::Purple => "purple",
                            ListImgDominantColor::Red => "red",
                            ListImgDominantColor::Teal => "teal",
                            ListImgDominantColor::White => "white",
                            ListImgDominantColor::Yellow => "yellow",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListImgDominantColor {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListImgDominantColor {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListImgDominantColor, ()> {
                        Ok(match s {
                            "black" => ListImgDominantColor::Black,
                            "blue" => ListImgDominantColor::Blue,
                            "brown" => ListImgDominantColor::Brown,
                            "gray" => ListImgDominantColor::Gray,
                            "green" => ListImgDominantColor::Green,
                            "imgDominantColorUndefined" => {
                                ListImgDominantColor::ImgDominantColorUndefined
                            }
                            "orange" => ListImgDominantColor::Orange,
                            "pink" => ListImgDominantColor::Pink,
                            "purple" => ListImgDominantColor::Purple,
                            "red" => ListImgDominantColor::Red,
                            "teal" => ListImgDominantColor::Teal,
                            "white" => ListImgDominantColor::White,
                            "yellow" => ListImgDominantColor::Yellow,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListImgDominantColor {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListImgDominantColor {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListImgDominantColor {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "black" => ListImgDominantColor::Black,
                            "blue" => ListImgDominantColor::Blue,
                            "brown" => ListImgDominantColor::Brown,
                            "gray" => ListImgDominantColor::Gray,
                            "green" => ListImgDominantColor::Green,
                            "imgDominantColorUndefined" => {
                                ListImgDominantColor::ImgDominantColorUndefined
                            }
                            "orange" => ListImgDominantColor::Orange,
                            "pink" => ListImgDominantColor::Pink,
                            "purple" => ListImgDominantColor::Purple,
                            "red" => ListImgDominantColor::Red,
                            "teal" => ListImgDominantColor::Teal,
                            "white" => ListImgDominantColor::White,
                            "yellow" => ListImgDominantColor::Yellow,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListImgDominantColor {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListImgDominantColor {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListImgSize {
                    #[doc = "Only the largest possible images."]
                    Huge,
                    #[doc = "Only very small icon-sized images."]
                    Icon,
                    #[doc = "No image size specified."]
                    ImgSizeUndefined,
                    #[doc = "Only large images."]
                    Large,
                    #[doc = "Only medium images."]
                    Medium,
                    #[doc = "Only small images."]
                    Small,
                    #[doc = "Only very large images."]
                    Xlarge,
                    #[doc = "Only extremely large images."]
                    Xxlarge,
                }
                impl ListImgSize {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListImgSize::Huge => "HUGE",
                            ListImgSize::Icon => "ICON",
                            ListImgSize::ImgSizeUndefined => "imgSizeUndefined",
                            ListImgSize::Large => "LARGE",
                            ListImgSize::Medium => "MEDIUM",
                            ListImgSize::Small => "SMALL",
                            ListImgSize::Xlarge => "XLARGE",
                            ListImgSize::Xxlarge => "XXLARGE",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListImgSize {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListImgSize {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListImgSize, ()> {
                        Ok(match s {
                            "HUGE" => ListImgSize::Huge,
                            "ICON" => ListImgSize::Icon,
                            "imgSizeUndefined" => ListImgSize::ImgSizeUndefined,
                            "LARGE" => ListImgSize::Large,
                            "MEDIUM" => ListImgSize::Medium,
                            "SMALL" => ListImgSize::Small,
                            "XLARGE" => ListImgSize::Xlarge,
                            "XXLARGE" => ListImgSize::Xxlarge,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListImgSize {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListImgSize {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListImgSize {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "HUGE" => ListImgSize::Huge,
                            "ICON" => ListImgSize::Icon,
                            "imgSizeUndefined" => ListImgSize::ImgSizeUndefined,
                            "LARGE" => ListImgSize::Large,
                            "MEDIUM" => ListImgSize::Medium,
                            "SMALL" => ListImgSize::Small,
                            "XLARGE" => ListImgSize::Xlarge,
                            "XXLARGE" => ListImgSize::Xxlarge,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListImgSize {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListImgSize {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListImgType {
                    #[doc = "Animated images only."]
                    Animated,
                    #[doc = "Clipart-style images only."]
                    Clipart,
                    #[doc = "Images of faces only."]
                    Face,
                    #[doc = "No image type specified."]
                    ImgTypeUndefined,
                    #[doc = "Line art images only."]
                    Lineart,
                    #[doc = "Photo images only."]
                    Photo,
                    #[doc = "Stock images only."]
                    Stock,
                }
                impl ListImgType {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListImgType::Animated => "animated",
                            ListImgType::Clipart => "clipart",
                            ListImgType::Face => "face",
                            ListImgType::ImgTypeUndefined => "imgTypeUndefined",
                            ListImgType::Lineart => "lineart",
                            ListImgType::Photo => "photo",
                            ListImgType::Stock => "stock",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListImgType {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListImgType {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListImgType, ()> {
                        Ok(match s {
                            "animated" => ListImgType::Animated,
                            "clipart" => ListImgType::Clipart,
                            "face" => ListImgType::Face,
                            "imgTypeUndefined" => ListImgType::ImgTypeUndefined,
                            "lineart" => ListImgType::Lineart,
                            "photo" => ListImgType::Photo,
                            "stock" => ListImgType::Stock,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListImgType {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListImgType {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListImgType {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "animated" => ListImgType::Animated,
                            "clipart" => ListImgType::Clipart,
                            "face" => ListImgType::Face,
                            "imgTypeUndefined" => ListImgType::ImgTypeUndefined,
                            "lineart" => ListImgType::Lineart,
                            "photo" => ListImgType::Photo,
                            "stock" => ListImgType::Stock,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListImgType {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListImgType {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListSafe {
                    #[doc = "Turn SafeSearch on."]
                    Active,
                    #[doc = "Deprecated, equivalent to “active”."]
                    High,
                    #[doc = "Deprecated, equivalent to “active”."]
                    Medium,
                    #[doc = "Turn SafeSearch off."]
                    Off,
                    #[doc = "SafeSearch mode unspecified. (Falls back to engine’s configuration.)"]
                    SafeUndefined,
                }
                impl ListSafe {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListSafe::Active => "active",
                            ListSafe::High => "high",
                            ListSafe::Medium => "medium",
                            ListSafe::Off => "off",
                            ListSafe::SafeUndefined => "safeUndefined",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListSafe {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListSafe {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListSafe, ()> {
                        Ok(match s {
                            "active" => ListSafe::Active,
                            "high" => ListSafe::High,
                            "medium" => ListSafe::Medium,
                            "off" => ListSafe::Off,
                            "safeUndefined" => ListSafe::SafeUndefined,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListSafe {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListSafe {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListSafe {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "active" => ListSafe::Active,
                            "high" => ListSafe::High,
                            "medium" => ListSafe::Medium,
                            "off" => ListSafe::Off,
                            "safeUndefined" => ListSafe::SafeUndefined,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListSafe {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListSafe {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListSearchType {
                    #[doc = "Image search."]
                    Image,
                    #[doc = "Search type unspecified (defaults to web search)."]
                    SearchTypeUndefined,
                }
                impl ListSearchType {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListSearchType::Image => "image",
                            ListSearchType::SearchTypeUndefined => "searchTypeUndefined",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListSearchType {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListSearchType {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListSearchType, ()> {
                        Ok(match s {
                            "image" => ListSearchType::Image,
                            "searchTypeUndefined" => ListSearchType::SearchTypeUndefined,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListSearchType {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListSearchType {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListSearchType {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "image" => ListSearchType::Image,
                            "searchTypeUndefined" => ListSearchType::SearchTypeUndefined,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListSearchType {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListSearchType {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListSiteSearchFilter {
                    #[doc = "Exclude results from the listed sites."]
                    E,
                    #[doc = "Include only results from the listed sites."]
                    I,
                    #[doc = "Filter mode unspecified."]
                    SiteSearchFilterUndefined,
                }
                impl ListSiteSearchFilter {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListSiteSearchFilter::E => "e",
                            ListSiteSearchFilter::I => "i",
                            ListSiteSearchFilter::SiteSearchFilterUndefined => {
                                "siteSearchFilterUndefined"
                            }
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListSiteSearchFilter {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListSiteSearchFilter {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListSiteSearchFilter, ()> {
                        Ok(match s {
                            "e" => ListSiteSearchFilter::E,
                            "i" => ListSiteSearchFilter::I,
                            "siteSearchFilterUndefined" => {
                                ListSiteSearchFilter::SiteSearchFilterUndefined
                            }
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListSiteSearchFilter {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListSiteSearchFilter {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListSiteSearchFilter {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "e" => ListSiteSearchFilter::E,
                            "i" => ListSiteSearchFilter::I,
                            "siteSearchFilterUndefined" => {
                                ListSiteSearchFilter::SiteSearchFilterUndefined
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
                impl ::google_field_selector::FieldSelector for ListSiteSearchFilter {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListSiteSearchFilter {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct SiterestrictActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> SiterestrictActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Returns metadata about the search performed, metadata about the engine used for the search, and the search results. Uses a small set of url patterns."]
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
                        c_2coff: None,
                        cr: None,
                        cx: None,
                        date_restrict: None,
                        exact_terms: None,
                        exclude_terms: None,
                        file_type: None,
                        filter: None,
                        gl: None,
                        googlehost: None,
                        high_range: None,
                        hl: None,
                        hq: None,
                        img_color_type: None,
                        img_dominant_color: None,
                        img_size: None,
                        img_type: None,
                        link_site: None,
                        low_range: None,
                        lr: None,
                        num: None,
                        or_terms: None,
                        q: None,
                        related_site: None,
                        rights: None,
                        safe: None,
                        search_type: None,
                        site_search: None,
                        site_search_filter: None,
                        sort: None,
                        start: None,
                    }
                }
            }
            #[doc = "Created via [SiterestrictActions::list()](struct.SiterestrictActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                c_2coff: ::std::option::Option<String>,
                cr: ::std::option::Option<String>,
                cx: ::std::option::Option<String>,
                date_restrict: ::std::option::Option<String>,
                exact_terms: ::std::option::Option<String>,
                exclude_terms: ::std::option::Option<String>,
                file_type: ::std::option::Option<String>,
                filter: ::std::option::Option<String>,
                gl: ::std::option::Option<String>,
                googlehost: ::std::option::Option<String>,
                high_range: ::std::option::Option<String>,
                hl: ::std::option::Option<String>,
                hq: ::std::option::Option<String>,
                img_color_type: ::std::option::Option<
                    crate::resources::cse::siterestrict::params::ListImgColorType,
                >,
                img_dominant_color: ::std::option::Option<
                    crate::resources::cse::siterestrict::params::ListImgDominantColor,
                >,
                img_size:
                    ::std::option::Option<crate::resources::cse::siterestrict::params::ListImgSize>,
                img_type:
                    ::std::option::Option<crate::resources::cse::siterestrict::params::ListImgType>,
                link_site: ::std::option::Option<String>,
                low_range: ::std::option::Option<String>,
                lr: ::std::option::Option<String>,
                num: ::std::option::Option<i32>,
                or_terms: ::std::option::Option<String>,
                q: ::std::option::Option<String>,
                related_site: ::std::option::Option<String>,
                rights: ::std::option::Option<String>,
                safe: ::std::option::Option<crate::resources::cse::siterestrict::params::ListSafe>,
                search_type: ::std::option::Option<
                    crate::resources::cse::siterestrict::params::ListSearchType,
                >,
                site_search: ::std::option::Option<String>,
                site_search_filter: ::std::option::Option<
                    crate::resources::cse::siterestrict::params::ListSiteSearchFilter,
                >,
                sort: ::std::option::Option<String>,
                start: ::std::option::Option<u32>,
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
                #[doc = "Enables or disables [Simplified and Traditional Chinese Search](https://developers.google.com/custom-search/docs/json_api_reference#chineseSearch). The default value for this parameter is 0 (zero), meaning that the feature is enabled. Supported values are: * `1`: Disabled * `0`: Enabled (default)"]
                pub fn c_2coff(mut self, value: impl Into<String>) -> Self {
                    self.c_2coff = Some(value.into());
                    self
                }
                #[doc = "Restricts search results to documents originating in a particular country. You may use [Boolean operators](https://developers.google.com/custom-search/docs/json_api_reference#booleanOperators) in the cr parameter’s value. Google Search determines the country of a document by analyzing: * the top-level domain (TLD) of the document’s URL * the geographic location of the Web server’s IP address See the [Country Parameter Values](https://developers.google.com/custom-search/docs/json_api_reference#countryCollections) page for a list of valid values for this parameter."]
                pub fn cr(mut self, value: impl Into<String>) -> Self {
                    self.cr = Some(value.into());
                    self
                }
                #[doc = "The Programmable Search Engine ID to use for this request."]
                pub fn cx(mut self, value: impl Into<String>) -> Self {
                    self.cx = Some(value.into());
                    self
                }
                #[doc = "Restricts results to URLs based on date. Supported values include: * `d[number]`: requests results from the specified number of past days. * `w[number]`: requests results from the specified number of past weeks. * `m[number]`: requests results from the specified number of past months. * `y[number]`: requests results from the specified number of past years."]
                pub fn date_restrict(mut self, value: impl Into<String>) -> Self {
                    self.date_restrict = Some(value.into());
                    self
                }
                #[doc = "Identifies a phrase that all documents in the search results must contain."]
                pub fn exact_terms(mut self, value: impl Into<String>) -> Self {
                    self.exact_terms = Some(value.into());
                    self
                }
                #[doc = "Identifies a word or phrase that should not appear in any documents in the search results."]
                pub fn exclude_terms(mut self, value: impl Into<String>) -> Self {
                    self.exclude_terms = Some(value.into());
                    self
                }
                #[doc = "Restricts results to files of a specified extension. A list of file types indexable by Google can be found in Search Console [Help Center](https://support.google.com/webmasters/answer/35287)."]
                pub fn file_type(mut self, value: impl Into<String>) -> Self {
                    self.file_type = Some(value.into());
                    self
                }
                #[doc = "Controls turning on or off the duplicate content filter. * See [Automatic Filtering](https://developers.google.com/custom-search/docs/json_api_reference#automaticFiltering) for more information about Google’s search results filters. Note that host crowding filtering applies only to multi-site searches. * By default, Google applies filtering to all search results to improve the quality of those results. Acceptable values are: * `0`: Turns off duplicate content filter. * `1`: Turns on duplicate content filter."]
                pub fn filter(mut self, value: impl Into<String>) -> Self {
                    self.filter = Some(value.into());
                    self
                }
                #[doc = "Geolocation of end user. * The `gl` parameter value is a two-letter country code. The `gl` parameter boosts search results whose country of origin matches the parameter value. See the [Country Codes](https://developers.google.com/custom-search/docs/json_api_reference#countryCodes) page for a list of valid values. * Specifying a `gl` parameter value should lead to more relevant results. This is particularly true for international customers and, even more specifically, for customers in English- speaking countries other than the United States."]
                pub fn gl(mut self, value: impl Into<String>) -> Self {
                    self.gl = Some(value.into());
                    self
                }
                #[doc = "**Deprecated**. Use the `gl` parameter for a similar effect. The local Google domain (for example, google.com, google.de, or google.fr) to use to perform the search."]
                pub fn googlehost(mut self, value: impl Into<String>) -> Self {
                    self.googlehost = Some(value.into());
                    self
                }
                #[doc = "Specifies the ending value for a search range. * Use `lowRange` and `highRange` to append an inclusive search range of `lowRange...highRange` to the query."]
                pub fn high_range(mut self, value: impl Into<String>) -> Self {
                    self.high_range = Some(value.into());
                    self
                }
                #[doc = "Sets the user interface language. * Explicitly setting this parameter improves the performance and the quality of your search results. * See the [Interface Languages](https://developers.google.com/custom-search/docs/json_api_reference#wsInterfaceLanguages) section of [Internationalizing Queries and Results Presentation](https://developers.google.com/custom-search/docs/json_api_reference#wsInternationalizing) for more information, and [Supported Interface Languages](https://developers.google.com/custom-search/docs/json_api_reference#interfaceLanguages) for a list of supported languages."]
                pub fn hl(mut self, value: impl Into<String>) -> Self {
                    self.hl = Some(value.into());
                    self
                }
                #[doc = "Appends the specified query terms to the query, as if they were combined with a logical AND operator."]
                pub fn hq(mut self, value: impl Into<String>) -> Self {
                    self.hq = Some(value.into());
                    self
                }
                #[doc = "Returns black and white, grayscale, transparent, or color images. Acceptable values are: * `\"color\"` * `\"gray\"` * `\"mono\"`: black and white * `\"trans\"`: transparent background"]
                pub fn img_color_type(
                    mut self,
                    value: crate::resources::cse::siterestrict::params::ListImgColorType,
                ) -> Self {
                    self.img_color_type = Some(value);
                    self
                }
                #[doc = "Returns images of a specific dominant color. Acceptable values are: * `\"black\"` * `\"blue\"` * `\"brown\"` * `\"gray\"` * `\"green\"` * `\"orange\"` * `\"pink\"` * `\"purple\"` * `\"red\"` * `\"teal\"` * `\"white\"` * `\"yellow\"`"]
                pub fn img_dominant_color(
                    mut self,
                    value: crate::resources::cse::siterestrict::params::ListImgDominantColor,
                ) -> Self {
                    self.img_dominant_color = Some(value);
                    self
                }
                #[doc = "Returns images of a specified size. Acceptable values are: * `\"huge\"` * `\"icon\"` * `\"large\"` * `\"medium\"` * `\"small\"` * `\"xlarge\"` * `\"xxlarge\"`"]
                pub fn img_size(
                    mut self,
                    value: crate::resources::cse::siterestrict::params::ListImgSize,
                ) -> Self {
                    self.img_size = Some(value);
                    self
                }
                #[doc = "Returns images of a type. Acceptable values are: * `\"clipart\"` * `\"face\"` * `\"lineart\"` * `\"stock\"` * `\"photo\"` * `\"animated\"`"]
                pub fn img_type(
                    mut self,
                    value: crate::resources::cse::siterestrict::params::ListImgType,
                ) -> Self {
                    self.img_type = Some(value);
                    self
                }
                #[doc = "Specifies that all search results should contain a link to a particular URL."]
                pub fn link_site(mut self, value: impl Into<String>) -> Self {
                    self.link_site = Some(value.into());
                    self
                }
                #[doc = "Specifies the starting value for a search range. Use `lowRange` and `highRange` to append an inclusive search range of `lowRange...highRange` to the query."]
                pub fn low_range(mut self, value: impl Into<String>) -> Self {
                    self.low_range = Some(value.into());
                    self
                }
                #[doc = "Restricts the search to documents written in a particular language (e.g., `lr=lang_ja`). Acceptable values are: * `\"lang_ar\"`: Arabic * `\"lang_bg\"`: Bulgarian * `\"lang_ca\"`: Catalan * `\"lang_cs\"`: Czech * `\"lang_da\"`: Danish * `\"lang_de\"`: German * `\"lang_el\"`: Greek * `\"lang_en\"`: English * `\"lang_es\"`: Spanish * `\"lang_et\"`: Estonian * `\"lang_fi\"`: Finnish * `\"lang_fr\"`: French * `\"lang_hr\"`: Croatian * `\"lang_hu\"`: Hungarian * `\"lang_id\"`: Indonesian * `\"lang_is\"`: Icelandic * `\"lang_it\"`: Italian * `\"lang_iw\"`: Hebrew * `\"lang_ja\"`: Japanese * `\"lang_ko\"`: Korean * `\"lang_lt\"`: Lithuanian * `\"lang_lv\"`: Latvian * `\"lang_nl\"`: Dutch * `\"lang_no\"`: Norwegian * `\"lang_pl\"`: Polish * `\"lang_pt\"`: Portuguese * `\"lang_ro\"`: Romanian * `\"lang_ru\"`: Russian * `\"lang_sk\"`: Slovak * `\"lang_sl\"`: Slovenian * `\"lang_sr\"`: Serbian * `\"lang_sv\"`: Swedish * `\"lang_tr\"`: Turkish * `\"lang_zh-CN\"`: Chinese (Simplified) * `\"lang_zh-TW\"`: Chinese (Traditional)"]
                pub fn lr(mut self, value: impl Into<String>) -> Self {
                    self.lr = Some(value.into());
                    self
                }
                #[doc = "Number of search results to return. * Valid values are integers between 1 and 10, inclusive."]
                pub fn num(mut self, value: i32) -> Self {
                    self.num = Some(value);
                    self
                }
                #[doc = "Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms."]
                pub fn or_terms(mut self, value: impl Into<String>) -> Self {
                    self.or_terms = Some(value.into());
                    self
                }
                #[doc = "Query"]
                pub fn q(mut self, value: impl Into<String>) -> Self {
                    self.q = Some(value.into());
                    self
                }
                #[doc = "Specifies that all search results should be pages that are related to the specified URL."]
                pub fn related_site(mut self, value: impl Into<String>) -> Self {
                    self.related_site = Some(value.into());
                    self
                }
                #[doc = "Filters based on licensing. Supported values include: `cc_publicdomain`, `cc_attribute`, `cc_sharealike`, `cc_noncommercial`, `cc_nonderived` and combinations of these. See [typical combinations](https://wiki.creativecommons.org/wiki/CC_Search_integration)."]
                pub fn rights(mut self, value: impl Into<String>) -> Self {
                    self.rights = Some(value.into());
                    self
                }
                #[doc = "Search safety level. Acceptable values are: * `\"active\"`: Enables SafeSearch filtering. * `\"off\"`: Disables SafeSearch filtering. (default)"]
                pub fn safe(
                    mut self,
                    value: crate::resources::cse::siterestrict::params::ListSafe,
                ) -> Self {
                    self.safe = Some(value);
                    self
                }
                #[doc = "Specifies the search type: `image`. If unspecified, results are limited to webpages. Acceptable values are: * `\"image\"`: custom image search."]
                pub fn search_type(
                    mut self,
                    value: crate::resources::cse::siterestrict::params::ListSearchType,
                ) -> Self {
                    self.search_type = Some(value);
                    self
                }
                #[doc = "Specifies a given site which should always be included or excluded from results (see `siteSearchFilter` parameter, below)."]
                pub fn site_search(mut self, value: impl Into<String>) -> Self {
                    self.site_search = Some(value.into());
                    self
                }
                #[doc = "Controls whether to include or exclude results from the site named in the `siteSearch` parameter. Acceptable values are: * `\"e\"`: exclude * `\"i\"`: include"]
                pub fn site_search_filter(
                    mut self,
                    value: crate::resources::cse::siterestrict::params::ListSiteSearchFilter,
                ) -> Self {
                    self.site_search_filter = Some(value);
                    self
                }
                #[doc = "The sort expression to apply to the results. The sort parameter specifies that the results be sorted according to the specified expression i.e. sort by date. [Example: sort=date](https://developers.google.com/custom-search/docs/structured_search#sort-by-attribute)."]
                pub fn sort(mut self, value: impl Into<String>) -> Self {
                    self.sort = Some(value.into());
                    self
                }
                #[doc = "The index of the first result to return. The default number of results per page is 10, so `&start=11` would start at the top of the second page of results. **Note**: The JSON API will never return more than 100 results, even if more than 100 documents match the query, so setting the sum of `start + num` to a number greater than 100 will produce an error. Also note that the maximum value for `num` is 10."]
                pub fn start(mut self, value: u32) -> Self {
                    self.start = Some(value);
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
                ) -> Result<crate::schemas::Search, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Search, crate::Error> {
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
                    let mut output = "https://customsearch.googleapis.com/".to_owned();
                    output.push_str("customsearch/v1/siterestrict");
                    output
                }
                async fn _request(
                    &self,
                    path: &str,
                ) -> Result<::reqwest::RequestBuilder, crate::Error> {
                    let mut req = self.reqwest.request(::reqwest::Method::GET, path);
                    req = req.query(&[("c2coff", &self.c_2coff)]);
                    req = req.query(&[("cr", &self.cr)]);
                    req = req.query(&[("cx", &self.cx)]);
                    req = req.query(&[("dateRestrict", &self.date_restrict)]);
                    req = req.query(&[("exactTerms", &self.exact_terms)]);
                    req = req.query(&[("excludeTerms", &self.exclude_terms)]);
                    req = req.query(&[("fileType", &self.file_type)]);
                    req = req.query(&[("filter", &self.filter)]);
                    req = req.query(&[("gl", &self.gl)]);
                    req = req.query(&[("googlehost", &self.googlehost)]);
                    req = req.query(&[("highRange", &self.high_range)]);
                    req = req.query(&[("hl", &self.hl)]);
                    req = req.query(&[("hq", &self.hq)]);
                    req = req.query(&[("imgColorType", &self.img_color_type)]);
                    req = req.query(&[("imgDominantColor", &self.img_dominant_color)]);
                    req = req.query(&[("imgSize", &self.img_size)]);
                    req = req.query(&[("imgType", &self.img_type)]);
                    req = req.query(&[("linkSite", &self.link_site)]);
                    req = req.query(&[("lowRange", &self.low_range)]);
                    req = req.query(&[("lr", &self.lr)]);
                    req = req.query(&[("num", &self.num)]);
                    req = req.query(&[("orTerms", &self.or_terms)]);
                    req = req.query(&[("q", &self.q)]);
                    req = req.query(&[("relatedSite", &self.related_site)]);
                    req = req.query(&[("rights", &self.rights)]);
                    req = req.query(&[("safe", &self.safe)]);
                    req = req.query(&[("searchType", &self.search_type)]);
                    req = req.query(&[("siteSearch", &self.site_search)]);
                    req = req.query(&[("siteSearchFilter", &self.site_search_filter)]);
                    req = req.query(&[("sort", &self.sort)]);
                    req = req.query(&[("start", &self.start)]);
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
