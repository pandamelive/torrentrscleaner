use crate::models::{CleanRule, PreviewTorrent};

pub type RulePredicate = Box<dyn Fn(&PreviewTorrent) -> Option<String> + Send + Sync>;

pub fn compile_rules(rules:&[CleanRule]) -> Vec<RulePredicate> {
    let mut predicates = Vec::new();
    for r in rules {
        if !r.enable { continue; }
        let blacklist = r.blacklist_infohash.clone();
        let min_ratio = r.min_ratio;
        let min_seed = r.min_seed_seconds;
        let name = r.name.clone();

        let pred:RulePredicate = Box::new(move |t:&PreviewTorrent|{
            if blacklist.contains(&t.infohash) {
                return Some("黑名单hash".to_string());
            }
            if t.ratio >= min_ratio && t.seed_time >= min_seed {
                return Some(format!("规则:{} ratio:{},seed:{}", name, t.ratio, t.seed_time));
            }
            None
        });
        predicates.push(pred);
    }
    predicates
}

pub fn apply_precompiled(list:&mut Vec<PreviewTorrent>, predicates:&[RulePredicate]) {
    for item in list {
        for p in predicates {
            if let Some(reason) = p(item) {
                item.will_remove = true;
                item.reason = reason;
                break;
            }
        }
    }
}