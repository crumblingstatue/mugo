use {
    crate::root::{Root, RootKind, Step},
    log::debug,
    log_dbg::ldbg,
};

pub fn deconjugate(word: &str) -> Vec<Root> {
    log::debug!("deconjugate({word})");
    let mut roots = Vec::new();
    let chars: Vec<char> = word.chars().collect();
    let steps = vec![];
    deconj_expr(chars, &mut roots, steps);
    ldbg!(log::Level::Debug, roots)
}

fn deconj_expr(mut chars: Vec<char>, roots: &mut Vec<Root>, steps: Vec<Step>) {
    log::debug!("deconj_expr");
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
    let Some(last_ch) = chars.pop() else {
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
        'ん' => deconj_n(roots, chars, steps),
        'る' => deconj_ru(roots, chars, steps),
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
        'ぬ' => deconj_nu(roots, chars, steps),
        _ => {}
    }
}

fn deconj_nu(roots: &mut Vec<Root>, chars: Vec<char>, steps: Vec<Step>) {
    debug!("deconj_nu");
    push_negative_root(chars, roots, steps.with(Step::Nu));
}

fn deconj_mi(roots: &mut Vec<Root>, chars: Vec<char>, steps: Vec<Step>) {
    debug!("deconj_mi");
    // Godan mu verb stem
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::GodanMu,
        steps: steps.with(Step::Stem),
    });
}

fn deconj_ki(roots: &mut Vec<Root>, chars: Vec<char>, steps: Vec<Step>) {
    debug!("deconj_ki");
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

fn deconj_se(roots: &mut Vec<Root>, chars: Vec<char>, steps: Vec<Step>) {
    debug!("deconj_se");
    // Godan su imperative
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::GodanSu,
        steps: steps.clone().with(Step::Imperative),
    });
    // Causative stem
    push_causative(steps.with(Step::Stem), chars, roots);
}

fn deconj_shi(roots: &mut Vec<Root>, chars: Vec<char>, steps: Vec<Step>) {
    debug!("deconj_shi");
    // Godan su verb stem
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::GodanSu,
        steps: steps.with(Step::Stem),
    });
}

fn deconj_na(roots: &mut Vec<Root>, chars: Vec<char>, mut steps: Vec<Step>) {
    debug!("deconj_na");
    steps.insert(0, Step::Na);
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::NaAdjective,
        steps,
    });
}

fn deconj_ba(roots: &mut Vec<Root>, chars: Vec<char>, mut steps: Vec<Step>) {
    debug!("deconj_ba");
    steps.push(Step::Ba);
    push_e_root(roots, chars, steps);
}

