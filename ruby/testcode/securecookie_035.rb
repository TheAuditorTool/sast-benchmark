require_relative 'shared'
require 'securerandom'

# Stub Rails-style encrypted cookies
module FakeCookies
  def self.encrypted
    EncryptedJar.new
  end

  class EncryptedJar
    def []=(key, value)
      # encrypted store -- value is never written in plaintext
    end
  end
end

# vuln-code-snippet start ruby_securecookie_encrypted_rails
def set_session(req)
  token = SecureRandom.hex(32)
  FakeCookies.encrypted[:session] = token # vuln-code-snippet safe-line ruby_securecookie_encrypted_rails
  BenchmarkResponse.ok('session set')
end
# vuln-code-snippet end ruby_securecookie_encrypted_rails
