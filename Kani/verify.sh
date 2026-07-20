#!/bin/sh

set -e

cargo kani -Z function-contracts -Z stubbing -Z loop-contracts
