use libp2p::{
    identity, PeerId, Swarm,
    swarm::SwarmEvent,
    tcp::TokioTcpConfig, core::upgrade,
    mplex, plaintext, request_response::{RequestResponse, RequestResponseCodec, ProtocolName, RequestResponseConfig, RequestResponseEvent}
};
use anyhow::Result;
use async_trait::async_trait;
use std::{io, iter};

#[derive(Clone)]
struct HeaderExchangeProtocol();
#[derive(Clone)]
struct HeaderExchangeCodec();

#[derive(Debug, Clone)]
pub struct HeaderRequest(pub u64);
#[derive(Debug, Clone)]
pub struct HeaderResponse(pub Vec<u8>);

#[async_trait]
impl RequestResponseCodec for HeaderExchangeCodec {
    type Protocol = HeaderExchangeProtocol;
    type Request = HeaderRequest;
    type Response = HeaderResponse;

    async fn read_request<T>(&mut self, _: &HeaderExchangeProtocol, io: &mut T) -> io::Result<Self::Request>
    where T: AsyncRead + Unpin + Send {
        let mut buf = vec![]; io.read_to_end(&mut buf).await?;
        let height = u64::from_be_bytes(buf.try_into().unwrap());
        Ok(HeaderRequest(height))
    }

    async fn read_response<T>(&mut self, _: &HeaderExchangeProtocol, io: &mut T) -> io::Result<Self::Response>
    where T: AsyncRead + Unpin + Send {
        let mut buf = vec![]; io.read_to_end(&mut buf).await?;
        Ok(HeaderResponse(buf))
    }

    async fn write_request<T>(&mut self, _: &HeaderExchangeProtocol, io: &mut T, HeaderRequest(height): HeaderRequest) -> io::Result<()>
    where T: AsyncWrite + Unpin + Send {
        io.write_all(&height.to_be_bytes()).await
    }

    async fn write_response<T>(&mut self, _: &HeaderExchangeProtocol, io: &mut T, HeaderResponse(data): HeaderResponse) -> io::Result<()>
    where T: AsyncWrite + Unpin + Send {
        io.write_all(&data).await
    }
}

pub type NetworkBehaviour = RequestResponse<HeaderExchangeCodec>;

pub async fn build_swarm(settings: &crate::config::Settings) -> Result<Swarm<NetworkBehaviour>> {
    let key = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(key.public());
    let transport = TokioTcpConfig::new().nodelay(true)
        .upgrade(upgrade::Version::V1)
        .authenticate(plaintext::PlainText2Config { local_public_key: key.public() })
        .multiplex(mplex::MplexConfig::new())
        .boxed();

    let protocols = iter::once((HeaderExchangeProtocol(), RequestResponseConfig::default()));
    let behaviour = RequestResponse::new(HeaderExchangeCodec(), protocols, RequestResponseConfig::default());
    let mut swarm = Swarm::with_tokio_executor(transport, behaviour, peer_id);

    let addr = settings.peer_addr.parse()?;
    swarm.dial(addr)?;

    Ok(swarm)
}
