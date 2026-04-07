require_relative 'shared'

class MyService044
  def read;  "reading";  end
  def write; "writing";  end
  def list;  "listing";  end
end

def public_send_own(req)
  m   = req.param('method').to_sym
  svc = MyService044.new
  raise ArgumentError, 'not allowed' unless MyService044.public_method_defined?(m)
  result = svc.public_send(m)
  BenchmarkResponse.json({ result: result })
end
