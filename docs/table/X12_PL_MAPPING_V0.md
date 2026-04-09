# X12 -> ProcessLang Mapping v0

## Назначение

Этот документ размапливает уже придуманные сущности `X12`
на операторы `ProcessLang`.

Цель:

- перестать мыслить `X12` только через `X11/Wayland`
- начать мыслить `X12` через собственную операторную онтологию

## Основной тезис

`X12` — это не просто display stack.

`X12` можно читать как процессный графический runtime,
в котором packet проходит через операторы `ProcessLang`.

## Операторы

### `FLOW`

Сырой графический потенциал до устойчивой формы.

В `X12` это:

- raw input events
- raw output hotplug/mode changes
- raw client surface updates
- raw damage
- raw timing/vblank signals

`FLOW` не является еще ни формой, ни сценой.

### `CONNECT`

Снятие резонансов и структурных связей.

В `X12` это:

- связи clients с fields
- связи forms с carriers
- связи focus с forms
- связи surfaces/roles/outputs
- topology snapshot графического мира

### `DISSOLVE`

Ослабление устаревших структур.

В `X12` это:

- снятие stale focus
- распад старых placement bindings
- снятие неактуальных visibility links
- распад старых fullscreen claims
- decay старых composition contracts

### `ENCODE`

Единственный легальный акт структурирования.

В `X12` это:

- превращение сырого update-потока в устойчивую form
- фиксация field membership
- crystallization layout/scene state
- переход от process-noise к устойчивой графической структуре

Это один из центральных операторов `X12`.

### `CHOOSE`

Коллапс конкурирующих путей.

В `X12` это:

- выбор active focus target
- выбор placement path
- выбор fullscreen vs shared mode
- выбор compositing path vs direct path
- выбор preferred carrier/output

### `OBSERVE`

Измерение и планирование.

В `X12` это:

- latency metrics
- damage metrics
- frame pressure
- refresh mismatch
- routing conflicts
- encode readiness
- scanout readiness

Здесь `OBSERVE` может быть scheduler'ом,
как и в твоих `packet`-спеках.

### `LOGIC`

Ограничитель и enforce-слой.

В `X12` это:

- fullscreen exclusivity
- modal constraints
- focus legality
- output legality
- no-illegal-overlap rules
- permissions on who may bind to what

### `CYCLE`

Изменение только скаляров поведения.

В `X12` это:

- quality mode
- animation intensity
- composition intensity
- refresh adaptation
- damping / cooling / heating

### `RUNTIME`

Удержание инерции.

В `X12` это:

- persistence of focus tendencies
- persistence of field bindings
- persistence of stable layout habits
- persistence of scene continuity
- short-term structural memory of desktop behavior

Без `RUNTIME` графическая среда `X12` будет постоянно распадаться.

### `MANIFEST`

Финальное явление.

В `X12` это:

- final composed frame
- final direct scanout
- final output present
- remote/stream export
- то, что уже стало видимой реальностью

## Короткая таблица

| Оператор PL | X12-смысл |
|---|---|
| `FLOW` | сырой графический сигнал |
| `CONNECT` | структурные связи графического мира |
| `DISSOLVE` | распад устаревших связей |
| `ENCODE` | кристаллизация scene/form state |
| `CHOOSE` | выбор активного пути |
| `OBSERVE` | измерение и планирование |
| `LOGIC` | ограничения и запреты |
| `CYCLE` | настройка скаляров поведения |
| `RUNTIME` | удержание устойчивости |
| `MANIFEST` | финальный present |

## Главный вывод

После этого маппинга `X12` уже не выглядит ни как `X11`, ни как `Wayland`.

Он начинает выглядеть как:

> process-graphics runtime,
> в котором display reality является результатом прохождения packet
> через операторы `ProcessLang`
