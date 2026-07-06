import Aeneas
import AeneasTest.Code.Funs

open Aeneas Aeneas.Std Result ControlFlow Error

open AeneasTest

#setup_aeneas_simps

@[reducible]
def Range.inv(r: range.m.Range) : Prop :=
  r.start ≤ r.end

@[step]
theorem range_new_spec (start: I32) (end1: I32) (h : start.val ≤ end1.val) :
  range.m.Range.new start end1 ⦃ result => result.start = start ∧ result.«end» = end1 ⦄ := by
  unfold range.m.Range.new
  step*

@[step]
theorem range_get_start_spec (self: range.m.Range) (hInv: Range.inv self) :
  range.m.Range.get_start self ⦃ result => Range.inv self ∧ result.val = self.start ⦄ := by
  unfold range.m.Range.get_start
  step*

@[step]
theorem range_set_start_spec (self: range.m.Range) (start: I32) (h: start ≤ self.«end») (hInv: Range.inv self) :
  range.m.Range.set_start self start ⦃ result => Range.inv self ∧ result.start = start ⦄ := by
  unfold range.m.Range.set_start
  step*

@[step]
theorem range_get_end_spec (self: range.m.Range) (hInv: Range.inv self) :
  range.m.Range.get_end self ⦃ result => Range.inv self ∧ result.val = self.«end» ⦄ := by
  unfold range.m.Range.get_end
  step*

@[step]
theorem range_set_end_spec (self: range.m.Range) (end1: I32) (h: self.«end» ≤ end1) (hInv: Range.inv self) :
  range.m.Range.set_end self end1 ⦃ result => Range.inv self ∧ result.«end» = end1 ⦄ := by
  unfold range.m.Range.set_end
  step*
