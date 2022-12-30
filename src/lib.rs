use std::collections::HashMap;
use std::include_bytes;

use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;

pub struct TeaPotApp {
    pub http_codes: HttpCode,
}

impl TeaPotApp {
    pub fn new() -> TeaPotApp {
        let bytes = include_bytes!("../config.yml");
        let contents = String::from_utf8_lossy(bytes);
        TeaPotApp {
            http_codes: serde_yaml::from_str(&contents).unwrap(),
        }
    }
}

impl Default for TeaPotApp {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct HttpStatusCode {
    /// represents the code for the HTTP status
    pub code: u16,
    /// represents the name for the HTTP status
    pub name: String,
    /// represents a brief overview of the HTTP status
    pub description: String,
    /// provides additional information for the HTTP status
    pub supplementary_description: String,
    /// provides the URL for the HTTP status is found.
    pub documentation_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct HttpCode {
    /// represents a map for all HTTP status codes
    pub codes: HashMap<String, HttpStatusCode>,
}

/// Represents an HTTP status code
#[derive(AsRefStr, Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Serialize, Deserialize)]
pub enum StatusCode {
    /// 100 Continue (RFC 7231)
    Continue,

    /// 101 Switching Protocols (RFC 7231)
    SwitchingProtocols,

    /// 102 Processing (RFC 2518)
    Processing,

    /// 200 OK (RFC 7231)
    Ok,

    /// 201 Created (RFC 7231)
    Created,

    /// 202 Accepted (RFC 7231)
    Accepted,

    /// 203 Non-Authoritative Information (RFC 7231)
    NonAuthoritativeInformation,

    /// 204 No Content (RFC 7231)
    NoContent,

    /// 205 Reset Content (RFC 7231)
    ResetContent,

    /// 206 Partial Content (RFC 7233)
    PartialContent,

    /// 207 Multi-Status (RFC 4918)
    MultiStatus,

    /// 208 Already Reported (RFC 5842)
    AlreadyReported,

    /// 226 IM Used (RFC 3229)
    IMUsed,

    /// 300 Multiple Choices (RFC 7231)
    MultipleChoices,

    /// 301 Moved Permanently (RFC 7231)
    MovedPermanently,

    /// 302 Found (RFC 7231)
    Found,

    /// 303 See Other (RFC 7231)
    SeeOther,

    /// 304 Not Modified (RFC 7232)
    NotModified,

    /// 305 Use Proxy (RFC 7231)
    UseProxy,

    /// 307 Temporary Redirect (RFC 7231)
    TemporaryRedirect,

    /// 308 Permanent Redirect (RFC 7538)
    PermanentRedirect,

    /// 400 Bad Request (RFC 7231)
    BadRequest,

    /// 401 Unauthorized (RFC 7235)
    Unauthorized,

    /// 402 Payment Required (RFC 7231)
    PaymentRequired,

    /// 403 Forbidden (RFC 7231)
    Forbidden,

    /// 404 Not Found (RFC 7231)
    NotFound,

    /// 405 Method Not Allowed (RFC 7231)
    MethodNotAllowed,

    /// 406 Not Acceptable (RFC 7231)
    NotAcceptable,

    /// 407 Proxy Authentication Required (RFC 7235)
    ProxyAuthenticationRequired,

    /// 408 Request Timeout (RFC 7231)
    RequestTimeout,

    /// 409 Conflict (RFC 7231)
    Conflict,

    /// 410 Gone (RFC 7231)
    Gone,

    /// 411 Length Required (RFC 7231)
    LengthRequired,

    /// 412 Precondition Failed (RFC 7232)
    PreconditionFailed,

    /// 413 Payload Too Large (RFC 7231)
    PayloadTooLarge,

    /// 414 URI Too Long (RFC 7231)
    UriTooLong,

    /// 415 Unsupported Media Type (RFC 7231)
    UnsupportedMediaType,

    /// 416 Range Not Satisfiable (RFC 7233)
    RangeNotSatisfiable,

    /// 417 Expectation Failed (RFC 7231)
    ExpectationFailed,

    /// 418 I'm a teapot (RFC 2324)
    ImATeapot,

    /// 421 Misdirected Request (RFC 7540)
    MisdirectedRequest,

    /// 422 Unprocessable Entity (RFC 4918)
    UnprocessableEntity,

    /// 423 Locked (RFC 4918)
    Locked,

    /// 424 Failed Dependency (RFC 4918)
    FailedDependency,

