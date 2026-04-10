# X12 X11 Wire Subset v0

## Зачем этот документ

После `Milestone 0`
пустое место сместилось.

Проблема теперь уже не в том,
как устроен внутренний spine `X12`,
а в том,
какой именно минимальный `X11` wire-вход
мы реально берем первым.

Нельзя просто сказать:

- "`Unix socket`"
- "binary `X11` protocol"

и пойти писать parser.

Нужен очень узкий поднабор.

## Главный тезис

`X12 X11 bridge v0`
не должен сразу быть "почти всем X11".

Он должен быть:

> минимальным protocol-correct local bridge

То есть:

- local-only
- `Unix domain socket`
- настоящий binary `X11` framing
- настоящий setup handshake
- только 4 запроса
- только минимальные ошибки
- без нормальных событий и без расширений

## Что именно берем

### Transport

- только `Unix domain socket`
- только local client
- TCP не берем
- display-discovery story пока не фиксируем

### Handshake

Берем настоящий `X11` connection setup:

- `xConnClientPrefix`
- `xConnSetupPrefix`
- `xConnSetup`

### Версия протокола

- `X_PROTOCOL = 11`
- `X_PROTOCOL_REVISION = 0`

### Byte order

Для `v0` поддерживаем только:

- little-endian client
- `byteOrder = 'l'`

`'B'` пока можно честно отклонять.

Это не навсегда.
Это просто режет первую сложность.

### Authorization

Для `v0`:

- только пустая авторизация
- `nbytesAuthProto = 0`
- `nbytesAuthString = 0`

Если клиент пытается прислать auth payload,
bridge может вернуть setup failure.

Это local-only phase,
поэтому такой срез нормален.

## Setup success v0

Bridge должен отвечать не абстрактным "ок",
а настоящим `SetupSuccess`.

Но setup при этом может быть очень узким и synthetic.

### В `v0` bridge публикует

- одну synthetic root
- один synthetic visual path
- один basic pixmap format

То есть:

- не реальный DRM root
- не богатый desktop-world
- а минимальный protocol-surface,
  достаточный для простых клиентов и наших тестов

## Синтетический root v0

На `v0` root нужно мыслить так:

- это bridge-owned synthetic root window
- это boundary-object между `X11` client world
  и server truth `X12`
- это не "настоящий корень мира X12"

То есть root в wire-совместимости
и root как внутренняя онтология `X12`
не обязаны совпадать один к одному.

Это важно.

## Какие request'ы реально поддерживаются

Только эти 4:

- `CreateWindow`
- `MapWindow`
- `UnmapWindow`
- `ConfigureWindow`

Их opcode'ы фиксированы по `X11`:

- `CreateWindow = 1`
- `MapWindow = 8`
- `UnmapWindow = 10`
- `ConfigureWindow = 12`

## Framing

### Общий request header

Все request'ы читаются через базовый `xReq`:

- `reqType: CARD8`
- `data: CARD8`
- `length: CARD16`

`length` всегда считается в единицах по 4 байта,
включая header самого request.

### V0 rule

Если `length` не соответствует ожидаемой форме request,
bridge отвечает:

- `BadLength`

## Request-specific subset

### 1. `CreateWindow`

Берем настоящий `xCreateWindowReq`,
но с жесткими ограничениями `v0`.

#### Обязательные поля wire-формы

- `wid`
- `parent`
- `x`
- `y`
- `width`
- `height`
- `borderWidth`
- `class`
- `visual`
- `mask`

#### Ограничения `v0`

- `parent` должен быть synthetic root
- `class` только `InputOutput`
- `visual` только `CopyFromParent`
- `borderWidth` только `0`
- `mask` только `0`

То есть:

- никаких attrs payload после request
- никакой сложной visual/config story

#### Почему так

Нам сейчас нужно не "поддержать X11 красиво",
а честно завести самый первый wire-path
до `table`.

### 2. `MapWindow`

Берем обычный `xResourceReq`.

Ожидаем:

- `length = 2`
- `id = window`

#### V0 behavior

- если window известен, поднимаем `MapWindow`
- если нет, `BadWindow`

### 3. `UnmapWindow`

Тоже обычный `xResourceReq`.

Ожидаем:

- `length = 2`
- `id = window`

#### V0 behavior

- если window известен, уходит в `table -> crystall`
- если нет, `BadWindow`

### 4. `ConfigureWindow`

Берем `xConfigureWindowReq`
и trailing values.

#### В `v0` разрешены только mask-биты

- `CWX`
- `CWY`
- `CWWidth`
- `CWHeight`

То есть:

- `CWX = 1 << 0`
- `CWY = 1 << 1`
- `CWWidth = 1 << 2`
- `CWHeight = 1 << 3`

#### Что не берем пока

- border width change
- sibling
- stack mode

#### V0 rule

Если приходят mask-биты вне этого набора,
bridge возвращает:

- `BadMatch`

Если trailing values не совпадают с mask,
bridge возвращает:

- `BadLength`

## Каких reply/event paths пока нет

Для этих 4 request'ов в `v0`:

- обычных replies нет
- normal async events пока нет
- `Expose`, `MapNotify`, `ConfigureNotify` пока не шлем

То есть первая wire-итерация
ориентирована не на полный `Xlib desktop`,
а на:

- собственный test client
- controlled local harness
- protocol-correct ingress

Это важная честность.

`xclock` и прочие обычные клиенты —
это уже следующий шаг,
после минимального request path.

## Минимальный error subset

На `v0` достаточно:

- `BadRequest`
- `BadValue`
- `BadWindow`
- `BadMatch`
- `BadLength`

### Когда что использовать

- `BadRequest`
  неизвестный opcode
- `BadLength`
  неправильный request length / broken trailing payload
- `BadWindow`
  неизвестный window id
- `BadValue`
  невалидные width/height/coords или byte order/version mismatch
- `BadMatch`
  синтаксически понятный request,
  но semantic mismatch для `v0`
  вроде unsupported `class`, `visual`, `configure mask`

## Что bridge делает с этим внутри

После parsing:

- wire request превращается в narrow internal `X11Request`
- дальше уже работает текущий spine:
  - `chaos`
  - `table`
  - `crystall`
  - `manifest`

То есть wire-layer не должен
тащить старую `X11` модель глубже, чем надо.

## V0 sequencing

Порядок реализации после этой спеки должен быть такой:

1. `Unix socket` listener
2. setup handshake only
3. parse `xReq`
4. dispatch 4 opcode'ов
5. translate into existing `X11Request`
6. прогнать через current spine
7. вернуть minimal errors when needed

## Что сознательно остается за рамкой

Пока не фиксируем:

- display name story
- auth beyond empty auth
- big-endian clients
- replies/events beyond setup and errors
- atoms/properties
- GC / pixmap / drawing
- extensions
- multi-client correctness
- remote transport

## Короткая формула

`X12 X11 wire subset v0` —
это не "маленький Xorg".

Это:

- local-only
- protocol-correct
- setup-aware
- 4-request bridge
- с минимальными ошибками
- поверх уже живого `X12` spine
