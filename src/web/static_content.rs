use axum::{
    body::Body,
    http::{header, HeaderValue, Response, StatusCode},
};
use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use headers::{ETag, HeaderMapExt, IfNoneMatch};
use sha2::{Digest, Sha512_256};
use tracing::trace;

/// Struct to hold the static text and ETag
pub struct StaticContent {
    content: &'static str,
    content_type: &'static str,
    etag: ETag,
}

fn compute_etag(content: &str) -> ETag {
    let hash = Sha512_256::digest(content.as_bytes());
    let etag = format!("\"{}\"", BASE64_URL_SAFE_NO_PAD.encode(&hash[..]));
    etag.parse().expect("valid etag value")
}

impl StaticContent {
    pub fn new(content: &'static str, content_type: &'static str) -> Self {
        Self {
            content,
            content_type,
            etag: compute_etag(content),
        }
    }

    // Handle a GET request with ETag logic
    pub fn get_request(&self, if_none_match: Option<IfNoneMatch>) -> Response<Body> {
        if let Some(if_none_match) = if_none_match {
            trace!("IfNoneMatch found: {:?}", &if_none_match);
            if !if_none_match.precondition_passes(&self.etag) {
                let mut res = Response::new(Body::empty());
                *res.status_mut() = StatusCode::NOT_MODIFIED;
                res.headers_mut().typed_insert(self.etag.clone());
                return res;
            }
        }

        let mut res = Response::new(self.content.into());
        res.headers_mut().typed_insert(self.etag.clone());
        res.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static(self.content_type),
        );
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Bytes;

    const CONTENT: &str = "Hello, world!";
    const CONTENT_TYPE: &str = "text/plain";

    #[test]
    fn static_content_creation() {
        let static_content = StaticContent::new(CONTENT, CONTENT_TYPE);

        assert_eq!(static_content.content, CONTENT);
        assert_eq!(static_content.content_type, CONTENT_TYPE);
        assert_eq!(static_content.etag, compute_etag(CONTENT));
    }

    #[test]
    fn get_request_with_matching_etag() {
        let static_content = StaticContent::new(CONTENT, CONTENT_TYPE);
        let if_none_match = IfNoneMatch::from(static_content.etag.clone());

        let response = static_content.get_request(Some(if_none_match));

        assert_eq!(response.status(), StatusCode::NOT_MODIFIED);
        assert_eq!(
            response.headers().typed_get::<ETag>(),
            Some(static_content.etag)
        );
    }

    #[tokio::test]
    async fn get_request_with_non_matching_etag() {
        let static_content = StaticContent::new(CONTENT, CONTENT_TYPE);
        let non_matching_etag: ETag = "\"non-matching-etag\"".parse().unwrap();
        let if_none_match: IfNoneMatch = non_matching_etag.into();

        let response = static_content.get_request(Some(if_none_match));

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers().typed_get(), Some(static_content.etag));
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE),
            Some(&HeaderValue::from_static(CONTENT_TYPE))
        );
        assert_eq!(body_as_bytes(response).await, CONTENT);
    }

    #[tokio::test]
    async fn get_request_without_if_none_match() {
        let static_content = StaticContent::new(CONTENT, CONTENT_TYPE);

        let response = static_content.get_request(None);

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers().typed_get(), Some(static_content.etag));
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE),
            Some(&HeaderValue::from_static(CONTENT_TYPE))
        );
        assert_eq!(body_as_bytes(response).await, CONTENT);
    }

    async fn body_as_bytes(response: Response<Body>) -> Bytes {
        axum::body::to_bytes(response.into_parts().1, 1024)
            .await
            .unwrap()
    }
}
