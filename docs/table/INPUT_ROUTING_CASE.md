# Кейс: Input Routing

## Зачем этот кейс

`input routing` выбран первым, потому что это один из самых опасных узлов во всем проекте.

Именно здесь сталкиваются:

- relation truth
- focus truth
- visible form
- final manifestation

Если `X12` не сможет разжать этот узел лучше, чем `Wayland`,
то весь разговор про разделение слоев останется красивой философией.

## Вопрос кейса

Кто в `X12` должен решать, куда идет input,
если композитор больше не является центром всей истины о мире?

## Как это работает в Wayland

В `Wayland` input routing фактически держится композитором.

Причина простая:

- композитор получает input от kernel-side substrate
- композитор знает, какая сцена сейчас видима
- композитор знает, какая surface где находится
- композитор знает, какой surface принадлежит focus
- композитор поэтому и решает, кому отправить pointer/keyboard/touch

Практический плюс:

- coherence высокая
- мало рассинхрона между тем, что видно, и тем, кто получает input

Архитектурная цена:

- input truth завязан на compositor-owned scene truth

## Почему этого недостаточно для X12

Если `X12` хочет:

- не делать композитор центром истины
- не сводить мир к одной compositor-owned scene
- разделять `table`, `crystall`, `manifest`

тогда input routing нельзя просто оставить как "пусть композитор решает".

Иначе произойдет одно из двух:

1. либо композитор снова тайно станет центром мира
2. либо input truth оторвется от visible truth и система начнет врать

Значит нужен промежуточный набор сущностей.

## Какие слои здесь реально участвуют

### Chaos

- сырой input signal
- события от evdev/libinput
- тайминг и device-level ограничения

### Table

- relation between process and graphic domain
- capability rights
- seat attachment
- разрешение на получение определенного класса input

### Crystall

- какая form сейчас активна
- какая form находится в каком field
- какая form допускает focus
- как устроено текущее устойчивое расположение графических форм

### Manifest

- финальная видимая сцена
- финальная трансформация
- прямой fullscreen path
- direct scanout case

## Первый вывод

`input routing` не живет чисто в одном слое.

Он живет на границе:

- `table`
- `crystall`

и иногда задевает `manifest`.

Значит его нельзя:

- ни целиком спустить в `table`
- ни целиком оставить в `manifest`

## Минимальный набор примитивов X12 для input routing

### 1. `relation`

Нужно знать:

- какой процесс к чему вообще привязан
- на какие fields он имеет право
- имеет ли он input rights
- какие manifestation paths ему разрешены

Без этого input routing превращается в произвольную магию поверх видимости.

### 2. `field`

Нужно знать:

- в каком graphic domain живет form
- shared это domain или dedicated
- может ли seat работать через несколько fields

Без `field` все снова скатывается к одному большому desktop-space.

### 3. `form`

Нужно знать:

- какая именно устойчивая графическая форма сейчас targetable
- принимает ли она input
- является ли она modal / passive / hidden / exclusive

### 4. `focus chain`

Нужно знать:

- какой путь у input authority прямо сейчас
- какой seat на кого смотрит
- кто имеет keyboard focus
- кто имеет pointer focus
- кто имеет touch stream ownership

Без `focus chain` input routing снова станет неявным выводом из scene order.

## Предварительная модель маршрута

Не код, а логика:

1. `chaos` получает raw input event
2. boundary-слой нормализует событие до input unit
3. `table` проверяет:
   - какой seat это породил
   - какие relations у seat сейчас активны
   - какие права вообще допустимы
4. `crystall` проверяет:
   - какой field сейчас активен
   - какая form внутри field является focus target
   - не действует ли modal/exclusive rule
5. если активен special `manifest path`:
   - fullscreen
   - dedicated field
   - direct path
   тогда routing может быть укорочен
6. событие доходит до target relation/process

## Главная проблема

Видимость и право не одно и то же.

То, что form видима, еще не значит, что она должна получать input.
И наоборот:
в некоторых режимах form может быть не просто "сверху",
а иметь формальное exclusive authority.

Значит `X12` не может строить input routing только по визуальному stacking order.

Это очень важный вывод.

## Где именно Wayland схлопывает слои

В `Wayland` композитор обычно одновременно знает:

- scene order
- visible transforms
- focus
- surface ownership
- output reality

Поэтому routing там получается цельным.

Но для `X12` это означает, что надо вытащить из этого монолита хотя бы часть истины:

- relation truth
- focus truth
- field truth

и не дать композитору снова все захватить.

## Гипотеза X12

`input routing` в `X12` должен определяться не композитором,
а пересечением трех вещей:

- `relation`
- `focus chain`
- текущей `crystall form`

`Manifest` вмешивается только в специальных режимах,
где final path действительно меняет правило маршрута:

- fullscreen exclusive
- direct presentation
- kiosk-like mode

## Что это дает

Если гипотеза верна, `X12` получает:

- input routing без compositor monarchy
- меньшую зависимость от одной общей scene truth
- возможность иметь несколько graphic domains без лжи о том, кто active

## Что пока не ясно

Пока еще неясно:

- как exactly описывать pointer hit-testing
- где заканчивается `crystall visibility`, а где начинается `manifest visibility`
- как routing ведет себя при нескольких carriers
- как routing должен работать между shared field и dedicated field

## Промежуточный вывод кейса

`Input routing` не разрушает `X12`, но сразу показывает:

- без `relation` нельзя
- без `focus chain` нельзя
- без `field` нельзя
- без distinction между form и final manifestation тоже нельзя

Главное:

`X12` не должен принимать правило
"кто визуально сверху, тот и есть истина input".

Это слишком wayland-shaped правило.

Для `X12` истина input должна быть формальнее, чем просто сцена.

## Следующий шаг по этому кейсу

После этой записи можно сделать одну из двух вещей:

1. углубить `input routing` до подпункта `pointer hit-testing`
2. перейти ко второму узлу: `scene graph ownership`

Сейчас разумнее перейти к `scene graph ownership`,
потому что именно он определит,
насколько вообще `input routing` может быть отделен от одной scene truth.
