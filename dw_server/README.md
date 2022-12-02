## dw-server

Server need redis && mysql service.

### Install redis

https://redis.io/docs/getting-started/installation/install-redis-on-linux/

#### ubuntu:

``` BASH
sudo apt install lsb-release

curl -fsSL https://packages.redis.io/gpg | sudo gpg --dearmor -o /usr/share/keyrings/redis-archive-keyring.gpg

echo "deb [signed-by=/usr/share/keyrings/redis-archive-keyring.gpg] https://packages.redis.io/deb $(lsb_release -cs) main" | sudo tee /etc/apt/sources.list.d/redis.list

sudo apt-get update
sudo apt-get install redis

```

#### FAQ

如果新的redis服务起不来，可能是没删干净旧的配置，
我这边报错：
`Failed to start redis.service: Unit redis-server.service is masked`

解决方法
`sudo systemctl unmask  redis-server.service`

### install mysql

``` CPP
sudo apt update
sudo apt install mysql-server
sudo systemctl start mysql.service
```

https://www.digitalocean.com/community/tutorials/how-to-install-mysql-on-ubuntu-20-04

``` Bash
sudo mysql
ALTER USER 'root'@'localhost' IDENTIFIED WITH mysql_native_password BY 'xxxxxxxxxxxxxxxxx';
exit
mysql -u root -p
ALTER USER 'root'@'localhost' IDENTIFIED WITH auth_socket;

sudo mysql_secure_installation

CREATE USER 'dw-consumer'@'localhost' IDENTIFIED WITH mysql_native_password BY 'xxxxxxxxxxxxxxxxx';

CREATE USER 'dw-dashboard'@'localhost' IDENTIFIED WITH mysql_native_password BY 'xxxxxxxxxxxxxxxxx';

GRANT CREATE, ALTER, DROP, INSERT, UPDATE, DELETE, SELECT, REFERENCES, RELOAD on *.* TO 'dw-consumer'@'localhost';
GRANT SELECT on *.* TO 'dw-dashboard'@'localhost';
FLUSH PRIVILEGES;
```
