require_relative 'shared'

def rack_response(req)
  header_name = req.param('header')
  header_value = req.param('value')
  [200, { header_name => header_value }, ['ok']]
end
