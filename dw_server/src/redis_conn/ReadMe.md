### install redis

https://redis.io/docs/getting-started/installation/install-redis-on-linux/

#### ubuntu:

``` BASH
sudo apt install lsb-release

curl -fsSL https://packages.redis.io/gpg | sudo gpg --dearmor -o /usr/share/keyrings/redis-archive-keyring.gpg

echo "deb [signed-by=/usr/share/keyrings/redis-archive-keyring.gpg] https://packages.redis.io/deb $(lsb_release -cs) main" | sudo tee /etc/apt/sources.list.d/redis.list

sudo apt-get update
sudo apt-get install redis

```

##### FAQ

如果新的redis服务起不来，可能是没删干净旧的配置，
我这边报错：
`Failed to start redis.service: Unit redis-server.service is masked`

解决方法
`sudo systemctl unmask  redis-server.service`

