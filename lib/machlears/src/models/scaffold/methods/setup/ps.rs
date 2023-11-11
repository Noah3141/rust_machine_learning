use std::default;

use crate::models::{active::entity::Mach, info::entity::Info};

/// Choose which fields to print. 
/// 
/// Can be manually declared, or a preset used:
/// 
/// 1. `PrintStyle::Default()`
/// 2. `PrintStyle::All()`
/// 3. `PrintStyle::Silent()`
/// 4. `PrintStyle::Custom(func)` where `func` is a provided closure that does the printing you want given the data in `Mach`
/// 
/// For instance, you could try the following:
/// 
/// ```
///     let json_fn = &|mach| {
///         println!("{}", serde_json::to_string_pretty(&mach).expect("deser"))
///     };
///     let print_style = PrintStyle::Custom(json_fn);
/// ```
pub enum PrintStyle<'func, Func> where Func: Fn(&Info) -> () + ?Sized {
    Silent,
    Default,
    All,
    Custom(&'func Func)
} 

