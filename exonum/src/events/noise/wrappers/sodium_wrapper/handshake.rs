use failure;
use futures::future::{done, Future};
use tokio_codec::{Decoder, Framed};
use tokio_io::{AsyncRead, AsyncWrite};

use std::net::SocketAddr;

use super::wrapper::NoiseWrapper;
use crypto::{
    x25519::{self, into_x25519_keypair, into_x25519_public_key},
    PublicKey, SecretKey,
};
use events::{
    codec::MessagesCodec,
    noise::{Handshake, HandshakeRawMessage, HandshakeResult},
};
use messages::{Connect, Signed};
use node::state::SharedConnectList;
use storage::StorageValue;

/// Params needed to establish secured connection using Noise Protocol.
#[derive(Debug, Clone)]
pub struct HandshakeParams {
    pub public_key: x25519::PublicKey,
    pub secret_key: x25519::SecretKey,
    pub remote_key: Option<x25519::PublicKey>,
    pub connect_list: SharedConnectList,
    pub connect: Signed<Connect>,
    max_message_len: u32,
}

impl HandshakeParams {
    pub fn new(
        public_key: PublicKey,
        secret_key: SecretKey,
        connect_list: SharedConnectList,
        connect: Signed<Connect>,
        max_message_len: u32,
    ) -> Self {
        let (public_key, secret_key) = into_x25519_keypair(public_key, secret_key).unwrap();

        HandshakeParams {
            public_key,
            secret_key,
            max_message_len,
            remote_key: None,
            connect,
            connect_list,
        }
    }

    pub fn set_remote_key(&mut self, remote_key: PublicKey) {
        self.remote_key = Some(into_x25519_public_key(remote_key));
    }
}

#[derive(Debug)]
pub struct NoiseHandshake {
    noise: NoiseWrapper,
    peer_address: SocketAddr,
    max_message_len: u32,
    connect_list: SharedConnectList,
    connect: Signed<Connect>,
}

impl NoiseHandshake {
    pub fn initiator(params: &HandshakeParams, peer_address: &SocketAddr) -> Self {
        let noise = NoiseWrapper::initiator(params);
        NoiseHandshake {
            noise,
            peer_address: *peer_address,
            max_message_len: params.max_message_len,
            connect_list: params.connect_list.clone(),
            connect: params.connect.clone(),
        }
    }

    pub fn responder(params: &HandshakeParams, peer_address: &SocketAddr) -> Self {
        let noise = NoiseWrapper::responder(params);
        NoiseHandshake {
            noise,
            peer_address: *peer_address,
            max_message_len: params.max_message_len,
            connect_list: params.connect_list.clone(),
            connect: params.connect.clone(),
        }
    }

    pub fn read_handshake_msg<S: AsyncRead + 'static>(
        mut self,
        stream: S,
    ) -> impl Future<Item = (S, Self, Vec<u8>), Error = failure::Error> {
        HandshakeRawMessage::read(stream).and_then(move |(stream, msg)| {
            let message = self.noise.read_handshake_msg(&msg.0)?;
            Ok((stream, self, message))
        })
    }

    pub fn write_handshake_msg<S: AsyncWrite + 'static>(
        mut self,
        stream: S,
        msg: &[u8],
    ) -> impl Future<Item = (S, Self), Error = failure::Error> {
        done(self.noise.write_handshake_msg(msg))
            .map_err(|e| e.into())
            .and_then(|buf| HandshakeRawMessage(buf).write(stream))
            .map(move |(stream, _)| (stream, self))
    }

    pub fn finalize<S: AsyncRead + AsyncWrite + 'static>(
        self,
        stream: S,
        message: Vec<u8>,
    ) -> Result<(Framed<S, MessagesCodec>, Vec<u8>), failure::Error> {
        let remote_static_key = {
            // Panic because with selected handshake pattern we must have
            // `remote_static_key` on final step of handshake.
            let rs = self
                .noise
                .session
                .get_remote_static()
                .expect("Remote static key is not present!");
            x25519::PublicKey::from_slice(rs).expect("Remote static key is not valid x25519 key!")
        };

        if !self.is_peer_allowed(&remote_static_key) {
            bail!("peer is not in ConnectList")
        }

        let noise = self.noise.into_transport_mode()?;
        let framed = MessagesCodec::new(self.max_message_len, noise).framed(stream);
        Ok((framed, message))
    }

    fn is_peer_allowed(&self, remote_static_key: &x25519::PublicKey) -> bool {
        self.connect_list
            .peers()
            .iter()
            .map(|info| into_x25519_public_key(info.public_key))
            .any(|key| remote_static_key == &key)
    }
}

impl Handshake for NoiseHandshake {
    fn listen<S>(self, stream: S) -> HandshakeResult<S>
    where
        S: AsyncRead + AsyncWrite + 'static,
    {
        let peer_address = self.peer_address;
        let connect = self.connect.clone();
        let framed = self
            .read_handshake_msg(stream)
            .and_then(|(stream, handshake, _)| {
                handshake.write_handshake_msg(stream, &connect.into_bytes())
            })
            .and_then(|(stream, handshake)| handshake.read_handshake_msg(stream))
            .and_then(|(stream, handshake, message)| handshake.finalize(stream, message))
            .map_err(move |e| {
                e.context(format!("peer {} disconnected", peer_address))
                    .into()
            });
        Box::new(framed)
    }

    fn send<S>(self, stream: S) -> HandshakeResult<S>
    where
        S: AsyncRead + AsyncWrite + 'static,
    {
        let peer_address = self.peer_address;
        let connect = self.connect.clone();
        let framed = self
            .write_handshake_msg(stream, &[])
            .and_then(|(stream, handshake)| handshake.read_handshake_msg(stream))
            .and_then(|(stream, handshake, message)| {
                (
                    handshake.write_handshake_msg(stream, &connect.into_bytes()),
                    Ok(message),
                )
            })
            .and_then(|((stream, handshake), message)| handshake.finalize(stream, message))
            .map_err(move |e| {
                e.context(format!("peer {} disconnected", peer_address))
                    .into()
            });
        Box::new(framed)
    }
}
