use std::fmt;

/// Rendered HTML plus optional template metadata for slots and fragments.
///
/// Most values are created with [`markup!`](crate::markup). `Markup` is escaped
/// by default when rendered from strings; use [`Markup::raw`] only for trusted
/// HTML that should be inserted as-is.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Markup {
    html: String,
    template: Option<Vec<MarkupPart>>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum MarkupPart {
    Html(String),
    Children,
    Fragment {
        name: String,
        parts: Vec<MarkupPart>,
    },
}

impl MarkupPart {
    fn accepts_children(&self) -> bool {
        match self {
            Self::Html(_) => false,
            Self::Children => true,
            Self::Fragment { parts, .. } => parts.iter().any(Self::accepts_children),
        }
    }

    fn html_len(&self) -> usize {
        match self {
            Self::Html(html) => html.len(),
            Self::Children => 0,
            Self::Fragment { parts, .. } => parts.iter().map(Self::html_len).sum(),
        }
    }

    fn append_with_children(&self, children: &Markup, out: &mut Vec<Self>) {
        match self {
            Self::Html(_) => push_markup_part(out, self.clone()),
            Self::Children => children.append_parts_to(out),
            Self::Fragment { name, parts } => {
                let mut fragment_parts = Vec::new();
                append_parts_with_children(parts, children, &mut fragment_parts);
                push_markup_part(
                    out,
                    Self::Fragment {
                        name: name.clone(),
                        parts: fragment_parts,
                    },
                );
            }
        }
    }

    fn find_fragment_parts<'a>(&'a self, target_name: &str) -> Option<&'a [Self]> {
        let Self::Fragment { name, parts } = self else {
            return None;
        };

        if name == target_name {
            return Some(parts);
        }

        parts
            .iter()
            .find_map(|part| part.find_fragment_parts(target_name))
    }
}

impl Default for Markup {
    fn default() -> Self {
        Self::new()
    }
}

impl Markup {
    /// Creates empty markup.
    #[must_use]
    pub fn new() -> Self {
        Self::raw(String::new())
    }

    /// Creates markup from trusted raw HTML without escaping.
    ///
    /// This is intentionally explicit because it bypasses the default escaping
    /// used by `markup!`, `Render for str`, and attribute rendering.
    #[must_use]
    pub fn raw(html: impl Into<String>) -> Self {
        Self {
            html: html.into(),
            template: None,
        }
    }

    /// Creates a slot placeholder that accepts children later.
    #[must_use]
    pub fn slot() -> Self {
        Self::from_parts(vec![MarkupPart::Children])
    }

    /// Creates a named fragment placeholder.
    ///
    /// The fragment body is supplied by the children rendered at the call site.
    #[must_use]
    pub fn fragment(name: impl Into<String>) -> Self {
        Self::from_parts(vec![MarkupPart::Fragment {
            name: name.into(),
            parts: vec![MarkupPart::Children],
        }])
    }

    /// Returns the HTML5 doctype.
    #[must_use]
    pub fn doctype() -> Self {
        Self::raw("<!DOCTYPE html>")
    }

