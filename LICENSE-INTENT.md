# Licensing

Chronicles of Slavia is layered, and each layer is licensed for what it is.

| Layer | Licence |
|-------|---------|
| **Engine & code** | [AGPL-3.0-or-later](https://www.gnu.org/licenses/agpl-3.0.html) |
| **Content** (design notes, lore, writing, worldbuilding, art) | [CC-BY-SA-4.0](https://creativecommons.org/licenses/by-sa/4.0/) |
| **Names & marks** (Chronicles of Slavia, Anya, Donna) | Reserved |

The engine is strong copyleft by design: anyone who builds on it, including as
a hosted service, must share their entire derivative source. The content is free
culture — use it, modify it, sell derivatives, provided you attribute and keep
them under the same share-alike terms.

## MPL-2.0 components

Some incorporated components carry [MPL-2.0](https://www.mozilla.org/MPL/2.0/)
and keep it. That is not an exception to the above: MPL-2.0 §3.3 explicitly
permits distributing covered software as part of a Larger Work under a Secondary
License, and names AGPL 3.0 as one of them. Incorporating MPL components into an
AGPL product is the intended arrangement, not a special case.

Licence texts for all three are vendored under `LICENSES/`.

See [ADR-0008](docs/decisions/0008-licence-position.adoc) for the decision and
the metadata corrections that follow from it.

---

**This file previously stated that no licence had been granted and that all
rights were reserved.** That was stale, and it contradicted the SPDX headers
present in every source file. Corrected here rather than left to be discovered.
