use {
    crate::root::{Root, RootKind, Step},
    log::debug,
    log_dbg::ldbg,
};

pub fn deconjugate(word: &str) -> Vec<Root> {
    debug!("deconjugate({word})");
    let mut roots = Vec::new();
    let chars: Vec<char> = word.chars().collect();
    let steps = vec![];
    deconj_expr(&chars, &mut roots, steps);
    ldbg!(log::Level::Debug, roots)
}

fn deconj_expr(chars: &[char], roots: &mut Vec<Root>, steps: Vec<Step>) {
    debug!("deconj_expr: {chars:?}, {steps:?}");
    // Anything can be an い adjective root (I guess)
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::IAdjective,
        steps: steps.clone(),
    });
    // Also anything can be an ichidan verb stem... I guess?
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::Ichidan,
        steps: steps.clone().with(Step::Stem),
    });
    if let Some(['け', 'れ', 'ば']) = chars.last_chunk() {
        push_i_adjective_root(roots, &chars[..chars.len() - 3], steps.with(Step::Kereba));
        return;
    }
    if let Some((chars, ['ま', 'せ', 'ん'])) = chars.split_last_chunk() {
        push_masu_root(chars, roots, steps.clone().with(Step::Masen));
    }
    let Some((last_ch, chars)) = chars.split_last() else {
        return;
    };
    log::debug!("last char: {last_ch}");
    match last_ch {
        'て' => deconj_te(roots, chars, steps),
        'で' => deconj_de(roots, chars, steps),
        'た' => deconj_ta(roots, chars, steps),
        'だ' => deconj_da(roots, chars, steps),
        'い' => deconj_i(roots, chars, steps),
        'う' => deconj_u(roots, chars, steps),
        'く' => deconj_ku(roots, chars, steps),
        'ろ' => deconj_ro(roots, chars, steps),
        'れ' => deconj_re(roots, chars, steps),
        'け' => deconj_ke(roots, chars, steps),
        'ね' => deconj_ne(roots, chars, steps),
        'す' => deconj_su(roots, chars, steps),
        'る' => push_ichidan_root(chars, roots, steps, false),
        'ず' => deconj_zu(roots, chars, steps),
        'か' => deconj_ka(roots, chars, steps),
        'り' => deconj_ri(roots, chars.to_owned(), steps),
        'ら' => deconj_ra(roots, chars, steps),
        'ば' => deconj_ba(roots, chars, steps),
        'な' => deconj_na(roots, chars, steps),
        'し' => deconj_shi(roots, chars, steps),
        'せ' => deconj_se(roots, chars, steps),
        'き' => deconj_ki(roots, chars, steps),
        'み' => deconj_mi(roots, chars, steps),
        'ぬ' => deconj_nu(roots, chars, steps),
        'げ' => deconj_ge(roots, chars, steps),
        _ => {}
    }
}

fn push_i_adjective_root(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>) {
    debug!("push_i_adjective_root: {chars:?}, {steps:?}");
    // Anything can be an い adjective root (I guess)
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::IAdjective,
        steps: steps.clone(),
    });
    // ない
    if let Some('な') = chars.last() {
        push_negative_root(chars.init(), roots, steps.with(Step::Nai));
    }
}

fn deconj_ge(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>) {
    debug!("deconj_ge: {chars:?}, {steps:?}");
    // Godan gu imperative
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::GodanGu,
        steps: steps.clone().with(Step::Imperative),
    });
}

fn deconj_nu(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>) {
    debug!("deconj_nu: {chars:?}, {steps:?}");
    push_negative_root(chars, roots, steps.with(Step::Nu));
}

fn deconj_mi(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>) {
    debug!("deconj_mi: {chars:?}, {steps:?}");
    // Godan mu verb stem
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::GodanMu,
        steps: steps.with(Step::Stem),
    });
}

