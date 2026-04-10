# X12 Next Step: Client Session v0

## Зачем этот документ

После:

- живого `X12` spine
- `X11` wire handshake
- request parsing
- real `X11` smoke-test

следующий шаг уже не про "еще один request".

Следующий шаг про то,
что `X12` должен стать сервером
не только по форме boundary,
но и по внутренней server identity.

## Главный тезис

Следующий правильный шаг для `X12`:

> ввести `ClientSession`

Не "сделать loop".
Не "принять второй request".

А именно:

- ввести session identity
- ввести session-owned sequence counter
- ввести session-owned XID space

Потому что `X12` все-таки сервер.

## Почему это важно

### 1. Loop без session state — это не сервер

Если сервер просто крутится в цикле и принимает request'ы,
но не знает:

- кто клиент
- какой у него sequence
- какой у него XID range

то это не настоящая server semantics,
а только boundary loop.

### 2. XID allocation — это server function

В `X11` клиент не должен быть сувереном resource id.

Сервер:

- задает клиенту xid-space
- определяет `xid_base`
- определяет `xid_mask`
- и именно сервер решает,
  может ли клиент использовать данный id

Если этого нет,
серверность `X12` остается неполной.

### 3. Per-client isolation нельзя откладывать слишком поздно

Это не только про коллизии вида:

- client A создал `id=77`
- client B тоже создал `id=77`

Это еще и про фундаментальную truth:

- кому вообще принадлежит resource
- кто имеет право ссылаться на него
- может ли клиент подделать чужой id

## Что должно появиться следующим

### `ClientSession`

Минимальная сущность:

- `session_id`
- `next_sequence: u16`
- `xid_base: u32`
- `xid_mask: u32`
- `setup_done: bool`

Потом можно добавить:

- auth state
- root binding
- event queue
- per-client registry

Но для `v0` это не обязательно.

## Следующий порядок работы

### 1. Ввести `ClientSession`

Пока даже в минимальной форме.

### 2. `accept_client_once` -> `accept_client_session`

То есть:

- один setup
- потом loop request'ов
- sequence number живет внутри session

### 3. Multi-request smoke

Первый честный session test:

- `CreateWindow`
- `MapWindow`
- `ConfigureWindow`

в одном соединении.

### 4. XID validation

Клиент может использовать только ids
из своего server-assigned range.

### 5. Только потом events

После этого уже нормально добавлять:

- `MapNotify`
- `ConfigureNotify`
- `Expose`

Потому что тогда у этих событий уже будет
настоящий session-owner.

## Чего не надо делать

Не надо сначала делать:

- большой loop
- второй клиент
- event zoo

без `ClientSession`.

Иначе потом придется засовывать server identity
в уже распухший boundary code.

## Как это связано с `table`

Это не только transport issue.

`ClientSession` очень сильно касается `table`,
потому что именно там живет truth:

- relation client -> resource
- relation client -> xid-space
- legality of request
- ownership of forms

То есть это не просто network plumbing,
а следующий настоящий кусок server ontology.

## Короткая формула

Следующий шаг после `Milestone 1`:

- не "больше request'ов"
- а "первая настоящая `ClientSession`"

Потому что `X12` — это сервер,
а сервер начинается с identity.
