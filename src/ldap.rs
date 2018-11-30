use std::cmp::min;
use std::path::PathBuf;

use serde::de::Error;
use serde_json::Value;
use uuid::Uuid;

use avatar::*;
use schema::*;
use username::generate_username;

pub fn map_ldap(
    mut p2: Profile,
    mut ldap: Value,
    avatar_in: &Option<PathBuf>,
    avatar_out: &Option<PathBuf>,
    entropy: &str,
) -> Result<Profile, serde_json::Error> {
    let primary_email = ldap["primary_email"]["value"].take();
    let primary_email = primary_email
        .as_str()
        .map(String::from)
        .ok_or_else(|| Error::custom(format!("{:?}", ldap)))?;
    let dinopark_id = format!(
        "{}",
        Uuid::new_v5(&Uuid::NAMESPACE_URL, primary_email.as_bytes())
    );
    let username = ldap["usernames"]["values"]
        .as_object()
        .and_then(|o| {
            o.values()
                .filter_map(|v| {
                    v.as_str().and_then(|s| match &s[0..min(s.len(), 3)] {
                        "IRC" | "irc" => s.split(' ').last().map(String::from),
                        _ => None,
                    })
                }).next()
        }).unwrap_or_else(|| generate_username(&primary_email, entropy));
    if !ldap["first_name"]["value"].is_null() {
        p2.first_name.value = serde_json::from_value(ldap["first_name"]["value"].take())?;
    }
    if !ldap["last_name"]["value"].is_null() {
        p2.last_name.value = serde_json::from_value(ldap["last_name"]["value"].take())?;
    }
    p2.ssh_public_keys.values = serde_json::from_value(ldap["ssh_public_keys"]["values"].take())?;
    p2.pgp_public_keys.values = serde_json::from_value(ldap["pgp_public_keys"]["values"].take())?;
    p2.phone_numbers.values = serde_json::from_value(ldap["phone_numbers"]["values"].take())?;
    p2.identities.bugzilla_mozilla_org_id.value =
        serde_json::from_value(ldap["identities"]["bugzilla_mozilla_org_id"]["value"].take())?;
    p2.identities.dinopark_id.value = Some(dinopark_id.clone());
    p2.identities.firefox_accounts_id.value =
        serde_json::from_value(ldap["identities"]["firefox_accounts_id"]["value"].take())?;
    p2.identities.github_id_v3.value =
        serde_json::from_value(ldap["identities"]["github_id_v3"]["value"].take())?;
    p2.identities.github_id_v4.value =
        serde_json::from_value(ldap["identities"]["github_id_v4"]["value"].take())?;
    p2.identities.google_oauth2_id.value =
        serde_json::from_value(ldap["identities"]["google_oauth2_id"]["value"].take())?;
    p2.identities.mozilla_ldap_id.value =
        serde_json::from_value(ldap["identities"]["mozilla_ldap_id"]["value"].take())?;
    p2.identities.mozilla_posix_id.value =
        serde_json::from_value(ldap["identities"]["mozilla_posix_id"]["value"].take())?;
    p2.identities.mozilliansorg_id.value =
        serde_json::from_value(ldap["identities"]["mozilliansorg_id"]["value"].take())?;
    p2.usernames.values = serde_json::from_value(ldap["usernames"]["values"].take())?;
    p2.user_id.value = serde_json::from_value(ldap["user_id"]["value"].take())?;
    p2.login_method.value = serde_json::from_value(ldap["login_method"]["value"].take())?;
    p2.primary_email.value = serde_json::from_value(ldap["primary_email"]["value"].take())?;
    p2.access_information.ldap.values =
        serde_json::from_value(ldap["access_information"]["ldap"]["values"].take())?;
    p2.fun_title.value = serde_json::from_value(ldap["fun_title"]["value"].take())?;
    p2.active.value = serde_json::from_value(ldap["active"]["value"].take())?;
    p2.description.value = serde_json::from_value(ldap["description"]["value"].take())?;

    p2.picture.value = serde_json::from_value(handle_picture(
        &ldap["picture"],
        avatar_in,
        avatar_out,
        &format!("{}.png", dinopark_id),
    ))?;

    p2.usernames
        .values
        .insert(String::from("mozilliansorg"), username.into());
    Ok(p2)
}

fn handle_picture(
    v: &Value,
    input_path: &Option<PathBuf>,
    output_path: &Option<PathBuf>,
    name: &str,
) -> Value {
    if let (Some(i), Some(o), Some(p)) = (input_path, output_path, v["value"].clone().as_str()) {
        let mut input = i.clone();
        let input_file_path = PathBuf::from(p);
        if let Some(input_file_name) = input_file_path.file_name() {
            input.push(input_file_name);
            match convert_path(&input, &o, &name) {
                Ok(()) => {
                    return json!(name);
                }
                Err(e) => {
                    eprintln!("error handling picture: {}", e);
                }
            };
        }
    };
    Value::Null
}
