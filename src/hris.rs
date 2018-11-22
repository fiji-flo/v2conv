use regex::Captures;
use regex::Regex;
use serde_json::Value;

use schema::*;

pub fn map_hris(mut p2: Profile, hris: Value) -> Profile {
    p2.access_information.hris.values = hris.clone();

    p2.staff_information.cost_center.value = hris["Cost_Center"].as_str().map(String::from);
    p2.staff_information.director.value = hris["isDirectorOrAbove"]
        .as_str()
        .map(|b| b == "TRUE")
        .unwrap_or_default();
    p2.staff_information.manager.value = hris["IsManager"]
        .as_str()
        .map(|b| b == "TRUE")
        .unwrap_or_default();
    p2.staff_information.office_location.value =
        hris["LocationDescription"].as_str().map(String::from);
    p2.staff_information.staff.value = hris["EmployeeID"]
        .as_str()
        .map(|_| true)
        .unwrap_or_default();
    p2.staff_information.team.value = hris["Team"].as_str().map(String::from);
    p2.staff_information.title.value = hris["businessTitle"].as_str().map(censor_title);
    p2.staff_information.worker_type.value = hris["WorkerType"].as_str().map(String::from);
    p2.staff_information.wpr_desk_number.value = hris["WPRDeskNumber"].as_str().map(String::from);

    p2
}

fn censor_title(title: &str) -> String {
    let re = Regex::new(r#"((Management)|(Engineering)|(Development))(:? Mgmt \d$)"#).unwrap();
    let censored = re.replace(&title, |caps: &Captures| format!("{}", &caps[1]));
    let re = Regex::new(r#"(:? Mgmt \d$)"#).unwrap();
    let censored = re.replace(&censored, " Management");
    let re = Regex::new(r#" \d$"#).unwrap();
    let censored = re.replace(&censored, "");
    censored.to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_censor() {
        let title = "Foo Engineering Mgmt 5";
        let censored = censor_title(title);
        assert_eq!(censored, "Foo Engineering");
    }
    #[test]
    fn test_mgmt_replace_censor() {
        let title = "Foo Product Mgmt 5";
        let censored = censor_title(title);
        assert_eq!(censored, "Foo Product Management");
    }
    #[test]
    fn test_no_number_censor() {
        let title = "Foo Engineer 2";
        let censored = censor_title(title);
        assert_eq!(censored, "Foo Engineer");
    }
}
