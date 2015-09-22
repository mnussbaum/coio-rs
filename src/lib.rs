// The MIT License (MIT)

// Copyright (c) 2015 Y. T. Chung <zonyitoo@gmail.com>

//  Permission is hereby granted, free of charge, to any person obtaining a
//  copy of this software and associated documentation files (the "Software"),
//  to deal in the Software without restriction, including without limitation
//  the rights to use, copy, modify, merge, publish, distribute, sublicense,
//  and/or sell copies of the Software, and to permit persons to whom the
//  Software is furnished to do so, subject to the following conditions:
//
//  The above copyright notice and this permission notice shall be included in
//  all copies or substantial portions of the Software.
//
//  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
//  OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
//  FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
//  DEALINGS IN THE SOFTWARE.

//! Coroutine scheduling with asynchronous I/O support

#![feature(catch_panic, drain, reflect_marker, fnbox)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate context;
extern crate mio;
extern crate bytes;
extern crate hyper;
extern crate url;
extern crate deque;
extern crate rand;
extern crate libc;

pub use scheduler::{Scheduler, JoinHandle};
pub use options::Options;
pub use promise::Promise;

pub mod net;
pub mod sync;
pub mod scheduler;
pub mod options;
pub mod promise;
mod runtime;
mod coroutine;

/// Spawn a new Coroutine
#[inline(always)]
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where F: FnOnce() -> T + Send + 'static,
          T: Send + 'static
{
    Scheduler::spawn(f)
}

/// Spawn a new Coroutine with options
#[inline(always)]
pub fn spawn_opts<F, T>(f: F, opts: Options) -> JoinHandle<T>
    where F: FnOnce() -> T + Send + 'static,
          T: Send + 'static
{
    Scheduler::spawn_opts(f, opts)
}

/// Giveup the CPU
#[inline(always)]
pub fn sched() {
    Scheduler::sched()
}

/// Run the scheduler with threads, block until all its threads finish
#[inline(always)]
pub fn run(threads: usize) {
    Scheduler::run(threads)
}

/// Run the scheduler with threads, don't block
#[inline(always)]
pub fn start(threads: usize) {
    Scheduler::start(threads)
}

/// Block until every scheduler thread finishes
#[inline(always)]
pub fn join() {
    Scheduler::join()
}

/// Put the current coroutine to sleep for the specific amount of time
#[inline]
pub fn sleep_ms(ms: u64) {
    runtime::Processor::current()
        .sleep_ms(ms);
}

/// Coroutine configuration. Provides detailed control over the properties and behavior of new coroutines.
pub struct Builder {
    opts: Options
}

impl Builder {
    /// Generates the base configuration for spawning a coroutine, from which configuration methods can be chained.
    pub fn new() -> Builder {
        Builder {
            opts: Options::new()
        }
    }

    /// Sets the size of the stack for the new coroutine.
    #[inline]
    pub fn stack_size(mut self, stack_size: usize) -> Builder {
        self.opts.stack_size = stack_size;
        self
    }

    /// Names the coroutine-to-be. Currently the name is used for identification only in panic messages.
    #[inline]
    pub fn name(mut self, name: Option<String>) -> Builder {
        self.opts.name = name;
        self
    }

    /// Spawn a new coroutine
    #[inline]
    pub fn spawn<F, T>(self, f: F) -> JoinHandle<T>
        where F: FnOnce() -> T + Send + 'static,
              T: Send + 'static
    {
        Scheduler::spawn_opts(f, self.opts)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sleep_ms() {
        spawn(|| {
            sleep_ms(1000);
        });

        run(1);
    }

    #[test]
    fn test_start_join() {
        spawn(|| {});

        start(1);
        join();
    }
}
