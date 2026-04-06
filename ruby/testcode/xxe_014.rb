require_relative 'shared'
require 'json'

# vuln-code-snippet start ruby_xxe_json_only
def parse_json_alternative(req)
  data = JSON.parse(req.body_str) # vuln-code-snippet safe-line ruby_xxe_json_only
  BenchmarkResponse.json(data)
end
# vuln-code-snippet end ruby_xxe_json_only
