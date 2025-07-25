// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::vec::IntoIter;

use crate::raw::oio;
use crate::raw::*;
use crate::*;

pub struct MemoryLister {
    root: String,
    keys: IntoIter<String>,
}

impl MemoryLister {
    pub fn new(root: &str, keys: Vec<String>) -> Self {
        Self {
            root: root.to_string(),
            keys: keys.into_iter(),
        }
    }
}

impl oio::List for MemoryLister {
    async fn next(&mut self) -> Result<Option<oio::Entry>> {
        match self.keys.next() {
            Some(key) => {
                let mode = if key.ends_with('/') {
                    EntryMode::DIR
                } else {
                    EntryMode::FILE
                };
                let mut path = build_rel_path(&self.root, &key);
                if path.is_empty() {
                    path = "/".to_string();
                }
                Ok(Some(oio::Entry::new(&path, Metadata::new(mode))))
            }
            None => Ok(None),
        }
    }
}
