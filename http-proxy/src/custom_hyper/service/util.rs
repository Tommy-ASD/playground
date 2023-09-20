use std::error::Error as StdError;
use std::fmt;
use std::marker::PhantomData;

use crate::custom_hyper::body::HttpBody;
use crate::custom_hyper::common::{task, Future, Poll};
use crate::custom_hyper::{Request, Response};

/// Create a `Service` from a function.
///
/// # Example
///
/// ```
/// use hyper::{Body, Request, Response, Version};
/// use hyper::service::service_fn;
///
/// let service = service_fn(|req: Request<Body>| async move {
///     if req.version() == Version::HTTP_11 {
///         Ok(Response::new(Body::from("Hello World")))
///     } else {
///         // Note: it's usually better to return a Response
///         // with an appropriate StatusCode instead of an Err.
///         Err("not HTTP/1.1, abort connection")
///     }
/// });
/// ```
pub(crate) fn service_fn<F, R, S>(f: F) -> ServiceFn<F, R>
where
    F: FnMut(Request<R>) -> S,
    S: Future,
{
    ServiceFn {
        f,
        _req: PhantomData,
    }
}

/// Service returned by [`service_fn`]
pub(crate) struct ServiceFn<F, R> {
    f: F,
    _req: PhantomData<fn(R)>,
}

impl<F, ReqBody, Ret, ResBody, E> tower_service::Service<crate::custom_hyper::Request<ReqBody>>
    for ServiceFn<F, ReqBody>
where
    F: FnMut(Request<ReqBody>) -> Ret,
    ReqBody: HttpBody,
    Ret: Future<Output = Result<Response<ResBody>, E>>,
    E: Into<Box<dyn StdError + Send + Sync>>,
    ResBody: HttpBody,
{
    type Response = crate::custom_hyper::Response<ResBody>;
    type Error = E;
    type Future = Ret;

    fn poll_ready(&mut self, _cx: &mut task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        (self.f)(req)
    }
}

impl<F, R> fmt::Debug for ServiceFn<F, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("impl Service").finish()
    }
}

impl<F, R> Clone for ServiceFn<F, R>
where
    F: Clone,
{
    fn clone(&self) -> Self {
        ServiceFn {
            f: self.f.clone(),
            _req: PhantomData,
        }
    }
}

impl<F, R> Copy for ServiceFn<F, R> where F: Copy {}
