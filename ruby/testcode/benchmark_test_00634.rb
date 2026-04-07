require_relative 'shared'

def do_read;   "reading";   end
def do_update; "updating";  end
def do_delete; "deleting";  end

def explicit_dispatch(req)
  action = req.param('action')
  result = if action == 'read'
             do_read
           elsif action == 'update'
             do_update
           elsif action == 'delete'
             do_delete
           else
             raise ArgumentError, 'unknown action'
           end
  BenchmarkResponse.json({ result: result })
end
