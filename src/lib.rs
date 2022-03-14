#![doc=include_str!("../README.md")]

#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate fehler;

pub use config::Config;
pub use error::Error;
use rocket::{
    request::{FromRequest, Outcome},
    Request,
};
use std::{fmt::Display, hash::Hash, str::FromStr};
mod accept_language;
mod config;
mod error;
mod url;

macro_rules! language_impls {
    ($($upper:ident | $lower:ident | $english_name:literal | $native_name:literal )*) => {
        ///  code | enum variant | English name | Native name
        ///  ----|----|---|----
        $(#[doc = stringify!($lower | $upper | $english_name | $native_name)])*
        #[non_exhaustive]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
        pub enum LangCode {$(
            #[doc=stringify!($english_name)]
            $upper
        ),+}
        pub use LangCode::*;

        impl LangCode {
            /// A collection with all the values to
            /// iterate through them easily.
            pub const ALL_CODES: &'static [Self] = &[$(Self::$upper,)*];
            /// transforms the enum value to its lower case string representation.
            /// ```rust
            /// let spanish = Es.as_str();
            /// assert!(spanish == "es");
            /// ```
            ///
            pub fn as_str(self) -> &'static str {
                match self {
                    $(Self::$upper => stringify!($lower)),*
                }
            }
            /// Returns the name of the language in
            /// english
            /// ```rust
            /// let fr = Fr.english_name();
            /// assert!(fr == "French");
            /// ```
            pub fn english_name(self) -> &'static str {
                match self {
                    $(Self::$upper => $english_name,)*
                }
            }
            /// Returns the name of the language in
            /// its native name.
            /// ```rust
            /// let german = De.native_name();
            /// assert!(german == "Deutsch");
            ///
            /// ```
            pub fn native_name(self) -> &'static str {
                match self {
                    $(Self::$upper => $native_name,)*
                }
            }
        }

        impl FromStr for LangCode {
            type Err = Error;
            fn from_str(input: &str) -> Result<LangCode, Error> {
                match input {
                    $(stringify!($lower) => Ok(Self::$upper),)*
                    _ => Err(Error::NotAcceptable),
                }
            }
        }

        impl Display for LangCode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.as_str())
            }
        }


    }
}
impl TryFrom<&Request<'_>> for LangCode {
    type Error = Error;
    fn try_from(req: &Request) -> Result<LangCode, Error> {
        req.local_cache(|| accept_language::without_config(req))
            .clone()
    }
}

/// The language code value gets cached on construction,
/// so it is ok to construct it multiple times.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for LangCode {
    type Error = Error;
    /// if there is a config struct set, then
    /// the local_cache already has a closure for creating the value,
    /// so this one will get ignored.
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request
            .local_cache(|| accept_language::without_config(request))
            .clone()
        {
            Ok(lang) => Outcome::Success(lang),
            Err(err) => Outcome::Failure((err.status(), err)),
        }
    }
}

#[test]
fn foo() {
    let x = 1.0 / 0.0;
    println!("{}", x);
}

