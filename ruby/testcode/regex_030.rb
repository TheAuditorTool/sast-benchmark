require_relative 'shared'

# vuln-code-snippet start ruby_regex_email_validate_redos
def handle_email_validate_redos(req)
  email = req.param('email')
  result = /^[\w+\-.]+@[a-z\d\-.]+\.[a-z]+$/i.match(email) # vuln-code-snippet vuln-line ruby_regex_email_validate_redos
  BenchmarkResponse.json({ valid: !result.nil? })
end
# vuln-code-snippet end ruby_regex_email_validate_redos
