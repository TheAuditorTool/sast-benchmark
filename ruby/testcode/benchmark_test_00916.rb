require_relative 'shared'

module Oj
  def self.load(input, opts = {}); input; end
end

def parse_oj_object(req)
  data = Oj.load(req.body_str, mode: :object)
  BenchmarkResponse.ok(data.to_s)
end
