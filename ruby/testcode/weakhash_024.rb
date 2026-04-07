require_relative 'shared'
require 'digest'

# vuln-code-snippet start ruby_weakhash_md5_email_concat
def store_password_md5_email(req)
  email    = req.param('email')
  password = req.param('password')
  digest = Digest::MD5.hexdigest(email + ':' + password) # vuln-code-snippet vuln-line ruby_weakhash_md5_email_concat
  BenchmarkResponse.json({ hash: digest })
end
# vuln-code-snippet end ruby_weakhash_md5_email_concat
