# X12 X11 Bridge v0

## Зачем этот документ

Если `X12` действительно остается линией `X`,
то `X11`-совместимость должна считаться обязательной.

Но она не должна заражать ядро старой онтологией.

Поэтому нужен отдельный `X11 -> X12` bridge.

## Главный тезис

`X12` не должен сначала строиться как полностью автономный новый мир,
а потом когда-нибудь получать совместимость.

Более сильный путь:

> строить `X12` через обязательный `X11` bridge,
> и по мере построения этого bridge
> ядро `X12` будет кристаллизоваться само.

## Что bridge должен делать

Bridge должен:

- принимать `X11` request'ы
- принимать `X11` wire format
- не тащить старую `X11` онтологию в ядро
- переводить request'ы в server-side reality `X12`
- запускать packet-путь только через серверную сторону

## Важное уточнение: bridge не только semantic, но и wire-aware

`X11 bridge` — это не просто логический adapter.

Он должен уметь:

- принимать binary `X11` protocol
- парсить request'ы
- держать `opcode`, `serial`, `error`, `reply`, `event`
- возвращать `X11` wire format обратно клиенту

То есть это первое место в `X12`,
где `wire form` уже нужен не "когда-нибудь потом",
а сразу.

## Три внутренних слоя bridge

### 1. `transport`

Как данные вообще доезжают до bridge.

### 2. `x11 protocol layer`

Как binary `X11` request
становится понятной protocol-структурой.

### 3. `semantic translation`

Как распарсенный `X11` request
переводится в server-side reality `X12`.

## Что берем на v0

На старте не надо брать весь transport-мир `X11`.

Для v0 достаточно:

- `Unix domain socket`
- local-only bridge
- реальный binary `X11` protocol

То есть:

- server-shaped architecture сохраняется
- wire-совместимость сохраняется
- transport complexity режется

## Что откладываем

На v0 можно отложить:

- TCP transport
- remote X11 sessions
- полную network/server story старого `X`

Это не отменяет серверность.

Это просто означает,
что первый bridge будет:

- реальным
- protocol-correct
- но локальным

## Что bridge не должен делать

Bridge не должен:

- превращать `X12` в старый `X11`
- копировать старую оконную модель один в один в ядро
- требовать наличия legacy `window manager` / `compositor manager`
- смешивать compatibility boundary с внутренней онтологией `X12`

## Минимальный bridge v0

Не надо брать весь `X11` сразу.

Нужен самый узкий путь,
который дает живую систему.

### Первый набор request'ов

Для v0 достаточно начать с:

- `CreateWindow`
- `MapWindow`
- `UnmapWindow`
- `ConfigureWindow`

Потом добавить:

- базовые атрибуты окна
- базовую event delivery
- базовый input path

## Почему именно эти request'ы

Потому что они уже дают минимальную живую петлю:

- клиент просит окно
- сервер создает server-side сущность
- сервер делает форму видимой
- сервер меняет форму
- сервер убирает форму

То есть это уже почти вся базовая геометрическая жизнь старого `X`,
но без преждевременного захода в сложные расширения.

## Что откладываем

На старте не берем:

- `GLX`
- `MIT-SHM`
- `XRender`
- `Composite`
- `RandR`
- `XInput2`
- сложные extension paths
- pixmap-sharing
- full network/X11 extension zoo

## Как bridge ложится на X12

### `CreateWindow`

Bridge принимает request
и переводит его не в legacy window-object,
а в server-side claim формы.

Дальше:

- `chaos` рождает packet
- `table` создает relation/claim
- `crystall` собирает form
- `manifest` позже проявляет ее

### `MapWindow`

Bridge не “рисует окно”.

Он только поднимает request:

- форма должна стать проявимой

Дальше `crystall` решает:

- какие packet'ы реально доживут до `manifest`
- какие packet'ы будут отрезаны перекрытием

### `UnmapWindow`

Bridge поднимает request:

- форма больше не должна проявляться

Дальше:

- `crystall` убирает packet-rights этой формы
- `manifest` перестает получать packet'ы от нее

### `ConfigureWindow`

Bridge поднимает request изменения формы:

- position
- size
- stacking claim

`table` валидирует структуру,
`crystall` перестраивает видимую форму,
`manifest` получает новый итог.

## Физическая форма реализации

На старте это лучше делать не как 4 отдельных программы,
а как:

- один `x12-server`
- внутри модуль `x11_bridge`

То есть:

- ядро одно
- bridge отдельный boundary-module

А внутри самого `x11_bridge` уже должны быть подмодули:

- `transport/unix`
- `protocol/x11`
- `translate/x11_to_x12`

## Как это тестировать

### Этап 1

Headless / machine-trace:

- request пришел
- packet родился
- packet прошел `table`
- packet дошел или не дошел до `crystall`
- packet дошел или не дошел до `manifest`

### Этап 2

Минимальный fake/text manifest:

- список форм
- stacking
- visibility
- occlusion

### Этап 3

Минимальный реальный `X11` клиент:

- маленькая test-прога
- `CreateWindow`
- `MapWindow`
- `ConfigureWindow`
- `UnmapWindow`

### Этап 4

Простые реальные `X11` программы:

- `xclock`
- `xeyes`
- `xterm`

## Почему это хороший маршрут

Этот маршрут:

- не требует написать весь `X12` с нуля заранее
- сразу дает practical test
- заставляет ядро кристаллизоваться честно
- удерживает `X12` как настоящий `X`,
  а не как просто новый packet-GUI

## Короткая формула

`X12` не надо писать "с нуля в пустоте".

Его правильнее проявлять через
обязательный `X11 -> X12` bridge.

Если bridge живой,
то и `X12` начинает становиться живым не только в онтологии,
но и в реальной совместимой форме.
