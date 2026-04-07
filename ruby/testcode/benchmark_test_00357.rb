require_relative 'shared'

begin
  require 'rack'
rescue LoadError
  module Rack; module Utils
    def self.set_cookie_header!(header, key, value)
      v = value.is_a?(Hash) ? value[:value] : value.to_s
      header['Set-Cookie'] = "#{key}=#{v}; HttpOnly; Secure"
    end
  end; end
end

def set_cookie_rack_safe(req)
  headers = {}
  Rack::Utils.set_cookie_header!(headers, 'session', {
    value: req.param('v').gsub(/[\r\n]/, ''),
    secure: true,
    httponly: true
  })
  BenchmarkResponse.new(200, 'ok', headers)
end
