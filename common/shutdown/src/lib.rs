use tokio::sync::{broadcast, mpsc};
use sq_logger::info;
/// Listens for the server shutdown signal.
///
/// Shutdown is signalled using a `broadcast::Receiver`. Only a single value is
/// ever sent. Once a value has been sent via the broadcast channel, the server
/// should shutdown.
///
/// The `Shutdown` struct listens for the signal and tracks that the signal has
/// been received. Callers may query for whether the shutdown signal has been
/// received or not.
#[derive(Debug)]
pub struct Shutdown {
    /// `true` if the shutdown signal has been received
    shutdown: bool,

    /// The receive half of the  channel used to listen for shutdown signal.
    notify: broadcast::Receiver<()>,
}

pub struct Stub {
    pub shutdown: Shutdown,
    pub _shutdown_complete: mpsc::Sender<()>,
}

impl Stub {
    pub fn new(context: &Context) -> Self {
        Stub {
            shutdown: Shutdown::new(context.notify_shutdown.subscribe()),
            _shutdown_complete: context.shutdown_complete_tx.clone(),
        }
    }
}

pub struct Context {
    /// Broadcasts a shutdown signal to all active connections.
    ///
    /// When a graceful shutdown is initiated, a `()` value is sent via
    /// the broadcast::Sender. Each subscriber receives it, reaches a
    /// safe terminal state, and completes the task.
    notify_shutdown: broadcast::Sender<()>,

    /// Used as part of the graceful shutdown process to wait for handlers
    ///  to complete processing.
    ///
    /// Tokio channels are closed once all `Sender` handles go out of scope.
    /// When a channel is closed, the receiver receives `None`. This is
    /// leveraged to detect all handlers completing. When a
    /// handler is initialized, it is assigned a clone of
    /// `shutdown_complete_tx`. When the listener shuts down, it drops the
    /// sender held by this `shutdown_complete_tx` field. Once all handler tasks
    /// complete, all clones of the `Sender` are also dropped. This results in
    /// `shutdown_complete_rx.recv()` completing with `None`. At this point, it
    /// is safe to exit the process.
    shutdown_complete_rx: mpsc::Receiver<()>,
    shutdown_complete_tx: mpsc::Sender<()>,
}

impl Context {
    /// Creates a new shutdown context
    pub fn new() -> Context {
        // We use a broadcast channel to send a shutdown message to active handlers.
        // The call below ignores the receiver of the broadcast pair, and when
        // a receiver is needed, the subscribe() method on the sender is used to create
        // one.
        let (notify_shutdown, _) = broadcast::channel(1);
        let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel(1);
        Context {
            notify_shutdown,
            shutdown_complete_tx,
            shutdown_complete_rx,
        }
    }

    /// Terminate the context
    pub async fn terminate(self) {
        // Extract the `shutdown_complete` receiver and transmitter
        // explicitly drop `shutdown_transmitter`. This is important, as the
        // `.await` below would otherwise never complete.
        let Context {
            mut shutdown_complete_rx,
            shutdown_complete_tx,
            notify_shutdown,
        } = self;
        // When `notify_shutdown` is dropped, all tasks which have `subscribe`d will
        // receive the shutdown signal and can exit
        info!("notify all the handlers to exit ...");
        drop(notify_shutdown);
        info!("wait for all the handlers to exit ...");
        // Drop final `Sender` so the `Receiver` below can complete
        drop(shutdown_complete_tx);

        // Wait for all handlers to finish processing. As the `Sender`
        // handle held by the context has been dropped above, the only remaining
        // `Sender` instances are held by handler tasks. When those drop,
        // the `mpsc` channel will close and `recv()` will return `None`.
        let _ = shutdown_complete_rx.recv().await;
    }
}

impl Shutdown {
    /// Creat a new `Shutdown` backed by the given `broadcast::Receiver`.
    pub fn new(notify: broadcast::Receiver<()>) -> Shutdown {
        Shutdown {
            shutdown: false,
            notify,
        }
    }

    /// Return `true` if the shutdown signal has been received
    pub fn is_shutdown(&self) -> bool {
        self.shutdown
    }

    /// Receive teh shutdown notice, waiting if necessary
    pub async fn recv(&mut self) {
        // If the shutdown signal has already been received, then return
        // immediately.
        if self.shutdown {
            return;
        }

        // Cannot receive a "lag error" as only one value is ever sent.
        let _ = self.notify.recv().await;

        // Remember that signal has been received.
        self.shutdown = true;
    }
}
