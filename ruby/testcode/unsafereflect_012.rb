require_relative 'shared'

# vuln-code-snippet start ruby_reflect_case_class
def select_handler(req)
  type = req.param('type')
  handler = case type # vuln-code-snippet safe-line ruby_reflect_case_class
            when 'json' then :handle_json
            when 'xml' then :handle_xml
            when 'csv' then :handle_csv
            else return BenchmarkResponse.bad_request('unknown')
            end
  BenchmarkResponse.ok("handler: #{handler}")
end
# vuln-code-snippet end ruby_reflect_case_class
