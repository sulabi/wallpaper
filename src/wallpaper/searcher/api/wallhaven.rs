use super::WallpaperApi;
use crate::wallpaper::{
    Wallpaper, WallpaperDetails, WallpaperImage,
    error::WallpaperError,
    searcher::{
        SearchQuery,
        query::{PortraitRatio, Ratio, SearchMode, SearchRatio, SquareRatio, WideRatio},
    },
    source::{WallpaperSource, WebSource},
};
use serde::Deserialize;

#[derive(Default)]
pub struct Wallhaven;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct SearchResult {
    url: String,
    id: String,

    #[serde(deserialize_with = "parse_f64")]
    ratio: f64,
}

fn parse_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    value.parse().map_err(serde::de::Error::custom)
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct SearchResponse {
    data: Vec<SearchResult>,
}

impl WallpaperApi for Wallhaven {
    async fn search(
        &self,
        query: &SearchQuery<Self>,
    ) -> Result<Vec<Wallpaper<WebSource<Self>>>, WallpaperError> {
        let url = Self::build_url(query);

        let res: SearchResponse = reqwest::get(url).await?.json().await?;
        let data = res.data;

        Ok(data.into_iter().map(Self::parse_preview).collect())
    }
}

impl WallpaperSource for WebSource<Wallhaven> {
    async fn load_wallpaper(&self) -> Result<WallpaperDetails, WallpaperError> {
        // request extra data using self.id
        todo!()
    }
}

const WIDE_RATIOS: &str = "16x9,16x10,21x9,32x9,48x9";
const PORTRAIT_RATIOS: &str = "9x16,9x18,10x16";
const SQUARE_RATIOS: &str = "1x1,3x2,4x3,5x4";
const ALL_RATIOS: &str = "16x9,16x10,21x9,32x9,48x9,9x16,9x18,10x16,1x1,3x2,4x3,5x4";

impl Ratio<Wallhaven> for WideRatio {
    fn as_str(&self) -> &'static str {
        match self {
            Self::R16x9 => "16x9",
            Self::R16x10 => "16x10",

            Self::R21x9 => "21x9",
            Self::R32x9 => "32x9",
            Self::R48x9 => "48x9",

            Self::All => WIDE_RATIOS,
        }
    }
}

impl Ratio<Wallhaven> for PortraitRatio {
    fn as_str(&self) -> &'static str {
        match self {
            Self::R9x16 => "9x16",
            Self::R9x18 => "9x18",
            Self::R10x16 => "10x16",

            Self::All => PORTRAIT_RATIOS,
        }
    }
}
impl Ratio<Wallhaven> for SquareRatio {
    fn as_str(&self) -> &'static str {
        match self {
            Self::R1x1 => "1x1",
            Self::R3x2 => "3x2",
            Self::R4x3 => "4x3",
            Self::R5x4 => "5x4",

            Self::All => SQUARE_RATIOS,
        }
    }
}

impl Ratio<Wallhaven> for SearchRatio<Wallhaven> {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Wide(wide_ratio) => wide_ratio.as_str(),
            Self::Square(square_ratio) => square_ratio.as_str(),
            Self::Portrait(portrait_ratio) => portrait_ratio.as_str(),
            Self::All => ALL_RATIOS,

            Self::_Api(..) => unreachable!(),
        }
    }
}

impl Wallhaven {
    fn parse_preview(result: SearchResult) -> Wallpaper<WebSource<Self>> {
        Wallpaper {
            id: result.id,
            source: WebSource {
                url: result.url,
                api: Wallhaven,
            },
            image: WallpaperImage {
                ratio: result.ratio,
            },
        }
    }

    fn build_url(query: &SearchQuery<Wallhaven>) -> String {
        format!(
            "https://wallhaven.cc/api/v1/search?q={}&sorting={}&ratios={}",
            match &query.mode {
                SearchMode::Query(q) => q,
                SearchMode::Random => "",
            },
            match &query.mode {
                SearchMode::Query(..) => "relavance",
                SearchMode::Random => "random",
            },
            query.ratio.as_str()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wallpaper::searcher::WebSearcher;

    #[tokio::test]
    async fn search_wallhaven() -> Result<(), WallpaperError> {
        let searcher = WebSearcher::new(Wallhaven);

        let query = SearchQuery::new(SearchMode::Query("mountain".into()), SearchRatio::All);

        let wallpapers = searcher.search(&query).await?;
        assert!(!wallpapers.is_empty(), "No wallpapers received");

        Ok(())
    }
}
