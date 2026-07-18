import Aeneas
import AeneasTest.Code.Funs

open Aeneas Aeneas.Std Result ControlFlow Error

open AeneasTest

#setup_aeneas_simps

@[step]
theorem get_mut_fst_spec :
  get_mut_fst.get_mut_fst (a, b) ⦃ result => result = (a, fun a' => (a', b)) ⦄ := by
  unfold get_mut_fst.get_mut_fst
  step*


@[step]
theorem get_mut_fst_main_spec :
  get_mut_fst.main ⦃ result => result = (100#i32, 20#i32) ⦄ := by
  unfold get_mut_fst.main
  step*
  simp [*]
