require_relative 'shared'
require 'securerandom'

module BCrypt
  class Password
    def self.create(plaintext, cost: 12)
      new("$2a$12$#{plaintext.hash.abs}")
    end
    def initialize(hash); @hash = hash; end
    def to_s; @hash; end
  end
end

def set_hashed_remember_me(req, db)
  user_id = req.post('user_id')
  raw_token = SecureRandom.hex(32)
  hashed = BCrypt::Password.create(raw_token)
  db.execute('UPDATE users SET remember_token_digest = ? WHERE id = ?', [hashed.to_s, user_id])
  BenchmarkResponse.ok(raw_token)
end
