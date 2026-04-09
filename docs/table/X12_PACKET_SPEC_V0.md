# X12 Packet Spec v0

## Назначение

Этот документ фиксирует первый технический драфт `X12 Packet`.

Не финальная схема.
Не ABI.
Не C-структура один в один.

Это техническая форма того,
что packet в `X12` уже должен означать.

## Коротко

`X12 Packet` — это один packet,
который:

- рождается из события в `chaos`
- растет по мере прохождения через слои
- сам выбирает следующий шаг внутри допустимой topology
- внизу либо манифестируется,
  либо завершает внутренний переход без визуального вывода

## Главный принцип

`packet` не царь системы.

`packet` — это курьер и носитель процесса.

Он не владеет truth сам по себе,
а удерживает truth,
которую в него вписывают слои и операторы.

## Жизненный цикл

### 1. Рождение

Packet рождается в `chaos`,
когда происходит один из входных импульсов:

- input event
- driver event
- output event
- hotplug event
- timing/vblank event
- client update signal
- internal runtime wakeup

Это рождение не дает packet полной формы.
Оно дает только seed.

### 2. Рост

Packet идет через допустимые шаги topology
и по дороге наполняется.

Рост идет сверху вниз:

- `chaos`
- `table`
- `crystall`
- `manifest`

### 3. Разрешение

Packet по дороге:

- получает связи
- теряет устаревшие связи
- кристаллизует форму
- проходит выбор пути
- получает runtime continuity

### 4. Завершение

В конце packet:

- либо проявляется в `manifest`
- либо завершает внутренний переход
- либо уходит в reduced/suspended path

### 5. Исчерпание или продолжение

После завершения packet:

- либо исчерпывается
- либо оставляет residue в runtime
- либо порождает следующий packet через continuity

## Минимальный seed

Packet не должен рождаться полным.

Минимально достаточный `seed` должен содержать:

- `packet_id`
- `tick_id`
- `origin`
- `process_kind`
- `current_layer`
- `current_operator`
- `raw_event`
- `log_ledger`

### `packet_id`

Уникальный идентификатор packet.

### `tick_id`

Идентификатор текущего хода/тика/runtime-цикла.

### `origin`

Откуда packet родился:

- `input`
- `driver`
- `output`
- `client`
- `runtime`
- `hotplug`

### `process_kind`

Какой процесс этот packet несет.

Рабочие варианты v0:

- `input`
- `scene`
- `visibility`
- `presentation`
- `topology`
- `runtime`

### `current_layer`

Где packet находится сейчас:

- `chaos`
- `table`
- `crystall`
- `manifest`

### `current_operator`

Текущий operator-state:

- `FLOW`
- `CONNECT`
- `DISSOLVE`
- `OBSERVE`
- `ENCODE`
- `CHOOSE`
- `LOGIC`
- `CYCLE`
- `RUNTIME`
- `MANIFEST`

### `raw_event`

Сырой импульс,
из которого packet вырос.

Примеры:

- движение мыши
- нажатие клавиши
- damage signal
- carrier mode change
- hotplug

### `log_ledger`

Append-only лог packet-истории.

Он существует с рождения packet,
даже если сначала почти пустой.

## Зоны роста packet

Packet один,
но растет по секциям.

### `header`

Техническая идентичность packet:

- `packet_id`
- `tick_id`
- `origin`
- `process_kind`
- `current_layer`
- `current_operator`

### `seed`

Первичный импульс:

- raw input
- raw driver signal
- raw update cause
- initial constraints

### `state.relations`

То, что packet узнал в `table`:

- client bindings
- seat bindings
- field access
- capability truth
- relation truth

### `state.fields`

Доменная структура packet:

- active field
- candidate fields
- dedicated/shared mode
- carrier relation pointers

### `state.forms`

То, что packet узнал или зафиксировал про форму:

- target form
- active form
- role
- modal/fullscreen/exclusive flags
- form state

### `state.visibility`

То, что packet знает о видимости:

- visible
- partially occluded
- fully occluded
- hidden
- exclusive visible

И связанные вещи:

- visible area
- targetability
- manifestation priority

### `state.focus`

То, что packet знает о маршруте внимания:

- keyboard focus
- pointer focus
- touch ownership
- focus chain

### `state.manifest`

То, как packet должен завершиться внизу:

- manifest path
- carrier target
- scanout eligibility
- direct/composed path
- present cadence

### `metrics`

Наблюдение за packet:

- frame pressure
- latency
- refresh mismatch
- routing conflict
- composition cost
- `PU`

### `log_ledger`

Растущий след packet.

Рабочие записи:

- `DISSOLVE`
- `CHOOSE`
- `ENCODE`
- `MANIFEST`
- bridge/translation
- forced fallback
- forced collapse

## Правило роста по слоям

### `chaos`

Дает packet:

- seed
- raw impulse
- raw constraints
- driver reality
- input reality

И может породить:

- `FLOW`
- `DISSOLVE`
- начальный `CONNECT`

### `table`

Дает packet:

- relation truth
- capability truth
- field access
- route candidates
- addressability

### `crystall`

Дает packet:

- form truth
- focus truth
- visibility truth
- composition-ready truth
- manifestation priority

### `manifest`

Дает packet:

- final present path
- final carrier target
- final scanout contract
- final realization mode

## Правило маршрута

Packet сам выбирает следующий шаг.

Но он не может идти куда угодно.

Ограничения два:

- topology операторов `ProcessLang`
- topology уровней `chaos -> table -> crystall -> manifest`

То есть:

- packet self-routed
- но topology-bounded

Рабочая формула:

`next_step = packet.choose_next(pl_topology, layer_topology)`

## Что packet не должен делать

Packet не должен:

- быть хозяином мира
- заменять собой все сущности системы
- хранить всю систему целиком
- самовольно выходить за topology
- быть сразу готовым экраном

## Что packet должен делать

Packet должен:

- нести текущий переход процесса
- расти по мере прохождения
- собирать truth по слоям
- сохранять историю перехода в `log_ledger`
- доводить процесс до manifest или внутреннего завершения

## Первый практический тест

Если packet-spec живой,
мы должны суметь прогнать через него простой сценарий:

- мышь сдвинулась
- packet родился в `chaos`
- в `table` получил relation
- в `crystall` получил target form / focus / visibility
- в `manifest` либо вызвал visual update,
  либо не вызвал его,
  если визуально ничего не изменилось

Если такой walkthrough не собирается,
значит spec еще сырая.
