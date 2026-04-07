require_relative 'shared'
require 'openssl'

# vuln-code-snippet start ruby_weakhash_rmd160
def store_password_rmd160(req)
  password = req.param('password')
  digest = OpenSSL::Digest.new('RIPEMD160').hexdigest(password) # vuln-code-snippet vuln-line ruby_weakhash_rmd160
  BenchmarkResponse.json({ hash: digest })
end
# vuln-code-snippet end ruby_weakhash_rmd160
