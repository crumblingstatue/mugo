/// A possible root word.
///
/// You must use a dictionary to look up whether this is an actual word or not
#[derive(PartialEq, Clone)]
pub struct Root {
    pub text: String,
    pub kind: RootKind,
    pub steps: Vec<Step>,
}

impl std::fmt::Debug for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "「{}」（{:?}）", self.dict_string(), self.kind)?;
        for (i, step) in self.steps.iter().enumerate() {
            write!(f, "{step:?}")?;
            if i != self.steps.len() - 1 {
                write!(f, " ➡ ")?;
            }
        }
        write!(f, " =「{}{}」", self.text, self.conjugation_suffix())?;
        Ok(())
    }
}

/// What kind of root word is this?
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RootKind {
    Ichidan,
    GodanBu,
    GodanMu,
    GodanNu,
    GodanRu,
    GodanSu,
    GodanTsu,
    GodanU,
    GodanGu,
    GodanKu,
    /// Irregular 行く
    Iku,
    /// Irregular 来る
    Kuru,
    /// Suru verb
    ///
    /// Note that these might require special handling if you're using a dictionary,
    /// because some suru verbs are listed with the する suffix as dictionary form,
    /// others are listed without. You need to handle both cases.
    Suru,
    /// Special suru verbs (they conjugate differently in some cases?)
    ///
    /// Note that these might require special handling if you're using a dictionary,
    /// because some suru verbs are listed with the する suffix as dictionary form,
    /// others are listed without. You need to handle both cases.
    SpecialSuru,
    IAdjective,
    NaAdjective,
}

#[cfg(feature = "labels")]
impl RootKind {
    /// Return a text label describing this root kind
    pub fn label(&self) -> &'static str {
        match self {
            RootKind::Ichidan => "ichidan",
            RootKind::GodanBu => "ぶ",
            RootKind::GodanMu => "む",
            RootKind::GodanNu => "ぬ",
            RootKind::GodanRu => "godan る",
            RootKind::GodanSu => "す",
            RootKind::GodanTsu => "つ",
            RootKind::GodanU => "う",
            RootKind::GodanGu => "ぐ",
            RootKind::GodanKu => "く",
            RootKind::Iku => "行く",
            RootKind::Kuru => "来る",
            RootKind::Suru => "する",
            RootKind::SpecialSuru => "する (special)",
            RootKind::IAdjective => "い adjective",
            RootKind::NaAdjective => "な adjective",
        }
    }
}

/// A conjugation step
#[derive(Debug, PartialEq, Clone)]
pub enum Step {
    /// ~て form
    Te,
    /// Casual, present negative
    Nai,
    /// Archaic form of nai
    Nu,
    /// without doing... Kinda special, not really Nai + Te
    Naide,
    /// Casual, past negative
    Nakatta,
    /// Casual past
    Ta,
    /// Volitional (よう)
    Volitional,
    /// Adverbial (く)
    AdverbialKu,
    /// Imperative (strong command)
    Imperative,
    /// Masu (polite)
    Masu,
    /// Masen (polite negative)
    Masen,
    /// ~ましょう
    Invitational,
    Continuous,
    ContRuAbbrev,
    Zu,
    /// Question particle
    Ka,
    /// Such things as...
    Tari,
    /// If/when (conjecture)
    Tara,
    /// Please do ...
    Nasai,
    /// While/during
    Nagara,
    Causative,
    Passive,
    /// Want to ...
    Tai,
    /// Conditional
    Ba,
    Potential,
    /// てしまう / でしまう abbrev
    Chau,
    /// na-adjective na
    Na,
    /// I adjective past
    Katta,
    /// Take the stem of a verb
    Stem,
    /// Arhaic い form
    Ki,
    /// のだ abbreviation
    Nda,
    /// い adjective conditional
    Kereba,
    /// Nakereba short form
    Nakya,
    /// Adjective さ suffix
    Sa,
    /// て + いく abbreviation
    Teku,
    /// て + おく (do something in advance, etc.)
    TeOku,
    /// TeOku abbreviation
    Toku,
}

#[cfg(feature = "labels")]
impl Step {
    /// Return a text label describing this step
    pub fn label(&self) -> &'static str {
        match self {
            Step::Te => "て",
            Step::Teku => "てく (て + いく)",
            Step::Nai => "ない",
            Step::Naide => "ないで",
            Step::Nakatta => "なかった",
            Step::Ta => "た",
            Step::Volitional => "volitional",
            Step::AdverbialKu => "く (adverb)",
            Step::Imperative => "imperative",
            Step::Masu => "ます",
            Step::Masen => "ません",
            Step::Invitational => "invitational",
            Step::Continuous => "ている",
            Step::ContRuAbbrev => "てる",
            Step::Zu => "ず",
            Step::Ka => "か",
            Step::Tari => "たり",
            Step::Tara => "たら",
            Step::Nasai => "なさい",
            Step::Nagara => "ながら",
            Step::Causative => "causative",
            Step::Passive => "passive",
            Step::Tai => "たい",
            Step::Ba => "ば (conditional)",
            Step::Potential => "potential",
            Step::Chau => "ちゃう",
            Step::Na => "な",
            Step::Katta => "かった",
            Step::Stem => "stem",
            Step::Nu => "ぬ",
            Step::Ki => "き (archaic い)",
            Step::Nda => "んだ",
            Step::Kereba => "ければ",
            Step::Nakya => "なきゃ",
            Step::Sa => "さ",
            Step::TeOku => "て + おく",
            Step::Toku => "とく (て + おく)",
        }
    }
}

impl Root {
    /// Get dictionary form (e.g.) しゃべ + ichidan ru = しゃべる
    pub fn dict_string(&self) -> String {
        [&self.text, self.dict_suffix()].concat()
    }
    /// Dictionary suffix
    pub fn dict_suffix(&self) -> &'static str {
        match self.kind {
            RootKind::Ichidan => "る",
            RootKind::GodanBu => "ぶ",
            RootKind::GodanMu => "む",
            RootKind::GodanNu => "ぬ",
            RootKind::GodanRu => "る",
            RootKind::GodanSu => "す",
            RootKind::GodanTsu => "つ",
            RootKind::GodanU => "う",
            RootKind::GodanGu => "ぐ",
            RootKind::GodanKu => "く",
            RootKind::IAdjective => "い",
            RootKind::Iku => "く",
            RootKind::Kuru => "くる",
            RootKind::Suru | RootKind::SpecialSuru => "する",
            RootKind::NaAdjective => "",
        }
    }
}

impl Step {
    /// If this step is used as a root, what is its kind?
    pub fn root_kind(&self) -> Option<RootKind> {
        Some(match self {
            Step::Te => RootKind::Ichidan,
            Step::Nai => RootKind::IAdjective,
            Step::Masu => RootKind::GodanSu,
            Step::Continuous => RootKind::Ichidan,
            Step::ContRuAbbrev => RootKind::Ichidan,
            Step::Potential => RootKind::Ichidan,
            Step::Chau => RootKind::GodanU,
            Step::Causative => RootKind::Ichidan,
            Step::Tai => RootKind::IAdjective,
            Step::Teku | Step::TeOku | Step::Toku => RootKind::GodanKu,
            _ => return None,
        })
    }
}
