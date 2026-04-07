require_relative 'shared'

# vuln-code-snippet start ruby_regex_char_class_anchor
def handle_char_class_anchor(req)
  email = req.param('email')
  result = /\A[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}\z/.match(email) # vuln-code-snippet safe-line ruby_regex_char_class_anchor
  BenchmarkResponse.json({ valid: !result.nil? })
end
# vuln-code-snippet end ruby_regex_char_class_anchor
