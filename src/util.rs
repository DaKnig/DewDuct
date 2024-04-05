use crate::cache::DewCache;
use humantime::format_duration;
use once_cell::sync::OnceCell;
use std::path::{Path, PathBuf};
use std::time::Duration;

pub(crate) fn format_rel_time(duration: Duration) -> String {
    let mut s: String = format_duration(duration).to_string();
    s = s.split_whitespace().next().unwrap().to_owned();
    s
}

pub fn format_semi_engineering(value: f32) -> String {
    static SUFFIXES: [char; 5] = [' ', 'k', 'M', 'B', 'T'];
    let Some(suffix) = (0..)
        .map(|x| 1000f32.powi(x))
        .zip(SUFFIXES)
        .filter(|x| value >= x.0 || x.1 == ' ')
        .last()
    else {
        gtk::glib::g_warning!("DewUtil", "wtf: cant format value {value}");
        return "".into();
    };

    // explain with an example: value = 15942
    let normalized = value / suffix.0; // normalized = 15.942
    let exp = suffix.1; // exp = 'k'

    let mut ret = format!("{}", normalized as u16);
    if normalized < 10. && value > 10. {
        ret += &format!(".{}", ((normalized % 1.) * 10.) as u8);
    }

    ret.push(exp);

    ret
}

pub(crate) fn cache() -> &'static DewCache {
    static APP_CACHE: OnceCell<DewCache> = OnceCell::new();
    APP_CACHE.get_or_init(DewCache::default)
}

pub(crate) fn cache_dir(fname: &Path) -> PathBuf {
    let mut dir = cache().dir().clone();
    dir.push(fname);
    dir
}
