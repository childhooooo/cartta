cmdname=$(basename $0)

echoerr() { echo "$@" 1>&2; }

wait_for()
{
    if [[ $TIMEOUT -gt 0 ]]; then
        echoerr "$cmdname: waiting $TIMEOUT seconds for $HOST:$PORT"
    else
        echoerr "$cmdname: waiting for $HOST:$PORT without a timeout"
    fi
    start_ts=$(date +%s)
    while :
    do
        if [[ $ISBUSY -eq 1 ]]; then
            nc -z $HOST $PORT
            result=$?
        else
            (echo > /dev/tcp/$HOST/$PORT) >/dev/null 2>&1
            result=$?
        fi
        if [[ $result -eq 0 ]]; then
            end_ts=$(date +%s)
            echoerr "$cmdname: $HOST:$PORT is available after $((end_ts - start_ts)) seconds"
            break
        fi
        sleep 1
    done
    return $result
}

while [[ $# -gt 0 ]]
do
    case "$1" in
        *:* )
        hostport=(${1//:/ })
        HOST=${hostport[0]}
        PORT=${hostport[1]}
        shift 1
        ;;
        -t)
        TIMEOUT="$2"
        if [[ $TIMEOUT == "" ]]; then break; fi
        shift 2
        ;;
        *)
        echoerr "Unknown argument: $1"
        ;;
    esac
done

if [[ "$HOST" == "" || "$PORT" == "" ]]; then
    echoerr "Error: you need to provide a host and port to test."
fi

TIMEOUT=${TIMEOUT:-15}

TIMEOUT_PATH=$(realpath $(which timeout))
if [[ $TIMEOUT_PATH =~ "busybox" ]]; then
    ISBUSY=1
else
    ISBUSY=0
fi

wait_for
RESULT=$?
exit $RESULT
