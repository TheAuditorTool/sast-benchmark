require_relative 'shared'

# vuln-code-snippet start ruby_regex_char_class_only
def validate_numeric(req)
  text = req.param('number')
  matched = text.match(/\A\d+\z/) # vuln-code-snippet safe-line ruby_regex_char_class_only
  BenchmarkResponse.ok(matched ? 'valid number' : 'invalid')
end
# vuln-code-snippet end ruby_regex_char_class_only
