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
     cargo run -- -c 418 201
```

```sh
┌──────┬──────────────┬────────────────────────────────────────────┬────────────────────────────────────────────┬────────────────────────────────────────────┐
│ Code ┆ Name         ┆ Description                                ┆ Supplementary                              ┆ Documentation Url                          │
╞══════╪══════════════╪════════════════════════════════════════════╪════════════════════════════════════════════╪════════════════════════════════════════════╡
│ 418  ┆ I'm a teapot ┆ Any attempt to brew coffee with a teapot   ┆                                            ┆ https://tools.ietf.org/html/rfc2324#sectio │
│      ┆              ┆ should result in the error code '418 I'm a ┆                                            ┆ n-2.3.2                                    │
│      ┆              ┆ teapot'. The resulting entity body MAY be  ┆                                            ┆                                            │
│      ┆              ┆ short and stout.                           ┆                                            ┆                                            │
├╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ 201  ┆ Created      ┆ The request has been fulfilled and has     ┆ The primary resource created by the        ┆ https://tools.ietf.org/html/rfc7231#sectio │
│      ┆              ┆ resulted in one or more new resources      ┆ request is identified by either a Location ┆ n-6.3.2                                    │
│      ┆              ┆ being created.                             ┆ header field in the response or, if no     ┆                                            │
│      ┆              ┆                                            ┆ Location field is received, by the         ┆                                            │
│      ┆              ┆                                            ┆ effective request URI.                     ┆                                            │
│      ┆              ┆                                            ┆ The 201 response payload typically         ┆                                            │
│      ┆              ┆                                            ┆ describes and links to the resource(s)     ┆                                            │
│      ┆              ┆                                            ┆ created. See Section 7.2 of RFC7231 for a  ┆                                            │
│      ┆              ┆                                            ┆ discussion of the meaning and purpose of   ┆                                            │
│      ┆              ┆                                            ┆ validator header fields, such as ETag and  ┆                                            │
│      ┆              ┆                                            ┆ Last-Modified, in a 201 response.          ┆                                            │
└──────┴──────────────┴────────────────────────────────────────────┴────────────────────────────────────────────┴────────────────────────────────────────────┘
```