    /// 426 Upgrade Required (RFC 7231)
    UpgradeRequired,

    /// 428 Precondition Required (RFC 6585)
    PreconditionRequired,

    /// 429 Too Many Requests (RFC 6585)
    TooManyRequests,

    /// 431 Request Header Fields Too Large (RFC 6585)
    RequestHeaderFieldsTooLarge,

    /// 451 Unavailable For Legal Reasons (RFC 7725)
    UnavailableForLegalReasons,

    /// 500 Internal Server Error (RFC 7231)
    InternalServerError,

    /// 501 Not Implemented (RFC 7231)
    NotImplemented,

    /// 502 Bad Gateway (RFC 7231)
    BadGateway,

    /// 503 Service Unavailable (RFC 7231)
    ServiceUnavailable,

    /// 504 Gateway Timeout (RFC 7231)
    GatewayTimeout,

    /// 505 HTTP Version Not Supported (RFC 7231)
    HttpVersionNotSupported,

    /// 506 Variant Also Negotiates (RFC 2295)
    VariantAlsoNegotiates,

    /// 507 Insufficient Storage (RFC 4918)
    InsufficientStorage,

    /// 508 Loop Detected (RFC 5842)
    LoopDetected,

    /// 510 Not Extended (RFC 2774)
    NotExtended,

    /// 511 Network Authentication Required (RFC 6585)
    NetworkAuthenticationRequired,

    /// Unknown status code
    Unknown(u16),
}

impl From<u16> for StatusCode {
    fn from(code: u16) -> Self {
        match code {
            100 => StatusCode::Continue,
            101 => StatusCode::SwitchingProtocols,
            102 => StatusCode::Processing,
            200 => StatusCode::Ok,
            201 => StatusCode::Created,
            202 => StatusCode::Accepted,
            203 => StatusCode::NonAuthoritativeInformation,
            204 => StatusCode::NoContent,
            205 => StatusCode::ResetContent,
            206 => StatusCode::PartialContent,
            207 => StatusCode::MultiStatus,
            208 => StatusCode::AlreadyReported,
            226 => StatusCode::IMUsed,
            300 => StatusCode::MultipleChoices,
            301 => StatusCode::MovedPermanently,
            302 => StatusCode::Found,
            303 => StatusCode::SeeOther,
            304 => StatusCode::NotModified,
            305 => StatusCode::UseProxy,
            307 => StatusCode::TemporaryRedirect,
            308 => StatusCode::PermanentRedirect,
            400 => StatusCode::BadRequest,
            401 => StatusCode::Unauthorized,
            402 => StatusCode::PaymentRequired,
            403 => StatusCode::Forbidden,
            404 => StatusCode::NotFound,
            405 => StatusCode::MethodNotAllowed,
            406 => StatusCode::NotAcceptable,
            407 => StatusCode::ProxyAuthenticationRequired,
            408 => StatusCode::RequestTimeout,
            409 => StatusCode::Conflict,
            410 => StatusCode::Gone,
            411 => StatusCode::LengthRequired,
            412 => StatusCode::PreconditionFailed,
            413 => StatusCode::PayloadTooLarge,
            414 => StatusCode::UriTooLong,
            415 => StatusCode::UnsupportedMediaType,
            416 => StatusCode::RangeNotSatisfiable,
            417 => StatusCode::ExpectationFailed,
            418 => StatusCode::ImATeapot,
            421 => StatusCode::MisdirectedRequest,
            422 => StatusCode::UnprocessableEntity,
            423 => StatusCode::Locked,
            424 => StatusCode::FailedDependency,
            426 => StatusCode::UpgradeRequired,
            428 => StatusCode::PreconditionRequired,
            429 => StatusCode::TooManyRequests,
            431 => StatusCode::RequestHeaderFieldsTooLarge,
            451 => StatusCode::UnavailableForLegalReasons,
            500 => StatusCode::InternalServerError,
            501 => StatusCode::NotImplemented,
            502 => StatusCode::BadGateway,
            503 => StatusCode::ServiceUnavailable,
            504 => StatusCode::GatewayTimeout,
            505 => StatusCode::HttpVersionNotSupported,
            506 => StatusCode::VariantAlsoNegotiates,
            507 => StatusCode::InsufficientStorage,
            508 => StatusCode::LoopDetected,
            510 => StatusCode::NotExtended,
            511 => StatusCode::NetworkAuthenticationRequired,
            _ => StatusCode::Unknown(code),
        }
    }
}
