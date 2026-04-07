require_relative 'shared'

# vuln-code-snippet start ruby_deser_hashie_mash_json
def hashie_mash_json_handler(req)
  obj = Hashie::Mash.new(JSON.parse(req.post('data')))  # vuln-code-snippet safe-line ruby_deser_hashie_mash_json
  BenchmarkResponse.json({ result: obj.to_h })
end
# vuln-code-snippet end ruby_deser_hashie_mash_json
