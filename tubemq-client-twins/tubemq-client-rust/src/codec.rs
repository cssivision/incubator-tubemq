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
const PACKET_BUFFER_LEN: usize = 4;

#[derive(Debug)]
pub struct Codec {
    state: DecodeState,
    body: PacketBody,
}

impl Default for Codec {
    fn default() -> Self {
        Codec {
            state: DecodeState::Head,
            body: PacketBody {
                state: 0,
                size: 0,
                list_size: 0,
                body: BytesMut::new(),
            },
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug)]
struct PacketBody {
    body: BytesMut,
    state: usize,
    size: u32,
    list_size: u32,
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
        buf.reserve(PACKET_BUFFER_LEN);

        Ok(Some(PacketHead {
            token,
            serial_no,
            list_size,
        }))
    }
}

#[derive(Debug)]
pub struct Request {
    head: PacketHead,
    body: BytesMut,
}

impl Codec {
    fn decode_data(
        &mut self,
        head: &PacketHead,
        buf: &mut BytesMut,
    ) -> io::Result<Option<BytesMut>> {
        if self.body.list_size == 0 {
            let n = self.body.body.len();
            return Ok(Some(self.body.body.split_to(n)));
        }

        if self.body.state == 0 {
            if buf.len() < PACKET_BUFFER_LEN {
                return Ok(None);
            }
            self.body.size = buf.get_u32();
            self.body.state = 1;
            buf.reserve(self.body.size as usize);
        }

        if self.body.state == 1 {
            if buf.len() < self.body.size as usize {
                return Ok(None);
            }

            let data = buf.split_to(self.body.state);
            self.body.body.extend_from_slice(&data);
            self.body.state = 0;
            self.body.size = 0;
            buf.reserve(PACKET_BUFFER_LEN);
            self.body.list_size -= 1;
        }

        if head.list_size == 0 {
            let n = self.body.body.len();
            return Ok(Some(self.body.body.split_to(n)));
        }

        Ok(None)
    }
}

impl Decoder for Codec {
    type Item = Request;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Request>> {
        let head = match self.state {
            DecodeState::Head => match PacketHeadCodec.decode(buf)? {
                Some(v) => {
                    self.state = DecodeState::Data(v.clone());
                    self.body.list_size = v.list_size;
                    v
                }
                None => return Ok(None),
            },
            DecodeState::Data(ref v) => v.clone(),
        };

        match self.decode_data(&head, buf)? {
            Some(body) => {
                // Update the decode state
                self.state = DecodeState::Head;

                // Make sure the buffer has enough space to read the next head
                buf.reserve(PACKET_HEAD_LEN);
                Ok(Some(Request { head, body }))
            }
            None => Ok(None),
        }
    }
}
