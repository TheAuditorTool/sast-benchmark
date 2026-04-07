require_relative 'shared'

class MyService044
  def read;  "reading";  end
  def write; "writing";  end
  def list;  "listing";  end
end

# vuln-code-snippet start ruby_dynmethod_public_send_own
def public_send_own(req)
  m   = req.param('method').to_sym
  svc = MyService044.new
  raise ArgumentError, 'not allowed' unless MyService044.public_method_defined?(m) # vuln-code-snippet safe-line ruby_dynmethod_public_send_own
  result = svc.public_send(m)
  BenchmarkResponse.json({ result: result })
end
# vuln-code-snippet end ruby_dynmethod_public_send_own
