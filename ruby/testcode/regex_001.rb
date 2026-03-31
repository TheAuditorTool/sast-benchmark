require_relative 'shared'

# vuln-code-snippet start ruby_regex_nested_quant
def validate_input(req)
  text = req.param('text')
  matched = text.match(/(a+)+$/)  # vuln-code-snippet vuln-line ruby_regex_nested_quant
  BenchmarkResponse.ok(matched ? 'match' : 'no match')
end
# vuln-code-snippet end ruby_regex_nested_quant