    /// Returns the rendered HTML string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.html
    }

    /// Consumes this markup and returns the rendered HTML string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.html
    }

    /// Returns `true` when this markup has no rendered HTML and no template
    /// metadata.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.html.is_empty() && self.template.is_none()
    }

    /// Returns `true` when this markup contains a slot directly or inside a
    /// fragment.
    #[must_use]
    pub fn accepts_children(&self) -> bool {
        self.template
            .as_deref()
            .is_some_and(|parts| parts.iter().any(MarkupPart::accepts_children))
    }

    /// Renders this markup with children inserted into its slots.
    ///
    /// Panics when the markup does not contain a slot.
    #[must_use]
    pub fn with_children(&self, children: impl Into<Markup>) -> Self {
        let template = self
            .template
            .as_deref()
            .filter(|parts| parts.iter().any(MarkupPart::accepts_children))
            .unwrap_or_else(|| panic!("cannot render children into markup without a slot"));

        let children = children.into();
        let mut parts = Vec::new();
        append_parts_with_children(template, &children, &mut parts);

        Self::from_parts(parts)
    }

    /// Renders the named fragment.
    ///
    /// Panics when the fragment is missing. Use [`try_render_fragment`](Self::try_render_fragment)
    /// when absence is expected.
    #[must_use]
    pub fn render_fragment(&self, name: &str) -> Self {
        self.try_render_fragment(name)
            .unwrap_or_else(|| panic!("fragment not found: {name}"))
    }

    /// Attempts to render the named fragment.
    #[must_use]
    pub fn try_render_fragment(&self, name: &str) -> Option<Self> {
        let template = self.template.as_deref()?;
        template
            .iter()
            .find_map(|part| part.find_fragment_parts(name))
            .map(|parts| Self::from_parts(parts.to_vec()))
    }

    pub(crate) fn from_parts(parts: Vec<MarkupPart>) -> Self {
        let html_len = parts.iter().map(MarkupPart::html_len).sum();
        let mut html = String::with_capacity(html_len);
        let mut normalized = Vec::with_capacity(parts.len());
        // Only set when a Children or Fragment part is seen; plain HTML skips template storage.
        let mut has_template_parts = false;

        for part in parts {
            match part {
                MarkupPart::Html(part_html) => {
                    if part_html.is_empty() {
                        continue;
                    }
                    html.push_str(&part_html);
                    push_markup_part(&mut normalized, MarkupPart::Html(part_html));
                }
                MarkupPart::Children => {
                    has_template_parts = true;
                    normalized.push(MarkupPart::Children);
                }
                MarkupPart::Fragment {
                    name,
                    parts: fragment_parts,
                } => {
                    has_template_parts = true;
                    let fragment = Self::from_parts(fragment_parts);
                    html.push_str(&fragment.html);
                    push_markup_part(
                        &mut normalized,
                        MarkupPart::Fragment {
                            name,
                            parts: fragment.into_parts(),
                        },
                    );
                }
            }
        }

        if has_template_parts {
            Self {
                html,
                template: Some(normalized),
            }
        } else {
            Self::raw(html)
        }
    }

    pub(crate) fn append_parts_to(&self, parts: &mut Vec<MarkupPart>) {
        if let Some(template) = self.template.as_ref() {
            for part in template {
                push_markup_part(parts, part.clone());
            }
        } else {
            push_markup_part(parts, MarkupPart::Html(self.html.clone()));
        }
    }

    fn into_parts(self) -> Vec<MarkupPart> {
        if let Some(template) = self.template {
            template
        } else if self.html.is_empty() {
            Vec::new()
        } else {
            vec![MarkupPart::Html(self.html)]
        }
    }
}

// Merges adjacent Html strings so part-list length stays O(template structure), not O(render calls).
pub(crate) fn push_markup_part(parts: &mut Vec<MarkupPart>, part: MarkupPart) {
    match part {
        MarkupPart::Html(html) if html.is_empty() => {}
        MarkupPart::Html(html) => {
            if let Some(MarkupPart::Html(last)) = parts.last_mut() {
                last.push_str(&html);
            } else {
                parts.push(MarkupPart::Html(html));
            }
        }
        part => parts.push(part),
    }
}

fn append_parts_with_children(parts: &[MarkupPart], children: &Markup, out: &mut Vec<MarkupPart>) {
    for part in parts {
        part.append_with_children(children, out);
    }
}

impl fmt::Display for Markup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for Markup {
    fn into_response(self) -> axum::response::Response {
        (
            [(
                axum::http::header::CONTENT_TYPE,
                "text/html; charset=utf-8",
            )],
            self.into_string(),
        )
            .into_response()
    }
}

#[cfg(feature = "actix-web")]
impl actix_web::Responder for Markup {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(self.into_string())
    }
}

#[cfg(feature = "rocket")]
impl<'r> rocket::response::Responder<'r, 'static> for Markup {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let html = self.into_string();
        rocket::Response::build()
            .header(rocket::http::ContentType::HTML)
            .sized_body(html.len(), std::io::Cursor::new(html))
            .ok()
    }
}
