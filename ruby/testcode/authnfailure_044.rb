require_relative 'shared'
require 'securerandom'

TTL_SECONDS = 900

# vuln-code-snippet start ruby_authn_secure_magic_link
def generate_secure_magic_link(req, db)
  username = req.post('username')
  token = SecureRandom.urlsafe_base64(32) # vuln-code-snippet safe-line ruby_authn_secure_magic_link
  expires_at = Time.now.to_i + TTL_SECONDS
  db.execute('INSERT INTO magic_links (username, token, expires_at) VALUES (?, ?, ?)', [username, token, expires_at])
  BenchmarkResponse.ok("Magic link sent to #{username}")
end
# vuln-code-snippet end ruby_authn_secure_magic_link
