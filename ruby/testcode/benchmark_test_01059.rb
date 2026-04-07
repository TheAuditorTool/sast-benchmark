require_relative 'shared'

def api_create_resource(req)
  auth_header = req.header('Authorization')
  return BenchmarkResponse.bad_request('missing token') unless auth_header&.start_with?('Bearer ')
  jwt_token = auth_header.split(' ', 2).last
  payload = JWT.decode(jwt_token, ENV.fetch('JWT_SECRET'), true, algorithms: ['HS256']).first
  BenchmarkResponse.json({ result: payload['sub'] })
end
