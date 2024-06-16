use biblatex::ChunksExt;
use itertools::Itertools;

use super::{BibliographyEntry, BibliographyEntryConfig};

#[derive(Debug, Clone, Copy, PartialEq, typed_builder::TypedBuilder)]
pub struct BibliographyConfig {
    /// Font size for the year
    #[builder(default = 24.0)]
    pub year_font_size: f32,
    /// Spacing before and after each year (spacing before the first year is ignored)
    #[builder(default = [8.0, 2.0])]
    pub year_spacing: [f32; 2],
    /// Spacing after each entry (spacing after the last entry is ignored)
    #[builder(default = 12.0)]
    pub entry_spacing: f32,
    /// Configuration for each bibliography entry
    #[builder(default)]
    pub entry_cfg: BibliographyEntryConfig,
}

impl Default for BibliographyConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[must_use = "You should call .show()"]
pub struct Bibliography {
    pub cfg: BibliographyConfig,
    bib: Vec<BibliographyEntry>,
}

impl Bibliography {
    pub fn len(&self) -> usize {
        self.bib.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &BibliographyEntry> {
        self.bib.iter()
    }

    pub fn parse(src: &str, thumbnails_dir: Option<&'static include_dir::Dir>) -> Self {
        Self::parse_with_cfg(BibliographyConfig::default(), src, thumbnails_dir)
    }

    pub fn parse_with_cfg(
        cfg: BibliographyConfig,
        src: &str,
        thumbnails_dir: Option<&'static include_dir::Dir>,
    ) -> Self {
        // Parse the bibliography
        let unsorted_bib = biblatex::Bibliography::parse(src).unwrap();

        // Sort and collect the bibliography entries with their thumbnails
        let bib = unsorted_bib
            .into_iter()
            .sorted_by_key(|entry| match entry.date().unwrap() {
                biblatex::PermissiveType::Typed(date) => match date.value {
                    biblatex::DateValue::At(date)
                    | biblatex::DateValue::After(date)
                    | biblatex::DateValue::Before(date)
                    | biblatex::DateValue::Between(date, _) => format!(
                        "{}{}{}",
                        date.year,
                        date.month.unwrap_or(0),
                        date.day.unwrap_or(0)
                    ),
                },
                biblatex::PermissiveType::Chunks(chunks) => chunks
                    .parse::<String>()
                    .unwrap()
                    .chars()
                    .filter(|c| c.is_numeric())
                    .collect::<String>(),
            })
            // Newest entries first
            .rev()
            .map(|entry| {
                let thumbnail = Self::get_thumbnail(thumbnails_dir, &entry.key);
                BibliographyEntry::new_with_cfg(cfg.entry_cfg, entry, thumbnail)
            })
            .collect();

        Self { cfg, bib }
    }

    pub fn show(&self, ui: &mut egui::Ui) {
        self.iter()
            .enumerate()
            .fold(0, |previous_entry_year, (i, entry)| {
                let year = entry.year().unwrap();
                if year != previous_entry_year {
                    // Add spacing before the year (except for the first year)
                    if i > 0 {
                        ui.add_space(self.cfg.year_spacing[0]);
                    }

                    // Add the year
                    crate::utils::egui::heading_sized(
                        ui,
                        year.to_string(),
                        self.cfg.year_font_size,
                    );

                    // Add spacing after the year
                    ui.add_space(self.cfg.year_spacing[1]);
                }

                // Show the entry
                entry.show(ui);

                // Add spacing between entries (except for the last one)
                if i < self.len() - 1 {
                    ui.add_space(self.cfg.entry_spacing);
                }

                year
            });
    }

    fn get_thumbnail(
        thumbnails_dir: Option<&'static include_dir::Dir>,
        key: &str,
    ) -> Option<egui::ImageSource<'static>> {
        thumbnails_dir?
            .files()
            .find(|file| file.path().file_stem().is_some_and(|stem| stem == key))
            .map(|file| egui::ImageSource::Bytes {
                uri: std::borrow::Cow::Owned(format!(
                    "bytes://{}",
                    file.path()
                        .file_name()
                        .unwrap_or_else(|| unreachable!())
                        .to_string_lossy()
                )),
                bytes: egui::load::Bytes::Static(file.contents()),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bib() {
        const BIB: &str = indoc::indoc! {"
            @inproceedings{orsula2022learning,
                title = {{Learning to Grasp on the Moon from 3D Octree Observations with Deep Reinforcement Learning}},
                author = {Andrej Orsula and Simon Bøgh and Miguel Olivares-Mendez and Carol Martinez},
                booktitle = {2022 IEEE/RSJ International Conference on Intelligent Robots and Systems (IROS)},
                year = {2022},
                organization = {IEEE},
                page = {4112--4119},
                repository = {https://github.com/AndrejOrsula/drl_grasping},
                article_pdf = {https://arxiv.org/pdf/2208.00818},
                publication_url = {https://ieeexplore.ieee.org/document/9981661},
            }
        "};

        let bibliography = Bibliography::parse(BIB, None);
        let entry = bibliography.iter().next().unwrap();

        assert_eq!(entry.title().unwrap(), "Learning to Grasp on the Moon from 3D Octree Observations with Deep Reinforcement Learning");
        assert_eq!(
            entry
                .author()
                .unwrap()
                .iter()
                .map(std::string::ToString::to_string)
                .collect_vec(),
            [
                "Andrej Orsula",
                "Simon Bøgh",
                "Miguel Olivares-Mendez",
                "Carol Martinez"
            ]
        );
        assert_eq!(entry.year().unwrap(), 2022);
        assert_eq!(
            entry.venue().unwrap(),
            "2022 IEEE/RSJ International Conference on Intelligent Robots and Systems (IROS)"
        );
        assert!(entry.homepage().is_err());
        assert_eq!(
            entry.publication_url().unwrap(),
            "https://ieeexplore.ieee.org/document/9981661"
        );
        assert_eq!(
            entry.article_pdf().unwrap(),
            "https://arxiv.org/pdf/2208.00818"
        );
        assert!(entry.article_html().is_err());
        assert_eq!(
            entry.repository().unwrap(),
            "https://github.com/AndrejOrsula/drl_grasping"
        );
    }
}
