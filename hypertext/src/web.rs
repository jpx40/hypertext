#[cfg(feature = "axum")]
mod axum_support {
    extern crate alloc;

    use axum_core::{
        body::Body,
        response::{IntoResponse, Response},
    };
    use http::{header, HeaderValue};

    use crate::Rendered;

    impl<T: Into<Body>> IntoResponse for Rendered<T> {
        #[inline]
        fn into_response(self) -> Response {
            (
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("text/html; charset=utf-8"),
                )],
                self.0.into(),
            )
                .into_response()
        }
    }
}

#[cfg(feature = "actix-web")]
mod atix_support {
    extern crate alloc;
    use alloc::string::String;

    use actix_web_dep::{body::BoxBody, http::header, HttpRequest, HttpResponse, Responder};

    use crate::Rendered;

    impl<T: Into<BoxBody>> Responder for Rendered<T> {
        type Body = String;

        #[inline]
        fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
            HttpResponse::Ok()
                .content_type(header::ContentType::html())
                .message_body(self.0)
                .unwrap()
        }
    }
}

// #[cfg(feature = "warp")]
// mod warp_support {
//     extern crate alloc;
//     use alloc::string::String;
//
//     use warp::{
//         f,
//         reply::{self, Reply, Response},
//     };
//
//     use crate::Rendered;
//     impl<T: Into<Body>> Reply for Rendered<T> {
//         #[inline]
//         fn into_response(self) -> Response {
//             reply::html(self.into_string()).into_response()
//         }
//     }
// }
