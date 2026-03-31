require_relative 'shared'

class PublicProfile
  def initialize
    @name     = 'Alice'
    @bio      = 'Developer'
    @password = 'hunter2'
  end
end

SAFE_ATTRS = %w[name bio].freeze

# vuln-code-snippet start ruby_dynmethod_respond_to
def dynmethod_respond_to(req)
  field   = req.param('field')
  profile = PublicProfile.new
  if profile.respond_to?(field) && SAFE_ATTRS.include?(field) # vuln-code-snippet safe-line ruby_dynmethod_respond_to
    value = profile.instance_variable_get("@#{field}")
    BenchmarkResponse.ok(value.to_s)
  else
    BenchmarkResponse.bad_request('not allowed')
  end
end
# vuln-code-snippet end ruby_dynmethod_respond_to
