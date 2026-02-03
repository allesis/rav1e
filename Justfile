set dotenv-load := true

DEFAULT_EXTENSION := `echo ${DEFAULT_EXTENSION:-"y4m"}`
DEFAULT_LOCAL_DIR := `echo ${DEFAULT_LOCAL_DIR:-"$HOME/video"}`
DEFAULT_INSTALL_NAME := `echo ${DEFAULT_INSTALL_NAME:-"$(pwd | xargs -n 1 basename)-$(git branch --show-current)"}`
DEFAULT_INSTALL_DIR := `echo ${DEFAULT_INSTALL_DIR:-"$HOME/.local/bin"}`
DEFAULT_REMOTE_NAME := `echo ${DEFAULT_REMOTE_NAME:-"cluster"}`
DEFAULT_REMOTE_DIR := `echo ${DEFAULT_REMOTE_DIR:-"/dev/null"}`

all: clean release install
setup REMOTE_NAME=DEFAULT_REMOTE_NAME REMOTE_DIRECTORY=DEFAULT_REMOTE_DIR LOCAL_DIRECTORY=DEFAULT_LOCAL_DIR:
  #!/usr/bin/env bash
  ls {{LOCAL_DIRECTORY}} > /dev/null
  if [ "$?" -ne 0 ]; then \
    mkdir -p {{LOCAL_DIRECTORY}}; \
  fi
  sshfs -o auto_unmount {{REMOTE_NAME}}:{{REMOTE_DIRECTORY}} {{LOCAL_DIRECTORY}} > 2&> /dev/null
  if [ "$?" -ne 0 ]; then \
    fusermount -u {{LOCAL_DIRECTORY}}; \
    sshfs -o auto_unmount {{REMOTE_NAME}}:{{REMOTE_DIRECTORY}} {{LOCAL_DIRECTORY}} > 2&> /dev/null; \
  else \
    exit 0; \
  fi
  if [ "$?" -ne 0 ]; then \
    echo "Setup failed"; \
  fi
list-videos EXTENSION=DEFAULT_EXTENSION DIRECTORY=DEFAULT_LOCAL_DIR:
  @ls -1 {{DIRECTORY}}/*.{{EXTENSION}} | xargs -n 1 basename | sed 's/\..*//'
clean:
	cargo clean
[confirm("Are you sure you want to clean up the video directory? This will delete things!")]
clean-video DIRECTORY=DEFAULT_LOCAL_DIR:
  rm {{DIRECTORY}}/*-decode.y4m {{DIRECTORY}}/*.ivf {{DIRECTORY}}/*.mp4
clean-video-command DIRECTORY=DEFAULT_LOCAL_DIR:
  @echo "rm {{DIRECTORY}}/\*-decode.y4m {{DIRECTORY}}/\*.ivf {{DIRECTORY}}/\*.mp4"
build:
	cargo build
release:
	cargo build --release
install INSTALL_DIR=DEFAULT_INSTALL_DIR INSTALL_NAME=DEFAULT_INSTALL_NAME: build
	cp target/debug/rav1e {{INSTALL_DIR}}/{{INSTALL_NAME}}
install-release INSTALL_DIR=DEFAULT_INSTALL_DIR INSTALL_NAME=DEFAULT_INSTALL_NAME: release
	cp target/release/rav1e {{INSTALL_DIR}}/{{INSTALL_NAME}}
run VIDEO DIRECTORY=DEFAULT_LOCAL_DIR *ARGS: release
  ./target/release/rav1e -y -o {{DIRECTORY}}/{{VIDEO}}.ivf {{ARGS}} {{DIRECTORY}}/{{VIDEO}}.y4m
check VIDEO DIRECTORY=DEFAULT_LOCAL_DIR: setup
  ffprobe -v error -select_streams v:0 -show_entries stream=ivf -of default=nokey=1:noprint_wrappers=1 {{DIRECTORY}}/{{VIDEO}}.ivf
size VIDEO DIRECTORY=DEFAULT_LOCAL_DIR FORMAT='k':
  dust -o {{FORMAT}} {{DIRECTORY}}/{{VIDEO}}.*
