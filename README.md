# actix-web-sse-broadcast-from-thread

This project is based on the [server-sent-events example of actix-web](https://github.com/actix/examples/tree/master/server-sent-events).

Instead of posting a message to an endpoint to be broadcasted, this project supports a thread from `actix_web::rt::spawn` which broadcasts messages every 200ms, messages which end up sent as server-sent-events.

Use cases could be:

- show the stock-exchange price in real-time
- multiplayer game: sending updates in real time
- ...

### Features

It is interruptible via Ctrl+C (it is necessary to handle `Arc<AtomicBool>` in threads).

## Run

```sh
cargo run

# go to http://localhost:8080/
```

## Resources:

- server sent events
  - https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events
  - https://www.reddit.com/r/actix/comments/wejj5e/sse_actix_web/
  - https://github.com/chaudharypraveen98/actix-sse-example
  - https://github.com/arve0/actix-sse
- working
  - [Share state between actix-web server and async closure](https://stackoverflow.com/questions/74167734/share-state-between-actix-web-server-and-async-closure)
  - [spawn'ed HttpServer is not interruptible](https://github.com/actix/actix-web/issues/2739)
