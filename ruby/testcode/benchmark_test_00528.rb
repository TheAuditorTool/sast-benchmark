require_relative 'shared'

class InternalConfig
  def initialize
    @db_password = 'secret'
    @api_key     = 'key-12345'
    @debug_mode  = false
  end
end

def handler(req)
  field  = req.param('field')
  config = InternalConfig.new
  value  = config.instance_variable_get("@#{field}")
  BenchmarkResponse.ok(value.to_s)
end
