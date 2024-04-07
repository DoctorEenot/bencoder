/// All tests are done with real data sent by QbitTorrent, captured via Wireshark
use bencode::utils::*;
use serde::Deserialize;
use serde::{self, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Node {
    pub id: String,
    pub addr: SocketAddr,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Error(u64, String);

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Ping {
    #[serde(with = "binary_string")]
    id: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct FindNode {
    #[serde(with = "binary_string")]
    id: Vec<u8>,
    #[serde(with = "binary_string")]
    target: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct FindNodeResponse {
    #[serde(with = "binary_string")]
    id: Vec<u8>,
    nodes: Vec<Node>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct GetPeers {
    #[serde(with = "binary_string")]
    id: Vec<u8>,
    #[serde(with = "binary_string")]
    info_hash: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct GetPeersResponse {
    #[serde(with = "binary_string")]
    id: Vec<u8>,
    #[serde(with = "binary_string")]
    nodes: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct AnnouncePeer {
    #[serde(with = "binary_string")]
    id: Vec<u8>,
    #[serde(with = "binary_string")]
    info_hash: Vec<u8>,
    port: u16,
    implied_port: bool,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "q", content = "a")]
pub enum Query {
    Ping(Ping),
    FindNode(FindNode),
    GetPeers(GetPeers),
    AnnouncePeer(AnnouncePeer),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(untagged)]
pub enum Response {
    GetPeers(GetPeersResponse),
    AnnouncePeer(Ping),
    Ping(Ping),
    //FindNode(FindNodeResponse),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ResponseWrapper {
    #[serde(rename = "r")]
    inner: Response,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "y")]
pub enum MessageData {
    #[serde(rename = "q")]
    Query(Query),
    #[serde(rename = "r")]
    Response(ResponseWrapper),
    #[serde(rename = "e")]
    Error(Error),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Message {
    #[serde(with = "binary_string")]
    pub t: Vec<u8>,
    #[serde(flatten)]
    pub inner: MessageData,
}

#[test]
fn test_deserialization() {
    let message = "64313a6164323a6273693165323a696432303a5fbb5ddff6ddf9074480fa82f538a8d80f33d405393a696e666f5f6861736832303a5fbb5ddff6ddf9074480fa8283e35f1fc55353ab65313a71393a6765745f7065657273313a74323aeb8b313a76343a4c54012f313a79313a7165";

    let message = hex::decode(message).unwrap();
    let deserialized: Message = bencode::from_bytes(&message).unwrap();

    let right = Message {
        t: vec![235, 139],
        inner: MessageData::Query(Query::GetPeers(GetPeers {
            id: vec![
                95, 187, 93, 223, 246, 221, 249, 7, 68, 128, 250, 130, 245, 56, 168, 216, 15, 51,
                212, 5,
            ],
            info_hash: vec![
                95, 187, 93, 223, 246, 221, 249, 7, 68, 128, 250, 130, 131, 227, 95, 31, 197, 83,
                83, 171,
            ],
        })),
    };

    assert_eq!(deserialized, right);

    println!("{:?}", deserialized);

    let message = "64313a7264323a696432303a6b5611b3bfb7b8c8372c069a1e22e540609bda5a353a6e6f6465733230383a22f6d37eb95dc510c5330ed90f850d9d95d3992cb125b5c0acae22f6d37eb95dc510c5330ed90f850d9d95d3992cb125b5c0acae22f6d37eb95dc510c5330ed90f850d9d95d3992cb125b5c0acae22f6d37eb95dc510c5330ed90f850d9d95d3992cb125b5c0acae22f6d37eb95dc510c5330ed90f850d9d95d3992cb125b5c0acae22f6d37eb95dc510c5330ed90f850d9d95d3992cb125b5c0acae22f6d37eb95dc510c5330ed90f850d9d95d3992cb125b5c0acae22f6d37eb95dc510c5330ed90f850d9d95d3992cb125b5c0acae65313a74323a42af313a76343a4a420000313a79313a7265";

    let message = hex::decode(message).unwrap();
    let deserialized: Message = bencode::from_bytes(&message).unwrap();

    let right = Message {
        t: vec![66, 175],
        inner: MessageData::Response(ResponseWrapper {
            inner: Response::GetPeers(GetPeersResponse {
                id: vec![
                    107, 86, 17, 179, 191, 183, 184, 200, 55, 44, 6, 154, 30, 34, 229, 64, 96, 155,
                    218, 90,
                ],
                nodes: vec![
                    34, 246, 211, 126, 185, 93, 197, 16, 197, 51, 14, 217, 15, 133, 13, 157, 149,
                    211, 153, 44, 177, 37, 181, 192, 172, 174, 34, 246, 211, 126, 185, 93, 197, 16,
                    197, 51, 14, 217, 15, 133, 13, 157, 149, 211, 153, 44, 177, 37, 181, 192, 172,
                    174, 34, 246, 211, 126, 185, 93, 197, 16, 197, 51, 14, 217, 15, 133, 13, 157,
                    149, 211, 153, 44, 177, 37, 181, 192, 172, 174, 34, 246, 211, 126, 185, 93,
                    197, 16, 197, 51, 14, 217, 15, 133, 13, 157, 149, 211, 153, 44, 177, 37, 181,
                    192, 172, 174, 34, 246, 211, 126, 185, 93, 197, 16, 197, 51, 14, 217, 15, 133,
                    13, 157, 149, 211, 153, 44, 177, 37, 181, 192, 172, 174, 34, 246, 211, 126,
                    185, 93, 197, 16, 197, 51, 14, 217, 15, 133, 13, 157, 149, 211, 153, 44, 177,
                    37, 181, 192, 172, 174, 34, 246, 211, 126, 185, 93, 197, 16, 197, 51, 14, 217,
                    15, 133, 13, 157, 149, 211, 153, 44, 177, 37, 181, 192, 172, 174, 34, 246, 211,
                    126, 185, 93, 197, 16, 197, 51, 14, 217, 15, 133, 13, 157, 149, 211, 153, 44,
                    177, 37, 181, 192, 172, 174,
                ],
            }),
        }),
    };

    assert_eq!(deserialized, right);

    println!("{:?}", deserialized);
}
