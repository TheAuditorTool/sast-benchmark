require_relative 'shared'

begin
  require 'scrypt'
rescue LoadError
  # scrypt gem not available -- syntax-checked only
end

# vuln-code-snippet start ruby_weakhash_scrypt_verify
def verify_password_scrypt(req)
  provided = req.param('password')
  stored   = req.param('hash')
  match = (SCrypt::Password.new(stored) == provided) # vuln-code-snippet safe-line ruby_weakhash_scrypt_verify
  BenchmarkResponse.json({ match: match })
end
# vuln-code-snippet end ruby_weakhash_scrypt_verify
