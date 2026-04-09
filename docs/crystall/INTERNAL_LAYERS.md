# Внутренние Слои Crystall

## Зачем этот документ

Хотя `crystall` является одним уровнем абстракции,
внутри него тоже есть внутренняя слоистость.

Это важно для `X12`, потому что:

- `window manager`
- `compositor`
- focus/form logic
- scene assembly

не являются одной и той же функцией,
хотя и живут очень близко друг к другу.

## Основная гипотеза

Внутри `crystall` есть как минимум два близких внутренних подуровня:

### Верхний `crystall`

Здесь живет закон формы:

- `window manager`
- `placement law`
- `focus form logic`
- workspace / shell rules
- role stabilization
- rules of active/inactive/modal/exclusive form

### Нижний `crystall`

Здесь живет закон видимости и сборки сцены:

- `compositor core`
- stacking resolution
- transforms
- occlusion
- visibility preparation
- damage/composition preparation

## Почему это важно

Если держать `window manager` и `compositor` слишком далеко,
мы рискуем повторить старую боль `X11`.

Если слить их в одного царя,
мы рискуем повторить `Wayland`.

Поэтому текущая рабочая гипотеза такая:

- `window manager` и `compositor` живут в одном уровне `crystall`
- но не обязаны быть одной физической сущностью
- между ними должен быть родной `X12`-канал / protocol contract

## Рабочая модель

### `window manager`

Знает:

- где form должна находиться
- какая у form роль
- активна ли form
- modal ли form
- fullscreen ли form
- как устроены workspace/layout rules

### `compositor core`

Знает:

- что реально видно
- в каком порядке это видно
- какие transforms применяются
- какие части сцены требуют пересборки
- как forms собираются в coherent visible scene

## Что не должен забирать `compositor`

`Compositor core` не должен становиться владельцем всего `table`.

Он не должен единолично определять:

- object model
- relation model
- capability truth
- всю input truth
- всю lifecycle truth

Иначе `X12` снова скатится в wayland-shaped center.

## Что должно идти через канал между `window manager` и `compositor`

Минимально:

- role/type form
- geometry
- active/inactive state
- stacking intent
- modal/exclusive state
- fullscreen eligibility
- effect eligibility
- visibility-relevant changes
- damage/composition hints

## Герцовка и уровни абстракции

Нужно различать три вещи:

### 1. Герцовка как физическое свойство output

Это ближе к `chaos` и к границе `table`.

Примеры:

- монитор поддерживает `60 Hz`
- монитор поддерживает `144 Hz`
- монитор поддерживает набор mode combinations

### 2. Герцовка как опубликованная capability / mode info

Это `table`.

Потому что система должна знать и описывать:

- какие carriers какие modes поддерживают
- какие refresh rates доступны
- какие combinations вообще допустимы

### 3. Герцовка как поведение сцены

Это `crystall`.

Потому что именно здесь решается:

- какой field на каком carrier живет
- как mixed-refresh behavior удерживается
- как scene assembly ведет себя при разных outputs

### 4. Герцовка как финальный presentation decision

Это граница `manifest`.

Потому что в конце есть конкретный act of present / scanout / submit.

## Короткая формула

- герцовка как факт output = `chaos/table`
- герцовка как capability = `table`
- герцовка как поведение сцены = `crystall`
- герцовка как финальный present = `manifest` boundary

## Промежуточный вывод

`Crystall` не является плоским.

Внутри него уже видно:

- верхний слой формы
- нижний слой видимости и сборки сцены

Это делает модель `X12` сильнее, потому что:

- `window manager` не уезжает слишком высоко
- `compositor` не падает слишком низко
- но и не захватывает весь мир целиком
