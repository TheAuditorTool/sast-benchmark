require_relative 'shared'

def issue_recovery_code(req)
  user_id = req.param('user_id')
  code = SecureRandom.hex(8)
  BenchmarkResponse.json({ code: code, user: user_id })
end
