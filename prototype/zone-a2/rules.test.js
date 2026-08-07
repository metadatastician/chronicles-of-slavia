// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>
//
// rules.test.js — canon conformance test for the A2 interaction mock.
//
//   Run:  bun prototype/zone-a2/rules.test.js
//
// A mock that nobody checks is just a drawing. This drives the mock's ACTUAL
// rule functions headlessly (canvas stubbed) and asserts they match the design
// canon they claim to implement — including the negative constraints, which are
// the easy ones to violate by accident.
//
// Canon under test:
//   docs/design/03-living-taxis.md          — the creature's own nature decides
//   docs/design/16-flora-and-non-animal-npcs.md — nature tiers; harvest types
//   docs/design/23-level-scope-and-pacing.md    — A2 scope; honey/yogurt; NO plant taxis

const HTML = new URL('./index.html', import.meta.url).pathname;
const src = await Bun.file(HTML).text();
const js = src.match(/<script>([\s\S]*)<\/script>/)[1];

// --- minimal browser stubs: enough to let the module initialise, no DOM ------
const ctxProxy = new Proxy({}, { get: () => (...a) => ({ addColorStop() {} }) });
globalThis.document = {
  getElementById: () => ({ getContext: () => ctxProxy, style: {}, width: 1280, height: 720 }),
};
globalThis.addEventListener = () => {};
globalThis.requestAnimationFrame = () => {};
globalThis.performance = { now: () => 0 };
globalThis.window = { innerWidth: 1280, innerHeight: 720 };
globalThis.innerWidth = 1280;
globalThis.innerHeight = 720;

let pass = 0, fail = 0;
const t = (name, cond) => {
  if (cond) { pass++; console.log('  ok   ' + name); }
  else      { fail++; console.log('  FAIL ' + name); }
};

// Evaluate the mock in this scope, then expose the bits we assert on.
const api = new Function(`${js}
return { state:()=>state, reset, reachOut, gather, update, WORLD, W };`)();

const st = api.state;
const F = k => st().fauna.find(f => f.kind === k);
const S = k => st().sites.find(s => s.kind === k);

console.log("\n-- the creature's own nature decides (doc 03) --");
api.reset(); api.reachOut(st().anya,  F('hare'));
t('hare stirred -> bolts',   F('hare').mood === 'bolted');
api.reset(); api.reachOut(st().donna, F('hare'));
t('hare settled -> freezes', F('hare').mood === 'frozen');
api.reset(); api.reachOut(st().anya,  F('bird'));
t('bird stirred -> stirred', F('bird').mood === 'stirred');
api.reset(); api.reachOut(st().donna, F('bird'));
t('bird settled -> settled', F('bird').mood === 'settled');

console.log('\n-- nature tiers: the failure is the diagnostic (doc 16) --');
api.reset(); api.reachOut(st().anya,  F('goat'));
t('domesticated stirred -> skittish', F('goat').mood === 'skittish');
api.reset(); api.reachOut(st().donna, F('goat'));
t('domesticated settled -> trusting', F('goat').mood === 'trusting');
api.reset(); api.reachOut(st().anya,  F('magpie'));
t('blighted ignores stir   -> unmoved', F('magpie').mood === 'unmoved');
api.reset(); api.reachOut(st().donna, F('magpie'));
t('blighted ignores settle -> unmoved', F('magpie').mood === 'unmoved');

console.log('\n-- co-op grammar gates the wild item --');
api.reset();
api.gather(st().anya, S('honey'));
t('honey refused while the hive is up', st().pouch.honey === false);
api.reachOut(st().anya, F('bees'));            // Anya can only stir — makes it worse
api.gather(st().anya, S('honey'));
t('Anya cannot settle the hive herself', st().pouch.honey === false);
api.reachOut(st().donna, F('bees'));
t('Donna settles the hive', st().beesSettled === true);
api.gather(st().donna, S('honey'));
t("honey is not Donna's to take", st().pouch.honey === false);
api.gather(st().anya, S('honey'));
t('Anya takes the honey once settled', st().pouch.honey === true);

console.log('\n-- signature restoratives: found vs made (doc 23) --');
api.reset();
api.gather(st().anya,  S('yogurt'));
t('Anya cannot make yogurt',  st().pouch.yogurt === false);
api.gather(st().donna, S('yogurt'));
t('Donna makes the yogurt',   st().pouch.yogurt === true);

console.log('\n-- plants are functional only; food helps OR harms (doc 16) --');
api.reset();
const good = st().sites.find(s => s.kind === 'food' && s.good);
const bad  = st().sites.find(s => s.kind === 'food' && !s.good);
api.gather(st().anya, good); t('edible food gathered',  st().pouch.food === 1);
api.gather(st().anya, bad);  t('poisonous food harms',  st().pouch.poisoned === 1);
api.gather(st().anya, S('craft'));   t('craft fibre gathered', st().pouch.craft === 1);
api.gather(st().anya, S('pattern')); t('pattern dye gathered', st().pouch.pattern === 1);

console.log('\n-- SCOPE GUARD: no plant answers taxis; no tropism (doc 23) --');
t('no plant appears in the fauna list',
  st().fauna.every(f => !['food', 'craft', 'pattern'].includes(f.kind)));
t('no tropism/vine vocabulary in the rule functions',
  !/tropism|grow a vine|thorn-wall|retract/i.test(api.reachOut.toString() + api.gather.toString()));

console.log('\n-- camera scrolls a world wider than the viewport (engine gap) --');
api.reset();
st().anya.x = 2000; api.update(0.016);
t('world is wider than the viewport', api.WORLD > api.W);
t('camera moved off origin',          st().cam > 0);

console.log(`\n${pass} passed, ${fail} failed`);
if (fail) process.exit(1);
