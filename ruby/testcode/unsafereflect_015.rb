require_relative 'shared'

class ApiClient
  def get(url); "GET #{url}"; end
  def post(url); "POST #{url}"; end
end

# vuln-code-snippet start ruby_reflect_method_dispatch
def api_call(req)
  method_name = req.param('method')
  url = req.param('url')
  client = ApiClient.new
  m = client.method(method_name.to_sym) # vuln-code-snippet vuln-line ruby_reflect_method_dispatch
  BenchmarkResponse.ok(m.call(url))
end
# vuln-code-snippet end ruby_reflect_method_dispatch
