require_relative 'shared'

module CookieJar
  def self.encrypted; EncryptedJar.new; end
  class EncryptedJar
    def []=(key, val); end
  end
end

def set_encrypted_cookie(req)
  user_id = req.param('user_id').to_i
  jar = CookieJar.encrypted
  jar[:user_id] = user_id
  BenchmarkResponse.ok('encrypted cookie set')
end
