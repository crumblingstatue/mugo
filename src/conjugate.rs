use {
    crate::{Root, RootKind, Step},
    log::debug,
};

impl Root {
    /// Returns the suffix that's needed to be appended in order to conjugate this root
    pub fn conjugation_suffix(&self) -> String {
        debug!("conjugation_suffix {:?}: {:?}", self.kind, self.steps);
        let mut text = String::new();
        if self.steps.is_empty() {
            // Verb stem handling
            push_masu_root_naked(self.kind, &mut text);
            return text;
        }
        for (i, step) in self.steps.iter().enumerate() {
            debug!("step: {step:?}");
            let kind = if i == 0 {
                Some(self.kind)
            } else {
                let prev = &self.steps[i - 1];
                prev.root_kind()
            };
            debug!("kind: {kind:?}");
            let Some(kind) = kind else {
                debug!("No root kind, special case spaghetti code:");
                match step {
                    Step::Ka => text.push('か'),
                    Step::Nda => text.push_str("んだ"),
                    // Nai after something that doesn't have a root... I guess. Just push ない.
                    Step::Nai => text.push_str("ない"),
                    _ => text.push_str("###BUG###"),
                }
                continue;
            };
            // There is no next step, or it's disjoint from the current conjugation sequence
            let next_step_disjoint = match self.steps.get(i + 1) {
                Some(step) => matches!(step, Step::Ka),
                None => true,
            };
            match step {
                Step::Te => {
                    push_te_root(kind, &mut text);
                    text.push(te_char(kind));
                }
                Step::Ta => push_ta(kind, &mut text),
                Step::Nai => {
                    push_neg_root(kind, &mut text);
                    text.push('な');
                    if next_step_disjoint {
                        text.push('い');
                    }
                }
                Step::Nu => {
                    push_neg_root(kind, &mut text);
                    text.push('ぬ');
                }
                Step::Naide => {
                    push_neg_root(kind, &mut text);
                    text.push_str("ないで");
                }
                Step::Volitional => match kind {
                    RootKind::Ichidan => text.push_str("よう"),
                    RootKind::Kuru => text.push_str("こよう"),
                    RootKind::GodanBu => text.push_str("ぼう"),
                    RootKind::GodanMu => text.push_str("もう"),
                    RootKind::GodanNu => text.push_str("のう"),
                    RootKind::GodanRu => text.push_str("ろう"),
                    RootKind::GodanSu => text.push_str("そう"),
                    RootKind::GodanTsu => text.push_str("とう"),
                    RootKind::GodanU => text.push_str("おう"),
                    RootKind::GodanGu => text.push_str("ごう"),
                    RootKind::GodanKu | RootKind::Iku => text.push_str("こう"),
                    RootKind::IAdjective => todo!(),
                    RootKind::NaAdjective => todo!(),
                    RootKind::Suru | RootKind::SpecialSuru => text.push_str("しよう"),
                },
                Step::AdverbialKu => text.push('く'),
                Step::Imperative => match kind {
                    RootKind::Ichidan => text.push('ろ'),
                    RootKind::GodanBu => text.push('べ'),
                    RootKind::GodanMu => text.push('め'),
                    RootKind::GodanNu => text.push('ね'),
                    RootKind::GodanRu => text.push('れ'),
                    RootKind::GodanSu => text.push('せ'),
                    RootKind::GodanTsu => text.push('て'),
                    RootKind::GodanU => text.push('え'),
                    RootKind::GodanGu => text.push('げ'),
                    RootKind::GodanKu | RootKind::Iku => text.push('け'),
                    RootKind::IAdjective => todo!(),
                    RootKind::Kuru => text.push('い'),
                    RootKind::NaAdjective => todo!(),
                    RootKind::Suru | RootKind::SpecialSuru => text.push_str("しろ"),
                },
                Step::Masu | Step::Masen => {
                    push_masu_root(kind, &mut text);
                    if next_step_disjoint {
                        match step {
                            Step::Masu => text.push('す'),
                            Step::Masen => text.push_str("せん"),
                            _ => {}
                        }
                    }
                }
                Step::Continuous => {
                    text.push('い');
                    if next_step_disjoint {
                        text.push('る');
                    }
                }
                Step::ContRuAbbrev => {
                    if next_step_disjoint {
                        text.push('る');
                    }
                }
                Step::Invitational => {
                    push_masu_root(kind, &mut text);
                    text.push_str("しょう");
                }
                Step::Zu => {
                    push_neg_root(kind, &mut text);
                    text.push('ず');
                }
                Step::Ka => text.push('か'),
                Step::Tari => {
                    push_ta(kind, &mut text);
                    text.push('り');
                }
                Step::Tara => {
                    push_ta(kind, &mut text);
                    text.push('ら');
                }
                Step::Nasai => {
                    push_masu_root_naked(kind, &mut text);
                    text.push_str("なさい");
                }
                Step::Nagara => {
                    push_masu_root_naked(kind, &mut text);
                    text.push_str("ながら");
                }
                Step::Causative => {
                    push_neg_root(kind, &mut text);
                    match kind {
                        RootKind::Ichidan => text.push_str("させ"),
                        RootKind::Suru | RootKind::SpecialSuru => {
                            text.pop();
                            text.push_str("させ");
                        }
                        RootKind::IAdjective => text.push_str("###TODO###"),
                        _ => text.push('せ'),
                    }
                    if next_step_disjoint {
                        text.push('る');
                    }
                }
                Step::Passive => {
                    push_neg_root(kind, &mut text);
                    match kind {
                        RootKind::Ichidan => text.push_str("られ"),
                        RootKind::IAdjective => todo!(),
                        _ => text.push('れ'),
                    }
                    if next_step_disjoint {
                        text.push('る');
                    }
                }
                Step::Tai => {
                    push_masu_root_naked(kind, &mut text);
                    text.push('た');
                    if next_step_disjoint {
                        text.push('い');
                    }
                }
                Step::Ba => {
                    push_e_root(kind, &mut text, true);
                    text.push('ば');
                }
                Step::Nakatta => {
                    push_neg_root(kind, &mut text);
                    text.push_str("なかった");
                }
                Step::Potential => {
                    push_e_root(kind, &mut text, false);
                    if next_step_disjoint {
                        text.push('る');
                    }
                }
                Step::Na => text.push('な'),
                Step::Chau => {
                    push_te_root(kind, &mut text);
                    push_chau_root(kind, &mut text);
                    if next_step_disjoint {
                        text.push('う');
                    }
                }
                Step::Katta => {
                    text.push_str("かった");
                }
                // Might or might not need special handling by API consumer
                //to remove extra part
                Step::Stem => push_masu_root_naked(kind, &mut text),
                Step::Ki => text.push('き'),
                Step::Nda => text.push_str("んだ"),
                Step::Kereba => text.push_str("ければ"),
                Step::Nakya => {
                    push_neg_root(kind, &mut text);
                    text.push_str("なきゃ");
                }
                Step::Sa => {
                    text.push('さ');
                }
            }
        }
        text
    }
}

