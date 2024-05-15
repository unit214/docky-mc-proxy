#!/bin/bash

# Start NGINX in the background
/usr/sbin/nginx -c /usr/src/docky-mc-proxy/nginx.conf &

# Open a bash shell for user interaction
exec bash