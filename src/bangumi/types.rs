use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, Debug)]
#[repr(u32)]
pub enum SubjectType {
    Book = 1,
    Anime = 2,
    Music = 3,
    Game = 4,
    Real = 6,
}

#[derive(Deserialize, Debug)]
pub struct SubjectImage {
    pub large: String,
    pub common: String,
    pub medium: String,
    pub small: String,
    pub grid: String,
}

#[derive(Deserialize, Debug)]
pub struct SubjectRatingCount {
    #[serde(rename = "1")]
    pub s1: u32,
    #[serde(rename = "2")]
    pub s2: u32,
    #[serde(rename = "3")]
    pub s3: u32,
    #[serde(rename = "4")]
    pub s4: u32,
    #[serde(rename = "5")]
    pub s5: u32,
    #[serde(rename = "6")]
    pub s6: u32,
    #[serde(rename = "7")]
    pub s7: u32,
    #[serde(rename = "8")]
    pub s8: u32,
    #[serde(rename = "9")]
    pub s9: u32,
    #[serde(rename = "10")]
    pub s10: u32,
}

#[derive(Deserialize, Debug)]
pub struct SubjectRating {
    pub total: u32,
    pub score: f32,
    pub count: SubjectRatingCount,
}

#[derive(Deserialize, Debug)]
pub struct SubjectCollection {
    pub wish: u32,
    pub collect: u32,
    pub doing: u32,
    pub on_hold: u32,
    pub dropped: u32,
}

#[derive(Deserialize, Debug)]
pub struct SubjectSmall {
    pub id: u32,
    pub url: String,
    #[serde(rename = "type")]
    pub subject_type: SubjectType,
    pub name: String,
    pub name_cn: String,
    pub summary: String,
    pub air_date: String,
    pub air_weekday: u8,
    pub images: SubjectImage,
}

#[derive(Deserialize)]
pub enum EpisodeType {
    Honpen = 0,
    Sp = 1,
    OP = 2,
    ED = 3,
    CM = 4,
    MAD = 5,
    Other = 6,
}

#[derive(Deserialize)]
pub enum EpisodeStatus {
    Air,
    Today,
    NA,
}

#[derive(Deserialize)]
pub struct Episode {
    pub id: u32,
    pub url: String,
    #[serde(rename = "type")]
    pub episode_type: EpisodeType,
    pub sort: u32,
    pub name: String,
    pub name_cn: String,
    pub duration: String,
    pub airdate: String,
    pub comment: u32,
    pub desc: String,
    pub status: EpisodeStatus,
}