fn te_char(kind: RootKind) -> char {
    match kind {
        RootKind::IAdjective
        | RootKind::Kuru
        | RootKind::Iku
        | RootKind::GodanKu
        | RootKind::GodanU
        | RootKind::GodanTsu
        | RootKind::GodanSu
        | RootKind::GodanRu
        | RootKind::Suru
        | RootKind::SpecialSuru
        | RootKind::Ichidan => 'て',
        RootKind::GodanGu | RootKind::GodanNu | RootKind::GodanMu | RootKind::GodanBu => 'で',
        RootKind::NaAdjective => todo!(),
    }
}

fn push_te_root(kind: RootKind, text: &mut String) {
    debug!("push_te_root");
    match kind {
        RootKind::Ichidan | RootKind::Kuru | RootKind::IAdjective => {}
        RootKind::GodanBu | RootKind::GodanMu | RootKind::GodanNu => text.push('ん'),
        RootKind::GodanRu | RootKind::GodanTsu | RootKind::GodanU | RootKind::Iku => {
            text.push('っ')
        }
        RootKind::GodanSu => text.push('し'),
        RootKind::GodanGu | RootKind::GodanKu => text.push('い'),
        RootKind::NaAdjective => todo!(),
        RootKind::Suru | RootKind::SpecialSuru => {
            text.push('し');
        }
    }
}

