require_relative 'shared'
require 'json'

# vuln-code-snippet start ruby_codeinj_json_schema
def parse_json_safe(req)
  data = JSON.parse(req.body_str) # vuln-code-snippet safe-line ruby_codeinj_json_schema
  BenchmarkResponse.json(data)
end
# vuln-code-snippet end ruby_codeinj_json_schema
