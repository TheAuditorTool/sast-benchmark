require_relative 'shared'

class FakeLogger
  def error(msg); end
end

def login_with_logging(req, logger)
  username = req.post('username')
  password = req.post('password')
  stored_hash = '$2a$12$examplehashedpasswordvalue'
  begin
    raise 'Invalid credentials' unless password == stored_hash
    BenchmarkResponse.ok("Welcome #{username}")
  rescue => e
    logger.error("Login failed for #{username} with password #{password}")
    BenchmarkResponse.error('Unauthorized', 401)
  end
end
