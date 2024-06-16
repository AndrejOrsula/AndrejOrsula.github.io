#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, strum::EnumIter)]
pub enum Social {
    Email,
    LinkedIn,
    Orcid,
    Scholar,
    YouTube,
    GitHub,
}

impl Social {
    pub fn show(self, ui: &mut egui::Ui) {
        let button = ui
            .add(egui::Button::new(self.icon()))
            .on_hover_text(self.description());

        if button.clicked() {
            ui.ctx().open_url(egui::OpenUrl::same_tab(self.url()));
        } else if button.middle_clicked() {
            match self {
                Self::Email => {
                    ui.ctx().open_url(egui::OpenUrl::same_tab(self.url()));
                }
                _ => {
                    ui.ctx().open_url(egui::OpenUrl::new_tab(self.url()));
                }
            }
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Self::Email => "Write me an email",
            Self::LinkedIn => "Connect with me on LinkedIn",
            Self::Orcid => "View my ORCID profile",
            Self::Scholar => "See my publications on Google Scholar",
            Self::YouTube => "Watch my videos on YouTube",
            Self::GitHub => "Find my projects on GitHub",
        }
    }

    pub fn url(&self) -> &str {
        match self {
            Self::Email => crate::EMAIL_ADDRESS,
            Self::LinkedIn => crate::LINKEDIN_URL,
            Self::Orcid => crate::ORCID_URL,
            Self::Scholar => crate::SCHOLAR_URL,
            Self::YouTube => crate::YOUTUBE_URL,
            Self::GitHub => crate::GITHUB_URL,
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            Self::Email => "@",
            Self::LinkedIn => "\u{f0e1}",
            Self::Orcid => "\u{f8d2}",
            Self::Scholar => "\u{e63b}",
            Self::YouTube => "\u{f167}",
            Self::GitHub => "\u{f09b}",
        }
    }
}

impl std::fmt::Display for Social {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
