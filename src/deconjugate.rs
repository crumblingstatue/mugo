use {
    crate::root::{Root, RootKind, Step},
    log::{debug, info},
    log_dbg::ldbg,
};

pub fn deconjugate(word: &str) -> Vec<Root> {
    let mut roots = Vec::new();
    let chars: Vec<char> = word.chars().collect();
    let steps = vec![];
    deconj_expr(&chars, &mut roots, steps);
    debug!("deconjugate({word}) = {roots:#?}");
    roots
}

fn deconj_expr(chars: &[char], roots: &mut Vec<Root>, steps: Vec<Step>) {
    info!("deconj_expr: {chars:?}, {steps:?}");
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
    if let Some((chars, ['け', 'れ', 'ば'])) = chars.split_last_chunk() {
        push_i_adjective_root(roots, chars, steps.clone().with(Step::Kereba));
    }
    if let Some((chars, ['な', 'き', 'ゃ'])) = chars.split_last_chunk() {
        push_negative_root(chars, roots, steps.with(Step::Nakya));
        return;
    }
    if let Some((chars, ['ま', 'す'])) = chars.split_last_chunk() {
        push_masu_root(chars, roots, steps.clone().with(Step::Masu));
        return;
    }
    if let Some((chars, ['ま', 'せ', 'ん'])) = chars.split_last_chunk() {
        push_masu_root(chars, roots, steps.clone().with(Step::Masen));
        return;
    }
    if let Some((chars, ['て', 'く'])) = chars.split_last_chunk() {
        push_te_root(roots, chars, steps.clone().with(Step::Teku));
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
        'さ' => roots.push(Root {
            text: chars.to_string(),
            kind: RootKind::IAdjective,
            steps: steps.with(Step::Sa),
        }),
        'え' => roots.push(Root {
            text: chars.to_string(),
            kind: RootKind::GodanU,
            steps: steps.with(Step::Imperative),
        }),
        'う' => deconj_u(roots, chars, steps),
        'く' => push_i_adjective_root(roots, chars, steps.with(Step::AdverbialKu)),
        'ろ' => roots.push(Root {
            text: chars.to_string(),
            kind: RootKind::Ichidan,
            steps: steps.with(Step::Imperative),
        }),
        'れ' => {
            roots.push(Root {
                text: chars.to_string(),
                kind: RootKind::GodanRu,
                steps: vec![Step::Imperative],
            });
        }
        'け' => {
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
        'げ' => {
            roots.push(Root {
                text: chars.to_string(),
                kind: RootKind::GodanGu,
                steps: steps.clone().with(Step::Imperative),
            });
        }
        'ね' => {
            roots.push(Root {
                text: chars.to_string(),
                kind: RootKind::GodanNu,
                steps: vec![Step::Imperative],
            });
        }
        'る' => push_ichidan_root(chars, roots, steps, false),
        'ず' => deconj_zu(roots, chars, steps),
        'か' => deconj_ka(roots, chars, steps),
        'り' => deconj_ri(roots, chars, steps),
        'ら' => deconj_ra(roots, chars, steps),
        'ば' => deconj_ba(roots, chars, steps),
        'な' => deconj_na(roots, chars, steps),
        'し' => deconj_shi(roots, chars, steps),
        'せ' => deconj_se(roots, chars, steps),
        'き' => deconj_ki(roots, chars, steps),
        'み' => deconj_mi(roots, chars, steps),
        'ぬ' => push_negative_root(chars, roots, steps.with(Step::Nu)),
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
    let Some((last, chars)) = chars.split_last() else {
        return;
    };
    match last {
        'な' => push_negative_root(chars, roots, steps.with(Step::Nai)),
        'た' => push_masu_root(chars, roots, steps.with(Step::Tai)),
        _ => {}
    }
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

fn deconj_na(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>) {
    debug!("deconj_na: {chars:?}, {steps:?}");
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::NaAdjective,
        steps: steps.with(Step::Na),
    });
}

fn deconj_ba(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>) {
    debug!("deconj_ba: {chars:?}, {steps:?}");
    push_e_root(roots, chars, steps.with(Step::Ba), true);
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

fn deconj_ra(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>) {
    debug!("deconj_ra: {chars:?}, {steps:?}");
    let Some((last, chars)) = chars.split_last() else {
        return;
    };
    match last {
        'が' => {
            let Some('な') = chars.last() else { return };
            push_masu_root(chars.init(), roots, steps.with(Step::Nagara));
        }
        'た' => push_ta_root(chars, roots, steps.with(Step::Tara)),
        'だ' => push_da_root(chars, roots, steps.with(Step::Tara)),
        _ => {}
    }
}

fn deconj_ri(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>) {
    debug!("deconj_ri: {chars:?}, {steps:?}");
    // Godan ru verb stem
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::GodanRu,
        steps: steps.clone().with(Step::Stem),
    });
    let Some((last, chars)) = chars.split_last() else {
        return;
    };
    match last {
        'た' => push_ta_root(chars, roots, steps.with(Step::Tari)),
        'だ' => push_da_root(chars, roots, steps.with(Step::Tari)),
        _ => (),
    }
}

