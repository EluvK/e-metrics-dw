https://github.com/EluvK/e-metrics-dw/issues/8

working on **support real_time metrics unit**

#### 0x01

some basic principles of 'design && usage' thinking at first thought:

1. both client && server need to indicate which category-tag's metrics infomation that need to handle.
2. only need to impl one `MetricsUnit` and wrapped `AlarmWrapper`, that is `RealTime`, neither multi category-tag's.
3. we probably need some macro_rules or even derive_macro to make adding a new real_time metrics easy and quick.


#### 0x02

following the data flow, sort out what we need to do to support real_time:

1. types - alarm_type - `enum MetricsAlarmType`: Add `RealTime`, str: `"real_time"`

2. client - log_handler - `handler_metrics()` - Regex : need to match category and tag to 

``` RUST
match alarm_type {
    // match (fulllog, type, category, tag) with REGEX
    //...
    MetricsAlarmType::RealTime => {
        // get exact unittype from category and tag, call type::handle_log
    }
}
```

3. server - proxy - alarm_type is "real_time" , corresponding redis key  

no much things todo 

4. server - consumer : add consumer handler

``` RUST
HANDLE_UNIT!(handle_real_time, RealTimeUnit, MetricsAlarmType::RealTime);
```

here comes the most difficult part.

5. types - RealTimeUnit : need to manage all kinds of real-time metrics

The `RealTimeUnit`, is struct like `CounterUnit`, has members like these

``` RUST
pub struct RealTimeUnit {
    send_timestamp: TimeStamp,
    public_ip: IpAddress,
    category: String,
    tag: String,
    kvs: HashMap<String, String or I64>
}
```

And we also need to impl Trait `SqlTable` && `UnitJsonLogHandler` for it.  Inside which we need to implement different behavior according to category-tag.

which is conflict with these two Trait original design...

#### 0x03

So From the Trait side, each real-time need one unique struct, or we can not separate them in database. That means the above principle 0x01-2 is not correct.

Each category-tag' RealTime need one unique Struct that impl Trait `SqlTable` && `UnitJsonLogHandler`

cognitive correction:

1. 0x02-4, HANDLE_UNIT! macro, need multi lines for every real-time that we concern.
2. 0x02-5, there will be no struct named `RealTimeUnit`, but multi ones with name like `NodeStatusRealTimeUnit`, `BroadcastInfoRealTimeUnit`, ..etc.
3. we do need to figure out how to generate these similar code easily and elegant