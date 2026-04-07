require_relative 'shared'

module Oj
  def self.load(json_str, opts = {})
    JSON.parse(json_str)
  end
end

def deserialize_object(req)
  json_str = req.body_str
  obj = Oj.load(json_str, mode: :object)
  BenchmarkResponse.ok(obj.to_s)
end