fn deconj_ka(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>) {
    debug!("deconj_ka: {chars:?}, {steps:?}");
    deconj_expr(chars, roots, steps.with(Step::Ka));
}

fn deconj_zu(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>) {
    debug!("deconj_zu: {chars:?}, {steps:?}");
    push_negative_root(chars, roots, steps.with(Step::Zu));
}

fn push_i_cont_root(steps: Vec<Step>, chars: &[char], roots: &mut Vec<Root>) {
    match chars.last() {
        Some('て') => deconj_te(roots, chars.init(), steps.with(Step::Continuous)),
        Some('で') => deconj_de(roots, chars.init(), steps.with(Step::Continuous)),
        _ => {}
    }
}

fn push_causative(steps: Vec<Step>, chars: &[char], roots: &mut Vec<Root>) {
    debug!("push_causative: {chars:?}, {steps:?}");
    if let Some('さ') = ldbg!(log::Level::Debug, chars.last()) {
        push_ichidan_root(
            chars.init(),
            roots,
            steps.clone().with(Step::Causative),
            true,
        );
    }
    push_other_negative_root(chars, roots, steps.with(Step::Causative));
}

fn push_passive(steps: Vec<Step>, chars: &[char], roots: &mut Vec<Root>) {
    debug!("push_passive: {chars:?}, {steps:?}");
    match chars.last() {
        Some('ら') => {
            push_ichidan_root(
                chars.init(),
                roots,
                steps.clone().with(Step::Passive),
                false,
            );
        }
        Some('さ') => push_causative(steps.clone().with(Step::Passive), chars.init(), roots),
        _ => (),
    }
    push_other_negative_root(chars, roots, steps.with(Step::Passive));
}

fn deconj_i(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>) {
    debug!("deconj_i: {chars:?}, {steps:?}");
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::GodanU,
        steps: steps.clone().with(Step::Stem),
    });
    let Some((last, chars)) = chars.split_last() else {
        return;
    };
    match last {
        'な' => deconj_nai(roots, chars, steps),
        'さ' => deconj_sai(roots, chars, steps),
        'こ' => roots.push(Root {
            text: chars.to_string(),
            kind: RootKind::Kuru,
            steps: steps.with(Step::Imperative),
        }),
        'た' => push_masu_root(chars, roots, steps.with(Step::Tai)),
        _ => {}
    }
}

fn deconj_sai(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>) {
    let Some(('な', chars)) = chars.split_last() else {
        return;
    };
    push_masu_root(chars, roots, steps.with(Step::Nasai));
}

fn deconj_nai(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>) {
    debug!("deconj_nai: {chars:?}, {steps:?}");
    push_negative_root(chars, roots, steps.with(Step::Nai));
}

/// Push both godan and ichidan negative roots
fn push_negative_root(chars: &[char], roots: &mut Vec<Root>, steps: Vec<Step>) {
    debug!("push_negative_root: : {chars:?}, {steps:?}");
    push_other_negative_root(chars, roots, steps.clone());
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
    let Some((last, chars)) = chars.split_last() else {
        return;
    };
    match last {
        'て' => {
            // Only add continuous ru handling if it's a terminal step...
            // TODO: Figure out how to handle this mess better
            if terminal {
                steps.push(Step::ContRuAbbrev);
            }
            deconj_te(roots, chars, steps);
        }
        'で' => {
            // Only add continuous ru handling if it's a terminal step...
            // TODO: Figure out how to handle this mess better
            if terminal {
                steps.push(Step::ContRuAbbrev);
            }
            deconj_de(roots, chars, steps);
        }
        'い' => push_i_cont_root(steps, chars, roots),
        'せ' => {
            debug!("seru");
            push_causative(steps, chars, roots);
        }
        'れ' => {
            debug!("reru");
            push_passive(steps, chars, roots);
        }
        'こ' => roots.push(Root {
            text: chars.to_string(),
            kind: RootKind::Kuru,
            steps,
        }),
        _ => {}
    }
}

