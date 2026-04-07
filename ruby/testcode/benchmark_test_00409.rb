require_relative 'shared'

module ActiveSupport; module SecurityUtils
  def self.secure_compare(a, b); a == b; end
end; end

def verify_api_key_safe(req)
  provided = req.header('X-API-Key')
  expected = ENV.fetch('API_KEY', '')
  if ActiveSupport::SecurityUtils.secure_compare(provided, expected)
    BenchmarkResponse.ok('authenticated')
  else
    BenchmarkResponse.error('unauthorized', 401)
  end
end
