require_relative 'shared'
require 'openssl'
require 'securerandom'

begin
  require 'argon2'
rescue LoadError
  # argon2 gem not available -- syntax-checked only
end

# vuln-code-snippet start ruby_weakcipher_argon2_derived_key
def derive_key_argon2(req)
  password = req.param('password')
  key_material = Argon2::Password.new(t_cost: 3, m_cost: 16).create(password) # vuln-code-snippet safe-line ruby_weakcipher_argon2_derived_key
  key = key_material[0, 32]
  cipher = OpenSSL::Cipher.new('AES-256-GCM')
  cipher.encrypt
  cipher.key = key
  cipher.iv  = SecureRandom.random_bytes(12)
  cipher.auth_data = ''
  ciphertext = cipher.update(req.body_str) + cipher.final
  BenchmarkResponse.json({ data: ciphertext.unpack1('H*') })
end
# vuln-code-snippet end ruby_weakcipher_argon2_derived_key
