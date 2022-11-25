### DW_CLIENT

#### Agent

Monitor metrics log file, scan each line's log, convert metrics log to net-packet then send to server.

The data flows like this:

`log file(IO) - metrics log handler(CPU) - net packet sent(IO)`

So two async queue is needed.