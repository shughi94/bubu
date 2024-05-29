#!/bin/bash

# Variables
USER="shughi"
PIIP="192.168.1.160"
SVELTY_COMMAND="cd /home/shughi/Documents/proj/bubu/svelty && npm run dev"
RUSTY_COMMAND="cd /home/shughi/Documents/proj/bubu/rusty && cargo run"

# Restart the server
ssh $USER@$PIIP "$RUSTY_COMMAND &"
ssh $USER@$PIIP "$SVELTY_COMMAND &"


echo "Server on $PIIP has been restarted."