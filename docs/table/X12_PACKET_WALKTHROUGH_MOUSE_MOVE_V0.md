# X12 Packet Walkthrough v0: Mouse Move

## Зачем этот документ

Этот walkthrough проверяет,
живет ли `X12 Packet Spec v0`
на простом реальном сценарии.

Сценарий:

- пользователь двинул мышкой
- система должна понять,
  кому идет pointer movement
- и решить,
  нужен ли вообще visual update

## Начальные условия

В системе уже есть:

- один `field` рабочего стола
- несколько `form`
- одна активная `form`
- один pointer/seat
- один carrier/monitor

Пользователь двигает мышкой,
но не нажимает кнопку.

## Шаг 1. Рождение packet в `chaos`

В `chaos` приходит raw input event:

- устройство: mouse
- тип: motion
- delta: `dx`, `dy`
- timestamp

Из этого рождается packet seed.

### Что в packet есть сразу

- `packet_id`
- `tick_id`
- `origin = input`
- `process_kind = input`
- `current_layer = chaos`
- `current_operator = FLOW`
- `raw_event = pointer_motion`
- пустой `log_ledger`

На этом шаге packet еще не знает:

- какое окно target
- какой `field` активен
- влияет ли это на scene
- нужен ли redraw

## Шаг 2. `FLOW` в `chaos`

`FLOW` оформляет сырой импульс как живое движение,
но еще не назначает адресата.

Packet получает:

- нормализованный pointer motion
- device/source metadata
- raw timing context

Если на этом шаге обнаруживается аппаратная аномалия,
она тоже пишется в packet.

## Шаг 3. `CONNECT` на границе `chaos -> table`

Packet поднимается в `table`.

Теперь система смотрит:

- какой `seat` прислал событие
- к какому `field` сейчас привязан этот pointer
- есть ли active relation между seat и current field

Packet дополняется:

- `state.relations.seat = pointer-seat-0`
- `state.relations.active_field = desktop-field-0`
- `state.relations.capability = pointer-motion`

На этом шаге packet уже знает,
в какой домен он вошел,
но еще не знает точную form-цель.

## Шаг 4. `OBSERVE` в `table`

Теперь packet наблюдает текущее состояние:

- где pointer был до этого
- изменился ли carrier
- есть ли конфликт route
- есть ли stale bindings

Если что-то распалось раньше,
packet видит это через state и `log_ledger`,
но сам еще не обязан манифестироваться.

Возможные исходы:

- packet стабилен и идет дальше
- packet видит stale state и идет через `DISSOLVE`
- packet видит конфликт и готовится к `CHOOSE`

В нормальном кейсе мышки:

- конфликтов нет
- packet идет дальше

## Шаг 5. `ENCODE` / `CHOOSE` в `crystall`

Packet входит в `crystall`.

Здесь выясняется уже форма:

- какая `form` сейчас под pointer
- видима ли она
- targetable ли она
- не перекрыта ли она modal/exclusive слоем

Packet дополняется:

- `state.forms.target_form`
- `state.visibility.visible_area`
- `state.visibility.targetability`
- `state.focus.pointer_focus`

Если под pointer несколько конкурирующих forms,
packet идет через `CHOOSE`.

Если target уже очевиден,
`CHOOSE` может быть тривиальным.

## Шаг 6. Нужен ли visual update?

Вот тут начинается самое полезное.

Движение мышки не всегда требует redraw.

Packet должен различить два случая.

### Случай A: redraw не нужен

Например:

- cursor аппаратный
- focus не изменился
- hover state не изменился
- сцена не изменилась

Тогда packet:

- обновляет pointer route
- фиксирует новое focus/target состояние
- не требует scene rebuild
- не требует `MANIFEST`

Итог:

- packet завершает внутренний переход
- визуального кадра не появляется

### Случай B: redraw нужен

Например:

- pointer вошел в другую form
- hover effect должен измениться
- cursor shape изменилась
- нужно подсветить элемент UI

Тогда packet дополняется:

- `state.manifest.manifest_path = reduced pointer update`
- `state.manifest.carrier_target = current carrier`
- `state.manifest.present_cadence = minimal needed`

И идет дальше к `MANIFEST`.

## Шаг 7. `MANIFEST`

Если visual update нужен,
packet доходит до `manifest`.

Там решается:

- делать ли full redraw
- делать ли partial update
- делать ли только cursor/overlay update
- нужен ли вообще scanout change

Для обычного mouse move чаще всего должно быть:

- не full scene redraw
- а минимальная manifestation

То есть packet проявляется ровно настолько,
насколько это действительно нужно.

## Шаг 8. `log_ledger`

По дороге packet пишет историю.

Например:

- `FLOW: pointer motion received`
- `CONNECT: seat bound to field`
- `OBSERVE: route stable`
- `ENCODE: target form resolved`
- `CHOOSE: trivial target selection`
- `MANIFEST: skipped`

или

- `MANIFEST: reduced pointer update`

То есть `log_ledger` здесь не про цену.
Он просто оставляет trace,
что с packet произошло.

## Что этот walkthrough проверяет

Он проверяет,
что spec умеет различать:

- input route
- form target
- visibility
- focus
- visual/no-visual outcome

И самое важное:

packet не обязан каждый раз заканчиваться redraw.

Он может:

- обновить внутреннюю truth
- и завершиться без лишнего графического шума

## Главный вывод

Даже простой `mouse move` показывает,
что packet в `X12`:

- рождается сверху как сырой импульс
- растет по слоям
- доходит до формы
- и только потом решает,
  нужен ли вообще visual manifest

Это хороший признак.

Значит `X12 Packet Spec v0` уже можно проверять на реальных сценариях,
а не только на общих тезисах.
