require_relative 'shared'

# vuln-code-snippet start ruby_regex_backreference_complex
def handle_backreference_complex(req)
  input = req.param('input')
  result = /((\w+)\s+\2)+/.match(input) # vuln-code-snippet vuln-line ruby_regex_backreference_complex
  BenchmarkResponse.json({ matched: !result.nil? })
end
# vuln-code-snippet end ruby_regex_backreference_complex
