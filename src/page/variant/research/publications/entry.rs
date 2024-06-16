use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, typed_builder::TypedBuilder)]
pub struct BibliographyEntryConfig {
    #[builder(default = 110.0)]
    pub thumbnail_size: f32,
    #[builder(default = 4.0)]
    pub thumbnail_rounding: f32,
    #[builder(default = 20.0)]
    pub title_font_size: f32,
    #[builder(default = 120.0)]
    pub min_spacing_per_author: f32,
    #[builder(default = 6.0)]
    pub button_spacing: f32,
    #[builder(default = 26.0)]
    pub button_size: f32,
}

impl Default for BibliographyEntryConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[must_use = "You should call .show()"]
pub struct BibliographyEntry {
    pub cfg: BibliographyEntryConfig,
    entry: biblatex::Entry,
    thumbnail: Option<egui::ImageSource<'static>>,
}

impl std::ops::Deref for BibliographyEntry {
    type Target = biblatex::Entry;
    fn deref(&self) -> &Self::Target {
        &self.entry
    }
}

impl BibliographyEntry {
    pub fn new_with_cfg(
        cfg: BibliographyEntryConfig,
        entry: biblatex::Entry,
        thumbnail: Option<egui::ImageSource<'static>>,
    ) -> Self {
        Self {
            cfg,
            entry,
            thumbnail,
        }
    }

