import Aeneas
import AeneasTest.Code.Funs

open Aeneas Aeneas.Std Result ControlFlow Error

open AeneasTest

#setup_aeneas_simps

@[step]
theorem binary_search_spec (v : Slice U64) (k : U64)
  (h : v.len.val ≥ 1 ∧ (∀ (i: Nat) (j: Nat), (0 ≤ i ∧ i ≤ j ∧ j < v.len) → v[i]! ≤ v[j]!) ∧ (∃ (i: Nat), 0 ≤ i ∧ i ≤ v.len ∧ k = v[i]!)) :
  binary_search.binary_search v k ⦃ result => result < v.len ∧ k = v[result]! ⦄ := by
  unfold binary_search.binary_search
  step*
  zify
  simp_lists
  simp [*]
  grind


@[step]
theorem binary_search_loop_spec (v : Slice U64) (k : U64) (i1 : Usize) (i2 : Usize)
  (h : (i2 < v.len) ∧ (∀ (i: Nat) (j: Nat), (0 ≤ i ∧ i ≤ j ∧ j < v.len) → v[i]! ≤ v[j]!) ∧ (∃ (i: Nat), i1 ≤ i ∧ i ≤ i2 ∧ k = v[i]!)) :
  binary_search.binary_search_loop v k i1 i2 ⦃ result => (i2 < v.len) ∧ (∀ (i: Nat) (j: Nat), 0 ≤ i ∧ i ≤ j ∧ j < v.len → v[i]! ≤ v[j]!) ∧ (∃ (i: Nat), i1 ≤ i ∧ i ≤ i2 ∧ k = v[i]!) ⦄ := by
  unfold binary_search.binary_search_loop
  step*
  if base: i1 ≠ i2
  then
    simp [base]
    step*
    split_conjs
    · agrind
    · zify
      simp [*]
      grind
    · zify
      simp [*]
      simp_lists
      sorry
  else
    simp [base]
    step*
termination_by i2 - i1
decreasing_by
  zify
  simp [*]
  simp_lists
  scalar_decr_tac