fn deconj_ki(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>) {
    debug!("deconj_ki: {chars:?}, {steps:?}");
    // Godan ku verb stem
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::GodanKu,
        steps: steps.clone().with(Step::Stem),
    });
    // Archaic ki (i adjective)
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::IAdjective,
        steps: steps.with(Step::Ki),
    });
}

fn deconj_se(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>) {
    debug!("deconj_se: {chars:?}, {steps:?}");
    // Godan su imperative
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::GodanSu,
        steps: steps.clone().with(Step::Imperative),
    });
    // Causative stem
    push_causative(steps.with(Step::Stem), chars, roots);
}

fn deconj_shi(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>) {
    debug!("deconj_shi: {chars:?}, {steps:?}");
    // Godan su verb stem
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::GodanSu,
        steps: steps.with(Step::Stem),
    });
}

fn deconj_na(roots: &mut Vec<Root>, chars: &[char], mut steps: Vec<Step>) {
    debug!("deconj_na: {chars:?}, {steps:?}");
    steps.insert(0, Step::Na);
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::NaAdjective,
        steps,
    });
}

fn deconj_ba(roots: &mut Vec<Root>, chars: &[char], mut steps: Vec<Step>) {
    debug!("deconj_ba: {chars:?}, {steps:?}");
    steps.push(Step::Ba);
    push_e_root(roots, chars, steps, true);
}

// Potential and ba roots are different for ichidan. Shocking, I know.
fn push_e_root(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>, ba: bool) {
    debug!("push_e_root: {chars:?}, {steps:?}");
    let Some((last, chars)) = chars.split_last() else {
        return;
    };
    match last {
        'え' => roots.push(Root {
            text: chars.to_string(),
            kind: RootKind::GodanU,
            steps,
        }),
        'け' => {
            roots.push(Root {
                text: chars.to_string(),
                kind: RootKind::GodanKu,
                steps: steps.clone(),
            });
            // Iku/yuku handling
            if matches!(chars.last(), Some('い' | 'ゆ')) {
                roots.push(Root {
                    text: chars.to_string(),
                    kind: RootKind::Iku,
                    steps,
                });
            }
        }
        'げ' => roots.push(Root {
            text: chars.to_string(),
            kind: RootKind::GodanGu,
            steps,
        }),
        'せ' => roots.push(Root {
            text: chars.to_string(),
            kind: RootKind::GodanSu,
            steps,
        }),
        'て' => roots.push(Root {
            text: chars.to_string(),
            kind: RootKind::GodanTsu,
            steps,
        }),
        'ね' => roots.push(Root {
            text: chars.to_string(),
            kind: RootKind::GodanNu,
            steps,
        }),
        'べ' => roots.push(Root {
            text: chars.to_string(),
            kind: RootKind::GodanBu,
            steps,
        }),
        'め' => roots.push(Root {
            text: chars.to_string(),
            kind: RootKind::GodanMu,
            steps,
        }),
        'れ' => {
            roots.push(Root {
                text: chars.to_string(),
                kind: RootKind::GodanRu,
                steps: steps.clone(),
            });
            if ba {
                push_ichidan_root(chars, roots, steps, false);
            } else if let Some(('ら', chars)) = chars.split_last() {
                debug!("ra!");
                push_ichidan_root(chars, roots, steps.clone(), false);
                if let Some('こ') = chars.last() {
                    roots.push(Root {
                        text: chars.to_string(),
                        kind: RootKind::Kuru,
                        steps,
                    });
                }
            }
        }
        _ => {}
    }
}

fn deconj_ra(roots: &mut Vec<Root>, chars: &[char], mut steps: Vec<Step>) {
    debug!("deconj_ra: {chars:?}, {steps:?}");
    let Some((last, chars)) = chars.split_last() else {
        return;
    };
    match last {
        'が' => {
            let Some('な') = chars.last() else { return };
            steps.push(Step::Nagara);
            push_masu_root(chars.init(), roots, steps);
        }
        'た' => {
            steps.push(Step::Tara);
            push_ta_root(chars, roots, steps);
        }
        _ => {}
    }
}

