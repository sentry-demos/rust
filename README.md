## Setup

1. Install Rust (https://www.rust-lang.org/tools/install)

#### Versions
this was tested on:

| dependency    | version
| ------------- |:-------------:|
| Rust | 1.45.0 |
| Cargo | 1.45.0 |
| Actix-Web | 0.7.19 |
| Sentry SDK | 0.19.0 |


## Run

1. go into the directory that has the Cargo.toml file in it
2. run "cargo run"
3. go to URL http://localhost:3001/checkout
4. go to URL http://localhost:3001/handled
5. go to URL http://localhost:3001/unhandled

## Testing

Checkout: https://sentry.io/organizations/testorg-az/issues/1811849457/?project=5250920&query=is%3Aunresolved

Handled: https://sentry.io/organizations/testorg-az/issues/1693289619/?project=5250920&query=is%3Aunresolved

Unhandled: https://sentry.io/organizations/testorg-az/issues/1811844726/?project=5250920&query=is%3Aunresolved