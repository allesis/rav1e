all: clean release install
setup REMOTE_NAME='cluster' REMOTE_DIRECTORY='/cluster/research-groups/wehrwein/home/$(whoami)/video' LOCAL_DIRECTORY='$HOME/video':
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
list-videos EXTENSION='y4m' DIRECTORY='$HOME/video':
  @ls -1 {{DIRECTORY}}/*.{{EXTENSION}} | xargs -n 1 basename | sed 's/\..*//'
clean:
	cargo clean
[confirm("Are you sure you want to clean up the video directory? This will delete things!")]
clean-video DIRECTORY='$HOME/video':
  rm {{DIRECTORY}}/*-decode.y4m {{DIRECTORY}}/*.ivf {{DIRECTORY}}/*.mp4
clean-video-command DIRECTORY='$HOME/video':
  @echo "rm {{DIRECTORY}}/\*-decode.y4m {{DIRECTORY}}/\*.ivf {{DIRECTORY}}/\*.mp4"
build:
	cargo build
release:
	cargo build --release
install INSTALL_DIR='$HOME/.local/bin':
	cp target/debug/rav1e {{INSTALL_DIR}}/rav1e
install-release INSTALL_DIR='$HOME/.local/bin':
	cp target/release/rav1e {{INSTALL_DIR}}/rav1e
run VIDEO DIRECTORY='$HOME/video' *ARGS: release
  ./target/release/rav1e -y -o {{DIRECTORY}}/{{VIDEO}}.ivf {{ARGS}} {{DIRECTORY}}/{{VIDEO}}.y4m
check VIDEO DIRECTORY='$HOME/video': setup
  ffprobe -v error -select_streams v:0 -show_entries stream=ivf -of default=nokey=1:noprint_wrappers=1 {{DIRECTORY}}/{{VIDEO}}.ivf
size VIDEO DIRECTORY='$HOME/video' FORMAT='k':
  dust -o {{FORMAT}} {{DIRECTORY}}/{{VIDEO}}.*