fn deconj_ri(roots: &mut Vec<Root>, mut chars: Vec<char>, mut steps: Vec<Step>) {
    debug!("deconj_ri: {chars:?}, {steps:?}");
    // Godan ru verb stem
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::GodanRu,
        steps: steps.clone().with(Step::Stem),
    });
    match chars.pop() {
        Some('た') => {
            steps.push(Step::Tari);
            push_ta_root(&chars, roots, steps);
        }
        Some('だ') => {
            steps.push(Step::Tari);
            push_da_root(&chars, roots, steps);
        }
        _ => (),
    }
}

fn deconj_ka(roots: &mut Vec<Root>, chars: &[char], mut steps: Vec<Step>) {
    debug!("deconj_ka: {chars:?}, {steps:?}");
    steps.push(Step::Ka);
    deconj_expr(chars, roots, steps);
}

fn deconj_zu(roots: &mut Vec<Root>, chars: &[char], mut steps: Vec<Step>) {
    debug!("deconj_zu: {chars:?}, {steps:?}");
    steps.push(Step::Zu);
    push_negative_root(chars, roots, steps);
}

fn push_i_cont_root(mut steps: Vec<Step>, mut chars: Vec<char>, roots: &mut Vec<Root>) {
    match chars.pop() {
        Some('て') => {
            steps.insert(0, Step::Continuous);
            deconj_te(roots, &chars, steps);
        }
        Some('で') => {
            steps.insert(0, Step::Continuous);
            deconj_de(roots, &chars, steps);
        }
        _ => {}
    }
}

fn push_causative(mut steps: Vec<Step>, chars: &[char], roots: &mut Vec<Root>) {
    debug!("push_causative: {chars:?}, {steps:?}");
    steps.insert(0, Step::Causative);
    if let Some('さ') = ldbg!(log::Level::Debug, chars.last()) {
        push_ichidan_root(chars.init(), roots, steps.clone(), true);
    }
    push_godan_negative_root(chars, roots, steps);
}

fn push_passive(mut steps: Vec<Step>, chars: &[char], roots: &mut Vec<Root>) {
    debug!("push_passive: {chars:?}, {steps:?}");
    steps.insert(0, Step::Passive);
    if let Some('ら') = ldbg!(log::Level::Debug, chars.last()) {
        push_ichidan_root(chars.init(), roots, steps.clone(), false);
    }
    push_godan_negative_root(chars, roots, steps);
}

fn deconj_i(roots: &mut Vec<Root>, chars: &[char], mut steps: Vec<Step>) {
    debug!("deconj_i: {chars:?}, {steps:?}");
    let Some((last, chars)) = chars.split_last() else {
        return;
    };
    match last {
        'な' => deconj_nai(roots, chars, steps),
        'さ' => deconj_sai(roots, chars.to_vec(), steps),
        'こ' => roots.push(Root {
            text: chars.to_string(),
            kind: RootKind::Kuru,
            steps: steps.with(Step::Imperative),
        }),
        'た' => {
            // Tai
            steps.push(Step::Tai);
            push_masu_root(chars, roots, steps);
        }
        _ => {}
    }
}

fn deconj_sai(roots: &mut Vec<Root>, mut chars: Vec<char>, mut steps: Vec<Step>) {
    let Some('な') = chars.pop() else { return };
    steps.push(Step::Nasai);
    push_masu_root(&chars, roots, steps);
}

fn deconj_nai(roots: &mut Vec<Root>, chars: &[char], mut steps: Vec<Step>) {
    debug!("deconj_nai: {chars:?}, {steps:?}");
    steps.insert(0, Step::Nai);
    push_negative_root(chars, roots, steps);
}

