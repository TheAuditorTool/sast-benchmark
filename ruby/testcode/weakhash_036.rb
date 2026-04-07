require_relative 'shared'

begin
  require 'scrypt'
rescue LoadError
  # scrypt gem not available -- syntax-checked only
end

# vuln-code-snippet start ruby_weakhash_scrypt_strong
def store_password_scrypt(req)
  password = req.param('password')
  hashed = SCrypt::Password.create(password, key_len: 32, cost: '65536/8/1') # vuln-code-snippet safe-line ruby_weakhash_scrypt_strong
  BenchmarkResponse.json({ hash: hashed.to_s })
end
# vuln-code-snippet end ruby_weakhash_scrypt_strong
