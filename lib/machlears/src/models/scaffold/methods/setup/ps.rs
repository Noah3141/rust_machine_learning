use std::default;

/// Choose which fields to print. 
/// 
/// Can be manually declared, or a preset used:
/// 
/// 1. `PrintStyle::default()`
/// 2. `PrintStyle::all()`
/// 3. `PrintStyle::silent()`
/// 
pub struct PrintStyle {
    iterations: bool,
    epochs: bool,
    alpha: bool,
    weights: bool,
}

impl PrintStyle {

    pub fn silent() -> PrintStyle {
        PrintStyle {
            iterations: false,
            epochs: false,
            alpha: false,
            weights: false,
        }
    }

    pub fn default() -> PrintStyle {
        PrintStyle {
            iterations: false,
            epochs: true,
            alpha: true,
            weights: true,
        }
    }

    pub fn all() -> PrintStyle {
        PrintStyle {
            iterations: true,
            epochs: true,
            alpha: true,
            weights: true,
        }
    }

}