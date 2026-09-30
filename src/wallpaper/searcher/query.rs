use crate::wallpaper::searcher::api::WallpaperApi;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct SearchQuery<A: WallpaperApi> {
    pub mode: SearchMode,
    pub ratio: SearchRatio<A>,

    _api: PhantomData<A>,
}

#[derive(Debug, Default)]
pub enum SearchMode {
    Query(String),

    #[default]
    Random,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum WideRatio {
    // wide ratios
    R16x9,
    R16x10,

    // ultrawide ratios
    R21x9,
    R32x9,
    R48x9,

    All,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum PortraitRatio {
    R9x16,
    R10x16,
    R9x18,

    All,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum SquareRatio {
    R1x1,
    R3x2,
    R4x3,
    R5x4,

    All,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum SearchRatio<A> {
    Wide(WideRatio),
    Portrait(PortraitRatio),
    Square(SquareRatio),

    All,

    _Api(PhantomData<A>),
}

pub trait Ratio<A: WallpaperApi> {
    fn as_str(&self) -> &'static str;
}

impl<A: WallpaperApi> SearchQuery<A> {
    pub fn new(mode: SearchMode, ratio: SearchRatio<A>) -> SearchQuery<A> {
        Self {
            mode,
            ratio,
            _api: PhantomData,
        }
    }
}