fn push_e_root(roots: &mut Vec<Root>, mut chars: Vec<char>, steps: Vec<Step>) {
    debug!("push_e_root: {chars:?}");
    match chars.pop() {
        Some('え') => roots.push(Root {
            text: chars.to_string(),
            kind: RootKind::GodanU,
            steps,
        }),
        Some('け') => roots.push(Root {
            text: chars.to_string(),
            kind: RootKind::GodanKu,
            steps,
        }),
        Some('げ') => roots.push(Root {
            text: chars.to_string(),
            kind: RootKind::GodanGu,
            steps,
        }),
        Some('せ') => roots.push(Root {
            text: chars.to_string(),
            kind: RootKind::GodanSu,
            steps,
        }),
        Some('て') => roots.push(Root {
            text: chars.to_string(),
            kind: RootKind::GodanTsu,
            steps,
        }),
        Some('ね') => roots.push(Root {
            text: chars.to_string(),
            kind: RootKind::GodanNu,
            steps,
        }),
        Some('べ') => roots.push(Root {
            text: chars.to_string(),
            kind: RootKind::GodanBu,
            steps,
        }),
        Some('め') => roots.push(Root {
            text: chars.to_string(),
            kind: RootKind::GodanMu,
            steps,
        }),
        Some('れ') => {
            roots.push(Root {
                text: chars.to_string(),
                kind: RootKind::GodanRu,
                steps: steps.clone(),
            });
            if let Some('ら') = chars.pop() {
                debug!("ra!");
                roots.ichidan(chars.to_string(), steps.clone());
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

fn deconj_ra(roots: &mut Vec<Root>, mut chars: Vec<char>, mut steps: Vec<Step>) {
    debug!("deconj_ra");
    match chars.pop() {
        Some('が') => {
            let Some('な') = chars.pop() else { return };
            steps.push(Step::Nagara);
            push_masu_root(chars, roots, steps);
        }
        Some('た') => {
            steps.push(Step::Tara);
            push_ta_root(chars, roots, steps);
        }
        _ => {}
    }
}

fn deconj_ri(roots: &mut Vec<Root>, mut chars: Vec<char>, mut steps: Vec<Step>) {
    debug!("deconj_ri");
    // Godan ru verb stem
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::GodanRu,
        steps: steps.clone().with(Step::Stem),
    });
    match chars.pop() {
        Some('た') => {
            steps.push(Step::Tari);
            push_ta_root(chars, roots, steps);
        }
        Some('だ') => {
            steps.push(Step::Tari);
            push_da_root(chars, roots, steps);
        }
        _ => (),
    }
}

fn deconj_ka(roots: &mut Vec<Root>, chars: Vec<char>, mut steps: Vec<Step>) {
    debug!("deconj_ka");
    steps.push(Step::Ka);
    deconj_expr(chars, roots, steps);
}

fn deconj_zu(roots: &mut Vec<Root>, chars: Vec<char>, mut steps: Vec<Step>) {
    debug!("deconj_zu");
    steps.push(Step::Zu);
    push_negative_root(chars, roots, steps);
}

fn deconj_ru(roots: &mut Vec<Root>, mut chars: Vec<char>, mut steps: Vec<Step>) {
    debug!("deconj_ru");
    // Try for potential
    steps.push(Step::Potential);
    push_e_root(roots, chars.clone(), steps.clone());
    steps.pop();
    match chars.pop() {
        Some('て') => {
            steps.push(Step::ContRuAbbrev);
            deconj_te(roots, chars, steps);
        }
        Some('で') => {
            steps.push(Step::ContRuAbbrev);
            deconj_de(roots, chars, steps);
        }
        Some('い') => push_i_cont_root(steps, chars, roots),
        Some('せ') => {
            debug!("seru");
            push_causative(steps, chars, roots);
        }
        _ => {}
    }
}

fn push_i_cont_root(mut steps: Vec<Step>, mut chars: Vec<char>, roots: &mut Vec<Root>) {
    match chars.pop() {
        Some('て') => {
            steps.insert(0, Step::Continuous);
            deconj_te(roots, chars, steps);
        }
        Some('で') => {
            steps.insert(0, Step::Continuous);
            deconj_de(roots, chars, steps);
        }
        _ => {}
    }
}

fn push_causative(mut steps: Vec<Step>, chars: Vec<char>, roots: &mut Vec<Root>) {
    steps.insert(0, Step::Causative);
    if let Some('さ') = ldbg!(log::Level::Debug, chars.last()) {
        roots.ichidan(chars.init().to_string(), steps.clone())
    }
    push_godan_negative_root(chars, roots, steps);
}

fn deconj_n(roots: &mut Vec<Root>, mut chars: Vec<char>, mut steps: Vec<Step>) {
    debug!("deconj_n");
    let Some('せ') = chars.pop() else {
        return;
    };
    let Some('ま') = chars.pop() else {
        return;
    };
    steps.insert(0, Step::Masen);
    push_masu_root(chars, roots, steps);
}

fn deconj_i(roots: &mut Vec<Root>, mut chars: Vec<char>, mut steps: Vec<Step>) {
    debug!("deconj_i");
    match chars.pop() {
        Some('な') => deconj_nai(roots, chars, steps),
        Some('さ') => deconj_sai(roots, chars, steps),
        Some('こ') => roots.push(Root {
            text: chars.to_string(),
            kind: RootKind::Kuru,
            steps: steps.with(Step::Imperative),
        }),
        Some('た') => {
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
    push_masu_root(chars, roots, steps);
}

fn deconj_nai(roots: &mut Vec<Root>, chars: Vec<char>, mut steps: Vec<Step>) {
    debug!("deconj_nai");
    steps.insert(0, Step::Nai);
    push_negative_root(chars, roots, steps);
}

/// Push both godan and ichidan negative roots
fn push_negative_root(chars: Vec<char>, roots: &mut Vec<Root>, steps: Vec<Step>) {
    if !push_godan_negative_root(chars.clone(), roots, steps.clone()) {
        roots.ichidan(chars.to_string(), steps);
    }
}

/// Push godan negative root, return false if there is no godan root match
fn push_godan_negative_root(chars: Vec<char>, roots: &mut Vec<Root>, steps: Vec<Step>) -> bool {
    debug!("push_godan_negative_root: {chars:?}");
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
            push_causative(steps, chars.init().to_owned(), roots);
        }
        _ => return false,
    }
    true
}

fn deconj_te(roots: &mut Vec<Root>, chars: Vec<char>, steps: Vec<Step>) {
    debug!("deconj_te");
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::GodanTsu,
        steps: steps.clone().with(Step::Imperative),
    });
    push_te_root(roots, chars, steps.with(Step::Te));
}

