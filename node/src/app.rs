use actix::prelude::*;

// Node Actor definition
pub struct App;

impl Actor for App {
    type Context = Context<App>;

}
