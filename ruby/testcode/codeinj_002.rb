require_relative 'shared'
require 'json'

# vuln-code-snippet start ruby_codeinj_json_parse
def parse_input(req)
  input = req.body_str
  data = JSON.parse(input) # vuln-code-snippet safe-line ruby_codeinj_json_parse
  name = data['name'].to_s
  BenchmarkResponse.ok("Hello, #{name}")
end
# vuln-code-snippet end ruby_codeinj_json_parse
