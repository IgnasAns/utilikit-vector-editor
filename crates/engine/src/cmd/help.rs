//! Help → project links. UtiliKit build: the links point at utilikit.eu, plus one to the upstream
//! open-source project (VectorCraft) this build is based on. The commands return the URL; the
//! frontend opens it.

use serde_json::{Value, json};

use super::{CommandSpec, Result, always, cmd};

pub const WEBSITE_URL: &str = "https://utilikit.eu/vector-editor/";
pub const MORE_TOOLS_URL: &str = "https://utilikit.eu/";
pub const ISSUES_URL: &str = "https://utilikit.eu/vector-editor/#feedback";
pub const GITHUB_URL: &str = "https://github.com/storytold/vectorcraft";

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(query "help.website", "UtiliKit Vector Editor Website", ["Help"], None, "{} → {url}", always, |_, _| url(WEBSITE_URL.into())),
        cmd!(query "help.moreTools", "More free tools", ["Help"], None, "{} → {url}", always, |_, _| url(MORE_TOOLS_URL.into())),
        cmd!(query "help.github", "Open-source project (VectorCraft)", ["Help"], None, "{} → {url} source code of the project this app is based on", always, |_, _| url(GITHUB_URL.into())),
        cmd!(query "help.reportIssue", "Report an Issue…", ["Help"], None, "{} → {url}", always, |_, _| url(ISSUES_URL.into())),
        cmd!(query "help.links", "Links", [], None, "{} → {website, moreTools, github, reportIssue}", always, |_, _| {
            Ok(json!({ "website": WEBSITE_URL, "moreTools": MORE_TOOLS_URL, "github": GITHUB_URL, "reportIssue": ISSUES_URL }))
        }),
    ]
}

fn url(u: String) -> Result<Value> {
    Ok(json!({ "url": u }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Session;

    #[test]
    fn links() {
        let mut s = Session::new();
        assert_eq!(s.execute("help.website", &json!({})).unwrap()["url"], "https://utilikit.eu/vector-editor/");
        assert_eq!(s.execute("help.moreTools", &json!({})).unwrap()["url"], "https://utilikit.eu/");
        assert_eq!(s.execute("help.github", &json!({})).unwrap()["url"], "https://github.com/storytold/vectorcraft");
        assert_eq!(s.execute("help.links", &json!({})).unwrap()["reportIssue"], "https://utilikit.eu/vector-editor/#feedback");
    }
}
