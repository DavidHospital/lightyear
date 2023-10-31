use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Copy, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum PacketType {
    // A packet containing actual data
    #[bitcode_hint(frequency = 100)]
    Data,
    // A packet sent to maintain the connection by preventing a timeout
    KeepAlive,

    ConnectionRequest,
    ConnectionDenied,
    ChallengeRequest,
    ChallengeResponse,
    // Used to request a graceful Client disconnect from the Server
    Disconnect,

    // // An initial handshake message sent by the Client to the Server
    // ClientChallengeRequest,
    // // The Server's response to the Client's initial handshake message
    // ServerChallengeResponse,
    // // The handshake message validating the Client
    // ClientValidateRequest,
    // // The Server's response to the Client's validation request
    // ServerValidateResponse,
    // // The final handshake message sent by the Client
    // ClientConnectRequest,
    // // The final handshake message sent by the Server, indicating that the
    // // connection has been established
    // ServerConnectResponse,
    // // Indicates that the authentication payload was rejected, handshake must restart
    // ServerRejectResponse,
    // A Ping message, used to calculate RTT. Must be responded to with a Pong
    // message
    Ping,
    // A Pong message, used to calculate RTT. Must be the response to all Ping
    // messages
    Pong,
    // A packet containing actual data, but which is fragmented into multiple parts
    #[bitcode_hint(frequency = 5)]
    DataFragment,
}
