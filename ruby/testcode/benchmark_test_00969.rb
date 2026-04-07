require_relative 'shared'

class MyService
  def process; 'done'; end
end

def handler(req)
  m = req.param('m').to_sym
  raise ArgumentError, 'method not allowed' unless MyService.public_method_defined?(m)
  result = MyService.new.send(m)
  BenchmarkResponse.json({ result: result })
end
