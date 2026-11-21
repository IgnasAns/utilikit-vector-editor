//! Project links (website, more free tools, the upstream open-source project): the link list of
//! About and Home. UtiliKit build: no Discord or ArtCraft links.
//! Icons are Lucide's (ISC).

use egui::Ui;

use crate::theme::Tokens;
use crate::{VectorcraftApp, icons};

/// One link row: icon, label, opens the command's URL. `centered` drops the icon so the label
/// alone sits in the middle of the column.
fn link(app: &mut VectorcraftApp, ui: &mut Ui, icon: &str, label: &str, cmd: &str, url: &str, centered: bool) {
    let t = Tokens::get(ui.ctx());
    let button = |ui: &mut Ui| ui.add(egui::Button::new(egui::RichText::new(label).color(t.accent).size(13.0)).frame(false)).on_hover_text(url);
    let r = if centered {
        button(ui)
    } else {
        ui.horizontal(|ui| {
            icons::icon(ui, icon, 16.0, t.icon);
            button(ui)
        })
        .inner
    };
    if r.clicked() {
        app.open_link(cmd);
    }
}

/// The website, more-tools and open-source links.
pub fn links(app: &mut VectorcraftApp, ui: &mut Ui) {
    links_in(app, ui, false);
}

/// [`links`] for a centred column (the Home screen's).
pub fn links_centered(app: &mut VectorcraftApp, ui: &mut Ui) {
    links_in(app, ui, true);
}

fn links_in(app: &mut VectorcraftApp, ui: &mut Ui, centered: bool) {
    let l = app.session.execute("help.links", &serde_json::json!({})).unwrap_or_default();
    let s = |k: &str| l[k].as_str().unwrap_or("").to_string();
    link(app, ui, "globe", tl!("Website"), "help.website", &s("website"), centered);
    link(app, ui, "external-link", tl!("More free tools"), "help.moreTools", &s("moreTools"), centered);
    link(app, ui, "git-branch", tl!("Open-source project (VectorCraft)"), "help.github", &s("github"), centered);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_clicks_open_the_right_urls() {
        use std::sync::{Arc, Mutex};
        let opened = Arc::new(Mutex::new(vec![]));
        let o = opened.clone();
        let services = crate::Services { open_url: Some(Box::new(move |u: &str| o.lock().unwrap().push(u.to_string()))), ..Default::default() };
        let mut app = VectorcraftApp::new(vectorcraft_engine::Session::new(), services);
        for id in ["help.website", "help.moreTools", "help.github", "help.reportIssue"] {
            crate::menus::invoke(&mut app, id, serde_json::json!({}));
        }
        assert_eq!(
            *opened.lock().unwrap(),
            [
                "https://utilikit.eu/vector-editor/",
                "https://utilikit.eu/",
                "https://github.com/storytold/vectorcraft",
                "https://utilikit.eu/vector-editor/#feedback"
            ]
        );
    }
}
