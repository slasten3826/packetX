# X12 Server / Client Layer Split v0

## Зачем этот документ

Этот документ фиксирует сильную рабочую гипотезу:

- `chaos` и `table` лучше читать как server-sovereign слои
- `crystall` и `manifest` лучше читать как
  `client-informed, server-controlled` слои

Это не отменяет того,
что `X12` в целом остается server-shaped системой.

Это уточняет,
как внутри `X12` делится authority.

## Главная формула

### Server-sovereign

- `chaos`
- `table`

### Client-informed, server-controlled

- `crystall`
- `manifest`

## Почему `chaos` server-side

`chaos` ближе всего к:

- hardware reality
- input
- network ingress
- driver truth
- packet birth

Это не клиентская зона.
Это верхняя boundary-зона сервера.

## Почему `table` server-side

`table` держит:

- relation truth
- stacking order
- ownership
- legality
- addressability
- "что над чем"

То есть `table` знает структуру мира,
а не красоту формы.

Это очень серверная truth.

## Почему `crystall` client-informed, server-controlled

`crystall` ближе к:

- shape
- visibility concretization
- визуальной форме
- "красивости"
- конкретному виду формы

Серверу не обязательно знать,
что у окна красивая тень или сложный внутренний shape.

Но это не делает `crystall`
клиентским сувереном.

Более точное чтение:

- клиент информирует `crystall`
  о visual detail и content shape
- сервер все равно контролирует
  visibility truth и final legality

## Почему `manifest` client-informed, server-controlled

`manifest` тоже связан с клиентской стороной,
потому что он связан с:

- final visual realization
- packet-flow проявления
- concrete output surface
- тем, как форма реально выглядит на выходе

Но здесь особенно важно не уйти в ложную формулировку.

`manifest` не должен читаться как client-controlled,
потому что:

- scanout
- page flip
- output commit
- владение тем, что физически видно на мониторе

для `X12` остаются server operations.

Поэтому:

- клиент может информировать `manifest`
  о content changes / dirty regions / visual surface
- но final output commit остается server-controlled

## Важное ограничение

Несмотря на то, что
`crystall` и `manifest` читаются как
client-informed,
`X12` все равно остается сервером.

Значит:

- final display legality не уходит от сервера
- сервер по-прежнему ограничивает,
  что вообще может дойти до экрана
- клиент не решает глобальный stacking order
- клиент не решает мировую relation truth

То есть:

- client-informed != client-controlled

## Что это дает

Такое разделение помогает убрать путаницу:

### Сервер знает

- какие формы вообще существуют
- что над чем находится
- кому что принадлежит
- какие claims legal
- какие packet-path вообще допустимы

### Клиент знает / сообщает

- как его форма реально выглядит
- что внутри формы изменилось
- какие regions/tiles его surface dirty
- какая visual detail у его формы

## Что это значит для damage

Из этого разделения естественно следует
не мутный общий гибрид,
а более чистая схема:

### Server-driven part

Через `chaos` и `table`:

- world changes
- stacking changes
- ownership changes
- topology changes
- legality changes

### Client-informed part

Через `crystall` и `manifest`:

- content changes
- surface dirtiness
- visual detail changes
- form-internal realization

То есть:

- сервер знает мир вокруг формы
- клиент знает содержимое формы

## Короткая формула

Не так:

- сервер знает все
- или клиент знает все

А так:

- сервер держит структуру мира
- клиент информирует систему о своей форме и содержимом
- сервер все равно сохраняет final boundary authority

## Уточнение про `crystall policy`

Здесь есть отдельная важная развилка.

Надо различать:

- `crystall core`
- `crystall policy`

### `crystall core`

Это server-controlled часть:

- visibility filter
- overlap resolution
- packet-pressure filtering
- final visual legality

### `crystall policy`

Это уже можно читать как
privileged client logic:

- WM-like decisions
- raise / lower intentions
- focus-related policy
- visual policy hints

Но такая policy не должна менять server truth напрямую.

Она должна производить intention / request,
который потом проходит через server path
и валидируется в `table`.

## Открытые вопросы

Это разделение выглядит сильным,
но еще не добито до конца.

Надо отдельно думать:

- насколько буквально `manifest` должен быть
  client-informed, а не client-controlled
- где проходит граница между `crystall` и `manifest`
- как это ляжет на real Linux output path
- что именно server final authority будет означать
  для client-informed manifest

## Текущий вывод

Рабочая гипотеза сейчас такая:

- `chaos` + `table` = server-sovereign core
- `crystall` + `manifest` = client-informed,
  но server-controlled realization

И это выглядит чище,
чем пытаться делить authority
по случайным типам событий.
