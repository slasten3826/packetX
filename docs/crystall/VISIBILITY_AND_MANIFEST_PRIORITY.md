# Видимость И Приоритет Манифестации

## Зачем этот документ

В `X12` нельзя держать слишком грубую модель:

- видно -> рисуем
- не видно -> не рисуем

Такая модель слишком примитивна.

Нужна более точная схема,
в которой различаются:

- жизнь процесса
- жизнь формы
- частота manifestation

## Главный тезис

То, что не видно, не обязано проявляться с полной частотой.

Но то, что не проявляется или проявляется слабо,
не обязано переставать существовать.

## Ключевое различие

### Процесс

Может продолжать жить:

- видео может продолжать декодироваться
- игра может продолжать simulation
- приложение может продолжать state updates
- аудио может продолжать играть

### Manifestation

При этом визуальное проявление может быть:

- полным
- ослабленным
- редким
- почти нулевым

## Следствие

`X12` не должен опираться на бинарный флаг `visible`.

`X12` должен опираться на:

- visibility state
- manifestation priority

## Visibility States

### `active_visible`

Форма:

- видима
- активна
- имеет высокий user-priority

Ожидаемое поведение:

- полный render / manifest rate
- нормальный input priority
- полноценная composition path

### `visible_background`

Форма:

- видима
- но не является главным active target

Ожидаемое поведение:

- пониженный manifest rate возможен
- render по необходимости
- reduced composition priority

### `partially_occluded`

Форма:

- видна частично
- часть ее площади реально перекрыта

Ожидаемое поведение:

- partial or reduced manifestation
- render rate может зависеть от реально видимой области

### `fully_occluded`

Форма:

- полностью перекрыта
- пользователю не видна

Ожидаемое поведение:

- manifestation может быть резко понижен
- render может перейти в keepalive/snapshot mode
- full-rate redraw не нужен

### `hidden`

Форма:

- выведена из пользовательской видимости
- не участвует в текущем visual field

Ожидаемое поведение:

- manifestation почти нулевой
- процесс может жить отдельно

### `exclusive_visible`

Форма:

- fullscreen / direct / kiosk-like
- является главным manifestation target

Ожидаемое поведение:

- максимальный manifest priority
- возможен direct path

## Manifestation Priority

Нужна не только классификация видимости,
но и шкала manifestation priority.

Черновая схема:

- `P3`
  full-rate manifest

- `P2`
  reduced-rate manifest

- `P1`
  keepalive / snapshot manifest

- `P0`
  process only, without active manifestation

## Пример

### Видео-плеер под другим окном

Если видео-плеер полностью закрыт другим окном,
это не значит:

- нужно убить процесс
- нужно убить декодирование

Но это может значить:

- не рендерить 60 fps
- не композить полную сцену
- перейти на `P1`
  - 1 fps
  - 0.5 fps
  - только snapshot update

Когда форма снова входит в видимость,
manifest priority поднимается обратно.

## Где это живет по слоям

### `OBSERVE`

Измеряет:

- visibility state
- степень occlusion
- active/inactive
- load pressure

### `CHOOSE`

Выбирает:

- manifestation priority
- какой form дать больше visual preference

### `ENCODE`

Фиксирует:

- устойчивый режим формы
- переход между visibility states

### `RUNTIME`

Удерживает:

- жизнь процесса
- continuity формы
- even when manifestation priority low

### `MANIFEST`

Дает:

- реальную частоту visual realization
- в соответствии с chosen priority

## Короткая формула

В `X12`:

- не все, что живет, обязано рисоваться с полной частотой
- не все, что не видно, обязано переставать существовать

## Промежуточный вывод

`X12` должен различать:

- process life
- form life
- manifestation rate

Иначе он либо повторит старую расточительность,
либо впадет в слишком грубую модель "видно / не видно".
