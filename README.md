# TeaPot

Experimental HTTP Status codes command line

## Dependencies
* clap = 4.0.32 for command line
* http = 0.2.8 for http status codes
* serde = 1.0 Serialize and Deserialize
* serde_yaml = 0.9 for read yaml configuration
* comfy-table = 6.1.3 for print console
* strum = 0.24 = handle enums
* strum_macros = 0.24  handle enums

## How to use

```sh
    cargo run -- -c 100 500 201
```

```sh
      Running `target/debug/teapot -c 100 500 201`
┌──────┬───────────────────────┬───────────────────────────────────────────┬───────────────────────────────────────────┬──────────────────────────────────────────┐
│ Code ┆ Name                  ┆ Description                               ┆ Supplementary                             ┆ Documentation Url                        │
╞══════╪═══════════════════════╪═══════════════════════════════════════════╪═══════════════════════════════════════════╪══════════════════════════════════════════╡
│ 100  ┆ Continue              ┆ The initial part of a request has been    ┆ When the request contains an Expect       ┆ https://www.rfc-editor.org/rfc/rfc7231#s │
│      ┆                       ┆ received and has not yet been rejected by ┆ header field that includes a 100-continue ┆ ection-6.2.1                             │
│      ┆                       ┆ the server.The server intends to send a   ┆ expectation, the 100 response indicates   ┆                                          │
│      ┆                       ┆ final response after the request has been ┆ that the server wishes to receive the     ┆                                          │
│      ┆                       ┆ fully received and acted upon             ┆ request payload body. The client ought to ┆                                          │
│      ┆                       ┆                                           ┆ continue sending the request and discard  ┆                                          │
│      ┆                       ┆                                           ┆ the 100 response. If the request did not  ┆                                          │
│      ┆                       ┆                                           ┆ contain an Expect header field containing ┆                                          │
│      ┆                       ┆                                           ┆ the 100-continue expectation, the client  ┆                                          │
│      ┆                       ┆                                           ┆ can simply discard this interim response. ┆                                          │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 500  ┆ Internal Server Error ┆ The server encountered an unexpected      ┆                                           ┆ https://tools.ietf.org/html/rfc7231#sect │
│      ┆                       ┆ condition that prevented it from          ┆                                           ┆ ion-6.6.1                                │
│      ┆                       ┆ fulfilling the request.                   ┆                                           ┆                                          │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 201  ┆ Created               ┆ The request has been fulfilled and has    ┆ The primary resource created by the       ┆ https://tools.ietf.org/html/rfc7231#sect │
│      ┆                       ┆ resulted in one or more new resources     ┆ request is identified by either a         ┆ ion-6.3.2                                │
│      ┆                       ┆ being created.                            ┆ Location header field in the response or, ┆                                          │
│      ┆                       ┆                                           ┆ if no Location field is received, by the  ┆                                          │
│      ┆                       ┆                                           ┆ effective request URI.                    ┆                                          │
│      ┆                       ┆                                           ┆ The 201 response payload typically        ┆                                          │
│      ┆                       ┆                                           ┆ describes and links to the resource(s)    ┆                                          │
│      ┆                       ┆                                           ┆ created. See Section 7.2 of RFC7231 for a ┆                                          │
│      ┆                       ┆                                           ┆ discussion of the meaning and purpose of  ┆                                          │
│      ┆                       ┆                                           ┆ validator header fields, such as ETag and ┆                                          │
│      ┆                       ┆                                           ┆ Last-Modified, in a 201 response.         ┆                                          │
└──────┴───────────────────────┴───────────────────────────────────────────┴───────────────────────────────────────────┴──────────────────────────────────────────┘
```