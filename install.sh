#/bin/bash

cargo build --release

sudo systemctl stop dart-sweeper

sudo cp target/release/dart-sweeper /usr/bin/dart-sweeper
sudo cp dart-sweeper.service /etc/systemd/system/dart-sweeper.service

sudo systemctl enable dart-sweeper
sudo systemctl start dart-sweeper