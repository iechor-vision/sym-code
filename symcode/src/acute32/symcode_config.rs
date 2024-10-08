use visioniechor::PointF64;
use crate::interfaces::{Debugger, DummyDebugger};
use super::{Acute32Library, CircleFinder};

pub struct Acute32SymcodeConfig {
    pub symbol_library: Box<Acute32Library>, // To be referenced in RecognizerInput
    pub finder: CircleFinder,
    
    pub code_width: usize,
    pub code_height: usize,

    pub symbol_width: usize,
    pub symbol_height: usize,

    /// The centers of the finders
    pub finder_positions: Vec<PointF64>,
    /// The top-left corners of the glyphs
    pub glyph_anchors: Vec<PointF64>,

    pub max_extra_finder_candidates: usize,
    pub rectify_error_threshold: f64,
    pub stat_tolerance: f64,
    pub max_encoding_difference: usize,
    pub empty_cluster_threshold: f64,
    pub quiet_zone_width: usize,

    pub debugger: Box<dyn Debugger>,
}

impl Default for Acute32SymcodeConfig {
    fn default() -> Self {
        Self {
            code_width: 785,
            code_height: 785,
            symbol_width: 155,
            symbol_height: 155,
            finder_positions: vec![
                PointF64::new(392.0, 160.0),
                PointF64::new(392.0, 392.0),
                PointF64::new(160.0, 625.0),
                PointF64::new(625.0, 625.0),
            ],
            glyph_anchors: vec![
                PointF64::new(82.0, 82.0),
                PointF64::new(82.0, 315.0),
                PointF64::new(315.0, 547.0),
                PointF64::new(547.0, 315.0),
                PointF64::new(547.0, 82.0),
            ],
            max_extra_finder_candidates: 3,
            rectify_error_threshold: 0.5,
            stat_tolerance: 0.36,
            max_encoding_difference: 3,
            empty_cluster_threshold: 0.15,
            symbol_library: Box::new(Acute32Library::default()),
            finder: CircleFinder::default(),
            quiet_zone_width: 10,
            debugger: Box::new(DummyDebugger),
        }
    }
}

impl Acute32SymcodeConfig {
    #[inline]
    pub fn max_finder_candidates(&self) -> usize {
        self.finder_positions.len() + self.max_extra_finder_candidates
    }

    #[inline]
    pub fn absolute_empty_cluster_threshold(&self, image_width: usize, image_height: usize) -> u64 {
        (self.empty_cluster_threshold * (image_width * image_height) as f64) as u64
    }

    #[inline]
    pub fn num_glyphs_in_code(&self) -> usize {
        self.glyph_anchors.len()
    }
}
