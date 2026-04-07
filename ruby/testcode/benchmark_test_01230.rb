require_relative 'shared'
require 'securerandom'

def consume_magic_link(req, db)
  token = req.param('token')
  row = db.execute('SELECT user_id FROM magic_links WHERE token = ? AND expires_at > ?', [token, Time.now.to_i]).first
  return BenchmarkResponse.error('Invalid or expired token', 401) unless row
  db.execute('DELETE FROM magic_links WHERE token = ?', [token])
  BenchmarkResponse.ok("Authenticated as user #{row[0]}")
end
