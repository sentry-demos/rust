## Setup

Before begin — [Install Rust].


### Versions
---
this was tested on:

| dependency    | version
| ------------- |:-------------:|
| Rust | `1.45.0` — `1.51.0-nightly` |
| Cargo | `1.45.0` — `1.50.0` |
| Actix-Web | `0.7.19` |
| Sentry SDK | `0.19.0` |


### Using
---

```bash
# Export your DSN in terminal
1. export DSN=http://acc950dd09ce430a875cda90c3e3d3ce@cicd1-sentry.example.com:9000/1
2. cargo run
```

Follow the link below:

1. http://localhost:3001/handled
2. http://localhost:3001/unhandled
3. http://localhost:3001/checkout w/ payload from sentry-demos/react


 [Install Rust]: https://www.rust-lang.org/tools/install
