use crate::page::Page;
use crate::social::Social;

pub const ENABLED_PAGES: [Page; 3] = [
    Page::About,
    Page::Research,
    Page::Teaching,
    // Page::Software,
    // Page::Cv,
    // Page::Blog,
    // Page::Demos,
];

pub const AUTHOR_NAME_FULL: &str = "Andrej Orsula";
pub const AUTHOR_NAME_SHORT: &str = "A Orsula";
pub const AUTHOR_SURNAME: &str = "Orsula";
pub const AUTHOR_INITIALS: [char; 2] = ['A', 'O'];

pub const AUTHOR_TITLE: &str = "PhD Student in Space Robotics";

pub const ENABLED_SOCIALS: [Social; 6] = [
    Social::Email,
    Social::LinkedIn,
    Social::Orcid,
    Social::Scholar,
    Social::YouTube,
    Social::GitHub,
];

pub const EMAIL_ADDRESS: &str = "mailto:orsula.andrej@gmail.com";
pub const LINKEDIN_URL: &str = "https://linkedin.com/in/AndrejOrsula";
pub const ORCID_URL: &str = "https://orcid.org/0000-0003-0706-1191";
pub const SCHOLAR_URL: &str = "https://scholar.google.com/citations?user=sbQC2dAAAAAJ";
pub const YOUTUBE_URL: &str = "https://youtube.com/channel/UCqatO1yebNRswWSO2fxRWTg";
pub const GITHUB_URL: &str = "https://github.com/AndrejOrsula";