fn push_chau_root(kind: RootKind, text: &mut String) {
    match kind {
        RootKind::GodanGu | RootKind::GodanBu | RootKind::GodanMu | RootKind::GodanNu => {
            text.push_str("じゃ")
        }
        RootKind::GodanKu
        | RootKind::GodanU
        | RootKind::GodanTsu
        | RootKind::GodanSu
        | RootKind::GodanRu
        | RootKind::Ichidan
        | RootKind::Iku
        | RootKind::Suru
        | RootKind::SpecialSuru
        | RootKind::Kuru => text.push_str("ちゃ"),
        RootKind::IAdjective | RootKind::NaAdjective => {}
    }
}

fn push_e_root(kind: RootKind, text: &mut String, ba: bool) {
    match kind {
        RootKind::Ichidan => text.push_str(if ba { "れ" } else { "られ" }),
        RootKind::GodanBu => text.push('べ'),
        RootKind::GodanMu => text.push('め'),
        RootKind::GodanNu => text.push('ね'),
        RootKind::GodanRu => text.push('れ'),
        RootKind::GodanSu => text.push('せ'),
        RootKind::GodanTsu => text.push('て'),
        RootKind::GodanU => text.push('え'),
        RootKind::GodanGu => text.push('げ'),
        RootKind::GodanKu | RootKind::Iku => text.push('け'),
        RootKind::Kuru => text.push_str("こられ"),
        RootKind::IAdjective => todo!(),
        RootKind::NaAdjective => todo!(),
        RootKind::Suru | RootKind::SpecialSuru => todo!("できる special case needed(?)"),
    }
}

fn push_neg_root(kind: RootKind, text: &mut String) {
    match kind {
        RootKind::Ichidan => {}
        RootKind::GodanBu => text.push('ば'),
        RootKind::GodanMu => text.push('ま'),
        RootKind::GodanNu => text.push('な'),
        RootKind::GodanRu => text.push('ら'),
        RootKind::GodanSu => text.push('さ'),
        RootKind::GodanTsu => text.push('た'),
        RootKind::GodanU => text.push('わ'),
        RootKind::GodanGu => text.push('が'),
        RootKind::GodanKu => text.push('か'),
        RootKind::Iku => text.push('か'),
        RootKind::Kuru => text.push('こ'),
        RootKind::IAdjective => text.push_str("###TODO###"),
        RootKind::NaAdjective => todo!(),
        RootKind::Suru | RootKind::SpecialSuru => text.push('し'),
    }
}

fn push_ta(kind: RootKind, text: &mut String) {
    match kind {
        RootKind::Ichidan | RootKind::Kuru => text.push('た'),
        RootKind::GodanBu | RootKind::GodanMu | RootKind::GodanNu => text.push_str("んだ"),
        RootKind::GodanRu | RootKind::GodanTsu | RootKind::Iku => text.push_str("った"),
        RootKind::GodanSu | RootKind::Suru | RootKind::SpecialSuru => text.push_str("した"),
        RootKind::GodanU => text.push_str("った"),
        RootKind::GodanGu => text.push_str("いだ"),
        RootKind::GodanKu => text.push_str("いた"),
        RootKind::IAdjective => todo!(),
        RootKind::NaAdjective => todo!(),
    }
}

fn push_masu_root(kind: RootKind, text: &mut String) {
    push_masu_root_naked(kind, text);
    text.push('ま');
}

fn push_masu_root_naked(kind: RootKind, text: &mut String) {
    match kind {
        RootKind::Ichidan => {}
        RootKind::GodanBu => text.push('び'),
        RootKind::GodanMu => text.push('み'),
        RootKind::GodanNu => text.push('に'),
        RootKind::GodanRu => text.push('り'),
        RootKind::GodanSu => text.push('し'),
        RootKind::GodanTsu => text.push('ち'),
        RootKind::GodanU => text.push('い'),
        RootKind::GodanGu => text.push('ぎ'),
        RootKind::GodanKu => text.push('き'),
        RootKind::Iku => text.push('き'),
        RootKind::Kuru => text.push('き'),
        _ => {}
    }
}
