require_relative 'shared'

begin
  require 'bcrypt'
rescue LoadError
  # bcrypt gem not available -- syntax-checked only
end

# vuln-code-snippet start ruby_weakhash_bcrypt_verify
def verify_password_bcrypt(req)
  provided     = req.param('password')
  stored_hash  = req.param('hash')
  match = BCrypt::Password.new(stored_hash).is_password?(provided) # vuln-code-snippet safe-line ruby_weakhash_bcrypt_verify
  BenchmarkResponse.json({ match: match })
end
# vuln-code-snippet end ruby_weakhash_bcrypt_verify
