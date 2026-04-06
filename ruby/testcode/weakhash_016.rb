require_relative 'shared'

module SCrypt
  class Password
    def self.create(password, opts = {}); "scrypt:#{password}"; end
  end
end

# vuln-code-snippet start ruby_weakhash_scrypt_create
def hash_password_scrypt(req)
  password = req.param('password')
  hashed = SCrypt::Password.create(password) # vuln-code-snippet safe-line ruby_weakhash_scrypt_create
  BenchmarkResponse.ok(hashed)
end
# vuln-code-snippet end ruby_weakhash_scrypt_create
