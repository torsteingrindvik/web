# web-torstein

Personal website stuff.

## TODO:

### Blocked on external dev
* Use stable Rust when we don't depend on nightly features
* Cache statics
    - See: https://github.com/SergioBenitez/Rocket/issues/95
* Use async
    - See: https://github.com/SergioBenitez/Rocket/issues/1065
* Use server side events
    - See: https://github.com/SergioBenitez/Rocket/issues/33
    - What for? Who knows, we'll find something
* Use websockets
    - See: https://github.com/SergioBenitez/Rocket/issues/90
    - What for? Who knows, we'll find something

### Blocked on me
* Create a "recently made Markdown pages" thing
* Create a page for this `README.md` as well
    - Normally it should be inside /md/, can we simply symlink this?
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