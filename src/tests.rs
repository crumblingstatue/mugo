use {
    super::{
        deconjugate,
        root::{Root, RootKind, Step},
    },
    owo_colors::{AnsiColors, OwoColorize},
    std::io::Write as _,
};

#[test]
fn test_deconjugate_positive() {
    init_logger();
    macro_rules! test_cases {
        ($($kana:literal => $root:literal $kind:ident: $($step:ident)*)+) => {
            $(
                assert!(deconjugate($kana).contains(&Root{text: $root.into(), kind: RootKind::$kind, steps: vec![$(Step::$step),*]}));
            )+
        };
    }
    test_cases! {
        // て
        "かえて" => "かえ" Ichidan: Te
        "かえって" => "かえ" GodanRu: Te
        "まって" => "ま" GodanTsu: Te
        "まよって" => "まよ" GodanU: Te
        "かがやいて" => "かがや" GodanKu: Te
        "いそいで" => "いそ" GodanGu: Te
        "とんで" => "と" GodanBu: Te
        "すんで" => "す" GodanMu: Te
        "しんで" => "し" GodanNu: Te
        "のばして" => "のば" GodanSu: Te
        "いって" => "い" Iku: Te
        "あいして" => "あい" Suru: Te
        "こいしてる" => "こい" SpecialSuru: Te ContRuAbbrev
        // ない
        "わからない" => "わか" GodanRu: Nai
        "およがない" => "およ" GodanGu: Nai
        "ならばない" => "なら" GodanBu: Nai
        "わたさないで" => "わた" GodanSu: Naide
        "しんぱいしないで" => "しんぱい" Suru: Naide
        "しんぱいしないで" => "しんぱい" SpecialSuru: Naide
        "しんじられない" => "しんじ" Ichidan: Potential Nai
        "じっとしてられない" => "じっと" Suru: Te Potential Nai
        // ぬ
        "うしなわぬ" => "うしな" GodanU: Nu
        "つきすすめ" => "つきすす" GodanMu: Imperative
        // なかった
        "ならわなかった" => "なら" GodanU: Nakatta
        "こまなかった" => "こ" GodanMu: Nakatta
        // た
        "みつけた" => "みつけ" Ichidan: Ta
        "みつかった" => "みつか" GodanRu: Ta
        "いった" => "い" Iku: Ta
        "あそんだ" => "あそ" GodanBu: Ta
        "すんだ" => "す" GodanMu: Ta
        "しんだ" => "し" GodanNu: Ta
        "はなした" => "はな" GodanSu: Ta
        "つかった" => "つか" GodanU: Ta
        "行った" => "行" GodanU: Ta
        "つらぬいた" => "つらぬ" GodanKu: Ta
        "さわいだ" => "さわ" GodanGu: Ta
        // Potential た
        "であえた" => "であ" GodanU: Potential Ta
        // たり
        "つかったり" => "つか" GodanU: Tari
        "あそんだり" => "あそ" GodanBu: Tari
        // たら
        "もどしたら" => "もど" GodanSu: Tara
        "えらんだら" => "えら" GodanBu: Tara
        "すすんだら" => "すす" GodanMu: Tara
        "しんだら" => "し" GodanNu: Tara
        // Volitional
        "かけよう" => "かけ" Ichidan: Volitional
        "いこう" => "い" Iku: Volitional
        "おもおう" => "おも" GodanU: Volitional
        // Invitational
        "しにましょう" => "し" GodanNu: Invitational
        "きましょう" => "" Kuru: Invitational
        // Imperative
        "いけ" => "い" Iku: Imperative
        "かがやけ" => "かがや" GodanKu: Imperative
        "しね" => "し" GodanNu: Imperative
        "とまれ" => "とま" GodanRu: Imperative
        "とめろ" => "とめ" Ichidan: Imperative
        "つくせ" => "つく" GodanSu: Imperative
        "うて" => "う" GodanTsu: Imperative
        "いそげ" => "いそ" GodanGu: Imperative
        "たたかえ" => "たたか" GodanU: Imperative
        // ます
        "たべます" => "たべ" Ichidan: Masu
        "のみます" => "の" GodanMu: Masu
        "ききます" => "き" GodanKu: Masu
        "ききました" => "き" GodanKu: Masu Ta
        "いきます" => "い" Iku: Masu
        // たい
        "つらぬきたい" => "つらぬ" GodanKu: Tai
        "あいたく" => "あ" GodanU: Tai AdverbialKu
        "しにたくない" => "し" GodanNu: Tai AdverbialKu Nai
        // Masen
        "すみません" => "す" GodanMu: Masen
        "かりません" => "かり" Ichidan: Masen
        // Masenka (polite invite)
        "ききませんか" => "き" GodanKu: Masen Ka
        // てる/ている
        "いきている" => "いき" Ichidan: Te Continuous
        "いきてる" =>  "いき" Ichidan: Te ContRuAbbrev
        "あいしてる" => "あい" Suru: Te ContRuAbbrev
        "あいしてる" => "あい" SpecialSuru: Te ContRuAbbrev
        "こいしてる" => "こい" Suru: Te ContRuAbbrev
        "こいしてる" => "こい" SpecialSuru: Te ContRuAbbrev
        "とんでいる" => "と" GodanBu: Te Continuous
        "はこんでる" => "はこ" GodanBu: Te ContRuAbbrev
        "すんでいる" => "す" GodanMu: Te Continuous
        // ていた/てた
        "ゆらめいていた" => "ゆらめ" GodanKu: Te Continuous Ta
        "あこがれてた" => "あこがれ" Ichidan: Te ContRuAbbrev Ta
        // くる (irregular)
        "きます" => "" Kuru: Masu
        "きました" => "" Kuru: Masu Ta
        "きません" => "" Kuru: Masen
        "こよう" => "こ" Kuru: Volitional
        "こい" => "" Kuru: Imperative
        "きて" => "" Kuru: Te
        "きた" => "" Kuru: Ta
        "こられる" => "" Kuru: Passive
        "でてこない" => "でて" Kuru: Nai
        // ず
        "およがず" => "およ" GodanGu: Zu
        "しんじられず" => "しんじ" Ichidan: Potential Zu
        // なさい
        "あそびなさい" => "あそ" GodanBu: Nasai
        "たのみなさい" => "たの" GodanMu: Nasai
        // ながら
        "まよいながら" => "まよ" GodanU: Nagara
        "まちながら" => "ま" GodanTsu: Nagara
        // Causative
        "しなせる" => "し" GodanNu: Causative
        "みつけさせる" => "みつけ" Ichidan: Causative
        "ださせる" => "だ" GodanSu: Causative
        "じゃまさせる" => "じゃま" Suru: Causative
        // Causative stem
        "さかせ" => "さ" GodanKu: Causative Stem
        // Causative nai
        "いわせない" => "い" GodanU: Causative Nai
        // Passive
        "うごかれる" => "うご" GodanKu: Passive
        "あけられる" => "あけ" Ichidan: Passive
        // Causative passive
        "まわらされる" => "まわ" GodanRu: Causative Passive
        // Passive casual past
        "なられた" => "な" GodanRu: Passive Ta
        // Passive て
        "さらわれて" => "さら" GodanU: Passive Te
        // Yes, it's true, it's both passive AND potential. Same conjugation.
        "あけられる" => "あけ" Ichidan: Potential
        // ば
        "さわげば" => "さわ" GodanGu: Ba
        "ふりだせば" => "ふりだ" GodanSu: Ba
        "すすめば" => "すす" GodanMu: Ba
        "いれば" => "い" GodanRu: Ba
        "うければ" => "うけ" Ichidan: Ba
        // Potential
        "あげられる" => "あげ" Ichidan: Potential
        "とれる" => "と" GodanRu: Potential
        "よべる" => "よ" GodanBu: Potential
        "こられる" => "こ" Kuru: Potential
        "ぬげる" => "ぬ" GodanGu: Potential
        "すすめる" => "すす" GodanMu: Potential
        "ゆける" => "ゆ" GodanKu: Potential
        "ゆける" => "ゆ" Iku: Potential
        "いける" => "い" Iku: Potential
        // ちゃう/じゃう
        "のんじゃう" => "の" GodanMu: Chau
        "おちちゃった" => "おち" Ichidan: Chau Ta
        "しんじゃった" => "し" GodanNu: Chau Ta
        "かっちゃった" => "か" GodanU: Chau Ta
        "まちがっちゃった" => "まちが" GodanU: Chau Ta
        // Verb stems
        "かけめぐり" => "かけめぐ" GodanRu: Stem
        "もやし" => "もや" GodanSu: Stem
        "ふりむき" => "ふりむ" GodanKu: Stem
        "しずみ" => "しず" GodanMu: Stem
        "ひろげ" => "ひろげ" Ichidan: Stem
        "あい" => "あ" GodanU: Stem
        // い Adjectives
        "はやく" => "はや" IAdjective: AdverbialKu
        // い past
        "つまらなかった" => "つまらな" IAdjective: Katta
        // い stem
        "おいし" => "おいし" IAdjective:
        // い adjective archaic き
        "ちゃいろき" => "ちゃいろ" IAdjective: Ki
        "あつき" => "あつ" IAdjective: Ki
        // な Adjectives
        "かすかな" => "かすか" NaAdjective: Na
        "にぎやかな" => "にぎやか" NaAdjective: Na
        // んだ
        "だいすきなんだ" => "だいすき" NaAdjective: Na Nda
        "はじめたんだ" => "はじめ" Ichidan: Ta Nda
        // ければ
        "かわいければ" => "かわい" IAdjective: Kereba
        "にげださなければ" => "にげだ" GodanSu: Nai Kereba
        // なきゃ
        "たちあがらなきゃ" => "たちあが" GodanRu: Nakya
        // さ
        "やさしさ" => "やさし" IAdjective: Sa
        "つよさ" => "つよ" IAdjective: Sa
        // てく
        "ふりはらってく" => "ふりはら" GodanU: Teku
        "かわってく" => "かわ" GodanRu: Teku
        "よべ" => "よ" GodanBu: Imperative
    }
}

