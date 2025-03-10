use appstream::{
    enums::{Icon, Launchable},
    Component,
};

fn get_translatable<'a>(translatable: &'a appstream::TranslatableString, locale: &str) -> &'a str {
    match translatable.get_for_locale(locale) {
        Some(some) => some.as_str(),
        None => match translatable.get_default() {
            Some(some) => some.as_str(),
            None => "",
        },
    }
}

/*TODO: handle p tags with xml:lang
fn get_markup_translatable<'a>(
    translatable: &'a appstream::MarkupTranslatableString,
    locale: &str,
) -> &'a str {
    match translatable.get_for_locale(locale) {
        Some(some) => some.as_str(),
        None => match translatable.get_default() {
            Some(some) => some.as_str(),
            None => "",
        },
    }
}
*/

// Replaced Icon due to skip_field not supported in bitcode
#[derive(Debug, bitcode::Decode, bitcode::Encode)]
pub enum AppIcon {
    Cached(String, Option<u32>, Option<u32>, Option<u32>),
    Stock(String),
}

#[derive(Debug, bitcode::Decode, bitcode::Encode)]
pub struct AppInfo {
    pub origin_opt: Option<String>,
    pub name: String,
    pub summary: String,
    pub pkgname: Option<String>,
    pub icons: Vec<AppIcon>,
    pub desktop_ids: Vec<String>,
}

impl AppInfo {
    pub fn new(origin_opt: Option<&str>, component: Component, locale: &str) -> Self {
        let name = get_translatable(&component.name, locale);
        let summary = component
            .summary
            .as_ref()
            .map_or("", |x| get_translatable(x, locale));
        /*TODO: MarkupTranslatableString doesn't properly filter p tag with xml:lang
        if let Some(description) = &component.description {
            column = column.push(widget::text(get_markup_translatable(
                description,
                &self.locale,
            )));
        }
        */
        let icons = component
            .icons
            .into_iter()
            .filter_map(|icon| match icon {
                Icon::Cached {
                    path,
                    width,
                    height,
                    scale,
                } => Some(AppIcon::Cached(
                    path.to_str()?.to_string(),
                    width,
                    height,
                    scale,
                )),
                Icon::Stock(path) => Some(AppIcon::Stock(path)),
                _ => None,
            })
            .collect();
        let desktop_ids = component
            .launchables
            .into_iter()
            .filter_map(|launchable| match launchable {
                Launchable::DesktopId(desktop_id) => Some(desktop_id),
                _ => None,
            })
            .collect();
        Self {
            origin_opt: origin_opt.map(|x| x.to_string()),
            name: name.to_string(),
            summary: summary.to_string(),
            pkgname: component.pkgname,
            icons,
            desktop_ids,
        }
    }
}
