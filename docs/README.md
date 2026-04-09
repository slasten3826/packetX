# Документация X12

Документация `X12` разложена по уровням абстракции.

Это сделано специально, чтобы:

- не смешивать substrate, relation, form и final appearance
- не превращать проект в плоскую свалку заметок
- проверять архитектуру не только кодом, но и структурой документации

## Уровни

### [chaos](/home/slasten/dev/x12/docs/chaos)

Нижний слой:

- Linux kernel
- DRM/KMS
- GPU/VRAM
- input hardware
- timing / vblank / raw limits

### [table](/home/slasten/dev/x12/docs/table)

Слой отношений, адресации и coordination:

- protocol truth
- object model
- capability routing
- bridges / translators
- topology and lifecycle

### [crystall](/home/slasten/dev/x12/docs/crystall)

Слой устойчивой графической формы:

- field
- form
- placement law
- window management
- shell / workspace organization

### [manifest](/home/slasten/dev/x12/docs/manifest)

Слой финального явления:

- composition
- fullscreen path
- direct presentation
- scanout contracts
- final user-visible scene

## Сквозные разделы

### [log](/home/slasten/dev/x12/docs/log)

Сквозной журнал хода мысли:

- техлог
- легенда
- переходные матрицы и промежуточные гипотезы

## Правило

Если новая заметка не может быть уверенно положена в один слой,
надо либо:

- положить ее в `log/` как переходную,
- либо явно пометить как boundary-документ.
