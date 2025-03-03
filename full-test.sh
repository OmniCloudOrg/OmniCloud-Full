echo "Beginning test of the Omni stack..."


# Init the Omni environment
echo "Initializing the Omni environment..."
cd ./Omni-CLI

cargo build --release
./target/release/omni init
./target/release/omni --cpi vb_cli_linux -- action create_vm --name test-vm --image ubuntu-18.04 --flavor small --network public --keypair default