/// Push both godan and ichidan negative roots
fn push_negative_root(chars: &[char], roots: &mut Vec<Root>, steps: Vec<Step>) {
    debug!("push_negative_root: : {chars:?}, {steps:?}");
    push_godan_negative_root(chars, roots, steps.clone());
    push_ichidan_root(chars, roots, steps, false);
}

fn push_ichidan_root(
    chars: &[char],
    roots: &mut Vec<Root>,
    mut steps: Vec<Step>,
    suru_possible: bool,
) {
    debug!("push_ichidan_root: {chars:?}, {steps:?}");
    // The whole expression itself can be ichidan
    roots.ichidan(chars.to_string(), steps.clone());
    if suru_possible {
        // It can also indeed be suru/special suru
        roots.push(Root {
            text: chars.to_string(),
            kind: RootKind::Suru,
            steps: steps.clone(),
        });
        roots.push(Root {
            text: chars.to_string(),
            kind: RootKind::SpecialSuru,
            steps: steps.clone(),
        });
    }
    // Then we see what else it could be
    // Try for potential, but only if we're not already in a potential situation
    if steps.first() != Some(&Step::Potential) {
        push_e_root(roots, chars, steps.clone().with(Step::Potential), false);
    }
    debug!("push_ichidan_root (after e root push): {chars:?}, {steps:?}");
    let terminal = steps.is_empty();
    match chars.last() {
        Some('て') => {
            // Only add continuous ru handling if it's a terminal step...
            // TODO: Figure out how to handle this mess better
            if terminal {
                steps.push(Step::ContRuAbbrev);
            }
            deconj_te(roots, chars.init(), steps);
        }
        Some('で') => {
            // Only add continuous ru handling if it's a terminal step...
            // TODO: Figure out how to handle this mess better
            if terminal {
                steps.push(Step::ContRuAbbrev);
            }
            deconj_de(roots, chars.init(), steps);
        }
        Some('い') => push_i_cont_root(steps, chars.init().to_owned(), roots),
        Some('せ') => {
            debug!("seru");
            push_causative(steps, chars.init(), roots);
        }
        Some('れ') => {
            debug!("reru");
            push_passive(steps, chars.init(), roots);
        }
        _ => {}
    }
}

/// Push godan negative root, return false if there is no godan root match
fn push_godan_negative_root(chars: &[char], roots: &mut Vec<Root>, steps: Vec<Step>) {
    debug!("push_godan_negative_root: {chars:?}, {steps:?}");
    match chars.last() {
        Some('ら') => {
            // Godan ru
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanRu,
                steps,
            });
        }
        Some('な') => {
            // Godan nu
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanNu,
                steps,
            });
        }
        Some('か') => {
            // Godan ku
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanKu,
                steps,
            });
        }
        Some('が') => {
            // Godan gu
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanGu,
                steps,
            });
        }
        Some('ば') => {
            // Godan bu
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanBu,
                steps,
            });
        }
        Some('わ') => {
            // Godan u
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanU,
                steps,
            });
        }
        Some('さ') => {
            // Godan su
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanSu,
                steps,
            });
        }
        Some('た') => {
            // Godan su
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanTsu,
                steps,
            });
        }
        Some('ま') => {
            // Godan su
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanMu,
                steps,
            });
        }
        Some('し') => {
            // Suru verb... Technically not godan, but oh well...
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::Suru,
                steps: steps.clone(),
            });
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::SpecialSuru,
                steps,
            });
        }
        Some('こ') => {
            // Kuru verb... Also not godan
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::Kuru,
                steps: steps.clone(),
            });
        }
        Some('せ') => {
            push_causative(steps, chars.init(), roots);
        }
        _ => {}
    }
}

fn deconj_te(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>) {
    debug!("deconj_te: {chars:?}, {steps:?}");
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::GodanTsu,
        steps: steps.clone().with(Step::Imperative),
    });
    push_te_root(roots, chars, steps.with(Step::Te));
}

