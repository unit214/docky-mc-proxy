#!/bin/bash

# Start NGINX in the background
/usr/sbin/nginx -c /usr/src/docky-mc-proxy/nginx.conf &

# init the docky-mc-proxy
/usr/src/docky-mc-proxy/dmp init

# output nginx logs to shell
tail -f /var/log/nginx/access.log /var/log/nginx/error.log