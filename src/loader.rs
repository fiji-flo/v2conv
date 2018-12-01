use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use serde_json::Value;

#[derive(Default)]
pub struct Data {
    pub hris: Value,
    pub ldap: Value,
    pub mozillians: Value,
}

pub fn load_json(path: impl Into<PathBuf>) -> Result<Value, String> {
    let mut s = String::new();
    File::open(path.into())
        .map_err(|e| format!("{}", e))?
        .read_to_string(&mut s)
        .map_err(|e| format!("{}", e))?;
    serde_json::from_str(&s).map_err(|e| format!("{}", e))
}

pub fn load_all(hris: &str, ldap: &str, mozillians: &str) -> Result<HashMap<String, Data>, String> {
    let mut h = HashMap::<String, Data>::new();
    let mut ldap_to_mail = HashMap::new();
    load_hris(hris, &mut h)?;
    load_ldap(ldap, &mut h, &mut ldap_to_mail)?;
    load_mozillians(mozillians, &mut h, &ldap_to_mail)?;
    Ok(h)
}

fn load_hris(hris: &str, h: &mut HashMap<String, Data>) -> Result<(), String> {
    if hris.is_empty() {
        return Ok(());
    }
    let mut hris_data = load_json(hris)?;

    for e in hris_data["Report_Entry"]
        .as_array_mut()
        .ok_or_else(|| String::from("hirs data should be an array"))?
        .into_iter()
        .map(|e| e.take())
    {
        let mail = e["PrimaryWorkEmail"].clone();
        let active = e["CurrentlyActive"].clone();
        if active.as_str() == Some("1") {
            if let Some(mail) = mail.as_str() {
                h.insert(
                    String::from(mail),
                    Data {
                        hris: e,
                        ldap: Value::default(),
                        mozillians: Value::default(),
                    },
                );
            }
        }
    }
    Ok(())
}

fn load_ldap(
    ldap: &str,
    h: &mut HashMap<String, Data>,
    ldap_to_mail: &mut HashMap<String, String>,
) -> Result<(), String> {
    if ldap.is_empty() {
        return Ok(());
    }
    let mut ldap_data = load_json(ldap)?;

    for e in ldap_data
        .as_object_mut()
        .ok_or_else(|| String::from("ldap data should be an object"))?
        .into_iter()
        .map(|(_, v)| v.take())
    {
        let mail = e["primary_email"]["value"].clone();
        if let Some(mail) = mail.as_str() {
            if mail.ends_with("@mozilla.com")
                || mail.ends_with("@mozillafoundation.org")
                || mail.ends_with("@getpocket.com")
            {
                if let Some(id) = e["user_id"]["value"].as_str() {
                    ldap_to_mail.insert(String::from(id), String::from(mail));
                }
                if h.contains_key(mail) {
                    if let Some(data) = h.get_mut(mail) {
                        data.ldap = e;
                    }
                } else {
                    h.insert(
                        String::from(mail),
                        Data {
                            hris: Value::default(),
                            ldap: e,
                            mozillians: Value::default(),
                        },
                    );
                };
            }
        }
    }
    Ok(())
}

fn load_mozillians(
    mozillians: &str,
    h: &mut HashMap<String, Data>,
    ldap_to_mail: &HashMap<String, String>,
) -> Result<(), String> {
    if mozillians.is_empty() {
        return Ok(());
    }
    let mut mozillians_data = load_json(mozillians)?;

    for e in mozillians_data
        .as_array_mut()
        .ok_or_else(|| String::from("mozillians data should be an array"))?
        .into_iter()
        .map(|e| e.take())
    {
        let user_id = e["user_id"].clone();
        if let Some(user_id) = user_id.as_str() {
            if let Some(mail) = ldap_to_mail.get(user_id) {
                if let Some(data) = h.get_mut(mail) {
                    data.mozillians = e;
                }
            } else {
                h.insert(
                    String::from(user_id),
                    Data {
                        hris: Value::default(),
                        ldap: Value::default(),
                        mozillians: e,
                    },
                );
            }
        }
    }
    Ok(())
}
