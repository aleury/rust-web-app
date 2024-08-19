# rust-axum-intro

This is a project covered in [this course by Jeremy Chone](https://www.youtube.com/watch?v=XZtlD_m59sM).

## Setup

Start the application in one terminal:

```bash
$ cargo watch -q -c -w src/ -x run
```

and run the integration tests in another:

```bash
$ cargo watch -q -c -w examples/ -x "run --example quick_dev"
```
