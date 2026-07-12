import Aeneas
import AeneasTest.Code.Funs

open Aeneas Aeneas.Std Result ControlFlow Error

open AeneasTest

#setup_aeneas_simps

@[step]
theorem concat_slices_to_vec_mix_loop_spec {T : Type} [Inhabited T] (x : Slice T) (i: Usize)
  (concat : alloc.vec.Vec T) (h :
    ∃ prefixx, concat.val = prefixx ++ x.val.take i ∧
    concat.len + x.len - i ≤ Usize.max ∧ i ≤ x.len
  ) :
  concat_slices_to_vec_mix.concat_slices_to_vec_mix_loop x concat i ⦃ result =>
    result = concat.val ++ x.val.drop i
  ⦄ := by
  unfold concat_slices_to_vec_mix.concat_slices_to_vec_mix_loop
  if base: i < x.len
  then
    step*
    agrind
    zify
    simp [*]
    split_conjs
    · simp [List.take_add_one]
      grind
    · agrind
    · agrind
    · simp [List.drop_eq_getElem_cons base]
      grind
  else
    step*
termination_by x.len.val - i
decreasing_by scalar_decr_tac

@[step]
theorem concat_slices_to_vec_mix_spec {T : Type} [Inhabited T] (copyInst: core.marker.Copy T) (x : Slice T) (y: Slice T)
  (h :
    (∀ e ∈ x.val, copyInst.cloneInst.clone e = ok e) ∧
    x.len + y.len ≤ Usize.max
  ) :
  concat_slices_to_vec_mix.concat_slices_to_vec_mix copyInst x y ⦃ result =>
    result.val = x.val ++ y.val
  ⦄ := by
  unfold concat_slices_to_vec_mix.concat_slices_to_vec_mix
  step*
  simp [*]
  · agrind
  · simp [*]
