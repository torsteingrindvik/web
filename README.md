# web-torstein

Personal website stuff.

## TODO web:

* Create an "introspection" page using Tokei
* Move to use handlebars?
    - Can then wrap all other content in stuff
        - Header
        - CONTENT
        - Footer

## TODO dev:

* Create minimally reproducible bug reports:
    - issue with `uri!()` import
    - issue with `crate_relative!()` import
* Config:
    - Use production
    - Set Limits
        - Upload
* Misc code:
    - Fix deprecation:
        > => Error: Response was a non-`Responder` `Err`: No such file or directory (os error 2).
        >
        > => Warning: This `Responder` implementation has been deprecated.
    - Maybe define an own Error enum, impl Responder for it.
        * Can still use anyhow!