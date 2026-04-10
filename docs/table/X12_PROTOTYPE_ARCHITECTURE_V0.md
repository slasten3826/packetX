# X12 Prototype Architecture v0

## Зачем этот документ

После большой онтологической сборки
нужно зафиксировать не "весь X12 сразу",
а первый реалистичный прототип.

То есть:

- что именно мы прототипируем
- в какой форме
- что берем сразу
- что сознательно откладываем

## Главный тезис

Первый прототип `X12`
не должен быть "полным новым display stack".

Он должен быть:

> server spine prototype

То есть минимальным живым каркасом,
который доказывает:

- `chaos` рождает packet
- `table` формирует server truth
- `crystall` собирает и режет форму
- `manifest` выдает итог хотя бы в fake/text виде

## Физическая форма прототипа

### Один бинарник

- `x12-server`

### Один runtime loop

- event loop / tick loop

### Один transport на старте

- local `Unix domain socket`

### Один compatibility path

- `X11 bridge v0`

### Один manifest на старте

- fake/text manifest

То есть на первом этапе не реальный экран,
а machine-readable manifestation.

## Модульный каркас

### `src/main.rs`

- запуск сервера
- init runtime
- init socket
- запуск главного loop

### `src/server/`

- `ServerState`
- registry
- global counters
- debug snapshot state

### `src/chaos/`

- ingress
- raw event normalization
- packet birth
- input/network/hotplug boundaries

### `src/table/`

- relation truth
- legality
- field/form claims
- addressability
- early packet filtering

### `src/crystall/`

- form assembly
- stacking
- overlap
- visibility
- occlusion kill

### `src/manifest/`

- fake/text manifest
- final output summary
- packet death/finalization

### `src/compat/x11/`

- Unix transport
- X11 protocol parsing
- request dispatch
- `X11 -> X12` translation

## Ключевые сущности прототипа

### `PacketAtom`

Минимальный packet-атом:

- `id`
- `origin`
- `birth_tick`
- `current_layer`
- `status`

Пока без переусложнения.

### `FormAssembly`

Сборка packet'ов в форму:

- form id
- owner/client relation
- bounds
- stacking level
- visibility state

### `ServerState`

Глобальное состояние сервера:

- forms
- fields
- carriers
- client relations
- packet counters
- debug traces

### `LogLedger`

Append-only trace:

- packet born
- packet accepted/rejected
- packet crystallized
- packet occluded/killed
- packet manifested

## Что должен уметь прототип v0

На первом этапе достаточно very small path.

### Обязательный минимум

1. принять локальный `X11` request `CreateWindow`
2. перевести его в server-side claim
3. создать `FormAssembly`
4. принять `MapWindow`
5. прогнать это через `table -> crystall -> manifest`
6. выдать text snapshot состояния

### Что должно быть видно в snapshot

- какие формы существуют
- какая форма сверху
- какая перекрыта
- какие packet'ы убиты в `crystall`
- что дошло до `manifest`

## Что не берем в v0

Сознательно откладываем:

- реальный DRM/KMS path
- реальный framebuffer output
- `GLX`
- `XRender`
- `Composite`
- `RandR`
- TCP transport
- touch input
- multi-monitor
- Lua policy layer
- real performance packet flood

## Почему fake/text manifest достаточно

На старте нам нужно проверять не "красивую картинку",
а truth pipeline:

- packet birth
- table legality
- form assembly
- occlusion filtering
- final manifestation decision

Это гораздо дешевле и полезнее,
чем сразу биться в реальный scanout.

## Первый milestone

### `Milestone 0: Headless X12`

Есть:

- `x12-server`
- local `X11` bridge
- fake/text manifest
- реальный pipeline `chaos -> table -> crystall -> manifest`

Можно:

- создать окно
- отобразить окно
- изменить окно
- убрать окно

И наблюдать это как machine-readable truth.

## Почему этот путь правильный

Он:

- не требует "сразу писать весь X12"
- не теряет server-shape
- сразу опирается на обязательный `X11` bridge
- дает реальный тест системы
- позволяет дальше наращивать реальность постепенно

## Короткая формула

Первый прототип `X12` должен быть:

- один серверный процесс
- Rust core
- local `X11` bridge
- fake/text manifest
- живой packet/form pipeline

Если это заработает,
дальше уже можно будет постепенно наращивать
настоящий output, input и hardware reality.
