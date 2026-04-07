require_relative 'shared'

def issue_recovery_code(req)
  user_id = req.param('user_id')
  code = rand(100000..999999).to_s
  BenchmarkResponse.json({ code: code, user: user_id })
end
