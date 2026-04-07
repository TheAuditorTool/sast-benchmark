#!/bin/bash
structured_python_log() {
    local user_msg="$1"
    python3 -c "
import json, sys, datetime
entry = {'ts': datetime.datetime.utcnow().isoformat(), 'msg': sys.argv[1]}
print(json.dumps(entry))
" "$user_msg" >> app.log
}
