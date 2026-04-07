require_relative 'shared'

begin
  require 'bcrypt'
rescue LoadError
  # bcrypt gem not available -- syntax-checked only
end

class User
  include BCrypt

  def self.create_with_password(pw)
    BCrypt::Password.create(pw)
  end
end

# vuln-code-snippet start ruby_weakhash_has_secure_password
def register_user(req)
  password = req.param('password')
  hash = User.create_with_password(password)  # vuln-code-snippet safe-line ruby_weakhash_has_secure_password
  BenchmarkResponse.json({ ok: true, hash: hash })
end
# vuln-code-snippet end ruby_weakhash_has_secure_password