language_impls! {
Aa | aa  | "Afar"                | "Afaraf"
Ab | ab  | "Abkhaz"              | "аҧсуа бызшәа"
Af | af  | "Afrikaans"           | "Afrikaans"
Ak | ak  | "Akan"                | "Akan"
Sq | sq  | "Albanian"            | "Shqip"
Am | am  | "Amharic"             | "አማርኛ"
Ar | ar  | "Arabic"              | "العربية"
An | an  | "Aragonese"           | "aragonés"
Hy | hy  | "Armenian"            | "Հայերեն"
As | as  | "Assamese"            | "অসমীয়া"
Av | av  | "Avaric"              | "авар мацӀ"
Ae | ae  | "Avestan"             | "avesta"
Ay | ay  | "Aymara"              | "aymar aru"
Az | az  | "Azerbaijani"         | "azərbaycan dili"
Bm | bm  | "Bambara"             | "bamanankan"
Ba | ba  | "Bashkir"             | "башҡорт теле"
Eu | eu  | "Basque"              | "euskara"
Be | be  | "Belarusian"          | "беларуская мова"
Bn | bn  | "Bengali"             | "বাংলা"
Bh | bh  | "Bihari"              | "भोजपुरी"
Bi | bi  | "Bislama"             | "Bislama"
Bs | bs  | "Bosnian"             | "bosanski jezik"
Br | br  | "Breton"              | "brezhoneg"
Bg | bg  | "Bulgarian"           | "български език"
My | my  | "Burmese"             | "ဗမာစာ"
Ca | ca  | "Catalan"             | "català"
Ch | ch  | "Chamorro"            | "Chamoru"
Ce | ce  | "Chechen"             | "нохчийн мотт"
Ny | ny  | "Chichewa"            | "chiCheŵa"
Zh | zh  | "Chinese"             | "中文"
Cv | cv  | "Chuvash"             | "чӑваш чӗлхи"
Kw | kw  | "Cornish"             | "Kernewek"
Co | co  | "Corsican"            | "corsu"
Cr | cr  | "Cree"                | "ᓀᐦᐃᔭᐍᐏᐣ"
Hr | hr  | "Croatian"            | "hrvatski jezik"
Cs | cs  | "Czech"               | "čeština"
Da | da  | "Danish"              | "dansk"
Dv | dv  | "Divehi"              | "ދިވެހި"
Nl | nl  | "Dutch"               | "Nederlands"
Dz | dz  | "Dzongkha"            | "རྫོང་ཁ"
En | en  | "English"             | "English"
Eo | eo  | "Esperanto"           | "Esperanto"
Et | et  | "Estonian"            | "eesti"
Ee | ee  | "Ewe"                 | "Eʋegbe"
Fo | fo  | "Faroese"             | "føroyskt"
Fj | fj  | "Fijian"              | "vosa Vakaviti"
Fi | fi  | "Finnish"             | "suomi"
Fr | fr  | "French"              | "français"
Ff | ff  | "Fula"                | "Fulfulde"
Gl | gl  | "Galician"            | "galego"
Ka | ka  | "Georgian"            | "ქართული"
De | de  | "German"              | "Deutsch"
El | el  | "Greek"               | "ελληνικά"
Gn | gn  | "Guaraní"             | "Avañe'ẽ"
Gu | gu  | "Gujarati"            | "ગુજરાતી"
Ht | ht  | "Haitian"             | "Kreyòl ayisyen"
Ha | ha  | "Hausa"               | "(Hausa) هَوُسَ"
He | he  | "Hebrew"              | "עברית"
Hz | hz  | "Herero"              | "Otjiherero"
Hi | hi  | "Hindi"               | "हिन्दी"
Ho | ho  | "Hiri Motu"           | "Hiri Motu"
Hu | hu  | "Hungarian"           | "magyar"
Ia | ia  | "Interlingua"         | "Interlingua"
Id | id  | "Indonesian"          | "Bahasa Indonesia"
Ie | ie  | "Interlingue"         | "Interlingue"
Ga | ga  | "Irish"               | "Gaeilge"
Ig | ig  | "Igbo"                | "Asụsụ Igbo"
Ik | ik  | "Inupiaq"             | "Iñupiaq"
Io | io  | "Ido"                 | "Ido"
Is | is  | "Icelandic"           | "Íslenska"
It | it  | "Italian"             | "Italiano"
Iu | iu  | "Inuktitut"           | "ᐃᓄᒃᑎᑐᑦ"
Ja | ja  | "Japanese"            | "日本語 (にほんご)"
Jv | jv  | "Javanese"            | "ꦧꦱꦗꦮ"
Kl | kl  | "Kalaallisut"         | "kalaallisut"
Kn | kn  | "Kannada"             | "ಕನ್ನಡ"
Kr | kr  | "Kanuri"              | "Kanuri"
Ks | ks  | "Kashmiri"            | "कश्मीरी"
Kk | kk  | "Kazakh"              | "қазақ тілі"
Km | km  | "Khmer"               | "ខ្មែរ"
Ki | ki  | "Kikuyu"              | "Gĩkũyũ"
Rw | rw  | "Kinyarwanda"         | "Ikinyarwanda"
Ky | ky  | "Kyrgyz"              | "Кыргызча"
Kv | kv  | "Komi"                | "коми кыв"
Kg | kg  | "Kongo"               | "Kikongo"
Ko | ko  | "Korean"              | "한국어"
Ku | ku  | "Kurdish"             | "Kurdî"
Kj | kj  | "Kwanyama"            | "Kuanyama"
La | la  | "Latin"               | "lingua latina"
Lb | lb  | "Luxembourgish"       | "Lëtzebuergesch"
Lg | lg  | "Ganda"               | "Luganda"
Li | li  | "Limburgish"          | "Limburgs"
Ln | ln  | "Lingala"             | "Lingála"
Lo | lo  | "Lao"                 | "ພາສາລາວ"
Lt | lt  | "Lithuanian"          | "lietuvių kalba"
Lu | lu  | "Luba-Katanga"        | "Tshiluba"
Lv | lv  | "Latvian"             | "latviešu valoda"
Gv | gv  | "Manx"                | "Gaelg"
Mk | mk  | "Macedonian"          | "македонски јазик"
Mg | mg  | "Malagasy"            | "fiteny malagasy"
Ms | ms  | "Malay"               | "bahasa Melayu"
Ml | ml  | "Malayalam"           | "മലയാളം"
Mt | mt  | "Maltese"             | "Malti"
Mi | mi  | "Māori"               | "te reo Māori"
Mr | mr  | "Marathi"             | "मराठी"
Mh | mh  | "Marshallese"         | "Kajin M̧ajeļ"
Mn | mn  | "Mongolian"           | "Монгол хэл"
Na | na  | "Nauruan"             | "Dorerin Naoero"
Nv | nv  | "Navajo"              | "Diné bizaad"
Nd | nd  | "Northern Ndebele"    | "isiNdebele"
Ne | ne  | "Nepali"              | "नेपाली"
Ng | ng  | "Ndonga"              | "Owambo"
Nb | nb  | "Norwegian Bokmål"    | "Norsk bokmål"
Nn | nn  | "Norwegian Nynorsk"   | "Norsk nynorsk"
No | no  | "Norwegian"           | "Norsk"
Ii | ii  | "Nuosu"               | "ꆈꌠ꒿ Nuosuhxop"
Nr | nr  | "Southern Ndebele"    | "isiNdebele"
Oc | oc  | "Occitan"             | "occitan"
Oj | oj  | "Ojibwe"              | "ᐊᓂᔑᓈᐯᒧᐎᓐ"
Cu | cu  | "Old Church Slavonic" | "ѩзыкъ словѣньскъ"
Om | om  | "Oromo"               | "Afaan Oromoo"
Or | or  | "Oriya"               | "ଓଡ଼ିଆ"
Os | os  | "Ossetian"            | "ирон æвзаг"
Pa | pa  | "Punjabi"             | "ਪੰਜਾਬੀ"
Pi | pi  | "Pāli"                | "पाऴि"
Fa | fa  | "Persian"             | "فارسی"
Pl | pl  | "Polish"              | "język polski"
Ps | ps  | "Pashto"              | "پښتو"
Pt | pt  | "Portuguese"          | "Português"
Qu | qu  | "Quechua"             | "Runa Simi"
Rm | rm  | "Romansh"             | "rumantsch grischun"
Rn | rn  | "Kirundi"             | "Ikirundi"
Ro | ro  | "Romanian"            | "Română"
Ru | ru  | "Russian"             | "Русский"
Sa | sa  | "Sanskrit"            | "संस्कृतम्"
Sc | sc  | "Sardinian"           | "sardu"
Sd | sd  | "Sindhi"              | "सिन्धी"
Se | se  | "Northern Sami"       | "Davvisámegiella"
Sm | sm  | "Samoan"              | "gagana fa'a Samoa"
Sg | sg  | "Sango"               | "yângâ tî sängö"
Sr | sr  | "Serbian"             | "српски језик"
Gd | gd  | "Gaelic"              | "Gàidhlig"
Sn | sn  | "Shona"               | "chiShona"
Si | si  | "Sinhalese"           | "සිංහල"
Sk | sk  | "Slovak"              | "slovenčina"
Sl | sl  | "Slovene"             | "slovenski jezik"
So | so  | "Somali"              | "Soomaaliga"
St | st  | "Southern Sotho"      | "Sesotho"
Es | es  | "Spanish"             | "Español"
Su | su  | "Sundanese"           | "Basa Sunda"
Sw | sw  | "Swahili"             | "Kiswahili"
Ss | ss  | "Swati"               | "SiSwati"
Sv | sv  | "Swedish"             | "svenska"
Ta | ta  | "Tamil"               | "தமிழ்"
Te | te  | "Telugu"              | "తెలుగు"
Tg | tg  | "Tajik"               | "тоҷикӣ"
Th | th  | "Thai"                | "ไทย"
Ti | ti  | "Tigrinya"            | "ትግርኛ"
Bo | bo  | "Tibetan"             | "བོད་ཡིག"
Tk | tk  | "Turkmen"             | "Türkmen"
Tl | tl  | "Tagalog"             | "Wikang Tagalog"
Tn | tn  | "Tswana"              | "Setswana"
To | to  | "Tonga"               | "faka Tonga"
Tr | tr  | "Turkish"             | "Türkçe"
Ts | ts  | "Tsonga"              | "Xitsonga"
Tt | tt  | "Tatar"               | "татар теле"
Tw | tw  | "Twi"                 | "Twi"
Ty | ty  | "Tahitian"            | "Reo Tahiti"
Ug | ug  | "Uyghur"              | "ئۇيغۇرچە‎"
Uk | uk  | "Ukrainian"           | "Українська"
Ur | ur  | "Urdu"                | "اردو"
Uz | uz  | "Uzbek"               | "Oʻzbek"
Ve | ve  | "Venda"               | "Tshivenḓa"
Vi | vi  | "Vietnamese"          | "Tiếng Việt"
Vo | vo  | "Volapük"             | "Volapük"
Wa | wa  | "Walloon"             | "walon"
Cy | cy  | "Welsh"               | "Cymraeg"
Wo | wo  | "Wolof"               | "Wollof"
Fy | fy  | "Western Frisian"     | "Frysk"
Xh | xh  | "Xhosa"               | "isiXhosa"
Yi | yi  | "Yiddish"             | "ייִדיש"
Yo | yo  | "Yoruba"              | "Yorùbá"
Za | za  | "Zhuang"              | "Saɯ cueŋƅ"
Zu | zu  | "Zulu"                | "isiZulu"
}
