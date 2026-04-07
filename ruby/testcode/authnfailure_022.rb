require_relative 'shared'
require 'bcrypt'

# vuln-code-snippet start ruby_authn_empty_password
def login_empty_bypass(req)
  username = req.post('username')
  password = req.post('password')
  stored_hash = '$2a$12$examplehashedpasswordvalue'
  return BenchmarkResponse.error('Unauthorized', 401) if password.empty? # vuln-code-snippet vuln-line ruby_authn_empty_password
  return BenchmarkResponse.error('Unauthorized', 401) unless password == stored_hash
  BenchmarkResponse.ok("Welcome #{username}")
end
# vuln-code-snippet end ruby_authn_empty_password
