FROM haproxy:2.9-alpine

COPY haproxy.cfg /usr/local/etc/haproxy/haproxy.cfg
