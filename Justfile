DEFAULT_REMOTE_DIR := '$DEFAULT_REMOTE_DIR'
DEFAULT_DIRECTORY := `echo ${DEFAULT_DIRECTORY:-"$HOME/video"}`
DEFAULT_INSTALL_DIR := `echo ${DEFAULT_INSTALL_DIR:-"$HOME/.local/bin"}`
all: clean release install
setup REMOTE_NAME='cluster' REMOTE_DIRECTORY=DEFAULT_REMOTE_DIR LOCAL_DIRECTORY=DEFAULT_DIRECTORY:
  #!/usr/bin/env bash
  ls {{LOCAL_DIRECTORY}} > /dev/null
  if [ "$?" -ne 0 ]; then \
    mkdir -p {{LOCAL_DIRECTORY}}; \
  fi
  sshfs -o auto_unmount {{REMOTE_NAME}}:{{REMOTE_DIRECTORY}} {{LOCAL_DIRECTORY}} 2>& 1>/dev/null
  if [ "$?" -ne 0 ]; then \
    fusermount -u {{LOCAL_DIRECTORY}}; \
    sshfs -o auto_unmount {{REMOTE_NAME}}:{{REMOTE_DIRECTORY}} {{LOCAL_DIRECTORY}} 2>& 1>/dev/null; \
  else \
    exit 0; \
  fi
  if [ "$?" -ne 0 ]; then \
    echo "Setup failed"; \
  fi
list-videos EXTENSION='y4m' DIRECTORY=DEFAULT_DIRECTORY:
  @ls -1 {{DIRECTORY}}/*.{{EXTENSION}} | xargs -n 1 basename | sed 's/\..*//'
clean:
	cargo clean
[confirm("Are you sure you want to clean up the video directory? This will delete things!")]
clean-video DIRECTORY=DEFAULT_DIRECTORY:
  rm {{DIRECTORY}}/*-decode.y4m {{DIRECTORY}}/*.ivf {{DIRECTORY}}/*.mp4
clean-video-command DIRECTORY=DEFAULT_DIRECTORY:
  @echo "rm {{DIRECTORY}}/\*-decode.y4m {{DIRECTORY}}/\*.ivf {{DIRECTORY}}/\*.mp4"
build:
	cargo build
release:
	cargo build --release
install INSTALL_DIR=DEFAULT_INSTALL_DIR:
	cp target/debug/rav1e {{INSTALL_DIR}}/rav1e
install-release INSTALL_DIR=DEFAULT_INSTALL_DIR:
	cp target/release/rav1e {{INSTALL_DIR}}/rav1e
run VIDEO DIRECTORY=DEFAULT_DIRECTORY *ARGS: build
  ./target/debug/rav1e --frame-rate 30 --time-scale 1 -y -r {{DIRECTORY}}/{{VIDEO}}-reconstruction.y4m --threads 1 --low-latency -o {{DIRECTORY}}/{{VIDEO}}.ivf {{ARGS}} {{DIRECTORY}}/{{VIDEO}}.y4m
run-release VIDEO DIRECTORY=DEFAULT_DIRECTORY *ARGS: release
  ./target/release/rav1e --frame-rate 30 --time-scale 1 -y --threads 1 --low-latency -o {{DIRECTORY}}/{{VIDEO}}.ivf {{ARGS}} {{DIRECTORY}}/{{VIDEO}}.y4m
run-no-disable-reorder VIDEO DIRECTORY=DEFAULT_DIRECTORY *ARGS: build
  ./target/debug/rav1e --frame-rate 30 --time-scale 1 -y --threads 1 -o {{DIRECTORY}}/{{VIDEO}}.ivf {{ARGS}} {{DIRECTORY}}/{{VIDEO}}.y4m
run-no-disable-threading VIDEO DIRECTORY=DEFAULT_DIRECTORY *ARGS: build
  ./target/debug/rav1e --frame-rate 30 --time-scale 1 -y --low-latency -o {{DIRECTORY}}/{{VIDEO}}.ivf {{ARGS}} {{DIRECTORY}}/{{VIDEO}}.y4m
run-no-disable-all VIDEO DIRECTORY=DEFAULT_DIRECTORY *ARGS: build
  ./target/debug/rav1e --frame-rate 30 --time-scale 1 -y -o {{DIRECTORY}}/{{VIDEO}}.ivf {{ARGS}} {{DIRECTORY}}/{{VIDEO}}.y4m
check VIDEO DIRECTORY=DEFAULT_DIRECTORY: setup
  ffprobe -v error -select_streams v:0 -show_entries stream=ivf -of default=nokey=1:noprint_wrappers=1 {{DIRECTORY}}/{{VIDEO}}.ivf
size VIDEO DIRECTORY=DEFAULT_DIRECTORY FORMAT='k':
  dust -o {{FORMAT}} {{DIRECTORY}}/{{VIDEO}}.*