fn push_te_root(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>) {
    debug!("push_te_root: {chars:?}, {steps:?}");
    match chars.last() {
        Some('っ') => {
            // Godan ru て
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanRu,
                steps: steps.clone(),
            });
            // Godan tsu て
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanTsu,
                steps: steps.clone(),
            });
            // Godan u て
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanU,
                steps: steps.clone(),
            });
            // iku て
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::Iku,
                steps,
            });
        }
        Some('い') => {
            // Godan u
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanKu,
                steps,
            });
        }
        Some('し') => {
            // Godan su
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanSu,
                steps: steps.clone(),
            });
            // Suru
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::Suru,
                steps: steps.clone(),
            });
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::SpecialSuru,
                steps,
            });
        }
        Some('き') if chars.len() == 1 => {
            // kuru te
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::Kuru,
                steps,
            });
        }
        _ => {
            // Ichidan て
            roots.push(Root {
                text: chars.to_string(),
                kind: RootKind::Ichidan,
                steps,
            });
        }
    }
}

fn deconj_de(roots: &mut Vec<Root>, chars: &[char], mut steps: Vec<Step>) {
    debug!("deconj_de");
    if chars[chars.len().saturating_sub(2)..] == ['な', 'い'] {
        steps.insert(0, Step::Naide);
        push_negative_root(&chars[..chars.len() - 2], roots, steps);
        return;
    }
    steps.insert(0, Step::Te);
    push_de_root(roots, chars, steps);
}

fn push_de_root(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>) {
    match chars.last() {
        Some('い') => {
            // Godan gu te
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanGu,
                steps: steps.clone(),
            });
        }
        Some('ん') => {
            // Godan bu te
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanBu,
                steps: steps.clone(),
            });
            // Godan nu te
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanNu,
                steps: steps.clone(),
            });
            // Godan mu te
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanMu,
                steps,
            });
        }
        _ => {}
    }
}

fn deconj_ta(roots: &mut Vec<Root>, chars: &[char], mut steps: Vec<Step>) {
    debug!("deconj_ta: {chars:?}, {steps:?}");
    steps.insert(0, Step::Ta);
    push_ta_root(chars, roots, steps.clone());
    let Some((last, chars)) = chars.split_last() else {
        return;
    };
    match last {
        'っ' => {
            if let Some('ゃ') = chars.last() {
                deconj_small_ya(roots, chars.init().to_vec(), steps.clone());
            }
            if let Some(('か', chars)) = chars.split_last() {
                debug!("かった... い adjective past");
                steps.pop(); // It's not casual past た after all
                roots.push(Root {
                    text: chars.to_string(),
                    kind: RootKind::IAdjective,
                    steps: steps.clone().with(Step::Katta),
                });
                if let Some('な') = chars.last() {
                    steps.insert(0, Step::Nakatta);
                    push_negative_root(chars.init(), roots, steps);
                }
            }
        }
        'い' => push_i_cont_root(steps, chars.to_vec(), roots),
        'て' => {
            steps.insert(0, Step::ContRuAbbrev);
            deconj_te(roots, chars, steps);
        }
        'で' => {
            steps.insert(0, Step::ContRuAbbrev);
            deconj_de(roots, chars, steps);
        }
        _ => (),
    }
}

fn push_ta_root(chars: &[char], roots: &mut Vec<Root>, steps: Vec<Step>) {
    debug!("push_ta_root: {chars:?}, {steps:?}");
    push_e_root(roots, chars, steps.clone().with(Step::Potential), false);
    match chars.last() {
        Some('っ') => {
            // Godan ru
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanRu,
                steps: steps.clone(),
            });
            // Godan tsu
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanTsu,
                steps: steps.clone(),
            });
            // Godan u
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanU,
                steps: steps.clone(),
            });
            // iku
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::Iku,
                steps,
            });
        }
        Some('し') => {
            deconj_su(roots, chars.init(), steps.clone());
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanSu,
                steps,
            });
        }
        Some('い') => {
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanKu,
                steps,
            });
        }
        Some('き') => {
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::Kuru,
                steps,
            });
        }
        _ => {
            // Ichidan
            roots.push(Root {
                text: chars.to_string(),
                kind: RootKind::Ichidan,
                steps,
            });
        }
    }
}

