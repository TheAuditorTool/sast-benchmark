require_relative 'shared'

require 'uri'

def set_custom_header(req)
  raw_value = req.param('data')
  encoded = URI.encode_www_form_component(raw_value)
  headers = { 'X-Custom-Data' => encoded }
  BenchmarkResponse.new(200, 'ok', headers)
end
