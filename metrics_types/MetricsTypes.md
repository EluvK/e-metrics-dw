## Brief

current support metrics unit: `Counter` \ `Timer` \ `Flow`, corresponding `rs` file : [counter](./src/metrics_counter.rs), [timer](./src/metrics_timer.rs), [flow](./src/metrics_flow.rs)


## Format Regulation

### 1. raw result

dumped log from e-metrics module in form of compressed Json.

e.g.

#### counter:  

`{"category":"xvm","tag":"call_contract_succ","type":"counter","content":{"count":10,"value":9}}`

``` JSON
{
    "category": "xvm",
    "tag": "call_contract_succ",
    "type": "counter",
    "content": {
        "count": 10,
        "value": 9
    }
}
```

#### timer: 

`{"category":"xvm","tag":"execute_time","type":"timer","content":{"count":3060,"max_time":93926,"min_time":18,"avg_time":153}}`

``` JSON
{
    "category": "xvm",
    "tag": "execute_time",
    "type": "timer",
    "content": {
        "count": 3060,
        "max_time": 93926,
        "min_time": 18,
        "avg_time": 153
    }
}
```

#### flow

`{"category":"vhost","tag":"handle_data_ready_called","type":"flow","content":{"count":92,"max_flow":10,"min_flow":1,"sum_flow":131,"avg_flow":1,"tps_flow":131,"tps":"1.39"}}`

``` JSON
{
    "category": "vhost",
    "tag": "handle_data_ready_called",
    "type": "flow",
    "content": {
        "count": 92,
        "max_flow": 10,
        "min_flow": 1,
        "sum_flow": 131,
        "avg_flow": 1,
        "tps_flow": 131,
        "tps": "1.39"
    }
}
```


### 2. `Unit` data strust

Struct defined in corresponding `rs` file.

* add `TimeStamp` and `IpAddress`
* struct name could represent `type`.
* flatten the above `content`'s k-v pair into member

e.g. 

#### counter:

[file: metrics_counter.rs](./src/metrics_counter.rs)

``` RUST
pub struct CounterUnit {
    send_timestamp: TimeStamp,
    public_ip: IpAddress,
    category: String,
    tag: String,
    count: u64,
    value: i64,
}
```

### 3. `Wrapped Unit` data struct

Wrap `Unit` in `content`, add `env_name` and `alarm_type`.  

Aggregate a bundle of (might be different type) `Unit`, used in network transport.

``` RUST
pub struct AlarmWrapper<UnitType> {
    pub alarm_type: MetricsAlarmType,
    pub env: String,
    pub content: UnitType,
}

```

more reference at [alarm_wrapper.rs](./src/alarm_wrapper.rs)
