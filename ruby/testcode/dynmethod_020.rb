require_relative 'shared'

def handle_csv(data); "csv:#{data}"; end
def handle_json(data); "json:#{data}"; end
def handle_xml(data); "xml:#{data}"; end

# vuln-code-snippet start ruby_dynmethod_case_dispatch
def format_output(req)
  fmt = req.param('format')
  data = req.param('data')
  result = case fmt # vuln-code-snippet safe-line ruby_dynmethod_case_dispatch
           when 'csv' then handle_csv(data)
           when 'json' then handle_json(data)
           when 'xml' then handle_xml(data)
           else return BenchmarkResponse.bad_request('unsupported format')
           end
  BenchmarkResponse.ok(result)
end
# vuln-code-snippet end ruby_dynmethod_case_dispatch
