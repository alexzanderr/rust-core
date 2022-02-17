

# make
run:
	@cargo run --quiet

# make br
br:
	@cargo build --release

# make b
b:
	@cargo build

# make r
r:
	@cargo run

# make rr
rr:
	@cargo run --release



flow:
	@cargo run --example flow_control --quiet

rflow:
	@cargo run --example flow_control --release --quiet


spin:
	@cargo run --example spinners --quiet
	
rspin:
	@cargo run --example spinners --quiet --release



# .SILENT: flow | rflow | run | br | b | r | rr