    pub fn show(&self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                self.show_thumbnail(ui);
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    self.show_title(ui);
                    self.show_authors(ui);
                    self.show_venue(ui);
                    self.show_buttons(ui);
                });
            });
        });
    }

    fn show_thumbnail(&self, ui: &mut egui::Ui) {
        if let Some(thumbnail) = &self.thumbnail {
            let image = ui.add(
                egui::Image::new(thumbnail.clone())
                    .sense(egui::Sense::click())
                    .rounding(self.cfg.thumbnail_rounding)
                    .fit_to_exact_size(egui::Vec2::new(
                        self.cfg.thumbnail_size,
                        self.cfg.thumbnail_size,
                    )),
            );
            self.maybe_clickable_homepage_or_publication_url(image);
        } else {
            ui.add_space(self.cfg.thumbnail_size + ui.spacing().item_spacing.x);
        }
    }

    fn show_title(&self, ui: &mut egui::Ui) {
        let title = crate::utils::egui::strong_heading_sized(
            ui,
            self.title().unwrap(),
            self.cfg.title_font_size,
        );
        self.maybe_clickable_homepage_or_publication_url(title);
    }

    fn show_authors(&self, ui: &mut egui::Ui) {
        let authors = self.author().unwrap();
        let highlighted_name_index = authors
            .iter()
            .position(|author| {
                author.given_name.starts_with(crate::AUTHOR_INITIALS[0])
                    && author.name == crate::AUTHOR_SURNAME
            })
            .unwrap();

        ui.with_layout(
            egui::Layout::left_to_right(egui::Align::TOP).with_main_wrap(true),
            |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                match authors.len() {
                    1 => {
                        ui.label(egui::RichText::new(crate::AUTHOR_NAME_SHORT).strong());
                    }
                    _list_all
                        if ui.ctx().available_rect().width() - self.cfg.thumbnail_size
                            > authors.len() as f32 * self.cfg.min_spacing_per_author =>
                    {
                        // Names before the highlighted author
                        if highlighted_name_index > 0 {
                            ui.label(egui::RichText::new(format!(
                                "{}, ",
                                authors
                                    .iter()
                                    .take(highlighted_name_index)
                                    .map(|author| format!(
                                        "{} {}",
                                        author.given_name.chars().next().unwrap(),
                                        author.name
                                    ))
                                    .join(", "),
                            )));
                        }

                        // Highlighted author
                        ui.label(egui::RichText::new(crate::AUTHOR_NAME_SHORT).strong());

                        // Names after the highlighted author
                        if authors.len() > highlighted_name_index + 1 {
                            ui.label(egui::RichText::new(format!(
                                ", {}",
                                authors
                                    .iter()
                                    .skip(highlighted_name_index + 1)
                                    .map(|author| format!(
                                        "{} {}",
                                        author.given_name.chars().next().unwrap(),
                                        author.name
                                    ))
                                    .join(", "),
                            )));
                        }
                    }
                    _et_al => {
                        let first_author = if highlighted_name_index == 0 {
                            ui.label(
                                egui::RichText::new(format!("{} et al.", crate::AUTHOR_NAME_SHORT))
                                    .strong(),
                            )
                        } else {
                            ui.label(format!(
                                "{} {} et al.",
                                authors[0].given_name.chars().next().unwrap(),
                                authors[0].name
                            ))
                        };

                        // List the remaining authors on hover
                        first_author.on_hover_ui(|ui| {
                            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                                authors.iter().skip(1).enumerate().for_each(|(i, author)| {
                                    if highlighted_name_index == i + 1 {
                                        ui.label(
                                            egui::RichText::new(crate::AUTHOR_NAME_SHORT).strong(),
                                        );
                                    } else {
                                        ui.label(format!(
                                            "{} {}",
                                            author.given_name.chars().next().unwrap(),
                                            author.name
                                        ));
                                    }
                                });
                            });
                        });
                    }
                }
            },
        );
    }

    fn show_venue(&self, ui: &mut egui::Ui) {
        let venue = match self.venue().unwrap() {
            abbreviated if abbreviated.ends_with(')') => {
                format!(
                    "{} {}",
                    abbreviated
                        .split('(')
                        .last()
                        .unwrap()
                        .split(')')
                        .next()
                        .unwrap()
                        .to_owned(),
                    self.year().unwrap()
                )
            }
            arxiv_preprint if arxiv_preprint.to_lowercase().contains("arxiv") => {
                "arXiv preprint".to_string()
            }
            preprint if preprint.to_lowercase().contains("preprint") => "Preprint".to_string(),
            other => other,
        };
        ui.label(egui::RichText::new(venue).weak());
    }

    fn show_buttons(&self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.spacing_mut().item_spacing.x = self.cfg.button_spacing;

            // Simple hyperlink buttons
            macro_rules! maybe_show_simple_button {
                ($link:expr, $symbol:expr, $hover_text:expr) => {
                    if let Ok(link) = $link {
                        let button = ui
                            .add(egui::Button::new(
                                egui::RichText::new($symbol).size(self.cfg.button_size),
                            ))
                            .on_hover_text($hover_text);
                        crate::utils::egui::clickable_url(button, link);
                    }
                };
            }
            maybe_show_simple_button!(self.homepage(), "\u{e80b}", "Homepage");
            maybe_show_simple_button!(self.article_pdf(), "\u{e415}", "Article (PDF)");
            maybe_show_simple_button!(self.article_html(), "\u{eb7e}", "Article (HTML)");
            maybe_show_simple_button!(self.presentation(), "\u{eaf0}", "Full presentation");
            maybe_show_simple_button!(self.video(), "\u{f06a}", "Video summary");
            maybe_show_simple_button!(self.repository(), "\u{e86f}", "Source code");

            // Separator before the copy button
            ui.add(
                egui::Label::new(egui::RichText::new("|").size(self.cfg.button_size).weak())
                    .selectable(false),
            );

            self.show_copy_entry_button(ui);
        });
    }

    fn show_copy_entry_button(&self, ui: &mut egui::Ui) {
        let persistent_id = ui.make_persistent_id(self.key.clone());
        let is_copied =
            ui.memory_mut(|mem| *mem.data.get_temp_mut_or_default::<bool>(persistent_id));

        let button = ui
            .add(egui::Button::new(
                egui::RichText::new(if is_copied { "\u{e5ca}" } else { "\u{e609}" })
                    .size(self.cfg.button_size),
            ))
            .on_hover_text(if is_copied {
                "Copied!"
            } else {
                "Copy BibTeX entry"
            });

        if button.clicked() {
            // Mark the entry as copied
            ui.memory_mut(|m| *m.data.get_temp_mut_or_default(persistent_id) = true);

            // Create a clone of the entry without custom fields
            let mut raw_entry = self.entry.clone();
            for field in CUSTOM_BIB_FIELDS {
                raw_entry.remove(field);
            }

            // Copy the entry to the clipboard
            ui.ctx().copy_text(
                raw_entry
                    .to_bibtex_string()
                    .unwrap_or_else(|_| raw_entry.to_biblatex_string()),
            );
        } else if is_copied && !button.hovered() {
            // Unmark the entry as copied
            ui.memory_mut(|m| *m.data.get_temp_mut_or_default(persistent_id) = false);
        }
    }

    fn maybe_clickable_homepage_or_publication_url(
        &self,
        mut response: egui::Response,
    ) -> egui::Response {
        if let Ok(homepage) = self.homepage() {
            response = response
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .on_hover_text("Homepage");
            crate::utils::egui::clickable_url(response, homepage)
        } else if let Ok(publication_url) = self.publication_url() {
            response = response
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .on_hover_text("Publication permalink");
            crate::utils::egui::clickable_url(response, publication_url)
        } else {
            response
        }
    }
}

