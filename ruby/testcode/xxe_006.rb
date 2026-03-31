require_relative 'shared'
require 'json'

# vuln-code-snippet start ruby_xxe_json_alternative
def parse_input(req)
  input = req.body_str
  data = JSON.parse(input) # vuln-code-snippet safe-line ruby_xxe_json_alternative
  BenchmarkResponse.json(data)
end
# vuln-code-snippet end ruby_xxe_json_alternative
