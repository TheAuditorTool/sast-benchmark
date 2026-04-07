require_relative 'shared'

begin
  require 'bcrypt'
rescue LoadError
  # bcrypt gem not available -- syntax-checked only
end

# vuln-code-snippet start ruby_weakhash_bcrypt_cost12
def store_password_bcrypt(req)
  password = req.param('password')
  hashed = BCrypt::Password.create(password, cost: 12) # vuln-code-snippet safe-line ruby_weakhash_bcrypt_cost12
  BenchmarkResponse.json({ hash: hashed.to_s })
end
# vuln-code-snippet end ruby_weakhash_bcrypt_cost12