fn deconj_da(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>) {
    push_da_root(chars, roots, steps.clone().with(Step::Ta));
    if let Some(('ん', init)) = chars.split_last() {
        deconj_expr(init, roots, steps.with(Step::Nda));
    }
}

fn push_da_root(chars: &[char], roots: &mut Vec<Root>, steps: Vec<Step>) {
    match chars.last() {
        Some('ん') => {
            // Godan bu
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanBu,
                steps: steps.clone(),
            });
            // Godan mu
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanMu,
                steps: steps.clone(),
            });
            // Godan nu
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanNu,
                steps,
            });
        }
        Some('い') => {
            // Godan gu
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanGu,
                steps,
            });
        }
        _ => {}
    }
}

fn deconj_u(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>) {
    debug!("deconj_u: {chars:?}, {steps:?}");
    match chars.last() {
        Some('よ') => {
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::Ichidan,
                steps: steps.clone().with(Step::Volitional),
            });
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::Kuru,
                steps: steps.with(Step::Volitional),
            });
        }
        Some('ょ') => deconj_small_you(roots, chars.to_vec(), steps),
        Some('ゃ') => {
            deconj_small_ya(roots, chars.init().to_vec(), steps);
        }
        Some('ぼ') => roots.push(Root {
            text: chars.init().to_string(),
            kind: RootKind::GodanBu,
            steps: steps.with(Step::Volitional),
        }),
        Some('も') => roots.push(Root {
            text: chars.init().to_string(),
            kind: RootKind::GodanMu,
            steps: steps.with(Step::Volitional),
        }),
        Some('の') => roots.push(Root {
            text: chars.init().to_string(),
            kind: RootKind::GodanNu,
            steps: steps.with(Step::Volitional),
        }),
        Some('ろ') => roots.push(Root {
            text: chars.init().to_string(),
            kind: RootKind::GodanRu,
            steps: steps.with(Step::Volitional),
        }),
        Some('そ') => roots.push(Root {
            text: chars.init().to_string(),
            kind: RootKind::GodanSu,
            steps: steps.with(Step::Volitional),
        }),
        Some('と') => roots.push(Root {
            text: chars.init().to_string(),
            kind: RootKind::GodanTsu,
            steps: steps.with(Step::Volitional),
        }),
        Some('こ') => {
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanKu,
                steps: steps.clone().with(Step::Volitional),
            });
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::Iku,
                steps: steps.with(Step::Volitional),
            });
        }
        Some('ご') => roots.push(Root {
            text: chars.init().to_string(),
            kind: RootKind::GodanGu,
            steps: steps.with(Step::Volitional),
        }),
        Some('お') => roots.push(Root {
            text: chars.init().to_string(),
            kind: RootKind::GodanU,
            steps: steps.with(Step::Volitional),
        }),
        None => roots.push(Root {
            text: chars.init().to_string(),
            kind: RootKind::GodanU,
            steps: steps.with(Step::Volitional),
        }),
        _ => {}
    }
}

fn deconj_small_ya(roots: &mut Vec<Root>, mut chars: Vec<char>, mut steps: Vec<Step>) {
    debug!("deconj_small_yau: {chars:?}, {steps:?}");
    match ldbg!(log::Level::Debug, chars.pop()) {
        Some('ち') => {
            steps.insert(0, Step::Chau);
            push_te_root(roots, &chars, steps);
        }
        Some('じ') => {
            steps.insert(0, Step::Chau);
            push_de_root(roots, &chars, steps);
        }
        _ => {}
    }
}

