use std::collections::HashMap;

use super::{Edge, Plane, Profile};

#[derive(Debug)]
pub struct Profiles {
    pub list: HashMap<usize, Profile>,
    pub plane: Plane,
    pub next_profile_id: usize,
}

impl Profiles {
    pub fn new(plane: Plane) -> Self {
        Self {
            list: HashMap::new(),
            plane,
            next_profile_id: 0,
        }
    }

    pub fn add(&mut self, edge: Edge) {
        let matches = self.match_profiles(&edge);

        if matches.is_empty() {
            let profile_id = self.next_profile_id;
            self.next_profile_id += 1;
            let mut profile = Profile::new(self.plane.clone());
            profile.add(edge);
            self.list.insert(profile_id, profile);
            return;
        }

        if matches.len() == 1 {
            if let Some(profile) = self.list.get_mut(&matches[0]) {
                profile.add(edge);
            }
            return;
        }

        if matches.len() > 1 {
            if let Some(profile) = self.list.get_mut(&matches[0]) {
                profile.add(edge);
            }
            let profile_to_merge_id = matches[1];
            if let Some(profile_to_merge) = self.list.remove(&profile_to_merge_id) {
                if let Some(profile) = self.list.get_mut(&matches[0]) {
                    profile.merge(profile_to_merge);
                }
            }
        }
    }

    pub fn profiles(&self) -> Option<ProfileData> {
        let mut biggest_profile: Option<usize> = None;
        let mut biggest_profile_size = 0.0;

        for (profile_id, profile) in &self.list {
            let area = profile.area();
            if area > biggest_profile_size {
                biggest_profile_size = area;
                biggest_profile = Some(*profile_id);
            }
        }

        let biggest_profile = biggest_profile?;
        let profile = self.list.get(&biggest_profile)?.indices();
        let mut holes = Vec::new();
        for (profile_id, profile) in &self.list {
            if *profile_id == biggest_profile {
                continue;
            }
            holes.push(profile.indices());
        }

        Some(ProfileData { profile, holes })
    }

    fn match_profiles(&self, edge: &Edge) -> Vec<usize> {
        let mut ids = Vec::new();
        for (id, profile) in &self.list {
            if profile.match_edge(edge) > 0 {
                ids.push(*id);
            }
        }
        ids
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProfileData {
    pub profile: Vec<usize>,
    pub holes: Vec<Vec<usize>>,
}
