# Chaos

Слой `chaos` описывает substrate и ограничения, а не графическую форму.

Сюда должны ложиться документы про:

- Linux graphics substrate
- DRM/KMS
- input ingestion
- memory/buffer constraints
- timing / vblank / hardware pressure

Сюда не должны ложиться:

- layout rules
- scene ownership
- финальная композиция

Если документ одновременно про substrate и final presentation,
его лучше пометить как boundary и положить либо сюда, либо в `manifest/`.
