require_relative 'shared'

def handle_email_validate_redos(req)
  email = req.param('email')
  result = /^[\w+\-.]+@[a-z\d\-.]+\.[a-z]+$/i.match(email)
  BenchmarkResponse.json({ valid: !result.nil? })
end
