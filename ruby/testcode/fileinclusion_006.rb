require_relative 'shared'

# vuln-code-snippet start ruby_fi_require_hardcoded
def parse_request_body(req)
  require_relative('lib/parser') # vuln-code-snippet safe-line ruby_fi_require_hardcoded
  result = SimpleParser.parse(req.body_str)
  BenchmarkResponse.ok(result.to_s)
end
# vuln-code-snippet end ruby_fi_require_hardcoded
