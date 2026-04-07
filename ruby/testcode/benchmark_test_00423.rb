require_relative 'shared'

def store_password_ascii_sum(req)
  password = req.param('password')
  digest = password.bytes.sum.to_s(16)
  BenchmarkResponse.json({ hash: digest })
end
