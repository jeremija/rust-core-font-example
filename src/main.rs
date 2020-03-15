use core_text::font_collection::create_for_family;
use core_text::font_descriptor::SymbolicTraitAccessors;
use core_text::font_descriptor::TraitAccessors;
use core_text::font_descriptor::kCTFontFormatBitmap;
use core_text::font_descriptor::kCTFontFormatTrueType;
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  for i in 1..args.len() {
    let font = args[i].clone();
    let ct_collection = create_for_family(&font).unwrap();
    let descriptors = ct_collection.get_descriptors();
    println!("Descriptors for font: '{}'", font);
    if let Some(descriptors) = descriptors {
      for descriptor in descriptors.iter() {
        let traits = descriptor.traits().symbolic_traits();
        let font_format = descriptor.font_format().unwrap();
        println!("--\n\
          name: '{}' \n\
          display name: '{}'\n\
          is_bitmap: {}\n\
          is_truetype: {}\n\
          bold: {}",
          descriptor.font_name(),
          descriptor.display_name(),
          font_format == kCTFontFormatBitmap,
          font_format == kCTFontFormatTrueType,
          traits.is_bold(),
        );
      }
    }
  }
}
