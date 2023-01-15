# 🦀 Zero To Production 🚀

> [Zero To Production In Rust](https://zero2prod.com) is an opinionated introduction to back-end development using Rust.
>
> BIG thanks to @[Luca Palmieri](https://github.com/LukeMathWalker) for writing this 📕 and all of his contributions to the community!

![book cover](./assets/book_cover.png)

This repository contains my personal notes with excerpts and highlights of the book as I worked through it.

## Table of Contents

| Chapter               | Title                        | Description                                     |
| --------------------- | ---------------------------- | ----------------------------------------------- |
| [01](./docs/ch_01.md) | Getting Started              | Setup and tooling                               |
| [02](./docs/ch_02.md) | Building An Email Newsletter | Project planning                                |
| [03](./docs/ch_03.md) | Sign Up a New Subscriber     | Pick the tech stack & wire it all up with tests |
| [04](./docs/ch_04.md) | Telemetry                    | Implement logs then tracing                     |
| [05](./docs/ch_05.md) | Going Live                   | Containerization, CI/CD                         |
| [06](./docs/ch_06.md) | Reject Invalid Subscriber #1 | Form validation and code organization           |
| [07](./docs/ch_07.md) | Reject Invalid Subscriber #2 | Wire up confirmation email flow                 |
| [08](./docs/ch_08.md) | Error Handling               | Advanced and proper error handling              |
| [09](./docs/ch_09.md) | Naive Newsletter Delivery    | Initial feature to send newsletter              |
| [10](./docs/ch_10.md) | Securing Our API             | Authentication with protected routes            |
| [11](./docs/ch_11.md) | Fault-Tolerant Workflows     | Idempotency and concurrency                     |
| [QA](./docs/qa.md)    | Quality Assurance            | Field notes for QA-ing                          |

## Run it locally

### Configure base.yaml

Input your postmark API 📧 and 🔑 in `./configuration/local.yaml`.

**You get get your postmark API token at:**

- `https://account.postmarkapp.com/servers/{YOUR_POSTMARK_SERVER_ID}/credentials`

**Your postmark email is the email you used to sign up with**

- `https://account.postmarkapp.com/signature_domains`

```yaml
# TODO: Fill me out
# email_client:
#   base_url: "https://api.postmarkapp.com"
#   sender_email: "TODO 📧"
#   authorization_token: "TODO 🔑"
```

### Start The Sever

Make sure you have docker running.

```sh
# seed and run postgres database in container
./scripts/init_db.sh

# run redis server
./scripts/init_redis.sh

cargo run | bunyan
```

### Check The App

[`http://localhost:8000/`](http://localhost:8000/)

Please read through the documentation regarding [**QA**](./docs/qa.md)
