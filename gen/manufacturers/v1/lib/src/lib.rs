#![allow(rustdoc::bare_urls)]
#![doc = "# Resources and Methods\n* [accounts](resources/accounts/struct.AccountsActions.html)\n  * [products](resources/accounts/products/struct.ProductsActions.html)\n    * [*delete*](resources/accounts/products/struct.DeleteRequestBuilder.html), [*get*](resources/accounts/products/struct.GetRequestBuilder.html), [*list*](resources/accounts/products/struct.ListRequestBuilder.html), [*update*](resources/accounts/products/struct.UpdateRequestBuilder.html)\n"]
pub mod scopes {
    #[doc = "Manage your product listings for Google Manufacturer Center\n\n`https://www.googleapis.com/auth/manufacturercenter`"]
    pub const MANUFACTURERCENTER: &str = "https://www.googleapis.com/auth/manufacturercenter";
}
pub mod schemas {
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Attributes {
        #[doc = "The additional images of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#addlimage."]
        #[serde(
            rename = "additionalImageLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub additional_image_link: ::std::option::Option<Vec<crate::schemas::Image>>,
        #[doc = "The target age group of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#agegroup."]
        #[serde(
            rename = "ageGroup",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub age_group: ::std::option::Option<String>,
        #[doc = "The brand name of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#brand."]
        #[serde(
            rename = "brand",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub brand: ::std::option::Option<String>,
        #[doc = "The capacity of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#capacity."]
        #[serde(
            rename = "capacity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub capacity: ::std::option::Option<crate::schemas::Capacity>,
        #[doc = "The color of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#color."]
        #[serde(
            rename = "color",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub color: ::std::option::Option<String>,
        #[doc = "The count of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#count."]
        #[serde(
            rename = "count",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub count: ::std::option::Option<crate::schemas::Count>,
        #[doc = "The description of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#description."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The disclosure date of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#disclosure."]
        #[serde(
            rename = "disclosureDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub disclosure_date: ::std::option::Option<String>,
        #[doc = "A list of excluded destinations such as “ClientExport”, “ClientShoppingCatalog” or “PartnerShoppingCatalog”. For more information, see https://support.google.com/manufacturers/answer/7443550"]
        #[serde(
            rename = "excludedDestination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub excluded_destination: ::std::option::Option<Vec<String>>,
        #[doc = "The rich format description of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#featuredesc."]
        #[serde(
            rename = "featureDescription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub feature_description: ::std::option::Option<Vec<crate::schemas::FeatureDescription>>,
        #[doc = "The flavor of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#flavor."]
        #[serde(
            rename = "flavor",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub flavor: ::std::option::Option<String>,
        #[doc = "The format of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#format."]
        #[serde(
            rename = "format",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub format: ::std::option::Option<String>,
        #[doc = "The target gender of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#gender."]
        #[serde(
            rename = "gender",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gender: ::std::option::Option<String>,
        #[doc = "Grocery Attributes. See more at https://support.google.com/manufacturers/answer/12098458#grocery."]
        #[serde(
            rename = "grocery",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub grocery: ::std::option::Option<crate::schemas::Grocery>,
        #[doc = "The Global Trade Item Number (GTIN) of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#gtin."]
        #[serde(
            rename = "gtin",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub gtin: ::std::option::Option<Vec<String>>,
        #[doc = "The image of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#image."]
        #[serde(
            rename = "imageLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_link: ::std::option::Option<crate::schemas::Image>,
        #[doc = "A list of included destinations such as “ClientExport”, “ClientShoppingCatalog” or “PartnerShoppingCatalog”. For more information, see https://support.google.com/manufacturers/answer/7443550"]
        #[serde(
            rename = "includedDestination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub included_destination: ::std::option::Option<Vec<String>>,
        #[doc = "The item group id of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#itemgroupid."]
        #[serde(
            rename = "itemGroupId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub item_group_id: ::std::option::Option<String>,
        #[doc = "The material of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#material."]
        #[serde(
            rename = "material",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub material: ::std::option::Option<String>,
        #[doc = "The Manufacturer Part Number (MPN) of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#mpn."]
        #[serde(
            rename = "mpn",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub mpn: ::std::option::Option<String>,
        #[doc = "Nutrition Attributes. See more at https://support.google.com/manufacturers/answer/12098458#food-servings."]
        #[serde(
            rename = "nutrition",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub nutrition: ::std::option::Option<crate::schemas::Nutrition>,
        #[doc = "The pattern of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#pattern."]
        #[serde(
            rename = "pattern",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub pattern: ::std::option::Option<String>,
        #[doc = "The details of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#productdetail."]
        #[serde(
            rename = "productDetail",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub product_detail: ::std::option::Option<Vec<crate::schemas::ProductDetail>>,
        #[doc = "The product highlights. For more information, see https://support.google.com/manufacturers/answer/10066942"]
        #[serde(
            rename = "productHighlight",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub product_highlight: ::std::option::Option<Vec<String>>,
        #[doc = "The name of the group of products related to the product. For more information, see https://support.google.com/manufacturers/answer/6124116#productline."]
        #[serde(
            rename = "productLine",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub product_line: ::std::option::Option<String>,
        #[doc = "The canonical name of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#productname."]
        #[serde(
            rename = "productName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub product_name: ::std::option::Option<String>,
        #[doc = "The URL of the detail page of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#productpage."]
        #[serde(
            rename = "productPageUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub product_page_url: ::std::option::Option<String>,
        #[doc = "The type or category of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#producttype."]
        #[serde(
            rename = "productType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub product_type: ::std::option::Option<Vec<String>>,
        #[doc = "The release date of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#release."]
        #[serde(
            rename = "releaseDate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub release_date: ::std::option::Option<String>,
        #[doc = "Rich product content. For more information, see https://support.google.com/manufacturers/answer/9389865"]
        #[serde(
            rename = "richProductContent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub rich_product_content: ::std::option::Option<Vec<String>>,
        #[doc = "The scent of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#scent."]
        #[serde(
            rename = "scent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub scent: ::std::option::Option<String>,
        #[doc = "The size of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#size."]
        #[serde(
            rename = "size",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub size: ::std::option::Option<String>,
        #[doc = "The size system of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#sizesystem."]
        #[serde(
            rename = "sizeSystem",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub size_system: ::std::option::Option<String>,
        #[doc = "The size type of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#sizetype."]
        #[serde(
            rename = "sizeType",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub size_type: ::std::option::Option<Vec<String>>,
        #[doc = "The suggested retail price (MSRP) of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#price."]
        #[serde(
            rename = "suggestedRetailPrice",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub suggested_retail_price: ::std::option::Option<crate::schemas::Price>,
        #[doc = "The target client id. Should only be used in the accounts of the data partners. For more information, see https://support.google.com/manufacturers/answer/10857344"]
        #[serde(
            rename = "targetClientId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_client_id: ::std::option::Option<String>,
        #[doc = "The theme of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#theme."]
        #[serde(
            rename = "theme",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub theme: ::std::option::Option<String>,
        #[doc = "The title of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#title."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
        #[doc = "The videos of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#video."]
        #[serde(
            rename = "videoLink",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub video_link: ::std::option::Option<Vec<String>>,
    }
    impl ::google_field_selector::FieldSelector for Attributes {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Attributes {
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
    pub struct Capacity {
        #[doc = "The unit of the capacity, i.e., MB, GB, or TB."]
        #[serde(
            rename = "unit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unit: ::std::option::Option<String>,
        #[doc = "The numeric value of the capacity."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub value: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for Capacity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Capacity {
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
    pub struct Count {
        #[doc = "The unit in which these products are counted."]
        #[serde(
            rename = "unit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unit: ::std::option::Option<String>,
        #[doc = "The numeric value of the number of products in a package."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        #[serde(with = "crate::parsed_string")]
        pub value: ::std::option::Option<i64>,
    }
    impl ::google_field_selector::FieldSelector for Count {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Count {
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
    pub struct DestinationStatus {
        #[doc = "The name of the destination."]
        #[serde(
            rename = "destination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination: ::std::option::Option<String>,
        #[doc = "The status of the destination."]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::DestinationStatusStatus>,
    }
    impl ::google_field_selector::FieldSelector for DestinationStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DestinationStatus {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DestinationStatusStatus {
        #[doc = "The product is used for this destination."]
        Active,
        #[doc = "The product is disapproved. Please look at the issues."]
        Disapproved,
        #[doc = "The decision is still pending."]
        Pending,
        #[doc = "Unspecified status, never used."]
        Unknown,
    }
    impl DestinationStatusStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                DestinationStatusStatus::Active => "ACTIVE",
                DestinationStatusStatus::Disapproved => "DISAPPROVED",
                DestinationStatusStatus::Pending => "PENDING",
                DestinationStatusStatus::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::convert::AsRef<str> for DestinationStatusStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for DestinationStatusStatus {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<DestinationStatusStatus, ()> {
            Ok(match s {
                "ACTIVE" => DestinationStatusStatus::Active,
                "DISAPPROVED" => DestinationStatusStatus::Disapproved,
                "PENDING" => DestinationStatusStatus::Pending,
                "UNKNOWN" => DestinationStatusStatus::Unknown,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for DestinationStatusStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DestinationStatusStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DestinationStatusStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTIVE" => DestinationStatusStatus::Active,
                "DISAPPROVED" => DestinationStatusStatus::Disapproved,
                "PENDING" => DestinationStatusStatus::Pending,
                "UNKNOWN" => DestinationStatusStatus::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for DestinationStatusStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for DestinationStatusStatus {
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
    pub struct FeatureDescription {
        #[doc = "A short description of the feature."]
        #[serde(
            rename = "headline",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub headline: ::std::option::Option<String>,
        #[doc = "An optional image describing the feature."]
        #[serde(
            rename = "image",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image: ::std::option::Option<crate::schemas::Image>,
        #[doc = "A detailed description of the feature."]
        #[serde(
            rename = "text",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub text: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for FeatureDescription {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FeatureDescription {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct FloatUnit {
        #[doc = "amount."]
        #[serde(
            rename = "amount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub amount: ::std::option::Option<f64>,
        #[doc = "unit."]
        #[serde(
            rename = "unit",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub unit: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for FloatUnit {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for FloatUnit {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Grocery {
        #[doc = "Active ingredients."]
        #[serde(
            rename = "activeIngredients",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub active_ingredients: ::std::option::Option<String>,
        #[doc = "Alcohol by volume."]
        #[serde(
            rename = "alcoholByVolume",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub alcohol_by_volume: ::std::option::Option<f64>,
        #[doc = "Allergens."]
        #[serde(
            rename = "allergens",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub allergens: ::std::option::Option<String>,
        #[doc = "Derived nutrition claim."]
        #[serde(
            rename = "derivedNutritionClaim",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub derived_nutrition_claim: ::std::option::Option<Vec<String>>,
        #[doc = "Directions."]
        #[serde(
            rename = "directions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub directions: ::std::option::Option<String>,
        #[doc = "Indications."]
        #[serde(
            rename = "indications",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub indications: ::std::option::Option<String>,
        #[doc = "Ingredients."]
        #[serde(
            rename = "ingredients",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub ingredients: ::std::option::Option<String>,
        #[doc = "Nutrition claim."]
        #[serde(
            rename = "nutritionClaim",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub nutrition_claim: ::std::option::Option<Vec<String>>,
        #[doc = "Storage instructions."]
        #[serde(
            rename = "storageInstructions",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub storage_instructions: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Grocery {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Grocery {
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
    pub struct Image {
        #[doc = "The URL of the image. For crawled images, this is the provided URL. For uploaded images, this is a serving URL from Google if the image has been processed successfully."]
        #[serde(
            rename = "imageUrl",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub image_url: ::std::option::Option<String>,
        #[doc = "The type of the image, i.e., crawled or uploaded. @OutputOnly"]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<crate::schemas::ImageType>,
        #[doc = "The status of the image. @OutputOnly"]
        #[serde(
            rename = "status",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub status: ::std::option::Option<crate::schemas::ImageStatus>,
    }
    impl ::google_field_selector::FieldSelector for Image {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Image {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ImageType {
        #[doc = "The image was crawled from a provided URL."]
        Crawled,
        #[doc = "Type is unspecified. Should not be used."]
        TypeUnspecified,
        #[doc = "The image was uploaded."]
        Uploaded,
    }
    impl ImageType {
        pub fn as_str(self) -> &'static str {
            match self {
                ImageType::Crawled => "CRAWLED",
                ImageType::TypeUnspecified => "TYPE_UNSPECIFIED",
                ImageType::Uploaded => "UPLOADED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ImageType {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ImageType {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ImageType, ()> {
            Ok(match s {
                "CRAWLED" => ImageType::Crawled,
                "TYPE_UNSPECIFIED" => ImageType::TypeUnspecified,
                "UPLOADED" => ImageType::Uploaded,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ImageType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ImageType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ImageType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CRAWLED" => ImageType::Crawled,
                "TYPE_UNSPECIFIED" => ImageType::TypeUnspecified,
                "UPLOADED" => ImageType::Uploaded,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ImageType {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImageType {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ImageStatus {
        #[doc = "There was an error while crawling the image."]
        CrawlError,
        #[doc = "The image was manually overridden and will not be crawled."]
        CrawlSkipped,
        #[doc = "The image cannot be decoded."]
        DecodingError,
        #[doc = "The image crawl was postponed to avoid overloading the host."]
        Hostloaded,
        #[doc = "The image URL returned a “404 Not Found” error."]
        Http404,
        #[doc = "The image was processed and it meets the requirements."]
        Ok,
        #[doc = "The image crawl is still pending."]
        PendingCrawl,
        #[doc = "The image was uploaded and is being processed."]
        PendingProcessing,
        #[doc = "The image cannot be processed."]
        ProcessingError,
        #[doc = "The image URL is protected by robots.txt file and cannot be crawled."]
        Roboted,
        #[doc = "The image status is unspecified. Should not be used."]
        StatusUnspecified,
        #[doc = "The image is too big."]
        TooBig,
        #[doc = "The image URL is protected by X-Robots-Tag and cannot be crawled."]
        Xroboted,
    }
    impl ImageStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                ImageStatus::CrawlError => "CRAWL_ERROR",
                ImageStatus::CrawlSkipped => "CRAWL_SKIPPED",
                ImageStatus::DecodingError => "DECODING_ERROR",
                ImageStatus::Hostloaded => "HOSTLOADED",
                ImageStatus::Http404 => "HTTP_404",
                ImageStatus::Ok => "OK",
                ImageStatus::PendingCrawl => "PENDING_CRAWL",
                ImageStatus::PendingProcessing => "PENDING_PROCESSING",
                ImageStatus::ProcessingError => "PROCESSING_ERROR",
                ImageStatus::Roboted => "ROBOTED",
                ImageStatus::StatusUnspecified => "STATUS_UNSPECIFIED",
                ImageStatus::TooBig => "TOO_BIG",
                ImageStatus::Xroboted => "XROBOTED",
            }
        }
    }
    impl ::std::convert::AsRef<str> for ImageStatus {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for ImageStatus {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<ImageStatus, ()> {
            Ok(match s {
                "CRAWL_ERROR" => ImageStatus::CrawlError,
                "CRAWL_SKIPPED" => ImageStatus::CrawlSkipped,
                "DECODING_ERROR" => ImageStatus::DecodingError,
                "HOSTLOADED" => ImageStatus::Hostloaded,
                "HTTP_404" => ImageStatus::Http404,
                "OK" => ImageStatus::Ok,
                "PENDING_CRAWL" => ImageStatus::PendingCrawl,
                "PENDING_PROCESSING" => ImageStatus::PendingProcessing,
                "PROCESSING_ERROR" => ImageStatus::ProcessingError,
                "ROBOTED" => ImageStatus::Roboted,
                "STATUS_UNSPECIFIED" => ImageStatus::StatusUnspecified,
                "TOO_BIG" => ImageStatus::TooBig,
                "XROBOTED" => ImageStatus::Xroboted,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for ImageStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ImageStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ImageStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CRAWL_ERROR" => ImageStatus::CrawlError,
                "CRAWL_SKIPPED" => ImageStatus::CrawlSkipped,
                "DECODING_ERROR" => ImageStatus::DecodingError,
                "HOSTLOADED" => ImageStatus::Hostloaded,
                "HTTP_404" => ImageStatus::Http404,
                "OK" => ImageStatus::Ok,
                "PENDING_CRAWL" => ImageStatus::PendingCrawl,
                "PENDING_PROCESSING" => ImageStatus::PendingProcessing,
                "PROCESSING_ERROR" => ImageStatus::ProcessingError,
                "ROBOTED" => ImageStatus::Roboted,
                "STATUS_UNSPECIFIED" => ImageStatus::StatusUnspecified,
                "TOO_BIG" => ImageStatus::TooBig,
                "XROBOTED" => ImageStatus::Xroboted,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for ImageStatus {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ImageStatus {
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
    pub struct Issue {
        #[doc = "If present, the attribute that triggered the issue. For more information about attributes, see https://support.google.com/manufacturers/answer/6124116."]
        #[serde(
            rename = "attribute",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attribute: ::std::option::Option<String>,
        #[doc = "Longer description of the issue focused on how to resolve it."]
        #[serde(
            rename = "description",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub description: ::std::option::Option<String>,
        #[doc = "The destination this issue applies to."]
        #[serde(
            rename = "destination",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination: ::std::option::Option<String>,
        #[doc = "The server-generated type of the issue, for example, “INCORRECT_TEXT_FORMATTING”, “IMAGE_NOT_SERVEABLE”, etc."]
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub r#type: ::std::option::Option<String>,
        #[doc = "What needs to happen to resolve the issue."]
        #[serde(
            rename = "resolution",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub resolution: ::std::option::Option<crate::schemas::IssueResolution>,
        #[doc = "The severity of the issue."]
        #[serde(
            rename = "severity",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub severity: ::std::option::Option<crate::schemas::IssueSeverity>,
        #[doc = "The timestamp when this issue appeared."]
        #[serde(
            rename = "timestamp",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub timestamp: ::std::option::Option<String>,
        #[doc = "Short title describing the nature of the issue."]
        #[serde(
            rename = "title",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub title: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Issue {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Issue {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum IssueResolution {
        #[doc = "The issue will be resolved automatically (for example image crawl or Google review). No action is required now. Resolution might lead to another issue (for example if crawl fails)."]
        PendingProcessing,
        #[doc = "Unspecified resolution, never used."]
        ResolutionUnspecified,
        #[doc = "The user who provided the data must act in order to resolve the issue (for example by correcting some data)."]
        UserAction,
    }
    impl IssueResolution {
        pub fn as_str(self) -> &'static str {
            match self {
                IssueResolution::PendingProcessing => "PENDING_PROCESSING",
                IssueResolution::ResolutionUnspecified => "RESOLUTION_UNSPECIFIED",
                IssueResolution::UserAction => "USER_ACTION",
            }
        }
    }
    impl ::std::convert::AsRef<str> for IssueResolution {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for IssueResolution {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<IssueResolution, ()> {
            Ok(match s {
                "PENDING_PROCESSING" => IssueResolution::PendingProcessing,
                "RESOLUTION_UNSPECIFIED" => IssueResolution::ResolutionUnspecified,
                "USER_ACTION" => IssueResolution::UserAction,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for IssueResolution {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for IssueResolution {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for IssueResolution {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PENDING_PROCESSING" => IssueResolution::PendingProcessing,
                "RESOLUTION_UNSPECIFIED" => IssueResolution::ResolutionUnspecified,
                "USER_ACTION" => IssueResolution::UserAction,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for IssueResolution {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IssueResolution {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum IssueSeverity {
        #[doc = "Error severity. The issue prevents the usage of the whole item."]
        Error,
        #[doc = "Info severity. The issue is one that doesn’t require immediate attention. It is, for example, used to communicate which attributes are still pending review."]
        Info,
        #[doc = "Unspecified severity, never used."]
        SeverityUnspecified,
        #[doc = "Warning severity. The issue is either one that prevents the usage of the attribute that triggered it or one that will soon prevent the usage of the whole item."]
        Warning,
    }
    impl IssueSeverity {
        pub fn as_str(self) -> &'static str {
            match self {
                IssueSeverity::Error => "ERROR",
                IssueSeverity::Info => "INFO",
                IssueSeverity::SeverityUnspecified => "SEVERITY_UNSPECIFIED",
                IssueSeverity::Warning => "WARNING",
            }
        }
    }
    impl ::std::convert::AsRef<str> for IssueSeverity {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }
    impl ::std::str::FromStr for IssueSeverity {
        type Err = ();
        fn from_str(s: &str) -> ::std::result::Result<IssueSeverity, ()> {
            Ok(match s {
                "ERROR" => IssueSeverity::Error,
                "INFO" => IssueSeverity::Info,
                "SEVERITY_UNSPECIFIED" => IssueSeverity::SeverityUnspecified,
                "WARNING" => IssueSeverity::Warning,
                _ => return Err(()),
            })
        }
    }
    impl ::std::fmt::Display for IssueSeverity {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for IssueSeverity {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for IssueSeverity {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ERROR" => IssueSeverity::Error,
                "INFO" => IssueSeverity::Info,
                "SEVERITY_UNSPECIFIED" => IssueSeverity::SeverityUnspecified,
                "WARNING" => IssueSeverity::Warning,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::google_field_selector::FieldSelector for IssueSeverity {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for IssueSeverity {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListProductsResponse {
        #[doc = "The token for the retrieval of the next page of product statuses."]
        #[serde(
            rename = "nextPageToken",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "List of the products."]
        #[serde(
            rename = "products",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub products: ::std::option::Option<Vec<crate::schemas::Product>>,
    }
    impl ::google_field_selector::FieldSelector for ListProductsResponse {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ListProductsResponse {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    impl crate::GetNextPageToken<String> for ListProductsResponse {
        fn next_page_token(&self) -> ::std::option::Option<String> {
            self.next_page_token.to_owned()
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Nutrition {
        #[doc = "Added sugars."]
        #[serde(
            rename = "addedSugars",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub added_sugars: ::std::option::Option<crate::schemas::FloatUnit>,
        #[doc = "Added sugars daily percentage."]
        #[serde(
            rename = "addedSugarsDailyPercentage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub added_sugars_daily_percentage: ::std::option::Option<f64>,
        #[doc = "Calcium."]
        #[serde(
            rename = "calcium",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub calcium: ::std::option::Option<crate::schemas::FloatUnit>,
        #[doc = "Calcium daily percentage."]
        #[serde(
            rename = "calciumDailyPercentage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub calcium_daily_percentage: ::std::option::Option<f64>,
        #[doc = "Cholesterol."]
        #[serde(
            rename = "cholesterol",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cholesterol: ::std::option::Option<crate::schemas::FloatUnit>,
        #[doc = "Cholesterol daily percentage."]
        #[serde(
            rename = "cholesterolDailyPercentage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub cholesterol_daily_percentage: ::std::option::Option<f64>,
        #[doc = "Dietary fiber."]
        #[serde(
            rename = "dietaryFiber",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dietary_fiber: ::std::option::Option<crate::schemas::FloatUnit>,
        #[doc = "Dietary fiber daily percentage."]
        #[serde(
            rename = "dietaryFiberDailyPercentage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub dietary_fiber_daily_percentage: ::std::option::Option<f64>,
        #[doc = "Mandatory Nutrition Facts. Energy."]
        #[serde(
            rename = "energy",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub energy: ::std::option::Option<crate::schemas::FloatUnit>,
        #[doc = "Energy from fat."]
        #[serde(
            rename = "energyFromFat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub energy_from_fat: ::std::option::Option<crate::schemas::FloatUnit>,
        #[doc = "Folate daily percentage."]
        #[serde(
            rename = "folateDailyPercentage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub folate_daily_percentage: ::std::option::Option<f64>,
        #[doc = "Folate folic acid."]
        #[serde(
            rename = "folateFolicAcid",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub folate_folic_acid: ::std::option::Option<crate::schemas::FloatUnit>,
        #[doc = "Folate mcg DFE."]
        #[serde(
            rename = "folateMcgDfe",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub folate_mcg_dfe: ::std::option::Option<f64>,
        #[doc = "Iron."]
        #[serde(
            rename = "iron",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub iron: ::std::option::Option<crate::schemas::FloatUnit>,
        #[doc = "Iron daily percentage."]
        #[serde(
            rename = "ironDailyPercentage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub iron_daily_percentage: ::std::option::Option<f64>,
        #[doc = "Monounsaturated fat."]
        #[serde(
            rename = "monounsaturatedFat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub monounsaturated_fat: ::std::option::Option<crate::schemas::FloatUnit>,
        #[doc = "Nutrition fact measure."]
        #[serde(
            rename = "nutritionFactMeasure",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub nutrition_fact_measure: ::std::option::Option<String>,
        #[doc = "Polyols."]
        #[serde(
            rename = "polyols",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub polyols: ::std::option::Option<crate::schemas::FloatUnit>,
        #[doc = "Polyunsaturated fat."]
        #[serde(
            rename = "polyunsaturatedFat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub polyunsaturated_fat: ::std::option::Option<crate::schemas::FloatUnit>,
        #[doc = "Potassium."]
        #[serde(
            rename = "potassium",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub potassium: ::std::option::Option<crate::schemas::FloatUnit>,
        #[doc = "Potassium daily percentage."]
        #[serde(
            rename = "potassiumDailyPercentage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub potassium_daily_percentage: ::std::option::Option<f64>,
        #[doc = "Prepared size description."]
        #[serde(
            rename = "preparedSizeDescription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub prepared_size_description: ::std::option::Option<String>,
        #[doc = "Protein."]
        #[serde(
            rename = "protein",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protein: ::std::option::Option<crate::schemas::FloatUnit>,
        #[doc = "Protein daily percentage."]
        #[serde(
            rename = "proteinDailyPercentage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub protein_daily_percentage: ::std::option::Option<f64>,
        #[doc = "Saturated fat."]
        #[serde(
            rename = "saturatedFat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub saturated_fat: ::std::option::Option<crate::schemas::FloatUnit>,
        #[doc = "Saturated fat daily percentage."]
        #[serde(
            rename = "saturatedFatDailyPercentage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub saturated_fat_daily_percentage: ::std::option::Option<f64>,
        #[doc = "Food Serving Size. Serving size description."]
        #[serde(
            rename = "servingSizeDescription",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub serving_size_description: ::std::option::Option<String>,
        #[doc = "Serving size measure."]
        #[serde(
            rename = "servingSizeMeasure",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub serving_size_measure: ::std::option::Option<crate::schemas::FloatUnit>,
        #[doc = "Servings per container."]
        #[serde(
            rename = "servingsPerContainer",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub servings_per_container: ::std::option::Option<String>,
        #[doc = "Sodium."]
        #[serde(
            rename = "sodium",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sodium: ::std::option::Option<crate::schemas::FloatUnit>,
        #[doc = "Sodium daily percentage."]
        #[serde(
            rename = "sodiumDailyPercentage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub sodium_daily_percentage: ::std::option::Option<f64>,
        #[doc = "Starch."]
        #[serde(
            rename = "starch",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub starch: ::std::option::Option<crate::schemas::FloatUnit>,
        #[doc = "Total carbohydrate."]
        #[serde(
            rename = "totalCarbohydrate",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_carbohydrate: ::std::option::Option<crate::schemas::FloatUnit>,
        #[doc = "Total carbohydrate daily percentage."]
        #[serde(
            rename = "totalCarbohydrateDailyPercentage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_carbohydrate_daily_percentage: ::std::option::Option<f64>,
        #[doc = "Total fat."]
        #[serde(
            rename = "totalFat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_fat: ::std::option::Option<crate::schemas::FloatUnit>,
        #[doc = "Total fat daily percentage."]
        #[serde(
            rename = "totalFatDailyPercentage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_fat_daily_percentage: ::std::option::Option<f64>,
        #[doc = "Total sugars."]
        #[serde(
            rename = "totalSugars",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_sugars: ::std::option::Option<crate::schemas::FloatUnit>,
        #[doc = "Total sugars daily percentage."]
        #[serde(
            rename = "totalSugarsDailyPercentage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub total_sugars_daily_percentage: ::std::option::Option<f64>,
        #[doc = "Trans fat."]
        #[serde(
            rename = "transFat",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub trans_fat: ::std::option::Option<crate::schemas::FloatUnit>,
        #[doc = "Trans fat daily percentage."]
        #[serde(
            rename = "transFatDailyPercentage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub trans_fat_daily_percentage: ::std::option::Option<f64>,
        #[doc = "Vitamin D."]
        #[serde(
            rename = "vitaminD",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vitamin_d: ::std::option::Option<crate::schemas::FloatUnit>,
        #[doc = "Vitamin D daily percentage."]
        #[serde(
            rename = "vitaminDDailyPercentage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub vitamin_d_daily_percentage: ::std::option::Option<f64>,
        #[doc = "Voluntary nutrition fact."]
        #[serde(
            rename = "voluntaryNutritionFact",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub voluntary_nutrition_fact:
            ::std::option::Option<Vec<crate::schemas::VoluntaryNutritionFact>>,
    }
    impl ::google_field_selector::FieldSelector for Nutrition {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Nutrition {
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
    pub struct Price {
        #[doc = "The numeric value of the price."]
        #[serde(
            rename = "amount",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub amount: ::std::option::Option<String>,
        #[doc = "The currency in which the price is denoted."]
        #[serde(
            rename = "currency",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub currency: ::std::option::Option<String>,
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
    pub struct Product {
        #[doc = "Attributes of the product uploaded to the Manufacturer Center. Manually edited attributes are taken into account."]
        #[serde(
            rename = "attributes",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attributes: ::std::option::Option<crate::schemas::Attributes>,
        #[doc = "The content language of the product as a two-letter ISO 639-1 language code (for example, en)."]
        #[serde(
            rename = "contentLanguage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub content_language: ::std::option::Option<String>,
        #[doc = "The status of the destinations."]
        #[serde(
            rename = "destinationStatuses",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub destination_statuses: ::std::option::Option<Vec<crate::schemas::DestinationStatus>>,
        #[doc = "A server-generated list of issues associated with the product."]
        #[serde(
            rename = "issues",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub issues: ::std::option::Option<Vec<crate::schemas::Issue>>,
        #[doc = "Name in the format `{target_country}:{content_language}:{product_id}`. `target_country` - The target country of the product as a CLDR territory code (for example, US). `content_language` - The content language of the product as a two-letter ISO 639-1 language code (for example, en). `product_id` - The ID of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#id."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Parent ID in the format `accounts/{account_id}`. `account_id` - The ID of the Manufacturer Center account."]
        #[serde(
            rename = "parent",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub parent: ::std::option::Option<String>,
        #[doc = "The ID of the product. For more information, see https://support.google.com/manufacturers/answer/6124116#id."]
        #[serde(
            rename = "productId",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub product_id: ::std::option::Option<String>,
        #[doc = "The target country of the product as a CLDR territory code (for example, US)."]
        #[serde(
            rename = "targetCountry",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub target_country: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for Product {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for Product {
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
    pub struct ProductDetail {
        #[doc = "The name of the attribute."]
        #[serde(
            rename = "attributeName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attribute_name: ::std::option::Option<String>,
        #[doc = "The value of the attribute."]
        #[serde(
            rename = "attributeValue",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub attribute_value: ::std::option::Option<String>,
        #[doc = "A short section name that can be reused between multiple product details."]
        #[serde(
            rename = "sectionName",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub section_name: ::std::option::Option<String>,
    }
    impl ::google_field_selector::FieldSelector for ProductDetail {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for ProductDetail {
        fn field_type() -> ::google_field_selector::FieldType {
            ::google_field_selector::FieldType::Leaf
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct VoluntaryNutritionFact {
        #[doc = "Daily percentage."]
        #[serde(
            rename = "dailyPercentage",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub daily_percentage: ::std::option::Option<f64>,
        #[doc = "Name."]
        #[serde(
            rename = "name",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub name: ::std::option::Option<String>,
        #[doc = "Value."]
        #[serde(
            rename = "value",
            default,
            skip_serializing_if = "std::option::Option::is_none"
        )]
        pub value: ::std::option::Option<crate::schemas::FloatUnit>,
    }
    impl ::google_field_selector::FieldSelector for VoluntaryNutritionFact {
        fn fields() -> Vec<::google_field_selector::Field> {
            Vec::new()
        }
    }
    impl ::google_field_selector::ToFieldType for VoluntaryNutritionFact {
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
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
        }
        impl<'a> AccountsActions<'a> {
            fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                self.auth
            }
            #[doc = "Actions that can be performed on the products resource"]
            pub fn products(&self) -> crate::resources::accounts::products::ProductsActions {
                crate::resources::accounts::products::ProductsActions {
                    reqwest: &self.reqwest,
                    auth: self.auth_ref(),
                }
            }
        }
        pub mod products {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum GetIncludeItems {
                    #[doc = "Include the attributes of the product."]
                    Attributes,
                    #[doc = "Include the destination statuses of the product."]
                    DestinationStatuses,
                    #[doc = "Include the issues of the product."]
                    Issues,
                    #[doc = "Unknown, never used."]
                    Unknown,
                }
                impl GetIncludeItems {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            GetIncludeItems::Attributes => "ATTRIBUTES",
                            GetIncludeItems::DestinationStatuses => "DESTINATION_STATUSES",
                            GetIncludeItems::Issues => "ISSUES",
                            GetIncludeItems::Unknown => "UNKNOWN",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for GetIncludeItems {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for GetIncludeItems {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<GetIncludeItems, ()> {
                        Ok(match s {
                            "ATTRIBUTES" => GetIncludeItems::Attributes,
                            "DESTINATION_STATUSES" => GetIncludeItems::DestinationStatuses,
                            "ISSUES" => GetIncludeItems::Issues,
                            "UNKNOWN" => GetIncludeItems::Unknown,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for GetIncludeItems {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for GetIncludeItems {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for GetIncludeItems {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "ATTRIBUTES" => GetIncludeItems::Attributes,
                            "DESTINATION_STATUSES" => GetIncludeItems::DestinationStatuses,
                            "ISSUES" => GetIncludeItems::Issues,
                            "UNKNOWN" => GetIncludeItems::Unknown,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for GetIncludeItems {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for GetIncludeItems {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListIncludeItems {
                    #[doc = "Include the attributes of the product."]
                    Attributes,
                    #[doc = "Include the destination statuses of the product."]
                    DestinationStatuses,
                    #[doc = "Include the issues of the product."]
                    Issues,
                    #[doc = "Unknown, never used."]
                    Unknown,
                }
                impl ListIncludeItems {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListIncludeItems::Attributes => "ATTRIBUTES",
                            ListIncludeItems::DestinationStatuses => "DESTINATION_STATUSES",
                            ListIncludeItems::Issues => "ISSUES",
                            ListIncludeItems::Unknown => "UNKNOWN",
                        }
                    }
                }
                impl ::std::convert::AsRef<str> for ListIncludeItems {
                    fn as_ref(&self) -> &str {
                        self.as_str()
                    }
                }
                impl ::std::str::FromStr for ListIncludeItems {
                    type Err = ();
                    fn from_str(s: &str) -> ::std::result::Result<ListIncludeItems, ()> {
                        Ok(match s {
                            "ATTRIBUTES" => ListIncludeItems::Attributes,
                            "DESTINATION_STATUSES" => ListIncludeItems::DestinationStatuses,
                            "ISSUES" => ListIncludeItems::Issues,
                            "UNKNOWN" => ListIncludeItems::Unknown,
                            _ => return Err(()),
                        })
                    }
                }
                impl ::std::fmt::Display for ListIncludeItems {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListIncludeItems {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListIncludeItems {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "ATTRIBUTES" => ListIncludeItems::Attributes,
                            "DESTINATION_STATUSES" => ListIncludeItems::DestinationStatuses,
                            "ISSUES" => ListIncludeItems::Issues,
                            "UNKNOWN" => ListIncludeItems::Unknown,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::google_field_selector::FieldSelector for ListIncludeItems {
                    fn fields() -> Vec<::google_field_selector::Field> {
                        Vec::new()
                    }
                }
                impl ::google_field_selector::ToFieldType for ListIncludeItems {
                    fn field_type() -> ::google_field_selector::FieldType {
                        ::google_field_selector::FieldType::Leaf
                    }
                }
            }
            pub struct ProductsActions<'a> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
            }
            impl<'a> ProductsActions<'a> {
                fn auth_ref(&self) -> &dyn ::google_api_auth::GetAccessToken {
                    self.auth
                }
                #[doc = "Deletes the product from a Manufacturer Center account."]
                pub fn delete(
                    &self,
                    parent: impl Into<String>,
                    name: impl Into<String>,
                ) -> DeleteRequestBuilder {
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
                        parent: parent.into(),
                        name: name.into(),
                    }
                }
                #[doc = "Gets the product from a Manufacturer Center account, including product issues. A recently updated product takes around 15 minutes to process. Changes are only visible after it has been processed. While some issues may be available once the product has been processed, other issues may take days to appear."]
                pub fn get(
                    &self,
                    parent: impl Into<String>,
                    name: impl Into<String>,
                ) -> GetRequestBuilder {
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
                        parent: parent.into(),
                        name: name.into(),
                        include: None,
                    }
                }
                #[doc = "Lists all the products in a Manufacturer Center account."]
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
                        include: None,
                        page_size: None,
                        page_token: None,
                    }
                }
                #[doc = "Inserts or updates the attributes of the product in a Manufacturer Center account. Creates a product with the provided attributes. If the product already exists, then all attributes are replaced with the new ones. The checks at upload time are minimal. All required attributes need to be present for a product to be valid. Issues may show up later after the API has accepted a new upload for a product and it is possible to overwrite an existing valid product with an invalid product. To detect this, you should retrieve the product and check it for issues once the new version is available. Uploaded attributes first need to be processed before they can be retrieved. Until then, new products will be unavailable, and retrieval of previously uploaded products will return the original state of the product."]
                pub fn update(
                    &self,
                    request: crate::schemas::Attributes,
                    parent: impl Into<String>,
                    name: impl Into<String>,
                ) -> UpdateRequestBuilder {
                    UpdateRequestBuilder {
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
                        name: name.into(),
                    }
                }
            }
            #[doc = "Created via [ProductsActions::delete()](struct.ProductsActions.html#method.delete)"]
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
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
            impl<'a> DeleteRequestBuilder<'a> {
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
                    let mut output = "https://manufacturers.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/products/");
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
            #[doc = "Created via [ProductsActions::get()](struct.ProductsActions.html#method.get)"]
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                name: String,
                include: ::std::option::Option<
                    Vec<crate::resources::accounts::products::params::GetIncludeItems>,
                >,
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
                #[doc = "The information to be included in the response. Only sections listed here will be returned."]
                pub fn include(
                    mut self,
                    value: impl Into<Vec<crate::resources::accounts::products::params::GetIncludeItems>>,
                ) -> Self {
                    self.include = Some(value.into());
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
                ) -> Result<crate::schemas::Product, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::Product, crate::Error> {
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
                    let mut output = "https://manufacturers.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/products/");
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
                    for value in self.include.iter().flatten() {
                        req = req.query(&[("include", value)]);
                    }
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
            #[doc = "Created via [ProductsActions::list()](struct.ProductsActions.html#method.list)"]
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                parent: String,
                include: ::std::option::Option<
                    Vec<crate::resources::accounts::products::params::ListIncludeItems>,
                >,
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
                #[doc = "The information to be included in the response. Only sections listed here will be returned."]
                pub fn include(
                    mut self,
                    value: impl Into<
                        Vec<crate::resources::accounts::products::params::ListIncludeItems>,
                    >,
                ) -> Self {
                    self.include = Some(value.into());
                    self
                }
                #[doc = "Maximum number of product statuses to return in the response, used for paging."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "The token returned by the previous request."]
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
                ) -> impl ::futures::Stream<Item = Result<crate::schemas::Product, crate::Error>> + 'a
                {
                    self.stream_products_with_fields(None::<String>)
                }
                #[doc = "\nExecute the request and yield each item in the `products` list. If the response contains a\n`nextPageToken`, the request is executed again with the new token. This process is\nrepeated until no page token is returned.\n\nRequests all fields from the server.\n"]
                pub fn stream_products_with_all_fields(
                    self,
                ) -> impl ::futures::Stream<Item = Result<crate::schemas::Product, crate::Error>> + 'a
                {
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
                #[doc = r" Requests the default set of fields from the server."]
                pub fn stream_with_default_fields(
                    self,
                ) -> impl ::futures::Stream<
                    Item = Result<crate::schemas::ListProductsResponse, crate::Error>,
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
                    Item = Result<crate::schemas::ListProductsResponse, crate::Error>,
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
                ) -> Result<crate::schemas::ListProductsResponse, crate::Error> {
                    self.execute_with_fields(None::<&str>).await
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub async fn execute_with_all_fields(
                    self,
                ) -> Result<crate::schemas::ListProductsResponse, crate::Error> {
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
                    let mut output = "https://manufacturers.googleapis.com/".to_owned();
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
                    for value in self.include.iter().flatten() {
                        req = req.query(&[("include", value)]);
                    }
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
            #[doc = "Created via [ProductsActions::update()](struct.ProductsActions.html#method.update)"]
            #[derive(Debug, Clone)]
            pub struct UpdateRequestBuilder<'a> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a dyn ::google_api_auth::GetAccessToken,
                request: crate::schemas::Attributes,
                parent: String,
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
            impl<'a> UpdateRequestBuilder<'a> {
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
                    let mut output = "https://manufacturers.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/products/");
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
                    let mut req = self.reqwest.request(::reqwest::Method::PUT, path);
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
