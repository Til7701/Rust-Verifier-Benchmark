import Aeneas
import AeneasTest.Code.Funs

open Aeneas Aeneas.Std Result ControlFlow Error

open AeneasTest

#setup_aeneas_simps

@[step]
theorem triangle_rec_spec (n : U64) (h : (n.val * (n.val + 1)) / 2 ≤ U64.max) :
  triangle_rec.triangle_rec n ⦃ result => result = (n.val * (n.val + 1)) / 2 ⦄ := by
  unfold triangle_rec.triangle_rec
  if base: n = 0#u64
  then
    simp [base]
  else
    simp [base]
    step*
    · zify
      simp [*]
      agrind
    · zify
      simp [*]
      agrind
    · simp [*]
      zify
      simp [*]
      agrind
termination_by n
decreasing_by scalar_decr_tac
