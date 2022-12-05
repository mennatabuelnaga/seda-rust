//! Defines a MainChainConfig type based on features when compiling.

#[cfg(feature = "near")]
mod near;
#[cfg(feature = "near")]
pub type MainChainConfig = near::NearConfig;

// A place holder for when a main chain is not selected.
// It gets overwritten by the selected main chain.
#[cfg(feature = "default")]
mod dummy;
#[cfg(feature = "default")]
pub type MainChainConfig = dummy::DummyConfig;
