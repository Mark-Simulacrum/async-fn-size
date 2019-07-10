// Test that we don't duplicate storage for futures moved around in .await, and
// for futures moved into other futures.
//
// The exact sizes can change by a few bytes (we'd like to know when they do).
// What we don't want to see is the wrong multiple of 1024 (the size of BigFut)
// being reflected in the size.
//
// See issue #59123 for a full explanation.

// edition:2018
#![feature(async_await)]
extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;



use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub const BIG_FUT_SIZE: usize = 1024;
pub struct BigFut([u8; BIG_FUT_SIZE]);

impl BigFut {
    fn new() -> Self {
        BigFut([0; BIG_FUT_SIZE])
    } }

impl Drop for BigFut {
    fn drop(&mut self) {}
}

impl Future for BigFut {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _ctx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}

#[allow(dead_code)]
pub struct Joiner {
    a: Option<BigFut>,
    b: Option<BigFut>,
    c: Option<BigFut>,
}

impl Future for Joiner {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _ctx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}

pub fn noop() {}

pub async fn single() {
    let x = BigFut::new();
    x.await;
}

pub async fn single_with_noop() {
    let x = BigFut::new();
    noop();
    x.await;
}

pub async fn joined() {
    let a = BigFut::new();
    let b = BigFut::new();
    let c = BigFut::new();

    let joiner = Joiner {
        a: Some(a),
        b: Some(b),
        c: Some(c),
    };
    joiner.await
}

pub async fn joined_with_noop() {
    let a = BigFut::new();
    let b = BigFut::new();
    let c = BigFut::new();

    let joiner = Joiner {
        a: Some(a),
        b: Some(b),
        c: Some(c),
    };
    noop();
    joiner.await
}

#[wasm_bindgen_test]
#[cfg_attr(test, test)]
pub fn entry_main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    assert_eq!(1028, std::mem::size_of_val(&single()));
    assert_eq!(1028, std::mem::size_of_val(&single_with_noop()));
    assert_eq!(3080, std::mem::size_of_val(&joined()));
    assert_eq!(3080, std::mem::size_of_val(&joined_with_noop()));
}
