use bevy::{prelude::*, text::FontSmoothing};
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

pub struct ShowFPSPlugin;

impl Plugin for ShowFPSPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    // Here we define size of our overlay
                    font_size: 42.0,
                    // If we want, we can use a custom font
                    font: default(),
                    // We could also disable font smoothing,
                    font_smoothing: FontSmoothing::default(),
                },
                // We can also change color of the overlay
                text_color: Color::srgb(0.0, 1.0, 0.0),
                enabled: true,
            },
        });
    }
}