#[test]
fn test_deconjugate_negative() {
    init_logger();
    macro_rules! test_cases {
        ($($kana:literal => $root:literal $kind:ident: $($step:ident)*)+) => {
            $(
                assert!(!deconjugate($kana).contains(&Root{text: $root.into(), kind: RootKind::$kind, steps: vec![$(Step::$step),*]}));
            )+
        };
    }
    test_cases! {
        "いかない" => "いか" Suru: Nai
    }
}

#[test]
fn test_conj() {
    init_logger();
    macro_rules! test_cases {
        ($($kind:ident: $($step:ident)* => $kana:literal)+) => {
            $(
                assert_eq!(Root{text: "".into(), kind: RootKind::$kind, steps: vec![$(Step::$step),*]}.conjugation_suffix(), $kana);
            )+
        };
    }
    test_cases! {
        GodanKu: Imperative => "け"
        Iku: Imperative => "け"
        GodanKu: Masu Ta => "きました"
        GodanNu: Invitational => "にましょう"
        GodanGu: Masen Ka => "ぎませんか"
        GodanU: Ta => "った"
        GodanU: Tari => "ったり"
        GodanBu: Nasai => "びなさい"
        GodanMu: Nasai => "みなさい"
        GodanNu: Causative => "なせる"
        Ichidan: Causative => "させる"
        GodanKu: Passive => "かれる"
        Ichidan: Passive => "られる"
        GodanSu: Zu => "さず"
        GodanRu: Stem => "り"
        GodanSu: Stem => "し"
        NaAdjective: Na => "な"
        NaAdjective: Na => "な"
        GodanNu: Chau => "んじゃう"
        Ichidan: Chau Ta => "ちゃった"
        GodanNu: Chau Ta => "んじゃった"
        GodanMu: Naide => "まないで"
        IAdjective: Katta => "かった"
        Suru: Volitional => "しよう"
        SpecialSuru: Volitional => "しよう"
        Suru: Imperative => "しろ"
        SpecialSuru: Imperative => "しろ"
        SpecialSuru: Te ContRuAbbrev => "してる"
        GodanKu: Ka => "か"
        GodanKu: Causative Stem => "かせ"
        GodanKu: Te Continuous Ta => "いていた"
        Ichidan: Te ContRuAbbrev Ta => "てた"
        GodanU: Nu => "わぬ"
        IAdjective: Ki => "き"
        NaAdjective: Na Nda => "なんだ"
        Ichidan: Ta Nda => "たんだ"
        Kuru: Nai => "こない" // Fine as long as it's 来る kanji... TODO: Find solution for kanjiless 出てこない
        Ichidan: Potential => "られる"
        GodanGu: Imperative => "げ"
        Ichidan: Ba => "れば"
        GodanKu: Potential => "ける"
        Iku: Potential => "ける"
        Suru: Te Potential Nai => "してられない"
        Suru: Causative => "させる"
        IAdjective: Kereba => "ければ"
        GodanSu: Nai Kereba => "さなければ"
        GodanBu: Tara => "んだら"
        GodanMu: Tara => "んだら"
        GodanNu: Tara => "んだら"
        Kuru: Invitational => "きましょう"
        Kuru: Masu => "きます"
        Kuru: Masen => "きません"
        Kuru: Masu Ta => "きました"
        Kuru: Volitional => "こよう"
        Kuru: Passive => "こられる"
        GodanRu: Nakya => "らなきゃ"
        IAdjective: Sa => "さ"
        GodanU: Tai AdverbialKu => "いたく"
        GodanNu: Tai AdverbialKu Nai => "にたくない"
        GodanRu: Causative => "らせる"
        GodanRu: Causative Passive => "らされる"
        GodanU: Teku => "ってく"
        GodanRu: Teku => "ってく"
        GodanU: Imperative => "え"
        GodanU: Stem => "い"
        GodanMu: Imperative => "め"
        GodanBu: Imperative => "べ"
    }
}

