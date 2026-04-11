# X12 Server / Client Layer Split v0

## Зачем этот документ

Этот документ фиксирует сильную рабочую гипотезу:

- `chaos` и `table` лучше читать как server-side слои
- `crystall` и `manifest` лучше читать как client-near слои

Это не отменяет того,
что `X12` в целом остается server-shaped системой.

Это уточняет,
как внутри `X12` делится authority.

## Главная формула

### Server side

- `chaos`
- `table`

### Client-near side

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

## Почему `crystall` client-near

`crystall` ближе к:

- shape
- visibility concretization
- визуальной форме
- "красивости"
- конкретному виду формы

Серверу не обязательно знать,
что у окна красивая тень или сложный внутренний shape.

Это уже ближе к клиентской стороне проявления.

## Почему `manifest` client-near

`manifest` тоже ближе к клиентской стороне,
потому что он связан с:

- final visual realization
- packet-flow проявления
- concrete output surface
- тем, как форма реально выглядит на выходе

Но это не значит,
что `manifest` становится полностью client-sovereign.

## Важное ограничение

Несмотря на то, что
`crystall` и `manifest` читаются как client-near,
`X12` все равно остается сервером.

Значит:

- final display legality не уходит от сервера
- сервер по-прежнему ограничивает,
  что вообще может дойти до экрана
- клиент не решает глобальный stacking order
- клиент не решает мировую relation truth

То есть:

- client-near != client-sovereign

## Что это дает

Такое разделение помогает убрать путаницу:

### Сервер знает

- какие формы вообще существуют
- что над чем находится
- кому что принадлежит
- какие claims legal
- какие packet-path вообще допустимы

### Клиент знает

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

### Client-driven part

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
- клиент держит форму и проявление
- сервер все равно сохраняет final boundary authority

## Открытые вопросы

Это разделение выглядит сильным,
но еще не добито до конца.

Надо отдельно думать:

- насколько буквально `manifest` должен быть client-near
- где проходит граница между `crystall` и `manifest`
- как это ляжет на real Linux output path
- что именно server final authority будет означать
  для client-near manifest

## Текущий вывод

Рабочая гипотеза сейчас такая:

- `chaos` + `table` = server core
- `crystall` + `manifest` = client-near realization

И это выглядит чище,
чем пытаться делить authority
по случайным типам событий.
