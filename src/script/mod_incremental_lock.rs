// Copyright (c) 2017, All Contributors (see CONTRIBUTORS file)
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//!
//! # Atomic Counter Value: ACV, an incremental stamp 
//!
//! This module handles acces to a global usize count for increasing count needs
//!
//!


//Word Decleration 
word!(ACV, b"\x83ACV");

use std::marker::PhantomData;
use super::{Module, PassResult, Error, Env, EnvId};
use incrementalstamp;

pub struct Handler<'a> {
    phantom: PhantomData<&'a ()>
}

impl<'a> Module<'a> for Handler<'a> {
    fn handle(&mut self, env: &mut Env<'a>, word: &'a [u8], pid: EnvId) -> PassResult<'a> {
        try_word!(env, self.handle_acv(env, word, pid));
        Err(Error::UnknownWord)
    }
}

impl<'a> Handler<'a> {
    pub fn new() -> Self {
        Handler { phantom: PhantomData }
    }

    #[inline]
    pub fn handle_acv(&mut self, env: &mut Env<'a>, word: &'a [u8], _: EnvId) -> PassResult<'a> {
        if word == ACV {
            let c = incrementalstamp::count();
            let buf = c.to_bytes_be();
            let slice = alloc_and_write!(&buf, env);
            env.push(slice);
            
            Ok(())
        }else {
            //Does not match word 
            Err(Error::UnknownWord)
        }
    }

}

