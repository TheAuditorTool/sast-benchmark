require_relative 'shared'

class FixedTarget047
  def read;  "reading";  end
  def write; "writing";  end
  def list;  "listing";  end
end

METHODS_047 = %i[read write list].freeze

def method_fixed_list(req)
  m      = req.param('m').to_sym
  target = FixedTarget047.new
  raise ArgumentError, 'not in list' unless METHODS_047.include?(m)
  result = target.send(m)
  BenchmarkResponse.json({ result: result })
end
