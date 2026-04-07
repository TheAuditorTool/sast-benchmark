require_relative 'shared'
require 'openssl'
require 'rack/utils'

def verify_password_pbkdf2(req)
  provided    = req.param('password')
  stored_hex  = req.param('stored_hash')
  salt_hex    = req.param('salt')
  stored      = [stored_hex].pack('H*')
  salt        = [salt_hex].pack('H*')
  derived = OpenSSL::PKCS5.pbkdf2_hmac(provided, salt, 600_000, 32, 'SHA256')
  match = Rack::Utils.secure_compare(derived, stored)
  BenchmarkResponse.json({ match: match })
end
