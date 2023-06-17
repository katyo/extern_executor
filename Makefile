test: test_cbindgen test_no_std test_example_bin test_example_uv test_example_dart

test_cbindgen:
	cargo test --features cbindgen

test_no_std:
	cargo test --no-default-features --features no_std

test_example_%:
	$(MAKE) -C example_$*
