use std::collections::HashMap;

use bit_vec::BitVec;
use visioniechor::{BinaryImage, Sampler};

use super::{Acute32SymcodeConfig, GlyphLabel, GlyphTrace, Symbol, Trace, image_diff_area};

#[derive(Debug)]
pub struct Acute32Library {
    templates: Vec<Symbol>,
}

impl Default for Acute32Library {
    fn default() -> Self {
        Self { templates: vec![] }
    }
}

impl Acute32Library {
    pub fn is_empty(&self) -> bool {
        self.templates.is_empty()
    }

    pub fn len(&self) -> usize {
        self.templates.len()
    }

    pub fn get_glyph_at(&self, i: usize) -> Option<&Symbol> {
        if i >= self.templates.len() {
            None
        } else {
            Some(&self.templates[i])
        }
    }

    pub fn get_glyph_with_label(&self, label: GlyphLabel) -> Option<&Symbol> {
        for glyph in self.templates.iter() {
            if glyph.label == label {
                return Some(glyph);
            }
        }
        None
    }

    pub fn print_label_and_trace(&self) -> String {
        let list: Vec<String> = self.templates.iter().map(|glyph| {
            format!("{:?}: {:?}\n", glyph.label, glyph.encoding.bits)
        }).collect();
        list.join("")
    }

    pub fn get_labels_grouped_by_trace(&self) -> String {
        let mut map: HashMap<BitVec, Vec<GlyphLabel>> = HashMap::new();

        self.templates.iter().for_each(|glyph| {
            match map.get_mut(&glyph.encoding.bits) {
                Some(labels) => {
                    labels.push(glyph.label);
                },
                None => {
                    map.insert(glyph.encoding.bits.clone(), vec![glyph.label]);
                }
            }
        });

        let list: Vec<String> = map.iter().map(|(bits, labels)| {
            format!("{:?}: {}\n", bits, labels.len())
        }).collect();

        list.join("")
    }

    /// Takes the binary image of the template and the usize representation of the label
    pub fn add_template(&mut self, image: BinaryImage, symcode_config: &Acute32SymcodeConfig) {
        //let image = Sampler::resample_image(&image, symcode_config.symbol_width, symcode_config.symbol_height);
        let label = GlyphLabel::from_usize_representation(self.templates.len());
        //console_log_util(&format!("{:?}\n{}", label, image.to_string()));
        self.templates.push(Symbol::from_image_label(image, label, symcode_config.stat_tolerance));
    }

    pub fn find_most_similar_glyph(&self, image: BinaryImage, symcode_config: &Acute32SymcodeConfig) -> GlyphLabel {
        let image = &Sampler::resample_image(&image, symcode_config.symbol_width, symcode_config.symbol_height);
        let input_encoding = &GlyphTrace::from_image(image, symcode_config.stat_tolerance);
        //console_log_util(&format!("{:?}", input_encoding));

        self.templates.iter()
            .fold( (std::u64::MAX, GlyphLabel::Invalid),
                |(min_error, min_label), template| {
                    if template.encoding.diff(input_encoding) > symcode_config.max_encoding_difference {
                        return (min_error, min_label);
                    }
                    let error = image_diff_area(&template.image, image);
                    if error < min_error {
                        (error, template.label)
                    } else {
                        (min_error, min_label)
                    }
                }
            ).1
    }
}