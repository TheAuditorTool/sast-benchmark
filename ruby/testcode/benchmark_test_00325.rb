require_relative 'shared'
require 'securerandom'

TTL_SECONDS = 900

def generate_secure_magic_link(req, db)
  username = req.post('username')
  token = SecureRandom.urlsafe_base64(32)
  expires_at = Time.now.to_i + TTL_SECONDS
  db.execute('INSERT INTO magic_links (username, token, expires_at) VALUES (?, ?, ?)', [username, token, expires_at])
  BenchmarkResponse.ok("Magic link sent to #{username}")
end
