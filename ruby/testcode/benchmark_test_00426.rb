require_relative 'shared'
require 'digest'

def store_password_per_char(req)
  password = req.param('password')
  digest = password.chars.map { |c| Digest::SHA1.hexdigest(c) }.join
  BenchmarkResponse.json({ hash: digest })
end
