# X12 Real X11 Smoke Test 0001

Дата: 2026-04-10

## Что проверялось

Первый реальный boundary smoke-test `X12`:

- поднять `x12-server` как local `Unix socket` listener
- подключить отдельный реальный probe-клиент
- пройти `X11` setup handshake
- отправить настоящий `CreateWindow` request
- убедиться, что request доходит до `X12` spine

## Каким стендом это делалось

### Сервер

- `cargo run --bin x12-server -- --x11-client-once /home/slasten/dev/x12/.x12-smoke.sock`

### Клиент

- `cargo run --bin x11_probe -- /home/slasten/dev/x12/.x12-smoke.sock create-window`

`x11_probe` — это не synthetic byte fixture внутри теста,
а отдельный маленький бинарник,
который реально подключается к socket и говорит по `X11` wire subset `v0`.

## Что ответил клиент

Клиент получил:

- `setup: success version=11.0 vendor=packetX`

И затем:

- `request sent; server closed connection without wire error`

Это значит:

- setup handshake прошел
- server surface распознан как валидный `X11` setup
- `CreateWindow` request не упал в `xError`

## Что ответил сервер

Сервер зафиксировал:

- `X11Client`
- `Scene`
- `forms_total: 1`
- `id: 77`
- `parent: 1`
- `pos: [10, 20]`
- `size: [640, 480]`
- `mapped: false`

И в `log_ledger`:

- `chaos: packet born`
- `table: create form claim id=77`
- `crystall: no visible forms, packet dies before manifest`
- `manifest: skipped because packet is dead`

## Что это значит

Это не "полный `X11` клиент заработал".

Но это уже **настоящий живой boundary proof**:

- отдельный клиент
- отдельный сервер
- реальный `Unix socket`
- реальный `X11` handshake
- реальный `CreateWindow`
- реальный проход в `X12` spine

То есть `X12` уже не только:

- docs
- ontology
- synthetic tests

Он уже умеет:

- принимать внешний `X11` client ingress
- и рожать из этого внутреннюю server truth

## Что smoke-test не проверяет

Пока еще не проверено:

- multi-request session
- `MapWindow` после `CreateWindow` в том же соединении
- `MapNotify`
- `ConfigureNotify`
- `Expose`
- desktop-grade `Xlib/XCB` behavior

То есть это smoke,
а не полный compatibility proof.

## Почему это важный момент

До этого `X12` был:

- headless
- protocol-aware
- test-driven

После этого smoke-test
`X12` уже можно честно назвать:

> живым `X11`-facing server prototype

Пока очень узким.
Но уже настоящим.
