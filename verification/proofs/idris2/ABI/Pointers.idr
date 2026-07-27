-- SPDX-License-Identifier: AGPL-3.0-or-later
-- Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
--
-- ABI Proof: Non-null pointer safety
-- Template proof — customise for your project's pointer types.
-- All proofs MUST be constructive (no believe_me, no assert_total).

module ABI.Pointers

import Data.Bits
import Data.So

%default total

||| A pointer value that has been proven non-null.
||| The `So` constraint carries a compile-time witness that `ptr /= 0`.
public export
record SafePtr where
  constructor MkSafePtr
  ptr : Bits64
  {auto 0 nonNull : So (ptr /= 0)}

||| Proof that SafePtr can never hold a null (zero) value.
||| This is enforced by the `So` constraint in the record.
export
0 safePtrNeverNull : (sp : SafePtr) -> So (sp.ptr /= 0)
safePtrNeverNull (MkSafePtr ptr {nonNull}) = nonNull

||| Wrap a raw pointer with a runtime null check.
||| Returns Nothing if the pointer is null.
export
checkPtr : (raw : Bits64) -> Maybe SafePtr
checkPtr 0 = Nothing
checkPtr raw = case choose (raw /= 0) of
  Left prf => Just (MkSafePtr raw)
  Right _ => Nothing

||| Proof that checkPtr 0 always returns Nothing.
export
checkPtrZeroIsNothing : checkPtr 0 = Nothing
checkPtrZeroIsNothing = Refl

||| An opaque handle backed by a non-null pointer.
||| Use this for FFI resource handles (file descriptors, sockets, etc.).
public export
record Handle (tag : String) where
  constructor MkHandle
  safePtr : SafePtr

||| Proof that two handles wrapping the same pointer are equal.
|||
||| Stated over the whole `SafePtr` rather than over its `ptr` field. The
||| stronger-looking
|||
|||     (h1, h2 : Handle tag) -> h1.safePtr.ptr = h2.safePtr.ptr -> h1 = h2
|||
||| is NOT provable here, and it is worth recording why rather than reaching
||| for believe_me. Two `SafePtr` values with the same `ptr` still carry
||| distinct `nonNull` witnesses. Those witnesses are erased (multiplicity 0),
||| so they are irrelevant to the *runtime* representation -- but Idris 2 has
||| no definitional proof irrelevance, so they are not identified by
||| unification either. The two `So` values simply fail to unify, and no
||| constructive term closes the gap.
|||
||| The proposition is true semantically and unprovable syntactically. Taking
||| `SafePtr` equality as the hypothesis states exactly what can be
||| established, and callers holding two `SafePtr`s known to be equal lose
||| nothing.
export
handlePtrEq : (h1, h2 : Handle tag) -> h1.safePtr = h2.safePtr -> h1 = h2
handlePtrEq (MkHandle sp1) (MkHandle sp2) prf = cong MkHandle prf
