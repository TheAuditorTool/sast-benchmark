require_relative 'shared'

def hashie_mash_json_handler(req)
  obj = Hashie::Mash.new(JSON.parse(req.post('data')))
  BenchmarkResponse.json({ result: obj.to_h })
end
