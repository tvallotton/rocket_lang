#![doc=include_str!("../README.md")]

#[macro_use]
extern crate thiserror;


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
        req.local_cache(|| Ok::<_, Error>(En))
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
            .local_cache(|| Ok::<_, Error>(En))
            .clone()
        {
            Ok(lang) => Outcome::Success(lang),
            Err(err) => Outcome::Failure((err.status(), err)),
        }
    }
}



language_impls! {
Aa | aa  | "Afar"                | "Afaraf"
Ab | ab  | "Abkhaz"              | "?????????? ????????????"
Af | af  | "Afrikaans"           | "Afrikaans"
Ak | ak  | "Akan"                | "Akan"
Sq | sq  | "Albanian"            | "Shqip"
Am | am  | "Amharic"             | "????????????"
Ar | ar  | "Arabic"              | "??????????????"
An | an  | "Aragonese"           | "aragon??s"
Hy | hy  | "Armenian"            | "??????????????"
As | as  | "Assamese"            | "?????????????????????"
Av | av  | "Avaric"              | "???????? ????????"
Ae | ae  | "Avestan"             | "avesta"
Ay | ay  | "Aymara"              | "aymar aru"
Az | az  | "Azerbaijani"         | "az??rbaycan dili"
Bm | bm  | "Bambara"             | "bamanankan"
Ba | ba  | "Bashkir"             | "?????????????? ????????"
Eu | eu  | "Basque"              | "euskara"
Be | be  | "Belarusian"          | "???????????????????? ????????"
Bn | bn  | "Bengali"             | "???????????????"
Bh | bh  | "Bihari"              | "?????????????????????"
Bi | bi  | "Bislama"             | "Bislama"
Bs | bs  | "Bosnian"             | "bosanski jezik"
Br | br  | "Breton"              | "brezhoneg"
Bg | bg  | "Bulgarian"           | "?????????????????? ????????"
My | my  | "Burmese"             | "???????????????"
Ca | ca  | "Catalan"             | "catal??"
Ch | ch  | "Chamorro"            | "Chamoru"
Ce | ce  | "Chechen"             | "?????????????? ????????"
Ny | ny  | "Chichewa"            | "chiChe??a"
Zh | zh  | "Chinese"             | "??????"
Cv | cv  | "Chuvash"             | "?????????? ??????????"
Kw | kw  | "Cornish"             | "Kernewek"
Co | co  | "Corsican"            | "corsu"
Cr | cr  | "Cree"                | "?????????????????????"
Hr | hr  | "Croatian"            | "hrvatski jezik"
Cs | cs  | "Czech"               | "??e??tina"
Da | da  | "Danish"              | "dansk"
Dv | dv  | "Divehi"              | "????????????"
Nl | nl  | "Dutch"               | "Nederlands"
Dz | dz  | "Dzongkha"            | "??????????????????"
En | en  | "English"             | "English"
Eo | eo  | "Esperanto"           | "Esperanto"
Et | et  | "Estonian"            | "eesti"
Ee | ee  | "Ewe"                 | "E??egbe"
Fo | fo  | "Faroese"             | "f??royskt"
Fj | fj  | "Fijian"              | "vosa Vakaviti"
Fi | fi  | "Finnish"             | "suomi"
Fr | fr  | "French"              | "fran??ais"
Ff | ff  | "Fula"                | "Fulfulde"
Gl | gl  | "Galician"            | "galego"
Ka | ka  | "Georgian"            | "?????????????????????"
De | de  | "German"              | "Deutsch"
El | el  | "Greek"               | "????????????????"
Gn | gn  | "Guaran??"             | "Ava??e'???"
Gu | gu  | "Gujarati"            | "?????????????????????"
Ht | ht  | "Haitian"             | "Krey??l ayisyen"
Ha | ha  | "Hausa"               | "(Hausa) ????????????"
He | he  | "Hebrew"              | "??????????"
Hz | hz  | "Herero"              | "Otjiherero"
Hi | hi  | "Hindi"               | "??????????????????"
Ho | ho  | "Hiri Motu"           | "Hiri Motu"
Hu | hu  | "Hungarian"           | "magyar"
Ia | ia  | "Interlingua"         | "Interlingua"
Id | id  | "Indonesian"          | "Bahasa Indonesia"
Ie | ie  | "Interlingue"         | "Interlingue"
Ga | ga  | "Irish"               | "Gaeilge"
Ig | ig  | "Igbo"                | "As???s??? Igbo"
Ik | ik  | "Inupiaq"             | "I??upiaq"
Io | io  | "Ido"                 | "Ido"
Is | is  | "Icelandic"           | "??slenska"
It | it  | "Italian"             | "Italiano"
Iu | iu  | "Inuktitut"           | "??????????????????"
Ja | ja  | "Japanese"            | "????????? (????????????)"
Jv | jv  | "Javanese"            | "????????????"
Kl | kl  | "Kalaallisut"         | "kalaallisut"
Kn | kn  | "Kannada"             | "???????????????"
Kr | kr  | "Kanuri"              | "Kanuri"
Ks | ks  | "Kashmiri"            | "?????????????????????"
Kk | kk  | "Kazakh"              | "?????????? ????????"
Km | km  | "Khmer"               | "???????????????"
Ki | ki  | "Kikuyu"              | "G??k??y??"
Rw | rw  | "Kinyarwanda"         | "Ikinyarwanda"
Ky | ky  | "Kyrgyz"              | "????????????????"
Kv | kv  | "Komi"                | "???????? ??????"
Kg | kg  | "Kongo"               | "Kikongo"
Ko | ko  | "Korean"              | "?????????"
Ku | ku  | "Kurdish"             | "Kurd??"
Kj | kj  | "Kwanyama"            | "Kuanyama"
La | la  | "Latin"               | "lingua latina"
Lb | lb  | "Luxembourgish"       | "L??tzebuergesch"
Lg | lg  | "Ganda"               | "Luganda"
Li | li  | "Limburgish"          | "Limburgs"
Ln | ln  | "Lingala"             | "Ling??la"
Lo | lo  | "Lao"                 | "?????????????????????"
Lt | lt  | "Lithuanian"          | "lietuvi?? kalba"
Lu | lu  | "Luba-Katanga"        | "Tshiluba"
Lv | lv  | "Latvian"             | "latvie??u valoda"
Gv | gv  | "Manx"                | "Gaelg"
Mk | mk  | "Macedonian"          | "???????????????????? ??????????"
Mg | mg  | "Malagasy"            | "fiteny malagasy"
Ms | ms  | "Malay"               | "bahasa Melayu"
Ml | ml  | "Malayalam"           | "??????????????????"
Mt | mt  | "Maltese"             | "Malti"
Mi | mi  | "M??ori"               | "te reo M??ori"
Mr | mr  | "Marathi"             | "???????????????"
Mh | mh  | "Marshallese"         | "Kajin M??aje??"
Mn | mn  | "Mongolian"           | "???????????? ??????"
Na | na  | "Nauruan"             | "Dorerin Naoero"
Nv | nv  | "Navajo"              | "Din?? bizaad"
Nd | nd  | "Northern Ndebele"    | "isiNdebele"
Ne | ne  | "Nepali"              | "??????????????????"
Ng | ng  | "Ndonga"              | "Owambo"
Nb | nb  | "Norwegian Bokm??l"    | "Norsk bokm??l"
Nn | nn  | "Norwegian Nynorsk"   | "Norsk nynorsk"
No | no  | "Norwegian"           | "Norsk"
Ii | ii  | "Nuosu"               | "????????? Nuosuhxop"
Nr | nr  | "Southern Ndebele"    | "isiNdebele"
Oc | oc  | "Occitan"             | "occitan"
Oj | oj  | "Ojibwe"              | "????????????????????????"
Cu | cu  | "Old Church Slavonic" | "?????????? ????????????????????"
Om | om  | "Oromo"               | "Afaan Oromoo"
Or | or  | "Oriya"               | "???????????????"
Os | os  | "Ossetian"            | "???????? ??????????"
Pa | pa  | "Punjabi"             | "??????????????????"
Pi | pi  | "P??li"                | "????????????"
Fa | fa  | "Persian"             | "??????????"
Pl | pl  | "Polish"              | "j??zyk polski"
Ps | ps  | "Pashto"              | "????????"
Pt | pt  | "Portuguese"          | "Portugu??s"
Qu | qu  | "Quechua"             | "Runa Simi"
Rm | rm  | "Romansh"             | "rumantsch grischun"
Rn | rn  | "Kirundi"             | "Ikirundi"
Ro | ro  | "Romanian"            | "Rom??n??"
Ru | ru  | "Russian"             | "??????????????"
Sa | sa  | "Sanskrit"            | "???????????????????????????"
Sc | sc  | "Sardinian"           | "sardu"
Sd | sd  | "Sindhi"              | "??????????????????"
Se | se  | "Northern Sami"       | "Davvis??megiella"
Sm | sm  | "Samoan"              | "gagana fa'a Samoa"
Sg | sg  | "Sango"               | "y??ng?? t?? s??ng??"
Sr | sr  | "Serbian"             | "???????????? ??????????"
Gd | gd  | "Gaelic"              | "G??idhlig"
Sn | sn  | "Shona"               | "chiShona"
Si | si  | "Sinhalese"           | "???????????????"
Sk | sk  | "Slovak"              | "sloven??ina"
Sl | sl  | "Slovene"             | "slovenski jezik"
So | so  | "Somali"              | "Soomaaliga"
St | st  | "Southern Sotho"      | "Sesotho"
Es | es  | "Spanish"             | "Espa??ol"
Su | su  | "Sundanese"           | "Basa Sunda"
Sw | sw  | "Swahili"             | "Kiswahili"
Ss | ss  | "Swati"               | "SiSwati"
Sv | sv  | "Swedish"             | "svenska"
Ta | ta  | "Tamil"               | "???????????????"
Te | te  | "Telugu"              | "??????????????????"
Tg | tg  | "Tajik"               | "????????????"
Th | th  | "Thai"                | "?????????"
Ti | ti  | "Tigrinya"            | "????????????"
Bo | bo  | "Tibetan"             | "?????????????????????"
Tk | tk  | "Turkmen"             | "T??rkmen"
Tl | tl  | "Tagalog"             | "Wikang Tagalog"
Tn | tn  | "Tswana"              | "Setswana"
To | to  | "Tonga"               | "faka Tonga"
Tr | tr  | "Turkish"             | "T??rk??e"
Ts | ts  | "Tsonga"              | "Xitsonga"
Tt | tt  | "Tatar"               | "?????????? ????????"
Tw | tw  | "Twi"                 | "Twi"
Ty | ty  | "Tahitian"            | "Reo Tahiti"
Ug | ug  | "Uyghur"              | "???????????????????"
Uk | uk  | "Ukrainian"           | "????????????????????"
Ur | ur  | "Urdu"                | "????????"
Uz | uz  | "Uzbek"               | "O??zbek"
Ve | ve  | "Venda"               | "Tshiven???a"
Vi | vi  | "Vietnamese"          | "Ti???ng Vi???t"
Vo | vo  | "Volap??k"             | "Volap??k"
Wa | wa  | "Walloon"             | "walon"
Cy | cy  | "Welsh"               | "Cymraeg"
Wo | wo  | "Wolof"               | "Wollof"
Fy | fy  | "Western Frisian"     | "Frysk"
Xh | xh  | "Xhosa"               | "isiXhosa"
Yi | yi  | "Yiddish"             | "????????????"
Yo | yo  | "Yoruba"              | "Yor??b??"
Za | za  | "Zhuang"              | "Sa?? cue????"
Zu | zu  | "Zulu"                | "isiZulu"
}
