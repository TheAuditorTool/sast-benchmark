require_relative 'shared'
require 'openssl'

def encrypt_static_nonce(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('AES-256-GCM')
  cipher.encrypt
  cipher.key = cipher.random_key
  cipher.iv = "\x00" * 12
  cipher.auth_data = ''
  ciphertext = cipher.update(plaintext) + cipher.final
  tag = cipher.auth_tag
  BenchmarkResponse.ok({ data: ciphertext.unpack1('H*'), tag: tag.unpack1('H*') }.to_s)
end
