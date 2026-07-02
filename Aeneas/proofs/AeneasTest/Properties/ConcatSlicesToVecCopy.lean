import Aeneas
import AeneasTest.Code.Funs

open Aeneas Aeneas.Std Result ControlFlow Error

open AeneasTest

#setup_aeneas_simps

-- @[step]
-- theorem concat_slices_to_vec_std_spec {T : Type} (coremarkerCopyInst : core.marker.Copy T) (x : Slice T)
--   (y : Slice T) (h : x.len.val + y.len.val <= Usize.max) :
--   concat_slices_to_vec_copy.concat_slices_to_vec_copy coremarkerCopyInst x y ⦃ result => result = x + y ⦄ := by
--   unfold concat_slices_to_vec_copy.concat_slices_to_vec_copy
--   step*

@[step]
theorem concat_slices_to_vec_copy_loop0_spec {T : Type} (iter : core.ops.range.Range Std.Usize) (x : Slice T)
  (concat : alloc.vec.Vec T) (h : concat.len + x.len ≤ Usize.max) :
  concat_slices_to_vec_copy.concat_slices_to_vec_copy_loop0 iter x concat ⦃ result => ∀ (i: Nat), (0 ≤ i ∧ i < iter.start.val) ∧ (x[i]? = result[i]?) ⦄ := by
  unfold concat_slices_to_vec_copy.concat_slices_to_vec_copy_loop0
  -- unfold core.iter.range.IteratorRange.next
  -- unfold Slice.index_usize
  -- unfold alloc.vec.Vec.push
  step*
  simp_lists
  simp_all
  agrind
termination_by iter.end - iter.start
decreasing_by scalar_decr_tac

-- @[step]
-- theorem concat_slices_to_vec_copy_loop1_spec {T : Type} (iter : core.ops.range.Range Std.Usize) (y : Slice T)
--   (concat : alloc.vec.Vec T) (h : y.len < Usize.max) :
--   concat_slices_to_vec_copy.concat_slices_to_vec_copy_loop1 iter y concat ⦃ result => concat = y ⦄ := by
--   unfold concat_slices_to_vec_copy.concat_slices_to_vec_copy_loop1
--   step*
--   agrind
-- termination_by Usize.max - concat.len
-- decreasing_by scalar_decr_tac
