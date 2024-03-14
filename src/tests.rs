use {
    super::{
        deconjugate,
        root::{Root, RootKind, Step},
    },
    owo_colors::OwoColorize,
    std::io::Write as _,
};

#[test]
fn test_deconjugate() {
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
        "つらぬいた" => "つらぬ" GodanKu: Ta
        "さわいだ" => "さわ" GodanGu: Ta
        // たり
        "つかったり" => "つか" GodanU: Tari
        "あそんだり" => "あそ" GodanBu: Tari
        // たら
        "もどしたら" => "もど" GodanSu: Tara
        // Volitional
        "かけよう" => "かけ" Ichidan: Volitional
        "いこう" => "い" Iku: Volitional
        "おもおう" => "おも" GodanU: Volitional
        // Invitational
        "しにましょう" => "し" GodanNu: Invitational
        // Imperative
        "いけ" => "い" Iku: Imperative
        "かがやけ" => "かがや" GodanKu: Imperative
        "しね" => "し" GodanNu: Imperative
        "とまれ" => "とま" GodanRu: Imperative
        "とめろ" => "とめ" Ichidan: Imperative
        "つくせ" => "つく" GodanSu: Imperative
        "うて" => "う" GodanTsu: Imperative
        // ます
        "たべます" => "たべ" Ichidan: Masu
        "のみます" => "の" GodanMu: Masu
        "ききます" => "き" GodanKu: Masu
        "ききました" => "き" GodanKu: Masu Ta
        "いきます" => "い" Iku: Masu
        // たい
        "つらぬきたい" => "つらぬ" GodanKu: Tai
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
        "きます" => "き" Kuru: Masu
        "きました" => "き" Kuru: Masu Ta
        "きません" => "き" Kuru: Masen
        "こよう" => "こ" Kuru: Volitional
        "こい" => "" Kuru: Imperative
        "きて" => "" Kuru: Te
        "きた" => "" Kuru: Ta
        // ず
        "およがず" => "およ" GodanGu: Zu
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
        // Causative stem
        "さかせ" => "さ" GodanKu: Causative Stem
        // ば
        "さわげば" => "さわ" GodanGu: Ba
        "ふりだせば" => "ふりだ" GodanSu: Ba
        "すすめば" => "すす" GodanMu: Ba
        // Potential
        "あげられる" => "あげ" Ichidan: Potential
        "とれる" => "と" GodanRu: Potential
        "よべる" => "よ" GodanBu: Potential
        "こられる" => "こ" Kuru: Potential
        "ぬげる" => "ぬ" GodanGu: Potential
        "すすめる" => "すす" GodanMu: Potential
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
        // い Adjectives
        "はやく" => "はや" IAdjective: AdverbialKu
        // い past
        "つまらなかった" => "つまらな" IAdjective: Katta
        // い stem
        "おいし" => "おいし" IAdjective:
        // な Adjectives
        "かすかな" => "かすか" NaAdjective: Na
        "にぎやかな" => "にぎやか" NaAdjective: Na
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
        GodanSu: Zu => "さず"
        GodanRu: => "り"
        GodanSu: => "し"
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
    }
}

#[test]
fn test_dict() {
    init_logger();
    macro_rules! test_cases {
        ($($root:literal $kind:ident => $kana:literal)+) => {
            $(
                assert_eq!(&dbg!(Root{text: $root.into(), kind: RootKind::$kind, steps: vec![]}).dict(), $kana);
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
    }
}

fn init_logger() {
    static INIT: std::sync::Once = std::sync::Once::new();

    INIT.call_once(|| {
        env_logger::builder()
            .format(|buf, rec| {
                writeln!(
                    buf,
                    "{}:{}: {} {}",
                    rec.file().unwrap().yellow(),
                    rec.line().unwrap().red(),
                    rec.level().blue(),
                    rec.args()
                )
            })
            .init();
    });
}
