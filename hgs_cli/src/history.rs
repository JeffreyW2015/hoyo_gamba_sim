// use std::collections::HashMap;

// use hgs_core::enums::{FiveStarType, FourStarBanner, FourStarLoss, FourStarType, Pull};

// pub struct PullHistory {
//     pulls: Vec<Pull>,
//     five_star_metadata: FiveStarMetadata,
//     four_star_metadata: FourStarMetadata,
// }

// impl PullHistory {
//     pub fn new() -> Self {
//         PullHistory {
//             pulls: Vec::new(),
//             five_star_metadata: FiveStarMetadata::new(),
//             four_star_metadata: FourStarMetadata::new(),
//         }
//     }

//     pub fn add(&mut self, pull: Pull) {
//         self.pulls.push(pull);

//         self.five_star_metadata.pity += 1;
//         self.four_star_metadata.pity += 1;

//         match pull {
//             Pull::FiveStar(five_star) => {
//                 self.five_star_metadata.process(five_star);
//             }
//             Pull::FourStar(four_star) => {
//                 self.four_star_metadata.process(four_star);
//             }
//             _ => {}
//         }
//     }

//     pub fn total_count(&self) -> u64 {
//         self.pulls.len() as u64
//     }

//     pub fn five_star_metadata(&self) -> &FiveStarMetadata {
//         &self.five_star_metadata
//     }

//     pub fn four_star_metadata(&self) -> &FourStarMetadata {
//         &self.four_star_metadata
//     }
// }

// pub struct FiveStarMetadata {
//     pub pity_to_pulls: HashMap<u64, HashMap<FiveStarType, u64>>,
//     pub limited_pulls: u64,
//     pub standard_pulls: u64,
//     pub pity: u64,

//     pub next_guaranteed: bool,
//     pub wins: u64,
//     pub losses: u64,
//     pub guaranteeds: u64,
// }

// impl FiveStarMetadata {
//     fn new() -> Self {
//         FiveStarMetadata {
//             pity_to_pulls: HashMap::new(),
//             limited_pulls: 0,
//             standard_pulls: 0,
//             pity: 0,
//             next_guaranteed: false,
//             wins: 0,
//             losses: 0,
//             guaranteeds: 0,
//         }
//     }

//     pub fn total(&self) -> u64 {
//         self.limited_pulls + self.standard_pulls
//     }

//     pub fn _pity_of_type(&self, pity: u64, five_star: FiveStarType) -> u64 {
//         self.pity_to_pulls
//             .get(&pity)
//             .and_then(|inner| inner.get(&five_star))
//             .copied()
//             .unwrap_or(0)
//     }

//     fn process(&mut self, five_star: FiveStarType) {
//         let pity_entry = self
//             .pity_to_pulls
//             .entry(self.pity)
//             .or_insert(HashMap::new());

//         match five_star {
//             FiveStarType::Limited => {
//                 pity_entry
//                     .entry(FiveStarType::Limited)
//                     .and_modify(|count| *count += 1)
//                     .or_insert(1);
//                 self.limited_pulls += 1;
//                 if self.next_guaranteed {
//                     self.guaranteeds += 1;
//                 } else {
//                     self.wins += 1
//                 }
//                 self.next_guaranteed = false;
//             }
//             FiveStarType::Standard => {
//                 pity_entry
//                     .entry(FiveStarType::Standard)
//                     .and_modify(|count| *count += 1)
//                     .or_insert(1);
//                 self.standard_pulls += 1;
//                 self.losses += 1;
//                 self.next_guaranteed = true;
//             }
//         }
//         self.pity = 0;
//     }
// }

// pub struct FourStarMetadata {
//     pub pity_to_pulls: HashMap<u64, HashMap<FourStarType, u64>>,
//     pub a_pulls: u64,
//     pub b_pulls: u64,
//     pub c_pulls: u64,
//     pub character_non_banner_pulls: u64,
//     pub light_cone_pulls: u64,
//     pub pity: u64,

//     pub next_guaranteed: bool,
//     pub wins: u64,
//     pub losses: u64,
//     pub guaranteeds: u64,
// }

// impl FourStarMetadata {
//     fn new() -> Self {
//         FourStarMetadata {
//             pity_to_pulls: HashMap::new(),
//             a_pulls: 0,
//             b_pulls: 0,
//             c_pulls: 0,
//             character_non_banner_pulls: 0,
//             light_cone_pulls: 0,
//             pity: 0,
//             next_guaranteed: false,
//             wins: 0,
//             losses: 0,
//             guaranteeds: 0,
//         }
//     }

//     pub fn banners(&self) -> u64 {
//         self.a_pulls + self.b_pulls + self.c_pulls
//     }

//     pub fn non_banners(&self) -> u64 {
//         self.character_non_banner_pulls + self.light_cone_pulls
//     }

//     pub fn total(&self) -> u64 {
//         self.banners() + self.non_banners()
//     }

//     pub fn pity_of_type(&self, pity: u64, four_star: FourStarType) -> u64 {
//         self.pity_to_pulls
//             .get(&pity)
//             .and_then(|inner| inner.get(&four_star))
//             .copied()
//             .unwrap_or(0)
//     }

//     fn process(&mut self, four_star: FourStarType) {
//         let pity_entry = self
//             .pity_to_pulls
//             .entry(self.pity)
//             .or_insert(HashMap::new());

//         match four_star {
//             FourStarType::Banner(banner) => {
//                 if self.next_guaranteed {
//                     self.guaranteeds += 1;
//                 } else {
//                     self.wins += 1;
//                 }
//                 match banner {
//                     FourStarBanner::A => {
//                         self.a_pulls += 1;
//                         pity_entry
//                             .entry(FourStarType::Banner(FourStarBanner::A))
//                             .and_modify(|count| *count += 1)
//                             .or_insert(1);
//                     }
//                     FourStarBanner::B => {
//                         self.b_pulls += 1;
//                         pity_entry
//                             .entry(FourStarType::Banner(FourStarBanner::B))
//                             .and_modify(|count| *count += 1)
//                             .or_insert(1);
//                     }
//                     FourStarBanner::C => {
//                         self.c_pulls += 1;
//                         pity_entry
//                             .entry(FourStarType::Banner(FourStarBanner::C))
//                             .and_modify(|count| *count += 1)
//                             .or_insert(1);
//                     }
//                 }
//             }
//             FourStarType::Loss(loss) => {
//                 self.losses += 1;
//                 match loss {
//                     FourStarLoss::Character => {
//                         self.character_non_banner_pulls += 1;
//                         pity_entry
//                             .entry(FourStarType::Loss(FourStarLoss::Character))
//                             .and_modify(|count| *count += 1)
//                             .or_insert(1);
//                     }
//                     FourStarLoss::LightCone => {
//                         self.light_cone_pulls += 1;
//                         pity_entry
//                             .entry(FourStarType::Loss(FourStarLoss::LightCone))
//                             .and_modify(|count| *count += 1)
//                             .or_insert(1);
//                     }
//                 }
//             }
//         }
//         self.pity = 0;
//     }
// }
