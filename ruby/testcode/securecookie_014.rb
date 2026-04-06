require_relative 'shared'

module CookieJar
  def self.signed; SignedJar.new; end
  class SignedJar
    def []=(key, val); end
  end
end

# vuln-code-snippet start ruby_securecookie_signed
def set_signed_cookie(req)
  user_id = req.param('user_id').to_i
  jar = CookieJar.signed
  jar[:user_id] = user_id # vuln-code-snippet safe-line ruby_securecookie_signed
  BenchmarkResponse.ok('signed cookie set')
end
# vuln-code-snippet end ruby_securecookie_signed
