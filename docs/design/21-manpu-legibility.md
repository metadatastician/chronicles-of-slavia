# Manpu — reading the cognitive state

The legibility layer for `20-cognitive-npcs-and-theory-of-mind.md`.
Architecture: `docs/decisions/0004-hybrid-fsm-esm-mind-body-architecture.adoc`.

## Why this exists

**An NPC that reasons invisibly is indistinguishable from an NPC that cheats.**

The ESM gives NPCs private, partial, fallible beliefs. If the player cannot see
*why* a guard covered a bottleneck, a carefully-argued relational deduction reads
as arbitrary — or worse, as the game reading their inputs. Every ounce of
sophistication in the mind is wasted unless it is *legible* from outside.

So this is **not decoration. It is the ESM's user interface.**

## The mechanism

Classic manga/anime **Manpu** (漫画符号) overlays, drawn directly over the
character sprites: a compressed, instantly-readable symbolic vocabulary for
interior states.

### The "Speak Your Mind" toggle

| Setting | What the player gets |
|---|---|
| **ON** | Manpu overlays **and** explicit text bubbles spelling out the deduction — *"Wth?! It's gone?!"* |
| **OFF** | No text. The entire social logic — suspicion, confusion, panic — is read purely through **manpu icons, voice groans, and physical posture**. |

The OFF mode is the design's real ambition, and it is a strong claim: that the
cognitive model is *rich enough and consistent enough* to be understood without
words. ON is the tutorial and the accessibility path; OFF is the game the design
is actually arguing for.

Architecturally this is trivial, which is the point: the toggle is **view
config**. It changes nothing about what NPCs know or do.

## The iconography, bound to ESM state

Each symbol is a **read of a specific machine state** — not a mood the animator
picked. This binding is the contract:

| Manpu | Reads as | ESM/FSM state behind it |
|---|---|---|
| **Squiggly vertical line / swirl** | Perplexed, searching | Solver returned **multiple** solutions — *Ambiguous* (`20`, §A). He genuinely doesn't know which target you want. |
| **Giant blue sweat drop / face shading** | Panic, grief | A belief was violated by observation — the key object is *gone*, the comrade is *injured*. Often the `failo`/**contradiction** case. |
| **Popping vein** | Extreme frustration | Suspicion at ceiling. Mechanically: **bribes will not work.** |
| **Zzz bubble** | Complete comfort, sleeping | Detection thresholds — auditory *and* visual — are **lowered**. The sleeping sheepdog. |

### A precision worth keeping

The Zzz bubble looks like an exception — it has a *mechanical* effect (lowered
detection), so is it really "just a view"? No, and the distinction matters:
**the lowered threshold is the state; the Zzz is how you read it.** The dog is
not sleepy *because* of the bubble. Manpu stays a pure read of L2/L3 state, with
no authority over it. If a symbol ever starts *causing* something, the layer has
been breached.

## Where this composes beautifully

The Zzz case is not a new system — it is **living taxis** (`03-living-taxis.md`)
arriving at a stealth mechanic on its own:

> Donna tends to calm, slow, settle, quieten, or reassure natural animals.

So: **Donna settles the sheepdog → it sleeps → Zzz → detection thresholds drop →
the way past opens.** And Anya, inevitably, does the reverse: she stirs it awake
and the route closes. Nobody designed a "stealth ability". It falls out of the
personality axis that has been canon since the beginning, made visible by a
symbol. That is what a good layer boundary buys you.

It also honours the canon rule that the girls **do not command animals**
(`03`) — Donna doesn't order the dog to sleep. She changes the rhythm around it,
and it sleeps because that is what a calm dog does.

## Open question — is Japanese visual grammar right for a Slavic game?

Manpu is a **Japanese** symbolic vocabulary. Slavia is built on Ukrainian and
Bulgarian folkloric visual language, and the game *already has* a symbolic system
of its own: embroidery and pattern-weaving, explicitly established as **a social
and cultural language** (`17-clothing-repair-and-pattern-weaving.md`).

That's a real tension, and possibly a real opportunity:

1. **Manpu as-is.** It is legible to a huge audience precisely *because* it is a
   borrowed, well-known grammar. Reads as a deliberate stylistic fusion.
2. **Embroidery-as-manpu.** Derive the symbol set from Slavic motif vocabulary
   instead — the interior state surfaces as *pattern*. This rhymes with the
   Parabasis idea in `19-chronicle-detail-imported.md`, where HUD embroidery
   lines become physical ledges: if the embroidery is already the UI, it can
   already be the emotional read.
3. **Both.** Manpu shapes carrying motif styling.

Not resolved here. Option 2 is the most *this game*, and the most expensive —
it means inventing and teaching a symbol set rather than borrowing a fluent one,
which cuts directly against the OFF-mode ambition of being understood without
words. Owner decision.
