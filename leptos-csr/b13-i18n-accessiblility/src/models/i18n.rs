use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;

pub fn use_lang() -> RwSignal<Lang> {
    use_context::<RwSignal<Lang>>().expect("language context")
}

pub fn initial_language() -> Lang {
    let saved: Option<String> = LocalStorage::get(STORAGE_KEY).ok();
    let browser = web_sys::window().and_then(|window| window.navigator().language());
    pick_language(saved.as_deref(), browser.as_deref())
}

pub fn t(lang: Lang, key: &'static str) -> &'static str {
    lookup(lang, key).unwrap_or(key)
}

pub const STORAGE_KEY: &str = "course_lang";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Lang {
    En,
    Es,
    Hi,
    Ar,
}

impl Lang {
    pub const ALL: [Lang; 4] = [Lang::En, Lang::Es, Lang::Hi, Lang::Ar];

    pub fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Es => "es",
            Lang::Hi => "hi",
            Lang::Ar => "ar",
        }
    }

    pub fn dir(self) -> &'static str {
        match self {
            Lang::Ar => "rtl",
            _ => "ltr",
        }
    }

    pub fn native_name(self) -> &'static str {
        match self {
            Lang::En => "English",
            Lang::Es => "Español",
            Lang::Hi => "हिन्दी",
            Lang::Ar => "العربية",
        }
    }

    fn from_tag(tag: &str) -> Option<Lang> {
        let primary = tag.split(['-', '_']).next()?.to_ascii_lowercase();
        Lang::ALL.into_iter().find(|lang| lang.code() == primary)
    }
}

fn pick_language(saved: Option<&str>, browser: Option<&str>) -> Lang {
    saved
        .and_then(Lang::from_tag)
        .or_else(|| browser.and_then(Lang::from_tag))
        .unwrap_or(Lang::En)
}

fn lookup(lang: Lang, key: &str) -> Option<&'static str> {
    table(lang)
        .iter()
        .find(|(name, _)| *name == key)
        .map(|(_, message)| *message)
}

type Table = &'static [(&'static str, &'static str)];

const EN: Table = &[
    ("title", "i18n and Accessibility"),
    ("greeting", "Welcome to the course"),
    ("checkout", "Checkout"),
    ("save", "Save settings"),
    ("alert", "Your settings were saved"),
    ("name", "Name"),
    ("submit", "Submit"),
    ("direction", "Reading direction"),
    ("direction_note", "This text reads from left to right"),
];

const ES: Table = &[
    ("title", "i18n y Accesibilidad"),
    ("greeting", "Bienvenido al curso"),
    ("checkout", "Pagar"),
    ("save", "Guardar ajustes"),
    ("alert", "Tus ajustes fueron guardados"),
    ("name", "Nombre"),
    ("submit", "Enviar"),
    ("direction", "Dirección de lectura"),
    ("direction_note", "Este texto se lee de izquierda a derecha"),
];

const HI: Table = &[
    ("title", "अंतरराष्ट्रीयकरण और पहुंच"),
    ("greeting", "कोर्स में आपका स्वागत है"),
    ("checkout", "चेकआउट"),
    ("save", "सेटिंग सहेजें"),
    ("alert", "आपकी सेटिंग सहेज दी गई"),
    ("name", "नाम"),
    ("submit", "जमा करें"),
    ("direction", "पढ़ने की दिशा"),
    ("direction_note", "यह पाठ बाएं से दाएं पढ़ा जाता है"),
];

const AR: Table = &[
    ("title", "التدويل وإمكانية الوصول"),
    ("greeting", "مرحبًا بك في الدورة"),
    ("checkout", "الدفع"),
    ("save", "حفظ الإعدادات"),
    ("alert", "تم حفظ إعداداتك"),
    ("name", "الاسم"),
    ("submit", "إرسال"),
    ("direction", "اتجاه القراءة"),
    ("direction_note", "هذا النص يُقرأ من اليمين إلى اليسار"),
];

fn table(lang: Lang) -> Table {
    match lang {
        Lang::En => EN,
        Lang::Es => ES,
        Lang::Hi => HI,
        Lang::Ar => AR,
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn every_language_has_every_key() {
        for lang in Lang::ALL {
            for (key, _) in EN {
                let message = lookup(lang, key);
                assert!(message.is_some(), "{lang:?} is missing {key}");
                assert!(
                    !message.unwrap().trim().is_empty(),
                    "{lang:?} has an empty {key}"
                );
            }
        }
    }

    #[test]
    fn no_language_has_extra_or_duplicate_keys() {
        for lang in Lang::ALL {
            let keys: Vec<&str> = table(lang).iter().map(|(key, _)| *key).collect();
            let mut sorted = keys.clone();
            sorted.sort();
            sorted.dedup();
            assert_eq!(sorted.len(), keys.len(), "{lang:?} repeats a key");
            assert_eq!(
                keys.len(),
                EN.len(),
                "{lang:?} has a different number of keys"
            );
        }
    }

    #[test]
    fn only_arabic_is_right_to_left() {
        assert_eq!(Lang::Ar.dir(), "rtl");
        for lang in [Lang::En, Lang::Es, Lang::Hi] {
            assert_eq!(lang.dir(), "ltr");
        }
    }

    #[test]
    fn language_codes_round_trip() {
        for lang in Lang::ALL {
            assert_eq!(Lang::from_tag(lang.code()), Some(lang));
        }
    }

    #[test]
    fn regional_tags_map_to_the_language() {
        assert_eq!(Lang::from_tag("hi-IN"), Some(Lang::Hi));
        assert_eq!(Lang::from_tag("es_MX"), Some(Lang::Es));
        assert_eq!(Lang::from_tag("AR-EG"), Some(Lang::Ar));
        assert_eq!(Lang::from_tag("fr-FR"), None);
        assert_eq!(Lang::from_tag(""), None);
    }

    #[test]
    fn starting_language_prefers_saved_then_browser_then_english() {
        assert_eq!(pick_language(Some("hi"), Some("es-ES")), Lang::Hi);
        assert_eq!(pick_language(None, Some("es-ES")), Lang::Es);
        assert_eq!(pick_language(Some("klingon"), Some("ar")), Lang::Ar);
        assert_eq!(pick_language(None, Some("de")), Lang::En);
        assert_eq!(pick_language(None, None), Lang::En);
    }

    #[test]
    fn a_missing_key_shows_the_key() {
        assert_eq!(t(Lang::En, "does_not_exist"), "does_not_exist");
    }
}
