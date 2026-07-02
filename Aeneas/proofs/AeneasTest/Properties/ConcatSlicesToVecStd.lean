import Aeneas
import AeneasTest.Code.Funs

open Aeneas Aeneas.Std Result ControlFlow Error

open AeneasTest

#setup_aeneas_simps

@[step]
theorem concat_slices_to_vec_std_spec {T : Type} (corecloneCloneInst : core.clone.Clone T) (x : Slice T) (y : Slice T) (h : x.len.val + y.len.val <= USize.max) :
  concat_slices_to_vec_std.concat_slices_to_vec_std corecloneCloneInst x y ⦃ result => true ⦄ := by
  unfold concat_slices_to_vec_std.concat_slices_to_vec_std
  unfold alloc.slice.Slice.to_vec
  unfold alloc.vec.Vec.extend_from_slice
  step*
