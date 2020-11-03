use super::LevelFilter;

////// INTERNAL HELPERS

// compile time string prefix calculations
const fn starts_with(p: &str, s: &str) -> bool {

    const fn _starts_with(p: &str, s: &str, i: usize) -> bool {
        i == 0 || {
            p.as_bytes()[i-1] == s.as_bytes()[i-1]
            && _starts_with(p, s, i-1)
        }
    }

    p.len() <= s.len() && _starts_with(p, s, p.len())
}

// An associated constant is need to force the compiler to do the calculation at compile-time
trait Boolean { const VALUE: bool; }

// This is the struct that will have the associated constant
#[allow(dead_code)]
struct StartsWith<const PREFIX: &'static str, const STRING: &'static str>;

// Calculate the associated constant using the const fn implementation
impl<const PREFIX: &'static str, const STRING: &'static str> Boolean for StartsWith<PREFIX, STRING> {
    const VALUE: bool = starts_with(PREFIX, STRING);
}

/// Returns the maximum log level per module at compile time.
///
/// If the `compile_time_filters` feature is enabled, the [`log!`], [`error!`], [`warn!`], [`info!`],
/// [`debug!`], and [`trace!`] macros check this value and discard any message logged at a higher level.
///
/// The maximum level can be controlled with the environment variable `RUST_LOG_FILTERS` at the time of
/// compilation. This variable consists of semicolon separated filters of the form `<module_path>=<level>`
/// and at most one default level of the form `<default_level>.
/// If no default level is used the `Trace` level is used.
/// 
/// Requires the `compile_time_filters` feature.
///
/// # Examples
/// When compiled with:
/// ```
/// RUST_LOG_FILTERS="Warn; problem::module=Trace; problem::module::safe=Error; other_crate=Off"
/// ```
/// Then the following asserts with not fail:
/// ```
/// use LevelFilter::*;
/// assert_eq!(Warn,  max_level_per_module::<"std::mem">());
/// assert_eq!(Warn,  max_level_per_module::<"problem">());
/// assert_eq!(Warn,  max_level_per_module::<"problem::other">());
/// assert_eq!(Trace, max_level_per_module::<"problem::module">());
/// assert_eq!(Trace, max_level_per_module::<"problem::module::submodule">());
/// assert_eq!(Error, max_level_per_module::<"problem::module::safe">());
/// assert_eq!(Error, max_level_per_module::<"problem::module::safe::submodule">());
/// assert_eq!(Off,   max_level_per_module::<"other_crate">());
/// assert_eq!(Off,   max_level_per_module::<"other_crate::submodule">());
/// ```
#[inline(always)]
pub const fn max_level_per_module<const MODULE_PATH: &'static str>() -> LevelFilter {

    use LevelFilter::*;

    // <prefix>=<filter> entries look like this:
    //  if (StartsWith::< "<prefix>" , MODULE_PATH>::VALUE) {
    //      return <filter>;
    //  }
    // These need to be ordered by length (longest first).
    //
    // The default filter will simply be returned last:
    //  <default_filter>

    log_proc_macros::parse_env_filters!();

}

