require_relative 'shared'

# vuln-code-snippet start ruby_regex_anchored_simple
def validate_hex_id(req)
  input = req.param('id')
  matched = input.match(/\A[a-f0-9]{8}\z/) # vuln-code-snippet safe-line ruby_regex_anchored_simple
  BenchmarkResponse.ok(matched ? 'valid' : 'invalid')
end
# vuln-code-snippet end ruby_regex_anchored_simple
