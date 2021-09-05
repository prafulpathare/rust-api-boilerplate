# Boilerplate Rust API

### Benchmark Performance
```
$ wrk -t12 -c400 -d30s http://127.0.0.1:8000
Running 30s test @ http://127.0.0.1:8000
12 threads and 400 connections
Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    50.42ms   68.10ms   1.75s    98.78%
    Req/Sec    250.64    163.21    757.00   61.45%
89664 requests in 30.06s, 13.77MB read
Socket errors : connect 0, read 89723, write 0, timeout 30
Requests/sec  :   2982.71
Transfer/sec  :   468.96KB
```

### 

Run directly.
```sh
$ cargo run
```

Compile & Launch

```sh
$ cargo build
$ sh ./target/debug/test-api
```
