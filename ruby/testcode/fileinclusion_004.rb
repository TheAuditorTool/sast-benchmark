require_relative 'shared'

# vuln-code-snippet start ruby_fi_require_constant
def process_json_request(req)
  require('json') # vuln-code-snippet safe-line ruby_fi_require_constant
  body = req.body_str
  data = JSON.parse(body)
  BenchmarkResponse.ok(data.to_s)
end
# vuln-code-snippet end ruby_fi_require_constant
