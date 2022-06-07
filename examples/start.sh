#!/bin/bash
# 
# Starts Bing Webserver
#

# set all env variables from config.env
set -o allexport
source config.env set +o allexport

# start webserver
../target/debug/bing-webserver