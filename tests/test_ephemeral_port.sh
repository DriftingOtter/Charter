#!/bin/bash

# Define the IP address and port
IP_ADDRESS="127.0.0.1"
PORT=8000

echo "hello there!" | nc -q 1 $IP_ADDRESS $PORT

echo "my name is daksh, 12345." | nc -q 1 $IP_ADDRESS $PORT

echo "see you soon :)" | nc -q 1 $IP_ADDRESS $PORT


