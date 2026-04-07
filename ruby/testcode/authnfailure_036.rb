require_relative 'shared'

module Rack
  module Utils
    def self.secure_compare(a, b)
      return false unless a.bytesize == b.bytesize
      l = a.unpack('C*')
      r = b.unpack('C*')
      l.zip(r).reduce(0) { |sum, (x, y)| sum | (x ^ y) } == 0
    end
  end
end

# vuln-code-snippet start ruby_authn_secure_compare_token
def authenticate_api_token_safe(req)
  provided = req.header('X-Api-Token')
  stored = ENV.fetch('API_TOKEN', 'secret-token')
  return BenchmarkResponse.error('Unauthorized', 401) unless Rack::Utils.secure_compare(provided, stored) # vuln-code-snippet safe-line ruby_authn_secure_compare_token
  BenchmarkResponse.ok('authenticated')
end
# vuln-code-snippet end ruby_authn_secure_compare_token
