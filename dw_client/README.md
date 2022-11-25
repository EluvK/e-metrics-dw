### DW_CLIENT

#### Agent

Monitor metrics log file, scan each line's log, convert metrics log to net-packet then send to server.

The data flows like this:

`Read log file(IO) - Processed by metrics log handler(CPU) - Send net packet (IO)`

So two async cache queue is needed.