all: clean release install
show-video EXTENSION='y4m':
  @ls -1 $HOME/video/*.{{EXTENSION}} | xargs -n 1 basename | sed 's/\..*//'
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
run VIDEO *ARGS: release
  ./target/release/rav1e -y -o $HOME/video/{{VIDEO}}.ivf {{ARGS}} $HOME/video/{{VIDEO}}.y4m
check VIDEO:
  ffprobe -v error -select_streams v:0 -show_entries stream=ivf -of default=nokey=1:noprint_wrappers=1 $HOME/video/{{VIDEO}}.ivf
size VIDEO FORMAT='b':
  dust -o {{FORMAT}} {{VIDEO}}.*
