#!/bin/bash

# A script to remove everything from bridges repository/subtree, except:
#
# - modules/grandpa;
# - modules/messages;
# - modules/parachains;
# - modules/relayers;
# - everything required from primitives folder.

# the script is able to work only on clean git copy
[[ -z "$(git status --porcelain)" ]] || { echo >&2 "The git copy must be clean"; exit 1; }

# let's avoid any restrictions on where this script can be called for - bridges repo may be
# plugged into any other repo folder. So the script (and other stuff that needs to be removed)
# may be located either in call dir, or one of it subdirs. 
BRIDGES_FOLDER="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )/.."

# let's leave repository/subtree in its original (clean) state if something fails below
function revert_to_clean_state {
	echo "Reverting to clean state..."
	git reset --hard
}
trap revert_to_clean_state EXIT

# remove everything we think is not required for our needs
rm -rf $BRIDGES_FOLDER/.config
rm -rf $BRIDGES_FOLDER/.github
rm -rf $BRIDGES_FOLDER/.maintain
rm -rf $BRIDGES_FOLDER/bin/millau
rm -rf $BRIDGES_FOLDER/bin/rialto
rm -rf $BRIDGES_FOLDER/bin/rialto-parachain
rm -rf $BRIDGES_FOLDER/deployments
rm -rf $BRIDGES_FOLDER/fuzz
rm -rf $BRIDGES_FOLDER/modules/beefy
rm -rf $BRIDGES_FOLDER/modules/shift-session-manager
rm -rf $BRIDGES_FOLDER/primitives/beefy
rm -rf $BRIDGES_FOLDER/primitives/chain-millau
rm -rf $BRIDGES_FOLDER/primitives/chain-rialto
rm -rf $BRIDGES_FOLDER/primitives/chain-rialto-parachain
rm -rf $BRIDGES_FOLDER/primitives/chain-westend
rm -rf $BRIDGES_FOLDER/relays
rm -f $BRIDGES_FOLDER/.dockerignore
rm -f $BRIDGES_FOLDER/.gitlab-ci.yml
rm -f $BRIDGES_FOLDER/Cargo.lock
rm -f $BRIDGES_FOLDER/Cargo.toml
rm -f $BRIDGES_FOLDER/ci.Dockerfile
rm -f $BRIDGES_FOLDER/Dockerfile

# let's fix Cargo.toml a bit (it'll be helpful if we are in the bridges repo)

cat > $BRIDGES_FOLDER/Cargo.toml <<-CARGO_TOML
[workspace]
resolver = "2"

members = [
	"bin/runtime-common",
	"modules/*",
	"primitives/*",
]
CARGO_TOML

# let's test if everything we need compiles

cargo check -p pallet-bridge-grandpa
cargo check -p pallet-bridge-grandpa --features runtime-benchmarks
cargo check -p pallet-bridge-grandpa --features try-runtime
cargo check -p pallet-bridge-messages
cargo check -p pallet-bridge-messages --features runtime-benchmarks
cargo check -p pallet-bridge-messages --features try-runtime
cargo check -p pallet-bridge-parachains
cargo check -p pallet-bridge-parachains --features runtime-benchmarks
cargo check -p pallet-bridge-parachains --features try-runtime
cargo check -p pallet-bridge-relayers
cargo check -p pallet-bridge-relayers --features runtime-benchmarks
cargo check -p pallet-bridge-relayers --features try-runtime
cargo check -p bridge-runtime-common
cargo check -p bridge-runtime-common --features runtime-benchmarks

echo "OK"
