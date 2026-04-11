# X12 Manifest Engineering Cleanup 0001

## Что улучшено

После первого software manifest
сделан отдельный engineering cleanup pass.

Сделано:

- дефолтный software buffer уменьшен
  до debug-sized `256x144`
- `manifest` теперь композитит только
  `mapped && visible` формы
- убран `mem::replace` на каждый render
- `ManifestState` теперь рендерит прямо
  из `&[FormAssembly]`
- добавлен `dirty` flag
- если state не dirty,
  `manifest` пропускает полный diff

## Почему это важно

Это не меняет онтологию `manifest`.

Это делает ее дешевле и чище:

- меньше лишней работы
- меньше лишних аллокаций
- больше доверия к `crystall` как visibility filter

## Что теперь делает `manifest`

- берет только реально видимые формы
- рендерит их в back buffer
- считает coarse damage только когда dirty
- держит front/back continuity

## Что проверено

Тестами дополнительно проверено:

- fully occluded форма не композитится
- clean state не делает новый damage diff

## Текущий вывод

`manifest` теперь уже не просто живой,
а и заметно более инженерно-плотный.

Это хороший рубеж перед следующими шагами:

- `PPM dump`
- tile-based damage
- backing store / `PutImage`
- потом `DRM/KMS`
