import Aeneas
import AeneasTest.Code.Funs

open Aeneas Aeneas.Std Result ControlFlow Error

open AeneasTest

#setup_aeneas_simps

@[step]
theorem octuple_spec (x1 : I8) (h : -16 ≤ x1.val ∧ x1.val < 16) :
  octuple.octuple x1 ⦃ x8 => x8.val = x1.val * 8 ⦄ := by
  unfold octuple.octuple
  step*
