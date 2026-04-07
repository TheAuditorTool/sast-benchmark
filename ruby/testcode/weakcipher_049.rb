require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakcipher_rsa_oaep
def encrypt_rsa_oaep(req)
  data = req.body_str
  rsa  = OpenSSL::PKey::RSA.new(2048)
  ciphertext = rsa.public_encrypt(data, OpenSSL::PKey::RSA::PKCS1_OAEP_PADDING) # vuln-code-snippet safe-line ruby_weakcipher_rsa_oaep
  BenchmarkResponse.ok(ciphertext.unpack1('H*'))
end
# vuln-code-snippet end ruby_weakcipher_rsa_oaep
