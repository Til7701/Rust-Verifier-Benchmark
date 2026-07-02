import Aeneas
import AeneasTest.Code.Funs

open Aeneas Aeneas.Std Result ControlFlow Error

open AeneasTest

#setup_aeneas_simps

@[step]
theorem clone_spec (self : ternary_logic.Trit) :
  ternary_logic.Trit.Insts.CoreCloneClone.clone self ⦃ result => result = self ⦄ := by
  unfold ternary_logic.Trit.Insts.CoreCloneClone.clone
  step*

@[step]
theorem eq_spec (self : ternary_logic.Trit) (other: ternary_logic.Trit) :
  ternary_logic.Trit.Insts.CoreCmpPartialEqTrit.eq self other ⦃ result => result ↔ self.read_discriminant = other.read_discriminant ⦄ := by
  unfold ternary_logic.Trit.Insts.CoreCmpPartialEqTrit.eq
  step*

@[step]
theorem not_spec (self : ternary_logic.Trit) :
  ternary_logic.Trit.Insts.CoreOpsBitNotTrit.not self ⦃ result =>
    (self = ternary_logic.Trit.True → result = ternary_logic.Trit.False) ∧
    (self = ternary_logic.Trit.Maybe → result = ternary_logic.Trit.Maybe) ∧
    (self = ternary_logic.Trit.False → result = ternary_logic.Trit.True)
  ⦄ := by
  unfold ternary_logic.Trit.Insts.CoreOpsBitNotTrit.not
  step*

@[step]
theorem bitand_spec (self : ternary_logic.Trit) (other : ternary_logic.Trit) :
  ternary_logic.Trit.Insts.CoreOpsBitBitAndTritTrit.bitand self other ⦃ result =>
    (self = ternary_logic.Trit.True ∧ other = ternary_logic.Trit.True → result = ternary_logic.Trit.True) ∨
    (self = ternary_logic.Trit.False ∨ other = ternary_logic.Trit.False → result = ternary_logic.Trit.False) ∨
    result = ternary_logic.Trit.Maybe
  ⦄ := by
  unfold ternary_logic.Trit.Insts.CoreOpsBitBitAndTritTrit.bitand
  step*

@[step]
theorem bitor_spec (self : ternary_logic.Trit) (other : ternary_logic.Trit) :
  ternary_logic.Trit.Insts.CoreOpsBitBitOrTritTrit.bitor self other ⦃ result =>
    (self = ternary_logic.Trit.True ∨ other = ternary_logic.Trit.True → result = ternary_logic.Trit.True) ∨
    (self = ternary_logic.Trit.False ∧ other = ternary_logic.Trit.False → result = ternary_logic.Trit.False) ∨
    result = ternary_logic.Trit.Maybe
  ⦄ := by
  unfold ternary_logic.Trit.Insts.CoreOpsBitBitOrTritTrit.bitor
  step*

@[step]
theorem imp_spec (self : ternary_logic.Trit) (other : ternary_logic.Trit) :
  ternary_logic.Trit.imp self other ⦃ result =>
    (self = ternary_logic.Trit.True → result = other) ∧
    (self = ternary_logic.Trit.Maybe ∧ other = ternary_logic.Trit.True → result = ternary_logic.Trit.True) ∧
    (self = ternary_logic.Trit.Maybe ∧ other ≠ ternary_logic.Trit.True → result = ternary_logic.Trit.Maybe) ∧
    (self = ternary_logic.Trit.False → result = ternary_logic.Trit.True)
  ⦄ := by
  unfold ternary_logic.Trit.imp
  step*

@[step]
theorem eqv_spec (self : ternary_logic.Trit) (other : ternary_logic.Trit) :
  ternary_logic.Trit.eqv self other ⦃ result =>
    (self = ternary_logic.Trit.True → result = other) ∧
    (self = ternary_logic.Trit.Maybe → result = ternary_logic.Trit.Maybe) ∧
    (self = ternary_logic.Trit.False → ok result = ternary_logic.Trit.Insts.CoreOpsBitNotTrit.not other)
  ⦄ := by
  unfold ternary_logic.Trit.eqv
  unfold ternary_logic.Trit.Insts.CoreOpsBitNotTrit.not
  step*

@[step]
theorem ternary_logic_main_spec :
  ternary_logic.main ⦃ _ => true⦄ := by
  unfold ternary_logic.main
  unfold ternary_logic.Trit.Insts.CoreOpsBitNotTrit.not
  unfold ternary_logic.Trit.Insts.CoreOpsBitBitAndTritTrit.bitand
  unfold ternary_logic.Trit.Insts.CoreOpsBitBitOrTritTrit.bitor
  unfold ternary_logic.Trit.imp
  unfold ternary_logic.Trit.eqv
  step*
