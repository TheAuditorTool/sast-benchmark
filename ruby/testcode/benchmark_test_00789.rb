require_relative 'shared'

class ApiClient
  def get(url); "GET #{url}"; end
  def post(url); "POST #{url}"; end
end

def api_call(req)
  method_name = req.param('method')
  url = req.param('url')
  client = ApiClient.new
  m = client.method(method_name.to_sym)
  BenchmarkResponse.ok(m.call(url))
end
