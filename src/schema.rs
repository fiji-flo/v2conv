use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct AccessInformationProviderSubObject {
    pub metadata: Metadata,
    pub signature: Signature,
    pub values: Value,
}

impl Default for AccessInformationProviderSubObject {
    fn default() -> Self {
        AccessInformationProviderSubObject::with(None, Classification::default())
    }
}

impl AccessInformationProviderSubObject {
    fn with(display: Option<Display>, classification: Classification) -> Self {
        AccessInformationProviderSubObject {
            metadata: Metadata::with(display, classification),
            signature: Signature::default(),
            values: json!({}),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum Alg {
    #[serde(rename = "HS256")]
    Hs256,
    #[serde(rename = "RS256")]
    Rs256,
    #[serde(rename = "RSA")]
    Rsa,
    #[serde(rename = "ED25519")]
    Ed25519,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum Classification {
    #[serde(rename = "MOZILLA CONFIDENTIAL")]
    MozillaConfidential,
    #[serde(rename = "WORKGROUP CONFIDENTIAL: STAFF ONLY")]
    WorkgroupConfidentialStaffOnly,
    #[serde(rename = "WORKGROUP CONFIDENTIAL")]
    WorkgroupConfidential,
    #[serde(rename = "PUBLIC")]
    Public,
    #[serde(rename = "INDIVIDUAL CONFIDENTIAL")]
    IndividualConfidential,
}

impl Default for Classification {
    fn default() -> Self {
        Classification::WorkgroupConfidential
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum Display {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "authenticated")]
    Authenticated,
    #[serde(rename = "vouched")]
    Vouched,
    #[serde(rename = "ndaed")]
    Ndaed,
    #[serde(rename = "staff")]
    Staff,
    #[serde(rename = "private")]
    Private,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Metadata {
    pub classification: Classification,
    pub created: String,
    pub display: Option<Display>,
    pub last_modified: String,
    pub verified: bool,
}

impl Default for Metadata {
    fn default() -> Self {
        Metadata::with(Some(Display::Staff), Classification::default())
    }
}

impl Metadata {
    fn with(display: Option<Display>, classification: Classification) -> Self {
        Metadata {
            classification,
            created: String::default(),
            display,
            last_modified: String::default(),
            verified: false,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum PublisherAuthority {
    #[serde(rename = "ldap")]
    Ldap,
    #[serde(rename = "mozilliansorg")]
    Mozilliansorg,
    #[serde(rename = "hris")]
    Hris,
    #[serde(rename = "cis")]
    Cis,
    #[serde(rename = "access_provider")]
    AccessProvider,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Publisher {
    pub alg: Alg,
    pub name: PublisherAuthority,
    pub typ: Typ,
    pub value: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Signature {
    pub additional: Vec<Publisher>,
    pub publisher: Publisher,
}

impl Default for Signature {
    fn default() -> Self {
        Signature {
            additional: vec![],
            publisher: Publisher {
                alg: Alg::Hs256,
                name: PublisherAuthority::Mozilliansorg,
                typ: Typ::Jws,
                value: String::default(),
            },
        }
    }
}

#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct StandardAttributeBoolean {
    pub metadata: Metadata,
    pub signature: Signature,
    pub value: bool,
}

impl StandardAttributeBoolean {
    fn with(value: bool, display: Option<Display>, classification: Classification) -> Self {
        StandardAttributeBoolean {
            metadata: Metadata::with(display, classification),
            signature: Signature::default(),
            value,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct StandardAttributeString {
    pub metadata: Metadata,
    pub signature: Signature,
    #[serde(default)]
    pub value: Option<String>,
}

impl StandardAttributeString {
    fn with(display: Option<Display>, classification: Classification) -> Self {
        StandardAttributeString {
            metadata: Metadata::with(display, classification),
            signature: Signature::default(),
            value: None,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct StandardAttributeValues {
    pub metadata: Metadata,
    pub signature: Signature,
    pub values: BTreeMap<String, serde_json::Value>,
}

impl StandardAttributeValues {
    fn with(display: Option<Display>, classification: Classification) -> Self {
        StandardAttributeValues {
            metadata: Metadata::with(display, classification),
            signature: Signature::default(),
            values: BTreeMap::default(),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum Typ {
    #[serde(rename = "JWS")]
    Jws,
    #[serde(rename = "PGP")]
    Pgp,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct AccessInformationValuesArray {
    pub access_provider: AccessInformationProviderSubObject,
    pub hris: AccessInformationProviderSubObject,
    pub ldap: AccessInformationProviderSubObject,
    pub mozilliansorg: AccessInformationProviderSubObject,
}

impl Default for AccessInformationValuesArray {
    fn default() -> Self {
        AccessInformationValuesArray {
            access_provider: AccessInformationProviderSubObject::default(),
            hris: AccessInformationProviderSubObject::with(
                None,
                Classification::WorkgroupConfidentialStaffOnly,
            ),
            ldap: AccessInformationProviderSubObject::with(None, Classification::Public),
            mozilliansorg: AccessInformationProviderSubObject::with(
                Some(Display::Staff),
                Classification::Public,
            ),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct IdentitiesAttributesValuesArray {
    #[serde(default)]
    pub github_id_v3: StandardAttributeString,
    #[serde(default)]
    pub github_id_v4: StandardAttributeString,
    #[serde(default)]
    pub github_primary_email: StandardAttributeString,
    #[serde(default)]
    pub dinopark_id: StandardAttributeString,
    #[serde(default)]
    pub mozilliansorg_id: StandardAttributeString,
    #[serde(default)]
    pub bugzilla_mozilla_org_id: StandardAttributeString,
    #[serde(default)]
    pub bugzilla_mozilla_primary_email: StandardAttributeString,
    #[serde(default)]
    pub mozilla_ldap_id: StandardAttributeString,
    #[serde(default)]
    pub mozilla_ldap_primary_email: StandardAttributeString,
    #[serde(default)]
    pub mozilla_posix_id: StandardAttributeString,
    #[serde(default)]
    pub google_oauth2_id: StandardAttributeString,
    #[serde(default)]
    pub google_primary_email: StandardAttributeString,
    #[serde(default)]
    pub firefox_accounts_id: StandardAttributeString,
    #[serde(default)]
    pub firefox_accounts_primary_email: StandardAttributeString,
}

impl Default for IdentitiesAttributesValuesArray {
    fn default() -> Self {
        IdentitiesAttributesValuesArray {
            github_id_v3: StandardAttributeString::default(),
            github_id_v4: StandardAttributeString::default(),
            github_primary_email: StandardAttributeString::with(
                Some(Display::Public),
                Classification::default(),
            ),
            dinopark_id: StandardAttributeString::with(
                Some(Display::Public),
                Classification::default(),
            ),
            mozilliansorg_id: StandardAttributeString::with(
                Some(Display::Staff),
                Classification::default(),
            ),
            bugzilla_mozilla_org_id: StandardAttributeString::default(),
            bugzilla_mozilla_primary_email: StandardAttributeString::with(
                Some(Display::Public),
                Classification::default(),
            ),
            mozilla_ldap_id: StandardAttributeString::with(
                Some(Display::Staff),
                Classification::default(),
            ),
            mozilla_ldap_primary_email: StandardAttributeString::with(
                Some(Display::Public),
                Classification::default(),
            ),
            mozilla_posix_id: StandardAttributeString::default(),
            google_oauth2_id: StandardAttributeString::default(),
            google_primary_email: StandardAttributeString::with(
                Some(Display::Public),
                Classification::default(),
            ),
            firefox_accounts_id: StandardAttributeString::default(),
            firefox_accounts_primary_email: StandardAttributeString::with(
                Some(Display::Public),
                Classification::default(),
            ),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct StaffInformationValuesArray {
    pub manager: StandardAttributeBoolean,
    pub director: StandardAttributeBoolean,
    pub staff: StandardAttributeBoolean,
    pub title: StandardAttributeString,
    pub team: StandardAttributeString,
    pub cost_center: StandardAttributeString,
    pub worker_type: StandardAttributeString,
    pub wpr_desk_number: StandardAttributeString,
    pub office_location: StandardAttributeString,
}

impl Default for StaffInformationValuesArray {
    fn default() -> Self {
        StaffInformationValuesArray {
            manager: StandardAttributeBoolean::default(),
            director: StandardAttributeBoolean::default(),
            staff: StandardAttributeBoolean::default(),
            title: StandardAttributeString::default(),
            team: StandardAttributeString::default(),
            cost_center: StandardAttributeString::with(
                Some(Display::Staff),
                Classification::WorkgroupConfidentialStaffOnly,
            ),
            worker_type: StandardAttributeString::with(
                Some(Display::Staff),
                Classification::WorkgroupConfidentialStaffOnly,
            ),
            wpr_desk_number: StandardAttributeString::default(),
            office_location: StandardAttributeString::default(),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Profile {
    pub access_information: AccessInformationValuesArray,
    pub active: StandardAttributeBoolean,
    pub alternative_name: StandardAttributeString,
    pub created: StandardAttributeString,
    pub description: StandardAttributeString,
    pub first_name: StandardAttributeString,
    pub fun_title: StandardAttributeString,
    pub identities: IdentitiesAttributesValuesArray,
    pub languages: StandardAttributeValues,
    pub last_modified: StandardAttributeString,
    pub last_name: StandardAttributeString,
    pub location: StandardAttributeString,
    pub login_method: StandardAttributeString,
    pub pgp_public_keys: StandardAttributeValues,
    pub phone_numbers: StandardAttributeValues,
    pub picture: StandardAttributeString,
    pub primary_email: StandardAttributeString,
    pub pronouns: StandardAttributeString,
    pub schema: String,
    pub ssh_public_keys: StandardAttributeValues,
    pub staff_information: StaffInformationValuesArray,
    pub tags: StandardAttributeValues,
    pub timezone: StandardAttributeString,
    pub uris: StandardAttributeValues,
    pub user_id: StandardAttributeString,
    pub usernames: StandardAttributeValues,
}

impl Default for Profile {
    fn default() -> Self {
        Profile {
            access_information: AccessInformationValuesArray::default(),
            active: StandardAttributeBoolean::with(true, None, Classification::default()),
            alternative_name: StandardAttributeString::default(),
            created: StandardAttributeString::with(Some(Display::Private), Classification::Public),
            description: StandardAttributeString::default(),
            first_name: StandardAttributeString::with(Some(Display::Staff), Classification::Public),
            fun_title: StandardAttributeString::default(),
            identities: IdentitiesAttributesValuesArray::default(),
            languages: StandardAttributeValues::default(),
            last_modified: StandardAttributeString::with(
                Some(Display::Staff),
                Classification::Public,
            ),
            last_name: StandardAttributeString::with(Some(Display::Staff), Classification::Public),
            location: StandardAttributeString::default(),
            login_method: StandardAttributeString::with(
                Some(Display::Staff),
                Classification::Public,
            ),
            pgp_public_keys: StandardAttributeValues::with(
                Some(Display::Staff),
                Classification::Public,
            ),
            phone_numbers: StandardAttributeValues::default(),
            picture: StandardAttributeString::with(Some(Display::Staff), Classification::Public),
            primary_email: StandardAttributeString::with(
                Some(Display::Staff),
                Classification::Public,
            ),
            pronouns: StandardAttributeString::default(),
            schema: String::from("https://person-api.sso.mozilla.com/schema/v2/profile"),
            ssh_public_keys: StandardAttributeValues::with(
                Some(Display::Staff),
                Classification::Public,
            ),
            staff_information: StaffInformationValuesArray::default(),
            tags: StandardAttributeValues::default(),
            timezone: StandardAttributeString::default(),
            uris: StandardAttributeValues::default(),
            user_id: StandardAttributeString::with(Some(Display::Staff), Classification::Public),
            usernames: StandardAttributeValues::with(
                Some(Display::Public),
                Classification::default(),
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basic_profile() {
        let p = Profile::default();
        println!("{}", json!(p))
    }

    #[test]
    fn signature() {
        let j = r#"
        {
                "publisher": {
                    "alg": "RS256",
                    "typ": "JWS",
                    "value": "foobar"
                },
                "additional": [
                    {
                        "alg": "RS256",
                        "typ": "JWS",
                        "value": ""
                    }
                ]
            }
        "#;
        let v: Value = serde_json::from_str(j).unwrap();
        let _s: Signature = serde_json::from_value(v).unwrap();
    }
}
