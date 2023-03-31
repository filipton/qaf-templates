#!/bin/bash

read -p "You can close tmux with C-f + x"

echo """
set -g base-index 1
setw -g pane-base-index 1
set -g mouse on
set -g remain-on-exit on
set-option -wg automatic-rename off

unbind C-b
set-option -g prefix C-f
bind-key C-f send-prefix
""" > /tmp/fntmux.conf

pid=$(pidof $(cat Cargo.toml | grep "name = " | awk -F'"' '{print $2}'))
kill -9 $pid

tmux -L fnstack -f /tmp/fntmux.conf new-session -c ./ -d "cargo watch -x run"
#tmux -L fnstack new-window -c ./url-shortener "cargo run"
tmux -L fnstack a

rm /tmp/fntmux.conf
