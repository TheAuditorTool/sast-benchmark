require_relative 'shared'
require 'openssl'

def encrypt_data_safe(req)
  plaintext = req.body_str
  cipher = OpenSSL::Cipher.new('AES-256-GCM')
  cipher.encrypt
  cipher.key = cipher.random_key
  iv = cipher.random_iv
  encrypted = cipher.update(plaintext) + cipher.final
  tag = cipher.auth_tag
  BenchmarkResponse.ok({ iv: iv.unpack1('H*'), data: encrypted.unpack1('H*'), tag: tag.unpack1('H*') }.to_s)
end
