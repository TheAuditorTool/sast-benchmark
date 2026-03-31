require_relative 'shared'

class InternalConfig
  def initialize
    @db_password = 'secret'
    @api_key     = 'key-12345'
    @debug_mode  = false
  end
end

# vuln-code-snippet start ruby_dynmethod_ivar_get
def dynmethod_ivar_get(req)
  field  = req.param('field')
  config = InternalConfig.new
  value  = config.instance_variable_get("@#{field}") # vuln-code-snippet vuln-line ruby_dynmethod_ivar_get
  BenchmarkResponse.ok(value.to_s)
end
# vuln-code-snippet end ruby_dynmethod_ivar_get
