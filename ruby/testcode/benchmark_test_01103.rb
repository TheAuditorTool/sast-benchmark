require_relative 'shared'

module CookieJar
  def self.signed; SignedJar.new; end
  class SignedJar
    def []=(key, val); end
  end
end

def set_signed_cookie(req)
  user_id = req.param('user_id').to_i
  jar = CookieJar.signed
  jar[:user_id] = user_id
  BenchmarkResponse.ok('signed cookie set')
end
