use strum_macros::{Display, EnumCount, EnumIter, EnumString};

#[derive(
    Debug,
    Display,
    PartialEq,
    Ord,
    PartialOrd,
    Eq,
    Copy,
    Clone,
    Hash,
    EnumIter,
    EnumString,
    EnumCount,
    Serialize,
    Deserialize,
    typename::TypeName,
)]
#[strum(serialize_all = "snake_case")]
pub enum Ease {
    Linear,

    QuadraticIn,
    QuadraticOut,
    QuadraticInOut,

    CubicIn,
    CubicOut,
    CubicInOut,

    QuarticIn,
    QuarticOut,
    QuarticInOut,

    QuinticIn,
    QuinticOut,
    QuinticInOut,

    SineIn,
    SineOut,
    SineInOut,

    CircularIn,
    CircularOut,
    CircularInOut,
    // Fuck this ease, it's weird and who likes it
    // ExponentialIn,
    // ExponentialOut,
    // ExponentialInOut,

    // Eventually, maybe!
    // ElasticIn,
    // ElasticOut,
    // ElasticInOut,

    // BackIn,
    // BackOut,
    // BackInOut,

    // BounceIn,
    // BounceOut,
    // BounceInOut,
}
