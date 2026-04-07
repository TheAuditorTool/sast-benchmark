require_relative 'shared'

def read_record; "reading"; end
def list_records; "listing"; end

# vuln-code-snippet start ruby_dynmethod_case_dispatch_safe
def case_dispatch(req)
  result = case req.param('action') # vuln-code-snippet safe-line ruby_dynmethod_case_dispatch_safe
           when 'read' then read_record
           when 'list' then list_records
           else raise ArgumentError, 'unknown action'
           end
  BenchmarkResponse.json({ result: result })
end
# vuln-code-snippet end ruby_dynmethod_case_dispatch_safe
