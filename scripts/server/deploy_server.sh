#!/bin/bash

# Variables
USER="shughi"
PIIP="192.168.1.160"
PI_SERVER_COMMAND="cd /home/shughi/Documents/proj/bubu && git fetch && git pull && cd svelty && npm install"

# Restart the server
ssh $USER@$PIIP "$PI_SERVER_COMMAND"

echo "Deployed on $PIIP"

