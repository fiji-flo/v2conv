use std::collections::BTreeMap;
use std::path::PathBuf;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use reqwest;
use serde::de::Error;
use serde_json::Value;
use uuid::Uuid;

use avatar::*;
use schema::*;

pub fn map_mozillians(
    mut p2: Profile,
    mut mozillians: Value,
    avatar_out: &Option<PathBuf>,
) -> Result<Profile, serde_json::Error> {
    if mozillians.is_null() {
        return Ok(p2);
    }
    let dinopark_id = format!(
        "{}",
        Uuid::new_v5(
            &Uuid::NAMESPACE_URL,
            mozillians["username"]
                .as_str()
                .ok_or_else(|| Error::custom(format!("{:?}", mozillians)))?
                .as_bytes()
        )
    );
    let m_username = mozillians["username"].as_str().map(String::from);
    eprintln!("mozillian: {}", m_username.clone().unwrap_or_default());
    if p2.first_name.value.is_none() {
        p2.first_name.value = serde_json::from_value(mozillians["first_name"].take())?;
    }
    if p2.last_name.value.is_none() {
        p2.last_name.value = serde_json::from_value(mozillians["last_name"].take())?;
    }
    if p2.identities.dinopark_id.value.is_none() {
        p2.identities.dinopark_id.value = Some(dinopark_id.clone());
    }
    if p2.user_id.value.is_none() {
        p2.user_id.value = serde_json::from_value(mozillians["user_id"].take())?;
    }
    if p2.fun_title.value.is_none() {
        p2.fun_title.value = serde_json::from_value(mozillians["fun_title"].take())?;
    }
    p2.active.value = true;
    if p2.description.value.is_none() {
        p2.description.value = serde_json::from_value(mozillians["description"].take())?;
    }

    if p2.timezone.value.is_none() {
        p2.timezone.value = serde_json::from_value(mozillians["timezone"].take())?;
    }

    p2.access_information.mozilliansorg.values =
        serde_json::from_value(mozillians["access_information"].take())?;

    let m_tags = mozillians["tags"].take();
    let m_tags = m_tags
        .as_array()
        .map(|a| a.into_iter())
        .unwrap_or_else(|| (&[]).into_iter());
    let m_skills = mozillians["skills"].take();
    let m_skills = m_skills
        .as_array()
        .map(|a| a.into_iter())
        .unwrap_or_else(|| (&[]).into_iter());
    let tags: BTreeMap<String, Value> = m_tags
        .chain(m_skills)
        .filter_map(|x| x.as_str().map(|s| (String::from(s), Value::default())))
        .collect();
    p2.tags.values = tags;

    let m_languages = mozillians["preferred_language"].take();
    let languages: BTreeMap<String, Value> = m_languages
        .as_array()
        .map(|a| a.into_iter())
        .unwrap_or_else(|| (&[]).into_iter())
        .filter_map(|x| x.as_str().map(|s| (String::from(s), Value::default())))
        .collect();
    p2.languages.values = languages;

    let m_uris = mozillians["uris"].take();
    if let Some(o) = m_uris.as_object() {
        let uris: BTreeMap<String, Value> = o
            .into_iter()
            .filter_map(|(k, v)| {
                if v.as_str().map(|s| !s.is_empty()).unwrap_or_default() {
                    Some((format!("EA#{}", k), v.clone()))
                } else {
                    None
                }
            }).collect();
        p2.uris.values = uris;
    }

    if p2.picture.value.is_none() {
        p2.picture.value = serde_json::from_value(handle_picture(
            mozillians["picture"].take(),
            avatar_out,
            &format!("{}.png", dinopark_id),
        ))?;
    }

    if p2.primary_email.value.is_none() {
        p2.primary_email.value = serde_json::from_value(mozillians["idps"][0]["email"].take())?;
    }

    if let Some(username) = m_username {
        p2.usernames
            .values
            .insert(String::from("mozilliansorg"), username.into());
    } else if p2.usernames.values.get("mozilliansorg").is_none() {
        let username: String = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
        p2.usernames
            .values
            .insert(String::from("mozilliansorg"), username.into());
    }
    Ok(p2)
}

fn handle_picture(v: Value, output_path: &Option<PathBuf>, name: &str) -> Value {
    match (output_path, v.clone().as_str()) {
        (Some(o), Some(u)) => {
            if let Ok(mut resp) = reqwest::get(u) {
                let mut buf: Vec<u8> = vec![];
                if resp.copy_to(&mut buf).is_ok() {
                    match convert_buf(&buf, &o, &name) {
                        Ok(()) => {
                            return json!(name);
                        }
                        Err(e) => {
                            eprintln!("error handling picture: {}", e);
                        }
                    };
                }
            }
        }
        _ => {}
    };
    Value::Null
}
