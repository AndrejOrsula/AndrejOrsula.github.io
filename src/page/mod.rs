use variant::{AboutPage, BlogPage, CvPage, DemosPage, ResearchPage, SoftwarePage, TeachingPage};

mod variant;

#[derive(
    Copy, Clone, Debug, Eq, PartialEq, Hash, serde::Deserialize, serde::Serialize, strum::EnumIter,
)]
pub enum Page {
    About,
    Research,
    Teaching,
    Software,
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
    pub fn title(&self) -> &str {
        match self {
            Self::About => "About",
            Self::Research => "Research",
            Self::Teaching => "Teaching",
            Self::Software => "Software",
            Self::Cv => "CV",
            Self::Blog => "Blog",
            Self::Demos => "Demos",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Self::About => "About me",
            Self::Research => "Research endeavors",
            Self::Teaching => "Teaching materials",
            Self::Software => "Software projects",
            Self::Cv => "Curriculum vitae",
            Self::Blog => "My Blog",
            Self::Demos => "Online demos",
        }
    }

    pub fn default_app(self) -> Box<dyn eframe::App> {
        match self {
            Self::About => Box::<AboutPage>::default(),
            Self::Research => Box::<ResearchPage>::default(),
            Self::Teaching => Box::<TeachingPage>::default(),
            Self::Software => Box::<SoftwarePage>::default(),
            Self::Cv => Box::<CvPage>::default(),
            Self::Blog => Box::<BlogPage>::default(),
            Self::Demos => Box::<DemosPage>::default(),
        }
    }
}
