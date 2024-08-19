# rust-web-app

This is a project covered in [this course by Jeremy Chone](https://www.youtube.com/watch?v=3cA_mk4vdWY).

## Setup

Start the application in one terminal:

```bash
$ cargo watch -q -c -w src/ -w .cargo/ -x run
```

and run the integration tests in another:

```bash
$ cargo watch -q -c -w examples/ -x "run --example quick_dev"
```
