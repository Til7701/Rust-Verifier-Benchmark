import Lake
open Lake DSL

require aeneas from "/opt/aeneas/backends/lean"

package «aeneas-test» {}

@[default_target]
lean_lib «AeneasTest»
