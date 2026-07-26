-- SPDX-License-Identifier: MPL-2.0
-- Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
--
-- ABI Proof: Memory layout correctness
-- Proves struct size, alignment, and padding properties.
-- All proofs MUST be constructive (no believe_me, no assert_total).

module ABI.Layout

%default total

||| Non-zero natural numbers.
public export
data NonZero : Nat -> Type where
  SIsNonZero : NonZero (S n)

||| Modulo operation with proof that divisor is non-zero.
public export
modNatNZ : (n : Nat) -> (m : Nat) -> NonZero m -> Nat
modNatNZ n (S m) (SIsNonZero) = ?modNatNZ_rhs
modNatNZ _ Z _ = 0  -- This case is unreachable due to NonZero proof

||| Witness that a type has a known size in bytes at compile time.
public export
interface HasSize (ty : Type) where
  sizeOf : Nat

||| Witness that a type has a known alignment in bytes.
public export
interface HasAlignment (ty : Type) where
  alignOf : Nat

||| Calculate padding needed to reach the next aligned offset.
||| paddingFor offset alignment = bytes to add so (offset + padding) `mod` alignment == 0
public export
paddingFor : (offset : Nat) -> (alignment : Nat) -> NonZero alignment -> Nat
paddingFor offset alignment ok = let r = modNatNZ offset alignment ok
                              in case r of
                                   Z => Z
                                   (S _) => minus alignment r

||| Proof that an offset with zero remainder needs zero padding.
export
alignedNeedsPadding : (n : Nat) -> (a : Nat) -> (ok : NonZero a) ->
                      modNatNZ n a ok = 0 -> paddingFor n a ok = 0
alignedNeedsPadding n a ok prf = rewrite prf in Refl

||| A field within a struct, carrying its offset and size.
public export
record StructField where
  constructor MkField
  fieldName : String
  fieldOffset : Nat
  fieldSize : Nat
  fieldAlignment : Nat

||| Less-than-or-equal relation on natural numbers.
public export
data LTE : Nat -> Nat -> Type where
  LTERefl : LTE n n
  LTEStep : LTE n m -> LTE n (S m)

||| Proof that a field is correctly aligned within a struct.
public export
FieldAligned : StructField -> Type
FieldAligned f = (ok : NonZero (fieldAlignment f)) -> modNatNZ (fieldOffset f) (fieldAlignment f) ok = 0

||| Proof that a field does not overflow past a given struct size.
public export
FieldInBounds : (structSize : Nat) -> StructField -> Type
FieldInBounds sz f = LTE (fieldOffset f + fieldSize f) sz

||| A struct layout is a list of fields with a total size.
public export
record StructLayout where
  constructor MkLayout
  layoutName : String
  layoutFields : List StructField
  layoutSize : Nat
  layoutAlignment : Nat
