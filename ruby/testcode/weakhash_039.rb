require_relative 'shared'

begin
  require 'bcrypt'
rescue LoadError
  # bcrypt gem not available -- syntax-checked only
end

# Devise default: BCrypt under the hood with cost 11+.
# vuln-code-snippet start ruby_weakhash_devise_bcrypt
def store_password_devise(req)
  password = req.param('password')
  hashed = BCrypt::Password.create(password) # vuln-code-snippet safe-line ruby_weakhash_devise_bcrypt
  BenchmarkResponse.json({ hash: hashed.to_s })
end
# vuln-code-snippet end ruby_weakhash_devise_bcrypt
