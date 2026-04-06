require_relative 'shared'

module Rack; module Utils
  def self.set_cookie_header!(header, key, value)
    header['Set-Cookie'] = "#{key}=#{value}; HttpOnly; Secure"
  end
end; end

# vuln-code-snippet start ruby_headerinj_rack_utils
def set_cookie_rack(req)
  session_id = req.param('session_id')
  headers = {}
  Rack::Utils.set_cookie_header!(headers, 'session', session_id) # vuln-code-snippet safe-line ruby_headerinj_rack_utils
  BenchmarkResponse.new(200, 'ok', headers)
end
# vuln-code-snippet end ruby_headerinj_rack_utils