fn push_te_root(roots: &mut Vec<Root>, chars: Vec<char>, steps: Vec<Step>) {
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

fn deconj_de(roots: &mut Vec<Root>, chars: Vec<char>, mut steps: Vec<Step>) {
    debug!("deconj_de");
    if chars[chars.len().saturating_sub(2)..] == ['な', 'い'] {
        steps.insert(0, Step::Naide);
        push_negative_root(chars[..chars.len() - 2].to_owned(), roots, steps);
        return;
    }
    steps.insert(0, Step::Te);
    push_de_root(roots, chars, steps);
}

fn push_de_root(roots: &mut Vec<Root>, chars: Vec<char>, steps: Vec<Step>) {
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

fn deconj_ta(roots: &mut Vec<Root>, mut chars: Vec<char>, mut steps: Vec<Step>) {
    debug!("deconj_ta");
    steps.insert(0, Step::Ta);
    push_ta_root(chars.clone(), roots, steps.clone());
    match chars.pop() {
        Some('っ') => {
            if let Some('ゃ') = chars.last() {
                let mut chars = chars.clone();
                chars.pop();
                deconj_small_ya(roots, chars, steps.clone());
            }
            if let Some('か') = chars.pop() {
                debug!("かった... い adjective past");
                steps.pop(); // It's not casual past た after all
                roots.push(Root {
                    text: chars.to_string(),
                    kind: RootKind::IAdjective,
                    steps: steps.clone().with(Step::Katta),
                });
                if let Some('な') = chars.pop() {
                    steps.insert(0, Step::Nakatta);
                    push_negative_root(chars, roots, steps);
                }
            }
        }
        Some('い') => push_i_cont_root(steps, chars, roots),
        Some('て') => {
            steps.insert(0, Step::ContRuAbbrev);
            deconj_te(roots, chars, steps);
        }
        Some('で') => {
            steps.insert(0, Step::ContRuAbbrev);
            deconj_de(roots, chars, steps);
        }
        _ => (),
    }
}

fn push_ta_root(chars: Vec<char>, roots: &mut Vec<Root>, steps: Vec<Step>) {
    debug!("push_ta_root");
    push_e_root(roots, chars.clone(), steps.clone().with(Step::Potential));
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
            deconj_su(roots, chars.init().to_owned(), steps.clone());
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

fn deconj_da(roots: &mut Vec<Root>, mut chars: Vec<char>, steps: Vec<Step>) {
    push_da_root(chars.clone(), roots, steps.clone().with(Step::Ta));
    if let Some('ん') = chars.pop() {
        deconj_expr(chars, roots, steps.with(Step::Nda));
    }
}

fn push_da_root(chars: Vec<char>, roots: &mut Vec<Root>, steps: Vec<Step>) {
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

fn deconj_u(roots: &mut Vec<Root>, mut chars: Vec<char>, steps: Vec<Step>) {
    debug!("deconj_u");
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
        Some('ょ') => deconj_small_you(roots, chars, steps),
        Some('ゃ') => {
            chars.pop();
            deconj_small_ya(roots, chars, steps);
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
    debug!("deconj_small_yau");
    match ldbg!(log::Level::Debug, chars.pop()) {
        Some('ち') => {
            steps.insert(0, Step::Chau);
            push_te_root(roots, chars, steps);
        }
        Some('じ') => {
            steps.insert(0, Step::Chau);
            push_de_root(roots, chars, steps);
        }
        _ => {}
    }
}

fn deconj_small_you(roots: &mut Vec<Root>, mut chars: Vec<char>, mut steps: Vec<Step>) {
    debug!("deconj_small_you: {chars:?}");
    let Some('ょ') = chars.pop() else { return };
    if let Some('し') = chars.pop() {
        debug!("しょう...");
        if let Some('ま') = chars.pop() {
            steps.insert(0, Step::Invitational);
            push_masu_root(chars, roots, steps);
        }
    }
}

fn deconj_ku(roots: &mut Vec<Root>, chars: Vec<char>, mut steps: Vec<Step>) {
    steps.insert(0, Step::AdverbialKu);
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::IAdjective,
        steps,
    });
}

fn deconj_ro(roots: &mut Vec<Root>, chars: Vec<char>, mut steps: Vec<Step>) {
    steps.insert(0, Step::Imperative);
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::Ichidan,
        steps,
    });
}

fn deconj_ke(roots: &mut Vec<Root>, chars: Vec<char>, _steps: Vec<Step>) {
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

fn deconj_ne(roots: &mut Vec<Root>, chars: Vec<char>, _steps: Vec<Step>) {
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::GodanNu,
        steps: vec![Step::Imperative],
    });
}

fn deconj_re(roots: &mut Vec<Root>, chars: Vec<char>, _steps: Vec<Step>) {
    roots.push(Root {
        text: chars.to_string(),
        kind: RootKind::GodanRu,
        steps: vec![Step::Imperative],
    });
}

fn deconj_su(roots: &mut Vec<Root>, chars: Vec<char>, steps: Vec<Step>) {
    debug!("deconj_su");
    if let Some('ま') = ldbg!(log::Level::Debug, chars.last()) {
        deconj_masu(roots, chars.clone().init().to_owned(), steps)
    }
}

fn deconj_masu(roots: &mut Vec<Root>, chars: Vec<char>, mut steps: Vec<Step>) {
    debug!("deconj_masu");
    steps.insert(0, Step::Masu);
    push_masu_root(chars, roots, steps);
}

fn push_masu_root(chars: Vec<char>, roots: &mut Vec<Root>, steps: Vec<Step>) {
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
        self.split_last().unwrap_or((&'\0', &[])).1
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
