all: clean release install
clean:
	cargo clean
build:
	cargo build
release:
	cargo build --release
install:
	cp target/debug/rav1e $HOME/.local/bin/rav1e
install-release:
	cp target/release/rav1e $HOME/.local/bin/rav1e
test-basic: release
  ./target/release/rav1e -y -o $HOME/video/color.ivf $HOME/video/color.y4m
  ffprobe -v error -select_streams v:0 -show_entries stream=ivf -of default=nokey=1:noprint_wrappers=1 $HOME/video/color.ivf
