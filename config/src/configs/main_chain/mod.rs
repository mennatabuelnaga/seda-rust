//! Defines a MainChainConfig type based on features when compiling.

#[cfg(feature = "near")]
mod near;
#[cfg(feature = "near")]
pub type MainChainConfig = near::NearConfig;

// A place holder for when a main chain is not selected.
// It gets overwritten by the selected main chain.
#[cfg(not(feature = "near"))]
mod another_config;
#[cfg(not(feature = "near"))]
pub type MainChainConfig = another_config::AnotherConfig;
