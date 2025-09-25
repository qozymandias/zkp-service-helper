# ZKWASM Service Helper

## How to Test

Update `test.json` with your details.

Run query (GET requests) tests:

```
cargo test tests::queries
```

Run task (POST requests) tests:

```
cargo test tests::tasks
```

Run archive query tests:

```
cargo test tests::archive
```

## Documentation

Build documentation

```
cargo doc --document-private-items --no-deps
```

Open index page in browser

```
open target/doc/zkp_service_helper/index.html
```