#[test]
fn test_dict() {
    init_logger();
    macro_rules! test_cases {
        ($($root:literal $kind:ident => $kana:literal)+) => {
            $(
                assert_eq!(&dbg!(Root{text: $root.into(), kind: RootKind::$kind, steps: vec![]}).dict_string(), $kana);
            )+
        };
    }
    test_cases! {
        // 逝く ? Not sure what I'm testing for here
        "い" GodanKu => "いく"
        "い" Iku => "いく"
        "かり" Ichidan => "かりる"
        "こい" Suru => "こいする"
        "あい" SpecialSuru => "あいする"
        "" Kuru => "くる"
        "でて" Kuru => "でてくる"
        "やさし" IAdjective => "やさしい"
        "行" GodanU => "行う"
    }
}

fn init_logger() {
    static INIT: std::sync::Once = std::sync::Once::new();

    INIT.call_once(|| {
        env_logger::builder()
            .format(|buf, rec| {
                let color = match rec.level() {
                    log::Level::Error => AnsiColors::Red,
                    log::Level::Warn => AnsiColors::Yellow,
                    log::Level::Info => AnsiColors::Green,
                    log::Level::Debug => AnsiColors::Blue,
                    log::Level::Trace => AnsiColors::White,
                };
                writeln!(
                    buf,
                    "{}:{}: {} {}",
                    rec.file().unwrap().yellow(),
                    rec.line().unwrap().red(),
                    rec.level().color(color),
                    rec.args()
                )
            })
            .init();
    });
}
