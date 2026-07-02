import Aeneas
import AeneasTest.Code.Funs

open Aeneas Aeneas.Std Result ControlFlow Error

open AeneasTest

#setup_aeneas_simps

@[step]
theorem assert_spec (b : Bool) (h : b) :
  assert.assert b ⦃ _ => b ⦄ := by
  unfold assert.assert
  step*

@[step]
theorem assert_main_spec  (h : b) :
  assert.main ⦃ _ => b ⦄ := by
  unfold assert.main
  step*
