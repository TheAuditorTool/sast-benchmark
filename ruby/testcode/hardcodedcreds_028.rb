require 'openssl'
require_relative 'shared'

RSA_KEY = <<~PEM
  -----BEGIN RSA PRIVATE KEY-----
  MIIEowIBAAKCAQEA0Z3VS5JJcds3xHn/ygWep4PAtEsHAAAAAAAAAAAAAACfj5t0
  Qv3HqAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
  AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
  -----END RSA PRIVATE KEY-----
PEM

# vuln-code-snippet start ruby_hardcoded_pem_private_key
def sign_jwt_handler(req)
  private_key = OpenSSL::PKey::RSA.new(RSA_KEY)  # vuln-code-snippet vuln-line ruby_hardcoded_pem_private_key
  payload = { sub: req[:user_id], exp: Time.now.to_i + 3600 }
  token = JWT.encode(payload, private_key, 'RS256')
  BenchmarkResponse.json({ token: token })
end
# vuln-code-snippet end ruby_hardcoded_pem_private_key
