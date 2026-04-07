require_relative 'shared'

module Oj
  def self.load(input, opts = {}); input; end
end

def parse_oj_safe(req)
  data = Oj.load(req.body_str, mode: :strict)
  BenchmarkResponse.ok(data.to_s)
end
