/**
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 * <p>
 * http://www.apache.org/licenses/LICENSE-2.0
 * <p>
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/
use std::io;

use bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

const PACKET_HEAD_LEN: usize = 12;

#[derive(Debug)]
pub struct Codec {
    state: DecodeState,
}

impl Default for Codec {
    fn default() -> Self {
        Codec {
            state: DecodeState::Head,
        }
    }
}

#[derive(Debug)]
enum DecodeState {
    Head,
    Data(PacketHead),
}

#[derive(Debug, Clone)]
struct PacketHead {
    token: u32,
    serial_no: u32,
    list_size: u32,
}

struct PacketBody {
    body: BytesMut,
}

#[derive(Debug)]
struct PacketHeadCodec;

impl Decoder for PacketHeadCodec {
    type Item = PacketHead;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<PacketHead>> {
        if buf.len() < PACKET_HEAD_LEN {
            return Ok(None);
        }

        let token = buf.get_u32();
        let serial_no = buf.get_u32();
        let list_size = buf.get_u32();

        // Ensure that the buffer has enough space to read the list[0] length.
        buf.reserve(4);

        Ok(Some(PacketHead {
            token,
            serial_no,
            list_size,
        }))
    }
}

pub struct Request {
    head: PacketBody,
    body: PacketBody,
}

impl Codec {
    fn decode_data(
        &self,
        head: &mut PacketHead,
        buf: &mut BytesMut,
    ) -> io::Result<Option<PacketBody>> {
        if buf.len() < 4 {
            return Ok(None);
        }

        buf.advance(4);

        let body = buf.split_to(0);

        Ok(Some(Request { body }))
    }
}

impl Decoder for Codec {
    type Item = Request;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<TrpcRequest>> {
        let head = match self.state {
            DecodeState::Head => match PacketHeadCodec.decode(buf)? {
                Some(v) => {
                    self.state = DecodeState::Data(v);
                    v
                }
                None => return Ok(None),
            },
            DecodeState::Data(v) => v,
        };

        match self.decode_data(&head, buf)? {
            Some(packet) => {
                // Update the decode state
                self.state = DecodeState::Head;

                // Make sure the buffer has enough space to read the next head
                buf.reserve(PACKET_HEAD_LEN);
                Ok(Some(Request {
                    head,
                    body: packet.packet_body,
                }))
            }
            None => Ok(None),
        }
    }
}
