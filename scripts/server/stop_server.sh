#!/bin/bash

# Variables
USER="shughi"
PIIP="192.168.1.160"

FRONTEND_JOB="npm run dev"
BACKEND_JOB="target/debug/rusty"

# Kill the server process
ssh $USER@$PIIP "pkill -f $FRONTEND_JOB"
ssh $USER@$PIIP "pkill -f $BACKEND_JOB"

echo "Server on $PIIP has been stopped."
