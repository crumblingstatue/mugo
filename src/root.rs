/// A possible root word.
///
/// You must use a dictionary to look up whether this is an actual word or not
#[derive(Debug, PartialEq, Clone)]
pub struct Root {
    pub text: String,
    pub kind: RootKind,
    pub steps: Vec<Step>,
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
}

impl Root {
    /// Get dictionary form (e.g.) しゃべ + ichidan ru = しゃべる
    pub fn dict(&self) -> String {
        let mut txt = self.text.clone();
        match self.kind {
            RootKind::Ichidan => txt.push('る'),
            RootKind::GodanBu => txt.push('ぶ'),
            RootKind::GodanMu => txt.push('む'),
            RootKind::GodanNu => txt.push('ぬ'),
            RootKind::GodanRu => txt.push('る'),
            RootKind::GodanSu => txt.push('す'),
            RootKind::GodanTsu => txt.push('つ'),
            RootKind::GodanU => txt.push('う'),
            RootKind::GodanGu => txt.push('ぐ'),
            RootKind::GodanKu => txt.push('く'),
            RootKind::IAdjective => txt.push('い'),
            RootKind::Iku => txt.push('く'),
            RootKind::Kuru => txt.push_str("くる"),
            RootKind::Suru | RootKind::SpecialSuru => txt.push_str("する"),
            RootKind::NaAdjective => {}
        };
        txt
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
            _ => return None,
        })
    }
}
