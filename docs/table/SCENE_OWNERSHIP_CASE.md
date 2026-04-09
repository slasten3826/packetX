# Кейс: Scene Ownership

## Зачем этот кейс

После `input routing` следующий главный вопрос такой:

кто вообще владеет истиной о сцене?

То есть:

- кто знает, что где находится
- кто знает, что реально видно
- кто знает, что перекрыто
- кто знает, какая form должна проявляться

В `Wayland` это в основном держит compositor.

Если `X12` хочет быть другой системой,
нужно понять:

можно ли вынуть эту истину из compositor-centered world model,
не развалив coherence.

## Вопрос кейса

Нужен ли `X12` один владелец сцены,
или scene truth можно разложить на несколько сущностей?

## Как это выглядит в Wayland

В `Wayland` compositor обычно знает все вместе:

- scene graph
- stacking order
- transforms
- visibility
- occlusion
- output relation

Это практично.

Но это делает compositor:

- не просто manifestation layer
- а владельцем истины о мире

## Почему для X12 это проблема

Если повторить это,
`X12` снова станет wayland-shaped center.

Если выбросить это совсем,
система начнет врать:

- input не совпадет с видимостью
- composition не совпадет с формой
- outputs не совпадут с manifestation

Значит вопрос не в том,
нужна ли scene truth.

Вопрос в том,
может ли scene truth быть распределенной.

## Первое различение

Надо развести три вещи,
которые в старых системах часто склеены:

### 1. truth of form

Что вообще существует как устойчивая графическая форма.

Это:

- geometry
- role
- active/inactive
- modal/exclusive
- field membership

Это больше похоже на `crystall`.

### 2. truth of visibility

Что из этих forms реально видно,
в какой степени,
на каких carriers,
через какие paths.

Это уже не просто форма.
Это производная.

### 3. truth of manifestation

Что именно сейчас должно быть выведено наружу,
с какой частотой,
с каким priority,
по какому present path.

Это уже ближе к `manifest`.

## Гипотеза X12

У `X12` не должно быть одного scene-owner как абсолютного царя.

Вместо этого:

- `form truth` живет в `form`
- `placement truth` живет в `field + placement law`
- `visibility truth` живет в `visibility graph`
- `manifest truth` живет в `manifest path`

То есть сцена не является одним объектом.

Сцена является:

> производной сборкой нескольких truths

## Что тогда делает compositor core

Текущая гипотеза:

`compositor core` не владеет всем миром,
а собирает manifestation-ready scene
из уже существующих truths.

То есть он:

- не определяет всю онтологию
- не является owner'ом всех отношений
- но является сильным сборщиком видимой сцены

Это важная разница.

## Основные примитивы для замещения scene ownership

### `form`

Несет truth of stable visible form.

### `field`

Несет domain truth:

- где форма вообще живет
- shared это домен или dedicated

### `placement law`

Несет truth of arrangement:

- как forms размещаются
- как fields связаны с carriers

### `visibility graph`

Несет truth of actual visibility:

- что видно
- что перекрыто
- что частично видно
- какие forms получают manifestation priority

### `manifest path`

Несет truth of final realization:

- full-rate path
- reduced-rate path
- direct path
- snapshot path

## Главный конфликт

Можно сказать:

"ну хорошо, раз visibility graph все знает,
значит он и есть новый compositor"

Это риск.

Чтобы не повторить Wayland,
надо держать distinction:

- `visibility graph` — это derived truth
- `compositor core` — это engine scene assembly

То есть derived graph != sovereign owner.

## Промежуточный вывод

Сейчас рабочая гипотеза такая:

`X12` не имеет одного абсолютного scene-owner.

Вместо этого он имеет:

- truth of form
- truth of placement
- truth of visibility
- truth of manifestation

И уже из этого compositor core собирает final scene.

## Что пока остается неясным

- как exactly строится `visibility graph`
- является ли он snapshot-only или живой runtime-структурой
- как считать partial occlusion
- где заканчивается `crystall visibility`, а где начинается `manifest visibility`
- как несколько carriers меняют scene truth

## Следующий шаг

После этого кейса логично идти либо в:

- `output placement`

либо глубже в:

- `visibility graph`

Потому что именно они добьют вопрос,
можно ли жить без одной compositor-owned scene truth.