macro_rules! fields {
    ($($name:ident: $field:expr => $ret:ty),* $(,)*) => {
        $(::paste::paste! {
            #[doc = "Get the `" $field "` field."]
            pub fn $name(&self) -> ::std::result::Result<$ret, ::biblatex::RetrievalError> {
                ::biblatex::ChunksExt::parse::<$ret>(
                    self.get($field)
                        .ok_or_else(|| ::biblatex::RetrievalError::Missing($field.to_string()))?
                ).map_err(::std::convert::Into::into)
            }
        })*
    };
}
macro_rules! alias_fields {
    ($($name:ident: $field:literal $(| $alias:literal)+ => $ret:ty),* $(,)*) => {
        $(::paste::paste! {
            #[doc = "Get the `" $field "` field or fall back on any of its aliases: " $("`" $alias)"`|"+ "`."]
            pub fn $name(&self) -> ::std::result::Result<$ret, ::biblatex::RetrievalError> {
                ::biblatex::ChunksExt::parse::<$ret>(
                self.get($field)
                    $(.or_else(|| self.get($alias)))+
                    .ok_or_else(|| ::biblatex::RetrievalError::Missing($field.to_string()))?
                ).map_err(::std::convert::Into::into)
            }
        })*
    };
}

impl BibliographyEntry {
    pub fn year(&self) -> Result<i32, biblatex::RetrievalError> {
        match self.date()? {
            biblatex::PermissiveType::Typed(date) => match date.value {
                biblatex::DateValue::At(date)
                | biblatex::DateValue::After(date)
                | biblatex::DateValue::Before(date)
                | biblatex::DateValue::Between(date, _) => Ok(date.year),
            },
            biblatex::PermissiveType::Chunks(_chunks) => unimplemented!(),
        }
    }

    fields! {
        title: "title" => String,
    }

    alias_fields! {
        // Alias that covers all possible fields for publication type/venue/journal
        venue: "booktitle" | "type" | "journal" | "journaltitle" => String,
        // Custom fields
        homepage: "homepage" | "website" => String,
        publication_url: "publication_url" | "url" => String,
        article_pdf: "article" | "article_pdf" | "pdf" => String,
        article_html: "article_html" | "html" => String,
        video: "video" | "short_video" => String,
        presentation: "presentation" | "video_presentation" => String,
        repository: "repository" | "code" => String,
    }
}

const CUSTOM_BIB_FIELDS: &[&str] = &[
    "homepage",
    "website",
    "publication_url",
    "url",
    "article",
    "article_pdf",
    "pdf",
    "article_html",
    "html",
    "video",
    "short_video",
    "presentation",
    "video_presentation",
    "repository",
    "code",
];
