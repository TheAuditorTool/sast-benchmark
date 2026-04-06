require_relative 'shared'

module CookieJar
  def self.encrypted; EncryptedJar.new; end
  class EncryptedJar
    def []=(key, val); end
  end
end

# vuln-code-snippet start ruby_securecookie_encrypted
def set_encrypted_cookie(req)
  user_id = req.param('user_id').to_i
  jar = CookieJar.encrypted
  jar[:user_id] = user_id # vuln-code-snippet safe-line ruby_securecookie_encrypted
  BenchmarkResponse.ok('encrypted cookie set')
end
# vuln-code-snippet end ruby_securecookie_encrypted
