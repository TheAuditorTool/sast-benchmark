require_relative 'shared'

def set_transfer_encoding(req)
  response = BenchmarkResponse.ok('ok')
  response.headers['Transfer-Encoding'] = req.param('encoding')
  response
end
