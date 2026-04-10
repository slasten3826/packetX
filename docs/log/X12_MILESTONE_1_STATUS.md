# X12 Milestone 1 Status

## Статус

`Milestone 1` начался и уже частично реализован.

Это больше не только:

- docs
- ontology
- internal spine

Теперь в коде уже есть настоящий `X11` wire boundary.

## Что уже закрыто

### 1. Живой server spine

Работает headless pipeline:

- `chaos`
- `table`
- `crystall`
- `manifest`

Он уже проверен тестами на:

- full occlusion
- partial overlap
- unmap restore
- manifest pressure accounting

### 2. X11 setup handshake

Есть настоящий `X11` setup path:

- `xConnClientPrefix`
- validation
- `SetupSuccess` / `SetupFailed`

Текущие ограничения:

- только `X11 11.0`
- только little-endian client
- только empty authorization

### 3. Минимальный wire subset

Есть parser для 4 opcode'ов:

- `CreateWindow`
- `MapWindow`
- `UnmapWindow`
- `ConfigureWindow`

### 4. Wire errors

Есть минимальный error subset:

- `BadRequest`
- `BadValue`
- `BadWindow`
- `BadMatch`
- `BadLength`

### 5. Wire -> X12 spine path

Есть живой путь:

- wire bytes
- parse
- translate into internal `X11Request`
- run through current `X12` spine

То есть `Milestone 1`
уже не только "будем делать wire ingress",
а реально начатый кодовый слой.

## Что еще не закрыто

Пока еще нет:

- полноценной session semantics
- нескольких request'ов в одном client connection
- per-client resource registry
- real event stream
- `MapNotify`
- `ConfigureNotify`
- `Expose`
- desktop-grade `Xlib/XCB` client compatibility

То есть:

`Milestone 1` открыт,
но не завершен.

## Что считать ближайшим практическим успехом

Следующий честный успех:

> минимальный реальный `X11` client
> должен суметь подключиться к `packetX`,
> пройти setup,
> и отправить хотя бы один поддерживаемый request

Это будет первым real boundary proof,
не synthetic и не self-generated.

## Что уже подтверждено после этого документа

Этот ближайший practical success уже случился.

Есть отдельный real smoke-test:

- отдельный `x12-server`
- отдельный `x11_probe`
- реальный `Unix socket`
- реальный `X11` setup
- реальный `CreateWindow`

И этот request уже действительно доходит до `X12` spine.

То есть:

- `Milestone 1` не просто "начат"
- он уже прошел первый внешний boundary proof

## Короткая формула

Сейчас состояние такое:

- `Milestone 0` — закрыт
- `Milestone 1` — живой и начат в коде
- следующий pressure-test — real `X11` client
