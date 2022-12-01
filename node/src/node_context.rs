

use actix::{*, dev::{ContextParts, AsyncContextParts, ContextFut, Mailbox, Envelope, ToEnvelope}};
use seda_chain_adapters::MainChainAdapterTrait;
use tokio::sync::oneshot::Sender;


/// Execution context for `WebSockets` actors
pub struct NodeContext<A, MC>
where
    A: Actor<Context = NodeContext<A, MC>>,
    MC: MainChainAdapterTrait,
{
    pub inner: ContextParts<A>,

    pub node_config: NodeConfig,
    pub main_chain_config: MC::Config,
}

impl<A, MC> ActorContext for NodeContext<A, MC>
where
    A: Actor<Context = Self>,
    MC: MainChainAdapterTrait,
{
    fn stop(&mut self) {
        self.inner.stop();
    }

    fn terminate(&mut self) {
        self.inner.terminate()
    }

    fn state(&self) -> ActorState {
        self.inner.state()
    }
}



impl<A, MC> AsyncContext<A> for NodeContext<A, MC>
where
    A: Actor<Context = Self>,
    MC: MainChainAdapterTrait,
{
    #[inline]
    fn spawn<F>(&mut self, fut: F) -> SpawnHandle
    where
        F: ActorFuture<A, Output = ()> + 'static,
    {
        self.inner.spawn(fut)
    }

    #[inline]
    fn wait<F>(&mut self, fut: F)
    where
        F: ActorFuture<A, Output = ()> + 'static,
    {
        self.inner.wait(fut)
    }

    #[doc(hidden)]
    #[inline]
    fn waiting(&self) -> bool {
        self.inner.waiting()
            || self.inner.state() == ActorState::Stopping
            || self.inner.state() == ActorState::Stopped
    }

    #[inline]
    fn cancel_future(&mut self, handle: SpawnHandle) -> bool {
        self.inner.cancel_future(handle)
    }

    #[inline]
    fn address(&self) -> Addr<A> {
        self.inner.address()
    }
}

impl<A, MC> NodeContext<A, MC>
where
    A: Actor<Context = Self>,
    MC: MainChainAdapterTrait,
{
    #[inline]
    /// Create a new Node Context from a request and an actor
    pub fn create(actor: A, node_config: NodeConfig, main_chain_config: MC::Config) -> NodeContextFut<A, MC> {
        let mb = Mailbox::default();
        let ctx = NodeContext {
            inner: ContextParts::new(mb.sender_producer()),
            node_config,
            main_chain_config,
        };
        NodeContextFut::new(ctx, actor, mb)
    }

    /// Create a new Node Context
    pub fn with_factory<F>(f: F, node_config: NodeConfig, main_chain_config: MC::Config) -> NodeContextFut<A, MC>
    where
        F: FnOnce(&mut Self) -> A + 'static,
    {
        let mb = Mailbox::default();
        let mut ctx = Self {
            inner: ContextParts::new(mb.sender_producer()),
            node_config,
            main_chain_config,

        };

        let act = f(&mut ctx);
        NodeContextFut::new(ctx, act, mb)
    }


}

impl<A, MC> AsyncContextParts<A> for NodeContext<A, MC>
where
    A: Actor<Context = Self>,
    MC: MainChainAdapterTrait,
{
    fn parts(&mut self) -> &mut ContextParts<A> {
        &mut self.inner
    }
}

pub struct NodeContextFut<A, MC>
where
    A: Actor<Context = NodeContext<A, MC>>,
    MC: MainChainAdapterTrait,
{
    fut: ContextFut<A, NodeContext<A, MC>>,
}

impl<A, MC> NodeContextFut<A, MC>
where
    A: Actor<Context = NodeContext<A, MC>>,
    MC: MainChainAdapterTrait,
{
    fn new(ctx: NodeContext<A, MC>, act: A, mailbox: Mailbox<A>) -> Self {
        let fut = ContextFut::new(ctx, act, mailbox);
        NodeContextFut { fut }
    }
}

impl<A, M, MC> ToEnvelope<A, M> for NodeContext<A, MC>
where
    A: Actor<Context = NodeContext<A, MC>> + Handler<M>,
    M: Message + Send + 'static,
    M::Result: Send,
    MC: MainChainAdapterTrait,
{
    fn pack(msg: M, tx: Option<Sender<M::Result>>) -> Envelope<A> {
        Envelope::new(msg, tx)
    }
}

pub fn start<A, MC>(actor: A, node_config: NodeConfig, main_chain_config: MC::Config) -> Addr<A>
where
    A: Actor<Context = NodeContext<A, MC>>,
    MC: MainChainAdapterTrait,
{
    let ctx = NodeContext::create(actor, node_config, main_chain_config);
    let addr = ctx.fut.address();
    actix_rt::spawn(ctx.fut);
    addr
}






