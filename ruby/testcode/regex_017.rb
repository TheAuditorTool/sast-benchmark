require_relative 'shared'

# vuln-code-snippet start ruby_regex_hex_nested
def validate_hex_string(req)
  text = req.param('hex')
  matched = text.match(/\A([0-9a-f]+)+\z/) # vuln-code-snippet vuln-line ruby_regex_hex_nested
  BenchmarkResponse.ok(matched ? 'valid hex' : 'invalid hex')
end
# vuln-code-snippet end ruby_regex_hex_nested
