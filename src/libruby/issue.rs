use std::any::Any;

use log::*;
use rutie::{class, methods, RString, AnyObject, NilClass, Object, eval, Binding, Hash};

use crate::modules::{Issue, IssueLevel, IssueConfidence};

use super::http::utils::ruby_resp_hash_to_reqresplog;
class!(RBIssue);

methods!(
    RBIssue,
    _rtself,
    fn push_issue(issue: Hash) -> AnyObject {
        _push_issue(issue.unwrap())
    }
);

fn _push_issue(issue: Hash) -> AnyObject {
    let name = issue.at(&RString::from("name")).try_convert_to::<RString>().unwrap().to_string();
    let detail = issue.at(&RString::from("detail")).try_convert_to::<RString>().unwrap().to_string();
    let _level = issue.at(&RString::from("level")).try_convert_to::<RString>().unwrap().to_string();
    let _confidence = issue.at(&RString::from("confidence")).try_convert_to::<RString>().unwrap().to_string();
    let host = issue.at(&RString::from("host")).try_convert_to::<RString>().unwrap().to_string();
    let response = issue.at(&RString::from("response")).try_convert_to::<Hash>().unwrap();
    let reqreslog = ruby_resp_hash_to_reqresplog(&response);
    let mut level = IssueLevel::Info;
    if _level.eq_ignore_ascii_case("info") {
        level = IssueLevel::Info;
    } else if _level.eq_ignore_ascii_case("critical") {
        level = IssueLevel::Critical;
    } else if _level.eq_ignore_ascii_case("medium") {
        level = IssueLevel::Medium;
    } else if _level.eq_ignore_ascii_case("highrisk") {
        level = IssueLevel::HighRisk;
    }
    let mut confidence = IssueConfidence::Suspicious;
    if _confidence.eq_ignore_ascii_case("Suspicious") {
        confidence = IssueConfidence::Suspicious;
    } else if _confidence.eq_ignore_ascii_case("Confirm") {
        confidence = IssueConfidence::Confirm;
    }

    let issue = Issue::new(&name, level, &detail, confidence, &host);
    Issue::add_issue(issue, &reqreslog);
    NilClass::new().try_convert_to::<AnyObject>().unwrap()
}

