import Aeneas
import AeneasTest.Code.Funs

open Aeneas Aeneas.Std Result ControlFlow Error

open AeneasTest

#setup_aeneas_simps

mutual

@[step]
theorem is_even_spec (n : U64) (h : true) :
  even_odd_mutual_rec.is_even n ⦃ result => result = (n.val % 2 == 0) ⦄ := by
  unfold even_odd_mutual_rec.is_even
  if base: n = 0#u64
  then
    simp [base]
  else
    simp [base]
    step*
    simp_all
    grind
termination_by n
decreasing_by scalar_decr_tac

@[step]
theorem is_odd_spec (n : U64) (h : true) :
  even_odd_mutual_rec.is_odd n ⦃ result => result = (n.val % 2 == 1) ⦄ := by
  unfold even_odd_mutual_rec.is_odd
  if base: n = 0#u64
  then
    simp [base]
  else
    simp [base]
    step*
    simp_all
    grind
termination_by n
decreasing_by scalar_decr_tac

end
