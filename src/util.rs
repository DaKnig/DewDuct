use crate::cache::DewCache;
use once_cell::sync::OnceCell;
use std::path::{Path, PathBuf};

pub fn format_semi_engineering(value: f32) -> String {
    static SUFFIXES: [char; 5] = [' ', 'k', 'M', 'B', 'T'];
    let suffix = (0..)
        .map(|x| 1000f32.powi(x) as f32)
        .zip(SUFFIXES)
        .filter(|x| value > x.0)
        .last()
        .unwrap();

    // explain with an example: value = 15942
    let normalized = value / suffix.0; // normalized = 15.942
    let exp = suffix.1; // exp = 'k'

    let mut ret = format!("{}", normalized as u16);
    if normalized < 10. {
        ret += &format!(".{}", ((normalized % 1.) * 10.) as u8);
    }

    ret.push(exp);
    ret += " views";

    ret
}

pub(crate) fn cache() -> &'static DewCache {
    static APP_CACHE: OnceCell<DewCache> = OnceCell::new();
    &APP_CACHE.get_or_init(|| DewCache::default())
}

pub(crate) fn cache_dir(fname: &Path) -> PathBuf {
    // let mut fname = cache().dir().clone().into();
    let mut dir = cache().dir().clone();
    dir.push(fname);
    // fname.push(&vid_data.id);
    // fname.push(&thumb.quality);
    // fname.set_extension("jpg");
    dir
}
