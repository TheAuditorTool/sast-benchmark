require_relative 'shared'

class FakeLogger
  def error(msg); end
end

# vuln-code-snippet start ruby_authn_password_in_log
def login_with_logging(req, logger)
  username = req.post('username')
  password = req.post('password')
  stored_hash = '$2a$12$examplehashedpasswordvalue'
  begin
    raise 'Invalid credentials' unless password == stored_hash
    BenchmarkResponse.ok("Welcome #{username}")
  rescue => e
    logger.error("Login failed for #{username} with password #{password}") # vuln-code-snippet vuln-line ruby_authn_password_in_log
    BenchmarkResponse.error('Unauthorized', 401)
  end
end
# vuln-code-snippet end ruby_authn_password_in_log
