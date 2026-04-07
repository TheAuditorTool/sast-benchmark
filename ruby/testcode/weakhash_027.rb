require_relative 'shared'
require 'digest'

# vuln-code-snippet start ruby_weakhash_sha1_per_char
def store_password_per_char(req)
  password = req.param('password')
  digest = password.chars.map { |c| Digest::SHA1.hexdigest(c) }.join # vuln-code-snippet vuln-line ruby_weakhash_sha1_per_char
  BenchmarkResponse.json({ hash: digest })
end
# vuln-code-snippet end ruby_weakhash_sha1_per_char
