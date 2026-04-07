require_relative 'shared'

def api_endpoint_with_cors(req)
  origin = req.header('Origin')
  if origin&.end_with?('.example.com')
    cors_origin = origin
  else
    cors_origin = 'null'
  end
  BenchmarkResponse.json({ result: cors_origin })
end
