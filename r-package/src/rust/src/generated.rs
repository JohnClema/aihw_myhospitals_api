#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
#[allow(unused_imports)]
use progenitor_client::{encode_path, ClientHooks, OperationInfo, RequestBuilderExt};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
        impl ::std::fmt::Debug for ConversionError {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }
        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }
        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }
    ///A Caveat is a footnote or suppression used to apply to, or instead of, a data value.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A Caveat is a footnote or suppression used to apply to, or instead of, a data value.",
    ///  "type": "object",
    ///  "required": [
    ///    "caveat_code",
    ///    "caveat_name"
    ///  ],
    ///  "properties": {
    ///    "caveat_code": {
    ///      "description": "The code for the corresponding Caveat.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "caveat_display_value": {
    ///      "description": "The display value or symbol for the corresponding Caveat. If the Caveat has no symbol then this field will be null.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "caveat_footnote": {
    ///      "description": "The footnote/explanation text for the corresponding Caveat. If the Caveat has no text then this field will be null.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "caveat_name": {
    ///      "description": "The name for the corresponding Caveat.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct CaveatModel {
        ///The code for the corresponding Caveat.
        pub caveat_code: CaveatModelCaveatCode,
        ///The display value or symbol for the corresponding Caveat. If the Caveat has no symbol then this field will be null.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub caveat_display_value: ::std::option::Option<::std::string::String>,
        ///The footnote/explanation text for the corresponding Caveat. If the Caveat has no text then this field will be null.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub caveat_footnote: ::std::option::Option<::std::string::String>,
        ///The name for the corresponding Caveat.
        pub caveat_name: CaveatModelCaveatName,
    }
    impl ::std::convert::From<&CaveatModel> for CaveatModel {
        fn from(value: &CaveatModel) -> Self {
            value.clone()
        }
    }
    impl CaveatModel {
        pub fn builder() -> builder::CaveatModel {
            Default::default()
        }
    }
    ///The code for the corresponding Caveat.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the corresponding Caveat.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CaveatModelCaveatCode(::std::string::String);
    impl ::std::ops::Deref for CaveatModelCaveatCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<CaveatModelCaveatCode> for ::std::string::String {
        fn from(value: CaveatModelCaveatCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&CaveatModelCaveatCode> for CaveatModelCaveatCode {
        fn from(value: &CaveatModelCaveatCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for CaveatModelCaveatCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for CaveatModelCaveatCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for CaveatModelCaveatCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CaveatModelCaveatCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CaveatModelCaveatCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the corresponding Caveat.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the corresponding Caveat.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct CaveatModelCaveatName(::std::string::String);
    impl ::std::ops::Deref for CaveatModelCaveatName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<CaveatModelCaveatName> for ::std::string::String {
        fn from(value: CaveatModelCaveatName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&CaveatModelCaveatName> for CaveatModelCaveatName {
        fn from(value: &CaveatModelCaveatName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for CaveatModelCaveatName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for CaveatModelCaveatName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for CaveatModelCaveatName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for CaveatModelCaveatName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CaveatModelCaveatName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///A DataExtract item is a flattened structure encapsulates a single DataItem and includes the DataItem's associated attributes.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A DataExtract item is a flattened structure encapsulates a single DataItem and includes the DataItem's associated attributes.",
    ///  "type": "object",
    ///  "required": [
    ///    "decimal_places",
    ///    "measure_category_code",
    ///    "measure_code",
    ///    "measure_name",
    ///    "reported_measure_category_code",
    ///    "reported_measure_category_name",
    ///    "reported_measure_category_type_code",
    ///    "reported_measure_category_type_name",
    ///    "reported_measure_code",
    ///    "reported_measure_name",
    ///    "reporting_end_date",
    ///    "reporting_start_date",
    ///    "reporting_unit_code",
    ///    "reporting_unit_name",
    ///    "reporting_unit_type_code",
    ///    "reporting_unit_type_name",
    ///    "units_are_prefix",
    ///    "units_name"
    ///  ],
    ///  "properties": {
    ///    "caveat": {
    ///      "description": "The Caveat symbols applicable to this DataItem. This will be null if there are no Caveats.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "caveat_codes": {
    ///      "description": "Semi-colon separated list of Caveat codes applicable to this DataItem. This will be null if there are no Caveats.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "caveat_footnotes": {
    ///      "description": "Semi-colon separated list of Caveat symbols and their explanatory footnote applicable to this DataItem. This will be null if there are no Caveats.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "data_set_caveat": {
    ///      "description": "The Caveat symbols applicable to the corresponding DataSet. This will be null if there are no Caveats.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "data_set_caveat_codes": {
    ///      "description": "Semi-colon separated list of Caveat codes applicable to the corresponding DataSet. This will be null if there are no Caveats.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "data_set_caveat_footnotes": {
    ///      "description": "Semi-colon separated list of Caveat symbols and their explanatory footnote applicable to the corresponding DataSet. This will be null if there are no Caveats.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "data_set_meta_tag_codes": {
    ///      "description": "Semi-colon separated list of MetaTag codes for the corresponding DataSet. This will be null if there are no DataSet MetaTags.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "data_set_meta_tags": {
    ///      "description": "Semi-colon separated list of MetaTag values for the corresponding DataSet. This will be null if there are no DataSet MetaTags.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "data_set_meta_tags_with_type": {
    ///      "description": "Semi-colon separated list of MetaTag values for the corresponding DataSet. Each MetaTag value matches the pattern MetaTagTypeCode|MetaTagValue. This will be null if there are no DataSet MetaTags.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "decimal_places": {
    ///      "description": "The number of decimal places that the data can be expressed to.",
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "group_number": {
    ///      "description": "Indicates a quintile, decile or other grouping number for the data value. This will be null if it is not applicable.",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32"
    ///    },
    ///    "lower_value": {
    ///      "description": "Represents the lower data value if this DataItem is a data range or has a confidence interval, otherwise this will be null.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "mapped_state": {
    ///      "description": "The state abbreviation of the corresponding ReportingUnit's associated state.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "measure_category_code": {
    ///      "description": "The code for the associated MeasureCategory.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "measure_code": {
    ///      "description": "The code for the associated Measure.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "measure_meta_tag_codes": {
    ///      "description": "Semi-colon separated list of MetaTag codes for the corresponding Measure. This will be null if there are no Measure MetaTags.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "measure_meta_tags": {
    ///      "description": "Semi-colon separated list of MetaTag values for the corresponding Measure. This will be null if there are no Measure MetaTags.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "measure_meta_tags_with_type": {
    ///      "description": "Semi-colon separated list of MetaTag values for the corresponding Measure. Each MetaTag value matches the pattern MetaTagTypeCode|MetaTagValue. This will be null if there are no Measure MetaTags.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "measure_name": {
    ///      "description": "The name for the associated Measure.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "peer_caveat": {
    ///      "description": "The Caveats symbols for the peer group value for this DataItem. If there are no peer caveats or no peer group then this will be null.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "peer_caveat_codes": {
    ///      "description": "Semi-colon separated list of Caveats codes for the peer group value for this DataItem. If there are no peer caveats or no peer group then this will be null.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "peer_caveat_footnotes": {
    ///      "description": "Semi-colon separated list of Caveat symbols and their explanatory footnote applicable to the peer group value for this DataItem. If there are no peer caveats or no peer group then this will be null.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "peer_group_name": {
    ///      "description": "The PeerGroup name for the associated PeerGroup ReportingUnit. If there is no peer then this will be null.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "peer_suppression": {
    ///      "description": "The Suppressions symbols for the peer group value for this DataItem. If there are no peer suppressions or no peer group then this will be null.",
    ///      "deprecated": true,
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "peer_suppression_codes": {
    ///      "description": "Semi-colon separated list of Suppressions codes for the peer group value for this DataItem. If there are no peer suppressions or no peer group then this will be null.",
    ///      "deprecated": true,
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "peer_value": {
    ///      "description": "The peer data value for this DataItem. This may be null if the peer data is suppressed or there is no peer.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "proxy_children": {
    ///      "description": "Semi-colon separated list of the names of any ReportingUnits that the associated ReportingUnit is reporting for/including in the data value.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "proxy_reporting_unit_name": {
    ///      "description": "The name of the hospital that reports on behalf of the associated ReportingUnit. This will be null if there is no proxy reporting.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reported_measure_category_code": {
    ///      "description": "The code for the first ReportedMeasure category.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reported_measure_category_name": {
    ///      "description": "The name for the first ReportedMeasure category.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reported_measure_category_three_code": {
    ///      "description": "The code for the third ReportedMeasure category. This will be null if the ReportedMeasure does not have 3 categories.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reported_measure_category_three_name": {
    ///      "description": "The name for the third ReportedMeasure category. This will be null if the ReportedMeasure does not have 3 categories.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reported_measure_category_three_type_code": {
    ///      "description": "The code for the third ReportedMeasure category type. This will be null if the ReportedMeasure does not have 3 categories.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reported_measure_category_three_type_name": {
    ///      "description": "The name for the third ReportedMeasure category type. This will be null if the ReportedMeasure does not have 3 categories.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reported_measure_category_two_code": {
    ///      "description": "The code for the second ReportedMeasure category. This will be null if the ReportedMeasure does not have 2 categories.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reported_measure_category_two_name": {
    ///      "description": "The name for the second ReportedMeasure category. This will be null if the ReportedMeasure does not have 2 categories.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reported_measure_category_two_type_code": {
    ///      "description": "The code for the second ReportedMeasure category type. This will be null if the ReportedMeasure does not have 2 categories.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reported_measure_category_two_type_name": {
    ///      "description": "The name for the second ReportedMeasure category type. This will be null if the ReportedMeasure does not have 2 categories.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reported_measure_category_type_code": {
    ///      "description": "The code for the first ReportedMeasure category type.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reported_measure_category_type_name": {
    ///      "description": "The name for the first ReportedMeasure category type.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reported_measure_code": {
    ///      "description": "The code for the associated ReportedMeasure.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reported_measure_meta_tag_codes": {
    ///      "description": "Semi-colon separated list of MetaTag codes for the corresponding ReportedMeasure. This will be null if there are no ReportedMeasure MetaTags.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reported_measure_meta_tags": {
    ///      "description": "Semi-colon separated list of MetaTag values for the corresponding ReportedMeasure. This will be null if there are no ReportedMeasure MetaTags.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reported_measure_meta_tags_with_type": {
    ///      "description": "Semi-colon separated list of MetaTag values for the corresponding ReportedMeasure. Each MetaTag value matches the pattern MetaTagTypeCode|MetaTagValue. This will be null if there are no ReportedMeasure MetaTags.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reported_measure_name": {
    ///      "description": "The name for the associated ReportedMeasure.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reporting_end_date": {
    ///      "description": "The end date for the DataSet data period.",
    ///      "type": "string",
    ///      "format": "date"
    ///    },
    ///    "reporting_start_date": {
    ///      "description": "The start date for the DataSet data period.",
    ///      "type": "string",
    ///      "format": "date"
    ///    },
    ///    "reporting_unit_code": {
    ///      "description": "The code for the associated ReportingUnit.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reporting_unit_meta_tag_codes": {
    ///      "description": "Semi-colon separated list of MetaTag codes for the corresponding ReportingUnit. This will be null if there are no ReportingUnit MetaTags.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reporting_unit_meta_tags": {
    ///      "description": "Semi-colon separated list of MetaTag values for the corresponding ReportingUnit. This will be null if there are no ReportingUnit MetaTags.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reporting_unit_meta_tags_with_type": {
    ///      "description": "Semi-colon separated list of MetaTag values for the corresponding ReportingUnit. Each MetaTag value matches the pattern MetaTagTypeCode|MetaTagValue. This will be null if there are no ReportingUnit MetaTags.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reporting_unit_name": {
    ///      "description": "The name for the associated ReportingUnit.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reporting_unit_type_code": {
    ///      "description": "The code for the associated ReportingUnitType.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reporting_unit_type_name": {
    ///      "description": "The name for the associated ReportingUnitType.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "suppression": {
    ///      "description": "The Suppression symbols applicable to this DataItem. This will be null if there are no Suppressions.",
    ///      "deprecated": true,
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "suppression_codes": {
    ///      "description": "Semi-colon separated list of Suppression codes applicable to this DataItem. This will be null if there are no Suppressions.",
    ///      "deprecated": true,
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "units_are_prefix": {
    ///      "description": "Whether the units should be applied as a prefix or suffix to the data value. e.g. $ (prefix) and % (suffix).",
    ///      "type": "boolean"
    ///    },
    ///    "units_display": {
    ///      "description": "The display symbol or text for the units for the data value. For units without a symbol this will be null.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "units_name": {
    ///      "description": "The name for the corresponding Units.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "upper_value": {
    ///      "description": "Represents the upper data value if this DataItem is a data range or has a confidence interval, otherwise this will be null.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "value": {
    ///      "description": "The data value for this item. This data value may be null if the data is suppressed, in which case a caveat will be present in the Caveats list.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct DataExtractModel {
        ///The Caveat symbols applicable to this DataItem. This will be null if there are no Caveats.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub caveat: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of Caveat codes applicable to this DataItem. This will be null if there are no Caveats.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub caveat_codes: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of Caveat symbols and their explanatory footnote applicable to this DataItem. This will be null if there are no Caveats.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub caveat_footnotes: ::std::option::Option<::std::string::String>,
        ///The Caveat symbols applicable to the corresponding DataSet. This will be null if there are no Caveats.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data_set_caveat: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of Caveat codes applicable to the corresponding DataSet. This will be null if there are no Caveats.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data_set_caveat_codes: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of Caveat symbols and their explanatory footnote applicable to the corresponding DataSet. This will be null if there are no Caveats.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data_set_caveat_footnotes: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of MetaTag codes for the corresponding DataSet. This will be null if there are no DataSet MetaTags.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data_set_meta_tag_codes: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of MetaTag values for the corresponding DataSet. This will be null if there are no DataSet MetaTags.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data_set_meta_tags: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of MetaTag values for the corresponding DataSet. Each MetaTag value matches the pattern MetaTagTypeCode|MetaTagValue. This will be null if there are no DataSet MetaTags.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data_set_meta_tags_with_type: ::std::option::Option<::std::string::String>,
        ///The number of decimal places that the data can be expressed to.
        pub decimal_places: i32,
        ///Indicates a quintile, decile or other grouping number for the data value. This will be null if it is not applicable.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub group_number: ::std::option::Option<i32>,
        ///Represents the lower data value if this DataItem is a data range or has a confidence interval, otherwise this will be null.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub lower_value: ::std::option::Option<f64>,
        ///The state abbreviation of the corresponding ReportingUnit's associated state.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mapped_state: ::std::option::Option<::std::string::String>,
        ///The code for the associated MeasureCategory.
        pub measure_category_code: DataExtractModelMeasureCategoryCode,
        ///The code for the associated Measure.
        pub measure_code: DataExtractModelMeasureCode,
        ///Semi-colon separated list of MetaTag codes for the corresponding Measure. This will be null if there are no Measure MetaTags.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub measure_meta_tag_codes: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of MetaTag values for the corresponding Measure. This will be null if there are no Measure MetaTags.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub measure_meta_tags: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of MetaTag values for the corresponding Measure. Each MetaTag value matches the pattern MetaTagTypeCode|MetaTagValue. This will be null if there are no Measure MetaTags.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub measure_meta_tags_with_type: ::std::option::Option<::std::string::String>,
        ///The name for the associated Measure.
        pub measure_name: DataExtractModelMeasureName,
        ///The Caveats symbols for the peer group value for this DataItem. If there are no peer caveats or no peer group then this will be null.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub peer_caveat: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of Caveats codes for the peer group value for this DataItem. If there are no peer caveats or no peer group then this will be null.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub peer_caveat_codes: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of Caveat symbols and their explanatory footnote applicable to the peer group value for this DataItem. If there are no peer caveats or no peer group then this will be null.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub peer_caveat_footnotes: ::std::option::Option<::std::string::String>,
        ///The PeerGroup name for the associated PeerGroup ReportingUnit. If there is no peer then this will be null.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub peer_group_name: ::std::option::Option<::std::string::String>,
        ///The Suppressions symbols for the peer group value for this DataItem. If there are no peer suppressions or no peer group then this will be null.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub peer_suppression: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of Suppressions codes for the peer group value for this DataItem. If there are no peer suppressions or no peer group then this will be null.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub peer_suppression_codes: ::std::option::Option<::std::string::String>,
        ///The peer data value for this DataItem. This may be null if the peer data is suppressed or there is no peer.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub peer_value: ::std::option::Option<f64>,
        ///Semi-colon separated list of the names of any ReportingUnits that the associated ReportingUnit is reporting for/including in the data value.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub proxy_children: ::std::option::Option<::std::string::String>,
        ///The name of the hospital that reports on behalf of the associated ReportingUnit. This will be null if there is no proxy reporting.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub proxy_reporting_unit_name: ::std::option::Option<::std::string::String>,
        ///The code for the first ReportedMeasure category.
        pub reported_measure_category_code: DataExtractModelReportedMeasureCategoryCode,
        ///The name for the first ReportedMeasure category.
        pub reported_measure_category_name: DataExtractModelReportedMeasureCategoryName,
        ///The code for the third ReportedMeasure category. This will be null if the ReportedMeasure does not have 3 categories.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reported_measure_category_three_code: ::std::option::Option<
            ::std::string::String,
        >,
        ///The name for the third ReportedMeasure category. This will be null if the ReportedMeasure does not have 3 categories.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reported_measure_category_three_name: ::std::option::Option<
            ::std::string::String,
        >,
        ///The code for the third ReportedMeasure category type. This will be null if the ReportedMeasure does not have 3 categories.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reported_measure_category_three_type_code: ::std::option::Option<
            ::std::string::String,
        >,
        ///The name for the third ReportedMeasure category type. This will be null if the ReportedMeasure does not have 3 categories.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reported_measure_category_three_type_name: ::std::option::Option<
            ::std::string::String,
        >,
        ///The code for the second ReportedMeasure category. This will be null if the ReportedMeasure does not have 2 categories.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reported_measure_category_two_code: ::std::option::Option<
            ::std::string::String,
        >,
        ///The name for the second ReportedMeasure category. This will be null if the ReportedMeasure does not have 2 categories.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reported_measure_category_two_name: ::std::option::Option<
            ::std::string::String,
        >,
        ///The code for the second ReportedMeasure category type. This will be null if the ReportedMeasure does not have 2 categories.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reported_measure_category_two_type_code: ::std::option::Option<
            ::std::string::String,
        >,
        ///The name for the second ReportedMeasure category type. This will be null if the ReportedMeasure does not have 2 categories.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reported_measure_category_two_type_name: ::std::option::Option<
            ::std::string::String,
        >,
        ///The code for the first ReportedMeasure category type.
        pub reported_measure_category_type_code: DataExtractModelReportedMeasureCategoryTypeCode,
        ///The name for the first ReportedMeasure category type.
        pub reported_measure_category_type_name: DataExtractModelReportedMeasureCategoryTypeName,
        ///The code for the associated ReportedMeasure.
        pub reported_measure_code: DataExtractModelReportedMeasureCode,
        ///Semi-colon separated list of MetaTag codes for the corresponding ReportedMeasure. This will be null if there are no ReportedMeasure MetaTags.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reported_measure_meta_tag_codes: ::std::option::Option<
            ::std::string::String,
        >,
        ///Semi-colon separated list of MetaTag values for the corresponding ReportedMeasure. This will be null if there are no ReportedMeasure MetaTags.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reported_measure_meta_tags: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of MetaTag values for the corresponding ReportedMeasure. Each MetaTag value matches the pattern MetaTagTypeCode|MetaTagValue. This will be null if there are no ReportedMeasure MetaTags.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reported_measure_meta_tags_with_type: ::std::option::Option<
            ::std::string::String,
        >,
        ///The name for the associated ReportedMeasure.
        pub reported_measure_name: DataExtractModelReportedMeasureName,
        ///The end date for the DataSet data period.
        pub reporting_end_date: ::chrono::naive::NaiveDate,
        ///The start date for the DataSet data period.
        pub reporting_start_date: ::chrono::naive::NaiveDate,
        ///The code for the associated ReportingUnit.
        pub reporting_unit_code: DataExtractModelReportingUnitCode,
        ///Semi-colon separated list of MetaTag codes for the corresponding ReportingUnit. This will be null if there are no ReportingUnit MetaTags.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reporting_unit_meta_tag_codes: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of MetaTag values for the corresponding ReportingUnit. This will be null if there are no ReportingUnit MetaTags.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reporting_unit_meta_tags: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of MetaTag values for the corresponding ReportingUnit. Each MetaTag value matches the pattern MetaTagTypeCode|MetaTagValue. This will be null if there are no ReportingUnit MetaTags.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reporting_unit_meta_tags_with_type: ::std::option::Option<
            ::std::string::String,
        >,
        ///The name for the associated ReportingUnit.
        pub reporting_unit_name: DataExtractModelReportingUnitName,
        ///The code for the associated ReportingUnitType.
        pub reporting_unit_type_code: DataExtractModelReportingUnitTypeCode,
        ///The name for the associated ReportingUnitType.
        pub reporting_unit_type_name: DataExtractModelReportingUnitTypeName,
        ///The Suppression symbols applicable to this DataItem. This will be null if there are no Suppressions.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub suppression: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of Suppression codes applicable to this DataItem. This will be null if there are no Suppressions.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub suppression_codes: ::std::option::Option<::std::string::String>,
        ///Whether the units should be applied as a prefix or suffix to the data value. e.g. $ (prefix) and % (suffix).
        pub units_are_prefix: bool,
        ///The display symbol or text for the units for the data value. For units without a symbol this will be null.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub units_display: ::std::option::Option<::std::string::String>,
        ///The name for the corresponding Units.
        pub units_name: DataExtractModelUnitsName,
        ///Represents the upper data value if this DataItem is a data range or has a confidence interval, otherwise this will be null.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub upper_value: ::std::option::Option<f64>,
        ///The data value for this item. This data value may be null if the data is suppressed, in which case a caveat will be present in the Caveats list.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub value: ::std::option::Option<f64>,
    }
    impl ::std::convert::From<&DataExtractModel> for DataExtractModel {
        fn from(value: &DataExtractModel) -> Self {
            value.clone()
        }
    }
    impl DataExtractModel {
        pub fn builder() -> builder::DataExtractModel {
            Default::default()
        }
    }
    ///The code for the associated MeasureCategory.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the associated MeasureCategory.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DataExtractModelMeasureCategoryCode(::std::string::String);
    impl ::std::ops::Deref for DataExtractModelMeasureCategoryCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<DataExtractModelMeasureCategoryCode>
    for ::std::string::String {
        fn from(value: DataExtractModelMeasureCategoryCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&DataExtractModelMeasureCategoryCode>
    for DataExtractModelMeasureCategoryCode {
        fn from(value: &DataExtractModelMeasureCategoryCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for DataExtractModelMeasureCategoryCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for DataExtractModelMeasureCategoryCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for DataExtractModelMeasureCategoryCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for DataExtractModelMeasureCategoryCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataExtractModelMeasureCategoryCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The code for the associated Measure.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the associated Measure.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DataExtractModelMeasureCode(::std::string::String);
    impl ::std::ops::Deref for DataExtractModelMeasureCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<DataExtractModelMeasureCode> for ::std::string::String {
        fn from(value: DataExtractModelMeasureCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&DataExtractModelMeasureCode>
    for DataExtractModelMeasureCode {
        fn from(value: &DataExtractModelMeasureCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for DataExtractModelMeasureCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for DataExtractModelMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for DataExtractModelMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for DataExtractModelMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataExtractModelMeasureCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the associated Measure.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the associated Measure.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DataExtractModelMeasureName(::std::string::String);
    impl ::std::ops::Deref for DataExtractModelMeasureName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<DataExtractModelMeasureName> for ::std::string::String {
        fn from(value: DataExtractModelMeasureName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&DataExtractModelMeasureName>
    for DataExtractModelMeasureName {
        fn from(value: &DataExtractModelMeasureName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for DataExtractModelMeasureName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for DataExtractModelMeasureName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for DataExtractModelMeasureName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for DataExtractModelMeasureName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataExtractModelMeasureName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The code for the first ReportedMeasure category.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the first ReportedMeasure category.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DataExtractModelReportedMeasureCategoryCode(::std::string::String);
    impl ::std::ops::Deref for DataExtractModelReportedMeasureCategoryCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<DataExtractModelReportedMeasureCategoryCode>
    for ::std::string::String {
        fn from(value: DataExtractModelReportedMeasureCategoryCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&DataExtractModelReportedMeasureCategoryCode>
    for DataExtractModelReportedMeasureCategoryCode {
        fn from(value: &DataExtractModelReportedMeasureCategoryCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for DataExtractModelReportedMeasureCategoryCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for DataExtractModelReportedMeasureCategoryCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for DataExtractModelReportedMeasureCategoryCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for DataExtractModelReportedMeasureCategoryCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataExtractModelReportedMeasureCategoryCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the first ReportedMeasure category.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the first ReportedMeasure category.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DataExtractModelReportedMeasureCategoryName(::std::string::String);
    impl ::std::ops::Deref for DataExtractModelReportedMeasureCategoryName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<DataExtractModelReportedMeasureCategoryName>
    for ::std::string::String {
        fn from(value: DataExtractModelReportedMeasureCategoryName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&DataExtractModelReportedMeasureCategoryName>
    for DataExtractModelReportedMeasureCategoryName {
        fn from(value: &DataExtractModelReportedMeasureCategoryName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for DataExtractModelReportedMeasureCategoryName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for DataExtractModelReportedMeasureCategoryName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for DataExtractModelReportedMeasureCategoryName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for DataExtractModelReportedMeasureCategoryName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataExtractModelReportedMeasureCategoryName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The code for the first ReportedMeasure category type.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the first ReportedMeasure category type.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DataExtractModelReportedMeasureCategoryTypeCode(::std::string::String);
    impl ::std::ops::Deref for DataExtractModelReportedMeasureCategoryTypeCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<DataExtractModelReportedMeasureCategoryTypeCode>
    for ::std::string::String {
        fn from(value: DataExtractModelReportedMeasureCategoryTypeCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&DataExtractModelReportedMeasureCategoryTypeCode>
    for DataExtractModelReportedMeasureCategoryTypeCode {
        fn from(value: &DataExtractModelReportedMeasureCategoryTypeCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for DataExtractModelReportedMeasureCategoryTypeCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str>
    for DataExtractModelReportedMeasureCategoryTypeCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for DataExtractModelReportedMeasureCategoryTypeCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for DataExtractModelReportedMeasureCategoryTypeCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de>
    for DataExtractModelReportedMeasureCategoryTypeCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the first ReportedMeasure category type.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the first ReportedMeasure category type.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DataExtractModelReportedMeasureCategoryTypeName(::std::string::String);
    impl ::std::ops::Deref for DataExtractModelReportedMeasureCategoryTypeName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<DataExtractModelReportedMeasureCategoryTypeName>
    for ::std::string::String {
        fn from(value: DataExtractModelReportedMeasureCategoryTypeName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&DataExtractModelReportedMeasureCategoryTypeName>
    for DataExtractModelReportedMeasureCategoryTypeName {
        fn from(value: &DataExtractModelReportedMeasureCategoryTypeName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for DataExtractModelReportedMeasureCategoryTypeName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str>
    for DataExtractModelReportedMeasureCategoryTypeName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for DataExtractModelReportedMeasureCategoryTypeName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for DataExtractModelReportedMeasureCategoryTypeName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de>
    for DataExtractModelReportedMeasureCategoryTypeName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The code for the associated ReportedMeasure.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the associated ReportedMeasure.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DataExtractModelReportedMeasureCode(::std::string::String);
    impl ::std::ops::Deref for DataExtractModelReportedMeasureCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<DataExtractModelReportedMeasureCode>
    for ::std::string::String {
        fn from(value: DataExtractModelReportedMeasureCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&DataExtractModelReportedMeasureCode>
    for DataExtractModelReportedMeasureCode {
        fn from(value: &DataExtractModelReportedMeasureCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for DataExtractModelReportedMeasureCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for DataExtractModelReportedMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for DataExtractModelReportedMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for DataExtractModelReportedMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataExtractModelReportedMeasureCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the associated ReportedMeasure.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the associated ReportedMeasure.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DataExtractModelReportedMeasureName(::std::string::String);
    impl ::std::ops::Deref for DataExtractModelReportedMeasureName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<DataExtractModelReportedMeasureName>
    for ::std::string::String {
        fn from(value: DataExtractModelReportedMeasureName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&DataExtractModelReportedMeasureName>
    for DataExtractModelReportedMeasureName {
        fn from(value: &DataExtractModelReportedMeasureName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for DataExtractModelReportedMeasureName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for DataExtractModelReportedMeasureName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for DataExtractModelReportedMeasureName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for DataExtractModelReportedMeasureName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataExtractModelReportedMeasureName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The code for the associated ReportingUnit.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the associated ReportingUnit.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DataExtractModelReportingUnitCode(::std::string::String);
    impl ::std::ops::Deref for DataExtractModelReportingUnitCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<DataExtractModelReportingUnitCode>
    for ::std::string::String {
        fn from(value: DataExtractModelReportingUnitCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&DataExtractModelReportingUnitCode>
    for DataExtractModelReportingUnitCode {
        fn from(value: &DataExtractModelReportingUnitCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for DataExtractModelReportingUnitCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for DataExtractModelReportingUnitCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for DataExtractModelReportingUnitCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for DataExtractModelReportingUnitCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataExtractModelReportingUnitCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the associated ReportingUnit.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the associated ReportingUnit.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DataExtractModelReportingUnitName(::std::string::String);
    impl ::std::ops::Deref for DataExtractModelReportingUnitName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<DataExtractModelReportingUnitName>
    for ::std::string::String {
        fn from(value: DataExtractModelReportingUnitName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&DataExtractModelReportingUnitName>
    for DataExtractModelReportingUnitName {
        fn from(value: &DataExtractModelReportingUnitName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for DataExtractModelReportingUnitName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for DataExtractModelReportingUnitName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for DataExtractModelReportingUnitName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for DataExtractModelReportingUnitName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataExtractModelReportingUnitName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The code for the associated ReportingUnitType.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the associated ReportingUnitType.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DataExtractModelReportingUnitTypeCode(::std::string::String);
    impl ::std::ops::Deref for DataExtractModelReportingUnitTypeCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<DataExtractModelReportingUnitTypeCode>
    for ::std::string::String {
        fn from(value: DataExtractModelReportingUnitTypeCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&DataExtractModelReportingUnitTypeCode>
    for DataExtractModelReportingUnitTypeCode {
        fn from(value: &DataExtractModelReportingUnitTypeCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for DataExtractModelReportingUnitTypeCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for DataExtractModelReportingUnitTypeCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for DataExtractModelReportingUnitTypeCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for DataExtractModelReportingUnitTypeCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataExtractModelReportingUnitTypeCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the associated ReportingUnitType.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the associated ReportingUnitType.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DataExtractModelReportingUnitTypeName(::std::string::String);
    impl ::std::ops::Deref for DataExtractModelReportingUnitTypeName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<DataExtractModelReportingUnitTypeName>
    for ::std::string::String {
        fn from(value: DataExtractModelReportingUnitTypeName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&DataExtractModelReportingUnitTypeName>
    for DataExtractModelReportingUnitTypeName {
        fn from(value: &DataExtractModelReportingUnitTypeName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for DataExtractModelReportingUnitTypeName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for DataExtractModelReportingUnitTypeName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for DataExtractModelReportingUnitTypeName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for DataExtractModelReportingUnitTypeName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataExtractModelReportingUnitTypeName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the corresponding Units.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the corresponding Units.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DataExtractModelUnitsName(::std::string::String);
    impl ::std::ops::Deref for DataExtractModelUnitsName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<DataExtractModelUnitsName> for ::std::string::String {
        fn from(value: DataExtractModelUnitsName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&DataExtractModelUnitsName> for DataExtractModelUnitsName {
        fn from(value: &DataExtractModelUnitsName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for DataExtractModelUnitsName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for DataExtractModelUnitsName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for DataExtractModelUnitsName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for DataExtractModelUnitsName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataExtractModelUnitsName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///Provides the pagination details for a paginated results set.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Provides the pagination details for a paginated results set.",
    ///  "type": "object",
    ///  "properties": {
    ///    "results_returned": {
    ///      "description": "The number of results returned in this response.",
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "starting_result_index": {
    ///      "description": "The index of the starting result in this list. This corresponds to the skip + 1 from the pagination request.",
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "total_results_available": {
    ///      "description": "The total number of results available via pagination.",
    ///      "type": "integer",
    ///      "format": "int32"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct DataExtractPagination {
        ///The number of results returned in this response.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub results_returned: ::std::option::Option<i32>,
        ///The index of the starting result in this list. This corresponds to the skip + 1 from the pagination request.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub starting_result_index: ::std::option::Option<i32>,
        ///The total number of results available via pagination.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub total_results_available: ::std::option::Option<i32>,
    }
    impl ::std::convert::From<&DataExtractPagination> for DataExtractPagination {
        fn from(value: &DataExtractPagination) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for DataExtractPagination {
        fn default() -> Self {
            Self {
                results_returned: Default::default(),
                starting_result_index: Default::default(),
                total_results_available: Default::default(),
            }
        }
    }
    impl DataExtractPagination {
        pub fn builder() -> builder::DataExtractPagination {
            Default::default()
        }
    }
    ///A DataItem encapsulates a data value for a specific Unit (ReportingUnit) covering a specific data period (DataSet).
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A DataItem encapsulates a data value for a specific Unit (ReportingUnit) covering a specific data period (DataSet).",
    ///  "type": "object",
    ///  "required": [
    ///    "caveats",
    ///    "data_set_id",
    ///    "measure_code",
    ///    "reported_measure_code",
    ///    "reporting_unit_summary"
    ///  ],
    ///  "properties": {
    ///    "caveats": {
    ///      "description": "List of Caveats applicable to this DataItem. This list will be empty if there are no Caveats.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/CaveatModel"
    ///      }
    ///    },
    ///    "data_set_id": {
    ///      "description": "The id for the associated DataSet.",
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "group_number": {
    ///      "description": "Indicates a quintile, decile or other grouping number for the data value. This will be null if it is not applicable.",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32"
    ///    },
    ///    "lower_value": {
    ///      "description": "Represents the lower data value if this DataItem is a data range or has a confidence interval, otherwise this will be null.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "measure_code": {
    ///      "description": "The code for the associated Measure.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "peer_group_summary": {
    ///      "$ref": "#/components/schemas/ReportingUnitSummaryModel"
    ///    },
    ///    "proxy_reporting_unit_summary": {
    ///      "$ref": "#/components/schemas/ReportingUnitSummaryModel"
    ///    },
    ///    "reported_measure_code": {
    ///      "description": "The code for the associated ReportedMeasure.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reporting_unit_summary": {
    ///      "$ref": "#/components/schemas/ReportingUnitSummaryModel"
    ///    },
    ///    "suppressions": {
    ///      "description": "Gets or sets the suppressions.",
    ///      "deprecated": true,
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/SuppressionModel"
    ///      }
    ///    },
    ///    "upper_value": {
    ///      "description": "Represents the upper data value if this DataItem is a data range or has a confidence interval, otherwise this will be null.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "value": {
    ///      "description": "The data value for this DataItem. This data value may be null if the data is suppressed, in which case a caveat will be present in the Caveats list.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct DataItemModel {
        ///List of Caveats applicable to this DataItem. This list will be empty if there are no Caveats.
        pub caveats: ::std::vec::Vec<CaveatModel>,
        ///The id for the associated DataSet.
        pub data_set_id: i32,
        ///Indicates a quintile, decile or other grouping number for the data value. This will be null if it is not applicable.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub group_number: ::std::option::Option<i32>,
        ///Represents the lower data value if this DataItem is a data range or has a confidence interval, otherwise this will be null.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub lower_value: ::std::option::Option<f64>,
        ///The code for the associated Measure.
        pub measure_code: DataItemModelMeasureCode,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub peer_group_summary: ::std::option::Option<ReportingUnitSummaryModel>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub proxy_reporting_unit_summary: ::std::option::Option<
            ReportingUnitSummaryModel,
        >,
        ///The code for the associated ReportedMeasure.
        pub reported_measure_code: DataItemModelReportedMeasureCode,
        pub reporting_unit_summary: ReportingUnitSummaryModel,
        ///Gets or sets the suppressions.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub suppressions: ::std::option::Option<::std::vec::Vec<SuppressionModel>>,
        ///Represents the upper data value if this DataItem is a data range or has a confidence interval, otherwise this will be null.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub upper_value: ::std::option::Option<f64>,
        ///The data value for this DataItem. This data value may be null if the data is suppressed, in which case a caveat will be present in the Caveats list.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub value: ::std::option::Option<f64>,
    }
    impl ::std::convert::From<&DataItemModel> for DataItemModel {
        fn from(value: &DataItemModel) -> Self {
            value.clone()
        }
    }
    impl DataItemModel {
        pub fn builder() -> builder::DataItemModel {
            Default::default()
        }
    }
    ///The code for the associated Measure.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the associated Measure.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DataItemModelMeasureCode(::std::string::String);
    impl ::std::ops::Deref for DataItemModelMeasureCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<DataItemModelMeasureCode> for ::std::string::String {
        fn from(value: DataItemModelMeasureCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&DataItemModelMeasureCode> for DataItemModelMeasureCode {
        fn from(value: &DataItemModelMeasureCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for DataItemModelMeasureCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for DataItemModelMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for DataItemModelMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for DataItemModelMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataItemModelMeasureCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The code for the associated ReportedMeasure.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the associated ReportedMeasure.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DataItemModelReportedMeasureCode(::std::string::String);
    impl ::std::ops::Deref for DataItemModelReportedMeasureCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<DataItemModelReportedMeasureCode>
    for ::std::string::String {
        fn from(value: DataItemModelReportedMeasureCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&DataItemModelReportedMeasureCode>
    for DataItemModelReportedMeasureCode {
        fn from(value: &DataItemModelReportedMeasureCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for DataItemModelReportedMeasureCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for DataItemModelReportedMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for DataItemModelReportedMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for DataItemModelReportedMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataItemModelReportedMeasureCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///A DataSet represents a specific data period for a specific ReportedMeasure and is used to group DataItems.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A DataSet represents a specific data period for a specific ReportedMeasure and is used to group DataItems.",
    ///  "type": "object",
    ///  "required": [
    ///    "caveats",
    ///    "data_set_id",
    ///    "data_set_name",
    ///    "meta_tags",
    ///    "reported_measure_summary",
    ///    "reporting_end_date",
    ///    "reporting_start_date"
    ///  ],
    ///  "properties": {
    ///    "caveats": {
    ///      "description": "List of Caveats applicable to this DataSet. This list will be empty if there are no Caveats.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/CaveatModel"
    ///      }
    ///    },
    ///    "data_set_id": {
    ///      "description": "The id for the DataSet.",
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "data_set_name": {
    ///      "description": "The display name for the corresponding DataSet.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "meta_tags": {
    ///      "description": "List of MetaTags applicable to this DataSet. This list will be empty if there are no MetaTags.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MetaTagModel"
    ///      }
    ///    },
    ///    "reported_measure_summary": {
    ///      "$ref": "#/components/schemas/ReportedMeasureSummaryModel"
    ///    },
    ///    "reporting_end_date": {
    ///      "description": "The end date for the DataSet data period.",
    ///      "type": "string",
    ///      "format": "date"
    ///    },
    ///    "reporting_start_date": {
    ///      "description": "The start date for the DataSet data period.",
    ///      "type": "string",
    ///      "format": "date"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct DataSetModel {
        ///List of Caveats applicable to this DataSet. This list will be empty if there are no Caveats.
        pub caveats: ::std::vec::Vec<CaveatModel>,
        ///The id for the DataSet.
        pub data_set_id: i32,
        ///The display name for the corresponding DataSet.
        pub data_set_name: DataSetModelDataSetName,
        ///List of MetaTags applicable to this DataSet. This list will be empty if there are no MetaTags.
        pub meta_tags: ::std::vec::Vec<MetaTagModel>,
        pub reported_measure_summary: ReportedMeasureSummaryModel,
        ///The end date for the DataSet data period.
        pub reporting_end_date: ::chrono::naive::NaiveDate,
        ///The start date for the DataSet data period.
        pub reporting_start_date: ::chrono::naive::NaiveDate,
    }
    impl ::std::convert::From<&DataSetModel> for DataSetModel {
        fn from(value: &DataSetModel) -> Self {
            value.clone()
        }
    }
    impl DataSetModel {
        pub fn builder() -> builder::DataSetModel {
            Default::default()
        }
    }
    ///The display name for the corresponding DataSet.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The display name for the corresponding DataSet.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DataSetModelDataSetName(::std::string::String);
    impl ::std::ops::Deref for DataSetModelDataSetName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<DataSetModelDataSetName> for ::std::string::String {
        fn from(value: DataSetModelDataSetName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&DataSetModelDataSetName> for DataSetModelDataSetName {
        fn from(value: &DataSetModelDataSetName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for DataSetModelDataSetName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for DataSetModelDataSetName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for DataSetModelDataSetName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for DataSetModelDataSetName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataSetModelDataSetName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///A DatasheetConfigurationSummary provides the details of what datasheets are available for generation.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A DatasheetConfigurationSummary provides the details of what datasheets are available for generation.",
    ///  "type": "object",
    ///  "required": [
    ///    "datasheet_code",
    ///    "datasheet_description",
    ///    "datasheet_type"
    ///  ],
    ///  "properties": {
    ///    "datasheet_code": {
    ///      "description": "The code used to specify the datasheet download.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "datasheet_description": {
    ///      "description": "The description of the datasheet that will be generated using this code.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "datasheet_type": {
    ///      "description": "The type of datasheet that will be generated using this code.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct DatasheetConfigurationSummaryModel {
        ///The code used to specify the datasheet download.
        pub datasheet_code: DatasheetConfigurationSummaryModelDatasheetCode,
        ///The description of the datasheet that will be generated using this code.
        pub datasheet_description: DatasheetConfigurationSummaryModelDatasheetDescription,
        ///The type of datasheet that will be generated using this code.
        pub datasheet_type: DatasheetConfigurationSummaryModelDatasheetType,
    }
    impl ::std::convert::From<&DatasheetConfigurationSummaryModel>
    for DatasheetConfigurationSummaryModel {
        fn from(value: &DatasheetConfigurationSummaryModel) -> Self {
            value.clone()
        }
    }
    impl DatasheetConfigurationSummaryModel {
        pub fn builder() -> builder::DatasheetConfigurationSummaryModel {
            Default::default()
        }
    }
    ///The code used to specify the datasheet download.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code used to specify the datasheet download.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DatasheetConfigurationSummaryModelDatasheetCode(::std::string::String);
    impl ::std::ops::Deref for DatasheetConfigurationSummaryModelDatasheetCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<DatasheetConfigurationSummaryModelDatasheetCode>
    for ::std::string::String {
        fn from(value: DatasheetConfigurationSummaryModelDatasheetCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&DatasheetConfigurationSummaryModelDatasheetCode>
    for DatasheetConfigurationSummaryModelDatasheetCode {
        fn from(value: &DatasheetConfigurationSummaryModelDatasheetCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for DatasheetConfigurationSummaryModelDatasheetCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str>
    for DatasheetConfigurationSummaryModelDatasheetCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for DatasheetConfigurationSummaryModelDatasheetCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for DatasheetConfigurationSummaryModelDatasheetCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de>
    for DatasheetConfigurationSummaryModelDatasheetCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The description of the datasheet that will be generated using this code.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The description of the datasheet that will be generated using this code.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DatasheetConfigurationSummaryModelDatasheetDescription(
        ::std::string::String,
    );
    impl ::std::ops::Deref for DatasheetConfigurationSummaryModelDatasheetDescription {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<DatasheetConfigurationSummaryModelDatasheetDescription>
    for ::std::string::String {
        fn from(value: DatasheetConfigurationSummaryModelDatasheetDescription) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&DatasheetConfigurationSummaryModelDatasheetDescription>
    for DatasheetConfigurationSummaryModelDatasheetDescription {
        fn from(value: &DatasheetConfigurationSummaryModelDatasheetDescription) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for DatasheetConfigurationSummaryModelDatasheetDescription {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str>
    for DatasheetConfigurationSummaryModelDatasheetDescription {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for DatasheetConfigurationSummaryModelDatasheetDescription {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for DatasheetConfigurationSummaryModelDatasheetDescription {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de>
    for DatasheetConfigurationSummaryModelDatasheetDescription {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The type of datasheet that will be generated using this code.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The type of datasheet that will be generated using this code.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct DatasheetConfigurationSummaryModelDatasheetType(::std::string::String);
    impl ::std::ops::Deref for DatasheetConfigurationSummaryModelDatasheetType {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<DatasheetConfigurationSummaryModelDatasheetType>
    for ::std::string::String {
        fn from(value: DatasheetConfigurationSummaryModelDatasheetType) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&DatasheetConfigurationSummaryModelDatasheetType>
    for DatasheetConfigurationSummaryModelDatasheetType {
        fn from(value: &DatasheetConfigurationSummaryModelDatasheetType) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for DatasheetConfigurationSummaryModelDatasheetType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str>
    for DatasheetConfigurationSummaryModelDatasheetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for DatasheetConfigurationSummaryModelDatasheetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for DatasheetConfigurationSummaryModelDatasheetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de>
    for DatasheetConfigurationSummaryModelDatasheetType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///`File`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct File {}
    impl ::std::convert::From<&File> for File {
        fn from(value: &File) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for File {
        fn default() -> Self {
            Self {}
        }
    }
    impl File {
        pub fn builder() -> builder::File {
            Default::default()
        }
    }
    ///A FormattedDataExtract item is a flattened structure which encapsulates a single DataItem and includes a formatted display value and the DataItem's associated attributes.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A FormattedDataExtract item is a flattened structure which encapsulates a single DataItem and includes a formatted display value and the DataItem's associated attributes.",
    ///  "type": "object",
    ///  "required": [
    ///    "data_period",
    ///    "data_period_type",
    ///    "data_set_id",
    ///    "measure_category_code",
    ///    "measure_category_name",
    ///    "measure_code",
    ///    "measure_name",
    ///    "private",
    ///    "reported_measure_category_code",
    ///    "reported_measure_category_name",
    ///    "reported_measure_category_type_code",
    ///    "reported_measure_category_type_name",
    ///    "reported_measure_code",
    ///    "reported_measure_name",
    ///    "reporting_end_date",
    ///    "reporting_start_date",
    ///    "reporting_unit_code",
    ///    "reporting_unit_name",
    ///    "reporting_unit_type_code",
    ///    "reporting_unit_type_name"
    ///  ],
    ///  "properties": {
    ///    "caveat": {
    ///      "description": "The Caveat symbols applicable to this DataItem. This will be null if there are no Caveats.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "caveat_codes": {
    ///      "description": "Semi-colon separated list of Caveat codes applicable to this DataItem. This will be null if there are no Caveats.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "caveat_footnotes": {
    ///      "description": "Semi-colon separated list of Caveat symbols and their explanatory footnote applicable to this DataItem. This will be null if there are no Caveats.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "data_period": {
    ///      "description": "A formatted display of the data period including the start and end dates.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "data_period_type": {
    ///      "description": "A description of the data period type, e.g. FinancialYear.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "data_set_caveat": {
    ///      "description": "The Caveat symbols applicable to the corresponding DataSet. This will be null if there are no Caveats.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "data_set_caveat_codes": {
    ///      "description": "Semi-colon separated list of Caveat codes applicable to the corresponding DataSet. This will be null if there are no Caveats.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "data_set_caveat_footnotes": {
    ///      "description": "Semi-colon separated list of Caveat symbols and their explanatory footnote applicable to the corresponding DataSet. This will be null if there are no Caveats.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "data_set_id": {
    ///      "description": "The id for the corresponding DataSet.",
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "data_set_meta_tag_codes": {
    ///      "description": "Semi-colon separated list of MetaTag codes for the corresponding DataSet. This will be null if there are no DataSet MetaTags.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "data_set_meta_tags": {
    ///      "description": "Semi-colon separated list of MetaTag values for the corresponding DataSet. This will be null if there are no DataSet MetaTags.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "data_set_meta_tags_with_type": {
    ///      "description": "Semi-colon separated list of MetaTag values for the corresponding DataSet. Each MetaTag value matches the pattern MetaTagTypeCode|MetaTagValue. This will be null if there are no DataSet MetaTags.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "formatted_peer_value": {
    ///      "description": "A formatted display of the peer data value including any caveats and units. This will be null if there is no peer.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "formatted_value": {
    ///      "description": "A formatted display of the data value including any caveats and units.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "group_number": {
    ///      "description": "Indicates a quintile, decile or other grouping number for the data value. This will be null if it is not applicable.",
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32"
    ///    },
    ///    "mapped_local_hospital_network": {
    ///      "description": "The LHN name for the corresponding ReportingUnit's associated LHN. This will be null if the ReportingUnit has no mapped LHN.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "mapped_primary_health_network": {
    ///      "description": "The PHN name for the corresponding ReportingUnit's associated PHN. This will be null if the ReportingUnit has no mapped PHN.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "mapped_state": {
    ///      "description": "The state abbreviation of the corresponding ReportingUnit's associated state.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "measure_category_code": {
    ///      "description": "The code for the associated MeasureCategory.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "measure_category_name": {
    ///      "description": "The name for the associated MeasureCategory.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "measure_code": {
    ///      "description": "The code for the associated Measure.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "measure_meta_tag_codes": {
    ///      "description": "Semi-colon separated list of MetaTag codes for the corresponding Measure. This will be null if there are no Measure MetaTags.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "measure_meta_tags": {
    ///      "description": "Semi-colon separated list of MetaTag values for the corresponding Measure. This will be null if there are no Measure MetaTags.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "measure_meta_tags_with_type": {
    ///      "description": "Semi-colon separated list of MetaTag values for the corresponding Measure. Each MetaTag value matches the pattern MetaTagTypeCode|MetaTagValue. This will be null if there are no Measure MetaTags.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "measure_name": {
    ///      "description": "The name for the associated Measure.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "peer_group_code": {
    ///      "description": "The code for the associated PeerGroup. This will be null if there is no peer.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "peer_group_name": {
    ///      "description": "The name for the associated PeerGroup. This will be null if there is no peer.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "private": {
    ///      "description": "Indicates whether this ReportingUnit is private, e.g. a private hospital.",
    ///      "type": "boolean"
    ///    },
    ///    "proxy_reporting_unit_name": {
    ///      "description": "The name of the proxy reporting unit that reports on behalf of the associated ReportingUnit. This will be null if there is no proxy reporting.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "raw_lower_value": {
    ///      "description": "Represents the lower data value if this DataItem is a data range or has a confidence interval, otherwise this will be null.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "raw_peer_value": {
    ///      "description": "The peer data value for this DataItem. This may be null if the peer data is suppressed or there is no peer.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "raw_upper_value": {
    ///      "description": "Represents the upper data value if this DataItem is a data range or has a confidence interval, otherwise this will be null.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "raw_value": {
    ///      "description": "The data value for this item. This data value may be null if the data is suppressed, in which case a caveat will be present in the Caveats list.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "reported_measure_category_code": {
    ///      "description": "The code for the first ReportedMeasure category.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reported_measure_category_name": {
    ///      "description": "The name for the first ReportedMeasure category.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reported_measure_category_three_code": {
    ///      "description": "The code for the third ReportedMeasure category. This will be null if the ReportedMeasure does not have 3 categories.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reported_measure_category_three_name": {
    ///      "description": "The name for the third ReportedMeasure category. This will be null if the ReportedMeasure does not have 3 categories.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reported_measure_category_three_type_code": {
    ///      "description": "The code for the third ReportedMeasure category type. This will be null if the ReportedMeasure does not have 3 categories.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reported_measure_category_three_type_name": {
    ///      "description": "The name for the third ReportedMeasure category type. This will be null if the ReportedMeasure does not have 3 categories.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reported_measure_category_two_code": {
    ///      "description": "The code for the second ReportedMeasure category. This will be null if the ReportedMeasure does not have 2 categories.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reported_measure_category_two_name": {
    ///      "description": "The name for the second ReportedMeasure category. This will be null if the ReportedMeasure does not have 2 categories.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reported_measure_category_two_type_code": {
    ///      "description": "The code for the second ReportedMeasure category type. This will be null if the ReportedMeasure does not have 2 categories.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reported_measure_category_two_type_name": {
    ///      "description": "The name for the second ReportedMeasure category type. This will be null if the ReportedMeasure does not have 2 categories.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reported_measure_category_type_code": {
    ///      "description": "The code for the first ReportedMeasure category.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reported_measure_category_type_name": {
    ///      "description": "The name for the first ReportedMeasure category type.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reported_measure_code": {
    ///      "description": "The code for the associated ReportedMeasure.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reported_measure_meta_tag_codes": {
    ///      "description": "Semi-colon separated list of MetaTag codes for the corresponding ReportedMeasure. This will be null if there are no ReportedMeasure MetaTags.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reported_measure_meta_tags": {
    ///      "description": "Semi-colon separated list of MetaTag values for the corresponding ReportedMeasure. This will be null if there are no ReportedMeasure MetaTags.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reported_measure_meta_tags_with_type": {
    ///      "description": "Semi-colon separated list of MetaTag values for the corresponding ReportedMeasure. Each MetaTag value matches the pattern MetaTagTypeCode|MetaTagValue. This will be null if there are no ReportedMeasure MetaTags.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reported_measure_name": {
    ///      "description": "The name for the associated ReportedMeasure.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reporting_end_date": {
    ///      "description": "The end date for the DataSet data period.",
    ///      "type": "string",
    ///      "format": "date"
    ///    },
    ///    "reporting_start_date": {
    ///      "description": "The start date for the DataSet data period.",
    ///      "type": "string",
    ///      "format": "date"
    ///    },
    ///    "reporting_unit_code": {
    ///      "description": "The code for the associated ReportingUnit.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reporting_unit_meta_tag_codes": {
    ///      "description": "Semi-colon separated list of MetaTag codes for the corresponding ReportingUnit. This will be null if there are no ReportingUnit MetaTags.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reporting_unit_meta_tags": {
    ///      "description": "Semi-colon separated list of MetaTag values for the corresponding ReportingUnit. This will be null if there are no ReportingUnit MetaTags.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reporting_unit_meta_tags_with_type": {
    ///      "description": "Semi-colon separated list of MetaTag values for the corresponding ReportingUnit. Each MetaTag value matches the pattern MetaTagTypeCode|MetaTagValue. This will be null if there are no ReportingUnit MetaTags.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reporting_unit_name": {
    ///      "description": "The name for the associated ReportingUnit.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reporting_unit_type_code": {
    ///      "description": "The code for the associated ReportingUnitType.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reporting_unit_type_name": {
    ///      "description": "The name for the associated ReportingUnitType.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "suppression": {
    ///      "description": "The Suppression symbols applicable to this DataItem. This will be null if there are no Suppressions.",
    ///      "deprecated": true,
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "suppression_codes": {
    ///      "description": "Semi-colon separated list of Suppression codes applicable to this DataItem. This will be null if there are no Suppressions.",
    ///      "deprecated": true,
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct FormattedDataExtractModel {
        ///The Caveat symbols applicable to this DataItem. This will be null if there are no Caveats.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub caveat: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of Caveat codes applicable to this DataItem. This will be null if there are no Caveats.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub caveat_codes: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of Caveat symbols and their explanatory footnote applicable to this DataItem. This will be null if there are no Caveats.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub caveat_footnotes: ::std::option::Option<::std::string::String>,
        ///A formatted display of the data period including the start and end dates.
        pub data_period: FormattedDataExtractModelDataPeriod,
        ///A description of the data period type, e.g. FinancialYear.
        pub data_period_type: FormattedDataExtractModelDataPeriodType,
        ///The Caveat symbols applicable to the corresponding DataSet. This will be null if there are no Caveats.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data_set_caveat: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of Caveat codes applicable to the corresponding DataSet. This will be null if there are no Caveats.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data_set_caveat_codes: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of Caveat symbols and their explanatory footnote applicable to the corresponding DataSet. This will be null if there are no Caveats.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data_set_caveat_footnotes: ::std::option::Option<::std::string::String>,
        ///The id for the corresponding DataSet.
        pub data_set_id: i32,
        ///Semi-colon separated list of MetaTag codes for the corresponding DataSet. This will be null if there are no DataSet MetaTags.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data_set_meta_tag_codes: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of MetaTag values for the corresponding DataSet. This will be null if there are no DataSet MetaTags.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data_set_meta_tags: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of MetaTag values for the corresponding DataSet. Each MetaTag value matches the pattern MetaTagTypeCode|MetaTagValue. This will be null if there are no DataSet MetaTags.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data_set_meta_tags_with_type: ::std::option::Option<::std::string::String>,
        ///A formatted display of the peer data value including any caveats and units. This will be null if there is no peer.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub formatted_peer_value: ::std::option::Option<::std::string::String>,
        ///A formatted display of the data value including any caveats and units.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub formatted_value: ::std::option::Option<::std::string::String>,
        ///Indicates a quintile, decile or other grouping number for the data value. This will be null if it is not applicable.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub group_number: ::std::option::Option<i32>,
        ///The LHN name for the corresponding ReportingUnit's associated LHN. This will be null if the ReportingUnit has no mapped LHN.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mapped_local_hospital_network: ::std::option::Option<::std::string::String>,
        ///The PHN name for the corresponding ReportingUnit's associated PHN. This will be null if the ReportingUnit has no mapped PHN.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mapped_primary_health_network: ::std::option::Option<::std::string::String>,
        ///The state abbreviation of the corresponding ReportingUnit's associated state.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mapped_state: ::std::option::Option<::std::string::String>,
        ///The code for the associated MeasureCategory.
        pub measure_category_code: FormattedDataExtractModelMeasureCategoryCode,
        ///The name for the associated MeasureCategory.
        pub measure_category_name: FormattedDataExtractModelMeasureCategoryName,
        ///The code for the associated Measure.
        pub measure_code: FormattedDataExtractModelMeasureCode,
        ///Semi-colon separated list of MetaTag codes for the corresponding Measure. This will be null if there are no Measure MetaTags.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub measure_meta_tag_codes: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of MetaTag values for the corresponding Measure. This will be null if there are no Measure MetaTags.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub measure_meta_tags: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of MetaTag values for the corresponding Measure. Each MetaTag value matches the pattern MetaTagTypeCode|MetaTagValue. This will be null if there are no Measure MetaTags.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub measure_meta_tags_with_type: ::std::option::Option<::std::string::String>,
        ///The name for the associated Measure.
        pub measure_name: FormattedDataExtractModelMeasureName,
        ///The code for the associated PeerGroup. This will be null if there is no peer.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub peer_group_code: ::std::option::Option<::std::string::String>,
        ///The name for the associated PeerGroup. This will be null if there is no peer.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub peer_group_name: ::std::option::Option<::std::string::String>,
        ///Indicates whether this ReportingUnit is private, e.g. a private hospital.
        pub private: bool,
        ///The name of the proxy reporting unit that reports on behalf of the associated ReportingUnit. This will be null if there is no proxy reporting.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub proxy_reporting_unit_name: ::std::option::Option<::std::string::String>,
        ///Represents the lower data value if this DataItem is a data range or has a confidence interval, otherwise this will be null.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub raw_lower_value: ::std::option::Option<f64>,
        ///The peer data value for this DataItem. This may be null if the peer data is suppressed or there is no peer.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub raw_peer_value: ::std::option::Option<f64>,
        ///Represents the upper data value if this DataItem is a data range or has a confidence interval, otherwise this will be null.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub raw_upper_value: ::std::option::Option<f64>,
        ///The data value for this item. This data value may be null if the data is suppressed, in which case a caveat will be present in the Caveats list.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub raw_value: ::std::option::Option<f64>,
        ///The code for the first ReportedMeasure category.
        pub reported_measure_category_code: FormattedDataExtractModelReportedMeasureCategoryCode,
        ///The name for the first ReportedMeasure category.
        pub reported_measure_category_name: FormattedDataExtractModelReportedMeasureCategoryName,
        ///The code for the third ReportedMeasure category. This will be null if the ReportedMeasure does not have 3 categories.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reported_measure_category_three_code: ::std::option::Option<
            ::std::string::String,
        >,
        ///The name for the third ReportedMeasure category. This will be null if the ReportedMeasure does not have 3 categories.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reported_measure_category_three_name: ::std::option::Option<
            ::std::string::String,
        >,
        ///The code for the third ReportedMeasure category type. This will be null if the ReportedMeasure does not have 3 categories.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reported_measure_category_three_type_code: ::std::option::Option<
            ::std::string::String,
        >,
        ///The name for the third ReportedMeasure category type. This will be null if the ReportedMeasure does not have 3 categories.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reported_measure_category_three_type_name: ::std::option::Option<
            ::std::string::String,
        >,
        ///The code for the second ReportedMeasure category. This will be null if the ReportedMeasure does not have 2 categories.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reported_measure_category_two_code: ::std::option::Option<
            ::std::string::String,
        >,
        ///The name for the second ReportedMeasure category. This will be null if the ReportedMeasure does not have 2 categories.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reported_measure_category_two_name: ::std::option::Option<
            ::std::string::String,
        >,
        ///The code for the second ReportedMeasure category type. This will be null if the ReportedMeasure does not have 2 categories.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reported_measure_category_two_type_code: ::std::option::Option<
            ::std::string::String,
        >,
        ///The name for the second ReportedMeasure category type. This will be null if the ReportedMeasure does not have 2 categories.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reported_measure_category_two_type_name: ::std::option::Option<
            ::std::string::String,
        >,
        ///The code for the first ReportedMeasure category.
        pub reported_measure_category_type_code: FormattedDataExtractModelReportedMeasureCategoryTypeCode,
        ///The name for the first ReportedMeasure category type.
        pub reported_measure_category_type_name: FormattedDataExtractModelReportedMeasureCategoryTypeName,
        ///The code for the associated ReportedMeasure.
        pub reported_measure_code: FormattedDataExtractModelReportedMeasureCode,
        ///Semi-colon separated list of MetaTag codes for the corresponding ReportedMeasure. This will be null if there are no ReportedMeasure MetaTags.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reported_measure_meta_tag_codes: ::std::option::Option<
            ::std::string::String,
        >,
        ///Semi-colon separated list of MetaTag values for the corresponding ReportedMeasure. This will be null if there are no ReportedMeasure MetaTags.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reported_measure_meta_tags: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of MetaTag values for the corresponding ReportedMeasure. Each MetaTag value matches the pattern MetaTagTypeCode|MetaTagValue. This will be null if there are no ReportedMeasure MetaTags.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reported_measure_meta_tags_with_type: ::std::option::Option<
            ::std::string::String,
        >,
        ///The name for the associated ReportedMeasure.
        pub reported_measure_name: FormattedDataExtractModelReportedMeasureName,
        ///The end date for the DataSet data period.
        pub reporting_end_date: ::chrono::naive::NaiveDate,
        ///The start date for the DataSet data period.
        pub reporting_start_date: ::chrono::naive::NaiveDate,
        ///The code for the associated ReportingUnit.
        pub reporting_unit_code: FormattedDataExtractModelReportingUnitCode,
        ///Semi-colon separated list of MetaTag codes for the corresponding ReportingUnit. This will be null if there are no ReportingUnit MetaTags.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reporting_unit_meta_tag_codes: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of MetaTag values for the corresponding ReportingUnit. This will be null if there are no ReportingUnit MetaTags.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reporting_unit_meta_tags: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of MetaTag values for the corresponding ReportingUnit. Each MetaTag value matches the pattern MetaTagTypeCode|MetaTagValue. This will be null if there are no ReportingUnit MetaTags.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reporting_unit_meta_tags_with_type: ::std::option::Option<
            ::std::string::String,
        >,
        ///The name for the associated ReportingUnit.
        pub reporting_unit_name: FormattedDataExtractModelReportingUnitName,
        ///The code for the associated ReportingUnitType.
        pub reporting_unit_type_code: FormattedDataExtractModelReportingUnitTypeCode,
        ///The name for the associated ReportingUnitType.
        pub reporting_unit_type_name: FormattedDataExtractModelReportingUnitTypeName,
        ///The Suppression symbols applicable to this DataItem. This will be null if there are no Suppressions.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub suppression: ::std::option::Option<::std::string::String>,
        ///Semi-colon separated list of Suppression codes applicable to this DataItem. This will be null if there are no Suppressions.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub suppression_codes: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&FormattedDataExtractModel> for FormattedDataExtractModel {
        fn from(value: &FormattedDataExtractModel) -> Self {
            value.clone()
        }
    }
    impl FormattedDataExtractModel {
        pub fn builder() -> builder::FormattedDataExtractModel {
            Default::default()
        }
    }
    ///A formatted display of the data period including the start and end dates.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A formatted display of the data period including the start and end dates.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct FormattedDataExtractModelDataPeriod(::std::string::String);
    impl ::std::ops::Deref for FormattedDataExtractModelDataPeriod {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<FormattedDataExtractModelDataPeriod>
    for ::std::string::String {
        fn from(value: FormattedDataExtractModelDataPeriod) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&FormattedDataExtractModelDataPeriod>
    for FormattedDataExtractModelDataPeriod {
        fn from(value: &FormattedDataExtractModelDataPeriod) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for FormattedDataExtractModelDataPeriod {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for FormattedDataExtractModelDataPeriod {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for FormattedDataExtractModelDataPeriod {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for FormattedDataExtractModelDataPeriod {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FormattedDataExtractModelDataPeriod {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///A description of the data period type, e.g. FinancialYear.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A description of the data period type, e.g. FinancialYear.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct FormattedDataExtractModelDataPeriodType(::std::string::String);
    impl ::std::ops::Deref for FormattedDataExtractModelDataPeriodType {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<FormattedDataExtractModelDataPeriodType>
    for ::std::string::String {
        fn from(value: FormattedDataExtractModelDataPeriodType) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&FormattedDataExtractModelDataPeriodType>
    for FormattedDataExtractModelDataPeriodType {
        fn from(value: &FormattedDataExtractModelDataPeriodType) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for FormattedDataExtractModelDataPeriodType {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for FormattedDataExtractModelDataPeriodType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for FormattedDataExtractModelDataPeriodType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for FormattedDataExtractModelDataPeriodType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FormattedDataExtractModelDataPeriodType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The code for the associated MeasureCategory.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the associated MeasureCategory.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct FormattedDataExtractModelMeasureCategoryCode(::std::string::String);
    impl ::std::ops::Deref for FormattedDataExtractModelMeasureCategoryCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<FormattedDataExtractModelMeasureCategoryCode>
    for ::std::string::String {
        fn from(value: FormattedDataExtractModelMeasureCategoryCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&FormattedDataExtractModelMeasureCategoryCode>
    for FormattedDataExtractModelMeasureCategoryCode {
        fn from(value: &FormattedDataExtractModelMeasureCategoryCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for FormattedDataExtractModelMeasureCategoryCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for FormattedDataExtractModelMeasureCategoryCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for FormattedDataExtractModelMeasureCategoryCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for FormattedDataExtractModelMeasureCategoryCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de>
    for FormattedDataExtractModelMeasureCategoryCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the associated MeasureCategory.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the associated MeasureCategory.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct FormattedDataExtractModelMeasureCategoryName(::std::string::String);
    impl ::std::ops::Deref for FormattedDataExtractModelMeasureCategoryName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<FormattedDataExtractModelMeasureCategoryName>
    for ::std::string::String {
        fn from(value: FormattedDataExtractModelMeasureCategoryName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&FormattedDataExtractModelMeasureCategoryName>
    for FormattedDataExtractModelMeasureCategoryName {
        fn from(value: &FormattedDataExtractModelMeasureCategoryName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for FormattedDataExtractModelMeasureCategoryName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for FormattedDataExtractModelMeasureCategoryName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for FormattedDataExtractModelMeasureCategoryName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for FormattedDataExtractModelMeasureCategoryName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de>
    for FormattedDataExtractModelMeasureCategoryName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The code for the associated Measure.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the associated Measure.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct FormattedDataExtractModelMeasureCode(::std::string::String);
    impl ::std::ops::Deref for FormattedDataExtractModelMeasureCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<FormattedDataExtractModelMeasureCode>
    for ::std::string::String {
        fn from(value: FormattedDataExtractModelMeasureCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&FormattedDataExtractModelMeasureCode>
    for FormattedDataExtractModelMeasureCode {
        fn from(value: &FormattedDataExtractModelMeasureCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for FormattedDataExtractModelMeasureCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for FormattedDataExtractModelMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for FormattedDataExtractModelMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for FormattedDataExtractModelMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FormattedDataExtractModelMeasureCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the associated Measure.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the associated Measure.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct FormattedDataExtractModelMeasureName(::std::string::String);
    impl ::std::ops::Deref for FormattedDataExtractModelMeasureName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<FormattedDataExtractModelMeasureName>
    for ::std::string::String {
        fn from(value: FormattedDataExtractModelMeasureName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&FormattedDataExtractModelMeasureName>
    for FormattedDataExtractModelMeasureName {
        fn from(value: &FormattedDataExtractModelMeasureName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for FormattedDataExtractModelMeasureName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for FormattedDataExtractModelMeasureName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for FormattedDataExtractModelMeasureName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for FormattedDataExtractModelMeasureName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FormattedDataExtractModelMeasureName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The code for the first ReportedMeasure category.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the first ReportedMeasure category.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct FormattedDataExtractModelReportedMeasureCategoryCode(
        ::std::string::String,
    );
    impl ::std::ops::Deref for FormattedDataExtractModelReportedMeasureCategoryCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<FormattedDataExtractModelReportedMeasureCategoryCode>
    for ::std::string::String {
        fn from(value: FormattedDataExtractModelReportedMeasureCategoryCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&FormattedDataExtractModelReportedMeasureCategoryCode>
    for FormattedDataExtractModelReportedMeasureCategoryCode {
        fn from(value: &FormattedDataExtractModelReportedMeasureCategoryCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for FormattedDataExtractModelReportedMeasureCategoryCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str>
    for FormattedDataExtractModelReportedMeasureCategoryCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for FormattedDataExtractModelReportedMeasureCategoryCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for FormattedDataExtractModelReportedMeasureCategoryCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de>
    for FormattedDataExtractModelReportedMeasureCategoryCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the first ReportedMeasure category.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the first ReportedMeasure category.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct FormattedDataExtractModelReportedMeasureCategoryName(
        ::std::string::String,
    );
    impl ::std::ops::Deref for FormattedDataExtractModelReportedMeasureCategoryName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<FormattedDataExtractModelReportedMeasureCategoryName>
    for ::std::string::String {
        fn from(value: FormattedDataExtractModelReportedMeasureCategoryName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&FormattedDataExtractModelReportedMeasureCategoryName>
    for FormattedDataExtractModelReportedMeasureCategoryName {
        fn from(value: &FormattedDataExtractModelReportedMeasureCategoryName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for FormattedDataExtractModelReportedMeasureCategoryName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str>
    for FormattedDataExtractModelReportedMeasureCategoryName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for FormattedDataExtractModelReportedMeasureCategoryName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for FormattedDataExtractModelReportedMeasureCategoryName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de>
    for FormattedDataExtractModelReportedMeasureCategoryName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The code for the first ReportedMeasure category.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the first ReportedMeasure category.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct FormattedDataExtractModelReportedMeasureCategoryTypeCode(
        ::std::string::String,
    );
    impl ::std::ops::Deref for FormattedDataExtractModelReportedMeasureCategoryTypeCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<FormattedDataExtractModelReportedMeasureCategoryTypeCode>
    for ::std::string::String {
        fn from(
            value: FormattedDataExtractModelReportedMeasureCategoryTypeCode,
        ) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&FormattedDataExtractModelReportedMeasureCategoryTypeCode>
    for FormattedDataExtractModelReportedMeasureCategoryTypeCode {
        fn from(
            value: &FormattedDataExtractModelReportedMeasureCategoryTypeCode,
        ) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr
    for FormattedDataExtractModelReportedMeasureCategoryTypeCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str>
    for FormattedDataExtractModelReportedMeasureCategoryTypeCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for FormattedDataExtractModelReportedMeasureCategoryTypeCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for FormattedDataExtractModelReportedMeasureCategoryTypeCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de>
    for FormattedDataExtractModelReportedMeasureCategoryTypeCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the first ReportedMeasure category type.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the first ReportedMeasure category type.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct FormattedDataExtractModelReportedMeasureCategoryTypeName(
        ::std::string::String,
    );
    impl ::std::ops::Deref for FormattedDataExtractModelReportedMeasureCategoryTypeName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<FormattedDataExtractModelReportedMeasureCategoryTypeName>
    for ::std::string::String {
        fn from(
            value: FormattedDataExtractModelReportedMeasureCategoryTypeName,
        ) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&FormattedDataExtractModelReportedMeasureCategoryTypeName>
    for FormattedDataExtractModelReportedMeasureCategoryTypeName {
        fn from(
            value: &FormattedDataExtractModelReportedMeasureCategoryTypeName,
        ) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr
    for FormattedDataExtractModelReportedMeasureCategoryTypeName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str>
    for FormattedDataExtractModelReportedMeasureCategoryTypeName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for FormattedDataExtractModelReportedMeasureCategoryTypeName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for FormattedDataExtractModelReportedMeasureCategoryTypeName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de>
    for FormattedDataExtractModelReportedMeasureCategoryTypeName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The code for the associated ReportedMeasure.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the associated ReportedMeasure.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct FormattedDataExtractModelReportedMeasureCode(::std::string::String);
    impl ::std::ops::Deref for FormattedDataExtractModelReportedMeasureCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<FormattedDataExtractModelReportedMeasureCode>
    for ::std::string::String {
        fn from(value: FormattedDataExtractModelReportedMeasureCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&FormattedDataExtractModelReportedMeasureCode>
    for FormattedDataExtractModelReportedMeasureCode {
        fn from(value: &FormattedDataExtractModelReportedMeasureCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for FormattedDataExtractModelReportedMeasureCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for FormattedDataExtractModelReportedMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for FormattedDataExtractModelReportedMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for FormattedDataExtractModelReportedMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de>
    for FormattedDataExtractModelReportedMeasureCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the associated ReportedMeasure.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the associated ReportedMeasure.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct FormattedDataExtractModelReportedMeasureName(::std::string::String);
    impl ::std::ops::Deref for FormattedDataExtractModelReportedMeasureName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<FormattedDataExtractModelReportedMeasureName>
    for ::std::string::String {
        fn from(value: FormattedDataExtractModelReportedMeasureName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&FormattedDataExtractModelReportedMeasureName>
    for FormattedDataExtractModelReportedMeasureName {
        fn from(value: &FormattedDataExtractModelReportedMeasureName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for FormattedDataExtractModelReportedMeasureName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for FormattedDataExtractModelReportedMeasureName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for FormattedDataExtractModelReportedMeasureName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for FormattedDataExtractModelReportedMeasureName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de>
    for FormattedDataExtractModelReportedMeasureName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The code for the associated ReportingUnit.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the associated ReportingUnit.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct FormattedDataExtractModelReportingUnitCode(::std::string::String);
    impl ::std::ops::Deref for FormattedDataExtractModelReportingUnitCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<FormattedDataExtractModelReportingUnitCode>
    for ::std::string::String {
        fn from(value: FormattedDataExtractModelReportingUnitCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&FormattedDataExtractModelReportingUnitCode>
    for FormattedDataExtractModelReportingUnitCode {
        fn from(value: &FormattedDataExtractModelReportingUnitCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for FormattedDataExtractModelReportingUnitCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for FormattedDataExtractModelReportingUnitCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for FormattedDataExtractModelReportingUnitCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for FormattedDataExtractModelReportingUnitCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FormattedDataExtractModelReportingUnitCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the associated ReportingUnit.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the associated ReportingUnit.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct FormattedDataExtractModelReportingUnitName(::std::string::String);
    impl ::std::ops::Deref for FormattedDataExtractModelReportingUnitName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<FormattedDataExtractModelReportingUnitName>
    for ::std::string::String {
        fn from(value: FormattedDataExtractModelReportingUnitName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&FormattedDataExtractModelReportingUnitName>
    for FormattedDataExtractModelReportingUnitName {
        fn from(value: &FormattedDataExtractModelReportingUnitName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for FormattedDataExtractModelReportingUnitName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for FormattedDataExtractModelReportingUnitName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for FormattedDataExtractModelReportingUnitName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for FormattedDataExtractModelReportingUnitName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FormattedDataExtractModelReportingUnitName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The code for the associated ReportingUnitType.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the associated ReportingUnitType.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct FormattedDataExtractModelReportingUnitTypeCode(::std::string::String);
    impl ::std::ops::Deref for FormattedDataExtractModelReportingUnitTypeCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<FormattedDataExtractModelReportingUnitTypeCode>
    for ::std::string::String {
        fn from(value: FormattedDataExtractModelReportingUnitTypeCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&FormattedDataExtractModelReportingUnitTypeCode>
    for FormattedDataExtractModelReportingUnitTypeCode {
        fn from(value: &FormattedDataExtractModelReportingUnitTypeCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for FormattedDataExtractModelReportingUnitTypeCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str>
    for FormattedDataExtractModelReportingUnitTypeCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for FormattedDataExtractModelReportingUnitTypeCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for FormattedDataExtractModelReportingUnitTypeCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de>
    for FormattedDataExtractModelReportingUnitTypeCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the associated ReportingUnitType.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the associated ReportingUnitType.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct FormattedDataExtractModelReportingUnitTypeName(::std::string::String);
    impl ::std::ops::Deref for FormattedDataExtractModelReportingUnitTypeName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<FormattedDataExtractModelReportingUnitTypeName>
    for ::std::string::String {
        fn from(value: FormattedDataExtractModelReportingUnitTypeName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&FormattedDataExtractModelReportingUnitTypeName>
    for FormattedDataExtractModelReportingUnitTypeName {
        fn from(value: &FormattedDataExtractModelReportingUnitTypeName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for FormattedDataExtractModelReportingUnitTypeName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str>
    for FormattedDataExtractModelReportingUnitTypeName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for FormattedDataExtractModelReportingUnitTypeName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for FormattedDataExtractModelReportingUnitTypeName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de>
    for FormattedDataExtractModelReportingUnitTypeName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///`GetCaveatsByCaveatCodeResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "$ref": "#/components/schemas/CaveatModel"
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetCaveatsByCaveatCodeResponse {
        pub result: CaveatModel,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetCaveatsByCaveatCodeResponse>
    for GetCaveatsByCaveatCodeResponse {
        fn from(value: &GetCaveatsByCaveatCodeResponse) -> Self {
            value.clone()
        }
    }
    impl GetCaveatsByCaveatCodeResponse {
        pub fn builder() -> builder::GetCaveatsByCaveatCodeResponse {
            Default::default()
        }
    }
    ///`GetCaveatsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/CaveatModel"
    ///      }
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetCaveatsResponse {
        pub result: ::std::vec::Vec<CaveatModel>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetCaveatsResponse> for GetCaveatsResponse {
        fn from(value: &GetCaveatsResponse) -> Self {
            value.clone()
        }
    }
    impl GetCaveatsResponse {
        pub fn builder() -> builder::GetCaveatsResponse {
            Default::default()
        }
    }
    ///`GetDatasetsByDatasetIdDataItemsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "$ref": "#/components/schemas/DataItemModel"
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetDatasetsByDatasetIdDataItemsResponse {
        pub result: DataItemModel,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetDatasetsByDatasetIdDataItemsResponse>
    for GetDatasetsByDatasetIdDataItemsResponse {
        fn from(value: &GetDatasetsByDatasetIdDataItemsResponse) -> Self {
            value.clone()
        }
    }
    impl GetDatasetsByDatasetIdDataItemsResponse {
        pub fn builder() -> builder::GetDatasetsByDatasetIdDataItemsResponse {
            Default::default()
        }
    }
    ///`GetDatasetsByDatasetIdResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "$ref": "#/components/schemas/DataSetModel"
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetDatasetsByDatasetIdResponse {
        pub result: DataSetModel,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetDatasetsByDatasetIdResponse>
    for GetDatasetsByDatasetIdResponse {
        fn from(value: &GetDatasetsByDatasetIdResponse) -> Self {
            value.clone()
        }
    }
    impl GetDatasetsByDatasetIdResponse {
        pub fn builder() -> builder::GetDatasetsByDatasetIdResponse {
            Default::default()
        }
    }
    ///`GetDatasetsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/DataSetModel"
    ///      }
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetDatasetsResponse {
        pub result: ::std::vec::Vec<DataSetModel>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetDatasetsResponse> for GetDatasetsResponse {
        fn from(value: &GetDatasetsResponse) -> Self {
            value.clone()
        }
    }
    impl GetDatasetsResponse {
        pub fn builder() -> builder::GetDatasetsResponse {
            Default::default()
        }
    }
    ///`GetFlatDataExtractByMeasureCategoryCodeResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "$ref": "#/components/schemas/PaginatedDataExtractModel"
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetFlatDataExtractByMeasureCategoryCodeResponse {
        pub result: PaginatedDataExtractModel,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetFlatDataExtractByMeasureCategoryCodeResponse>
    for GetFlatDataExtractByMeasureCategoryCodeResponse {
        fn from(value: &GetFlatDataExtractByMeasureCategoryCodeResponse) -> Self {
            value.clone()
        }
    }
    impl GetFlatDataExtractByMeasureCategoryCodeResponse {
        pub fn builder() -> builder::GetFlatDataExtractByMeasureCategoryCodeResponse {
            Default::default()
        }
    }
    ///`GetFlatFormattedDataExtractByMeasureCategoryCodeResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "$ref": "#/components/schemas/PaginatedFormattedDataExtractModel"
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetFlatFormattedDataExtractByMeasureCategoryCodeResponse {
        pub result: PaginatedFormattedDataExtractModel,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetFlatFormattedDataExtractByMeasureCategoryCodeResponse>
    for GetFlatFormattedDataExtractByMeasureCategoryCodeResponse {
        fn from(
            value: &GetFlatFormattedDataExtractByMeasureCategoryCodeResponse,
        ) -> Self {
            value.clone()
        }
    }
    impl GetFlatFormattedDataExtractByMeasureCategoryCodeResponse {
        pub fn builder() -> builder::GetFlatFormattedDataExtractByMeasureCategoryCodeResponse {
            Default::default()
        }
    }
    ///`GetMeasureCategoriesByMeasureCategoryCodeMeasuresResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MeasureModel"
    ///      }
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetMeasureCategoriesByMeasureCategoryCodeMeasuresResponse {
        pub result: ::std::vec::Vec<MeasureModel>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetMeasureCategoriesByMeasureCategoryCodeMeasuresResponse>
    for GetMeasureCategoriesByMeasureCategoryCodeMeasuresResponse {
        fn from(
            value: &GetMeasureCategoriesByMeasureCategoryCodeMeasuresResponse,
        ) -> Self {
            value.clone()
        }
    }
    impl GetMeasureCategoriesByMeasureCategoryCodeMeasuresResponse {
        pub fn builder() -> builder::GetMeasureCategoriesByMeasureCategoryCodeMeasuresResponse {
            Default::default()
        }
    }
    ///`GetMeasureCategoriesByMeasureCategoryCodeResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "$ref": "#/components/schemas/MeasureCategoryModel"
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetMeasureCategoriesByMeasureCategoryCodeResponse {
        pub result: MeasureCategoryModel,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetMeasureCategoriesByMeasureCategoryCodeResponse>
    for GetMeasureCategoriesByMeasureCategoryCodeResponse {
        fn from(value: &GetMeasureCategoriesByMeasureCategoryCodeResponse) -> Self {
            value.clone()
        }
    }
    impl GetMeasureCategoriesByMeasureCategoryCodeResponse {
        pub fn builder() -> builder::GetMeasureCategoriesByMeasureCategoryCodeResponse {
            Default::default()
        }
    }
    ///`GetMeasureCategoriesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MeasureCategoryModel"
    ///      }
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetMeasureCategoriesResponse {
        pub result: ::std::vec::Vec<MeasureCategoryModel>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetMeasureCategoriesResponse>
    for GetMeasureCategoriesResponse {
        fn from(value: &GetMeasureCategoriesResponse) -> Self {
            value.clone()
        }
    }
    impl GetMeasureCategoriesResponse {
        pub fn builder() -> builder::GetMeasureCategoriesResponse {
            Default::default()
        }
    }
    ///`GetMeasureDownloadsMeasureDownloadCodesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "array",
    ///        "items": {
    ///          "$ref": "#/components/schemas/DatasheetConfigurationSummaryModel"
    ///        }
    ///      }
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetMeasureDownloadsMeasureDownloadCodesResponse {
        pub result: ::std::collections::HashMap<
            ::std::string::String,
            ::std::vec::Vec<DatasheetConfigurationSummaryModel>,
        >,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetMeasureDownloadsMeasureDownloadCodesResponse>
    for GetMeasureDownloadsMeasureDownloadCodesResponse {
        fn from(value: &GetMeasureDownloadsMeasureDownloadCodesResponse) -> Self {
            value.clone()
        }
    }
    impl GetMeasureDownloadsMeasureDownloadCodesResponse {
        pub fn builder() -> builder::GetMeasureDownloadsMeasureDownloadCodesResponse {
            Default::default()
        }
    }
    ///`GetMeasuresByMeasureCodeDataItemsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/DataItemModel"
    ///      }
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetMeasuresByMeasureCodeDataItemsResponse {
        pub result: ::std::vec::Vec<DataItemModel>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetMeasuresByMeasureCodeDataItemsResponse>
    for GetMeasuresByMeasureCodeDataItemsResponse {
        fn from(value: &GetMeasuresByMeasureCodeDataItemsResponse) -> Self {
            value.clone()
        }
    }
    impl GetMeasuresByMeasureCodeDataItemsResponse {
        pub fn builder() -> builder::GetMeasuresByMeasureCodeDataItemsResponse {
            Default::default()
        }
    }
    ///`GetMeasuresByMeasureCodeReportingUnitsAvailableResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ReportingUnitSummaryModel"
    ///      }
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetMeasuresByMeasureCodeReportingUnitsAvailableResponse {
        pub result: ::std::vec::Vec<ReportingUnitSummaryModel>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetMeasuresByMeasureCodeReportingUnitsAvailableResponse>
    for GetMeasuresByMeasureCodeReportingUnitsAvailableResponse {
        fn from(
            value: &GetMeasuresByMeasureCodeReportingUnitsAvailableResponse,
        ) -> Self {
            value.clone()
        }
    }
    impl GetMeasuresByMeasureCodeReportingUnitsAvailableResponse {
        pub fn builder() -> builder::GetMeasuresByMeasureCodeReportingUnitsAvailableResponse {
            Default::default()
        }
    }
    ///`GetMeasuresByMeasureCodeResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "$ref": "#/components/schemas/MeasureModel"
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetMeasuresByMeasureCodeResponse {
        pub result: MeasureModel,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetMeasuresByMeasureCodeResponse>
    for GetMeasuresByMeasureCodeResponse {
        fn from(value: &GetMeasuresByMeasureCodeResponse) -> Self {
            value.clone()
        }
    }
    impl GetMeasuresByMeasureCodeResponse {
        pub fn builder() -> builder::GetMeasuresByMeasureCodeResponse {
            Default::default()
        }
    }
    ///`GetMeasuresResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MeasureModel"
    ///      }
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetMeasuresResponse {
        pub result: ::std::vec::Vec<MeasureModel>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetMeasuresResponse> for GetMeasuresResponse {
        fn from(value: &GetMeasuresResponse) -> Self {
            value.clone()
        }
    }
    impl GetMeasuresResponse {
        pub fn builder() -> builder::GetMeasuresResponse {
            Default::default()
        }
    }
    ///`GetReportedMeasureCategoriesByReportedMeasureCategoryCodeReportedMeasuresResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ReportedMeasureModel"
    ///      }
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetReportedMeasureCategoriesByReportedMeasureCategoryCodeReportedMeasuresResponse {
        pub result: ::std::vec::Vec<ReportedMeasureModel>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<
        &GetReportedMeasureCategoriesByReportedMeasureCategoryCodeReportedMeasuresResponse,
    >
    for GetReportedMeasureCategoriesByReportedMeasureCategoryCodeReportedMeasuresResponse {
        fn from(
            value: &GetReportedMeasureCategoriesByReportedMeasureCategoryCodeReportedMeasuresResponse,
        ) -> Self {
            value.clone()
        }
    }
    impl GetReportedMeasureCategoriesByReportedMeasureCategoryCodeReportedMeasuresResponse {
        pub fn builder() -> builder::GetReportedMeasureCategoriesByReportedMeasureCategoryCodeReportedMeasuresResponse {
            Default::default()
        }
    }
    ///`GetReportedMeasureCategoriesByReportedMeasureCategoryCodeResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ReportedMeasureCategoryModel"
    ///      }
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetReportedMeasureCategoriesByReportedMeasureCategoryCodeResponse {
        pub result: ::std::vec::Vec<ReportedMeasureCategoryModel>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<
        &GetReportedMeasureCategoriesByReportedMeasureCategoryCodeResponse,
    > for GetReportedMeasureCategoriesByReportedMeasureCategoryCodeResponse {
        fn from(
            value: &GetReportedMeasureCategoriesByReportedMeasureCategoryCodeResponse,
        ) -> Self {
            value.clone()
        }
    }
    impl GetReportedMeasureCategoriesByReportedMeasureCategoryCodeResponse {
        pub fn builder() -> builder::GetReportedMeasureCategoriesByReportedMeasureCategoryCodeResponse {
            Default::default()
        }
    }
    ///`GetReportedMeasureCategoriesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ReportedMeasureCategoryModel"
    ///      }
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetReportedMeasureCategoriesResponse {
        pub result: ::std::vec::Vec<ReportedMeasureCategoryModel>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetReportedMeasureCategoriesResponse>
    for GetReportedMeasureCategoriesResponse {
        fn from(value: &GetReportedMeasureCategoriesResponse) -> Self {
            value.clone()
        }
    }
    impl GetReportedMeasureCategoriesResponse {
        pub fn builder() -> builder::GetReportedMeasureCategoriesResponse {
            Default::default()
        }
    }
    ///`GetReportedMeasuresByReportedMeasureCodeDataItemsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/DataItemModel"
    ///      }
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetReportedMeasuresByReportedMeasureCodeDataItemsResponse {
        pub result: ::std::vec::Vec<DataItemModel>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetReportedMeasuresByReportedMeasureCodeDataItemsResponse>
    for GetReportedMeasuresByReportedMeasureCodeDataItemsResponse {
        fn from(
            value: &GetReportedMeasuresByReportedMeasureCodeDataItemsResponse,
        ) -> Self {
            value.clone()
        }
    }
    impl GetReportedMeasuresByReportedMeasureCodeDataItemsResponse {
        pub fn builder() -> builder::GetReportedMeasuresByReportedMeasureCodeDataItemsResponse {
            Default::default()
        }
    }
    ///`GetReportedMeasuresByReportedMeasureCodeResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "$ref": "#/components/schemas/ReportedMeasureModel"
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetReportedMeasuresByReportedMeasureCodeResponse {
        pub result: ReportedMeasureModel,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetReportedMeasuresByReportedMeasureCodeResponse>
    for GetReportedMeasuresByReportedMeasureCodeResponse {
        fn from(value: &GetReportedMeasuresByReportedMeasureCodeResponse) -> Self {
            value.clone()
        }
    }
    impl GetReportedMeasuresByReportedMeasureCodeResponse {
        pub fn builder() -> builder::GetReportedMeasuresByReportedMeasureCodeResponse {
            Default::default()
        }
    }
    ///`GetReportedMeasuresResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ReportedMeasureModel"
    ///      }
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetReportedMeasuresResponse {
        pub result: ::std::vec::Vec<ReportedMeasureModel>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetReportedMeasuresResponse>
    for GetReportedMeasuresResponse {
        fn from(value: &GetReportedMeasuresResponse) -> Self {
            value.clone()
        }
    }
    impl GetReportedMeasuresResponse {
        pub fn builder() -> builder::GetReportedMeasuresResponse {
            Default::default()
        }
    }
    ///`GetReportingUnitTypesByReportingUnitTypeCodeBricksAvailableResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "array",
    ///        "items": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetReportingUnitTypesByReportingUnitTypeCodeBricksAvailableResponse {
        pub result: ::std::collections::HashMap<
            ::std::string::String,
            ::std::vec::Vec<::std::string::String>,
        >,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<
        &GetReportingUnitTypesByReportingUnitTypeCodeBricksAvailableResponse,
    > for GetReportingUnitTypesByReportingUnitTypeCodeBricksAvailableResponse {
        fn from(
            value: &GetReportingUnitTypesByReportingUnitTypeCodeBricksAvailableResponse,
        ) -> Self {
            value.clone()
        }
    }
    impl GetReportingUnitTypesByReportingUnitTypeCodeBricksAvailableResponse {
        pub fn builder() -> builder::GetReportingUnitTypesByReportingUnitTypeCodeBricksAvailableResponse {
            Default::default()
        }
    }
    ///`GetReportingUnitTypesByReportingUnitTypeCodeResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "$ref": "#/components/schemas/ReportingUnitTypeModel"
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetReportingUnitTypesByReportingUnitTypeCodeResponse {
        pub result: ReportingUnitTypeModel,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetReportingUnitTypesByReportingUnitTypeCodeResponse>
    for GetReportingUnitTypesByReportingUnitTypeCodeResponse {
        fn from(value: &GetReportingUnitTypesByReportingUnitTypeCodeResponse) -> Self {
            value.clone()
        }
    }
    impl GetReportingUnitTypesByReportingUnitTypeCodeResponse {
        pub fn builder() -> builder::GetReportingUnitTypesByReportingUnitTypeCodeResponse {
            Default::default()
        }
    }
    ///`GetReportingUnitTypesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ReportingUnitTypeModel"
    ///      }
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetReportingUnitTypesResponse {
        pub result: ::std::vec::Vec<ReportingUnitTypeModel>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetReportingUnitTypesResponse>
    for GetReportingUnitTypesResponse {
        fn from(value: &GetReportingUnitTypesResponse) -> Self {
            value.clone()
        }
    }
    impl GetReportingUnitTypesResponse {
        pub fn builder() -> builder::GetReportingUnitTypesResponse {
            Default::default()
        }
    }
    ///`GetReportingUnitsByReportingUnitCodeBricksAvailableResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetReportingUnitsByReportingUnitCodeBricksAvailableResponse {
        pub result: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<
        &GetReportingUnitsByReportingUnitCodeBricksAvailableResponse,
    > for GetReportingUnitsByReportingUnitCodeBricksAvailableResponse {
        fn from(
            value: &GetReportingUnitsByReportingUnitCodeBricksAvailableResponse,
        ) -> Self {
            value.clone()
        }
    }
    impl GetReportingUnitsByReportingUnitCodeBricksAvailableResponse {
        pub fn builder() -> builder::GetReportingUnitsByReportingUnitCodeBricksAvailableResponse {
            Default::default()
        }
    }
    ///`GetReportingUnitsByReportingUnitCodeDataItemsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/DataItemModel"
    ///      }
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetReportingUnitsByReportingUnitCodeDataItemsResponse {
        pub result: ::std::vec::Vec<DataItemModel>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetReportingUnitsByReportingUnitCodeDataItemsResponse>
    for GetReportingUnitsByReportingUnitCodeDataItemsResponse {
        fn from(value: &GetReportingUnitsByReportingUnitCodeDataItemsResponse) -> Self {
            value.clone()
        }
    }
    impl GetReportingUnitsByReportingUnitCodeDataItemsResponse {
        pub fn builder() -> builder::GetReportingUnitsByReportingUnitCodeDataItemsResponse {
            Default::default()
        }
    }
    ///`GetReportingUnitsByReportingUnitCodeMeasuresAvailableResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MeasureSummaryModel"
    ///      }
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetReportingUnitsByReportingUnitCodeMeasuresAvailableResponse {
        pub result: ::std::vec::Vec<MeasureSummaryModel>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<
        &GetReportingUnitsByReportingUnitCodeMeasuresAvailableResponse,
    > for GetReportingUnitsByReportingUnitCodeMeasuresAvailableResponse {
        fn from(
            value: &GetReportingUnitsByReportingUnitCodeMeasuresAvailableResponse,
        ) -> Self {
            value.clone()
        }
    }
    impl GetReportingUnitsByReportingUnitCodeMeasuresAvailableResponse {
        pub fn builder() -> builder::GetReportingUnitsByReportingUnitCodeMeasuresAvailableResponse {
            Default::default()
        }
    }
    ///`GetReportingUnitsByReportingUnitCodeResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "$ref": "#/components/schemas/ReportingUnitModel"
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetReportingUnitsByReportingUnitCodeResponse {
        pub result: ReportingUnitModel,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetReportingUnitsByReportingUnitCodeResponse>
    for GetReportingUnitsByReportingUnitCodeResponse {
        fn from(value: &GetReportingUnitsByReportingUnitCodeResponse) -> Self {
            value.clone()
        }
    }
    impl GetReportingUnitsByReportingUnitCodeResponse {
        pub fn builder() -> builder::GetReportingUnitsByReportingUnitCodeResponse {
            Default::default()
        }
    }
    ///`GetReportingUnitsDownloadsDatasheetCodesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "array",
    ///        "items": {
    ///          "$ref": "#/components/schemas/DatasheetConfigurationSummaryModel"
    ///        }
    ///      }
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetReportingUnitsDownloadsDatasheetCodesResponse {
        pub result: ::std::collections::HashMap<
            ::std::string::String,
            ::std::vec::Vec<DatasheetConfigurationSummaryModel>,
        >,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetReportingUnitsDownloadsDatasheetCodesResponse>
    for GetReportingUnitsDownloadsDatasheetCodesResponse {
        fn from(value: &GetReportingUnitsDownloadsDatasheetCodesResponse) -> Self {
            value.clone()
        }
    }
    impl GetReportingUnitsDownloadsDatasheetCodesResponse {
        pub fn builder() -> builder::GetReportingUnitsDownloadsDatasheetCodesResponse {
            Default::default()
        }
    }
    ///`GetReportingUnitsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ReportingUnitModel"
    ///      }
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetReportingUnitsResponse {
        pub result: ::std::vec::Vec<ReportingUnitModel>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetReportingUnitsResponse> for GetReportingUnitsResponse {
        fn from(value: &GetReportingUnitsResponse) -> Self {
            value.clone()
        }
    }
    impl GetReportingUnitsResponse {
        pub fn builder() -> builder::GetReportingUnitsResponse {
            Default::default()
        }
    }
    ///`GetSimpleDownloadsDownloadCodesResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "array",
    ///        "items": {
    ///          "$ref": "#/components/schemas/DatasheetConfigurationSummaryModel"
    ///        }
    ///      }
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetSimpleDownloadsDownloadCodesResponse {
        pub result: ::std::collections::HashMap<
            ::std::string::String,
            ::std::vec::Vec<DatasheetConfigurationSummaryModel>,
        >,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetSimpleDownloadsDownloadCodesResponse>
    for GetSimpleDownloadsDownloadCodesResponse {
        fn from(value: &GetSimpleDownloadsDownloadCodesResponse) -> Self {
            value.clone()
        }
    }
    impl GetSimpleDownloadsDownloadCodesResponse {
        pub fn builder() -> builder::GetSimpleDownloadsDownloadCodesResponse {
            Default::default()
        }
    }
    ///`GetSuppressionsBySuppressionCodeResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "$ref": "#/components/schemas/SuppressionModel"
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetSuppressionsBySuppressionCodeResponse {
        pub result: SuppressionModel,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetSuppressionsBySuppressionCodeResponse>
    for GetSuppressionsBySuppressionCodeResponse {
        fn from(value: &GetSuppressionsBySuppressionCodeResponse) -> Self {
            value.clone()
        }
    }
    impl GetSuppressionsBySuppressionCodeResponse {
        pub fn builder() -> builder::GetSuppressionsBySuppressionCodeResponse {
            Default::default()
        }
    }
    ///`GetSuppressionsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "result"
    ///  ],
    ///  "properties": {
    ///    "result": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/SuppressionModel"
    ///      }
    ///    },
    ///    "version_information": {
    ///      "$ref": "#/components/schemas/VersionInformation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct GetSuppressionsResponse {
        pub result: ::std::vec::Vec<SuppressionModel>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub version_information: ::std::option::Option<VersionInformation>,
    }
    impl ::std::convert::From<&GetSuppressionsResponse> for GetSuppressionsResponse {
        fn from(value: &GetSuppressionsResponse) -> Self {
            value.clone()
        }
    }
    impl GetSuppressionsResponse {
        pub fn builder() -> builder::GetSuppressionsResponse {
            Default::default()
        }
    }
    ///A MappedReportingUnit is used to show a relationship between two ReportingUnits.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A MappedReportingUnit is used to show a relationship between two ReportingUnits.",
    ///  "type": "object",
    ///  "required": [
    ///    "map_type",
    ///    "mapped_reporting_unit"
    ///  ],
    ///  "properties": {
    ///    "map_type": {
    ///      "$ref": "#/components/schemas/MappedReportingUnitTypeModel"
    ///    },
    ///    "mapped_reporting_unit": {
    ///      "$ref": "#/components/schemas/ReportingUnitSummaryModel"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct MappedReportingUnitModel {
        pub map_type: MappedReportingUnitTypeModel,
        pub mapped_reporting_unit: ReportingUnitSummaryModel,
    }
    impl ::std::convert::From<&MappedReportingUnitModel> for MappedReportingUnitModel {
        fn from(value: &MappedReportingUnitModel) -> Self {
            value.clone()
        }
    }
    impl MappedReportingUnitModel {
        pub fn builder() -> builder::MappedReportingUnitModel {
            Default::default()
        }
    }
    ///A MappedReportingUnitType indicates the type of relationship for a ReportingUnitMapping giving context to the relationship.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A MappedReportingUnitType indicates the type of relationship for a ReportingUnitMapping giving context to the relationship.",
    ///  "type": "object",
    ///  "required": [
    ///    "mapped_reporting_unit_code",
    ///    "mapped_reporting_unit_name"
    ///  ],
    ///  "properties": {
    ///    "mapped_reporting_unit_code": {
    ///      "description": "The code for the corresponding MappedReportingUnitType.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "mapped_reporting_unit_name": {
    ///      "description": "The name for the corresponding MappedReportingUnitType.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct MappedReportingUnitTypeModel {
        ///The code for the corresponding MappedReportingUnitType.
        pub mapped_reporting_unit_code: MappedReportingUnitTypeModelMappedReportingUnitCode,
        ///The name for the corresponding MappedReportingUnitType.
        pub mapped_reporting_unit_name: MappedReportingUnitTypeModelMappedReportingUnitName,
    }
    impl ::std::convert::From<&MappedReportingUnitTypeModel>
    for MappedReportingUnitTypeModel {
        fn from(value: &MappedReportingUnitTypeModel) -> Self {
            value.clone()
        }
    }
    impl MappedReportingUnitTypeModel {
        pub fn builder() -> builder::MappedReportingUnitTypeModel {
            Default::default()
        }
    }
    ///The code for the corresponding MappedReportingUnitType.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the corresponding MappedReportingUnitType.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct MappedReportingUnitTypeModelMappedReportingUnitCode(
        ::std::string::String,
    );
    impl ::std::ops::Deref for MappedReportingUnitTypeModelMappedReportingUnitCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<MappedReportingUnitTypeModelMappedReportingUnitCode>
    for ::std::string::String {
        fn from(value: MappedReportingUnitTypeModelMappedReportingUnitCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&MappedReportingUnitTypeModelMappedReportingUnitCode>
    for MappedReportingUnitTypeModelMappedReportingUnitCode {
        fn from(value: &MappedReportingUnitTypeModelMappedReportingUnitCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for MappedReportingUnitTypeModelMappedReportingUnitCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str>
    for MappedReportingUnitTypeModelMappedReportingUnitCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for MappedReportingUnitTypeModelMappedReportingUnitCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for MappedReportingUnitTypeModelMappedReportingUnitCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de>
    for MappedReportingUnitTypeModelMappedReportingUnitCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the corresponding MappedReportingUnitType.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the corresponding MappedReportingUnitType.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct MappedReportingUnitTypeModelMappedReportingUnitName(
        ::std::string::String,
    );
    impl ::std::ops::Deref for MappedReportingUnitTypeModelMappedReportingUnitName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<MappedReportingUnitTypeModelMappedReportingUnitName>
    for ::std::string::String {
        fn from(value: MappedReportingUnitTypeModelMappedReportingUnitName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&MappedReportingUnitTypeModelMappedReportingUnitName>
    for MappedReportingUnitTypeModelMappedReportingUnitName {
        fn from(value: &MappedReportingUnitTypeModelMappedReportingUnitName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for MappedReportingUnitTypeModelMappedReportingUnitName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str>
    for MappedReportingUnitTypeModelMappedReportingUnitName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for MappedReportingUnitTypeModelMappedReportingUnitName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for MappedReportingUnitTypeModelMappedReportingUnitName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de>
    for MappedReportingUnitTypeModelMappedReportingUnitName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///A MeasureCategory is used to provide grouping of Measures.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A MeasureCategory is used to provide grouping of Measures.",
    ///  "type": "object",
    ///  "required": [
    ///    "measure_category_code",
    ///    "measure_category_name"
    ///  ],
    ///  "properties": {
    ///    "measure_category_code": {
    ///      "description": "The code for the corresponding MeasureCategory.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "measure_category_name": {
    ///      "description": "The name for the corresponding MeasureCategory.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct MeasureCategoryModel {
        ///The code for the corresponding MeasureCategory.
        pub measure_category_code: MeasureCategoryModelMeasureCategoryCode,
        ///The name for the corresponding MeasureCategory.
        pub measure_category_name: MeasureCategoryModelMeasureCategoryName,
    }
    impl ::std::convert::From<&MeasureCategoryModel> for MeasureCategoryModel {
        fn from(value: &MeasureCategoryModel) -> Self {
            value.clone()
        }
    }
    impl MeasureCategoryModel {
        pub fn builder() -> builder::MeasureCategoryModel {
            Default::default()
        }
    }
    ///The code for the corresponding MeasureCategory.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the corresponding MeasureCategory.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct MeasureCategoryModelMeasureCategoryCode(::std::string::String);
    impl ::std::ops::Deref for MeasureCategoryModelMeasureCategoryCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<MeasureCategoryModelMeasureCategoryCode>
    for ::std::string::String {
        fn from(value: MeasureCategoryModelMeasureCategoryCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&MeasureCategoryModelMeasureCategoryCode>
    for MeasureCategoryModelMeasureCategoryCode {
        fn from(value: &MeasureCategoryModelMeasureCategoryCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for MeasureCategoryModelMeasureCategoryCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for MeasureCategoryModelMeasureCategoryCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for MeasureCategoryModelMeasureCategoryCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for MeasureCategoryModelMeasureCategoryCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MeasureCategoryModelMeasureCategoryCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the corresponding MeasureCategory.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the corresponding MeasureCategory.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct MeasureCategoryModelMeasureCategoryName(::std::string::String);
    impl ::std::ops::Deref for MeasureCategoryModelMeasureCategoryName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<MeasureCategoryModelMeasureCategoryName>
    for ::std::string::String {
        fn from(value: MeasureCategoryModelMeasureCategoryName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&MeasureCategoryModelMeasureCategoryName>
    for MeasureCategoryModelMeasureCategoryName {
        fn from(value: &MeasureCategoryModelMeasureCategoryName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for MeasureCategoryModelMeasureCategoryName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for MeasureCategoryModelMeasureCategoryName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for MeasureCategoryModelMeasureCategoryName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for MeasureCategoryModelMeasureCategoryName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MeasureCategoryModelMeasureCategoryName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    /**A Measure is a statistic used to measure something. Some measures are performance indicators (e.g. median waiting times for elective surgery) and some provide context for
            these indicators (e.g. number of elective surgeries).*/
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A Measure is a statistic used to measure something. Some measures are performance indicators (e.g. median waiting times for elective surgery) and some provide context for\r\n            these indicators (e.g. number of elective surgeries).",
    ///  "type": "object",
    ///  "required": [
    ///    "measure_categories",
    ///    "measure_code",
    ///    "measure_name",
    ///    "meta_tags",
    ///    "units"
    ///  ],
    ///  "properties": {
    ///    "measure_categories": {
    ///      "description": "List of MeasureCategories applicable to this Measure.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MeasureCategoryModel"
    ///      }
    ///    },
    ///    "measure_code": {
    ///      "description": "The code for the corresponding Measure.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "measure_name": {
    ///      "description": "The name for the corresponding Measure.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "meta_tags": {
    ///      "description": "List of MetaTags applicable to this Measure. This list will be empty if there are no MetaTags.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MetaTagModel"
    ///      }
    ///    },
    ///    "units": {
    ///      "$ref": "#/components/schemas/UnitsModel"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct MeasureModel {
        ///List of MeasureCategories applicable to this Measure.
        pub measure_categories: ::std::vec::Vec<MeasureCategoryModel>,
        ///The code for the corresponding Measure.
        pub measure_code: MeasureModelMeasureCode,
        ///The name for the corresponding Measure.
        pub measure_name: MeasureModelMeasureName,
        ///List of MetaTags applicable to this Measure. This list will be empty if there are no MetaTags.
        pub meta_tags: ::std::vec::Vec<MetaTagModel>,
        pub units: UnitsModel,
    }
    impl ::std::convert::From<&MeasureModel> for MeasureModel {
        fn from(value: &MeasureModel) -> Self {
            value.clone()
        }
    }
    impl MeasureModel {
        pub fn builder() -> builder::MeasureModel {
            Default::default()
        }
    }
    ///The code for the corresponding Measure.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the corresponding Measure.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct MeasureModelMeasureCode(::std::string::String);
    impl ::std::ops::Deref for MeasureModelMeasureCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<MeasureModelMeasureCode> for ::std::string::String {
        fn from(value: MeasureModelMeasureCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&MeasureModelMeasureCode> for MeasureModelMeasureCode {
        fn from(value: &MeasureModelMeasureCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for MeasureModelMeasureCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for MeasureModelMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for MeasureModelMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for MeasureModelMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MeasureModelMeasureCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the corresponding Measure.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the corresponding Measure.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct MeasureModelMeasureName(::std::string::String);
    impl ::std::ops::Deref for MeasureModelMeasureName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<MeasureModelMeasureName> for ::std::string::String {
        fn from(value: MeasureModelMeasureName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&MeasureModelMeasureName> for MeasureModelMeasureName {
        fn from(value: &MeasureModelMeasureName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for MeasureModelMeasureName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for MeasureModelMeasureName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for MeasureModelMeasureName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for MeasureModelMeasureName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MeasureModelMeasureName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///A MeasureSummaryModel details just the code and name of a Measure without any additional fields.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A MeasureSummaryModel details just the code and name of a Measure without any additional fields.",
    ///  "type": "object",
    ///  "required": [
    ///    "measure_code",
    ///    "measure_name"
    ///  ],
    ///  "properties": {
    ///    "measure_code": {
    ///      "description": "The code for the corresponding Measure.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "measure_name": {
    ///      "description": "The name for the corresponding Measure.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct MeasureSummaryModel {
        ///The code for the corresponding Measure.
        pub measure_code: MeasureSummaryModelMeasureCode,
        ///The name for the corresponding Measure.
        pub measure_name: MeasureSummaryModelMeasureName,
    }
    impl ::std::convert::From<&MeasureSummaryModel> for MeasureSummaryModel {
        fn from(value: &MeasureSummaryModel) -> Self {
            value.clone()
        }
    }
    impl MeasureSummaryModel {
        pub fn builder() -> builder::MeasureSummaryModel {
            Default::default()
        }
    }
    ///The code for the corresponding Measure.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the corresponding Measure.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct MeasureSummaryModelMeasureCode(::std::string::String);
    impl ::std::ops::Deref for MeasureSummaryModelMeasureCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<MeasureSummaryModelMeasureCode> for ::std::string::String {
        fn from(value: MeasureSummaryModelMeasureCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&MeasureSummaryModelMeasureCode>
    for MeasureSummaryModelMeasureCode {
        fn from(value: &MeasureSummaryModelMeasureCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for MeasureSummaryModelMeasureCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for MeasureSummaryModelMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for MeasureSummaryModelMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for MeasureSummaryModelMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MeasureSummaryModelMeasureCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the corresponding Measure.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the corresponding Measure.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct MeasureSummaryModelMeasureName(::std::string::String);
    impl ::std::ops::Deref for MeasureSummaryModelMeasureName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<MeasureSummaryModelMeasureName> for ::std::string::String {
        fn from(value: MeasureSummaryModelMeasureName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&MeasureSummaryModelMeasureName>
    for MeasureSummaryModelMeasureName {
        fn from(value: &MeasureSummaryModelMeasureName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for MeasureSummaryModelMeasureName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for MeasureSummaryModelMeasureName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for MeasureSummaryModelMeasureName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for MeasureSummaryModelMeasureName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MeasureSummaryModelMeasureName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///A MetaTag is a grouping category that can be applied to a Measure, ReportedMeasure, ReportingUnit or DataSet.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A MetaTag is a grouping category that can be applied to a Measure, ReportedMeasure, ReportingUnit or DataSet.",
    ///  "type": "object",
    ///  "required": [
    ///    "meta_tag_code",
    ///    "meta_tag_name",
    ///    "meta_tag_type"
    ///  ],
    ///  "properties": {
    ///    "meta_tag_code": {
    ///      "description": "The code representing the MetaTag.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "meta_tag_name": {
    ///      "description": "The display text for the MetaTag.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "meta_tag_type": {
    ///      "$ref": "#/components/schemas/MetaTagTypeModel"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct MetaTagModel {
        ///The code representing the MetaTag.
        pub meta_tag_code: MetaTagModelMetaTagCode,
        ///The display text for the MetaTag.
        pub meta_tag_name: MetaTagModelMetaTagName,
        pub meta_tag_type: MetaTagTypeModel,
    }
    impl ::std::convert::From<&MetaTagModel> for MetaTagModel {
        fn from(value: &MetaTagModel) -> Self {
            value.clone()
        }
    }
    impl MetaTagModel {
        pub fn builder() -> builder::MetaTagModel {
            Default::default()
        }
    }
    ///The code representing the MetaTag.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code representing the MetaTag.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct MetaTagModelMetaTagCode(::std::string::String);
    impl ::std::ops::Deref for MetaTagModelMetaTagCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<MetaTagModelMetaTagCode> for ::std::string::String {
        fn from(value: MetaTagModelMetaTagCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&MetaTagModelMetaTagCode> for MetaTagModelMetaTagCode {
        fn from(value: &MetaTagModelMetaTagCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for MetaTagModelMetaTagCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for MetaTagModelMetaTagCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for MetaTagModelMetaTagCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for MetaTagModelMetaTagCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MetaTagModelMetaTagCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The display text for the MetaTag.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The display text for the MetaTag.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct MetaTagModelMetaTagName(::std::string::String);
    impl ::std::ops::Deref for MetaTagModelMetaTagName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<MetaTagModelMetaTagName> for ::std::string::String {
        fn from(value: MetaTagModelMetaTagName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&MetaTagModelMetaTagName> for MetaTagModelMetaTagName {
        fn from(value: &MetaTagModelMetaTagName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for MetaTagModelMetaTagName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for MetaTagModelMetaTagName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for MetaTagModelMetaTagName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for MetaTagModelMetaTagName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MetaTagModelMetaTagName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///A MetaTagType indicates the type for a series of MetaTags and acts as a MetaTag grouping.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A MetaTagType indicates the type for a series of MetaTags and acts as a MetaTag grouping.",
    ///  "type": "object",
    ///  "required": [
    ///    "meta_tag_type_code",
    ///    "meta_tag_type_name"
    ///  ],
    ///  "properties": {
    ///    "meta_tag_type_code": {
    ///      "description": "The code for the corresponding MetaTagType.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "meta_tag_type_name": {
    ///      "description": "The name for the corresponding MetaTagType.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct MetaTagTypeModel {
        ///The code for the corresponding MetaTagType.
        pub meta_tag_type_code: MetaTagTypeModelMetaTagTypeCode,
        ///The name for the corresponding MetaTagType.
        pub meta_tag_type_name: MetaTagTypeModelMetaTagTypeName,
    }
    impl ::std::convert::From<&MetaTagTypeModel> for MetaTagTypeModel {
        fn from(value: &MetaTagTypeModel) -> Self {
            value.clone()
        }
    }
    impl MetaTagTypeModel {
        pub fn builder() -> builder::MetaTagTypeModel {
            Default::default()
        }
    }
    ///The code for the corresponding MetaTagType.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the corresponding MetaTagType.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct MetaTagTypeModelMetaTagTypeCode(::std::string::String);
    impl ::std::ops::Deref for MetaTagTypeModelMetaTagTypeCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<MetaTagTypeModelMetaTagTypeCode>
    for ::std::string::String {
        fn from(value: MetaTagTypeModelMetaTagTypeCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&MetaTagTypeModelMetaTagTypeCode>
    for MetaTagTypeModelMetaTagTypeCode {
        fn from(value: &MetaTagTypeModelMetaTagTypeCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for MetaTagTypeModelMetaTagTypeCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for MetaTagTypeModelMetaTagTypeCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for MetaTagTypeModelMetaTagTypeCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for MetaTagTypeModelMetaTagTypeCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MetaTagTypeModelMetaTagTypeCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the corresponding MetaTagType.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the corresponding MetaTagType.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct MetaTagTypeModelMetaTagTypeName(::std::string::String);
    impl ::std::ops::Deref for MetaTagTypeModelMetaTagTypeName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<MetaTagTypeModelMetaTagTypeName>
    for ::std::string::String {
        fn from(value: MetaTagTypeModelMetaTagTypeName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&MetaTagTypeModelMetaTagTypeName>
    for MetaTagTypeModelMetaTagTypeName {
        fn from(value: &MetaTagTypeModelMetaTagTypeName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for MetaTagTypeModelMetaTagTypeName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for MetaTagTypeModelMetaTagTypeName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for MetaTagTypeModelMetaTagTypeName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for MetaTagTypeModelMetaTagTypeName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for MetaTagTypeModelMetaTagTypeName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///A paginated results set containing a list of DataExtract items and the pagination information for the request.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A paginated results set containing a list of DataExtract items and the pagination information for the request.",
    ///  "type": "object",
    ///  "properties": {
    ///    "data": {
    ///      "description": "The list of DataExtract items.",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/DataExtractModel"
    ///      }
    ///    },
    ///    "pagination": {
    ///      "$ref": "#/components/schemas/DataExtractPagination"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PaginatedDataExtractModel {
        ///The list of DataExtract items.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data: ::std::option::Option<::std::vec::Vec<DataExtractModel>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub pagination: ::std::option::Option<DataExtractPagination>,
    }
    impl ::std::convert::From<&PaginatedDataExtractModel> for PaginatedDataExtractModel {
        fn from(value: &PaginatedDataExtractModel) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for PaginatedDataExtractModel {
        fn default() -> Self {
            Self {
                data: Default::default(),
                pagination: Default::default(),
            }
        }
    }
    impl PaginatedDataExtractModel {
        pub fn builder() -> builder::PaginatedDataExtractModel {
            Default::default()
        }
    }
    ///A paginated results set containing a list of FormattedDataExtract items and the pagination information for the request.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A paginated results set containing a list of FormattedDataExtract items and the pagination information for the request.",
    ///  "type": "object",
    ///  "properties": {
    ///    "data": {
    ///      "description": "The list of FormattedDataExtract items.",
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/FormattedDataExtractModel"
    ///      }
    ///    },
    ///    "pagination": {
    ///      "$ref": "#/components/schemas/DataExtractPagination"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PaginatedFormattedDataExtractModel {
        ///The list of FormattedDataExtract items.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data: ::std::option::Option<::std::vec::Vec<FormattedDataExtractModel>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub pagination: ::std::option::Option<DataExtractPagination>,
    }
    impl ::std::convert::From<&PaginatedFormattedDataExtractModel>
    for PaginatedFormattedDataExtractModel {
        fn from(value: &PaginatedFormattedDataExtractModel) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for PaginatedFormattedDataExtractModel {
        fn default() -> Self {
            Self {
                data: Default::default(),
                pagination: Default::default(),
            }
        }
    }
    impl PaginatedFormattedDataExtractModel {
        pub fn builder() -> builder::PaginatedFormattedDataExtractModel {
            Default::default()
        }
    }
    ///`ProblemDetails`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "detail": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "instance": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "status": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32"
    ///    },
    ///    "title": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "type": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  },
    ///  "additionalProperties": {}
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct ProblemDetails {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub detail: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub instance: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<i32>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
        #[serde(flatten)]
        pub extra: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    }
    impl ::std::convert::From<&ProblemDetails> for ProblemDetails {
        fn from(value: &ProblemDetails) -> Self {
            value.clone()
        }
    }
    impl ProblemDetails {
        pub fn builder() -> builder::ProblemDetails {
            Default::default()
        }
    }
    ///A ReportedMeasureCategory provides the method for the disaggregation of a particular Measure and also allows grouping of ReportedMeasures.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A ReportedMeasureCategory provides the method for the disaggregation of a particular Measure and also allows grouping of ReportedMeasures.",
    ///  "type": "object",
    ///  "required": [
    ///    "reported_measure_category_code",
    ///    "reported_measure_category_name",
    ///    "reported_measure_category_type"
    ///  ],
    ///  "properties": {
    ///    "reported_measure_category_code": {
    ///      "description": "The code representing the ReportedMeasureCategory.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reported_measure_category_name": {
    ///      "description": "The display text for the ReportedMeasureCategory.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reported_measure_category_type": {
    ///      "$ref": "#/components/schemas/ReportedMeasureCategoryTypeModel"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ReportedMeasureCategoryModel {
        ///The code representing the ReportedMeasureCategory.
        pub reported_measure_category_code: ReportedMeasureCategoryModelReportedMeasureCategoryCode,
        ///The display text for the ReportedMeasureCategory.
        pub reported_measure_category_name: ReportedMeasureCategoryModelReportedMeasureCategoryName,
        pub reported_measure_category_type: ReportedMeasureCategoryTypeModel,
    }
    impl ::std::convert::From<&ReportedMeasureCategoryModel>
    for ReportedMeasureCategoryModel {
        fn from(value: &ReportedMeasureCategoryModel) -> Self {
            value.clone()
        }
    }
    impl ReportedMeasureCategoryModel {
        pub fn builder() -> builder::ReportedMeasureCategoryModel {
            Default::default()
        }
    }
    ///The code representing the ReportedMeasureCategory.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code representing the ReportedMeasureCategory.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ReportedMeasureCategoryModelReportedMeasureCategoryCode(
        ::std::string::String,
    );
    impl ::std::ops::Deref for ReportedMeasureCategoryModelReportedMeasureCategoryCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<ReportedMeasureCategoryModelReportedMeasureCategoryCode>
    for ::std::string::String {
        fn from(value: ReportedMeasureCategoryModelReportedMeasureCategoryCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&ReportedMeasureCategoryModelReportedMeasureCategoryCode>
    for ReportedMeasureCategoryModelReportedMeasureCategoryCode {
        fn from(
            value: &ReportedMeasureCategoryModelReportedMeasureCategoryCode,
        ) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr
    for ReportedMeasureCategoryModelReportedMeasureCategoryCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str>
    for ReportedMeasureCategoryModelReportedMeasureCategoryCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for ReportedMeasureCategoryModelReportedMeasureCategoryCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for ReportedMeasureCategoryModelReportedMeasureCategoryCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de>
    for ReportedMeasureCategoryModelReportedMeasureCategoryCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The display text for the ReportedMeasureCategory.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The display text for the ReportedMeasureCategory.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ReportedMeasureCategoryModelReportedMeasureCategoryName(
        ::std::string::String,
    );
    impl ::std::ops::Deref for ReportedMeasureCategoryModelReportedMeasureCategoryName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<ReportedMeasureCategoryModelReportedMeasureCategoryName>
    for ::std::string::String {
        fn from(value: ReportedMeasureCategoryModelReportedMeasureCategoryName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&ReportedMeasureCategoryModelReportedMeasureCategoryName>
    for ReportedMeasureCategoryModelReportedMeasureCategoryName {
        fn from(
            value: &ReportedMeasureCategoryModelReportedMeasureCategoryName,
        ) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr
    for ReportedMeasureCategoryModelReportedMeasureCategoryName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str>
    for ReportedMeasureCategoryModelReportedMeasureCategoryName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for ReportedMeasureCategoryModelReportedMeasureCategoryName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for ReportedMeasureCategoryModelReportedMeasureCategoryName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de>
    for ReportedMeasureCategoryModelReportedMeasureCategoryName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The reported measure category type model.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The reported measure category type model.",
    ///  "type": "object",
    ///  "required": [
    ///    "reported_measure_category_type_code",
    ///    "reported_measure_category_type_name"
    ///  ],
    ///  "properties": {
    ///    "reported_measure_category_type_code": {
    ///      "description": "The code for the corresponding ReportedMeasureCategoryType.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reported_measure_category_type_name": {
    ///      "description": "The name for the corresponding ReportedMeasureCategoryType.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ReportedMeasureCategoryTypeModel {
        ///The code for the corresponding ReportedMeasureCategoryType.
        pub reported_measure_category_type_code: ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeCode,
        ///The name for the corresponding ReportedMeasureCategoryType.
        pub reported_measure_category_type_name: ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeName,
    }
    impl ::std::convert::From<&ReportedMeasureCategoryTypeModel>
    for ReportedMeasureCategoryTypeModel {
        fn from(value: &ReportedMeasureCategoryTypeModel) -> Self {
            value.clone()
        }
    }
    impl ReportedMeasureCategoryTypeModel {
        pub fn builder() -> builder::ReportedMeasureCategoryTypeModel {
            Default::default()
        }
    }
    ///The code for the corresponding ReportedMeasureCategoryType.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the corresponding ReportedMeasureCategoryType.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeCode(
        ::std::string::String,
    );
    impl ::std::ops::Deref
    for ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<
        ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeCode,
    > for ::std::string::String {
        fn from(
            value: ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeCode,
        ) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<
        &ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeCode,
    > for ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeCode {
        fn from(
            value: &ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeCode,
        ) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr
    for ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str>
    for ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de>
    for ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the corresponding ReportedMeasureCategoryType.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the corresponding ReportedMeasureCategoryType.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeName(
        ::std::string::String,
    );
    impl ::std::ops::Deref
    for ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<
        ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeName,
    > for ::std::string::String {
        fn from(
            value: ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeName,
        ) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<
        &ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeName,
    > for ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeName {
        fn from(
            value: &ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeName,
        ) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr
    for ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str>
    for ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de>
    for ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///A ReportedMeasure is a disaggregation of a particular Measure. This disaggregation is indicated by 1 or more ReportedMeasureCategories.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A ReportedMeasure is a disaggregation of a particular Measure. This disaggregation is indicated by 1 or more ReportedMeasureCategories.",
    ///  "type": "object",
    ///  "required": [
    ///    "measure",
    ///    "meta_tags",
    ///    "reported_measure_categories",
    ///    "reported_measure_code",
    ///    "reported_measure_name"
    ///  ],
    ///  "properties": {
    ///    "measure": {
    ///      "$ref": "#/components/schemas/MeasureModel"
    ///    },
    ///    "meta_tags": {
    ///      "description": "List of MetaTags applicable to this Measure. This list will be empty if there are no MetaTags.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MetaTagModel"
    ///      }
    ///    },
    ///    "reported_measure_categories": {
    ///      "description": "List of ReportedMeasureCategories applicable to this ReportedMeasure.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ReportedMeasureCategoryModel"
    ///      }
    ///    },
    ///    "reported_measure_code": {
    ///      "description": "The code for the corresponding ReportedMeasure.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reported_measure_name": {
    ///      "description": "The name for the corresponding ReportedMeasure.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ReportedMeasureModel {
        pub measure: MeasureModel,
        ///List of MetaTags applicable to this Measure. This list will be empty if there are no MetaTags.
        pub meta_tags: ::std::vec::Vec<MetaTagModel>,
        ///List of ReportedMeasureCategories applicable to this ReportedMeasure.
        pub reported_measure_categories: ::std::vec::Vec<ReportedMeasureCategoryModel>,
        ///The code for the corresponding ReportedMeasure.
        pub reported_measure_code: ReportedMeasureModelReportedMeasureCode,
        ///The name for the corresponding ReportedMeasure.
        pub reported_measure_name: ReportedMeasureModelReportedMeasureName,
    }
    impl ::std::convert::From<&ReportedMeasureModel> for ReportedMeasureModel {
        fn from(value: &ReportedMeasureModel) -> Self {
            value.clone()
        }
    }
    impl ReportedMeasureModel {
        pub fn builder() -> builder::ReportedMeasureModel {
            Default::default()
        }
    }
    ///The code for the corresponding ReportedMeasure.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the corresponding ReportedMeasure.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ReportedMeasureModelReportedMeasureCode(::std::string::String);
    impl ::std::ops::Deref for ReportedMeasureModelReportedMeasureCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<ReportedMeasureModelReportedMeasureCode>
    for ::std::string::String {
        fn from(value: ReportedMeasureModelReportedMeasureCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&ReportedMeasureModelReportedMeasureCode>
    for ReportedMeasureModelReportedMeasureCode {
        fn from(value: &ReportedMeasureModelReportedMeasureCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for ReportedMeasureModelReportedMeasureCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for ReportedMeasureModelReportedMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for ReportedMeasureModelReportedMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for ReportedMeasureModelReportedMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReportedMeasureModelReportedMeasureCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the corresponding ReportedMeasure.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the corresponding ReportedMeasure.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ReportedMeasureModelReportedMeasureName(::std::string::String);
    impl ::std::ops::Deref for ReportedMeasureModelReportedMeasureName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<ReportedMeasureModelReportedMeasureName>
    for ::std::string::String {
        fn from(value: ReportedMeasureModelReportedMeasureName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&ReportedMeasureModelReportedMeasureName>
    for ReportedMeasureModelReportedMeasureName {
        fn from(value: &ReportedMeasureModelReportedMeasureName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for ReportedMeasureModelReportedMeasureName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for ReportedMeasureModelReportedMeasureName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for ReportedMeasureModelReportedMeasureName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for ReportedMeasureModelReportedMeasureName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReportedMeasureModelReportedMeasureName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///A ReportedMeasureSummary details the code and name of a ReportedMeasure, including a summary of the associated Measure.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A ReportedMeasureSummary details the code and name of a ReportedMeasure, including a summary of the associated Measure.",
    ///  "type": "object",
    ///  "required": [
    ///    "measure_summary",
    ///    "reported_measure_code",
    ///    "reported_measure_name"
    ///  ],
    ///  "properties": {
    ///    "measure_summary": {
    ///      "$ref": "#/components/schemas/MeasureSummaryModel"
    ///    },
    ///    "reported_measure_code": {
    ///      "description": "The code for the corresponding ReportedMeasure.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reported_measure_name": {
    ///      "description": "The name for the corresponding ReportedMeasure.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ReportedMeasureSummaryModel {
        pub measure_summary: MeasureSummaryModel,
        ///The code for the corresponding ReportedMeasure.
        pub reported_measure_code: ReportedMeasureSummaryModelReportedMeasureCode,
        ///The name for the corresponding ReportedMeasure.
        pub reported_measure_name: ReportedMeasureSummaryModelReportedMeasureName,
    }
    impl ::std::convert::From<&ReportedMeasureSummaryModel>
    for ReportedMeasureSummaryModel {
        fn from(value: &ReportedMeasureSummaryModel) -> Self {
            value.clone()
        }
    }
    impl ReportedMeasureSummaryModel {
        pub fn builder() -> builder::ReportedMeasureSummaryModel {
            Default::default()
        }
    }
    ///The code for the corresponding ReportedMeasure.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the corresponding ReportedMeasure.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ReportedMeasureSummaryModelReportedMeasureCode(::std::string::String);
    impl ::std::ops::Deref for ReportedMeasureSummaryModelReportedMeasureCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<ReportedMeasureSummaryModelReportedMeasureCode>
    for ::std::string::String {
        fn from(value: ReportedMeasureSummaryModelReportedMeasureCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&ReportedMeasureSummaryModelReportedMeasureCode>
    for ReportedMeasureSummaryModelReportedMeasureCode {
        fn from(value: &ReportedMeasureSummaryModelReportedMeasureCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for ReportedMeasureSummaryModelReportedMeasureCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str>
    for ReportedMeasureSummaryModelReportedMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for ReportedMeasureSummaryModelReportedMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for ReportedMeasureSummaryModelReportedMeasureCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de>
    for ReportedMeasureSummaryModelReportedMeasureCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the corresponding ReportedMeasure.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the corresponding ReportedMeasure.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ReportedMeasureSummaryModelReportedMeasureName(::std::string::String);
    impl ::std::ops::Deref for ReportedMeasureSummaryModelReportedMeasureName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<ReportedMeasureSummaryModelReportedMeasureName>
    for ::std::string::String {
        fn from(value: ReportedMeasureSummaryModelReportedMeasureName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&ReportedMeasureSummaryModelReportedMeasureName>
    for ReportedMeasureSummaryModelReportedMeasureName {
        fn from(value: &ReportedMeasureSummaryModelReportedMeasureName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for ReportedMeasureSummaryModelReportedMeasureName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str>
    for ReportedMeasureSummaryModelReportedMeasureName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for ReportedMeasureSummaryModelReportedMeasureName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for ReportedMeasureSummaryModelReportedMeasureName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de>
    for ReportedMeasureSummaryModelReportedMeasureName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///A ReportingUnit indicates what unit of analysis a specific data value corresponds to (often a geographical or otherwise bordered area).
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A ReportingUnit indicates what unit of analysis a specific data value corresponds to (often a geographical or otherwise bordered area).",
    ///  "type": "object",
    ///  "required": [
    ///    "alternative_names",
    ///    "closed",
    ///    "mapped_reporting_units",
    ///    "meta_tags",
    ///    "private",
    ///    "reporting_unit_code",
    ///    "reporting_unit_name",
    ///    "reporting_unit_type"
    ///  ],
    ///  "properties": {
    ///    "alternative_names": {
    ///      "description": "The list of alternate names for this ReportingUnit. If there are no alternate names then this list will be empty.",
    ///      "readOnly": true,
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "closed": {
    ///      "description": "Indicates whether this ReportingUnit is closed.",
    ///      "type": "boolean"
    ///    },
    ///    "latitude": {
    ///      "description": "The latitude for the ReportingUnit. This field may be null if no geographical point is applicable for the ReportingUnit.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "longitude": {
    ///      "description": "The longitude for the ReportingUnit. This field may be null if no geographical point is applicable for the ReportingUnit.",
    ///      "type": [
    ///        "number",
    ///        "null"
    ///      ],
    ///      "format": "double"
    ///    },
    ///    "mapped_reporting_units": {
    ///      "description": "List of mapped ReportingUnits applicable to this ReportingUnit. This list will be empty if there are no mappings.",
    ///      "readOnly": true,
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MappedReportingUnitModel"
    ///      }
    ///    },
    ///    "meta_tags": {
    ///      "description": "List of MetaTags applicable to this Measure. This list will be empty if there are no MetaTags.",
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MetaTagModel"
    ///      }
    ///    },
    ///    "private": {
    ///      "description": "Indicates whether this ReportingUnit is private, e.g. a private hospital.",
    ///      "type": "boolean"
    ///    },
    ///    "reporting_unit_code": {
    ///      "description": "The code for the corresponding ReportingUnit.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reporting_unit_name": {
    ///      "description": "The name for the corresponding ReportingUnit.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reporting_unit_type": {
    ///      "$ref": "#/components/schemas/ReportingUnitTypeModel"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ReportingUnitModel {
        ///The list of alternate names for this ReportingUnit. If there are no alternate names then this list will be empty.
        pub alternative_names: ::std::vec::Vec<::std::string::String>,
        ///Indicates whether this ReportingUnit is closed.
        pub closed: bool,
        ///The latitude for the ReportingUnit. This field may be null if no geographical point is applicable for the ReportingUnit.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub latitude: ::std::option::Option<f64>,
        ///The longitude for the ReportingUnit. This field may be null if no geographical point is applicable for the ReportingUnit.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub longitude: ::std::option::Option<f64>,
        ///List of mapped ReportingUnits applicable to this ReportingUnit. This list will be empty if there are no mappings.
        pub mapped_reporting_units: ::std::vec::Vec<MappedReportingUnitModel>,
        ///List of MetaTags applicable to this Measure. This list will be empty if there are no MetaTags.
        pub meta_tags: ::std::vec::Vec<MetaTagModel>,
        ///Indicates whether this ReportingUnit is private, e.g. a private hospital.
        pub private: bool,
        ///The code for the corresponding ReportingUnit.
        pub reporting_unit_code: ReportingUnitModelReportingUnitCode,
        ///The name for the corresponding ReportingUnit.
        pub reporting_unit_name: ReportingUnitModelReportingUnitName,
        pub reporting_unit_type: ReportingUnitTypeModel,
    }
    impl ::std::convert::From<&ReportingUnitModel> for ReportingUnitModel {
        fn from(value: &ReportingUnitModel) -> Self {
            value.clone()
        }
    }
    impl ReportingUnitModel {
        pub fn builder() -> builder::ReportingUnitModel {
            Default::default()
        }
    }
    ///The code for the corresponding ReportingUnit.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the corresponding ReportingUnit.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ReportingUnitModelReportingUnitCode(::std::string::String);
    impl ::std::ops::Deref for ReportingUnitModelReportingUnitCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<ReportingUnitModelReportingUnitCode>
    for ::std::string::String {
        fn from(value: ReportingUnitModelReportingUnitCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&ReportingUnitModelReportingUnitCode>
    for ReportingUnitModelReportingUnitCode {
        fn from(value: &ReportingUnitModelReportingUnitCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for ReportingUnitModelReportingUnitCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for ReportingUnitModelReportingUnitCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for ReportingUnitModelReportingUnitCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for ReportingUnitModelReportingUnitCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReportingUnitModelReportingUnitCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the corresponding ReportingUnit.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the corresponding ReportingUnit.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ReportingUnitModelReportingUnitName(::std::string::String);
    impl ::std::ops::Deref for ReportingUnitModelReportingUnitName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<ReportingUnitModelReportingUnitName>
    for ::std::string::String {
        fn from(value: ReportingUnitModelReportingUnitName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&ReportingUnitModelReportingUnitName>
    for ReportingUnitModelReportingUnitName {
        fn from(value: &ReportingUnitModelReportingUnitName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for ReportingUnitModelReportingUnitName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for ReportingUnitModelReportingUnitName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for ReportingUnitModelReportingUnitName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for ReportingUnitModelReportingUnitName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReportingUnitModelReportingUnitName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///A ReportingUnitSummary details the code and name of a ReportingUnit, including a summary of the associated ReportingUnitType.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A ReportingUnitSummary details the code and name of a ReportingUnit, including a summary of the associated ReportingUnitType.",
    ///  "type": "object",
    ///  "required": [
    ///    "reporting_unit_code",
    ///    "reporting_unit_name",
    ///    "reporting_unit_type"
    ///  ],
    ///  "properties": {
    ///    "reporting_unit_code": {
    ///      "description": "The code for the corresponding ReportingUnit.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reporting_unit_name": {
    ///      "description": "The name for the corresponding ReportingUnit.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reporting_unit_type": {
    ///      "$ref": "#/components/schemas/ReportingUnitTypeModel"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ReportingUnitSummaryModel {
        ///The code for the corresponding ReportingUnit.
        pub reporting_unit_code: ReportingUnitSummaryModelReportingUnitCode,
        ///The name for the corresponding ReportingUnit.
        pub reporting_unit_name: ReportingUnitSummaryModelReportingUnitName,
        pub reporting_unit_type: ReportingUnitTypeModel,
    }
    impl ::std::convert::From<&ReportingUnitSummaryModel> for ReportingUnitSummaryModel {
        fn from(value: &ReportingUnitSummaryModel) -> Self {
            value.clone()
        }
    }
    impl ReportingUnitSummaryModel {
        pub fn builder() -> builder::ReportingUnitSummaryModel {
            Default::default()
        }
    }
    ///The code for the corresponding ReportingUnit.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the corresponding ReportingUnit.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ReportingUnitSummaryModelReportingUnitCode(::std::string::String);
    impl ::std::ops::Deref for ReportingUnitSummaryModelReportingUnitCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<ReportingUnitSummaryModelReportingUnitCode>
    for ::std::string::String {
        fn from(value: ReportingUnitSummaryModelReportingUnitCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&ReportingUnitSummaryModelReportingUnitCode>
    for ReportingUnitSummaryModelReportingUnitCode {
        fn from(value: &ReportingUnitSummaryModelReportingUnitCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for ReportingUnitSummaryModelReportingUnitCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for ReportingUnitSummaryModelReportingUnitCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for ReportingUnitSummaryModelReportingUnitCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for ReportingUnitSummaryModelReportingUnitCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReportingUnitSummaryModelReportingUnitCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the corresponding ReportingUnit.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the corresponding ReportingUnit.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ReportingUnitSummaryModelReportingUnitName(::std::string::String);
    impl ::std::ops::Deref for ReportingUnitSummaryModelReportingUnitName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<ReportingUnitSummaryModelReportingUnitName>
    for ::std::string::String {
        fn from(value: ReportingUnitSummaryModelReportingUnitName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&ReportingUnitSummaryModelReportingUnitName>
    for ReportingUnitSummaryModelReportingUnitName {
        fn from(value: &ReportingUnitSummaryModelReportingUnitName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for ReportingUnitSummaryModelReportingUnitName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for ReportingUnitSummaryModelReportingUnitName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for ReportingUnitSummaryModelReportingUnitName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for ReportingUnitSummaryModelReportingUnitName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReportingUnitSummaryModelReportingUnitName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///A ReportingUnitType indicates the type for a ReportingUnit and is used to group ReportingUnits.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A ReportingUnitType indicates the type for a ReportingUnit and is used to group ReportingUnits.",
    ///  "type": "object",
    ///  "required": [
    ///    "reporting_unit_type_code",
    ///    "reporting_unit_type_name"
    ///  ],
    ///  "properties": {
    ///    "reporting_unit_type_code": {
    ///      "description": "The code for the corresponding ReportingUnitType.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "reporting_unit_type_name": {
    ///      "description": "The name for the corresponding ReportingUnitType.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct ReportingUnitTypeModel {
        ///The code for the corresponding ReportingUnitType.
        pub reporting_unit_type_code: ReportingUnitTypeModelReportingUnitTypeCode,
        ///The name for the corresponding ReportingUnitType.
        pub reporting_unit_type_name: ReportingUnitTypeModelReportingUnitTypeName,
    }
    impl ::std::convert::From<&ReportingUnitTypeModel> for ReportingUnitTypeModel {
        fn from(value: &ReportingUnitTypeModel) -> Self {
            value.clone()
        }
    }
    impl ReportingUnitTypeModel {
        pub fn builder() -> builder::ReportingUnitTypeModel {
            Default::default()
        }
    }
    ///The code for the corresponding ReportingUnitType.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the corresponding ReportingUnitType.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ReportingUnitTypeModelReportingUnitTypeCode(::std::string::String);
    impl ::std::ops::Deref for ReportingUnitTypeModelReportingUnitTypeCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<ReportingUnitTypeModelReportingUnitTypeCode>
    for ::std::string::String {
        fn from(value: ReportingUnitTypeModelReportingUnitTypeCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&ReportingUnitTypeModelReportingUnitTypeCode>
    for ReportingUnitTypeModelReportingUnitTypeCode {
        fn from(value: &ReportingUnitTypeModelReportingUnitTypeCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for ReportingUnitTypeModelReportingUnitTypeCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for ReportingUnitTypeModelReportingUnitTypeCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for ReportingUnitTypeModelReportingUnitTypeCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for ReportingUnitTypeModelReportingUnitTypeCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReportingUnitTypeModelReportingUnitTypeCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the corresponding ReportingUnitType.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the corresponding ReportingUnitType.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ReportingUnitTypeModelReportingUnitTypeName(::std::string::String);
    impl ::std::ops::Deref for ReportingUnitTypeModelReportingUnitTypeName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<ReportingUnitTypeModelReportingUnitTypeName>
    for ::std::string::String {
        fn from(value: ReportingUnitTypeModelReportingUnitTypeName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&ReportingUnitTypeModelReportingUnitTypeName>
    for ReportingUnitTypeModelReportingUnitTypeName {
        fn from(value: &ReportingUnitTypeModelReportingUnitTypeName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for ReportingUnitTypeModelReportingUnitTypeName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for ReportingUnitTypeModelReportingUnitTypeName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for ReportingUnitTypeModelReportingUnitTypeName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for ReportingUnitTypeModelReportingUnitTypeName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ReportingUnitTypeModelReportingUnitTypeName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///A Suppression is a footnote or caveat used to apply to, or instead of, a data value. This is now obsolete and has been replaced by the CaveatModel.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "A Suppression is a footnote or caveat used to apply to, or instead of, a data value. This is now obsolete and has been replaced by the CaveatModel.",
    ///  "type": "object",
    ///  "required": [
    ///    "suppression_code",
    ///    "suppression_name"
    ///  ],
    ///  "properties": {
    ///    "suppression_code": {
    ///      "description": "The code for the corresponding Suppression.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    },
    ///    "suppression_display_value": {
    ///      "description": "The display value or symbol for the corresponding Suppression. If the Suppression has no symbol then this field will be null.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "suppression_footnote": {
    ///      "description": "The footnote/explanation text for the corresponding Suppression. If the Suppression has no text then this field will be null.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "suppression_name": {
    ///      "description": "The name for the corresponding Suppression.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct SuppressionModel {
        ///The code for the corresponding Suppression.
        pub suppression_code: SuppressionModelSuppressionCode,
        ///The display value or symbol for the corresponding Suppression. If the Suppression has no symbol then this field will be null.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub suppression_display_value: ::std::option::Option<::std::string::String>,
        ///The footnote/explanation text for the corresponding Suppression. If the Suppression has no text then this field will be null.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub suppression_footnote: ::std::option::Option<::std::string::String>,
        ///The name for the corresponding Suppression.
        pub suppression_name: SuppressionModelSuppressionName,
    }
    impl ::std::convert::From<&SuppressionModel> for SuppressionModel {
        fn from(value: &SuppressionModel) -> Self {
            value.clone()
        }
    }
    impl SuppressionModel {
        pub fn builder() -> builder::SuppressionModel {
            Default::default()
        }
    }
    ///The code for the corresponding Suppression.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The code for the corresponding Suppression.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct SuppressionModelSuppressionCode(::std::string::String);
    impl ::std::ops::Deref for SuppressionModelSuppressionCode {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<SuppressionModelSuppressionCode>
    for ::std::string::String {
        fn from(value: SuppressionModelSuppressionCode) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&SuppressionModelSuppressionCode>
    for SuppressionModelSuppressionCode {
        fn from(value: &SuppressionModelSuppressionCode) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for SuppressionModelSuppressionCode {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for SuppressionModelSuppressionCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for SuppressionModelSuppressionCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for SuppressionModelSuppressionCode {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SuppressionModelSuppressionCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///The name for the corresponding Suppression.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the corresponding Suppression.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct SuppressionModelSuppressionName(::std::string::String);
    impl ::std::ops::Deref for SuppressionModelSuppressionName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<SuppressionModelSuppressionName>
    for ::std::string::String {
        fn from(value: SuppressionModelSuppressionName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&SuppressionModelSuppressionName>
    for SuppressionModelSuppressionName {
        fn from(value: &SuppressionModelSuppressionName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for SuppressionModelSuppressionName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for SuppressionModelSuppressionName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String>
    for SuppressionModelSuppressionName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String>
    for SuppressionModelSuppressionName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SuppressionModelSuppressionName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///Units indicates how data values for a Measure should be expressed.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Units indicates how data values for a Measure should be expressed.",
    ///  "type": "object",
    ///  "required": [
    ///    "decimal_places",
    ///    "units_are_prefix",
    ///    "units_name"
    ///  ],
    ///  "properties": {
    ///    "decimal_places": {
    ///      "description": "The number of decimal places that the data can be expressed to.",
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "units_are_prefix": {
    ///      "description": "Whether the units should be applied as a prefix or suffix to the data value. e.g. $ (prefix) and % (suffix).",
    ///      "type": "boolean"
    ///    },
    ///    "units_display": {
    ///      "description": "The display symbol or text for the units. For units without a symbol this will be null.",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "units_name": {
    ///      "description": "The name for the corresponding Units.",
    ///      "type": "string",
    ///      "minLength": 1
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct UnitsModel {
        ///The number of decimal places that the data can be expressed to.
        pub decimal_places: i32,
        ///Whether the units should be applied as a prefix or suffix to the data value. e.g. $ (prefix) and % (suffix).
        pub units_are_prefix: bool,
        ///The display symbol or text for the units. For units without a symbol this will be null.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub units_display: ::std::option::Option<::std::string::String>,
        ///The name for the corresponding Units.
        pub units_name: UnitsModelUnitsName,
    }
    impl ::std::convert::From<&UnitsModel> for UnitsModel {
        fn from(value: &UnitsModel) -> Self {
            value.clone()
        }
    }
    impl UnitsModel {
        pub fn builder() -> builder::UnitsModel {
            Default::default()
        }
    }
    ///The name for the corresponding Units.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The name for the corresponding Units.",
    ///  "type": "string",
    ///  "minLength": 1
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct UnitsModelUnitsName(::std::string::String);
    impl ::std::ops::Deref for UnitsModelUnitsName {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }
    impl ::std::convert::From<UnitsModelUnitsName> for ::std::string::String {
        fn from(value: UnitsModelUnitsName) -> Self {
            value.0
        }
    }
    impl ::std::convert::From<&UnitsModelUnitsName> for UnitsModelUnitsName {
        fn from(value: &UnitsModelUnitsName) -> Self {
            value.clone()
        }
    }
    impl ::std::str::FromStr for UnitsModelUnitsName {
        type Err = self::error::ConversionError;
        fn from_str(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() < 1usize {
                return Err("shorter than 1 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }
    impl ::std::convert::TryFrom<&str> for UnitsModelUnitsName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &str,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<&::std::string::String> for UnitsModelUnitsName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl ::std::convert::TryFrom<::std::string::String> for UnitsModelUnitsName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl<'de> ::serde::Deserialize<'de> for UnitsModelUnitsName {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }
    ///`VersionInformation`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "api_version": {
    ///      "type": "string"
    ///    },
    ///    "data_version": {
    ///      "type": "integer"
    ///    },
    ///    "date_uploaded": {
    ///      "type": "string"
    ///    },
    ///    "requested_time_stamp": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
    pub struct VersionInformation {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub api_version: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub data_version: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub date_uploaded: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub requested_time_stamp: ::std::option::Option<::std::string::String>,
    }
    impl ::std::convert::From<&VersionInformation> for VersionInformation {
        fn from(value: &VersionInformation) -> Self {
            value.clone()
        }
    }
    impl ::std::default::Default for VersionInformation {
        fn default() -> Self {
            Self {
                api_version: Default::default(),
                data_version: Default::default(),
                date_uploaded: Default::default(),
                requested_time_stamp: Default::default(),
            }
        }
    }
    impl VersionInformation {
        pub fn builder() -> builder::VersionInformation {
            Default::default()
        }
    }
    /// Types for composing complex structures.
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct CaveatModel {
            caveat_code: ::std::result::Result<
                super::CaveatModelCaveatCode,
                ::std::string::String,
            >,
            caveat_display_value: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            caveat_footnote: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            caveat_name: ::std::result::Result<
                super::CaveatModelCaveatName,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for CaveatModel {
            fn default() -> Self {
                Self {
                    caveat_code: Err("no value supplied for caveat_code".to_string()),
                    caveat_display_value: Ok(Default::default()),
                    caveat_footnote: Ok(Default::default()),
                    caveat_name: Err("no value supplied for caveat_name".to_string()),
                }
            }
        }
        impl CaveatModel {
            pub fn caveat_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::CaveatModelCaveatCode>,
                T::Error: ::std::fmt::Display,
            {
                self.caveat_code = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for caveat_code: {}", e)
                    });
                self
            }
            pub fn caveat_display_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.caveat_display_value = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for caveat_display_value: {}",
                            e
                        )
                    });
                self
            }
            pub fn caveat_footnote<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.caveat_footnote = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for caveat_footnote: {}", e
                        )
                    });
                self
            }
            pub fn caveat_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::CaveatModelCaveatName>,
                T::Error: ::std::fmt::Display,
            {
                self.caveat_name = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for caveat_name: {}", e)
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<CaveatModel> for super::CaveatModel {
            type Error = super::error::ConversionError;
            fn try_from(
                value: CaveatModel,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    caveat_code: value.caveat_code?,
                    caveat_display_value: value.caveat_display_value?,
                    caveat_footnote: value.caveat_footnote?,
                    caveat_name: value.caveat_name?,
                })
            }
        }
        impl ::std::convert::From<super::CaveatModel> for CaveatModel {
            fn from(value: super::CaveatModel) -> Self {
                Self {
                    caveat_code: Ok(value.caveat_code),
                    caveat_display_value: Ok(value.caveat_display_value),
                    caveat_footnote: Ok(value.caveat_footnote),
                    caveat_name: Ok(value.caveat_name),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct DataExtractModel {
            caveat: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            caveat_codes: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            caveat_footnotes: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            data_set_caveat: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            data_set_caveat_codes: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            data_set_caveat_footnotes: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            data_set_meta_tag_codes: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            data_set_meta_tags: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            data_set_meta_tags_with_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            decimal_places: ::std::result::Result<i32, ::std::string::String>,
            group_number: ::std::result::Result<
                ::std::option::Option<i32>,
                ::std::string::String,
            >,
            lower_value: ::std::result::Result<
                ::std::option::Option<f64>,
                ::std::string::String,
            >,
            mapped_state: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            measure_category_code: ::std::result::Result<
                super::DataExtractModelMeasureCategoryCode,
                ::std::string::String,
            >,
            measure_code: ::std::result::Result<
                super::DataExtractModelMeasureCode,
                ::std::string::String,
            >,
            measure_meta_tag_codes: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            measure_meta_tags: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            measure_meta_tags_with_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            measure_name: ::std::result::Result<
                super::DataExtractModelMeasureName,
                ::std::string::String,
            >,
            peer_caveat: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            peer_caveat_codes: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            peer_caveat_footnotes: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            peer_group_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            peer_suppression: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            peer_suppression_codes: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            peer_value: ::std::result::Result<
                ::std::option::Option<f64>,
                ::std::string::String,
            >,
            proxy_children: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            proxy_reporting_unit_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reported_measure_category_code: ::std::result::Result<
                super::DataExtractModelReportedMeasureCategoryCode,
                ::std::string::String,
            >,
            reported_measure_category_name: ::std::result::Result<
                super::DataExtractModelReportedMeasureCategoryName,
                ::std::string::String,
            >,
            reported_measure_category_three_code: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reported_measure_category_three_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reported_measure_category_three_type_code: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reported_measure_category_three_type_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reported_measure_category_two_code: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reported_measure_category_two_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reported_measure_category_two_type_code: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reported_measure_category_two_type_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reported_measure_category_type_code: ::std::result::Result<
                super::DataExtractModelReportedMeasureCategoryTypeCode,
                ::std::string::String,
            >,
            reported_measure_category_type_name: ::std::result::Result<
                super::DataExtractModelReportedMeasureCategoryTypeName,
                ::std::string::String,
            >,
            reported_measure_code: ::std::result::Result<
                super::DataExtractModelReportedMeasureCode,
                ::std::string::String,
            >,
            reported_measure_meta_tag_codes: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reported_measure_meta_tags: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reported_measure_meta_tags_with_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reported_measure_name: ::std::result::Result<
                super::DataExtractModelReportedMeasureName,
                ::std::string::String,
            >,
            reporting_end_date: ::std::result::Result<
                ::chrono::naive::NaiveDate,
                ::std::string::String,
            >,
            reporting_start_date: ::std::result::Result<
                ::chrono::naive::NaiveDate,
                ::std::string::String,
            >,
            reporting_unit_code: ::std::result::Result<
                super::DataExtractModelReportingUnitCode,
                ::std::string::String,
            >,
            reporting_unit_meta_tag_codes: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reporting_unit_meta_tags: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reporting_unit_meta_tags_with_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reporting_unit_name: ::std::result::Result<
                super::DataExtractModelReportingUnitName,
                ::std::string::String,
            >,
            reporting_unit_type_code: ::std::result::Result<
                super::DataExtractModelReportingUnitTypeCode,
                ::std::string::String,
            >,
            reporting_unit_type_name: ::std::result::Result<
                super::DataExtractModelReportingUnitTypeName,
                ::std::string::String,
            >,
            suppression: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            suppression_codes: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            units_are_prefix: ::std::result::Result<bool, ::std::string::String>,
            units_display: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            units_name: ::std::result::Result<
                super::DataExtractModelUnitsName,
                ::std::string::String,
            >,
            upper_value: ::std::result::Result<
                ::std::option::Option<f64>,
                ::std::string::String,
            >,
            value: ::std::result::Result<
                ::std::option::Option<f64>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for DataExtractModel {
            fn default() -> Self {
                Self {
                    caveat: Ok(Default::default()),
                    caveat_codes: Ok(Default::default()),
                    caveat_footnotes: Ok(Default::default()),
                    data_set_caveat: Ok(Default::default()),
                    data_set_caveat_codes: Ok(Default::default()),
                    data_set_caveat_footnotes: Ok(Default::default()),
                    data_set_meta_tag_codes: Ok(Default::default()),
                    data_set_meta_tags: Ok(Default::default()),
                    data_set_meta_tags_with_type: Ok(Default::default()),
                    decimal_places: Err(
                        "no value supplied for decimal_places".to_string(),
                    ),
                    group_number: Ok(Default::default()),
                    lower_value: Ok(Default::default()),
                    mapped_state: Ok(Default::default()),
                    measure_category_code: Err(
                        "no value supplied for measure_category_code".to_string(),
                    ),
                    measure_code: Err("no value supplied for measure_code".to_string()),
                    measure_meta_tag_codes: Ok(Default::default()),
                    measure_meta_tags: Ok(Default::default()),
                    measure_meta_tags_with_type: Ok(Default::default()),
                    measure_name: Err("no value supplied for measure_name".to_string()),
                    peer_caveat: Ok(Default::default()),
                    peer_caveat_codes: Ok(Default::default()),
                    peer_caveat_footnotes: Ok(Default::default()),
                    peer_group_name: Ok(Default::default()),
                    peer_suppression: Ok(Default::default()),
                    peer_suppression_codes: Ok(Default::default()),
                    peer_value: Ok(Default::default()),
                    proxy_children: Ok(Default::default()),
                    proxy_reporting_unit_name: Ok(Default::default()),
                    reported_measure_category_code: Err(
                        "no value supplied for reported_measure_category_code"
                            .to_string(),
                    ),
                    reported_measure_category_name: Err(
                        "no value supplied for reported_measure_category_name"
                            .to_string(),
                    ),
                    reported_measure_category_three_code: Ok(Default::default()),
                    reported_measure_category_three_name: Ok(Default::default()),
                    reported_measure_category_three_type_code: Ok(Default::default()),
                    reported_measure_category_three_type_name: Ok(Default::default()),
                    reported_measure_category_two_code: Ok(Default::default()),
                    reported_measure_category_two_name: Ok(Default::default()),
                    reported_measure_category_two_type_code: Ok(Default::default()),
                    reported_measure_category_two_type_name: Ok(Default::default()),
                    reported_measure_category_type_code: Err(
                        "no value supplied for reported_measure_category_type_code"
                            .to_string(),
                    ),
                    reported_measure_category_type_name: Err(
                        "no value supplied for reported_measure_category_type_name"
                            .to_string(),
                    ),
                    reported_measure_code: Err(
                        "no value supplied for reported_measure_code".to_string(),
                    ),
                    reported_measure_meta_tag_codes: Ok(Default::default()),
                    reported_measure_meta_tags: Ok(Default::default()),
                    reported_measure_meta_tags_with_type: Ok(Default::default()),
                    reported_measure_name: Err(
                        "no value supplied for reported_measure_name".to_string(),
                    ),
                    reporting_end_date: Err(
                        "no value supplied for reporting_end_date".to_string(),
                    ),
                    reporting_start_date: Err(
                        "no value supplied for reporting_start_date".to_string(),
                    ),
                    reporting_unit_code: Err(
                        "no value supplied for reporting_unit_code".to_string(),
                    ),
                    reporting_unit_meta_tag_codes: Ok(Default::default()),
                    reporting_unit_meta_tags: Ok(Default::default()),
                    reporting_unit_meta_tags_with_type: Ok(Default::default()),
                    reporting_unit_name: Err(
                        "no value supplied for reporting_unit_name".to_string(),
                    ),
                    reporting_unit_type_code: Err(
                        "no value supplied for reporting_unit_type_code".to_string(),
                    ),
                    reporting_unit_type_name: Err(
                        "no value supplied for reporting_unit_type_name".to_string(),
                    ),
                    suppression: Ok(Default::default()),
                    suppression_codes: Ok(Default::default()),
                    units_are_prefix: Err(
                        "no value supplied for units_are_prefix".to_string(),
                    ),
                    units_display: Ok(Default::default()),
                    units_name: Err("no value supplied for units_name".to_string()),
                    upper_value: Ok(Default::default()),
                    value: Ok(Default::default()),
                }
            }
        }
        impl DataExtractModel {
            pub fn caveat<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.caveat = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for caveat: {}", e)
                    });
                self
            }
            pub fn caveat_codes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.caveat_codes = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for caveat_codes: {}", e
                        )
                    });
                self
            }
            pub fn caveat_footnotes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.caveat_footnotes = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for caveat_footnotes: {}", e
                        )
                    });
                self
            }
            pub fn data_set_caveat<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.data_set_caveat = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for data_set_caveat: {}", e
                        )
                    });
                self
            }
            pub fn data_set_caveat_codes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.data_set_caveat_codes = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for data_set_caveat_codes: {}",
                            e
                        )
                    });
                self
            }
            pub fn data_set_caveat_footnotes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.data_set_caveat_footnotes = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for data_set_caveat_footnotes: {}",
                            e
                        )
                    });
                self
            }
            pub fn data_set_meta_tag_codes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.data_set_meta_tag_codes = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for data_set_meta_tag_codes: {}",
                            e
                        )
                    });
                self
            }
            pub fn data_set_meta_tags<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.data_set_meta_tags = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for data_set_meta_tags: {}",
                            e
                        )
                    });
                self
            }
            pub fn data_set_meta_tags_with_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.data_set_meta_tags_with_type = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for data_set_meta_tags_with_type: {}",
                            e
                        )
                    });
                self
            }
            pub fn decimal_places<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i32>,
                T::Error: ::std::fmt::Display,
            {
                self.decimal_places = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for decimal_places: {}", e
                        )
                    });
                self
            }
            pub fn group_number<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.group_number = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for group_number: {}", e
                        )
                    });
                self
            }
            pub fn lower_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.lower_value = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for lower_value: {}", e)
                    });
                self
            }
            pub fn mapped_state<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.mapped_state = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for mapped_state: {}", e
                        )
                    });
                self
            }
            pub fn measure_category_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::DataExtractModelMeasureCategoryCode>,
                T::Error: ::std::fmt::Display,
            {
                self.measure_category_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for measure_category_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn measure_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::DataExtractModelMeasureCode>,
                T::Error: ::std::fmt::Display,
            {
                self.measure_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for measure_code: {}", e
                        )
                    });
                self
            }
            pub fn measure_meta_tag_codes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.measure_meta_tag_codes = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for measure_meta_tag_codes: {}",
                            e
                        )
                    });
                self
            }
            pub fn measure_meta_tags<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.measure_meta_tags = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for measure_meta_tags: {}",
                            e
                        )
                    });
                self
            }
            pub fn measure_meta_tags_with_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.measure_meta_tags_with_type = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for measure_meta_tags_with_type: {}",
                            e
                        )
                    });
                self
            }
            pub fn measure_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::DataExtractModelMeasureName>,
                T::Error: ::std::fmt::Display,
            {
                self.measure_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for measure_name: {}", e
                        )
                    });
                self
            }
            pub fn peer_caveat<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.peer_caveat = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for peer_caveat: {}", e)
                    });
                self
            }
            pub fn peer_caveat_codes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.peer_caveat_codes = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for peer_caveat_codes: {}",
                            e
                        )
                    });
                self
            }
            pub fn peer_caveat_footnotes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.peer_caveat_footnotes = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for peer_caveat_footnotes: {}",
                            e
                        )
                    });
                self
            }
            pub fn peer_group_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.peer_group_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for peer_group_name: {}", e
                        )
                    });
                self
            }
            pub fn peer_suppression<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.peer_suppression = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for peer_suppression: {}", e
                        )
                    });
                self
            }
            pub fn peer_suppression_codes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.peer_suppression_codes = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for peer_suppression_codes: {}",
                            e
                        )
                    });
                self
            }
            pub fn peer_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.peer_value = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for peer_value: {}", e)
                    });
                self
            }
            pub fn proxy_children<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.proxy_children = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for proxy_children: {}", e
                        )
                    });
                self
            }
            pub fn proxy_reporting_unit_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.proxy_reporting_unit_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for proxy_reporting_unit_name: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::DataExtractModelReportedMeasureCategoryCode,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::DataExtractModelReportedMeasureCategoryName,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_name: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_three_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_three_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_three_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_three_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_three_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_three_name: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_three_type_code<T>(
                mut self,
                value: T,
            ) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_three_type_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_three_type_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_three_type_name<T>(
                mut self,
                value: T,
            ) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_three_type_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_three_type_name: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_two_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_two_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_two_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_two_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_two_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_two_name: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_two_type_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_two_type_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_two_type_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_two_type_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_two_type_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_two_type_name: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_type_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::DataExtractModelReportedMeasureCategoryTypeCode,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_type_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_type_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_type_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::DataExtractModelReportedMeasureCategoryTypeName,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_type_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_type_name: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::DataExtractModelReportedMeasureCode>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_meta_tag_codes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_meta_tag_codes = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_meta_tag_codes: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_meta_tags<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_meta_tags = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_meta_tags: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_meta_tags_with_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_meta_tags_with_type = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_meta_tags_with_type: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::DataExtractModelReportedMeasureName>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_name: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_end_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::naive::NaiveDate>,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_end_date = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_end_date: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_start_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::naive::NaiveDate>,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_start_date = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_start_date: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_unit_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::DataExtractModelReportingUnitCode>,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_unit_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_unit_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_unit_meta_tag_codes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_unit_meta_tag_codes = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_unit_meta_tag_codes: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_unit_meta_tags<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_unit_meta_tags = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_unit_meta_tags: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_unit_meta_tags_with_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_unit_meta_tags_with_type = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_unit_meta_tags_with_type: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_unit_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::DataExtractModelReportingUnitName>,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_unit_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_unit_name: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_unit_type_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::DataExtractModelReportingUnitTypeCode>,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_unit_type_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_unit_type_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_unit_type_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::DataExtractModelReportingUnitTypeName>,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_unit_type_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_unit_type_name: {}",
                            e
                        )
                    });
                self
            }
            pub fn suppression<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.suppression = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for suppression: {}", e)
                    });
                self
            }
            pub fn suppression_codes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.suppression_codes = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for suppression_codes: {}",
                            e
                        )
                    });
                self
            }
            pub fn units_are_prefix<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.units_are_prefix = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for units_are_prefix: {}", e
                        )
                    });
                self
            }
            pub fn units_display<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.units_display = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for units_display: {}", e
                        )
                    });
                self
            }
            pub fn units_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::DataExtractModelUnitsName>,
                T::Error: ::std::fmt::Display,
            {
                self.units_name = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for units_name: {}", e)
                    });
                self
            }
            pub fn upper_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.upper_value = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for upper_value: {}", e)
                    });
                self
            }
            pub fn value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.value = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for value: {}", e)
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<DataExtractModel> for super::DataExtractModel {
            type Error = super::error::ConversionError;
            fn try_from(
                value: DataExtractModel,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    caveat: value.caveat?,
                    caveat_codes: value.caveat_codes?,
                    caveat_footnotes: value.caveat_footnotes?,
                    data_set_caveat: value.data_set_caveat?,
                    data_set_caveat_codes: value.data_set_caveat_codes?,
                    data_set_caveat_footnotes: value.data_set_caveat_footnotes?,
                    data_set_meta_tag_codes: value.data_set_meta_tag_codes?,
                    data_set_meta_tags: value.data_set_meta_tags?,
                    data_set_meta_tags_with_type: value.data_set_meta_tags_with_type?,
                    decimal_places: value.decimal_places?,
                    group_number: value.group_number?,
                    lower_value: value.lower_value?,
                    mapped_state: value.mapped_state?,
                    measure_category_code: value.measure_category_code?,
                    measure_code: value.measure_code?,
                    measure_meta_tag_codes: value.measure_meta_tag_codes?,
                    measure_meta_tags: value.measure_meta_tags?,
                    measure_meta_tags_with_type: value.measure_meta_tags_with_type?,
                    measure_name: value.measure_name?,
                    peer_caveat: value.peer_caveat?,
                    peer_caveat_codes: value.peer_caveat_codes?,
                    peer_caveat_footnotes: value.peer_caveat_footnotes?,
                    peer_group_name: value.peer_group_name?,
                    peer_suppression: value.peer_suppression?,
                    peer_suppression_codes: value.peer_suppression_codes?,
                    peer_value: value.peer_value?,
                    proxy_children: value.proxy_children?,
                    proxy_reporting_unit_name: value.proxy_reporting_unit_name?,
                    reported_measure_category_code: value
                        .reported_measure_category_code?,
                    reported_measure_category_name: value
                        .reported_measure_category_name?,
                    reported_measure_category_three_code: value
                        .reported_measure_category_three_code?,
                    reported_measure_category_three_name: value
                        .reported_measure_category_three_name?,
                    reported_measure_category_three_type_code: value
                        .reported_measure_category_three_type_code?,
                    reported_measure_category_three_type_name: value
                        .reported_measure_category_three_type_name?,
                    reported_measure_category_two_code: value
                        .reported_measure_category_two_code?,
                    reported_measure_category_two_name: value
                        .reported_measure_category_two_name?,
                    reported_measure_category_two_type_code: value
                        .reported_measure_category_two_type_code?,
                    reported_measure_category_two_type_name: value
                        .reported_measure_category_two_type_name?,
                    reported_measure_category_type_code: value
                        .reported_measure_category_type_code?,
                    reported_measure_category_type_name: value
                        .reported_measure_category_type_name?,
                    reported_measure_code: value.reported_measure_code?,
                    reported_measure_meta_tag_codes: value
                        .reported_measure_meta_tag_codes?,
                    reported_measure_meta_tags: value.reported_measure_meta_tags?,
                    reported_measure_meta_tags_with_type: value
                        .reported_measure_meta_tags_with_type?,
                    reported_measure_name: value.reported_measure_name?,
                    reporting_end_date: value.reporting_end_date?,
                    reporting_start_date: value.reporting_start_date?,
                    reporting_unit_code: value.reporting_unit_code?,
                    reporting_unit_meta_tag_codes: value.reporting_unit_meta_tag_codes?,
                    reporting_unit_meta_tags: value.reporting_unit_meta_tags?,
                    reporting_unit_meta_tags_with_type: value
                        .reporting_unit_meta_tags_with_type?,
                    reporting_unit_name: value.reporting_unit_name?,
                    reporting_unit_type_code: value.reporting_unit_type_code?,
                    reporting_unit_type_name: value.reporting_unit_type_name?,
                    suppression: value.suppression?,
                    suppression_codes: value.suppression_codes?,
                    units_are_prefix: value.units_are_prefix?,
                    units_display: value.units_display?,
                    units_name: value.units_name?,
                    upper_value: value.upper_value?,
                    value: value.value?,
                })
            }
        }
        impl ::std::convert::From<super::DataExtractModel> for DataExtractModel {
            fn from(value: super::DataExtractModel) -> Self {
                Self {
                    caveat: Ok(value.caveat),
                    caveat_codes: Ok(value.caveat_codes),
                    caveat_footnotes: Ok(value.caveat_footnotes),
                    data_set_caveat: Ok(value.data_set_caveat),
                    data_set_caveat_codes: Ok(value.data_set_caveat_codes),
                    data_set_caveat_footnotes: Ok(value.data_set_caveat_footnotes),
                    data_set_meta_tag_codes: Ok(value.data_set_meta_tag_codes),
                    data_set_meta_tags: Ok(value.data_set_meta_tags),
                    data_set_meta_tags_with_type: Ok(value.data_set_meta_tags_with_type),
                    decimal_places: Ok(value.decimal_places),
                    group_number: Ok(value.group_number),
                    lower_value: Ok(value.lower_value),
                    mapped_state: Ok(value.mapped_state),
                    measure_category_code: Ok(value.measure_category_code),
                    measure_code: Ok(value.measure_code),
                    measure_meta_tag_codes: Ok(value.measure_meta_tag_codes),
                    measure_meta_tags: Ok(value.measure_meta_tags),
                    measure_meta_tags_with_type: Ok(value.measure_meta_tags_with_type),
                    measure_name: Ok(value.measure_name),
                    peer_caveat: Ok(value.peer_caveat),
                    peer_caveat_codes: Ok(value.peer_caveat_codes),
                    peer_caveat_footnotes: Ok(value.peer_caveat_footnotes),
                    peer_group_name: Ok(value.peer_group_name),
                    peer_suppression: Ok(value.peer_suppression),
                    peer_suppression_codes: Ok(value.peer_suppression_codes),
                    peer_value: Ok(value.peer_value),
                    proxy_children: Ok(value.proxy_children),
                    proxy_reporting_unit_name: Ok(value.proxy_reporting_unit_name),
                    reported_measure_category_code: Ok(
                        value.reported_measure_category_code,
                    ),
                    reported_measure_category_name: Ok(
                        value.reported_measure_category_name,
                    ),
                    reported_measure_category_three_code: Ok(
                        value.reported_measure_category_three_code,
                    ),
                    reported_measure_category_three_name: Ok(
                        value.reported_measure_category_three_name,
                    ),
                    reported_measure_category_three_type_code: Ok(
                        value.reported_measure_category_three_type_code,
                    ),
                    reported_measure_category_three_type_name: Ok(
                        value.reported_measure_category_three_type_name,
                    ),
                    reported_measure_category_two_code: Ok(
                        value.reported_measure_category_two_code,
                    ),
                    reported_measure_category_two_name: Ok(
                        value.reported_measure_category_two_name,
                    ),
                    reported_measure_category_two_type_code: Ok(
                        value.reported_measure_category_two_type_code,
                    ),
                    reported_measure_category_two_type_name: Ok(
                        value.reported_measure_category_two_type_name,
                    ),
                    reported_measure_category_type_code: Ok(
                        value.reported_measure_category_type_code,
                    ),
                    reported_measure_category_type_name: Ok(
                        value.reported_measure_category_type_name,
                    ),
                    reported_measure_code: Ok(value.reported_measure_code),
                    reported_measure_meta_tag_codes: Ok(
                        value.reported_measure_meta_tag_codes,
                    ),
                    reported_measure_meta_tags: Ok(value.reported_measure_meta_tags),
                    reported_measure_meta_tags_with_type: Ok(
                        value.reported_measure_meta_tags_with_type,
                    ),
                    reported_measure_name: Ok(value.reported_measure_name),
                    reporting_end_date: Ok(value.reporting_end_date),
                    reporting_start_date: Ok(value.reporting_start_date),
                    reporting_unit_code: Ok(value.reporting_unit_code),
                    reporting_unit_meta_tag_codes: Ok(
                        value.reporting_unit_meta_tag_codes,
                    ),
                    reporting_unit_meta_tags: Ok(value.reporting_unit_meta_tags),
                    reporting_unit_meta_tags_with_type: Ok(
                        value.reporting_unit_meta_tags_with_type,
                    ),
                    reporting_unit_name: Ok(value.reporting_unit_name),
                    reporting_unit_type_code: Ok(value.reporting_unit_type_code),
                    reporting_unit_type_name: Ok(value.reporting_unit_type_name),
                    suppression: Ok(value.suppression),
                    suppression_codes: Ok(value.suppression_codes),
                    units_are_prefix: Ok(value.units_are_prefix),
                    units_display: Ok(value.units_display),
                    units_name: Ok(value.units_name),
                    upper_value: Ok(value.upper_value),
                    value: Ok(value.value),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct DataExtractPagination {
            results_returned: ::std::result::Result<
                ::std::option::Option<i32>,
                ::std::string::String,
            >,
            starting_result_index: ::std::result::Result<
                ::std::option::Option<i32>,
                ::std::string::String,
            >,
            total_results_available: ::std::result::Result<
                ::std::option::Option<i32>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for DataExtractPagination {
            fn default() -> Self {
                Self {
                    results_returned: Ok(Default::default()),
                    starting_result_index: Ok(Default::default()),
                    total_results_available: Ok(Default::default()),
                }
            }
        }
        impl DataExtractPagination {
            pub fn results_returned<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.results_returned = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for results_returned: {}", e
                        )
                    });
                self
            }
            pub fn starting_result_index<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.starting_result_index = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for starting_result_index: {}",
                            e
                        )
                    });
                self
            }
            pub fn total_results_available<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.total_results_available = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for total_results_available: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<DataExtractPagination>
        for super::DataExtractPagination {
            type Error = super::error::ConversionError;
            fn try_from(
                value: DataExtractPagination,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    results_returned: value.results_returned?,
                    starting_result_index: value.starting_result_index?,
                    total_results_available: value.total_results_available?,
                })
            }
        }
        impl ::std::convert::From<super::DataExtractPagination>
        for DataExtractPagination {
            fn from(value: super::DataExtractPagination) -> Self {
                Self {
                    results_returned: Ok(value.results_returned),
                    starting_result_index: Ok(value.starting_result_index),
                    total_results_available: Ok(value.total_results_available),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct DataItemModel {
            caveats: ::std::result::Result<
                ::std::vec::Vec<super::CaveatModel>,
                ::std::string::String,
            >,
            data_set_id: ::std::result::Result<i32, ::std::string::String>,
            group_number: ::std::result::Result<
                ::std::option::Option<i32>,
                ::std::string::String,
            >,
            lower_value: ::std::result::Result<
                ::std::option::Option<f64>,
                ::std::string::String,
            >,
            measure_code: ::std::result::Result<
                super::DataItemModelMeasureCode,
                ::std::string::String,
            >,
            peer_group_summary: ::std::result::Result<
                ::std::option::Option<super::ReportingUnitSummaryModel>,
                ::std::string::String,
            >,
            proxy_reporting_unit_summary: ::std::result::Result<
                ::std::option::Option<super::ReportingUnitSummaryModel>,
                ::std::string::String,
            >,
            reported_measure_code: ::std::result::Result<
                super::DataItemModelReportedMeasureCode,
                ::std::string::String,
            >,
            reporting_unit_summary: ::std::result::Result<
                super::ReportingUnitSummaryModel,
                ::std::string::String,
            >,
            suppressions: ::std::result::Result<
                ::std::option::Option<::std::vec::Vec<super::SuppressionModel>>,
                ::std::string::String,
            >,
            upper_value: ::std::result::Result<
                ::std::option::Option<f64>,
                ::std::string::String,
            >,
            value: ::std::result::Result<
                ::std::option::Option<f64>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for DataItemModel {
            fn default() -> Self {
                Self {
                    caveats: Err("no value supplied for caveats".to_string()),
                    data_set_id: Err("no value supplied for data_set_id".to_string()),
                    group_number: Ok(Default::default()),
                    lower_value: Ok(Default::default()),
                    measure_code: Err("no value supplied for measure_code".to_string()),
                    peer_group_summary: Ok(Default::default()),
                    proxy_reporting_unit_summary: Ok(Default::default()),
                    reported_measure_code: Err(
                        "no value supplied for reported_measure_code".to_string(),
                    ),
                    reporting_unit_summary: Err(
                        "no value supplied for reporting_unit_summary".to_string(),
                    ),
                    suppressions: Ok(Default::default()),
                    upper_value: Ok(Default::default()),
                    value: Ok(Default::default()),
                }
            }
        }
        impl DataItemModel {
            pub fn caveats<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::CaveatModel>>,
                T::Error: ::std::fmt::Display,
            {
                self.caveats = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for caveats: {}", e)
                    });
                self
            }
            pub fn data_set_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i32>,
                T::Error: ::std::fmt::Display,
            {
                self.data_set_id = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for data_set_id: {}", e)
                    });
                self
            }
            pub fn group_number<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.group_number = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for group_number: {}", e
                        )
                    });
                self
            }
            pub fn lower_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.lower_value = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for lower_value: {}", e)
                    });
                self
            }
            pub fn measure_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::DataItemModelMeasureCode>,
                T::Error: ::std::fmt::Display,
            {
                self.measure_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for measure_code: {}", e
                        )
                    });
                self
            }
            pub fn peer_group_summary<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::ReportingUnitSummaryModel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.peer_group_summary = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for peer_group_summary: {}",
                            e
                        )
                    });
                self
            }
            pub fn proxy_reporting_unit_summary<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::ReportingUnitSummaryModel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.proxy_reporting_unit_summary = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for proxy_reporting_unit_summary: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::DataItemModelReportedMeasureCode>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_unit_summary<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::ReportingUnitSummaryModel>,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_unit_summary = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_unit_summary: {}",
                            e
                        )
                    });
                self
            }
            pub fn suppressions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<super::SuppressionModel>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.suppressions = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for suppressions: {}", e
                        )
                    });
                self
            }
            pub fn upper_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.upper_value = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for upper_value: {}", e)
                    });
                self
            }
            pub fn value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.value = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for value: {}", e)
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<DataItemModel> for super::DataItemModel {
            type Error = super::error::ConversionError;
            fn try_from(
                value: DataItemModel,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    caveats: value.caveats?,
                    data_set_id: value.data_set_id?,
                    group_number: value.group_number?,
                    lower_value: value.lower_value?,
                    measure_code: value.measure_code?,
                    peer_group_summary: value.peer_group_summary?,
                    proxy_reporting_unit_summary: value.proxy_reporting_unit_summary?,
                    reported_measure_code: value.reported_measure_code?,
                    reporting_unit_summary: value.reporting_unit_summary?,
                    suppressions: value.suppressions?,
                    upper_value: value.upper_value?,
                    value: value.value?,
                })
            }
        }
        impl ::std::convert::From<super::DataItemModel> for DataItemModel {
            fn from(value: super::DataItemModel) -> Self {
                Self {
                    caveats: Ok(value.caveats),
                    data_set_id: Ok(value.data_set_id),
                    group_number: Ok(value.group_number),
                    lower_value: Ok(value.lower_value),
                    measure_code: Ok(value.measure_code),
                    peer_group_summary: Ok(value.peer_group_summary),
                    proxy_reporting_unit_summary: Ok(value.proxy_reporting_unit_summary),
                    reported_measure_code: Ok(value.reported_measure_code),
                    reporting_unit_summary: Ok(value.reporting_unit_summary),
                    suppressions: Ok(value.suppressions),
                    upper_value: Ok(value.upper_value),
                    value: Ok(value.value),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct DataSetModel {
            caveats: ::std::result::Result<
                ::std::vec::Vec<super::CaveatModel>,
                ::std::string::String,
            >,
            data_set_id: ::std::result::Result<i32, ::std::string::String>,
            data_set_name: ::std::result::Result<
                super::DataSetModelDataSetName,
                ::std::string::String,
            >,
            meta_tags: ::std::result::Result<
                ::std::vec::Vec<super::MetaTagModel>,
                ::std::string::String,
            >,
            reported_measure_summary: ::std::result::Result<
                super::ReportedMeasureSummaryModel,
                ::std::string::String,
            >,
            reporting_end_date: ::std::result::Result<
                ::chrono::naive::NaiveDate,
                ::std::string::String,
            >,
            reporting_start_date: ::std::result::Result<
                ::chrono::naive::NaiveDate,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for DataSetModel {
            fn default() -> Self {
                Self {
                    caveats: Err("no value supplied for caveats".to_string()),
                    data_set_id: Err("no value supplied for data_set_id".to_string()),
                    data_set_name: Err(
                        "no value supplied for data_set_name".to_string(),
                    ),
                    meta_tags: Err("no value supplied for meta_tags".to_string()),
                    reported_measure_summary: Err(
                        "no value supplied for reported_measure_summary".to_string(),
                    ),
                    reporting_end_date: Err(
                        "no value supplied for reporting_end_date".to_string(),
                    ),
                    reporting_start_date: Err(
                        "no value supplied for reporting_start_date".to_string(),
                    ),
                }
            }
        }
        impl DataSetModel {
            pub fn caveats<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::CaveatModel>>,
                T::Error: ::std::fmt::Display,
            {
                self.caveats = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for caveats: {}", e)
                    });
                self
            }
            pub fn data_set_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i32>,
                T::Error: ::std::fmt::Display,
            {
                self.data_set_id = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for data_set_id: {}", e)
                    });
                self
            }
            pub fn data_set_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::DataSetModelDataSetName>,
                T::Error: ::std::fmt::Display,
            {
                self.data_set_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for data_set_name: {}", e
                        )
                    });
                self
            }
            pub fn meta_tags<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::MetaTagModel>>,
                T::Error: ::std::fmt::Display,
            {
                self.meta_tags = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for meta_tags: {}", e)
                    });
                self
            }
            pub fn reported_measure_summary<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::ReportedMeasureSummaryModel>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_summary = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_summary: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_end_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::naive::NaiveDate>,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_end_date = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_end_date: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_start_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::naive::NaiveDate>,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_start_date = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_start_date: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<DataSetModel> for super::DataSetModel {
            type Error = super::error::ConversionError;
            fn try_from(
                value: DataSetModel,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    caveats: value.caveats?,
                    data_set_id: value.data_set_id?,
                    data_set_name: value.data_set_name?,
                    meta_tags: value.meta_tags?,
                    reported_measure_summary: value.reported_measure_summary?,
                    reporting_end_date: value.reporting_end_date?,
                    reporting_start_date: value.reporting_start_date?,
                })
            }
        }
        impl ::std::convert::From<super::DataSetModel> for DataSetModel {
            fn from(value: super::DataSetModel) -> Self {
                Self {
                    caveats: Ok(value.caveats),
                    data_set_id: Ok(value.data_set_id),
                    data_set_name: Ok(value.data_set_name),
                    meta_tags: Ok(value.meta_tags),
                    reported_measure_summary: Ok(value.reported_measure_summary),
                    reporting_end_date: Ok(value.reporting_end_date),
                    reporting_start_date: Ok(value.reporting_start_date),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct DatasheetConfigurationSummaryModel {
            datasheet_code: ::std::result::Result<
                super::DatasheetConfigurationSummaryModelDatasheetCode,
                ::std::string::String,
            >,
            datasheet_description: ::std::result::Result<
                super::DatasheetConfigurationSummaryModelDatasheetDescription,
                ::std::string::String,
            >,
            datasheet_type: ::std::result::Result<
                super::DatasheetConfigurationSummaryModelDatasheetType,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for DatasheetConfigurationSummaryModel {
            fn default() -> Self {
                Self {
                    datasheet_code: Err(
                        "no value supplied for datasheet_code".to_string(),
                    ),
                    datasheet_description: Err(
                        "no value supplied for datasheet_description".to_string(),
                    ),
                    datasheet_type: Err(
                        "no value supplied for datasheet_type".to_string(),
                    ),
                }
            }
        }
        impl DatasheetConfigurationSummaryModel {
            pub fn datasheet_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::DatasheetConfigurationSummaryModelDatasheetCode,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.datasheet_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for datasheet_code: {}", e
                        )
                    });
                self
            }
            pub fn datasheet_description<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::DatasheetConfigurationSummaryModelDatasheetDescription,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.datasheet_description = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for datasheet_description: {}",
                            e
                        )
                    });
                self
            }
            pub fn datasheet_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::DatasheetConfigurationSummaryModelDatasheetType,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.datasheet_type = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for datasheet_type: {}", e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<DatasheetConfigurationSummaryModel>
        for super::DatasheetConfigurationSummaryModel {
            type Error = super::error::ConversionError;
            fn try_from(
                value: DatasheetConfigurationSummaryModel,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    datasheet_code: value.datasheet_code?,
                    datasheet_description: value.datasheet_description?,
                    datasheet_type: value.datasheet_type?,
                })
            }
        }
        impl ::std::convert::From<super::DatasheetConfigurationSummaryModel>
        for DatasheetConfigurationSummaryModel {
            fn from(value: super::DatasheetConfigurationSummaryModel) -> Self {
                Self {
                    datasheet_code: Ok(value.datasheet_code),
                    datasheet_description: Ok(value.datasheet_description),
                    datasheet_type: Ok(value.datasheet_type),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct File {}
        impl ::std::default::Default for File {
            fn default() -> Self {
                Self {}
            }
        }
        impl File {}
        impl ::std::convert::TryFrom<File> for super::File {
            type Error = super::error::ConversionError;
            fn try_from(
                _value: File,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {})
            }
        }
        impl ::std::convert::From<super::File> for File {
            fn from(_value: super::File) -> Self {
                Self {}
            }
        }
        #[derive(Clone, Debug)]
        pub struct FormattedDataExtractModel {
            caveat: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            caveat_codes: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            caveat_footnotes: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            data_period: ::std::result::Result<
                super::FormattedDataExtractModelDataPeriod,
                ::std::string::String,
            >,
            data_period_type: ::std::result::Result<
                super::FormattedDataExtractModelDataPeriodType,
                ::std::string::String,
            >,
            data_set_caveat: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            data_set_caveat_codes: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            data_set_caveat_footnotes: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            data_set_id: ::std::result::Result<i32, ::std::string::String>,
            data_set_meta_tag_codes: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            data_set_meta_tags: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            data_set_meta_tags_with_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            formatted_peer_value: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            formatted_value: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            group_number: ::std::result::Result<
                ::std::option::Option<i32>,
                ::std::string::String,
            >,
            mapped_local_hospital_network: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            mapped_primary_health_network: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            mapped_state: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            measure_category_code: ::std::result::Result<
                super::FormattedDataExtractModelMeasureCategoryCode,
                ::std::string::String,
            >,
            measure_category_name: ::std::result::Result<
                super::FormattedDataExtractModelMeasureCategoryName,
                ::std::string::String,
            >,
            measure_code: ::std::result::Result<
                super::FormattedDataExtractModelMeasureCode,
                ::std::string::String,
            >,
            measure_meta_tag_codes: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            measure_meta_tags: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            measure_meta_tags_with_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            measure_name: ::std::result::Result<
                super::FormattedDataExtractModelMeasureName,
                ::std::string::String,
            >,
            peer_group_code: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            peer_group_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            private: ::std::result::Result<bool, ::std::string::String>,
            proxy_reporting_unit_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            raw_lower_value: ::std::result::Result<
                ::std::option::Option<f64>,
                ::std::string::String,
            >,
            raw_peer_value: ::std::result::Result<
                ::std::option::Option<f64>,
                ::std::string::String,
            >,
            raw_upper_value: ::std::result::Result<
                ::std::option::Option<f64>,
                ::std::string::String,
            >,
            raw_value: ::std::result::Result<
                ::std::option::Option<f64>,
                ::std::string::String,
            >,
            reported_measure_category_code: ::std::result::Result<
                super::FormattedDataExtractModelReportedMeasureCategoryCode,
                ::std::string::String,
            >,
            reported_measure_category_name: ::std::result::Result<
                super::FormattedDataExtractModelReportedMeasureCategoryName,
                ::std::string::String,
            >,
            reported_measure_category_three_code: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reported_measure_category_three_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reported_measure_category_three_type_code: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reported_measure_category_three_type_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reported_measure_category_two_code: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reported_measure_category_two_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reported_measure_category_two_type_code: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reported_measure_category_two_type_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reported_measure_category_type_code: ::std::result::Result<
                super::FormattedDataExtractModelReportedMeasureCategoryTypeCode,
                ::std::string::String,
            >,
            reported_measure_category_type_name: ::std::result::Result<
                super::FormattedDataExtractModelReportedMeasureCategoryTypeName,
                ::std::string::String,
            >,
            reported_measure_code: ::std::result::Result<
                super::FormattedDataExtractModelReportedMeasureCode,
                ::std::string::String,
            >,
            reported_measure_meta_tag_codes: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reported_measure_meta_tags: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reported_measure_meta_tags_with_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reported_measure_name: ::std::result::Result<
                super::FormattedDataExtractModelReportedMeasureName,
                ::std::string::String,
            >,
            reporting_end_date: ::std::result::Result<
                ::chrono::naive::NaiveDate,
                ::std::string::String,
            >,
            reporting_start_date: ::std::result::Result<
                ::chrono::naive::NaiveDate,
                ::std::string::String,
            >,
            reporting_unit_code: ::std::result::Result<
                super::FormattedDataExtractModelReportingUnitCode,
                ::std::string::String,
            >,
            reporting_unit_meta_tag_codes: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reporting_unit_meta_tags: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reporting_unit_meta_tags_with_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            reporting_unit_name: ::std::result::Result<
                super::FormattedDataExtractModelReportingUnitName,
                ::std::string::String,
            >,
            reporting_unit_type_code: ::std::result::Result<
                super::FormattedDataExtractModelReportingUnitTypeCode,
                ::std::string::String,
            >,
            reporting_unit_type_name: ::std::result::Result<
                super::FormattedDataExtractModelReportingUnitTypeName,
                ::std::string::String,
            >,
            suppression: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            suppression_codes: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for FormattedDataExtractModel {
            fn default() -> Self {
                Self {
                    caveat: Ok(Default::default()),
                    caveat_codes: Ok(Default::default()),
                    caveat_footnotes: Ok(Default::default()),
                    data_period: Err("no value supplied for data_period".to_string()),
                    data_period_type: Err(
                        "no value supplied for data_period_type".to_string(),
                    ),
                    data_set_caveat: Ok(Default::default()),
                    data_set_caveat_codes: Ok(Default::default()),
                    data_set_caveat_footnotes: Ok(Default::default()),
                    data_set_id: Err("no value supplied for data_set_id".to_string()),
                    data_set_meta_tag_codes: Ok(Default::default()),
                    data_set_meta_tags: Ok(Default::default()),
                    data_set_meta_tags_with_type: Ok(Default::default()),
                    formatted_peer_value: Ok(Default::default()),
                    formatted_value: Ok(Default::default()),
                    group_number: Ok(Default::default()),
                    mapped_local_hospital_network: Ok(Default::default()),
                    mapped_primary_health_network: Ok(Default::default()),
                    mapped_state: Ok(Default::default()),
                    measure_category_code: Err(
                        "no value supplied for measure_category_code".to_string(),
                    ),
                    measure_category_name: Err(
                        "no value supplied for measure_category_name".to_string(),
                    ),
                    measure_code: Err("no value supplied for measure_code".to_string()),
                    measure_meta_tag_codes: Ok(Default::default()),
                    measure_meta_tags: Ok(Default::default()),
                    measure_meta_tags_with_type: Ok(Default::default()),
                    measure_name: Err("no value supplied for measure_name".to_string()),
                    peer_group_code: Ok(Default::default()),
                    peer_group_name: Ok(Default::default()),
                    private: Err("no value supplied for private".to_string()),
                    proxy_reporting_unit_name: Ok(Default::default()),
                    raw_lower_value: Ok(Default::default()),
                    raw_peer_value: Ok(Default::default()),
                    raw_upper_value: Ok(Default::default()),
                    raw_value: Ok(Default::default()),
                    reported_measure_category_code: Err(
                        "no value supplied for reported_measure_category_code"
                            .to_string(),
                    ),
                    reported_measure_category_name: Err(
                        "no value supplied for reported_measure_category_name"
                            .to_string(),
                    ),
                    reported_measure_category_three_code: Ok(Default::default()),
                    reported_measure_category_three_name: Ok(Default::default()),
                    reported_measure_category_three_type_code: Ok(Default::default()),
                    reported_measure_category_three_type_name: Ok(Default::default()),
                    reported_measure_category_two_code: Ok(Default::default()),
                    reported_measure_category_two_name: Ok(Default::default()),
                    reported_measure_category_two_type_code: Ok(Default::default()),
                    reported_measure_category_two_type_name: Ok(Default::default()),
                    reported_measure_category_type_code: Err(
                        "no value supplied for reported_measure_category_type_code"
                            .to_string(),
                    ),
                    reported_measure_category_type_name: Err(
                        "no value supplied for reported_measure_category_type_name"
                            .to_string(),
                    ),
                    reported_measure_code: Err(
                        "no value supplied for reported_measure_code".to_string(),
                    ),
                    reported_measure_meta_tag_codes: Ok(Default::default()),
                    reported_measure_meta_tags: Ok(Default::default()),
                    reported_measure_meta_tags_with_type: Ok(Default::default()),
                    reported_measure_name: Err(
                        "no value supplied for reported_measure_name".to_string(),
                    ),
                    reporting_end_date: Err(
                        "no value supplied for reporting_end_date".to_string(),
                    ),
                    reporting_start_date: Err(
                        "no value supplied for reporting_start_date".to_string(),
                    ),
                    reporting_unit_code: Err(
                        "no value supplied for reporting_unit_code".to_string(),
                    ),
                    reporting_unit_meta_tag_codes: Ok(Default::default()),
                    reporting_unit_meta_tags: Ok(Default::default()),
                    reporting_unit_meta_tags_with_type: Ok(Default::default()),
                    reporting_unit_name: Err(
                        "no value supplied for reporting_unit_name".to_string(),
                    ),
                    reporting_unit_type_code: Err(
                        "no value supplied for reporting_unit_type_code".to_string(),
                    ),
                    reporting_unit_type_name: Err(
                        "no value supplied for reporting_unit_type_name".to_string(),
                    ),
                    suppression: Ok(Default::default()),
                    suppression_codes: Ok(Default::default()),
                }
            }
        }
        impl FormattedDataExtractModel {
            pub fn caveat<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.caveat = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for caveat: {}", e)
                    });
                self
            }
            pub fn caveat_codes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.caveat_codes = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for caveat_codes: {}", e
                        )
                    });
                self
            }
            pub fn caveat_footnotes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.caveat_footnotes = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for caveat_footnotes: {}", e
                        )
                    });
                self
            }
            pub fn data_period<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::FormattedDataExtractModelDataPeriod>,
                T::Error: ::std::fmt::Display,
            {
                self.data_period = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for data_period: {}", e)
                    });
                self
            }
            pub fn data_period_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::FormattedDataExtractModelDataPeriodType,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.data_period_type = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for data_period_type: {}", e
                        )
                    });
                self
            }
            pub fn data_set_caveat<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.data_set_caveat = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for data_set_caveat: {}", e
                        )
                    });
                self
            }
            pub fn data_set_caveat_codes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.data_set_caveat_codes = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for data_set_caveat_codes: {}",
                            e
                        )
                    });
                self
            }
            pub fn data_set_caveat_footnotes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.data_set_caveat_footnotes = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for data_set_caveat_footnotes: {}",
                            e
                        )
                    });
                self
            }
            pub fn data_set_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i32>,
                T::Error: ::std::fmt::Display,
            {
                self.data_set_id = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for data_set_id: {}", e)
                    });
                self
            }
            pub fn data_set_meta_tag_codes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.data_set_meta_tag_codes = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for data_set_meta_tag_codes: {}",
                            e
                        )
                    });
                self
            }
            pub fn data_set_meta_tags<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.data_set_meta_tags = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for data_set_meta_tags: {}",
                            e
                        )
                    });
                self
            }
            pub fn data_set_meta_tags_with_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.data_set_meta_tags_with_type = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for data_set_meta_tags_with_type: {}",
                            e
                        )
                    });
                self
            }
            pub fn formatted_peer_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.formatted_peer_value = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for formatted_peer_value: {}",
                            e
                        )
                    });
                self
            }
            pub fn formatted_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.formatted_value = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for formatted_value: {}", e
                        )
                    });
                self
            }
            pub fn group_number<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.group_number = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for group_number: {}", e
                        )
                    });
                self
            }
            pub fn mapped_local_hospital_network<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.mapped_local_hospital_network = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for mapped_local_hospital_network: {}",
                            e
                        )
                    });
                self
            }
            pub fn mapped_primary_health_network<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.mapped_primary_health_network = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for mapped_primary_health_network: {}",
                            e
                        )
                    });
                self
            }
            pub fn mapped_state<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.mapped_state = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for mapped_state: {}", e
                        )
                    });
                self
            }
            pub fn measure_category_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::FormattedDataExtractModelMeasureCategoryCode,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.measure_category_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for measure_category_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn measure_category_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::FormattedDataExtractModelMeasureCategoryName,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.measure_category_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for measure_category_name: {}",
                            e
                        )
                    });
                self
            }
            pub fn measure_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::FormattedDataExtractModelMeasureCode>,
                T::Error: ::std::fmt::Display,
            {
                self.measure_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for measure_code: {}", e
                        )
                    });
                self
            }
            pub fn measure_meta_tag_codes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.measure_meta_tag_codes = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for measure_meta_tag_codes: {}",
                            e
                        )
                    });
                self
            }
            pub fn measure_meta_tags<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.measure_meta_tags = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for measure_meta_tags: {}",
                            e
                        )
                    });
                self
            }
            pub fn measure_meta_tags_with_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.measure_meta_tags_with_type = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for measure_meta_tags_with_type: {}",
                            e
                        )
                    });
                self
            }
            pub fn measure_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::FormattedDataExtractModelMeasureName>,
                T::Error: ::std::fmt::Display,
            {
                self.measure_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for measure_name: {}", e
                        )
                    });
                self
            }
            pub fn peer_group_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.peer_group_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for peer_group_code: {}", e
                        )
                    });
                self
            }
            pub fn peer_group_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.peer_group_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for peer_group_name: {}", e
                        )
                    });
                self
            }
            pub fn private<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.private = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for private: {}", e)
                    });
                self
            }
            pub fn proxy_reporting_unit_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.proxy_reporting_unit_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for proxy_reporting_unit_name: {}",
                            e
                        )
                    });
                self
            }
            pub fn raw_lower_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.raw_lower_value = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for raw_lower_value: {}", e
                        )
                    });
                self
            }
            pub fn raw_peer_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.raw_peer_value = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for raw_peer_value: {}", e
                        )
                    });
                self
            }
            pub fn raw_upper_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.raw_upper_value = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for raw_upper_value: {}", e
                        )
                    });
                self
            }
            pub fn raw_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.raw_value = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for raw_value: {}", e)
                    });
                self
            }
            pub fn reported_measure_category_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::FormattedDataExtractModelReportedMeasureCategoryCode,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::FormattedDataExtractModelReportedMeasureCategoryName,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_name: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_three_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_three_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_three_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_three_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_three_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_three_name: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_three_type_code<T>(
                mut self,
                value: T,
            ) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_three_type_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_three_type_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_three_type_name<T>(
                mut self,
                value: T,
            ) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_three_type_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_three_type_name: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_two_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_two_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_two_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_two_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_two_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_two_name: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_two_type_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_two_type_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_two_type_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_two_type_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_two_type_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_two_type_name: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_type_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::FormattedDataExtractModelReportedMeasureCategoryTypeCode,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_type_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_type_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_type_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::FormattedDataExtractModelReportedMeasureCategoryTypeName,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_type_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_type_name: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::FormattedDataExtractModelReportedMeasureCode,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_meta_tag_codes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_meta_tag_codes = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_meta_tag_codes: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_meta_tags<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_meta_tags = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_meta_tags: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_meta_tags_with_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_meta_tags_with_type = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_meta_tags_with_type: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::FormattedDataExtractModelReportedMeasureName,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_name: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_end_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::naive::NaiveDate>,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_end_date = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_end_date: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_start_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::naive::NaiveDate>,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_start_date = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_start_date: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_unit_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::FormattedDataExtractModelReportingUnitCode,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_unit_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_unit_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_unit_meta_tag_codes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_unit_meta_tag_codes = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_unit_meta_tag_codes: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_unit_meta_tags<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_unit_meta_tags = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_unit_meta_tags: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_unit_meta_tags_with_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_unit_meta_tags_with_type = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_unit_meta_tags_with_type: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_unit_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::FormattedDataExtractModelReportingUnitName,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_unit_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_unit_name: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_unit_type_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::FormattedDataExtractModelReportingUnitTypeCode,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_unit_type_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_unit_type_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_unit_type_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::FormattedDataExtractModelReportingUnitTypeName,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_unit_type_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_unit_type_name: {}",
                            e
                        )
                    });
                self
            }
            pub fn suppression<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.suppression = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for suppression: {}", e)
                    });
                self
            }
            pub fn suppression_codes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.suppression_codes = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for suppression_codes: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<FormattedDataExtractModel>
        for super::FormattedDataExtractModel {
            type Error = super::error::ConversionError;
            fn try_from(
                value: FormattedDataExtractModel,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    caveat: value.caveat?,
                    caveat_codes: value.caveat_codes?,
                    caveat_footnotes: value.caveat_footnotes?,
                    data_period: value.data_period?,
                    data_period_type: value.data_period_type?,
                    data_set_caveat: value.data_set_caveat?,
                    data_set_caveat_codes: value.data_set_caveat_codes?,
                    data_set_caveat_footnotes: value.data_set_caveat_footnotes?,
                    data_set_id: value.data_set_id?,
                    data_set_meta_tag_codes: value.data_set_meta_tag_codes?,
                    data_set_meta_tags: value.data_set_meta_tags?,
                    data_set_meta_tags_with_type: value.data_set_meta_tags_with_type?,
                    formatted_peer_value: value.formatted_peer_value?,
                    formatted_value: value.formatted_value?,
                    group_number: value.group_number?,
                    mapped_local_hospital_network: value.mapped_local_hospital_network?,
                    mapped_primary_health_network: value.mapped_primary_health_network?,
                    mapped_state: value.mapped_state?,
                    measure_category_code: value.measure_category_code?,
                    measure_category_name: value.measure_category_name?,
                    measure_code: value.measure_code?,
                    measure_meta_tag_codes: value.measure_meta_tag_codes?,
                    measure_meta_tags: value.measure_meta_tags?,
                    measure_meta_tags_with_type: value.measure_meta_tags_with_type?,
                    measure_name: value.measure_name?,
                    peer_group_code: value.peer_group_code?,
                    peer_group_name: value.peer_group_name?,
                    private: value.private?,
                    proxy_reporting_unit_name: value.proxy_reporting_unit_name?,
                    raw_lower_value: value.raw_lower_value?,
                    raw_peer_value: value.raw_peer_value?,
                    raw_upper_value: value.raw_upper_value?,
                    raw_value: value.raw_value?,
                    reported_measure_category_code: value
                        .reported_measure_category_code?,
                    reported_measure_category_name: value
                        .reported_measure_category_name?,
                    reported_measure_category_three_code: value
                        .reported_measure_category_three_code?,
                    reported_measure_category_three_name: value
                        .reported_measure_category_three_name?,
                    reported_measure_category_three_type_code: value
                        .reported_measure_category_three_type_code?,
                    reported_measure_category_three_type_name: value
                        .reported_measure_category_three_type_name?,
                    reported_measure_category_two_code: value
                        .reported_measure_category_two_code?,
                    reported_measure_category_two_name: value
                        .reported_measure_category_two_name?,
                    reported_measure_category_two_type_code: value
                        .reported_measure_category_two_type_code?,
                    reported_measure_category_two_type_name: value
                        .reported_measure_category_two_type_name?,
                    reported_measure_category_type_code: value
                        .reported_measure_category_type_code?,
                    reported_measure_category_type_name: value
                        .reported_measure_category_type_name?,
                    reported_measure_code: value.reported_measure_code?,
                    reported_measure_meta_tag_codes: value
                        .reported_measure_meta_tag_codes?,
                    reported_measure_meta_tags: value.reported_measure_meta_tags?,
                    reported_measure_meta_tags_with_type: value
                        .reported_measure_meta_tags_with_type?,
                    reported_measure_name: value.reported_measure_name?,
                    reporting_end_date: value.reporting_end_date?,
                    reporting_start_date: value.reporting_start_date?,
                    reporting_unit_code: value.reporting_unit_code?,
                    reporting_unit_meta_tag_codes: value.reporting_unit_meta_tag_codes?,
                    reporting_unit_meta_tags: value.reporting_unit_meta_tags?,
                    reporting_unit_meta_tags_with_type: value
                        .reporting_unit_meta_tags_with_type?,
                    reporting_unit_name: value.reporting_unit_name?,
                    reporting_unit_type_code: value.reporting_unit_type_code?,
                    reporting_unit_type_name: value.reporting_unit_type_name?,
                    suppression: value.suppression?,
                    suppression_codes: value.suppression_codes?,
                })
            }
        }
        impl ::std::convert::From<super::FormattedDataExtractModel>
        for FormattedDataExtractModel {
            fn from(value: super::FormattedDataExtractModel) -> Self {
                Self {
                    caveat: Ok(value.caveat),
                    caveat_codes: Ok(value.caveat_codes),
                    caveat_footnotes: Ok(value.caveat_footnotes),
                    data_period: Ok(value.data_period),
                    data_period_type: Ok(value.data_period_type),
                    data_set_caveat: Ok(value.data_set_caveat),
                    data_set_caveat_codes: Ok(value.data_set_caveat_codes),
                    data_set_caveat_footnotes: Ok(value.data_set_caveat_footnotes),
                    data_set_id: Ok(value.data_set_id),
                    data_set_meta_tag_codes: Ok(value.data_set_meta_tag_codes),
                    data_set_meta_tags: Ok(value.data_set_meta_tags),
                    data_set_meta_tags_with_type: Ok(value.data_set_meta_tags_with_type),
                    formatted_peer_value: Ok(value.formatted_peer_value),
                    formatted_value: Ok(value.formatted_value),
                    group_number: Ok(value.group_number),
                    mapped_local_hospital_network: Ok(
                        value.mapped_local_hospital_network,
                    ),
                    mapped_primary_health_network: Ok(
                        value.mapped_primary_health_network,
                    ),
                    mapped_state: Ok(value.mapped_state),
                    measure_category_code: Ok(value.measure_category_code),
                    measure_category_name: Ok(value.measure_category_name),
                    measure_code: Ok(value.measure_code),
                    measure_meta_tag_codes: Ok(value.measure_meta_tag_codes),
                    measure_meta_tags: Ok(value.measure_meta_tags),
                    measure_meta_tags_with_type: Ok(value.measure_meta_tags_with_type),
                    measure_name: Ok(value.measure_name),
                    peer_group_code: Ok(value.peer_group_code),
                    peer_group_name: Ok(value.peer_group_name),
                    private: Ok(value.private),
                    proxy_reporting_unit_name: Ok(value.proxy_reporting_unit_name),
                    raw_lower_value: Ok(value.raw_lower_value),
                    raw_peer_value: Ok(value.raw_peer_value),
                    raw_upper_value: Ok(value.raw_upper_value),
                    raw_value: Ok(value.raw_value),
                    reported_measure_category_code: Ok(
                        value.reported_measure_category_code,
                    ),
                    reported_measure_category_name: Ok(
                        value.reported_measure_category_name,
                    ),
                    reported_measure_category_three_code: Ok(
                        value.reported_measure_category_three_code,
                    ),
                    reported_measure_category_three_name: Ok(
                        value.reported_measure_category_three_name,
                    ),
                    reported_measure_category_three_type_code: Ok(
                        value.reported_measure_category_three_type_code,
                    ),
                    reported_measure_category_three_type_name: Ok(
                        value.reported_measure_category_three_type_name,
                    ),
                    reported_measure_category_two_code: Ok(
                        value.reported_measure_category_two_code,
                    ),
                    reported_measure_category_two_name: Ok(
                        value.reported_measure_category_two_name,
                    ),
                    reported_measure_category_two_type_code: Ok(
                        value.reported_measure_category_two_type_code,
                    ),
                    reported_measure_category_two_type_name: Ok(
                        value.reported_measure_category_two_type_name,
                    ),
                    reported_measure_category_type_code: Ok(
                        value.reported_measure_category_type_code,
                    ),
                    reported_measure_category_type_name: Ok(
                        value.reported_measure_category_type_name,
                    ),
                    reported_measure_code: Ok(value.reported_measure_code),
                    reported_measure_meta_tag_codes: Ok(
                        value.reported_measure_meta_tag_codes,
                    ),
                    reported_measure_meta_tags: Ok(value.reported_measure_meta_tags),
                    reported_measure_meta_tags_with_type: Ok(
                        value.reported_measure_meta_tags_with_type,
                    ),
                    reported_measure_name: Ok(value.reported_measure_name),
                    reporting_end_date: Ok(value.reporting_end_date),
                    reporting_start_date: Ok(value.reporting_start_date),
                    reporting_unit_code: Ok(value.reporting_unit_code),
                    reporting_unit_meta_tag_codes: Ok(
                        value.reporting_unit_meta_tag_codes,
                    ),
                    reporting_unit_meta_tags: Ok(value.reporting_unit_meta_tags),
                    reporting_unit_meta_tags_with_type: Ok(
                        value.reporting_unit_meta_tags_with_type,
                    ),
                    reporting_unit_name: Ok(value.reporting_unit_name),
                    reporting_unit_type_code: Ok(value.reporting_unit_type_code),
                    reporting_unit_type_name: Ok(value.reporting_unit_type_name),
                    suppression: Ok(value.suppression),
                    suppression_codes: Ok(value.suppression_codes),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetCaveatsByCaveatCodeResponse {
            result: ::std::result::Result<super::CaveatModel, ::std::string::String>,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for GetCaveatsByCaveatCodeResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetCaveatsByCaveatCodeResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::CaveatModel>,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<GetCaveatsByCaveatCodeResponse>
        for super::GetCaveatsByCaveatCodeResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetCaveatsByCaveatCodeResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<super::GetCaveatsByCaveatCodeResponse>
        for GetCaveatsByCaveatCodeResponse {
            fn from(value: super::GetCaveatsByCaveatCodeResponse) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetCaveatsResponse {
            result: ::std::result::Result<
                ::std::vec::Vec<super::CaveatModel>,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for GetCaveatsResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetCaveatsResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::CaveatModel>>,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<GetCaveatsResponse> for super::GetCaveatsResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetCaveatsResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<super::GetCaveatsResponse> for GetCaveatsResponse {
            fn from(value: super::GetCaveatsResponse) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetDatasetsByDatasetIdDataItemsResponse {
            result: ::std::result::Result<super::DataItemModel, ::std::string::String>,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for GetDatasetsByDatasetIdDataItemsResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetDatasetsByDatasetIdDataItemsResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::DataItemModel>,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<GetDatasetsByDatasetIdDataItemsResponse>
        for super::GetDatasetsByDatasetIdDataItemsResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetDatasetsByDatasetIdDataItemsResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<super::GetDatasetsByDatasetIdDataItemsResponse>
        for GetDatasetsByDatasetIdDataItemsResponse {
            fn from(value: super::GetDatasetsByDatasetIdDataItemsResponse) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetDatasetsByDatasetIdResponse {
            result: ::std::result::Result<super::DataSetModel, ::std::string::String>,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for GetDatasetsByDatasetIdResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetDatasetsByDatasetIdResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::DataSetModel>,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<GetDatasetsByDatasetIdResponse>
        for super::GetDatasetsByDatasetIdResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetDatasetsByDatasetIdResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<super::GetDatasetsByDatasetIdResponse>
        for GetDatasetsByDatasetIdResponse {
            fn from(value: super::GetDatasetsByDatasetIdResponse) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetDatasetsResponse {
            result: ::std::result::Result<
                ::std::vec::Vec<super::DataSetModel>,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for GetDatasetsResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetDatasetsResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::DataSetModel>>,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<GetDatasetsResponse>
        for super::GetDatasetsResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetDatasetsResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<super::GetDatasetsResponse> for GetDatasetsResponse {
            fn from(value: super::GetDatasetsResponse) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetFlatDataExtractByMeasureCategoryCodeResponse {
            result: ::std::result::Result<
                super::PaginatedDataExtractModel,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default
        for GetFlatDataExtractByMeasureCategoryCodeResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetFlatDataExtractByMeasureCategoryCodeResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::PaginatedDataExtractModel>,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<GetFlatDataExtractByMeasureCategoryCodeResponse>
        for super::GetFlatDataExtractByMeasureCategoryCodeResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetFlatDataExtractByMeasureCategoryCodeResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<super::GetFlatDataExtractByMeasureCategoryCodeResponse>
        for GetFlatDataExtractByMeasureCategoryCodeResponse {
            fn from(
                value: super::GetFlatDataExtractByMeasureCategoryCodeResponse,
            ) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetFlatFormattedDataExtractByMeasureCategoryCodeResponse {
            result: ::std::result::Result<
                super::PaginatedFormattedDataExtractModel,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default
        for GetFlatFormattedDataExtractByMeasureCategoryCodeResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetFlatFormattedDataExtractByMeasureCategoryCodeResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::PaginatedFormattedDataExtractModel>,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<
            GetFlatFormattedDataExtractByMeasureCategoryCodeResponse,
        > for super::GetFlatFormattedDataExtractByMeasureCategoryCodeResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetFlatFormattedDataExtractByMeasureCategoryCodeResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<
            super::GetFlatFormattedDataExtractByMeasureCategoryCodeResponse,
        > for GetFlatFormattedDataExtractByMeasureCategoryCodeResponse {
            fn from(
                value: super::GetFlatFormattedDataExtractByMeasureCategoryCodeResponse,
            ) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetMeasureCategoriesByMeasureCategoryCodeMeasuresResponse {
            result: ::std::result::Result<
                ::std::vec::Vec<super::MeasureModel>,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default
        for GetMeasureCategoriesByMeasureCategoryCodeMeasuresResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetMeasureCategoriesByMeasureCategoryCodeMeasuresResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::MeasureModel>>,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<
            GetMeasureCategoriesByMeasureCategoryCodeMeasuresResponse,
        > for super::GetMeasureCategoriesByMeasureCategoryCodeMeasuresResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetMeasureCategoriesByMeasureCategoryCodeMeasuresResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<
            super::GetMeasureCategoriesByMeasureCategoryCodeMeasuresResponse,
        > for GetMeasureCategoriesByMeasureCategoryCodeMeasuresResponse {
            fn from(
                value: super::GetMeasureCategoriesByMeasureCategoryCodeMeasuresResponse,
            ) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetMeasureCategoriesByMeasureCategoryCodeResponse {
            result: ::std::result::Result<
                super::MeasureCategoryModel,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default
        for GetMeasureCategoriesByMeasureCategoryCodeResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetMeasureCategoriesByMeasureCategoryCodeResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::MeasureCategoryModel>,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<GetMeasureCategoriesByMeasureCategoryCodeResponse>
        for super::GetMeasureCategoriesByMeasureCategoryCodeResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetMeasureCategoriesByMeasureCategoryCodeResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<
            super::GetMeasureCategoriesByMeasureCategoryCodeResponse,
        > for GetMeasureCategoriesByMeasureCategoryCodeResponse {
            fn from(
                value: super::GetMeasureCategoriesByMeasureCategoryCodeResponse,
            ) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetMeasureCategoriesResponse {
            result: ::std::result::Result<
                ::std::vec::Vec<super::MeasureCategoryModel>,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for GetMeasureCategoriesResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetMeasureCategoriesResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::MeasureCategoryModel>>,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<GetMeasureCategoriesResponse>
        for super::GetMeasureCategoriesResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetMeasureCategoriesResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<super::GetMeasureCategoriesResponse>
        for GetMeasureCategoriesResponse {
            fn from(value: super::GetMeasureCategoriesResponse) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetMeasureDownloadsMeasureDownloadCodesResponse {
            result: ::std::result::Result<
                ::std::collections::HashMap<
                    ::std::string::String,
                    ::std::vec::Vec<super::DatasheetConfigurationSummaryModel>,
                >,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default
        for GetMeasureDownloadsMeasureDownloadCodesResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetMeasureDownloadsMeasureDownloadCodesResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::collections::HashMap<
                        ::std::string::String,
                        ::std::vec::Vec<super::DatasheetConfigurationSummaryModel>,
                    >,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<GetMeasureDownloadsMeasureDownloadCodesResponse>
        for super::GetMeasureDownloadsMeasureDownloadCodesResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetMeasureDownloadsMeasureDownloadCodesResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<super::GetMeasureDownloadsMeasureDownloadCodesResponse>
        for GetMeasureDownloadsMeasureDownloadCodesResponse {
            fn from(
                value: super::GetMeasureDownloadsMeasureDownloadCodesResponse,
            ) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetMeasuresByMeasureCodeDataItemsResponse {
            result: ::std::result::Result<
                ::std::vec::Vec<super::DataItemModel>,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for GetMeasuresByMeasureCodeDataItemsResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetMeasuresByMeasureCodeDataItemsResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::DataItemModel>>,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<GetMeasuresByMeasureCodeDataItemsResponse>
        for super::GetMeasuresByMeasureCodeDataItemsResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetMeasuresByMeasureCodeDataItemsResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<super::GetMeasuresByMeasureCodeDataItemsResponse>
        for GetMeasuresByMeasureCodeDataItemsResponse {
            fn from(value: super::GetMeasuresByMeasureCodeDataItemsResponse) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetMeasuresByMeasureCodeReportingUnitsAvailableResponse {
            result: ::std::result::Result<
                ::std::vec::Vec<super::ReportingUnitSummaryModel>,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default
        for GetMeasuresByMeasureCodeReportingUnitsAvailableResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetMeasuresByMeasureCodeReportingUnitsAvailableResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::vec::Vec<super::ReportingUnitSummaryModel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<
            GetMeasuresByMeasureCodeReportingUnitsAvailableResponse,
        > for super::GetMeasuresByMeasureCodeReportingUnitsAvailableResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetMeasuresByMeasureCodeReportingUnitsAvailableResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<
            super::GetMeasuresByMeasureCodeReportingUnitsAvailableResponse,
        > for GetMeasuresByMeasureCodeReportingUnitsAvailableResponse {
            fn from(
                value: super::GetMeasuresByMeasureCodeReportingUnitsAvailableResponse,
            ) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetMeasuresByMeasureCodeResponse {
            result: ::std::result::Result<super::MeasureModel, ::std::string::String>,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for GetMeasuresByMeasureCodeResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetMeasuresByMeasureCodeResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::MeasureModel>,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<GetMeasuresByMeasureCodeResponse>
        for super::GetMeasuresByMeasureCodeResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetMeasuresByMeasureCodeResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<super::GetMeasuresByMeasureCodeResponse>
        for GetMeasuresByMeasureCodeResponse {
            fn from(value: super::GetMeasuresByMeasureCodeResponse) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetMeasuresResponse {
            result: ::std::result::Result<
                ::std::vec::Vec<super::MeasureModel>,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for GetMeasuresResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetMeasuresResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::MeasureModel>>,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<GetMeasuresResponse>
        for super::GetMeasuresResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetMeasuresResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<super::GetMeasuresResponse> for GetMeasuresResponse {
            fn from(value: super::GetMeasuresResponse) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetReportedMeasureCategoriesByReportedMeasureCategoryCodeReportedMeasuresResponse {
            result: ::std::result::Result<
                ::std::vec::Vec<super::ReportedMeasureModel>,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default
        for GetReportedMeasureCategoriesByReportedMeasureCategoryCodeReportedMeasuresResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetReportedMeasureCategoriesByReportedMeasureCategoryCodeReportedMeasuresResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::ReportedMeasureModel>>,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<
            GetReportedMeasureCategoriesByReportedMeasureCategoryCodeReportedMeasuresResponse,
        >
        for super::GetReportedMeasureCategoriesByReportedMeasureCategoryCodeReportedMeasuresResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetReportedMeasureCategoriesByReportedMeasureCategoryCodeReportedMeasuresResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<
            super::GetReportedMeasureCategoriesByReportedMeasureCategoryCodeReportedMeasuresResponse,
        >
        for GetReportedMeasureCategoriesByReportedMeasureCategoryCodeReportedMeasuresResponse {
            fn from(
                value: super::GetReportedMeasureCategoriesByReportedMeasureCategoryCodeReportedMeasuresResponse,
            ) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetReportedMeasureCategoriesByReportedMeasureCategoryCodeResponse {
            result: ::std::result::Result<
                ::std::vec::Vec<super::ReportedMeasureCategoryModel>,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default
        for GetReportedMeasureCategoriesByReportedMeasureCategoryCodeResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetReportedMeasureCategoriesByReportedMeasureCategoryCodeResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::vec::Vec<super::ReportedMeasureCategoryModel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<
            GetReportedMeasureCategoriesByReportedMeasureCategoryCodeResponse,
        > for super::GetReportedMeasureCategoriesByReportedMeasureCategoryCodeResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetReportedMeasureCategoriesByReportedMeasureCategoryCodeResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<
            super::GetReportedMeasureCategoriesByReportedMeasureCategoryCodeResponse,
        > for GetReportedMeasureCategoriesByReportedMeasureCategoryCodeResponse {
            fn from(
                value: super::GetReportedMeasureCategoriesByReportedMeasureCategoryCodeResponse,
            ) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetReportedMeasureCategoriesResponse {
            result: ::std::result::Result<
                ::std::vec::Vec<super::ReportedMeasureCategoryModel>,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for GetReportedMeasureCategoriesResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetReportedMeasureCategoriesResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::vec::Vec<super::ReportedMeasureCategoryModel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<GetReportedMeasureCategoriesResponse>
        for super::GetReportedMeasureCategoriesResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetReportedMeasureCategoriesResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<super::GetReportedMeasureCategoriesResponse>
        for GetReportedMeasureCategoriesResponse {
            fn from(value: super::GetReportedMeasureCategoriesResponse) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetReportedMeasuresByReportedMeasureCodeDataItemsResponse {
            result: ::std::result::Result<
                ::std::vec::Vec<super::DataItemModel>,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default
        for GetReportedMeasuresByReportedMeasureCodeDataItemsResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetReportedMeasuresByReportedMeasureCodeDataItemsResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::DataItemModel>>,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<
            GetReportedMeasuresByReportedMeasureCodeDataItemsResponse,
        > for super::GetReportedMeasuresByReportedMeasureCodeDataItemsResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetReportedMeasuresByReportedMeasureCodeDataItemsResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<
            super::GetReportedMeasuresByReportedMeasureCodeDataItemsResponse,
        > for GetReportedMeasuresByReportedMeasureCodeDataItemsResponse {
            fn from(
                value: super::GetReportedMeasuresByReportedMeasureCodeDataItemsResponse,
            ) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetReportedMeasuresByReportedMeasureCodeResponse {
            result: ::std::result::Result<
                super::ReportedMeasureModel,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default
        for GetReportedMeasuresByReportedMeasureCodeResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetReportedMeasuresByReportedMeasureCodeResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::ReportedMeasureModel>,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<GetReportedMeasuresByReportedMeasureCodeResponse>
        for super::GetReportedMeasuresByReportedMeasureCodeResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetReportedMeasuresByReportedMeasureCodeResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<
            super::GetReportedMeasuresByReportedMeasureCodeResponse,
        > for GetReportedMeasuresByReportedMeasureCodeResponse {
            fn from(
                value: super::GetReportedMeasuresByReportedMeasureCodeResponse,
            ) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetReportedMeasuresResponse {
            result: ::std::result::Result<
                ::std::vec::Vec<super::ReportedMeasureModel>,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for GetReportedMeasuresResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetReportedMeasuresResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::ReportedMeasureModel>>,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<GetReportedMeasuresResponse>
        for super::GetReportedMeasuresResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetReportedMeasuresResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<super::GetReportedMeasuresResponse>
        for GetReportedMeasuresResponse {
            fn from(value: super::GetReportedMeasuresResponse) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetReportingUnitTypesByReportingUnitTypeCodeBricksAvailableResponse {
            result: ::std::result::Result<
                ::std::collections::HashMap<
                    ::std::string::String,
                    ::std::vec::Vec<::std::string::String>,
                >,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default
        for GetReportingUnitTypesByReportingUnitTypeCodeBricksAvailableResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetReportingUnitTypesByReportingUnitTypeCodeBricksAvailableResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::collections::HashMap<
                        ::std::string::String,
                        ::std::vec::Vec<::std::string::String>,
                    >,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<
            GetReportingUnitTypesByReportingUnitTypeCodeBricksAvailableResponse,
        >
        for super::GetReportingUnitTypesByReportingUnitTypeCodeBricksAvailableResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetReportingUnitTypesByReportingUnitTypeCodeBricksAvailableResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<
            super::GetReportingUnitTypesByReportingUnitTypeCodeBricksAvailableResponse,
        > for GetReportingUnitTypesByReportingUnitTypeCodeBricksAvailableResponse {
            fn from(
                value: super::GetReportingUnitTypesByReportingUnitTypeCodeBricksAvailableResponse,
            ) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetReportingUnitTypesByReportingUnitTypeCodeResponse {
            result: ::std::result::Result<
                super::ReportingUnitTypeModel,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default
        for GetReportingUnitTypesByReportingUnitTypeCodeResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetReportingUnitTypesByReportingUnitTypeCodeResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::ReportingUnitTypeModel>,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<
            GetReportingUnitTypesByReportingUnitTypeCodeResponse,
        > for super::GetReportingUnitTypesByReportingUnitTypeCodeResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetReportingUnitTypesByReportingUnitTypeCodeResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<
            super::GetReportingUnitTypesByReportingUnitTypeCodeResponse,
        > for GetReportingUnitTypesByReportingUnitTypeCodeResponse {
            fn from(
                value: super::GetReportingUnitTypesByReportingUnitTypeCodeResponse,
            ) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetReportingUnitTypesResponse {
            result: ::std::result::Result<
                ::std::vec::Vec<super::ReportingUnitTypeModel>,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for GetReportingUnitTypesResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetReportingUnitTypesResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::vec::Vec<super::ReportingUnitTypeModel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<GetReportingUnitTypesResponse>
        for super::GetReportingUnitTypesResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetReportingUnitTypesResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<super::GetReportingUnitTypesResponse>
        for GetReportingUnitTypesResponse {
            fn from(value: super::GetReportingUnitTypesResponse) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetReportingUnitsByReportingUnitCodeBricksAvailableResponse {
            result: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default
        for GetReportingUnitsByReportingUnitCodeBricksAvailableResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetReportingUnitsByReportingUnitCodeBricksAvailableResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<
            GetReportingUnitsByReportingUnitCodeBricksAvailableResponse,
        > for super::GetReportingUnitsByReportingUnitCodeBricksAvailableResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetReportingUnitsByReportingUnitCodeBricksAvailableResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<
            super::GetReportingUnitsByReportingUnitCodeBricksAvailableResponse,
        > for GetReportingUnitsByReportingUnitCodeBricksAvailableResponse {
            fn from(
                value: super::GetReportingUnitsByReportingUnitCodeBricksAvailableResponse,
            ) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetReportingUnitsByReportingUnitCodeDataItemsResponse {
            result: ::std::result::Result<
                ::std::vec::Vec<super::DataItemModel>,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default
        for GetReportingUnitsByReportingUnitCodeDataItemsResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetReportingUnitsByReportingUnitCodeDataItemsResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::DataItemModel>>,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<
            GetReportingUnitsByReportingUnitCodeDataItemsResponse,
        > for super::GetReportingUnitsByReportingUnitCodeDataItemsResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetReportingUnitsByReportingUnitCodeDataItemsResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<
            super::GetReportingUnitsByReportingUnitCodeDataItemsResponse,
        > for GetReportingUnitsByReportingUnitCodeDataItemsResponse {
            fn from(
                value: super::GetReportingUnitsByReportingUnitCodeDataItemsResponse,
            ) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetReportingUnitsByReportingUnitCodeMeasuresAvailableResponse {
            result: ::std::result::Result<
                ::std::vec::Vec<super::MeasureSummaryModel>,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default
        for GetReportingUnitsByReportingUnitCodeMeasuresAvailableResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetReportingUnitsByReportingUnitCodeMeasuresAvailableResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::MeasureSummaryModel>>,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<
            GetReportingUnitsByReportingUnitCodeMeasuresAvailableResponse,
        > for super::GetReportingUnitsByReportingUnitCodeMeasuresAvailableResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetReportingUnitsByReportingUnitCodeMeasuresAvailableResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<
            super::GetReportingUnitsByReportingUnitCodeMeasuresAvailableResponse,
        > for GetReportingUnitsByReportingUnitCodeMeasuresAvailableResponse {
            fn from(
                value: super::GetReportingUnitsByReportingUnitCodeMeasuresAvailableResponse,
            ) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetReportingUnitsByReportingUnitCodeResponse {
            result: ::std::result::Result<
                super::ReportingUnitModel,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for GetReportingUnitsByReportingUnitCodeResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetReportingUnitsByReportingUnitCodeResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::ReportingUnitModel>,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<GetReportingUnitsByReportingUnitCodeResponse>
        for super::GetReportingUnitsByReportingUnitCodeResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetReportingUnitsByReportingUnitCodeResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<super::GetReportingUnitsByReportingUnitCodeResponse>
        for GetReportingUnitsByReportingUnitCodeResponse {
            fn from(value: super::GetReportingUnitsByReportingUnitCodeResponse) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetReportingUnitsDownloadsDatasheetCodesResponse {
            result: ::std::result::Result<
                ::std::collections::HashMap<
                    ::std::string::String,
                    ::std::vec::Vec<super::DatasheetConfigurationSummaryModel>,
                >,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default
        for GetReportingUnitsDownloadsDatasheetCodesResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetReportingUnitsDownloadsDatasheetCodesResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::collections::HashMap<
                        ::std::string::String,
                        ::std::vec::Vec<super::DatasheetConfigurationSummaryModel>,
                    >,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<GetReportingUnitsDownloadsDatasheetCodesResponse>
        for super::GetReportingUnitsDownloadsDatasheetCodesResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetReportingUnitsDownloadsDatasheetCodesResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<
            super::GetReportingUnitsDownloadsDatasheetCodesResponse,
        > for GetReportingUnitsDownloadsDatasheetCodesResponse {
            fn from(
                value: super::GetReportingUnitsDownloadsDatasheetCodesResponse,
            ) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetReportingUnitsResponse {
            result: ::std::result::Result<
                ::std::vec::Vec<super::ReportingUnitModel>,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for GetReportingUnitsResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetReportingUnitsResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::ReportingUnitModel>>,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<GetReportingUnitsResponse>
        for super::GetReportingUnitsResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetReportingUnitsResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<super::GetReportingUnitsResponse>
        for GetReportingUnitsResponse {
            fn from(value: super::GetReportingUnitsResponse) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetSimpleDownloadsDownloadCodesResponse {
            result: ::std::result::Result<
                ::std::collections::HashMap<
                    ::std::string::String,
                    ::std::vec::Vec<super::DatasheetConfigurationSummaryModel>,
                >,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for GetSimpleDownloadsDownloadCodesResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetSimpleDownloadsDownloadCodesResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::collections::HashMap<
                        ::std::string::String,
                        ::std::vec::Vec<super::DatasheetConfigurationSummaryModel>,
                    >,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<GetSimpleDownloadsDownloadCodesResponse>
        for super::GetSimpleDownloadsDownloadCodesResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetSimpleDownloadsDownloadCodesResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<super::GetSimpleDownloadsDownloadCodesResponse>
        for GetSimpleDownloadsDownloadCodesResponse {
            fn from(value: super::GetSimpleDownloadsDownloadCodesResponse) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetSuppressionsBySuppressionCodeResponse {
            result: ::std::result::Result<
                super::SuppressionModel,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for GetSuppressionsBySuppressionCodeResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetSuppressionsBySuppressionCodeResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::SuppressionModel>,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<GetSuppressionsBySuppressionCodeResponse>
        for super::GetSuppressionsBySuppressionCodeResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetSuppressionsBySuppressionCodeResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<super::GetSuppressionsBySuppressionCodeResponse>
        for GetSuppressionsBySuppressionCodeResponse {
            fn from(value: super::GetSuppressionsBySuppressionCodeResponse) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct GetSuppressionsResponse {
            result: ::std::result::Result<
                ::std::vec::Vec<super::SuppressionModel>,
                ::std::string::String,
            >,
            version_information: ::std::result::Result<
                ::std::option::Option<super::VersionInformation>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for GetSuppressionsResponse {
            fn default() -> Self {
                Self {
                    result: Err("no value supplied for result".to_string()),
                    version_information: Ok(Default::default()),
                }
            }
        }
        impl GetSuppressionsResponse {
            pub fn result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::SuppressionModel>>,
                T::Error: ::std::fmt::Display,
            {
                self.result = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for result: {}", e)
                    });
                self
            }
            pub fn version_information<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::VersionInformation>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.version_information = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for version_information: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<GetSuppressionsResponse>
        for super::GetSuppressionsResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetSuppressionsResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    result: value.result?,
                    version_information: value.version_information?,
                })
            }
        }
        impl ::std::convert::From<super::GetSuppressionsResponse>
        for GetSuppressionsResponse {
            fn from(value: super::GetSuppressionsResponse) -> Self {
                Self {
                    result: Ok(value.result),
                    version_information: Ok(value.version_information),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct MappedReportingUnitModel {
            map_type: ::std::result::Result<
                super::MappedReportingUnitTypeModel,
                ::std::string::String,
            >,
            mapped_reporting_unit: ::std::result::Result<
                super::ReportingUnitSummaryModel,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for MappedReportingUnitModel {
            fn default() -> Self {
                Self {
                    map_type: Err("no value supplied for map_type".to_string()),
                    mapped_reporting_unit: Err(
                        "no value supplied for mapped_reporting_unit".to_string(),
                    ),
                }
            }
        }
        impl MappedReportingUnitModel {
            pub fn map_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::MappedReportingUnitTypeModel>,
                T::Error: ::std::fmt::Display,
            {
                self.map_type = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for map_type: {}", e)
                    });
                self
            }
            pub fn mapped_reporting_unit<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::ReportingUnitSummaryModel>,
                T::Error: ::std::fmt::Display,
            {
                self.mapped_reporting_unit = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for mapped_reporting_unit: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<MappedReportingUnitModel>
        for super::MappedReportingUnitModel {
            type Error = super::error::ConversionError;
            fn try_from(
                value: MappedReportingUnitModel,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    map_type: value.map_type?,
                    mapped_reporting_unit: value.mapped_reporting_unit?,
                })
            }
        }
        impl ::std::convert::From<super::MappedReportingUnitModel>
        for MappedReportingUnitModel {
            fn from(value: super::MappedReportingUnitModel) -> Self {
                Self {
                    map_type: Ok(value.map_type),
                    mapped_reporting_unit: Ok(value.mapped_reporting_unit),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct MappedReportingUnitTypeModel {
            mapped_reporting_unit_code: ::std::result::Result<
                super::MappedReportingUnitTypeModelMappedReportingUnitCode,
                ::std::string::String,
            >,
            mapped_reporting_unit_name: ::std::result::Result<
                super::MappedReportingUnitTypeModelMappedReportingUnitName,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for MappedReportingUnitTypeModel {
            fn default() -> Self {
                Self {
                    mapped_reporting_unit_code: Err(
                        "no value supplied for mapped_reporting_unit_code".to_string(),
                    ),
                    mapped_reporting_unit_name: Err(
                        "no value supplied for mapped_reporting_unit_name".to_string(),
                    ),
                }
            }
        }
        impl MappedReportingUnitTypeModel {
            pub fn mapped_reporting_unit_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::MappedReportingUnitTypeModelMappedReportingUnitCode,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.mapped_reporting_unit_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for mapped_reporting_unit_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn mapped_reporting_unit_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::MappedReportingUnitTypeModelMappedReportingUnitName,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.mapped_reporting_unit_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for mapped_reporting_unit_name: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<MappedReportingUnitTypeModel>
        for super::MappedReportingUnitTypeModel {
            type Error = super::error::ConversionError;
            fn try_from(
                value: MappedReportingUnitTypeModel,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    mapped_reporting_unit_code: value.mapped_reporting_unit_code?,
                    mapped_reporting_unit_name: value.mapped_reporting_unit_name?,
                })
            }
        }
        impl ::std::convert::From<super::MappedReportingUnitTypeModel>
        for MappedReportingUnitTypeModel {
            fn from(value: super::MappedReportingUnitTypeModel) -> Self {
                Self {
                    mapped_reporting_unit_code: Ok(value.mapped_reporting_unit_code),
                    mapped_reporting_unit_name: Ok(value.mapped_reporting_unit_name),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct MeasureCategoryModel {
            measure_category_code: ::std::result::Result<
                super::MeasureCategoryModelMeasureCategoryCode,
                ::std::string::String,
            >,
            measure_category_name: ::std::result::Result<
                super::MeasureCategoryModelMeasureCategoryName,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for MeasureCategoryModel {
            fn default() -> Self {
                Self {
                    measure_category_code: Err(
                        "no value supplied for measure_category_code".to_string(),
                    ),
                    measure_category_name: Err(
                        "no value supplied for measure_category_name".to_string(),
                    ),
                }
            }
        }
        impl MeasureCategoryModel {
            pub fn measure_category_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::MeasureCategoryModelMeasureCategoryCode,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.measure_category_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for measure_category_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn measure_category_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::MeasureCategoryModelMeasureCategoryName,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.measure_category_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for measure_category_name: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<MeasureCategoryModel>
        for super::MeasureCategoryModel {
            type Error = super::error::ConversionError;
            fn try_from(
                value: MeasureCategoryModel,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    measure_category_code: value.measure_category_code?,
                    measure_category_name: value.measure_category_name?,
                })
            }
        }
        impl ::std::convert::From<super::MeasureCategoryModel> for MeasureCategoryModel {
            fn from(value: super::MeasureCategoryModel) -> Self {
                Self {
                    measure_category_code: Ok(value.measure_category_code),
                    measure_category_name: Ok(value.measure_category_name),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct MeasureModel {
            measure_categories: ::std::result::Result<
                ::std::vec::Vec<super::MeasureCategoryModel>,
                ::std::string::String,
            >,
            measure_code: ::std::result::Result<
                super::MeasureModelMeasureCode,
                ::std::string::String,
            >,
            measure_name: ::std::result::Result<
                super::MeasureModelMeasureName,
                ::std::string::String,
            >,
            meta_tags: ::std::result::Result<
                ::std::vec::Vec<super::MetaTagModel>,
                ::std::string::String,
            >,
            units: ::std::result::Result<super::UnitsModel, ::std::string::String>,
        }
        impl ::std::default::Default for MeasureModel {
            fn default() -> Self {
                Self {
                    measure_categories: Err(
                        "no value supplied for measure_categories".to_string(),
                    ),
                    measure_code: Err("no value supplied for measure_code".to_string()),
                    measure_name: Err("no value supplied for measure_name".to_string()),
                    meta_tags: Err("no value supplied for meta_tags".to_string()),
                    units: Err("no value supplied for units".to_string()),
                }
            }
        }
        impl MeasureModel {
            pub fn measure_categories<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::MeasureCategoryModel>>,
                T::Error: ::std::fmt::Display,
            {
                self.measure_categories = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for measure_categories: {}",
                            e
                        )
                    });
                self
            }
            pub fn measure_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::MeasureModelMeasureCode>,
                T::Error: ::std::fmt::Display,
            {
                self.measure_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for measure_code: {}", e
                        )
                    });
                self
            }
            pub fn measure_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::MeasureModelMeasureName>,
                T::Error: ::std::fmt::Display,
            {
                self.measure_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for measure_name: {}", e
                        )
                    });
                self
            }
            pub fn meta_tags<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::MetaTagModel>>,
                T::Error: ::std::fmt::Display,
            {
                self.meta_tags = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for meta_tags: {}", e)
                    });
                self
            }
            pub fn units<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::UnitsModel>,
                T::Error: ::std::fmt::Display,
            {
                self.units = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for units: {}", e)
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<MeasureModel> for super::MeasureModel {
            type Error = super::error::ConversionError;
            fn try_from(
                value: MeasureModel,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    measure_categories: value.measure_categories?,
                    measure_code: value.measure_code?,
                    measure_name: value.measure_name?,
                    meta_tags: value.meta_tags?,
                    units: value.units?,
                })
            }
        }
        impl ::std::convert::From<super::MeasureModel> for MeasureModel {
            fn from(value: super::MeasureModel) -> Self {
                Self {
                    measure_categories: Ok(value.measure_categories),
                    measure_code: Ok(value.measure_code),
                    measure_name: Ok(value.measure_name),
                    meta_tags: Ok(value.meta_tags),
                    units: Ok(value.units),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct MeasureSummaryModel {
            measure_code: ::std::result::Result<
                super::MeasureSummaryModelMeasureCode,
                ::std::string::String,
            >,
            measure_name: ::std::result::Result<
                super::MeasureSummaryModelMeasureName,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for MeasureSummaryModel {
            fn default() -> Self {
                Self {
                    measure_code: Err("no value supplied for measure_code".to_string()),
                    measure_name: Err("no value supplied for measure_name".to_string()),
                }
            }
        }
        impl MeasureSummaryModel {
            pub fn measure_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::MeasureSummaryModelMeasureCode>,
                T::Error: ::std::fmt::Display,
            {
                self.measure_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for measure_code: {}", e
                        )
                    });
                self
            }
            pub fn measure_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::MeasureSummaryModelMeasureName>,
                T::Error: ::std::fmt::Display,
            {
                self.measure_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for measure_name: {}", e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<MeasureSummaryModel>
        for super::MeasureSummaryModel {
            type Error = super::error::ConversionError;
            fn try_from(
                value: MeasureSummaryModel,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    measure_code: value.measure_code?,
                    measure_name: value.measure_name?,
                })
            }
        }
        impl ::std::convert::From<super::MeasureSummaryModel> for MeasureSummaryModel {
            fn from(value: super::MeasureSummaryModel) -> Self {
                Self {
                    measure_code: Ok(value.measure_code),
                    measure_name: Ok(value.measure_name),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct MetaTagModel {
            meta_tag_code: ::std::result::Result<
                super::MetaTagModelMetaTagCode,
                ::std::string::String,
            >,
            meta_tag_name: ::std::result::Result<
                super::MetaTagModelMetaTagName,
                ::std::string::String,
            >,
            meta_tag_type: ::std::result::Result<
                super::MetaTagTypeModel,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for MetaTagModel {
            fn default() -> Self {
                Self {
                    meta_tag_code: Err(
                        "no value supplied for meta_tag_code".to_string(),
                    ),
                    meta_tag_name: Err(
                        "no value supplied for meta_tag_name".to_string(),
                    ),
                    meta_tag_type: Err("no value supplied for meta_tag_type".to_string()),
                }
            }
        }
        impl MetaTagModel {
            pub fn meta_tag_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::MetaTagModelMetaTagCode>,
                T::Error: ::std::fmt::Display,
            {
                self.meta_tag_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for meta_tag_code: {}", e
                        )
                    });
                self
            }
            pub fn meta_tag_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::MetaTagModelMetaTagName>,
                T::Error: ::std::fmt::Display,
            {
                self.meta_tag_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for meta_tag_name: {}", e
                        )
                    });
                self
            }
            pub fn meta_tag_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::MetaTagTypeModel>,
                T::Error: ::std::fmt::Display,
            {
                self.meta_tag_type = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for meta_tag_type: {}", e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<MetaTagModel> for super::MetaTagModel {
            type Error = super::error::ConversionError;
            fn try_from(
                value: MetaTagModel,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    meta_tag_code: value.meta_tag_code?,
                    meta_tag_name: value.meta_tag_name?,
                    meta_tag_type: value.meta_tag_type?,
                })
            }
        }
        impl ::std::convert::From<super::MetaTagModel> for MetaTagModel {
            fn from(value: super::MetaTagModel) -> Self {
                Self {
                    meta_tag_code: Ok(value.meta_tag_code),
                    meta_tag_name: Ok(value.meta_tag_name),
                    meta_tag_type: Ok(value.meta_tag_type),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct MetaTagTypeModel {
            meta_tag_type_code: ::std::result::Result<
                super::MetaTagTypeModelMetaTagTypeCode,
                ::std::string::String,
            >,
            meta_tag_type_name: ::std::result::Result<
                super::MetaTagTypeModelMetaTagTypeName,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for MetaTagTypeModel {
            fn default() -> Self {
                Self {
                    meta_tag_type_code: Err(
                        "no value supplied for meta_tag_type_code".to_string(),
                    ),
                    meta_tag_type_name: Err(
                        "no value supplied for meta_tag_type_name".to_string(),
                    ),
                }
            }
        }
        impl MetaTagTypeModel {
            pub fn meta_tag_type_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::MetaTagTypeModelMetaTagTypeCode>,
                T::Error: ::std::fmt::Display,
            {
                self.meta_tag_type_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for meta_tag_type_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn meta_tag_type_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::MetaTagTypeModelMetaTagTypeName>,
                T::Error: ::std::fmt::Display,
            {
                self.meta_tag_type_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for meta_tag_type_name: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<MetaTagTypeModel> for super::MetaTagTypeModel {
            type Error = super::error::ConversionError;
            fn try_from(
                value: MetaTagTypeModel,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    meta_tag_type_code: value.meta_tag_type_code?,
                    meta_tag_type_name: value.meta_tag_type_name?,
                })
            }
        }
        impl ::std::convert::From<super::MetaTagTypeModel> for MetaTagTypeModel {
            fn from(value: super::MetaTagTypeModel) -> Self {
                Self {
                    meta_tag_type_code: Ok(value.meta_tag_type_code),
                    meta_tag_type_name: Ok(value.meta_tag_type_name),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct PaginatedDataExtractModel {
            data: ::std::result::Result<
                ::std::option::Option<::std::vec::Vec<super::DataExtractModel>>,
                ::std::string::String,
            >,
            pagination: ::std::result::Result<
                ::std::option::Option<super::DataExtractPagination>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for PaginatedDataExtractModel {
            fn default() -> Self {
                Self {
                    data: Ok(Default::default()),
                    pagination: Ok(Default::default()),
                }
            }
        }
        impl PaginatedDataExtractModel {
            pub fn data<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::std::vec::Vec<super::DataExtractModel>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.data = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for data: {}", e)
                    });
                self
            }
            pub fn pagination<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::DataExtractPagination>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.pagination = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for pagination: {}", e)
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<PaginatedDataExtractModel>
        for super::PaginatedDataExtractModel {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PaginatedDataExtractModel,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    data: value.data?,
                    pagination: value.pagination?,
                })
            }
        }
        impl ::std::convert::From<super::PaginatedDataExtractModel>
        for PaginatedDataExtractModel {
            fn from(value: super::PaginatedDataExtractModel) -> Self {
                Self {
                    data: Ok(value.data),
                    pagination: Ok(value.pagination),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct PaginatedFormattedDataExtractModel {
            data: ::std::result::Result<
                ::std::option::Option<::std::vec::Vec<super::FormattedDataExtractModel>>,
                ::std::string::String,
            >,
            pagination: ::std::result::Result<
                ::std::option::Option<super::DataExtractPagination>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for PaginatedFormattedDataExtractModel {
            fn default() -> Self {
                Self {
                    data: Ok(Default::default()),
                    pagination: Ok(Default::default()),
                }
            }
        }
        impl PaginatedFormattedDataExtractModel {
            pub fn data<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<
                        ::std::vec::Vec<super::FormattedDataExtractModel>,
                    >,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.data = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for data: {}", e)
                    });
                self
            }
            pub fn pagination<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::DataExtractPagination>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.pagination = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for pagination: {}", e)
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<PaginatedFormattedDataExtractModel>
        for super::PaginatedFormattedDataExtractModel {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PaginatedFormattedDataExtractModel,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    data: value.data?,
                    pagination: value.pagination?,
                })
            }
        }
        impl ::std::convert::From<super::PaginatedFormattedDataExtractModel>
        for PaginatedFormattedDataExtractModel {
            fn from(value: super::PaginatedFormattedDataExtractModel) -> Self {
                Self {
                    data: Ok(value.data),
                    pagination: Ok(value.pagination),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ProblemDetails {
            detail: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            instance: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            status: ::std::result::Result<
                ::std::option::Option<i32>,
                ::std::string::String,
            >,
            title: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            type_: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            extra: ::std::result::Result<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ProblemDetails {
            fn default() -> Self {
                Self {
                    detail: Ok(Default::default()),
                    instance: Ok(Default::default()),
                    status: Ok(Default::default()),
                    title: Ok(Default::default()),
                    type_: Ok(Default::default()),
                    extra: Err("no value supplied for extra".to_string()),
                }
            }
        }
        impl ProblemDetails {
            pub fn detail<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.detail = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for detail: {}", e)
                    });
                self
            }
            pub fn instance<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.instance = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for instance: {}", e)
                    });
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for status: {}", e)
                    });
                self
            }
            pub fn title<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.title = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for title: {}", e)
                    });
                self
            }
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for type_: {}", e)
                    });
                self
            }
            pub fn extra<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.extra = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for extra: {}", e)
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<ProblemDetails> for super::ProblemDetails {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ProblemDetails,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    detail: value.detail?,
                    instance: value.instance?,
                    status: value.status?,
                    title: value.title?,
                    type_: value.type_?,
                    extra: value.extra?,
                })
            }
        }
        impl ::std::convert::From<super::ProblemDetails> for ProblemDetails {
            fn from(value: super::ProblemDetails) -> Self {
                Self {
                    detail: Ok(value.detail),
                    instance: Ok(value.instance),
                    status: Ok(value.status),
                    title: Ok(value.title),
                    type_: Ok(value.type_),
                    extra: Ok(value.extra),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ReportedMeasureCategoryModel {
            reported_measure_category_code: ::std::result::Result<
                super::ReportedMeasureCategoryModelReportedMeasureCategoryCode,
                ::std::string::String,
            >,
            reported_measure_category_name: ::std::result::Result<
                super::ReportedMeasureCategoryModelReportedMeasureCategoryName,
                ::std::string::String,
            >,
            reported_measure_category_type: ::std::result::Result<
                super::ReportedMeasureCategoryTypeModel,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ReportedMeasureCategoryModel {
            fn default() -> Self {
                Self {
                    reported_measure_category_code: Err(
                        "no value supplied for reported_measure_category_code"
                            .to_string(),
                    ),
                    reported_measure_category_name: Err(
                        "no value supplied for reported_measure_category_name"
                            .to_string(),
                    ),
                    reported_measure_category_type: Err(
                        "no value supplied for reported_measure_category_type"
                            .to_string(),
                    ),
                }
            }
        }
        impl ReportedMeasureCategoryModel {
            pub fn reported_measure_category_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::ReportedMeasureCategoryModelReportedMeasureCategoryCode,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::ReportedMeasureCategoryModelReportedMeasureCategoryName,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_name: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::ReportedMeasureCategoryTypeModel>,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_type = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_type: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<ReportedMeasureCategoryModel>
        for super::ReportedMeasureCategoryModel {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ReportedMeasureCategoryModel,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    reported_measure_category_code: value
                        .reported_measure_category_code?,
                    reported_measure_category_name: value
                        .reported_measure_category_name?,
                    reported_measure_category_type: value.reported_measure_category_type?,
                })
            }
        }
        impl ::std::convert::From<super::ReportedMeasureCategoryModel>
        for ReportedMeasureCategoryModel {
            fn from(value: super::ReportedMeasureCategoryModel) -> Self {
                Self {
                    reported_measure_category_code: Ok(
                        value.reported_measure_category_code,
                    ),
                    reported_measure_category_name: Ok(
                        value.reported_measure_category_name,
                    ),
                    reported_measure_category_type: Ok(
                        value.reported_measure_category_type,
                    ),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ReportedMeasureCategoryTypeModel {
            reported_measure_category_type_code: ::std::result::Result<
                super::ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeCode,
                ::std::string::String,
            >,
            reported_measure_category_type_name: ::std::result::Result<
                super::ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeName,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ReportedMeasureCategoryTypeModel {
            fn default() -> Self {
                Self {
                    reported_measure_category_type_code: Err(
                        "no value supplied for reported_measure_category_type_code"
                            .to_string(),
                    ),
                    reported_measure_category_type_name: Err(
                        "no value supplied for reported_measure_category_type_name"
                            .to_string(),
                    ),
                }
            }
        }
        impl ReportedMeasureCategoryTypeModel {
            pub fn reported_measure_category_type_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeCode,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_type_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_type_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_category_type_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::ReportedMeasureCategoryTypeModelReportedMeasureCategoryTypeName,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_category_type_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_category_type_name: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<ReportedMeasureCategoryTypeModel>
        for super::ReportedMeasureCategoryTypeModel {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ReportedMeasureCategoryTypeModel,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    reported_measure_category_type_code: value
                        .reported_measure_category_type_code?,
                    reported_measure_category_type_name: value
                        .reported_measure_category_type_name?,
                })
            }
        }
        impl ::std::convert::From<super::ReportedMeasureCategoryTypeModel>
        for ReportedMeasureCategoryTypeModel {
            fn from(value: super::ReportedMeasureCategoryTypeModel) -> Self {
                Self {
                    reported_measure_category_type_code: Ok(
                        value.reported_measure_category_type_code,
                    ),
                    reported_measure_category_type_name: Ok(
                        value.reported_measure_category_type_name,
                    ),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ReportedMeasureModel {
            measure: ::std::result::Result<super::MeasureModel, ::std::string::String>,
            meta_tags: ::std::result::Result<
                ::std::vec::Vec<super::MetaTagModel>,
                ::std::string::String,
            >,
            reported_measure_categories: ::std::result::Result<
                ::std::vec::Vec<super::ReportedMeasureCategoryModel>,
                ::std::string::String,
            >,
            reported_measure_code: ::std::result::Result<
                super::ReportedMeasureModelReportedMeasureCode,
                ::std::string::String,
            >,
            reported_measure_name: ::std::result::Result<
                super::ReportedMeasureModelReportedMeasureName,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ReportedMeasureModel {
            fn default() -> Self {
                Self {
                    measure: Err("no value supplied for measure".to_string()),
                    meta_tags: Err("no value supplied for meta_tags".to_string()),
                    reported_measure_categories: Err(
                        "no value supplied for reported_measure_categories".to_string(),
                    ),
                    reported_measure_code: Err(
                        "no value supplied for reported_measure_code".to_string(),
                    ),
                    reported_measure_name: Err(
                        "no value supplied for reported_measure_name".to_string(),
                    ),
                }
            }
        }
        impl ReportedMeasureModel {
            pub fn measure<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::MeasureModel>,
                T::Error: ::std::fmt::Display,
            {
                self.measure = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for measure: {}", e)
                    });
                self
            }
            pub fn meta_tags<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::MetaTagModel>>,
                T::Error: ::std::fmt::Display,
            {
                self.meta_tags = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for meta_tags: {}", e)
                    });
                self
            }
            pub fn reported_measure_categories<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::vec::Vec<super::ReportedMeasureCategoryModel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_categories = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_categories: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::ReportedMeasureModelReportedMeasureCode,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::ReportedMeasureModelReportedMeasureName,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_name: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<ReportedMeasureModel>
        for super::ReportedMeasureModel {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ReportedMeasureModel,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    measure: value.measure?,
                    meta_tags: value.meta_tags?,
                    reported_measure_categories: value.reported_measure_categories?,
                    reported_measure_code: value.reported_measure_code?,
                    reported_measure_name: value.reported_measure_name?,
                })
            }
        }
        impl ::std::convert::From<super::ReportedMeasureModel> for ReportedMeasureModel {
            fn from(value: super::ReportedMeasureModel) -> Self {
                Self {
                    measure: Ok(value.measure),
                    meta_tags: Ok(value.meta_tags),
                    reported_measure_categories: Ok(value.reported_measure_categories),
                    reported_measure_code: Ok(value.reported_measure_code),
                    reported_measure_name: Ok(value.reported_measure_name),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ReportedMeasureSummaryModel {
            measure_summary: ::std::result::Result<
                super::MeasureSummaryModel,
                ::std::string::String,
            >,
            reported_measure_code: ::std::result::Result<
                super::ReportedMeasureSummaryModelReportedMeasureCode,
                ::std::string::String,
            >,
            reported_measure_name: ::std::result::Result<
                super::ReportedMeasureSummaryModelReportedMeasureName,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ReportedMeasureSummaryModel {
            fn default() -> Self {
                Self {
                    measure_summary: Err(
                        "no value supplied for measure_summary".to_string(),
                    ),
                    reported_measure_code: Err(
                        "no value supplied for reported_measure_code".to_string(),
                    ),
                    reported_measure_name: Err(
                        "no value supplied for reported_measure_name".to_string(),
                    ),
                }
            }
        }
        impl ReportedMeasureSummaryModel {
            pub fn measure_summary<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::MeasureSummaryModel>,
                T::Error: ::std::fmt::Display,
            {
                self.measure_summary = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for measure_summary: {}", e
                        )
                    });
                self
            }
            pub fn reported_measure_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::ReportedMeasureSummaryModelReportedMeasureCode,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reported_measure_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::ReportedMeasureSummaryModelReportedMeasureName,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reported_measure_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reported_measure_name: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<ReportedMeasureSummaryModel>
        for super::ReportedMeasureSummaryModel {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ReportedMeasureSummaryModel,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    measure_summary: value.measure_summary?,
                    reported_measure_code: value.reported_measure_code?,
                    reported_measure_name: value.reported_measure_name?,
                })
            }
        }
        impl ::std::convert::From<super::ReportedMeasureSummaryModel>
        for ReportedMeasureSummaryModel {
            fn from(value: super::ReportedMeasureSummaryModel) -> Self {
                Self {
                    measure_summary: Ok(value.measure_summary),
                    reported_measure_code: Ok(value.reported_measure_code),
                    reported_measure_name: Ok(value.reported_measure_name),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ReportingUnitModel {
            alternative_names: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            closed: ::std::result::Result<bool, ::std::string::String>,
            latitude: ::std::result::Result<
                ::std::option::Option<f64>,
                ::std::string::String,
            >,
            longitude: ::std::result::Result<
                ::std::option::Option<f64>,
                ::std::string::String,
            >,
            mapped_reporting_units: ::std::result::Result<
                ::std::vec::Vec<super::MappedReportingUnitModel>,
                ::std::string::String,
            >,
            meta_tags: ::std::result::Result<
                ::std::vec::Vec<super::MetaTagModel>,
                ::std::string::String,
            >,
            private: ::std::result::Result<bool, ::std::string::String>,
            reporting_unit_code: ::std::result::Result<
                super::ReportingUnitModelReportingUnitCode,
                ::std::string::String,
            >,
            reporting_unit_name: ::std::result::Result<
                super::ReportingUnitModelReportingUnitName,
                ::std::string::String,
            >,
            reporting_unit_type: ::std::result::Result<
                super::ReportingUnitTypeModel,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ReportingUnitModel {
            fn default() -> Self {
                Self {
                    alternative_names: Err(
                        "no value supplied for alternative_names".to_string(),
                    ),
                    closed: Err("no value supplied for closed".to_string()),
                    latitude: Ok(Default::default()),
                    longitude: Ok(Default::default()),
                    mapped_reporting_units: Err(
                        "no value supplied for mapped_reporting_units".to_string(),
                    ),
                    meta_tags: Err("no value supplied for meta_tags".to_string()),
                    private: Err("no value supplied for private".to_string()),
                    reporting_unit_code: Err(
                        "no value supplied for reporting_unit_code".to_string(),
                    ),
                    reporting_unit_name: Err(
                        "no value supplied for reporting_unit_name".to_string(),
                    ),
                    reporting_unit_type: Err(
                        "no value supplied for reporting_unit_type".to_string(),
                    ),
                }
            }
        }
        impl ReportingUnitModel {
            pub fn alternative_names<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.alternative_names = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for alternative_names: {}",
                            e
                        )
                    });
                self
            }
            pub fn closed<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.closed = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for closed: {}", e)
                    });
                self
            }
            pub fn latitude<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.latitude = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for latitude: {}", e)
                    });
                self
            }
            pub fn longitude<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.longitude = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for longitude: {}", e)
                    });
                self
            }
            pub fn mapped_reporting_units<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::vec::Vec<super::MappedReportingUnitModel>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.mapped_reporting_units = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for mapped_reporting_units: {}",
                            e
                        )
                    });
                self
            }
            pub fn meta_tags<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::MetaTagModel>>,
                T::Error: ::std::fmt::Display,
            {
                self.meta_tags = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for meta_tags: {}", e)
                    });
                self
            }
            pub fn private<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.private = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for private: {}", e)
                    });
                self
            }
            pub fn reporting_unit_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::ReportingUnitModelReportingUnitCode>,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_unit_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_unit_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_unit_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::ReportingUnitModelReportingUnitName>,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_unit_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_unit_name: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_unit_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::ReportingUnitTypeModel>,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_unit_type = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_unit_type: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<ReportingUnitModel> for super::ReportingUnitModel {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ReportingUnitModel,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    alternative_names: value.alternative_names?,
                    closed: value.closed?,
                    latitude: value.latitude?,
                    longitude: value.longitude?,
                    mapped_reporting_units: value.mapped_reporting_units?,
                    meta_tags: value.meta_tags?,
                    private: value.private?,
                    reporting_unit_code: value.reporting_unit_code?,
                    reporting_unit_name: value.reporting_unit_name?,
                    reporting_unit_type: value.reporting_unit_type?,
                })
            }
        }
        impl ::std::convert::From<super::ReportingUnitModel> for ReportingUnitModel {
            fn from(value: super::ReportingUnitModel) -> Self {
                Self {
                    alternative_names: Ok(value.alternative_names),
                    closed: Ok(value.closed),
                    latitude: Ok(value.latitude),
                    longitude: Ok(value.longitude),
                    mapped_reporting_units: Ok(value.mapped_reporting_units),
                    meta_tags: Ok(value.meta_tags),
                    private: Ok(value.private),
                    reporting_unit_code: Ok(value.reporting_unit_code),
                    reporting_unit_name: Ok(value.reporting_unit_name),
                    reporting_unit_type: Ok(value.reporting_unit_type),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ReportingUnitSummaryModel {
            reporting_unit_code: ::std::result::Result<
                super::ReportingUnitSummaryModelReportingUnitCode,
                ::std::string::String,
            >,
            reporting_unit_name: ::std::result::Result<
                super::ReportingUnitSummaryModelReportingUnitName,
                ::std::string::String,
            >,
            reporting_unit_type: ::std::result::Result<
                super::ReportingUnitTypeModel,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ReportingUnitSummaryModel {
            fn default() -> Self {
                Self {
                    reporting_unit_code: Err(
                        "no value supplied for reporting_unit_code".to_string(),
                    ),
                    reporting_unit_name: Err(
                        "no value supplied for reporting_unit_name".to_string(),
                    ),
                    reporting_unit_type: Err(
                        "no value supplied for reporting_unit_type".to_string(),
                    ),
                }
            }
        }
        impl ReportingUnitSummaryModel {
            pub fn reporting_unit_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::ReportingUnitSummaryModelReportingUnitCode,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_unit_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_unit_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_unit_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::ReportingUnitSummaryModelReportingUnitName,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_unit_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_unit_name: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_unit_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::ReportingUnitTypeModel>,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_unit_type = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_unit_type: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<ReportingUnitSummaryModel>
        for super::ReportingUnitSummaryModel {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ReportingUnitSummaryModel,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    reporting_unit_code: value.reporting_unit_code?,
                    reporting_unit_name: value.reporting_unit_name?,
                    reporting_unit_type: value.reporting_unit_type?,
                })
            }
        }
        impl ::std::convert::From<super::ReportingUnitSummaryModel>
        for ReportingUnitSummaryModel {
            fn from(value: super::ReportingUnitSummaryModel) -> Self {
                Self {
                    reporting_unit_code: Ok(value.reporting_unit_code),
                    reporting_unit_name: Ok(value.reporting_unit_name),
                    reporting_unit_type: Ok(value.reporting_unit_type),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct ReportingUnitTypeModel {
            reporting_unit_type_code: ::std::result::Result<
                super::ReportingUnitTypeModelReportingUnitTypeCode,
                ::std::string::String,
            >,
            reporting_unit_type_name: ::std::result::Result<
                super::ReportingUnitTypeModelReportingUnitTypeName,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for ReportingUnitTypeModel {
            fn default() -> Self {
                Self {
                    reporting_unit_type_code: Err(
                        "no value supplied for reporting_unit_type_code".to_string(),
                    ),
                    reporting_unit_type_name: Err(
                        "no value supplied for reporting_unit_type_name".to_string(),
                    ),
                }
            }
        }
        impl ReportingUnitTypeModel {
            pub fn reporting_unit_type_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::ReportingUnitTypeModelReportingUnitTypeCode,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_unit_type_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_unit_type_code: {}",
                            e
                        )
                    });
                self
            }
            pub fn reporting_unit_type_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    super::ReportingUnitTypeModelReportingUnitTypeName,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.reporting_unit_type_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for reporting_unit_type_name: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<ReportingUnitTypeModel>
        for super::ReportingUnitTypeModel {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ReportingUnitTypeModel,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    reporting_unit_type_code: value.reporting_unit_type_code?,
                    reporting_unit_type_name: value.reporting_unit_type_name?,
                })
            }
        }
        impl ::std::convert::From<super::ReportingUnitTypeModel>
        for ReportingUnitTypeModel {
            fn from(value: super::ReportingUnitTypeModel) -> Self {
                Self {
                    reporting_unit_type_code: Ok(value.reporting_unit_type_code),
                    reporting_unit_type_name: Ok(value.reporting_unit_type_name),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct SuppressionModel {
            suppression_code: ::std::result::Result<
                super::SuppressionModelSuppressionCode,
                ::std::string::String,
            >,
            suppression_display_value: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            suppression_footnote: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            suppression_name: ::std::result::Result<
                super::SuppressionModelSuppressionName,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for SuppressionModel {
            fn default() -> Self {
                Self {
                    suppression_code: Err(
                        "no value supplied for suppression_code".to_string(),
                    ),
                    suppression_display_value: Ok(Default::default()),
                    suppression_footnote: Ok(Default::default()),
                    suppression_name: Err(
                        "no value supplied for suppression_name".to_string(),
                    ),
                }
            }
        }
        impl SuppressionModel {
            pub fn suppression_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::SuppressionModelSuppressionCode>,
                T::Error: ::std::fmt::Display,
            {
                self.suppression_code = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for suppression_code: {}", e
                        )
                    });
                self
            }
            pub fn suppression_display_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.suppression_display_value = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for suppression_display_value: {}",
                            e
                        )
                    });
                self
            }
            pub fn suppression_footnote<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.suppression_footnote = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for suppression_footnote: {}",
                            e
                        )
                    });
                self
            }
            pub fn suppression_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::SuppressionModelSuppressionName>,
                T::Error: ::std::fmt::Display,
            {
                self.suppression_name = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for suppression_name: {}", e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<SuppressionModel> for super::SuppressionModel {
            type Error = super::error::ConversionError;
            fn try_from(
                value: SuppressionModel,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    suppression_code: value.suppression_code?,
                    suppression_display_value: value.suppression_display_value?,
                    suppression_footnote: value.suppression_footnote?,
                    suppression_name: value.suppression_name?,
                })
            }
        }
        impl ::std::convert::From<super::SuppressionModel> for SuppressionModel {
            fn from(value: super::SuppressionModel) -> Self {
                Self {
                    suppression_code: Ok(value.suppression_code),
                    suppression_display_value: Ok(value.suppression_display_value),
                    suppression_footnote: Ok(value.suppression_footnote),
                    suppression_name: Ok(value.suppression_name),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct UnitsModel {
            decimal_places: ::std::result::Result<i32, ::std::string::String>,
            units_are_prefix: ::std::result::Result<bool, ::std::string::String>,
            units_display: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            units_name: ::std::result::Result<
                super::UnitsModelUnitsName,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for UnitsModel {
            fn default() -> Self {
                Self {
                    decimal_places: Err(
                        "no value supplied for decimal_places".to_string(),
                    ),
                    units_are_prefix: Err(
                        "no value supplied for units_are_prefix".to_string(),
                    ),
                    units_display: Ok(Default::default()),
                    units_name: Err("no value supplied for units_name".to_string()),
                }
            }
        }
        impl UnitsModel {
            pub fn decimal_places<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i32>,
                T::Error: ::std::fmt::Display,
            {
                self.decimal_places = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for decimal_places: {}", e
                        )
                    });
                self
            }
            pub fn units_are_prefix<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.units_are_prefix = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for units_are_prefix: {}", e
                        )
                    });
                self
            }
            pub fn units_display<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.units_display = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for units_display: {}", e
                        )
                    });
                self
            }
            pub fn units_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::UnitsModelUnitsName>,
                T::Error: ::std::fmt::Display,
            {
                self.units_name = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for units_name: {}", e)
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<UnitsModel> for super::UnitsModel {
            type Error = super::error::ConversionError;
            fn try_from(
                value: UnitsModel,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    decimal_places: value.decimal_places?,
                    units_are_prefix: value.units_are_prefix?,
                    units_display: value.units_display?,
                    units_name: value.units_name?,
                })
            }
        }
        impl ::std::convert::From<super::UnitsModel> for UnitsModel {
            fn from(value: super::UnitsModel) -> Self {
                Self {
                    decimal_places: Ok(value.decimal_places),
                    units_are_prefix: Ok(value.units_are_prefix),
                    units_display: Ok(value.units_display),
                    units_name: Ok(value.units_name),
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct VersionInformation {
            api_version: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            data_version: ::std::result::Result<
                ::std::option::Option<i64>,
                ::std::string::String,
            >,
            date_uploaded: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            requested_time_stamp: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }
        impl ::std::default::Default for VersionInformation {
            fn default() -> Self {
                Self {
                    api_version: Ok(Default::default()),
                    data_version: Ok(Default::default()),
                    date_uploaded: Ok(Default::default()),
                    requested_time_stamp: Ok(Default::default()),
                }
            }
        }
        impl VersionInformation {
            pub fn api_version<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.api_version = value
                    .try_into()
                    .map_err(|e| {
                        format!("error converting supplied value for api_version: {}", e)
                    });
                self
            }
            pub fn data_version<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.data_version = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for data_version: {}", e
                        )
                    });
                self
            }
            pub fn date_uploaded<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.date_uploaded = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for date_uploaded: {}", e
                        )
                    });
                self
            }
            pub fn requested_time_stamp<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.requested_time_stamp = value
                    .try_into()
                    .map_err(|e| {
                        format!(
                            "error converting supplied value for requested_time_stamp: {}",
                            e
                        )
                    });
                self
            }
        }
        impl ::std::convert::TryFrom<VersionInformation> for super::VersionInformation {
            type Error = super::error::ConversionError;
            fn try_from(
                value: VersionInformation,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    api_version: value.api_version?,
                    data_version: value.data_version?,
                    date_uploaded: value.date_uploaded?,
                    requested_time_stamp: value.requested_time_stamp?,
                })
            }
        }
        impl ::std::convert::From<super::VersionInformation> for VersionInformation {
            fn from(value: super::VersionInformation) -> Self {
                Self {
                    api_version: Ok(value.api_version),
                    data_version: Ok(value.data_version),
                    date_uploaded: Ok(value.date_uploaded),
                    requested_time_stamp: Ok(value.requested_time_stamp),
                }
            }
        }
    }
}
#[derive(Clone, Debug)]
/**Client for MyHospitals Data API

<p>This page provides details of the calls available within the MyHospitals Data API, followed by schema documentation of what the calls return.</p>

Version: 1.0*/
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}
impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = ::std::time::Duration::from_secs(15u64);
            reqwest::ClientBuilder::new().connect_timeout(dur).timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }
    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
}
impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "1.0"
    }
    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }
    fn client(&self) -> &reqwest::Client {
        &self.client
    }
    fn inner(&self) -> &() {
        &()
    }
}
impl ClientHooks<()> for &Client {}
impl Client {
    /**Gets a list of caveats

Gets list of all caveats.

Sends a `GET` request to `/api/v1/caveats`

Arguments:
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_caveats()
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_caveats(&self) -> builder::GetCaveats<'_> {
        builder::GetCaveats::new(self)
    }
    /**Gets a caveat

Gets a caveat matching the supplied caveat code.

Sends a `GET` request to `/api/v1/caveats/{caveat-code}`

Arguments:
- `caveat_code`: The caveat code.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_caveats_by_caveat_code()
    .caveat_code(caveat_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_caveats_by_caveat_code(&self) -> builder::GetCaveatsByCaveatCode<'_> {
        builder::GetCaveatsByCaveatCode::new(self)
    }
    /**Gets a list of datasets

Gets a list of datasets, optionally filtered by a list of measure and reported measure codes. \
                Matching within each filter is by logical disjunction (OR) and matching across filters is by logical conjunction (AND).

Sends a `GET` request to `/api/v1/datasets`

Arguments:
- `measure_code`: The measure codes.
- `reported_measure_code`: The reported measure codes.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_datasets()
    .measure_code(measure_code)
    .reported_measure_code(reported_measure_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_datasets(&self) -> builder::GetDatasets<'_> {
        builder::GetDatasets::new(self)
    }
    /**Gets a single dataset

Gets a single dataset matching the supplied dataset id.

Sends a `GET` request to `/api/v1/datasets/{dataset-id}`

Arguments:
- `dataset_id`: The dataset id.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_datasets_by_dataset_id()
    .dataset_id(dataset_id)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_datasets_by_dataset_id(&self) -> builder::GetDatasetsByDatasetId<'_> {
        builder::GetDatasetsByDatasetId::new(self)
    }
    /**Gets a list of data items for a dataset

Gets a list of data items for a dataset,, optionally filtered, using logical disjunction (OR), by a list of reporting unit codes.

Sends a `GET` request to `/api/v1/datasets/{dataset-id}/data-items`

Arguments:
- `dataset_id`: The dataset id.
- `reporting_unit_code`: The reporting unit codes.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_datasets_by_dataset_id_data_items()
    .dataset_id(dataset_id)
    .reporting_unit_code(reporting_unit_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_datasets_by_dataset_id_data_items(
        &self,
    ) -> builder::GetDatasetsByDatasetIdDataItems<'_> {
        builder::GetDatasetsByDatasetIdDataItems::new(self)
    }
    /**Gets flat data

Gets data in a flattened structure with each item corresponding to a single data point. \
                Data is restricted to within a single measure category and can be optionally filtered by measure, reporting unit type and reporting dates. Matching within each filter is by logical disjunction (OR) and matching across filters is by logical conjunction (AND). \
                NOTE: There is currently a pagination restriction on this query that restricts requests to a maximum of 1000 results. Use skip and top to control this pagination. Sorting is not currently available.

Sends a `GET` request to `/api/v1/flat-data-extract/{measure-category-code}`

Arguments:
- `measure_category_code`: Specifies what measure category code the data should belongs to.
- `end_date`: Specifies that only data before a certain point should be returned. endDate can be a date of the form yyyy, yyyy-MM or yyyy-MM-dd. When only a year is specified, the search period ends at the end of the year, when a year and month are specified, the search period ends at the end of the month.
- `measure_code`: Only include data that matches the specified measures.
- `reporting_unit_code`: Only include data for the specified reporting units.
- `reporting_unit_type_code`: Only include data for the specified reporting unit types.
- `skip`: The number of records to skip. This is used in the pagination of the data results to determine where to start taking the results from.
- `start_date`: Specifies that only data after a certain point should be returned. startDate can be a date of the form yyyy, yyyy-MM or yyyy-MM-dd. When only a year is specified, the search period starts at the beginning of the year, when a year and month are specified, the search period starts at the beginning of the month.
- `top`: The number of records to take. This is used in the pagination of the data results to determine how many results are retrieved. This value must be between 1 and 1000.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_flat_data_extract_by_measure_category_code()
    .measure_category_code(measure_category_code)
    .end_date(end_date)
    .measure_code(measure_code)
    .reporting_unit_code(reporting_unit_code)
    .reporting_unit_type_code(reporting_unit_type_code)
    .skip(skip)
    .start_date(start_date)
    .top(top)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_flat_data_extract_by_measure_category_code(
        &self,
    ) -> builder::GetFlatDataExtractByMeasureCategoryCode<'_> {
        builder::GetFlatDataExtractByMeasureCategoryCode::new(self)
    }
    /**Gets flat formatted data

Gets formatted data in a flattened structure with each item corresponding to a single data point. \
                Data is restricted to within a single measure category and can be optionally filtered by measure, reporting unit code, reporting unit type and reporting dates. Matching within each filter is by logical disjunction (OR) and matching across filters is by logical conjunction (AND). \
                NOTE: There is currently a pagination restriction on this query that restricts requests to a maximum of 1000 results. Use skip and top to control this pagination. Sorting is not currently available.

Sends a `GET` request to `/api/v1/flat-formatted-data-extract/{measure-category-code}`

Arguments:
- `measure_category_code`: Specifies what measure category code the data should belongs to.
- `end_date`: Specifies that only data before a certain point should be returned. endDate can be a date of the form yyyy, yyyy-MM or yyyy-MM-dd. When only a year is specified, the search period ends at the end of the year, when a year and month are specified, the search period ends at the end of the month.
- `measure_code`: Only include data that matches the specified measures.
- `reporting_unit_code`: Only include data for the specified reporting units.
- `reporting_unit_type_code`: Only include data for the specified reporting unit types.
- `skip`: The number of records to skip. This is used in the pagination of the data results to determine where to start taking the results from.
- `start_date`: Specifies that only data after a certain point should be returned. startDate can be a date of the form yyyy, yyyy-MM or yyyy-MM-dd. When only a year is specified, the search period starts at the beginning of the year, when a year and month are specified, the search period starts at the beginning of the month.
- `top`: The number of records to take. This is used in the pagination of the data results to determine how many results are retrieved. This value must be between 1 and 1000.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_flat_formatted_data_extract_by_measure_category_code()
    .measure_category_code(measure_category_code)
    .end_date(end_date)
    .measure_code(measure_code)
    .reporting_unit_code(reporting_unit_code)
    .reporting_unit_type_code(reporting_unit_type_code)
    .skip(skip)
    .start_date(start_date)
    .top(top)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_flat_formatted_data_extract_by_measure_category_code(
        &self,
    ) -> builder::GetFlatFormattedDataExtractByMeasureCategoryCode<'_> {
        builder::GetFlatFormattedDataExtractByMeasureCategoryCode::new(self)
    }
    /**Gets a list of measure categories

Gets list of all measure categories available.

Sends a `GET` request to `/api/v1/measure-categories`

Arguments:
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_measure_categories()
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_measure_categories(&self) -> builder::GetMeasureCategories<'_> {
        builder::GetMeasureCategories::new(self)
    }
    /**Gets a single measure category

Gets a single measure category matching the supplied measure category code.

Sends a `GET` request to `/api/v1/measure-categories/{measure-category-code}`

Arguments:
- `measure_category_code`: The measure category code.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_measure_categories_by_measure_category_code()
    .measure_category_code(measure_category_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_measure_categories_by_measure_category_code(
        &self,
    ) -> builder::GetMeasureCategoriesByMeasureCategoryCode<'_> {
        builder::GetMeasureCategoriesByMeasureCategoryCode::new(self)
    }
    /**Gets a list of measures for a measure category

Gets a list of measures for the specified measure category code.

Sends a `GET` request to `/api/v1/measure-categories/{measure-category-code}/measures`

Arguments:
- `measure_category_code`: The measure category code.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_measure_categories_by_measure_category_code_measures()
    .measure_category_code(measure_category_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_measure_categories_by_measure_category_code_measures(
        &self,
    ) -> builder::GetMeasureCategoriesByMeasureCategoryCodeMeasures<'_> {
        builder::GetMeasureCategoriesByMeasureCategoryCodeMeasures::new(self)
    }
    /**Gets a measure across reporting unit download

Gets a measure across reporting unit download, in XLSX format, for the supplied measure download code.

Sends a `GET` request to `/api/v1/measure-downloads/across-reporting-units/{measure-download-code}`

Arguments:
- `measure_download_code`: The measure download code.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_measure_downloads_across_reporting_units_by_measure_download_code()
    .measure_download_code(measure_download_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_measure_downloads_across_reporting_units_by_measure_download_code(
        &self,
    ) -> builder::GetMeasureDownloadsAcrossReportingUnitsByMeasureDownloadCode<'_> {
        builder::GetMeasureDownloadsAcrossReportingUnitsByMeasureDownloadCode::new(self)
    }
    /**Gets a dictionary, indexed by the measure download code, of the measure downloads available

Gets dictionary, indexed by the measure download code, of all available measure download codes and descriptions that can be used in the measure download API call.

Sends a `GET` request to `/api/v1/measure-downloads/measure-download-codes`

Arguments:
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_measure_downloads_measure_download_codes()
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_measure_downloads_measure_download_codes(
        &self,
    ) -> builder::GetMeasureDownloadsMeasureDownloadCodes<'_> {
        builder::GetMeasureDownloadsMeasureDownloadCodes::new(self)
    }
    /**Gets a measure download

Gets a measure download, in XLSX format, for the supplied measure download code.

Sends a `GET` request to `/api/v1/measure-downloads/{measure-download-code}`

Arguments:
- `measure_download_code`: The measure download code.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_measure_downloads_by_measure_download_code()
    .measure_download_code(measure_download_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_measure_downloads_by_measure_download_code(
        &self,
    ) -> builder::GetMeasureDownloadsByMeasureDownloadCode<'_> {
        builder::GetMeasureDownloadsByMeasureDownloadCode::new(self)
    }
    /**Gets a list of measures

Gets list of all measures, optionally filtered, using logical disjunction (OR), by a list of measure category codes.

Sends a `GET` request to `/api/v1/measures`

Arguments:
- `measure_category_code`: The measure category codes.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_measures()
    .measure_category_code(measure_category_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_measures(&self) -> builder::GetMeasures<'_> {
        builder::GetMeasures::new(self)
    }
    /**Gets a single measure

Gets a single measure matching the supplied measure code.

Sends a `GET` request to `/api/v1/measures/{measure-code}`

Arguments:
- `measure_code`: The measure code.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_measures_by_measure_code()
    .measure_code(measure_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_measures_by_measure_code(&self) -> builder::GetMeasuresByMeasureCode<'_> {
        builder::GetMeasuresByMeasureCode::new(self)
    }
    /**Gets a list of data items for a measure

Gets a list of all data items for the specified measure.

Sends a `GET` request to `/api/v1/measures/{measure-code}/data-items`

Arguments:
- `measure_code`: The measure code.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_measures_by_measure_code_data_items()
    .measure_code(measure_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_measures_by_measure_code_data_items(
        &self,
    ) -> builder::GetMeasuresByMeasureCodeDataItems<'_> {
        builder::GetMeasuresByMeasureCodeDataItems::new(self)
    }
    /**Gets a list reporting units available for a measure

Gets a list of all reporting units that have data available for the specified measure.

Sends a `GET` request to `/api/v1/measures/{measure-code}/reporting-units-available`

Arguments:
- `measure_code`: The measure code.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_measures_by_measure_code_reporting_units_available()
    .measure_code(measure_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_measures_by_measure_code_reporting_units_available(
        &self,
    ) -> builder::GetMeasuresByMeasureCodeReportingUnitsAvailable<'_> {
        builder::GetMeasuresByMeasureCodeReportingUnitsAvailable::new(self)
    }
    /**Gets a list of reported measure categories

Gets list of all reported measure categories available.

Sends a `GET` request to `/api/v1/reported-measure-categories`

Arguments:
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_reported_measure_categories()
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_reported_measure_categories(
        &self,
    ) -> builder::GetReportedMeasureCategories<'_> {
        builder::GetReportedMeasureCategories::new(self)
    }
    /**Gets a list of reported measure categories for the supplied reported measure category code

Gets a list reported measure categories matching the supplied reported measure category codes. \
                NOTE: Reported measure codes are not constrained to be unique in the system.

Sends a `GET` request to `/api/v1/reported-measure-categories/{reported-measure-category-code}`

Arguments:
- `reported_measure_category_code`: The reported measure category code.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_reported_measure_categories_by_reported_measure_category_code()
    .reported_measure_category_code(reported_measure_category_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_reported_measure_categories_by_reported_measure_category_code(
        &self,
    ) -> builder::GetReportedMeasureCategoriesByReportedMeasureCategoryCode<'_> {
        builder::GetReportedMeasureCategoriesByReportedMeasureCategoryCode::new(self)
    }
    /**Gets reported measures for a reported measure category

Gets a list of all reported measures for a specified reported measure category code.

Sends a `GET` request to `/api/v1/reported-measure-categories/{reported-measure-category-code}/reported-measures`

Arguments:
- `reported_measure_category_code`: The reported measure category code.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_reported_measure_categories_by_reported_measure_category_code_reported_measures()
    .reported_measure_category_code(reported_measure_category_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_reported_measure_categories_by_reported_measure_category_code_reported_measures(
        &self,
    ) -> builder::GetReportedMeasureCategoriesByReportedMeasureCategoryCodeReportedMeasures<
        '_,
    > {
        builder::GetReportedMeasureCategoriesByReportedMeasureCategoryCodeReportedMeasures::new(
            self,
        )
    }
    /**Gets a list of reported measures

Gets list of reported measures, optionally filtered by a list of measure and reported measure category codes. \
                Matching within each filter is by logical disjunction (OR) and matching across filters is by logical conjunction (AND).

Sends a `GET` request to `/api/v1/reported-measures`

Arguments:
- `measure_code`: The measure codes.
- `reported_measure_category_code`: The reported measure category codes.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_reported_measures()
    .measure_code(measure_code)
    .reported_measure_category_code(reported_measure_category_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_reported_measures(&self) -> builder::GetReportedMeasures<'_> {
        builder::GetReportedMeasures::new(self)
    }
    /**Gets a single reported measure

Gets a single reported measure matching the supplied reported measure code.

Sends a `GET` request to `/api/v1/reported-measures/{reported-measure-code}`

Arguments:
- `reported_measure_code`: The reported measure code.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_reported_measures_by_reported_measure_code()
    .reported_measure_code(reported_measure_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_reported_measures_by_reported_measure_code(
        &self,
    ) -> builder::GetReportedMeasuresByReportedMeasureCode<'_> {
        builder::GetReportedMeasuresByReportedMeasureCode::new(self)
    }
    /**Gets a list of data items for a reported measure

Gets a list of all data items for the specified reported measure.

Sends a `GET` request to `/api/v1/reported-measures/{reported-measure-code}/data-items`

Arguments:
- `reported_measure_code`: The reported measure code.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_reported_measures_by_reported_measure_code_data_items()
    .reported_measure_code(reported_measure_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_reported_measures_by_reported_measure_code_data_items(
        &self,
    ) -> builder::GetReportedMeasuresByReportedMeasureCodeDataItems<'_> {
        builder::GetReportedMeasuresByReportedMeasureCodeDataItems::new(self)
    }
    /**Gets a list of reporting unit types

Gets list of all reporting unit types.

Sends a `GET` request to `/api/v1/reporting-unit-types`

Arguments:
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_reporting_unit_types()
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_reporting_unit_types(&self) -> builder::GetReportingUnitTypes<'_> {
        builder::GetReportingUnitTypes::new(self)
    }
    /**Gets a single reporting unit type

Gets a single reporting unit type matching the supplied reporting unit type code.

Sends a `GET` request to `/api/v1/reporting-unit-types/{reporting-unit-type-code}`

Arguments:
- `reporting_unit_type_code`: The reporting unit type code.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_reporting_unit_types_by_reporting_unit_type_code()
    .reporting_unit_type_code(reporting_unit_type_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_reporting_unit_types_by_reporting_unit_type_code(
        &self,
    ) -> builder::GetReportingUnitTypesByReportingUnitTypeCode<'_> {
        builder::GetReportingUnitTypesByReportingUnitTypeCode::new(self)
    }
    /**Gets a dictionary of reporting unit codes and available bricks

Gets a dictionary of reporting units codes and their available bricks, for the specified reporting unit type.

Sends a `GET` request to `/api/v1/reporting-unit-types/{reporting-unit-type-code}/bricks-available`

Arguments:
- `reporting_unit_type_code`: The reporting unit type code.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_reporting_unit_types_by_reporting_unit_type_code_bricks_available()
    .reporting_unit_type_code(reporting_unit_type_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_reporting_unit_types_by_reporting_unit_type_code_bricks_available(
        &self,
    ) -> builder::GetReportingUnitTypesByReportingUnitTypeCodeBricksAvailable<'_> {
        builder::GetReportingUnitTypesByReportingUnitTypeCodeBricksAvailable::new(self)
    }
    /**Gets a list of reporting units

Gets list of all reporting units, optionally filtered, using logical disjunction (OR), by a list of reporting unit type codes.

Sends a `GET` request to `/api/v1/reporting-units`

Arguments:
- `reporting_unit_type_code`: The reporting unit type codes.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_reporting_units()
    .reporting_unit_type_code(reporting_unit_type_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_reporting_units(&self) -> builder::GetReportingUnits<'_> {
        builder::GetReportingUnits::new(self)
    }
    /**Gets a dictionary, indexed by the datasheet code, of the reporting unit datasheet codes available

Gets dictionary, indexed by the datasheet code, of all available reporting unit datasheet codes and descriptions that can be used in the reporting unit download API call.

Sends a `GET` request to `/api/v1/reporting-units-downloads/datasheet-codes`

Arguments:
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_reporting_units_downloads_datasheet_codes()
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_reporting_units_downloads_datasheet_codes(
        &self,
    ) -> builder::GetReportingUnitsDownloadsDatasheetCodes<'_> {
        builder::GetReportingUnitsDownloadsDatasheetCodes::new(self)
    }
    /**Gets the reporting unit mapping download

Gets the reporting unit mapping download, in XLSX format.

Sends a `GET` request to `/api/v1/reporting-units-downloads/mappings`

Arguments:
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_reporting_units_downloads_mappings()
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_reporting_units_downloads_mappings(
        &self,
    ) -> builder::GetReportingUnitsDownloadsMappings<'_> {
        builder::GetReportingUnitsDownloadsMappings::new(self)
    }
    /**Gets a reporting unit data download

Gets a reporting unit data download, in XLSX format, for the supplied reporting unit code and datasheet code.

Sends a `GET` request to `/api/v1/reporting-units-downloads/{datasheet-code}/{reporting-unit-code}`

Arguments:
- `datasheet_code`: The datasheet code.
- `reporting_unit_code`: The reporting unit code.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_reporting_units_downloads_by_datasheet_code_by_reporting_unit_code()
    .datasheet_code(datasheet_code)
    .reporting_unit_code(reporting_unit_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_reporting_units_downloads_by_datasheet_code_by_reporting_unit_code(
        &self,
    ) -> builder::GetReportingUnitsDownloadsByDatasheetCodeByReportingUnitCode<'_> {
        builder::GetReportingUnitsDownloadsByDatasheetCodeByReportingUnitCode::new(self)
    }
    /**Gets a single reporting unit

Gets a single reporting unit matching the supplied reporting unit code.

Sends a `GET` request to `/api/v1/reporting-units/{reporting-unit-code}`

Arguments:
- `reporting_unit_code`: The reporting unit code.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_reporting_units_by_reporting_unit_code()
    .reporting_unit_code(reporting_unit_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_reporting_units_by_reporting_unit_code(
        &self,
    ) -> builder::GetReportingUnitsByReportingUnitCode<'_> {
        builder::GetReportingUnitsByReportingUnitCode::new(self)
    }
    /**Gets a list brick codes available for a reporting unit

Gets a list of all brick codes available for the specified reporting unit.

Sends a `GET` request to `/api/v1/reporting-units/{reporting-unit-code}/bricks-available`

Arguments:
- `reporting_unit_code`: The reporting unit code.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_reporting_units_by_reporting_unit_code_bricks_available()
    .reporting_unit_code(reporting_unit_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_reporting_units_by_reporting_unit_code_bricks_available(
        &self,
    ) -> builder::GetReportingUnitsByReportingUnitCodeBricksAvailable<'_> {
        builder::GetReportingUnitsByReportingUnitCodeBricksAvailable::new(self)
    }
    /**Gets a list of data items for a reporting unit

Gets a list of all data items for the specified reporting unit.

Sends a `GET` request to `/api/v1/reporting-units/{reporting-unit-code}/data-items`

Arguments:
- `reporting_unit_code`: The reporting unit code.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_reporting_units_by_reporting_unit_code_data_items()
    .reporting_unit_code(reporting_unit_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_reporting_units_by_reporting_unit_code_data_items(
        &self,
    ) -> builder::GetReportingUnitsByReportingUnitCodeDataItems<'_> {
        builder::GetReportingUnitsByReportingUnitCodeDataItems::new(self)
    }
    /**Gets a list measures available for a reporting unit

Gets a list of all measures that have data available for the specified reporting unit.

Sends a `GET` request to `/api/v1/reporting-units/{reporting-unit-code}/measures-available`

Arguments:
- `reporting_unit_code`: The reporting unit code.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_reporting_units_by_reporting_unit_code_measures_available()
    .reporting_unit_code(reporting_unit_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_reporting_units_by_reporting_unit_code_measures_available(
        &self,
    ) -> builder::GetReportingUnitsByReportingUnitCodeMeasuresAvailable<'_> {
        builder::GetReportingUnitsByReportingUnitCodeMeasuresAvailable::new(self)
    }
    /**Gets a dictionary, indexed by the simple download code, of the simple downloads available

Gets dictionary, indexed by the simple download code, of all available simple download codes and descriptions that can be used in the simple download API call.

Sends a `GET` request to `/api/v1/simple-downloads/download-codes`

Arguments:
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_simple_downloads_download_codes()
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_simple_downloads_download_codes(
        &self,
    ) -> builder::GetSimpleDownloadsDownloadCodes<'_> {
        builder::GetSimpleDownloadsDownloadCodes::new(self)
    }
    /**Gets a simple download

Gets a simple download, in XLSX format, for the supplied download code.

Sends a `GET` request to `/api/v1/simple-downloads/{download-code}`

Arguments:
- `download_code`: The download code.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_simple_downloads_by_download_code()
    .download_code(download_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_simple_downloads_by_download_code(
        &self,
    ) -> builder::GetSimpleDownloadsByDownloadCode<'_> {
        builder::GetSimpleDownloadsByDownloadCode::new(self)
    }
    /**Gets a list of suppressions

Gets list of all suppressions.\
                The Suppressions API call is now obsolete and callers should switch to: /api/v1/caveats.

Sends a `GET` request to `/api/v1/suppressions`

Arguments:
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_suppressions()
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_suppressions(&self) -> builder::GetSuppressions<'_> {
        builder::GetSuppressions::new(self)
    }
    /**Gets a suppression

Gets a suppression matching the supplied suppression code.\
                The Suppressions API call is now obsolete and callers should switch to: /api/v1/caveats/{caveat-code}.

Sends a `GET` request to `/api/v1/suppressions/{suppression-code}`

Arguments:
- `suppression_code`: The suppression code.
- `x_swagger_ui`: For internal use
```ignore
let response = client.get_suppressions_by_suppression_code()
    .suppression_code(suppression_code)
    .x_swagger_ui(x_swagger_ui)
    .send()
    .await;
```*/
    pub fn get_suppressions_by_suppression_code(
        &self,
    ) -> builder::GetSuppressionsBySuppressionCode<'_> {
        builder::GetSuppressionsBySuppressionCode::new(self)
    }
}
/// Types for composing operation parameters.
#[allow(clippy::all)]
pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, ByteStream, ClientInfo, ClientHooks, Error, OperationInfo,
        RequestBuilderExt, ResponseValue,
    };
    /**Builder for [`Client::get_caveats`]

[`Client::get_caveats`]: super::Client::get_caveats*/
    #[derive(Debug, Clone)]
    pub struct GetCaveats<'a> {
        client: &'a super::Client,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetCaveats<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                x_swagger_ui: Ok(None),
            }
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/caveats`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetCaveatsResponse>, Error<()>> {
            let Self { client, x_swagger_ui } = self;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!("{}/api/v1/caveats", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_caveats",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_caveats_by_caveat_code`]

[`Client::get_caveats_by_caveat_code`]: super::Client::get_caveats_by_caveat_code*/
    #[derive(Debug, Clone)]
    pub struct GetCaveatsByCaveatCode<'a> {
        client: &'a super::Client,
        caveat_code: Result<::std::string::String, String>,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetCaveatsByCaveatCode<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                caveat_code: Err("caveat_code was not initialized".to_string()),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn caveat_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.caveat_code = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for caveat_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/caveats/{caveat-code}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetCaveatsByCaveatCodeResponse>, Error<()>> {
            let Self { client, caveat_code, x_swagger_ui } = self;
            let caveat_code = caveat_code.map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/caveats/{}", client.baseurl, encode_path(& caveat_code
                .to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_caveats_by_caveat_code",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_datasets`]

[`Client::get_datasets`]: super::Client::get_datasets*/
    #[derive(Debug, Clone)]
    pub struct GetDatasets<'a> {
        client: &'a super::Client,
        measure_code: Result<Option<::std::vec::Vec<::std::string::String>>, String>,
        reported_measure_code: Result<
            Option<::std::vec::Vec<::std::string::String>>,
            String,
        >,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetDatasets<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                measure_code: Ok(None),
                reported_measure_code: Ok(None),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn measure_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
        {
            self.measure_code = value
                .try_into()
                .map(Some)
                .map_err(|_| {
                    "conversion to `:: std :: vec :: Vec < :: std :: string :: String >` for measure_code failed"
                        .to_string()
                });
            self
        }
        pub fn reported_measure_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
        {
            self.reported_measure_code = value
                .try_into()
                .map(Some)
                .map_err(|_| {
                    "conversion to `:: std :: vec :: Vec < :: std :: string :: String >` for reported_measure_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/datasets`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetDatasetsResponse>, Error<()>> {
            let Self { client, measure_code, reported_measure_code, x_swagger_ui } = self;
            let measure_code = measure_code.map_err(Error::InvalidRequest)?;
            let reported_measure_code = reported_measure_code
                .map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!("{}/api/v1/datasets", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(
                    &progenitor_client::QueryParam::new("measure_code", &measure_code),
                )
                .query(
                    &progenitor_client::QueryParam::new(
                        "reported_measure_code",
                        &reported_measure_code,
                    ),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_datasets",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_datasets_by_dataset_id`]

[`Client::get_datasets_by_dataset_id`]: super::Client::get_datasets_by_dataset_id*/
    #[derive(Debug, Clone)]
    pub struct GetDatasetsByDatasetId<'a> {
        client: &'a super::Client,
        dataset_id: Result<i32, String>,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetDatasetsByDatasetId<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                dataset_id: Err("dataset_id was not initialized".to_string()),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn dataset_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i32>,
        {
            self.dataset_id = value
                .try_into()
                .map_err(|_| "conversion to `i32` for dataset_id failed".to_string());
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/datasets/{dataset-id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetDatasetsByDatasetIdResponse>, Error<()>> {
            let Self { client, dataset_id, x_swagger_ui } = self;
            let dataset_id = dataset_id.map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/datasets/{}", client.baseurl, encode_path(& dataset_id
                .to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_datasets_by_dataset_id",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_datasets_by_dataset_id_data_items`]

[`Client::get_datasets_by_dataset_id_data_items`]: super::Client::get_datasets_by_dataset_id_data_items*/
    #[derive(Debug, Clone)]
    pub struct GetDatasetsByDatasetIdDataItems<'a> {
        client: &'a super::Client,
        dataset_id: Result<i32, String>,
        reporting_unit_code: Result<
            Option<::std::vec::Vec<::std::string::String>>,
            String,
        >,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetDatasetsByDatasetIdDataItems<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                dataset_id: Err("dataset_id was not initialized".to_string()),
                reporting_unit_code: Ok(None),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn dataset_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i32>,
        {
            self.dataset_id = value
                .try_into()
                .map_err(|_| "conversion to `i32` for dataset_id failed".to_string());
            self
        }
        pub fn reporting_unit_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
        {
            self.reporting_unit_code = value
                .try_into()
                .map(Some)
                .map_err(|_| {
                    "conversion to `:: std :: vec :: Vec < :: std :: string :: String >` for reporting_unit_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/datasets/{dataset-id}/data-items`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::GetDatasetsByDatasetIdDataItemsResponse>,
            Error<()>,
        > {
            let Self { client, dataset_id, reporting_unit_code, x_swagger_ui } = self;
            let dataset_id = dataset_id.map_err(Error::InvalidRequest)?;
            let reporting_unit_code = reporting_unit_code
                .map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/datasets/{}/data-items", client.baseurl, encode_path(&
                dataset_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(
                    &progenitor_client::QueryParam::new(
                        "reporting_unit_code",
                        &reporting_unit_code,
                    ),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_datasets_by_dataset_id_data_items",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_flat_data_extract_by_measure_category_code`]

[`Client::get_flat_data_extract_by_measure_category_code`]: super::Client::get_flat_data_extract_by_measure_category_code*/
    #[derive(Debug, Clone)]
    pub struct GetFlatDataExtractByMeasureCategoryCode<'a> {
        client: &'a super::Client,
        measure_category_code: Result<::std::string::String, String>,
        end_date: Result<Option<::std::string::String>, String>,
        measure_code: Result<Option<::std::vec::Vec<::std::string::String>>, String>,
        reporting_unit_code: Result<
            Option<::std::vec::Vec<::std::string::String>>,
            String,
        >,
        reporting_unit_type_code: Result<
            Option<::std::vec::Vec<::std::string::String>>,
            String,
        >,
        skip: Result<i32, String>,
        start_date: Result<Option<::std::string::String>, String>,
        top: Result<::std::num::NonZeroU32, String>,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetFlatDataExtractByMeasureCategoryCode<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                measure_category_code: Err(
                    "measure_category_code was not initialized".to_string(),
                ),
                end_date: Ok(None),
                measure_code: Ok(None),
                reporting_unit_code: Ok(None),
                reporting_unit_type_code: Ok(None),
                skip: Err("skip was not initialized".to_string()),
                start_date: Ok(None),
                top: Err("top was not initialized".to_string()),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn measure_category_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.measure_category_code = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for measure_category_code failed"
                        .to_string()
                });
            self
        }
        pub fn end_date<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.end_date = value
                .try_into()
                .map(Some)
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for end_date failed"
                        .to_string()
                });
            self
        }
        pub fn measure_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
        {
            self.measure_code = value
                .try_into()
                .map(Some)
                .map_err(|_| {
                    "conversion to `:: std :: vec :: Vec < :: std :: string :: String >` for measure_code failed"
                        .to_string()
                });
            self
        }
        pub fn reporting_unit_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
        {
            self.reporting_unit_code = value
                .try_into()
                .map(Some)
                .map_err(|_| {
                    "conversion to `:: std :: vec :: Vec < :: std :: string :: String >` for reporting_unit_code failed"
                        .to_string()
                });
            self
        }
        pub fn reporting_unit_type_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
        {
            self.reporting_unit_type_code = value
                .try_into()
                .map(Some)
                .map_err(|_| {
                    "conversion to `:: std :: vec :: Vec < :: std :: string :: String >` for reporting_unit_type_code failed"
                        .to_string()
                });
            self
        }
        pub fn skip<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i32>,
        {
            self.skip = value
                .try_into()
                .map_err(|_| "conversion to `i32` for skip failed".to_string());
            self
        }
        pub fn start_date<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.start_date = value
                .try_into()
                .map(Some)
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for start_date failed"
                        .to_string()
                });
            self
        }
        pub fn top<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::num::NonZeroU32>,
        {
            self.top = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: num :: NonZeroU32` for top failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/flat-data-extract/{measure-category-code}`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::GetFlatDataExtractByMeasureCategoryCodeResponse>,
            Error<()>,
        > {
            let Self {
                client,
                measure_category_code,
                end_date,
                measure_code,
                reporting_unit_code,
                reporting_unit_type_code,
                skip,
                start_date,
                top,
                x_swagger_ui,
            } = self;
            let measure_category_code = measure_category_code
                .map_err(Error::InvalidRequest)?;
            let end_date = end_date.map_err(Error::InvalidRequest)?;
            let measure_code = measure_code.map_err(Error::InvalidRequest)?;
            let reporting_unit_code = reporting_unit_code
                .map_err(Error::InvalidRequest)?;
            let reporting_unit_type_code = reporting_unit_type_code
                .map_err(Error::InvalidRequest)?;
            let skip = skip.map_err(Error::InvalidRequest)?;
            let start_date = start_date.map_err(Error::InvalidRequest)?;
            let top = top.map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/flat-data-extract/{}", client.baseurl, encode_path(&
                measure_category_code.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("end_date", &end_date))
                .query(
                    &progenitor_client::QueryParam::new("measure_code", &measure_code),
                )
                .query(
                    &progenitor_client::QueryParam::new(
                        "reporting_unit_code",
                        &reporting_unit_code,
                    ),
                )
                .query(
                    &progenitor_client::QueryParam::new(
                        "reporting_unit_type_code",
                        &reporting_unit_type_code,
                    ),
                )
                .query(&progenitor_client::QueryParam::new("skip", &skip))
                .query(&progenitor_client::QueryParam::new("start_date", &start_date))
                .query(&progenitor_client::QueryParam::new("top", &top))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_flat_data_extract_by_measure_category_code",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_flat_formatted_data_extract_by_measure_category_code`]

[`Client::get_flat_formatted_data_extract_by_measure_category_code`]: super::Client::get_flat_formatted_data_extract_by_measure_category_code*/
    #[derive(Debug, Clone)]
    pub struct GetFlatFormattedDataExtractByMeasureCategoryCode<'a> {
        client: &'a super::Client,
        measure_category_code: Result<::std::string::String, String>,
        end_date: Result<Option<::std::string::String>, String>,
        measure_code: Result<Option<::std::vec::Vec<::std::string::String>>, String>,
        reporting_unit_code: Result<
            Option<::std::vec::Vec<::std::string::String>>,
            String,
        >,
        reporting_unit_type_code: Result<
            Option<::std::vec::Vec<::std::string::String>>,
            String,
        >,
        skip: Result<i32, String>,
        start_date: Result<Option<::std::string::String>, String>,
        top: Result<::std::num::NonZeroU32, String>,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetFlatFormattedDataExtractByMeasureCategoryCode<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                measure_category_code: Err(
                    "measure_category_code was not initialized".to_string(),
                ),
                end_date: Ok(None),
                measure_code: Ok(None),
                reporting_unit_code: Ok(None),
                reporting_unit_type_code: Ok(None),
                skip: Err("skip was not initialized".to_string()),
                start_date: Ok(None),
                top: Err("top was not initialized".to_string()),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn measure_category_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.measure_category_code = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for measure_category_code failed"
                        .to_string()
                });
            self
        }
        pub fn end_date<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.end_date = value
                .try_into()
                .map(Some)
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for end_date failed"
                        .to_string()
                });
            self
        }
        pub fn measure_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
        {
            self.measure_code = value
                .try_into()
                .map(Some)
                .map_err(|_| {
                    "conversion to `:: std :: vec :: Vec < :: std :: string :: String >` for measure_code failed"
                        .to_string()
                });
            self
        }
        pub fn reporting_unit_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
        {
            self.reporting_unit_code = value
                .try_into()
                .map(Some)
                .map_err(|_| {
                    "conversion to `:: std :: vec :: Vec < :: std :: string :: String >` for reporting_unit_code failed"
                        .to_string()
                });
            self
        }
        pub fn reporting_unit_type_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
        {
            self.reporting_unit_type_code = value
                .try_into()
                .map(Some)
                .map_err(|_| {
                    "conversion to `:: std :: vec :: Vec < :: std :: string :: String >` for reporting_unit_type_code failed"
                        .to_string()
                });
            self
        }
        pub fn skip<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i32>,
        {
            self.skip = value
                .try_into()
                .map_err(|_| "conversion to `i32` for skip failed".to_string());
            self
        }
        pub fn start_date<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.start_date = value
                .try_into()
                .map(Some)
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for start_date failed"
                        .to_string()
                });
            self
        }
        pub fn top<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::num::NonZeroU32>,
        {
            self.top = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: num :: NonZeroU32` for top failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/flat-formatted-data-extract/{measure-category-code}`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<
                types::GetFlatFormattedDataExtractByMeasureCategoryCodeResponse,
            >,
            Error<()>,
        > {
            let Self {
                client,
                measure_category_code,
                end_date,
                measure_code,
                reporting_unit_code,
                reporting_unit_type_code,
                skip,
                start_date,
                top,
                x_swagger_ui,
            } = self;
            let measure_category_code = measure_category_code
                .map_err(Error::InvalidRequest)?;
            let end_date = end_date.map_err(Error::InvalidRequest)?;
            let measure_code = measure_code.map_err(Error::InvalidRequest)?;
            let reporting_unit_code = reporting_unit_code
                .map_err(Error::InvalidRequest)?;
            let reporting_unit_type_code = reporting_unit_type_code
                .map_err(Error::InvalidRequest)?;
            let skip = skip.map_err(Error::InvalidRequest)?;
            let start_date = start_date.map_err(Error::InvalidRequest)?;
            let top = top.map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/flat-formatted-data-extract/{}", client.baseurl, encode_path(&
                measure_category_code.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("end_date", &end_date))
                .query(
                    &progenitor_client::QueryParam::new("measure_code", &measure_code),
                )
                .query(
                    &progenitor_client::QueryParam::new(
                        "reporting_unit_code",
                        &reporting_unit_code,
                    ),
                )
                .query(
                    &progenitor_client::QueryParam::new(
                        "reporting_unit_type_code",
                        &reporting_unit_type_code,
                    ),
                )
                .query(&progenitor_client::QueryParam::new("skip", &skip))
                .query(&progenitor_client::QueryParam::new("start_date", &start_date))
                .query(&progenitor_client::QueryParam::new("top", &top))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_flat_formatted_data_extract_by_measure_category_code",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_measure_categories`]

[`Client::get_measure_categories`]: super::Client::get_measure_categories*/
    #[derive(Debug, Clone)]
    pub struct GetMeasureCategories<'a> {
        client: &'a super::Client,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetMeasureCategories<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                x_swagger_ui: Ok(None),
            }
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/measure-categories`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetMeasureCategoriesResponse>, Error<()>> {
            let Self { client, x_swagger_ui } = self;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!("{}/api/v1/measure-categories", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_measure_categories",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_measure_categories_by_measure_category_code`]

[`Client::get_measure_categories_by_measure_category_code`]: super::Client::get_measure_categories_by_measure_category_code*/
    #[derive(Debug, Clone)]
    pub struct GetMeasureCategoriesByMeasureCategoryCode<'a> {
        client: &'a super::Client,
        measure_category_code: Result<::std::string::String, String>,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetMeasureCategoriesByMeasureCategoryCode<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                measure_category_code: Err(
                    "measure_category_code was not initialized".to_string(),
                ),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn measure_category_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.measure_category_code = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for measure_category_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/measure-categories/{measure-category-code}`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::GetMeasureCategoriesByMeasureCategoryCodeResponse>,
            Error<()>,
        > {
            let Self { client, measure_category_code, x_swagger_ui } = self;
            let measure_category_code = measure_category_code
                .map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/measure-categories/{}", client.baseurl, encode_path(&
                measure_category_code.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_measure_categories_by_measure_category_code",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_measure_categories_by_measure_category_code_measures`]

[`Client::get_measure_categories_by_measure_category_code_measures`]: super::Client::get_measure_categories_by_measure_category_code_measures*/
    #[derive(Debug, Clone)]
    pub struct GetMeasureCategoriesByMeasureCategoryCodeMeasures<'a> {
        client: &'a super::Client,
        measure_category_code: Result<::std::string::String, String>,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetMeasureCategoriesByMeasureCategoryCodeMeasures<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                measure_category_code: Err(
                    "measure_category_code was not initialized".to_string(),
                ),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn measure_category_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.measure_category_code = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for measure_category_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/measure-categories/{measure-category-code}/measures`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<
                types::GetMeasureCategoriesByMeasureCategoryCodeMeasuresResponse,
            >,
            Error<()>,
        > {
            let Self { client, measure_category_code, x_swagger_ui } = self;
            let measure_category_code = measure_category_code
                .map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/measure-categories/{}/measures", client.baseurl, encode_path(&
                measure_category_code.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_measure_categories_by_measure_category_code_measures",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_measure_downloads_across_reporting_units_by_measure_download_code`]

[`Client::get_measure_downloads_across_reporting_units_by_measure_download_code`]: super::Client::get_measure_downloads_across_reporting_units_by_measure_download_code*/
    #[derive(Debug, Clone)]
    pub struct GetMeasureDownloadsAcrossReportingUnitsByMeasureDownloadCode<'a> {
        client: &'a super::Client,
        measure_download_code: Result<::std::string::String, String>,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetMeasureDownloadsAcrossReportingUnitsByMeasureDownloadCode<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                measure_download_code: Err(
                    "measure_download_code was not initialized".to_string(),
                ),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn measure_download_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.measure_download_code = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for measure_download_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/measure-downloads/across-reporting-units/{measure-download-code}`
        pub async fn send(self) -> Result<ResponseValue<ByteStream>, Error<()>> {
            let Self { client, measure_download_code, x_swagger_ui } = self;
            let measure_download_code = measure_download_code
                .map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/measure-downloads/across-reporting-units/{}", client.baseurl,
                encode_path(& measure_download_code.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client.client.get(url).headers(header_map).build()?;
            let info = OperationInfo {
                operation_id: "get_measure_downloads_across_reporting_units_by_measure_download_code",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => Ok(ResponseValue::stream(response)),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_measure_downloads_measure_download_codes`]

[`Client::get_measure_downloads_measure_download_codes`]: super::Client::get_measure_downloads_measure_download_codes*/
    #[derive(Debug, Clone)]
    pub struct GetMeasureDownloadsMeasureDownloadCodes<'a> {
        client: &'a super::Client,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetMeasureDownloadsMeasureDownloadCodes<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                x_swagger_ui: Ok(None),
            }
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/measure-downloads/measure-download-codes`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::GetMeasureDownloadsMeasureDownloadCodesResponse>,
            Error<()>,
        > {
            let Self { client, x_swagger_ui } = self;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/measure-downloads/measure-download-codes", client.baseurl,
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_measure_downloads_measure_download_codes",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_measure_downloads_by_measure_download_code`]

[`Client::get_measure_downloads_by_measure_download_code`]: super::Client::get_measure_downloads_by_measure_download_code*/
    #[derive(Debug, Clone)]
    pub struct GetMeasureDownloadsByMeasureDownloadCode<'a> {
        client: &'a super::Client,
        measure_download_code: Result<::std::string::String, String>,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetMeasureDownloadsByMeasureDownloadCode<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                measure_download_code: Err(
                    "measure_download_code was not initialized".to_string(),
                ),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn measure_download_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.measure_download_code = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for measure_download_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/measure-downloads/{measure-download-code}`
        pub async fn send(self) -> Result<ResponseValue<ByteStream>, Error<()>> {
            let Self { client, measure_download_code, x_swagger_ui } = self;
            let measure_download_code = measure_download_code
                .map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/measure-downloads/{}", client.baseurl, encode_path(&
                measure_download_code.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client.client.get(url).headers(header_map).build()?;
            let info = OperationInfo {
                operation_id: "get_measure_downloads_by_measure_download_code",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => Ok(ResponseValue::stream(response)),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_measures`]

[`Client::get_measures`]: super::Client::get_measures*/
    #[derive(Debug, Clone)]
    pub struct GetMeasures<'a> {
        client: &'a super::Client,
        measure_category_code: Result<
            Option<::std::vec::Vec<::std::string::String>>,
            String,
        >,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetMeasures<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                measure_category_code: Ok(None),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn measure_category_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
        {
            self.measure_category_code = value
                .try_into()
                .map(Some)
                .map_err(|_| {
                    "conversion to `:: std :: vec :: Vec < :: std :: string :: String >` for measure_category_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/measures`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetMeasuresResponse>, Error<()>> {
            let Self { client, measure_category_code, x_swagger_ui } = self;
            let measure_category_code = measure_category_code
                .map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!("{}/api/v1/measures", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(
                    &progenitor_client::QueryParam::new(
                        "measure_category_code",
                        &measure_category_code,
                    ),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_measures",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_measures_by_measure_code`]

[`Client::get_measures_by_measure_code`]: super::Client::get_measures_by_measure_code*/
    #[derive(Debug, Clone)]
    pub struct GetMeasuresByMeasureCode<'a> {
        client: &'a super::Client,
        measure_code: Result<::std::string::String, String>,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetMeasuresByMeasureCode<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                measure_code: Err("measure_code was not initialized".to_string()),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn measure_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.measure_code = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for measure_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/measures/{measure-code}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetMeasuresByMeasureCodeResponse>, Error<()>> {
            let Self { client, measure_code, x_swagger_ui } = self;
            let measure_code = measure_code.map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/measures/{}", client.baseurl, encode_path(& measure_code
                .to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_measures_by_measure_code",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_measures_by_measure_code_data_items`]

[`Client::get_measures_by_measure_code_data_items`]: super::Client::get_measures_by_measure_code_data_items*/
    #[derive(Debug, Clone)]
    pub struct GetMeasuresByMeasureCodeDataItems<'a> {
        client: &'a super::Client,
        measure_code: Result<::std::string::String, String>,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetMeasuresByMeasureCodeDataItems<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                measure_code: Err("measure_code was not initialized".to_string()),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn measure_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.measure_code = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for measure_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/measures/{measure-code}/data-items`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::GetMeasuresByMeasureCodeDataItemsResponse>,
            Error<()>,
        > {
            let Self { client, measure_code, x_swagger_ui } = self;
            let measure_code = measure_code.map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/measures/{}/data-items", client.baseurl, encode_path(&
                measure_code.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_measures_by_measure_code_data_items",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_measures_by_measure_code_reporting_units_available`]

[`Client::get_measures_by_measure_code_reporting_units_available`]: super::Client::get_measures_by_measure_code_reporting_units_available*/
    #[derive(Debug, Clone)]
    pub struct GetMeasuresByMeasureCodeReportingUnitsAvailable<'a> {
        client: &'a super::Client,
        measure_code: Result<::std::string::String, String>,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetMeasuresByMeasureCodeReportingUnitsAvailable<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                measure_code: Err("measure_code was not initialized".to_string()),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn measure_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.measure_code = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for measure_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/measures/{measure-code}/reporting-units-available`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<
                types::GetMeasuresByMeasureCodeReportingUnitsAvailableResponse,
            >,
            Error<()>,
        > {
            let Self { client, measure_code, x_swagger_ui } = self;
            let measure_code = measure_code.map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/measures/{}/reporting-units-available", client.baseurl,
                encode_path(& measure_code.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_measures_by_measure_code_reporting_units_available",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_reported_measure_categories`]

[`Client::get_reported_measure_categories`]: super::Client::get_reported_measure_categories*/
    #[derive(Debug, Clone)]
    pub struct GetReportedMeasureCategories<'a> {
        client: &'a super::Client,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetReportedMeasureCategories<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                x_swagger_ui: Ok(None),
            }
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/reported-measure-categories`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::GetReportedMeasureCategoriesResponse>,
            Error<()>,
        > {
            let Self { client, x_swagger_ui } = self;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!("{}/api/v1/reported-measure-categories", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_reported_measure_categories",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_reported_measure_categories_by_reported_measure_category_code`]

[`Client::get_reported_measure_categories_by_reported_measure_category_code`]: super::Client::get_reported_measure_categories_by_reported_measure_category_code*/
    #[derive(Debug, Clone)]
    pub struct GetReportedMeasureCategoriesByReportedMeasureCategoryCode<'a> {
        client: &'a super::Client,
        reported_measure_category_code: Result<::std::string::String, String>,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetReportedMeasureCategoriesByReportedMeasureCategoryCode<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                reported_measure_category_code: Err(
                    "reported_measure_category_code was not initialized".to_string(),
                ),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn reported_measure_category_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.reported_measure_category_code = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for reported_measure_category_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/reported-measure-categories/{reported-measure-category-code}`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<
                types::GetReportedMeasureCategoriesByReportedMeasureCategoryCodeResponse,
            >,
            Error<()>,
        > {
            let Self { client, reported_measure_category_code, x_swagger_ui } = self;
            let reported_measure_category_code = reported_measure_category_code
                .map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/reported-measure-categories/{}", client.baseurl, encode_path(&
                reported_measure_category_code.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_reported_measure_categories_by_reported_measure_category_code",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_reported_measure_categories_by_reported_measure_category_code_reported_measures`]

[`Client::get_reported_measure_categories_by_reported_measure_category_code_reported_measures`]: super::Client::get_reported_measure_categories_by_reported_measure_category_code_reported_measures*/
    #[derive(Debug, Clone)]
    pub struct GetReportedMeasureCategoriesByReportedMeasureCategoryCodeReportedMeasures<
        'a,
    > {
        client: &'a super::Client,
        reported_measure_category_code: Result<::std::string::String, String>,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<
        'a,
    > GetReportedMeasureCategoriesByReportedMeasureCategoryCodeReportedMeasures<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                reported_measure_category_code: Err(
                    "reported_measure_category_code was not initialized".to_string(),
                ),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn reported_measure_category_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.reported_measure_category_code = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for reported_measure_category_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/reported-measure-categories/{reported-measure-category-code}/reported-measures`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<
                types::GetReportedMeasureCategoriesByReportedMeasureCategoryCodeReportedMeasuresResponse,
            >,
            Error<()>,
        > {
            let Self { client, reported_measure_category_code, x_swagger_ui } = self;
            let reported_measure_category_code = reported_measure_category_code
                .map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/reported-measure-categories/{}/reported-measures", client
                .baseurl, encode_path(& reported_measure_category_code.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_reported_measure_categories_by_reported_measure_category_code_reported_measures",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_reported_measures`]

[`Client::get_reported_measures`]: super::Client::get_reported_measures*/
    #[derive(Debug, Clone)]
    pub struct GetReportedMeasures<'a> {
        client: &'a super::Client,
        measure_code: Result<Option<::std::vec::Vec<::std::string::String>>, String>,
        reported_measure_category_code: Result<
            Option<::std::vec::Vec<::std::string::String>>,
            String,
        >,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetReportedMeasures<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                measure_code: Ok(None),
                reported_measure_category_code: Ok(None),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn measure_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
        {
            self.measure_code = value
                .try_into()
                .map(Some)
                .map_err(|_| {
                    "conversion to `:: std :: vec :: Vec < :: std :: string :: String >` for measure_code failed"
                        .to_string()
                });
            self
        }
        pub fn reported_measure_category_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
        {
            self.reported_measure_category_code = value
                .try_into()
                .map(Some)
                .map_err(|_| {
                    "conversion to `:: std :: vec :: Vec < :: std :: string :: String >` for reported_measure_category_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/reported-measures`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetReportedMeasuresResponse>, Error<()>> {
            let Self {
                client,
                measure_code,
                reported_measure_category_code,
                x_swagger_ui,
            } = self;
            let measure_code = measure_code.map_err(Error::InvalidRequest)?;
            let reported_measure_category_code = reported_measure_category_code
                .map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!("{}/api/v1/reported-measures", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(
                    &progenitor_client::QueryParam::new("measure_code", &measure_code),
                )
                .query(
                    &progenitor_client::QueryParam::new(
                        "reported_measure_category_code",
                        &reported_measure_category_code,
                    ),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_reported_measures",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_reported_measures_by_reported_measure_code`]

[`Client::get_reported_measures_by_reported_measure_code`]: super::Client::get_reported_measures_by_reported_measure_code*/
    #[derive(Debug, Clone)]
    pub struct GetReportedMeasuresByReportedMeasureCode<'a> {
        client: &'a super::Client,
        reported_measure_code: Result<::std::string::String, String>,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetReportedMeasuresByReportedMeasureCode<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                reported_measure_code: Err(
                    "reported_measure_code was not initialized".to_string(),
                ),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn reported_measure_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.reported_measure_code = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for reported_measure_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/reported-measures/{reported-measure-code}`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::GetReportedMeasuresByReportedMeasureCodeResponse>,
            Error<()>,
        > {
            let Self { client, reported_measure_code, x_swagger_ui } = self;
            let reported_measure_code = reported_measure_code
                .map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/reported-measures/{}", client.baseurl, encode_path(&
                reported_measure_code.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_reported_measures_by_reported_measure_code",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_reported_measures_by_reported_measure_code_data_items`]

[`Client::get_reported_measures_by_reported_measure_code_data_items`]: super::Client::get_reported_measures_by_reported_measure_code_data_items*/
    #[derive(Debug, Clone)]
    pub struct GetReportedMeasuresByReportedMeasureCodeDataItems<'a> {
        client: &'a super::Client,
        reported_measure_code: Result<::std::string::String, String>,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetReportedMeasuresByReportedMeasureCodeDataItems<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                reported_measure_code: Err(
                    "reported_measure_code was not initialized".to_string(),
                ),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn reported_measure_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.reported_measure_code = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for reported_measure_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/reported-measures/{reported-measure-code}/data-items`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<
                types::GetReportedMeasuresByReportedMeasureCodeDataItemsResponse,
            >,
            Error<()>,
        > {
            let Self { client, reported_measure_code, x_swagger_ui } = self;
            let reported_measure_code = reported_measure_code
                .map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/reported-measures/{}/data-items", client.baseurl,
                encode_path(& reported_measure_code.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_reported_measures_by_reported_measure_code_data_items",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_reporting_unit_types`]

[`Client::get_reporting_unit_types`]: super::Client::get_reporting_unit_types*/
    #[derive(Debug, Clone)]
    pub struct GetReportingUnitTypes<'a> {
        client: &'a super::Client,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetReportingUnitTypes<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                x_swagger_ui: Ok(None),
            }
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/reporting-unit-types`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetReportingUnitTypesResponse>, Error<()>> {
            let Self { client, x_swagger_ui } = self;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!("{}/api/v1/reporting-unit-types", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_reporting_unit_types",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_reporting_unit_types_by_reporting_unit_type_code`]

[`Client::get_reporting_unit_types_by_reporting_unit_type_code`]: super::Client::get_reporting_unit_types_by_reporting_unit_type_code*/
    #[derive(Debug, Clone)]
    pub struct GetReportingUnitTypesByReportingUnitTypeCode<'a> {
        client: &'a super::Client,
        reporting_unit_type_code: Result<::std::string::String, String>,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetReportingUnitTypesByReportingUnitTypeCode<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                reporting_unit_type_code: Err(
                    "reporting_unit_type_code was not initialized".to_string(),
                ),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn reporting_unit_type_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.reporting_unit_type_code = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for reporting_unit_type_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/reporting-unit-types/{reporting-unit-type-code}`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::GetReportingUnitTypesByReportingUnitTypeCodeResponse>,
            Error<()>,
        > {
            let Self { client, reporting_unit_type_code, x_swagger_ui } = self;
            let reporting_unit_type_code = reporting_unit_type_code
                .map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/reporting-unit-types/{}", client.baseurl, encode_path(&
                reporting_unit_type_code.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_reporting_unit_types_by_reporting_unit_type_code",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_reporting_unit_types_by_reporting_unit_type_code_bricks_available`]

[`Client::get_reporting_unit_types_by_reporting_unit_type_code_bricks_available`]: super::Client::get_reporting_unit_types_by_reporting_unit_type_code_bricks_available*/
    #[derive(Debug, Clone)]
    pub struct GetReportingUnitTypesByReportingUnitTypeCodeBricksAvailable<'a> {
        client: &'a super::Client,
        reporting_unit_type_code: Result<::std::string::String, String>,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetReportingUnitTypesByReportingUnitTypeCodeBricksAvailable<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                reporting_unit_type_code: Err(
                    "reporting_unit_type_code was not initialized".to_string(),
                ),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn reporting_unit_type_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.reporting_unit_type_code = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for reporting_unit_type_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/reporting-unit-types/{reporting-unit-type-code}/bricks-available`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<
                types::GetReportingUnitTypesByReportingUnitTypeCodeBricksAvailableResponse,
            >,
            Error<()>,
        > {
            let Self { client, reporting_unit_type_code, x_swagger_ui } = self;
            let reporting_unit_type_code = reporting_unit_type_code
                .map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/reporting-unit-types/{}/bricks-available", client.baseurl,
                encode_path(& reporting_unit_type_code.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_reporting_unit_types_by_reporting_unit_type_code_bricks_available",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_reporting_units`]

[`Client::get_reporting_units`]: super::Client::get_reporting_units*/
    #[derive(Debug, Clone)]
    pub struct GetReportingUnits<'a> {
        client: &'a super::Client,
        reporting_unit_type_code: Result<
            Option<::std::vec::Vec<::std::string::String>>,
            String,
        >,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetReportingUnits<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                reporting_unit_type_code: Ok(None),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn reporting_unit_type_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
        {
            self.reporting_unit_type_code = value
                .try_into()
                .map(Some)
                .map_err(|_| {
                    "conversion to `:: std :: vec :: Vec < :: std :: string :: String >` for reporting_unit_type_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/reporting-units`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetReportingUnitsResponse>, Error<()>> {
            let Self { client, reporting_unit_type_code, x_swagger_ui } = self;
            let reporting_unit_type_code = reporting_unit_type_code
                .map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!("{}/api/v1/reporting-units", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(
                    &progenitor_client::QueryParam::new(
                        "reporting_unit_type_code",
                        &reporting_unit_type_code,
                    ),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_reporting_units",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_reporting_units_downloads_datasheet_codes`]

[`Client::get_reporting_units_downloads_datasheet_codes`]: super::Client::get_reporting_units_downloads_datasheet_codes*/
    #[derive(Debug, Clone)]
    pub struct GetReportingUnitsDownloadsDatasheetCodes<'a> {
        client: &'a super::Client,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetReportingUnitsDownloadsDatasheetCodes<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                x_swagger_ui: Ok(None),
            }
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/reporting-units-downloads/datasheet-codes`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::GetReportingUnitsDownloadsDatasheetCodesResponse>,
            Error<()>,
        > {
            let Self { client, x_swagger_ui } = self;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/reporting-units-downloads/datasheet-codes", client.baseurl,
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_reporting_units_downloads_datasheet_codes",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_reporting_units_downloads_mappings`]

[`Client::get_reporting_units_downloads_mappings`]: super::Client::get_reporting_units_downloads_mappings*/
    #[derive(Debug, Clone)]
    pub struct GetReportingUnitsDownloadsMappings<'a> {
        client: &'a super::Client,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetReportingUnitsDownloadsMappings<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                x_swagger_ui: Ok(None),
            }
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/reporting-units-downloads/mappings`
        pub async fn send(self) -> Result<ResponseValue<ByteStream>, Error<()>> {
            let Self { client, x_swagger_ui } = self;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/reporting-units-downloads/mappings", client.baseurl,
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client.client.get(url).headers(header_map).build()?;
            let info = OperationInfo {
                operation_id: "get_reporting_units_downloads_mappings",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => Ok(ResponseValue::stream(response)),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_reporting_units_downloads_by_datasheet_code_by_reporting_unit_code`]

[`Client::get_reporting_units_downloads_by_datasheet_code_by_reporting_unit_code`]: super::Client::get_reporting_units_downloads_by_datasheet_code_by_reporting_unit_code*/
    #[derive(Debug, Clone)]
    pub struct GetReportingUnitsDownloadsByDatasheetCodeByReportingUnitCode<'a> {
        client: &'a super::Client,
        datasheet_code: Result<::std::string::String, String>,
        reporting_unit_code: Result<::std::string::String, String>,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetReportingUnitsDownloadsByDatasheetCodeByReportingUnitCode<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                datasheet_code: Err("datasheet_code was not initialized".to_string()),
                reporting_unit_code: Err(
                    "reporting_unit_code was not initialized".to_string(),
                ),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn datasheet_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.datasheet_code = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for datasheet_code failed"
                        .to_string()
                });
            self
        }
        pub fn reporting_unit_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.reporting_unit_code = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for reporting_unit_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/reporting-units-downloads/{datasheet-code}/{reporting-unit-code}`
        pub async fn send(self) -> Result<ResponseValue<ByteStream>, Error<()>> {
            let Self { client, datasheet_code, reporting_unit_code, x_swagger_ui } = self;
            let datasheet_code = datasheet_code.map_err(Error::InvalidRequest)?;
            let reporting_unit_code = reporting_unit_code
                .map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/reporting-units-downloads/{}/{}", client.baseurl,
                encode_path(& datasheet_code.to_string()), encode_path(&
                reporting_unit_code.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client.client.get(url).headers(header_map).build()?;
            let info = OperationInfo {
                operation_id: "get_reporting_units_downloads_by_datasheet_code_by_reporting_unit_code",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => Ok(ResponseValue::stream(response)),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_reporting_units_by_reporting_unit_code`]

[`Client::get_reporting_units_by_reporting_unit_code`]: super::Client::get_reporting_units_by_reporting_unit_code*/
    #[derive(Debug, Clone)]
    pub struct GetReportingUnitsByReportingUnitCode<'a> {
        client: &'a super::Client,
        reporting_unit_code: Result<::std::string::String, String>,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetReportingUnitsByReportingUnitCode<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                reporting_unit_code: Err(
                    "reporting_unit_code was not initialized".to_string(),
                ),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn reporting_unit_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.reporting_unit_code = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for reporting_unit_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/reporting-units/{reporting-unit-code}`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::GetReportingUnitsByReportingUnitCodeResponse>,
            Error<()>,
        > {
            let Self { client, reporting_unit_code, x_swagger_ui } = self;
            let reporting_unit_code = reporting_unit_code
                .map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/reporting-units/{}", client.baseurl, encode_path(&
                reporting_unit_code.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_reporting_units_by_reporting_unit_code",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_reporting_units_by_reporting_unit_code_bricks_available`]

[`Client::get_reporting_units_by_reporting_unit_code_bricks_available`]: super::Client::get_reporting_units_by_reporting_unit_code_bricks_available*/
    #[derive(Debug, Clone)]
    pub struct GetReportingUnitsByReportingUnitCodeBricksAvailable<'a> {
        client: &'a super::Client,
        reporting_unit_code: Result<::std::string::String, String>,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetReportingUnitsByReportingUnitCodeBricksAvailable<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                reporting_unit_code: Err(
                    "reporting_unit_code was not initialized".to_string(),
                ),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn reporting_unit_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.reporting_unit_code = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for reporting_unit_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/reporting-units/{reporting-unit-code}/bricks-available`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<
                types::GetReportingUnitsByReportingUnitCodeBricksAvailableResponse,
            >,
            Error<()>,
        > {
            let Self { client, reporting_unit_code, x_swagger_ui } = self;
            let reporting_unit_code = reporting_unit_code
                .map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/reporting-units/{}/bricks-available", client.baseurl,
                encode_path(& reporting_unit_code.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_reporting_units_by_reporting_unit_code_bricks_available",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_reporting_units_by_reporting_unit_code_data_items`]

[`Client::get_reporting_units_by_reporting_unit_code_data_items`]: super::Client::get_reporting_units_by_reporting_unit_code_data_items*/
    #[derive(Debug, Clone)]
    pub struct GetReportingUnitsByReportingUnitCodeDataItems<'a> {
        client: &'a super::Client,
        reporting_unit_code: Result<::std::string::String, String>,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetReportingUnitsByReportingUnitCodeDataItems<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                reporting_unit_code: Err(
                    "reporting_unit_code was not initialized".to_string(),
                ),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn reporting_unit_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.reporting_unit_code = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for reporting_unit_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/reporting-units/{reporting-unit-code}/data-items`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::GetReportingUnitsByReportingUnitCodeDataItemsResponse>,
            Error<()>,
        > {
            let Self { client, reporting_unit_code, x_swagger_ui } = self;
            let reporting_unit_code = reporting_unit_code
                .map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/reporting-units/{}/data-items", client.baseurl, encode_path(&
                reporting_unit_code.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_reporting_units_by_reporting_unit_code_data_items",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_reporting_units_by_reporting_unit_code_measures_available`]

[`Client::get_reporting_units_by_reporting_unit_code_measures_available`]: super::Client::get_reporting_units_by_reporting_unit_code_measures_available*/
    #[derive(Debug, Clone)]
    pub struct GetReportingUnitsByReportingUnitCodeMeasuresAvailable<'a> {
        client: &'a super::Client,
        reporting_unit_code: Result<::std::string::String, String>,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetReportingUnitsByReportingUnitCodeMeasuresAvailable<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                reporting_unit_code: Err(
                    "reporting_unit_code was not initialized".to_string(),
                ),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn reporting_unit_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.reporting_unit_code = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for reporting_unit_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/reporting-units/{reporting-unit-code}/measures-available`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<
                types::GetReportingUnitsByReportingUnitCodeMeasuresAvailableResponse,
            >,
            Error<()>,
        > {
            let Self { client, reporting_unit_code, x_swagger_ui } = self;
            let reporting_unit_code = reporting_unit_code
                .map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/reporting-units/{}/measures-available", client.baseurl,
                encode_path(& reporting_unit_code.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_reporting_units_by_reporting_unit_code_measures_available",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_simple_downloads_download_codes`]

[`Client::get_simple_downloads_download_codes`]: super::Client::get_simple_downloads_download_codes*/
    #[derive(Debug, Clone)]
    pub struct GetSimpleDownloadsDownloadCodes<'a> {
        client: &'a super::Client,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetSimpleDownloadsDownloadCodes<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                x_swagger_ui: Ok(None),
            }
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/simple-downloads/download-codes`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::GetSimpleDownloadsDownloadCodesResponse>,
            Error<()>,
        > {
            let Self { client, x_swagger_ui } = self;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/simple-downloads/download-codes", client.baseurl,
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_simple_downloads_download_codes",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_simple_downloads_by_download_code`]

[`Client::get_simple_downloads_by_download_code`]: super::Client::get_simple_downloads_by_download_code*/
    #[derive(Debug, Clone)]
    pub struct GetSimpleDownloadsByDownloadCode<'a> {
        client: &'a super::Client,
        download_code: Result<::std::string::String, String>,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetSimpleDownloadsByDownloadCode<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                download_code: Err("download_code was not initialized".to_string()),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn download_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.download_code = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for download_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/simple-downloads/{download-code}`
        pub async fn send(self) -> Result<ResponseValue<ByteStream>, Error<()>> {
            let Self { client, download_code, x_swagger_ui } = self;
            let download_code = download_code.map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/simple-downloads/{}", client.baseurl, encode_path(&
                download_code.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client.client.get(url).headers(header_map).build()?;
            let info = OperationInfo {
                operation_id: "get_simple_downloads_by_download_code",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => Ok(ResponseValue::stream(response)),
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_suppressions`]

[`Client::get_suppressions`]: super::Client::get_suppressions*/
    #[derive(Debug, Clone)]
    pub struct GetSuppressions<'a> {
        client: &'a super::Client,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetSuppressions<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                x_swagger_ui: Ok(None),
            }
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/suppressions`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetSuppressionsResponse>, Error<()>> {
            let Self { client, x_swagger_ui } = self;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!("{}/api/v1/suppressions", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_suppressions",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
    /**Builder for [`Client::get_suppressions_by_suppression_code`]

[`Client::get_suppressions_by_suppression_code`]: super::Client::get_suppressions_by_suppression_code*/
    #[derive(Debug, Clone)]
    pub struct GetSuppressionsBySuppressionCode<'a> {
        client: &'a super::Client,
        suppression_code: Result<::std::string::String, String>,
        x_swagger_ui: Result<Option<bool>, String>,
    }
    impl<'a> GetSuppressionsBySuppressionCode<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                suppression_code: Err(
                    "suppression_code was not initialized".to_string(),
                ),
                x_swagger_ui: Ok(None),
            }
        }
        pub fn suppression_code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.suppression_code = value
                .try_into()
                .map_err(|_| {
                    "conversion to `:: std :: string :: String` for suppression_code failed"
                        .to_string()
                });
            self
        }
        pub fn x_swagger_ui<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.x_swagger_ui = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for x_swagger_ui failed".to_string());
            self
        }
        ///Sends a `GET` request to `/api/v1/suppressions/{suppression-code}`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<types::GetSuppressionsBySuppressionCodeResponse>,
            Error<()>,
        > {
            let Self { client, suppression_code, x_swagger_ui } = self;
            let suppression_code = suppression_code.map_err(Error::InvalidRequest)?;
            let x_swagger_ui = x_swagger_ui.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api/v1/suppressions/{}", client.baseurl, encode_path(&
                suppression_code.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(2usize);
            header_map
                .append(
                    ::reqwest::header::HeaderName::from_static("api-version"),
                    ::reqwest::header::HeaderValue::from_static(
                        super::Client::api_version(),
                    ),
                );
            if let Some(value) = x_swagger_ui {
                header_map.append("X-SWAGGER-UI", value.to_string().try_into()?);
            }
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_suppressions_by_suppression_code",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
}
/// Items consumers will typically use such as the Client.
pub mod prelude {
    pub use self::super::Client;
}
