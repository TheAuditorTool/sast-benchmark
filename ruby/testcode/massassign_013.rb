require_relative 'shared'
require 'json'

# vuln-code-snippet start ruby_massassign_json_body
def create_from_json(req)
  attrs = JSON.parse(req.body_str)
  BenchmarkResponse.ok("created: #{attrs}") # vuln-code-snippet vuln-line ruby_massassign_json_body
end
# vuln-code-snippet end ruby_massassign_json_body
