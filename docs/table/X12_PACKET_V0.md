# X12 Packet v0

## Status

`Legacy / superseded by atomic packet model`

Документ не удаляется.

Но после `X12_ATOMIC_PACKET_MODEL_V0.md`
его нельзя читать как каноничное определение packet.

Теперь он полезен как:

- исторический шаг мысли
- legacy-драфт
- описание более крупной сущности,
  чем packet-атом

## Зачем нужен этот документ

До этого `X12` рассматривался через сравнение с `X11` и `Wayland`.

Но после сопоставления с `ProcessLang` и `packet` стало видно:

`X12` не обязан строиться вокруг старых сущностей вроде:

- screen
- compositor space
- window tree
- output layout

`X12` может строиться вокруг **контейнера текущего графического процесса**.

Это и есть `X12 Packet`.

## Базовый тезис

`X12 Packet` — это не сообщение и не готовая сцена.

Это:

> контейнер состояния текущего графического процесса

То есть:

- не "весь мир X12 сразу"
- не "экран"
- не "окно"
- не "итоговый кадр"

А текущее состояние одного живого графического хода,
который проходит через операторы и в конце может стать manifestation.

## Чем `X12 Packet` не является

`X12 Packet` не является:

- текстом
- логом
- командой
- прямым API-запросом приложения
- готовой картинкой
- screen-space как таковым

## Что является Packet-процессом в X12

Примеры:

- процесс маршрутизации ввода
- процесс перестройки сцены
- процесс fullscreen/direct path
- процесс output topology update
- процесс field placement
- процесс materialization в конечный present

То есть один `packet` = один графический процессный контур.

## Почему это важно

В `X11` и `Wayland` основой становятся уже оформленные display-сущности.

В `X12` основой может стать:

- процесс
- его текущее структурное состояние
- и правила его переходов

При этом packet не обязательно полон с самого начала.
Он заполняется по мере прохождения через уровни:

- `chaos`
- `table`
- `crystall`
- `manifest`

Это делает систему:

- ближе к `ProcessLang`
- ближе к `packet`-мышлению
- менее зависимой от исторических онтологий display stack

## Предварительная структура `X12 Packet`

Важно:

эти секции не означают,
что весь packet обязан быть полностью заполнен с самого начала.

Они означают зоны,
которые постепенно наполняются truth'ами по мере прохождения packet через слои.

### `header`

Метаданные процесса:

- `schema_version`
- `packet_id`
- `tick_id`
- `mode`
- `current_module`
- `next_module`
- `process_kind`

Примеры `process_kind`:

- `input`
- `scene`
- `topology`
- `fullscreen`
- `manifest`

### `state.relations`

Связи процесса:

- какие клиенты к чему привязаны
- какие fields им разрешены
- какие capabilities у них есть

### `state.fields`

Графические домены:

- shared / dedicated / transient / isolated
- связи fields с forms
- связи fields с carriers

### `state.forms`

Устойчивые графические формы:

- geometry
- role
- active/inactive
- modal/exclusive
- fullscreen state
- focus eligibility

### `state.carriers`

Носители manifestation:

- physical outputs
- virtual outputs
- remote outputs
- mode capabilities
- refresh capabilities
- scale / transform facts

### `state.focus_chain`

Маршрут authority:

- keyboard focus
- pointer focus
- touch stream ownership
- relation-path
- field-path

### `state.visibility_graph`

Производная видимость:

- какие forms потенциально видимы
- на каких carriers
- через какие manifestation paths

### `state.composition`

Подготовка сцены:

- stacking intent
- transforms
- damage regions
- effect eligibility
- composition hints

### `state.scanout_contract`

Граница final presentation:

- direct path eligibility
- carrier eligibility
- timing constraints
- mode constraints

### `metrics`

Наблюдение:

- frame pressure
- latency
- routing conflicts
- refresh mismatch
- composition cost
- fluidity
- `PU`

### `log_ledger`

Append-only лог packet-истории:

- `DISSOLVE`
- `CHOOSE`
- `ENCODE`
- `MANIFEST`
- bridge/translation
- forced collapse points

## Где живет Packet

`X12 Packet` начинается в `chaos`,
но основное наполнение получает между:

- `table`
- `crystall`
- `manifest`

Он не равен ни одному из этих уровней,
а является телом процесса, проходящего через них.

## Самая важная мысль

В `UPM` Packet несет состояние процесса,
а смысл появляется поздно, в `MANIFEST`.

В `X12` это переносится почти напрямую:

- packet несет структурное состояние графического процесса
- финальная картинка появляется только в `MANIFEST`
- packet становится все более определенным по мере прохождения через уровни

## Уточнение по `PU`

В `X12 v0` `PU` пока не является валютой процесса.

Пока `PU` означает:

- показатель скорости
- показатель ёмкости
- показатель того,
  насколько тяжело данный графический процесс удерживается на данном железе

То есть `PU` сейчас ближе к:

- throughput score
- render capacity
- graphics speed index

а не к обязательной оплате переходов.

## Вывод

С этого момента `X12` начинает выглядеть не как вариант `X11` или `Wayland`,
а как отдельный process-graphics runtime.
