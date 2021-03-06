use std::collections::{HashMap,HashSet};

#[derive(Debug,Clone,PartialEq)]
pub enum MemberRole {
    Inner(),
    Outer(),
    Unused(),
}

#[derive(Debug,Clone,PartialEq)]
pub enum MemberType {
    Node(),
    Way(),
    Relation(),
}

#[derive(Debug,Clone,PartialEq)]
pub struct Member {
    pub id: u64,
    pub role: MemberRole,
    pub member_type: MemberType,
    pub reverse: bool,
}

impl Member {
    pub fn new(id: u64, role: MemberRole, member_type: MemberType) -> Self {
        Member { id, role, member_type, reverse: false }
    }
    pub fn drain(members: &mut Vec<Member>, ways: &HashMap<u64, Vec<u64>>) -> () {
        // the only members that matter for rendering purposes are inner and outer ways
        members.drain_filter(|m| {
            let ref_len = ways.get(&m.id).and_then(|refs| Some(refs.len()));
            match (&m.role,&m.member_type,ref_len) {
                (MemberRole::Inner(),MemberType::Way(),Some(len)) => len == 0,
                (MemberRole::Outer(),MemberType::Way(),Some(len)) => len == 0,
                _ => true,
            }
        });
    }
    pub fn sort(mmembers: &[Member], ways: &HashMap<u64, Vec<u64>>) -> Vec<Member> {
        if mmembers.is_empty() { return vec![] }
        // first re-order so that the first role is an outer
        let n = mmembers.len();
        let mut ms = Vec::with_capacity(n);
        let members = {
            if mmembers.first().map(|m| m.role.clone()) == Some(MemberRole::Inner()) {
                let iend = mmembers.iter().position(|m| {
                    m.role != MemberRole::Inner()
                }).unwrap_or(0);
                let (ref0,ref1) = mmembers.get(iend)
                    .and_then(|m| ways.get(&m.id))
                    .map(|refs| (refs.first(),refs.last()))
                    .unwrap_or((None,None))
                ;
                let oend = mmembers[iend..].iter().enumerate()
                    .position(|(i,m)| {
                        m.role != MemberRole::Outer()
                        || (i > 0 && ways.get(&m.id) // closes an outer loop
                            .map(|refs| {
                                refs.first() == ref0 || refs.first() == ref1
                                || refs.last() == ref0 || refs.last() == ref1
                            })
                            .unwrap_or(false)
                        )
                    })
                    .map(|i| i + iend)
                    .unwrap_or(mmembers.len())
                    .min(mmembers.len());
                let inners = mmembers[0..iend].to_vec();
                let outers = mmembers[iend..oend].to_vec();
                let post = mmembers[oend..].to_vec();
                ms.extend(outers);
                ms.extend(inners);
                ms.extend(post);
                &ms
            } else {
                mmembers
            }
        };

        let mut first_ids: HashMap<u64,Vec<usize>> = HashMap::new();
        let mut last_ids: HashMap<u64,Vec<usize>> = HashMap::new();
        for (i,m) in members.iter().enumerate() {
            let refs = ways.get(&m.id).unwrap();
            let fi = refs.first().unwrap();
            let li = refs.last().unwrap();
            match first_ids.get_mut(fi) {
                None => { first_ids.insert(*fi, vec![i]); },
                Some(ii) => ii.push(i),
            }
            match last_ids.get_mut(li) {
                None => { last_ids.insert(*li, vec![i]); },
                Some(ii) => ii.push(i),
            }
        }
        let mut i = 0;
        let mut j = 0;
        let mut visited = HashSet::new();
        let mut sorted = vec![];
        let mut reverse = false;
        while i < members.len() {
            if visited.contains(&i) {
                i = j;
                j += 1;
                continue;
            }
            visited.insert(i);
            let mut m = members[i].clone();
            let id = m.id;
            m.reverse = reverse;
            sorted.push(m);
            if !ways.contains_key(&id) {
                i = j;
                j += 1;
                continue;
            }
            let refs = ways.get(&id).unwrap();
            let first = refs.first().unwrap();
            let last = refs.last().unwrap();

            let efifs = vec![];
            let elifs = vec![];
            let efils = vec![];
            let elils = vec![];
            let fifs = first_ids.get(first).or(Some(&efifs)).unwrap();
            let lifs = last_ids.get(first).or(Some(&elifs)).unwrap();
            let fils = first_ids.get(last).or(Some(&efils)).unwrap();
            let lils = last_ids.get(last).or(Some(&elils)).unwrap();

            let max_k = fifs.len().max(lifs.len()).max(fils.len()).max(lils.len());
            let mut found = false;
            for k in 0..max_k {
                let fif = fifs.get(k);
                let lif = lifs.get(k);
                let fil = fils.get(k);
                let lil = lils.get(k);
                if fil.is_some() && !visited.contains(&fil.unwrap()) {
                    i = *fil.unwrap();
                    sorted.last_mut().unwrap().reverse = false;
                    reverse = false;
                    found = true;
                    break;
                } else if lif.is_some() && !visited.contains(&lif.unwrap()) {
                    i = *lif.unwrap();
                    sorted.last_mut().unwrap().reverse = true;
                    reverse = true;
                    found = true;
                    break;
                } else if lil.is_some() && !visited.contains(&lil.unwrap()) {
                    i = *lil.unwrap();
                    sorted.last_mut().unwrap().reverse = false;
                    reverse = true;
                    found = true;
                    break;
                } else if fif.is_some() && !visited.contains(&fif.unwrap()) {
                    i = *fif.unwrap();
                    //sorted.last_mut().unwrap().reverse = false;
                    reverse = false;
                    found = true;
                    break;
                }
            }
            if !found {
                i = j;
                j += 1;
            }
        }
        sorted
    }
}
