# X12 PPM Visual Testing v0

## Зачем этот документ

Этот документ фиксирует
следующий живой шаг для `X12`:

не `DRM/KMS`,
не полноценный display backend,
а **визуальная проверка manifest через PPM dump**.

Цель:

- увидеть результат `manifest` глазами
- не лезть пока в hardware stack
- проверять compositing и damage
  на реальных артефактах,
  а не только на текстовых snapshot'ах

## Почему именно PPM

`PPM` подходит для `v0`,
потому что это:

- очень простой image format
- не требует внешних библиотек
- легко писать прямо из `ManifestBuffer`
- удобно открывать глазами
- удобно конвертировать в `png`
  если понадобится

Для `X12` это первый честный
`manifest artifact`.

Не “сервер сказал что картинка есть”,
а:

**вот файл, который реально собран
из текущего software manifest buffer.**

## Что именно тестируем через PPM

Через `PPM dump` мы хотим проверить:

- форма реально появляется после `MapWindow`
- overlap реально выглядит правильно
- верхняя форма затирает нижнюю
- `UnmapWindow` реально очищает output
- `cleanup_session` реально убирает форму
- `ConfigureWindow` реально двигает форму
- damage lifecycle не врет

То есть PPM нужен
не как “экспорт картинки ради красоты”,
а как:

**visual proof of current manifest truth**

## Что НЕ делаем на этом шаге

На этом шаге мы НЕ делаем:

- `DRM/KMS`
- page flip
- vsync
- real monitor output
- `PutImage`
- real client backing store
- alpha blending
- tile-based damage

Это важно.

`PPM dump` нужен,
чтобы проверить уже существующий
software manifest,
а не чтобы преждевременно
превратить его в hardware backend.

## Минимальный кодовый результат

Нужен один узкий путь:

- взять текущий `front` buffer из `ManifestState`
- сериализовать его в `PPM`
- записать файл на диск

Формат:

- binary `P6`
- header:
  - `P6`
  - `width height`
  - `255`
- потом raw `RGB` bytes

Alpha на этом шаге
можно просто отбросить.

## Где это должно жить

Минимально:

- в `src/manifest.rs`
  или рядом с ним

Например:

- `dump_front_buffer_ppm(path)`
- `write_ppm(buffer, path)`

Без разрастания в отдельный backend.

## Какой workflow тестирования нужен

### 1. Сценарий

Берем уже существующий scenario
или request sequence:

- `CreateWindow`
- `MapWindow`
- `ConfigureWindow`
- `UnmapWindow`

### 2. После каждого шага

Сохраняем:

- `frame-000.ppm`
- `frame-001.ppm`
- `frame-002.ppm`

### 3. Смотрим глазами

Проверяем:

- где форма появилась
- где исчезла
- правильно ли выглядит overlap

## Какой минимальный набор PPM сценариев нужен

### `single-map`

Проверяет:

- одна форма появилась в output

### `partial-overlap`

Проверяет:

- верхняя форма перекрывает часть нижней

### `full-occlusion`

Проверяет:

- нижняя форма полностью исчезает из output

### `unmap-restore`

Проверяет:

- после `UnmapWindow` верхней формы
  нижняя снова становится видимой

### `cleanup-session`

Проверяет:

- disconnect клиента реально убирает
  его форму из manifest output

## Что считается success

Успех на этом шаге:

- `PPM` файл пишется стабильно
- картинка соответствует текущему
  `ManifestBuffer`
- визуально видно,
  что compositing и cleanup
  делают именно то,
  что уже говорят тесты

То есть success здесь:

**text snapshot, tests и visual artifact
не противоречат друг другу**

## Что делать после этого шага

После `PPM dump` уже можно
намного увереннее решать:

- нужен ли tile-based damage раньше,
  чем `DRM/KMS`
- как должен выглядеть
  первый output backend
- где именно `manifest` еще врет,
  если visual proof разойдется
  с логикой snapshot'ов

## Коротко

Следующий живой шаг:

**не экран,
а PPM-артефакт текущего manifest buffer.**

Это первый визуальный pressure-test
для `X12 manifest`
без преждевременного прыжка
в hardware output.
