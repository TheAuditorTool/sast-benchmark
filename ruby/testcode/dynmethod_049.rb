require_relative 'shared'

def do_read;   "reading";   end
def do_update; "updating";  end
def do_delete; "deleting";  end

# vuln-code-snippet start ruby_dynmethod_no_dynamic_dispatch
def explicit_dispatch(req)
  action = req.param('action')
  result = if action == 'read' # vuln-code-snippet safe-line ruby_dynmethod_no_dynamic_dispatch
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
# vuln-code-snippet end ruby_dynmethod_no_dynamic_dispatch
