require_relative 'shared'

begin
  require 'argon2'
rescue LoadError
  # argon2 gem not available -- syntax-checked only
end

# vuln-code-snippet start ruby_weakhash_argon2_verify
def verify_password_argon2(req)
  provided = req.param('password')
  stored   = req.param('hash')
  match = Argon2::Password.new(stored).matches?(provided) # vuln-code-snippet safe-line ruby_weakhash_argon2_verify
  BenchmarkResponse.json({ match: match })
end
# vuln-code-snippet end ruby_weakhash_argon2_verify
