require_relative 'shared'

begin
  require 'rbnacl'
rescue LoadError
  # rbnacl gem not available -- syntax-checked only
end

# vuln-code-snippet start ruby_weakhash_rbnacl_argon2
def store_password_rbnacl_argon2(req)
  password = req.param('password')
  salt     = RbNaCl::Random.random_bytes(RbNaCl::PasswordHash::Argon2::SALTBYTES)
  hashed   = RbNaCl::PasswordHash.argon2(password, salt, opslimit: 3, memlimit: 2**17) # vuln-code-snippet safe-line ruby_weakhash_rbnacl_argon2
  BenchmarkResponse.json({ hash: hashed.unpack1('H*'), salt: salt.unpack1('H*') })
end
# vuln-code-snippet end ruby_weakhash_rbnacl_argon2
