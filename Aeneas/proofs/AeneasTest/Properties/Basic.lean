import Aeneas
import AeneasTest.Code.Funs

open Aeneas
open Aeneas.Std

open AeneasTest

#setup_aeneas_simps

@[step]
theorem add_spec (a b : U32) (h : a.val + b.val < U32.max) :
  add a b ⦃ c => c.val = a.val + b.val ⦄ := by
  unfold add
  step
  simp_all

@[step]
theorem octuple_spec (x1 : I8) (h : -16#i8 ≤ x1.val && x1.val < 16#i8) :
  octuple.octuple x1 ⦃ x8 => x8.val = x1.val * 8#i8 ⦄ := by
  unfold octuple.octuple
  step*
  simp_all
