use clap::Subcommand;

mod bridge;

#[derive(Debug, Subcommand)]
pub enum Task {
    Bridge(bridge::Bridge),
}

impl Task {
    pub fn handle(self) {
        match self {
            Self::Bridge(bridge) => bridge.handle(),
        }
    }
}
