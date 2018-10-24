use std::marker::PhantomData;

use actix_net::codec::Framed;
use actix_net::service::{NewService, Service};
use futures::future::{ok, FutureResult};
use futures::{Async, IntoFuture, Poll};

use h1::Codec;
use request::Request;

use super::{verify_handshake, HandshakeError};

pub struct VerifyWebSockets<T> {
    _t: PhantomData<T>,
}

impl<T> Default for VerifyWebSockets<T> {
    fn default() -> Self {
        VerifyWebSockets { _t: PhantomData }
    }
}

impl<T> NewService for VerifyWebSockets<T> {
    type Request = (Request, Framed<T, Codec>);
    type Response = (Request, Framed<T, Codec>);
    type Error = (HandshakeError, Framed<T, Codec>);
    type InitError = ();
    type Service = VerifyWebSockets<T>;
    type Future = FutureResult<Self::Service, Self::InitError>;

    fn new_service(&self) -> Self::Future {
        ok(VerifyWebSockets { _t: PhantomData })
    }
}

impl<T> Service for VerifyWebSockets<T> {
    type Request = (Request, Framed<T, Codec>);
    type Response = (Request, Framed<T, Codec>);
    type Error = (HandshakeError, Framed<T, Codec>);
    type Future = FutureResult<Self::Response, Self::Error>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        Ok(Async::Ready(()))
    }

    fn call(&mut self, (req, framed): Self::Request) -> Self::Future {
        match verify_handshake(&req) {
            Err(e) => Err((e, framed)).into_future(),
            Ok(_) => Ok((req, framed)).into_future(),
        }
    }
}