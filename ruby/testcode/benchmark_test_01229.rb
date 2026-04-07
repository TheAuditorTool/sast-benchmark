require_relative 'shared'

def validate_email(req)
  email = req.param('email')
  matched = email.match(/\A([a-zA-Z0-9]+\.)+[a-zA-Z]+\z/)
  BenchmarkResponse.ok(matched ? 'valid' : 'invalid')
end
