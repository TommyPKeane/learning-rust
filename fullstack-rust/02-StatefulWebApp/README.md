# Stateful Web App (Example)

This builds on the "First Web App", but is self-contained, here.

<!-- MarkdownTOC -->

- Package Setup
- Run
   - Make a `GET` Request

<!-- /MarkdownTOC -->

## Package Setup

These steps were already done, so you don't need to do them, but if you want to learn about how this was done, these are the steps that were followed to setup this example:

1. `cargo new messages-actix`
1. `cd messages-actix/`
1. `cargo add actix-web@1.0`
1. `cargo add env_logger@0.6`
1. `cargo add --features derive serde@1.0`
1. `cargo add serde_json@1.0`

⚠️ The above has specific versions associated with it, because the code from this book is from 2018/2020, and these packages have since been updated (some of them quite significantly), and if you don't use the exact versions then this code won't work at all, and cannot be fixed without significant rewrites.

## Run

```bash
cd messages-actix;
cargo run;
```

Expected output when run:

```bash
❯ cargo run
   Compiling messages-actix v0.1.1 (/Users/__USERNAME__/__PATHTO__/messages-actix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.62s
     Running `target/debug/messages-actix`
Starting http server: 127.0.0.1:8080
```

Note that `127.0.0.1:8080` is the IP address of the Webserver with the specified Port for handling any HTTP requests (TCP/IP):

- Server Address: `127.0.0.1`
- Server Port: `8080`

If you have a conflict because of other running local services, for either the address or the port, you'll need to edit the code to update the Address and/or Port.

### Make a `GET` Request

In a separate terminal from where you ran the app (above), make this call:

```bash
curl -X "GET" 127.0.0.1:8080
```

And you'll get this response:

```bash
{"server_id":3,"request_count":1,"message":[]}
```

And then if you go back to the other terminal where you ran the application, you should see log messages showing that your request was processed:


```bash
[2022-12-29T19:59:07Z INFO  actix_web::middleware::logger] 127.0.0.1:51904 "GET / HTTP/1.1" 200 19 "-" "curl/7.79.1" 0.000205
```
