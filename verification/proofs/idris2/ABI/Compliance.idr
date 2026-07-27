-- SPDX-License-Identifier: AGPL-3.0-or-later
-- Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
--
-- ABI Proof: C ABI compliance
-- Proves that struct layouts are C ABI compliant.
-- All proofs MUST be constructive (no believe_me, no assert_total).

module ABI.Compliance

import ABI.Layout
import ABI.Platform

%default total

||| Evidence that every field in a layout is correctly aligned.
public export
data AllFieldsAligned : List StructField -> Type where
  AFANil  : AllFieldsAligned []
  AFACons : FieldAligned f -> AllFieldsAligned fs -> AllFieldsAligned (f :: fs)

||| Evidence that every field is within the struct bounds.
public export
data AllFieldsInBounds : (size : Nat) -> List StructField -> Type where
  AFBNil  : AllFieldsInBounds size []
  AFBCons : FieldInBounds size f -> AllFieldsInBounds size fs -> AllFieldsInBounds size (f :: fs)

||| A struct layout is C ABI compliant when:
||| 1. All fields are aligned to their natural alignment
||| 2. All fields are within bounds of the struct size
||| 3. The struct size is a multiple of the struct alignment
public export
record CABICompliant (layout : StructLayout) where
  constructor MkCompliant
  fieldsAligned  : AllFieldsAligned (layoutFields layout)
  fieldsInBounds : AllFieldsInBounds (layoutSize layout) (layoutFields layout)
  sizeAligned    : {ok : NonZero (layoutAlignment layout)} -> modNatNZ (layoutSize layout) (layoutAlignment layout) ok = 0

||| An empty struct is trivially compliant (size=1, alignment=1).
export
emptyStructCompliant : CABICompliant (MkLayout "empty" [] 1 1)
emptyStructCompliant = MkCompliant AFANil AFBNil emptySizeAligned
  where
    -- `ok` must be matched, not left abstract: modNatNZ pattern-matches on the
    -- NonZero witness, so with `ok` opaque the term does not reduce and Refl
    -- cannot typecheck. NonZero 1 has the single inhabitant SIsNonZero, so the
    -- match is total.
    --
    -- With ok = SIsNonZero: modNatNZ 1 1 SIsNonZero reduces through mod' 1 1 0
    -- -> (1 <= 0 is False) -> mod' 0 (minus 1 1) 0 -> mod' 0 0 0 -> 0.
    emptySizeAligned : {ok : NonZero 1} -> modNatNZ 1 1 ok = 0
    emptySizeAligned {ok = SIsNonZero} = Refl
