require_relative 'shared'

def request_id_header(req)
  id = req.param('id').to_i
  BenchmarkResponse.new(200, 'ok', { 'X-Request-Id' => id.to_s })
end
