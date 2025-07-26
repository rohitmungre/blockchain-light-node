use crate::{network, header_store::HeaderStore};
use anyhow::Result;
use libp2p::swarm::SwarmEvent;
use tokio::sync::mpsc;

pub struct LightClient {
    store: HeaderStore,
    swarm: network::NetworkBehaviour,
    events_rx: mpsc::UnboundedReceiver<network::RequestResponseEvent<network::HeaderRequest, network::HeaderResponse>>,
}

impl LightClient {
    pub async fn new(settings: &crate::config::Settings) -> Result<Self> {
        let store = HeaderStore::open("headers.db")?;
        let mut swarm = network::build_swarm(settings).await?;
        let (tx, rx) = mpsc::unbounded_channel();
        // hook swarm events to tx…
        Ok(Self { store, swarm: swarm.behaviour_mut().clone(), events_rx: rx })
    }

    pub async fn sync_headers(&mut self) -> Result<()> {
        // request next height
        let next = self.store.last_height().unwrap_or(0) + 1;
        self.swarm.send_request(&(), network::HeaderRequest(next));
        // drive the swarm
        while let Some(event) = self.events_rx.recv().await {
            if let network::RequestResponseEvent::Message { message, .. } = event {
                let network::HeaderResponse(data) = message.into_response();
                if HeaderStore::verify_header(&data, "expected_hash_here") {
                    self.store.put(next, &data)?;
                    println!("Stored header at height {}", next);
                }
            }
        }
        Ok(())
    }
}
