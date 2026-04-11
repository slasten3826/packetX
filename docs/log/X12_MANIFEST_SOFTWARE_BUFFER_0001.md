# X12 Manifest Software Buffer 0001

## Что сделано

В `X12` появился первый живой `manifest`
не только как текстовый snapshot,
но и как software output layer.

Добавлено:

- `ManifestBuffer`
- clipping для прямоугольников
- `blit_form`
- `get_pixel`
- software compositing pass

## Что это значит

Теперь `manifest` умеет:

- собрать итоговое изображение в памяти
- накладывать формы по painter's algorithm
- корректно обрезать формы по границам output buffer
- быть проверяемым по конкретным пикселям

То есть `manifest` уже не только говорит,
что формы существуют,
а реально проявляет их в buffer.

## Что проверено

Тестами проверено:

- размеры буфера
- clipping
- появление одной mapped формы
- overwrite верхней формы над нижней
- отсутствие unmapped формы в output buffer

## Почему это важно

Это первый живой шаг между:

- server truth
- и будущим physical output

То есть до `DRM/KMS` теперь уже есть
не пустота,
а software manifest stage.

## Что еще не сделано

Пока еще нет:

- damage model
- front/back buffers
- PPM dump
- real client pixel content
- `DRM/KMS`

Но compositing proof уже есть.
