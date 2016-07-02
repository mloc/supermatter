/*
  Copyright 2016 Colm Hickey <colmohici@gmail.com>

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.
*/

use zmq;
use std::error;
use std::fmt;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Error {
    SocketError(zmq::Error),
    UnicodeDecodeError(FromUtf8Error),
    MalformedMessage,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "supermatter error"
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
