use variant::{AboutPage, BlogPage, CvPage, DemosPage, ProjectsPage, ResearchPage, TeachingPage};

mod variant;

#[derive(
    Copy, Clone, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize, strum::EnumIter,
)]
pub enum Page {
    About,
    Projects,
    Research,
    Teaching,
    Cv,
    Blog,
    Demos,
}

impl Default for Page {
    fn default() -> Self {
        Self::About
    }
}

impl std::fmt::Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Page {
    pub fn title(self) -> &'static str {
        match self {
            Self::About => "About",
            Self::Projects => "Projects",
            Self::Research => "Research",
            Self::Teaching => "Teaching",
            Self::Cv => "CV",
            Self::Blog => "Blog",
            Self::Demos => "Demos",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::About => "About me",
            Self::Projects => "Open source projects",
            Self::Research => "Research endeavors",
            Self::Teaching => "Teaching materials",
            Self::Cv => "Curriculum Vitae",
            Self::Blog => "Blog",
            Self::Demos => "Online demos",
        }
    }

    pub fn redirect_page(self) -> Option<&'static str> {
        match self {
            Self::Cv => Some(crate::CV_URL),
            _ => None,
        }
    }

    pub fn default_app(self) -> Box<dyn eframe::App> {
        match self {
            Self::About => Box::<AboutPage>::default(),
            Self::Projects => Box::<ProjectsPage>::default(),
            Self::Research => Box::<ResearchPage>::default(),
            Self::Teaching => Box::<TeachingPage>::default(),
            Self::Cv => Box::<CvPage>::default(),
            Self::Blog => Box::<BlogPage>::default(),
            Self::Demos => Box::<DemosPage>::default(),
        }
    }
}
