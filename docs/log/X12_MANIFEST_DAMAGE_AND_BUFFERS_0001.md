# X12 Manifest Damage And Buffers 0001

## Что сделано

`manifest` в `X12` теперь умеет не только
software compositing,
но и первый buffer lifecycle:

- `front` buffer
- `back` buffer
- coarse damage detection

## Что это значит

Теперь `manifest` уже умеет:

- собрать новое изображение в `back`
- сравнить его с предыдущим `front`
- вычислить грубый `damage rect`
- поменять `front/back`

То есть output layer стал уже не просто
"нарисуй картинку в памяти",
а первым живым состоянием output continuity.

## Какой damage сейчас

Пока damage грубый:

- сравнение `front` и `back`
- один bounding rectangle по всем changed pixels

Это не финальная damage model,
но уже рабочий `v0`.

## Что проверено

Тестами проверено:

- damage возникает при первом map формы
- после `UnmapWindow` front buffer
  возвращается к background
- output buffer реально отражает
  последнее server truth состояние

## Почему это важно

Теперь `manifest` уже проходит
следующую ступень:

- server truth
- compositing
- output buffers
- damage

То есть между `X12` и будущим `DRM/KMS`
теперь уже есть не пустое место,
а первый живой output backend в памяти.

## Что еще не сделано

Пока еще нет:

- tile-based flow
- PPM dump
- real client backing store
- `Expose`
- `PutImage`
- physical `DRM/KMS` backend

Но step 2 уже дает достаточно материала,
чтобы обсуждать `manifest` снаружи
не только по ontology,
а по живому коду.
