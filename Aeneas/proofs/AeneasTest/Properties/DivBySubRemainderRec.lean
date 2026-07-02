import Aeneas
import AeneasTest.Code.Funs

open Aeneas Aeneas.Std Result ControlFlow Error

open AeneasTest

#setup_aeneas_simps


@[step]
theorem div_by_sub_remainder_rec_spec (n : U64) (d : U64) (h : d ≥ 1#u64) :
  div_by_sub_remainder_rec.remainder_rec n d ⦃ result => (n = result.fst.val * d + result.snd.val) ∧ (result.fst <= n) ∧ (result.snd < d) ⦄ := by
  unfold div_by_sub_remainder_rec.remainder_rec
  if base: n < d
  then
    simp [base]
    step*
  else
    simp [base]
    step*
    --have a : ((n - d = result.fst.val * d + result.snd.val) → (n = (result.fst.val + 1) * d + result.snd.val)) := by
      --agrind
    simp_all
    --simp [*]
    agrind
termination_by n
decreasing_by scalar_decr_tac
