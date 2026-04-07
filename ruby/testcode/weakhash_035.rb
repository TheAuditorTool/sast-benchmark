require_relative 'shared'

begin
  require 'argon2'
rescue LoadError
  # argon2 gem not available -- syntax-checked only
end

# vuln-code-snippet start ruby_weakhash_argon2id
def store_password_argon2id(req)
  password = req.param('password')
  hashed = Argon2::Password.create(password, variant: :argon2id, t_cost: 3, m_cost: 65536) # vuln-code-snippet safe-line ruby_weakhash_argon2id
  BenchmarkResponse.json({ hash: hashed })
end
# vuln-code-snippet end ruby_weakhash_argon2id
