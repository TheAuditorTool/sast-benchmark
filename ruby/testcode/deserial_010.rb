require_relative 'shared'
require 'json'

# vuln-code-snippet start ruby_deser_json_strict
def load_api_payload(req)
  input = req.body_str
  data = JSON.parse(input, symbolize_names: true) # vuln-code-snippet safe-line ruby_deser_json_strict
  user = data.fetch(:user, 'anonymous').to_s
  BenchmarkResponse.ok("user: #{user}")
end
# vuln-code-snippet end ruby_deser_json_strict
