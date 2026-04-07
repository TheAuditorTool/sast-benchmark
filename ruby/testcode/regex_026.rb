require_relative 'shared'

# vuln-code-snippet start ruby_regex_nested_optional
def handle_nested_optional(req)
  input = req.param('input')
  result = /^(a?)+$/ =~ input # vuln-code-snippet vuln-line ruby_regex_nested_optional
  BenchmarkResponse.json({ matched: !result.nil? })
end
# vuln-code-snippet end ruby_regex_nested_optional
