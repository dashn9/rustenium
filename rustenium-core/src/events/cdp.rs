use rustenium_cdp_definitions::base::EventResponse;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex as StdMutex;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};
use tokio::task::JoinHandle;

type CdpEventHandler = Arc<
    Mutex<
        dyn FnMut(EventResponse) -> Pin<Box<dyn Future<Output = ()> + Send>>
            + Send
            + Sync
            + 'static,
    >,
>;

pub struct CdpEvent {
    pub id: String,
    /// CDP method names this handler listens to, e.g. `"Page.loadEventFired"`.
    pub methods: Vec<String>,
    pub handler: CdpEventHandler,
}

impl fmt::Debug for CdpEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CdpEvent")
            .field("id", &self.id)
            .field("methods", &self.methods)
            .field("handler", &"<CdpEventHandler>")
            .finish()
    }
}

pub trait CdpEventManagement {
    fn get_events(&mut self) -> &mut Arc<StdMutex<Vec<CdpEvent>>>;
    fn push_event(&mut self, event: CdpEvent);

    fn add_event_handler<F, R>(
        &mut self,
        methods: impl IntoIterator<Item = impl Into<String>>,
        mut handler: F,
    ) -> String
    where
        F: FnMut(EventResponse) -> R + Send + Sync + 'static,
        R: Future<Output = ()> + Send + 'static,
    {
        let id = format!(
            "handler_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        let event = CdpEvent {
            id: id.clone(),
            methods: methods.into_iter().map(|s| s.into()).collect(),
            handler: Arc::new(Mutex::new(move |event| {
                Box::pin(handler(event)) as Pin<Box<dyn Future<Output = ()> + Send>>
            })),
        };
        self.push_event(event);
        id
    }

    fn remove_cdp_event_handler(&mut self, id: &str) {
        self.get_events().lock().unwrap().retain(|e| e.id != id);
    }

    fn event_dispatch(
        &mut self,
    ) -> impl Future<Output = (JoinHandle<()>, UnboundedSender<EventResponse>)> {
        async move {
            let (tx, mut rx) = unbounded_channel::<EventResponse>();
            let cdp_events = self.get_events().clone();
            let handle = tokio::spawn(async move {
                while let Some(event) = rx.recv().await {
                    tracing::debug!("[CdpEventManagement] CDP Event received: {}", &event.identifier());
                    for cdp_event in cdp_events.lock().unwrap().iter() {
                        if cdp_event.methods.contains(&event.identifier().to_string()) {
                            let ch = Arc::clone(&cdp_event.handler);
                            let ce = event.clone();
                            tokio::spawn(async move {
                                (ch.lock().await)(ce).await;
                            });
                        }
                    }
                }
            });
            (handle, tx)
        }
    }
}
