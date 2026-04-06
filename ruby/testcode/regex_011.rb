require_relative 'shared'

# vuln-code-snippet start ruby_regex_email_bad
def validate_email(req)
  email = req.param('email')
  matched = email.match(/\A([a-zA-Z0-9]+\.)+[a-zA-Z]+\z/) # vuln-code-snippet vuln-line ruby_regex_email_bad
  BenchmarkResponse.ok(matched ? 'valid' : 'invalid')
end
# vuln-code-snippet end ruby_regex_email_bad
