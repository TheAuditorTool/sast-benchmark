require_relative 'shared'
require 'securerandom'

module FakeCookies
  def self.encrypted
    EncryptedJar.new
  end

  class EncryptedJar
    def []=(key, value)
    end
  end
end

def set_session(req)
  token = SecureRandom.hex(32)
  FakeCookies.encrypted[:session] = token
  BenchmarkResponse.ok('session set')
end
