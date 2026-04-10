# X12 Multi-Client Smoke Test 0001

## Что проверяли

Проверялся уже не one-shot и не single-session path,
а первый живой multi-client boundary:

- один `x12-server`
- режим `--x11-client-multi`
- два отдельных запуска `x11_probe`
- один и тот же `session-demo`

То есть:

- client 1 проходит setup и session-demo
- client 2 проходит setup и session-demo
- сервер должен не смешать их XID-space
- сервер должен сохранить ownership внутри `ServerState`

## Как запускали

Сервер:

```bash
cargo run --bin x12-server -- --x11-client-multi /home/slasten/dev/x12/.x12-multi.sock 2
```

Клиенты:

```bash
cargo run --bin x11_probe -- /home/slasten/dev/x12/.x12-multi.sock session-demo
cargo run --bin x11_probe -- /home/slasten/dev/x12/.x12-multi.sock session-demo
```

## Что получилось

### Client 1

Получил:

- `vendor=packetX`
- `xid_base=0x00200000`
- `xid_mask=0x001fffff`

### Client 2

Получил:

- `vendor=packetX`
- `xid_base=0x00400000`
- `xid_mask=0x001fffff`

То есть второй клиент уже не живет в том же xid-space,
что и первый.

## Server outcome

Обе session завершились как `Completed`:

- `processed_requests: 3`
- `protocol_errors: 0`
- `last_sequence: 3`

Во внутреннем snapshot:

- форма клиента 1 имеет `owner_session_id: 1`
- форма клиента 2 имеет `owner_session_id: 2`

Их ids:

- client 1 -> `2097229`
- client 2 -> `4194381`

То есть server truth уже удерживает:

- разный `xid_base`
- разный `session_id`
- разный owner у form

## Что это значит

`X12` уже умеет:

- принять больше одного клиента
- выдать разным клиентам разные xid-space
- не смешать их ids
- сохранить ownership формы внутри server state

Это уже не просто wire ingress и не только session discipline.
Это первый живой multi-client server proof.

## Что еще не сделано

Пока еще нет:

- event stream между сервером и клиентом
- `MapNotify`
- `ConfigureNotify`
- `Expose`
- полного multi-client runtime loop как постоянного демона

Но server identity уже стала заметно сильнее.
