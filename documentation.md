# Documentation

postgres single entry is 22.45 ops/sec git is 8.11 ops / sec

so almost 3 times fast with postgres

test GOS server

```bash
grpcurl -plaintext \
  -import-path crates/api/proto \
  -proto crates/api/proto/gos/v1/gos.proto \
  -d '{}' \
  localhost:50051 gos.v1.GOS/Health
```
