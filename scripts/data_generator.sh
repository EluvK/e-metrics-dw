#!/bin/bash

env_name='db_name'

rand_cnt=3000
each_cnt=5

for _i in $(seq 0 ${rand_cnt})
do
    date
    rand_ip=$(( ${RANDOM} % 255 )).$(( ${RANDOM} % 255 )).$(( ${RANDOM} % 255 )).$(( ${RANDOM} % 255 )):$(( ${RANDOM} % 1024 ))
    rand_ts=${RANDOM}
    rand_category=$(echo $RANDOM | md5sum | head -c 5)
    rand_tag=$(echo $RANDOM | md5sum | head -c 5)

    for _j in $(seq 0 ${each_cnt})
    do

    curl -X POST 127.0.0.1:3000/api/alarm -H 'Content-Type: application/json' -d '{"alarm_type":"timer","env":"'${env_name}'"'\
',"content":{"send_timestamp":"'${rand_ts}'","public_ip":"'${rand_ip}'","category":"'"${rand_category}"'","tag":"'"${rand_tag}"'","count":'${RANDOM}',"max_time":'${RANDOM}',"min_time":'${RANDOM}',"avg_time":'${RANDOM}'}}' --silent --output /dev/null

    curl -X POST 127.0.0.1:3000/api/alarm -H 'Content-Type: application/json' -d '{"alarm_type":"counter","env":"'${env_name}'"'\
',"content":{"send_timestamp":"'${rand_ts}'","public_ip":"'${rand_ip}'","category":"'"${rand_category}"'","tag":"'"${rand_tag}"'","count":'${RANDOM}',"value":'${RANDOM}'}}' --silent --output /dev/null

    curl -X POST 127.0.0.1:3000/api/alarm -H 'Content-Type: application/json' -d '{"alarm_type":"flow","env":"'${env_name}'"'\
',"content":{"send_timestamp":"'${rand_ts}'","public_ip":"'${rand_ip}'","category":"'"${rand_category}"'","tag":"'"${rand_tag}"'","count":'${RANDOM}',"max_flow":'${RANDOM}',"min_flow":'${RANDOM}',"sum_flow":'${RANDOM}',"avg_flow":'${RANDOM}',"tps_flow":'${RANDOM}',"tps":'${RANDOM}'}}' --silent --output /dev/null

    done
done
