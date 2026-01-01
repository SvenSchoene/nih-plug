//! Registration functions for Vizia's built-in fonts. These are not enabled by default in
//! `nih_plug_vizia` to save on binary size.
//!
//! NOTE: In vizia 0.3.0, the built-in fonts (Roboto, Tabler Icons) are no longer exported.
//! Users should provide their own fonts or use the fonts from nih_plug_assets.

use vizia::prelude::*;

/// The font name for the Roboto font family. Comes in regular, bold, and italic variations.
///
/// NOTE: Roboto fonts are no longer bundled with vizia 0.3.0. You will need to provide your own
/// font data or use a different font.
pub const ROBOTO: &str = "Roboto";

/// The font name for the icon font (tabler-icons).
///
/// NOTE: Tabler Icons are no longer bundled with vizia 0.3.0. You will need to provide your own
/// icon font data.
pub const TABLER_ICONS: &str = "tabler-icons";

/// Register Roboto Regular font.
///
/// NOTE: This function is a no-op in vizia 0.3.0 as Roboto is no longer bundled.
/// Provide your own font data using `cx.add_font_mem()`.
pub fn register_roboto(_cx: &mut Context) {
    // Roboto fonts are no longer bundled with vizia 0.3.0
    // Users should provide their own font data
}

/// Register Roboto Bold font.
///
/// NOTE: This function is a no-op in vizia 0.3.0 as Roboto is no longer bundled.
pub fn register_roboto_bold(_cx: &mut Context) {
    // Roboto fonts are no longer bundled with vizia 0.3.0
}

/// Register Roboto Italic font.
///
/// NOTE: This function is a no-op in vizia 0.3.0 as Roboto is no longer bundled.
pub fn register_roboto_italic(_cx: &mut Context) {
    // Roboto fonts are no longer bundled with vizia 0.3.0
}

/// Register Tabler Icons font.
///
/// NOTE: This function is a no-op in vizia 0.3.0 as Tabler Icons are no longer bundled.
pub fn register_tabler_icons(_cx: &mut Context) {
    // Tabler Icons are no longer bundled with vizia 0.3.0
}
