require 'openssl'
require_relative 'shared'

# vuln-code-snippet start ruby_hardcoded_pkcs12_pass
def load_cert_handler(req)
  p12 = OpenSSL::PKCS12.new(File.read('cert.p12'), 'hardcoded_pass')  # vuln-code-snippet vuln-line ruby_hardcoded_pkcs12_pass
  cert = p12.certificate
  BenchmarkResponse.json({ subject: cert.subject.to_s })
end
# vuln-code-snippet end ruby_hardcoded_pkcs12_pass