/// Godan, and other negative root handling
fn push_other_negative_root(chars: &[char], roots: &mut Vec<Root>, steps: Vec<Step>) {
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
        Some('く') => push_i_adjective_root(roots, chars.init(), steps.with(Step::AdverbialKu)),
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
    // Anything can be ichidan て root
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::Ichidan,
        steps: steps.clone(),
    });
    let Some((last, chars)) = chars.split_last() else {
        return;
    };
    let kinds = match last {
        'っ' => &[RootKind::GodanRu, RootKind::GodanTsu, RootKind::GodanU, RootKind::Iku][..],
        'い' => &[RootKind::GodanKu],
        'し' => &[RootKind::GodanSu, RootKind::Suru, RootKind::SpecialSuru],
        'き' if chars.is_empty() => &[RootKind::Kuru],
        _ => return,
    };
    for &kind in kinds {
        roots.push(Root {
            text: chars.to_string(),
            kind,
            steps: steps.clone(),
        });
    }
}

fn deconj_de(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>) {
    debug!("deconj_de");
    if chars[chars.len().saturating_sub(2)..] == ['な', 'い'] {
        push_negative_root(&chars[..chars.len() - 2], roots, steps.with(Step::Naide));
        return;
    }
    push_de_root(roots, chars, steps.with(Step::Te));
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

fn deconj_ta(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>) {
    debug!("deconj_ta: {chars:?}, {steps:?}");
    push_ta_root(chars, roots, steps.clone().with(Step::Ta));
    let Some((last, chars)) = chars.split_last() else {
        return;
    };
    match last {
        'っ' => {
            if let Some('ゃ') = chars.last() {
                deconj_small_ya(roots, chars.init(), steps.clone().with(Step::Ta));
            }
            if let Some(('か', chars)) = chars.split_last() {
                debug!("かった... い adjective past");
                roots.push(Root {
                    text: chars.to_string(),
                    kind: RootKind::IAdjective,
                    steps: steps.clone().with(Step::Katta),
                });
                if let Some('な') = chars.last() {
                    push_negative_root(chars.init(), roots, steps.with(Step::Nakatta));
                }
            }
        }
        'い' => push_i_cont_root(steps.with(Step::Ta), chars, roots),
        'れ' => push_passive(steps.with(Step::Ta), chars, roots),
        'て' => deconj_te(roots, chars, steps.with(Step::Ta).with(Step::ContRuAbbrev)),
        'で' => deconj_de(roots, chars, steps.with(Step::Ta).with(Step::ContRuAbbrev)),
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
            if let Some(('ま', chars)) = chars.init().split_last() {
                push_masu_root(chars, roots, steps.clone().with(Step::Masu))
            }
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
    let Some((last, chars)) = chars.split_last() else {
        return;
    };
    let kinds = match last {
        'ょ' => {
            if let Some((chars, ['ま', 'し'])) = chars.split_last_chunk() {
                push_masu_root(chars, roots, steps.with(Step::Invitational));
            }
            return;
        }
        'ゃ' => {
            deconj_small_ya(roots, chars, steps);
            return;
        }
        'よ' => &[RootKind::Ichidan, RootKind::Kuru][..],
        'ぼ' => &[RootKind::GodanBu],
        'も' => &[RootKind::GodanMu],
        'の' => &[RootKind::GodanNu],
        'ろ' => &[RootKind::GodanRu],
        'そ' => &[RootKind::GodanSu],
        'と' => &[RootKind::GodanTsu],
        'こ' => &[RootKind::GodanKu, RootKind::Iku][..],
        'ご' => &[RootKind::GodanGu],
        'お' => &[RootKind::GodanU],
        _ => return,
    };
    for &kind in kinds {
        roots.push(Root {
            text: chars.to_string(),
            kind,
            steps: steps.clone().with(Step::Volitional),
        });
    }
}

fn deconj_small_ya(roots: &mut Vec<Root>, chars: &[char], steps: Vec<Step>) {
    debug!("deconj_small_yau: {chars:?}, {steps:?}");
    match chars.last() {
        Some('ち') => push_te_root(roots, chars.init(), steps.with(Step::Chau)),
        Some('じ') => push_de_root(roots, chars.init(), steps.with(Step::Chau)),
        _ => {}
    }
}

fn push_masu_root(chars: &[char], roots: &mut Vec<Root>, steps: Vec<Step>) {
    debug!("push_masu_root: {chars:?}, {steps:?}");
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::Ichidan,
        steps: steps.clone(),
    });
    let Some((last, chars)) = chars.split_last() else {
        return;
    };
    let kinds = match last {
        'い' => &[RootKind::GodanU],
        'き' => &[RootKind::GodanKu, RootKind::Kuru, RootKind::Iku][..],
        'ぎ' => &[RootKind::GodanGu],
        'し' => &[RootKind::GodanSu],
        'ち' => &[RootKind::GodanTsu],
        'に' => &[RootKind::GodanNu],
        'び' => &[RootKind::GodanBu],
        'み' => &[RootKind::GodanMu],
        'り' => &[RootKind::GodanRu],
        _ => &[],
    };
    for kind in kinds {
        roots.push(Root {
            text: chars.to_string(),
            kind: *kind,
            steps: steps.clone(),
        });
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
