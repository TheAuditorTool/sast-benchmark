require_relative 'shared'

EMAIL_PATTERN = /\A[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}\z/

def validate_email(req)
  email = req.param('email')
  matched = EMAIL_PATTERN.match(email)
  BenchmarkResponse.ok(matched ? 'valid' : 'invalid')
end
