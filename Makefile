dev/create:
	multipass launch --name rust-test --cloud-init dev-env.yaml
	multipass transfer ~/.ssh/id_rsa.pub rust-test:/home/ubuntu/.ssh/authorized_host

dev/remove:
	multipass stop rust-test && multipass delete rust-test && multipass purge

dev/shell:
	@multipass shell rust-test

dev/info:
	@multipass ls | grep rust-test

test:
	@RUST_BACKTRACE=0 cargo test
