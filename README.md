# ZKWASM Service Helper

This crate is the Rust version of [ZKWASM Service Helper](https://github.com/DelphinusLab/zkWasm-service-helper), it provides
utilities for interacting with the ZKWasm service backend, similar to the TypeScript library. It allows querying tasks and
images, submitting proving tasks, and working with the archive server, all through Rust APIs.

Key Features

- Query APIs: Perform GET requests to retrieve tasks, images, and archive data.

- Task APIs: Perform POST requests to add proving tasks or manage existing tasks.

- Testing Utilities: Includes structured tests for queries, task submissions, and archive interactions.

- Documentation: Full Rust documentation can be generated locally using cargo doc.

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
