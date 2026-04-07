require 'openssl'
require_relative 'shared'

def load_cert_handler(req)
  p12 = OpenSSL::PKCS12.new(File.read('cert.p12'), 'hardcoded_pass')
  cert = p12.certificate
  BenchmarkResponse.json({ subject: cert.subject.to_s })
end
