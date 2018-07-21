use base64;
use futures::prelude::*;
use http::{header, Response, StatusCode};
use sha1;
use tokio_codec::Framed;
use tokio_io::{AsyncRead, AsyncWrite};
use websocket::codec::ws::{Context as WsContext, MessageCodec};
use websocket::OwnedMessage;

use tsukuyomi::error::HttpError;
use tsukuyomi::upgrade::UpgradeContext;
use tsukuyomi::{Error, Input};

#[derive(Debug, Fail)]
enum HandshakeError {
    #[fail(display = "The header is missing: `{}'", name)]
    MissingHeader { name: &'static str },

    #[fail(display = "The header value is invalid: `{}'", name)]
    InvalidHeader { name: &'static str },

    #[fail(display = "The value of `Sec-WebSocket-Key` is invalid")]
    InvalidSecWebSocketKey,

    #[fail(display = "The value of `Sec-WebSocket-Version` must be equal to '13'")]
    InvalidSecWebSocketVersion,
}

impl HttpError for HandshakeError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}

fn parse_handshake_request(input: &Input) -> Result<String, HandshakeError> {
    match input.headers().get(header::UPGRADE).map(|h| h.as_bytes()) {
        Some(b"websocket") => (),
        Some(..) => return Err(HandshakeError::InvalidHeader { name: "Upgrade" }),
        None => return Err(HandshakeError::MissingHeader { name: "Upgrade" }),
    }

    match input.headers().get(header::CONNECTION).map(|h| h.as_bytes()) {
        Some(b"Upgrade") => (),
        Some(..) => return Err(HandshakeError::InvalidHeader { name: "Connection" }),
        None => return Err(HandshakeError::MissingHeader { name: "Connection" }),
    }

    let accept_hash = match input.headers().get("Sec-WebSocket-Key") {
        Some(h) => {
            let decoded = base64::decode(h).map_err(|_| HandshakeError::InvalidSecWebSocketKey)?;
            if decoded.len() != 16 {
                return Err(HandshakeError::InvalidSecWebSocketKey);
            }

            let mut m = sha1::Sha1::new();
            m.update(h.as_bytes());
            m.update(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11");

            base64::encode(&m.digest().bytes()[..])
        }
        None => {
            return Err(HandshakeError::MissingHeader {
                name: "Sec-WebSocket-Key",
            })
        }
    };

    match input.headers().get("Sec-WebSocket-Version").map(|h| h.as_bytes()) {
        Some(b"13") => {}
        _ => return Err(HandshakeError::InvalidSecWebSocketVersion),
    }

    // TODO: Sec-WebSocket-Protocol, Sec-WebSocket-Extension

    Ok(accept_hash)
}
pub fn start(input: &mut Input) -> Result<Response<()>, Error> {
    let accept_hash = parse_handshake_request(input)?;

    input.body_mut().on_upgrade(move |cx: UpgradeContext| on_upgrade(cx.io));

    // TODO: Sec-WebSocket-Protocol, Sec-WebSocket-Extension
    Response::builder()
        .status(StatusCode::SWITCHING_PROTOCOLS)
        .header(header::UPGRADE, "websocket")
        .header(header::CONNECTION, "Upgrade")
        .header("Sec-WebSocket-Accept", accept_hash.as_str())
        .body(())
        .map_err(Error::internal_server_error)
}

pub fn on_upgrade(
    io: impl AsyncRead + AsyncWrite + Send + 'static,
) -> impl Future<Item = (), Error = ()> + Send + 'static {
    let framed = Framed::new(io, MessageCodec::default(WsContext::Server));
    let (sink, stream) = framed.split();
    stream
        .take_while(|m| Ok(!m.is_close()))
        .filter_map(|m| {
            println!("Message from client: {:?}", m);
            match m {
                OwnedMessage::Ping(p) => Some(OwnedMessage::Pong(p)),
                OwnedMessage::Pong(_) => None,
                _ => Some(m),
            }
        })
        .forward(sink)
        .and_then(|(_, sink)| sink.send(OwnedMessage::Close(None)))
        .then(|_| Ok(()))
}
