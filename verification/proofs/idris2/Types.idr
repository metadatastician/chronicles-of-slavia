-- SPDX-License-Identifier: MPL-2.0
-- Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
--
-- Typing Proof: Core data type well-formedness
-- Template — replace with your project's core types.
-- All proofs MUST be constructive (no believe_me, no assert_total).

module Types

%default total

||| Less-than-or-equal relation on natural numbers.
public export
data LTE : Nat -> Nat -> Type where
  LTERefl : LTE n n
  LTEStep : LTE n m -> LTE n (S m)

||| Example: A bounded natural number (0 to n).
||| Replace with your project's core types.
public export
record Bounded (n : Nat) where
  constructor MkBounded
  value : Nat
  inBounds : LTE value n

||| Proof that a Bounded value is always <= n.
export
boundedLeMax : (b : Bounded n) -> LTE b.value n
boundedLeMax (MkBounded _ prf) = prf

||| Proof that zero is always a valid Bounded value.
export
zeroIsBounded : {n : Nat} -> Bounded (S n)
zeroIsBounded = MkBounded 0 ?zeroIsBounded_prf

||| Example: A non-empty list with a compile-time guarantee.
public export
data NonEmpty : List a -> Type where
  IsNonEmpty : NonEmpty (x :: xs)

||| Proof that cons always produces a non-empty list.
export
consIsNonEmpty : (x : a) -> (xs : List a) -> NonEmpty (x :: xs)
consIsNonEmpty _ _ = IsNonEmpty
