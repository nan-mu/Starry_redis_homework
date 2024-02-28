./redis-server /redis.conf --loglevel verbose &
./redis-cli-static -p 6380 ping
./redis-cli-static -p 6380 info
./redis-cli-static -p 6380 set cicv good
./redis-cli-static -p 6380 get cicv