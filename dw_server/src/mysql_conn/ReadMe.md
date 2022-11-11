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
