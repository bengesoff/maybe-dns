# `maybe-dns`

A toy project mainly to exercise my new keyboard but also to refresh a bit on Rust.

Initial plan is to do something with DNS, hence the name, and so far implements a basic server that sort of parses the DNS query and echoes it back.
Thanks to DNS's query and response format being fairly similar (the same but with different flags set), using `dig` with it doesn't get too confused.

To test, run `cargo run` in one terminal, then use `dig` in another terminal similarly to below:

```
dig @127.0.0.1 -p 1053 random.domain.to.lookup
```

Not sure what I'll implement next.
It would be a good idea to "de-mank" the code so maybe I'll do that.
Or maybe I won't as it's only for me anyway.