fn deconj_small_you(roots: &mut Vec<Root>, mut chars: Vec<char>, mut steps: Vec<Step>) {
    debug!("deconj_small_you: {chars:?}, {steps:?}");
    let Some('ょ') = chars.pop() else { return };
    if let Some('し') = chars.pop() {
        debug!("しょう...");
        if let Some('ま') = chars.pop() {
            steps.insert(0, Step::Invitational);
            push_masu_root(&chars, roots, steps);
        }
    }
}

fn deconj_ku(roots: &mut Vec<Root>, chars: &[char], mut steps: Vec<Step>) {
    steps.insert(0, Step::AdverbialKu);
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::IAdjective,
        steps,
    });
}

fn deconj_ro(roots: &mut Vec<Root>, chars: &[char], mut steps: Vec<Step>) {
    steps.insert(0, Step::Imperative);
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::Ichidan,
        steps,
    });
}

fn deconj_ke(roots: &mut Vec<Root>, chars: &[char], _steps: Vec<Step>) {
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::GodanKu,
        steps: vec![Step::Imperative],
    });
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::Iku,
        steps: vec![Step::Imperative],
    });
}

fn deconj_ne(roots: &mut Vec<Root>, chars: &[char], _steps: Vec<Step>) {
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::GodanNu,
        steps: vec![Step::Imperative],
    });
}

fn deconj_re(roots: &mut Vec<Root>, chars: &[char], _steps: Vec<Step>) {
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::GodanRu,
        steps: vec![Step::Imperative],
    });
}

fn deconj_su(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>) {
    debug!("deconj_su: {chars:?}, {steps:?}");
    if let Some('ま') = ldbg!(log::Level::Debug, chars.last()) {
        deconj_masu(roots, chars.init(), steps)
    }
}

fn deconj_masu(roots: &mut Vec<Root>, chars: &[char], mut steps: Vec<Step>) {
    debug!("deconj_masu: {chars:?}, {steps:?}");
    steps.insert(0, Step::Masu);
    push_masu_root(chars, roots, steps);
}

fn push_masu_root(chars: &[char], roots: &mut Vec<Root>, steps: Vec<Step>) {
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::Ichidan,
        steps: steps.clone(),
    });
    match chars.last() {
        Some('い') => {
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanU,
                steps,
            });
        }
        Some('き') => {
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanKu,
                steps: steps.clone(),
            });
            roots.push(Root {
                text: chars.to_string(),
                kind: RootKind::Kuru,
                steps: steps.clone(),
            });
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::Iku,
                steps,
            })
        }
        Some('ぎ') => {
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanGu,
                steps,
            });
        }
        Some('し') => {
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanSu,
                steps,
            });
        }
        Some('ち') => {
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanTsu,
                steps,
            });
        }
        Some('に') => {
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanNu,
                steps,
            });
        }
        Some('び') => {
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanBu,
                steps,
            });
        }
        Some('み') => {
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanMu,
                steps,
            });
        }
        Some('り') => {
            roots.push(Root {
                text: chars.init().to_string(),
                kind: RootKind::GodanRu,
                steps,
            });
        }
        _ => {}
    }
}

trait CharsExt {
    fn to_string(&self) -> String;
    /// Everything but last part
    fn init(&self) -> &Self;
}

impl CharsExt for [char] {
    fn to_string(&self) -> String {
        self.iter().collect()
    }

    fn init(&self) -> &Self {
        self.get(..self.len() - 1).unwrap_or(&[])
    }
}

trait StepVecExt {
    fn with(self, step: Step) -> Self;
}

impl StepVecExt for Vec<Step> {
    fn with(mut self, step: Step) -> Self {
        self.insert(0, step);
        self
    }
}

trait RootVecExt {
    fn ichidan(&mut self, text: String, steps: Vec<Step>);
}

impl RootVecExt for Vec<Root> {
    fn ichidan(&mut self, text: String, steps: Vec<Step>) {
        self.push(Root {
            text,
            kind: RootKind::Ichidan,
            steps,
        })
    }
}
