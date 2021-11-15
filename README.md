# CyberPower UPS report server

This is a simple web server which reports the status of the UPS.

It has to be executed as a sudo, because it is using powerpanel program which actually does the real reading of the UPS status. This program just responds to request on demand.

Optionally you can change the server port.
```sh
> ./pwrstat-server --port 3333
```

You can test it with curl call on the default port 9999
```sh
> curl server-ip:9999/ups_status
```